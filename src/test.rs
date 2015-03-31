// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
mod debug_struct {
    use std::fmt;
    use super::super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugStruct::new(fmt, "Foo").finish()
            }
        }

        assert_eq!("Foo", format!("{:?}", Foo));
        assert_eq!("Foo", format!("{:#?}", Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugStruct::new(fmt, "Foo")
                    .field("bar", &true)
                    .finish()
            }
        }

        assert_eq!("Foo { bar: true }", format!("{:?}", Foo));
        assert_eq!(
"Foo {
    bar: true
}",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugStruct::new(fmt, "Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        assert_eq!("Foo { bar: true, baz: 10/20 }", format!("{:?}", Foo));
        assert_eq!(
"Foo {
    bar: true,
    baz: 10/20
}",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugStruct::new(fmt, "Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugStruct::new(fmt, "Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish()
            }
        }

        assert_eq!("Bar { foo: Foo { bar: true, baz: 10/20 }, hello: \"world\" }",
                   format!("{:?}", Bar));
        assert_eq!(
"Bar {
    foo: Foo {
        bar: true,
        baz: 10/20
    },
    hello: \"world\"
}",
                   format!("{:#?}", Bar));
    }
}

mod debug_tuple {
    use std::fmt;
    use super::super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugTuple::new(fmt, "Foo").finish()
            }
        }

        assert_eq!("Foo", format!("{:?}", Foo));
        assert_eq!("Foo", format!("{:#?}", Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugTuple::new(fmt, "Foo")
                    .field(&true)
                    .finish()
            }
        }

        assert_eq!("Foo(true)", format!("{:?}", Foo));
        assert_eq!(
"Foo(
    true
)",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugTuple::new(fmt, "Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        assert_eq!("Foo(true, 10/20)", format!("{:?}", Foo));
        assert_eq!(
"Foo(
    true,
    10/20
)",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugTuple::new(fmt, "Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugTuple::new(fmt, "Bar")
                    .field(&Foo)
                    .field(&"world")
                    .finish()
            }
        }

        assert_eq!("Bar(Foo(true, 10/20), \"world\")",
                   format!("{:?}", Bar));
        assert_eq!(
"Bar(
    Foo(
        true,
        10/20
    ),
    \"world\"
)",
                   format!("{:#?}", Bar));
    }
}

mod debug_map {
    use std::fmt;
    use super::super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugMap::new(fmt).finish()
            }
        }

        assert_eq!("{}", format!("{:?}", Foo));
        assert_eq!("{}", format!("{:#?}", Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugMap::new(fmt)
                    .entry(&"bar", &true)
                    .finish()
            }
        }

        assert_eq!("{\"bar\": true}", format!("{:?}", Foo));
        assert_eq!(
"{
    \"bar\": true
}",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugMap::new(fmt)
                    .entry(&"bar", &true)
                    .entry(&10i32, &format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        assert_eq!("{\"bar\": true, 10: 10/20}", format!("{:?}", Foo));
        assert_eq!(
"{
    \"bar\": true,
    10: 10/20
}",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugMap::new(fmt)
                    .entry(&"bar", &true)
                    .entry(&10i32, &format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugMap::new(fmt)
                    .entry(&"foo", &Foo)
                    .entry(&Foo, &"world")
                    .finish()
            }
        }

        assert_eq!("{\"foo\": {\"bar\": true, 10: 10/20}, \
                    {\"bar\": true, 10: 10/20}: \"world\"}",
                   format!("{:?}", Bar));
        assert_eq!(
"{
    \"foo\": {
        \"bar\": true,
        10: 10/20
    },
    {
        \"bar\": true,
        10: 10/20
    }: \"world\"
}",
                   format!("{:#?}", Bar));
    }
}

mod debug_set {
    use std::fmt;
    use super::super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugSet::new(fmt).finish()
            }
        }

        assert_eq!("{}", format!("{:?}", Foo));
        assert_eq!("{}", format!("{:#?}", Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugSet::new(fmt)
                    .entry(&true)
                    .finish()
            }
        }

        assert_eq!("{true}", format!("{:?}", Foo));
        assert_eq!(
"{
    true
}",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugSet::new(fmt)
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        assert_eq!("{true, 10/20}", format!("{:?}", Foo));
        assert_eq!(
"{
    true,
    10/20
}",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugSet::new(fmt)
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugSet::new(fmt)
                    .entry(&Foo)
                    .entry(&"world")
                    .finish()
            }
        }

        assert_eq!("{{true, 10/20}, \"world\"}",
                   format!("{:?}", Bar));
        assert_eq!(
"{
    {
        true,
        10/20
    },
    \"world\"
}",
                   format!("{:#?}", Bar));
    }
}

mod debug_list {
    use std::fmt;
    use super::super::*;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugList::new(fmt).finish()
            }
        }

        assert_eq!("[]", format!("{:?}", Foo));
        assert_eq!("[]", format!("{:#?}", Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugList::new(fmt)
                    .entry(&true)
                    .finish()
            }
        }

        assert_eq!("[true]", format!("{:?}", Foo));
        assert_eq!(
"[
    true
]",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugList::new(fmt)
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        assert_eq!("[true, 10/20]", format!("{:?}", Foo));
        assert_eq!(
"[
    true,
    10/20
]",
                   format!("{:#?}", Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugList::new(fmt)
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10i32, 20i32))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                DebugList::new(fmt)
                    .entry(&Foo)
                    .entry(&"world")
                    .finish()
            }
        }

        assert_eq!("[[true, 10/20], \"world\"]",
                   format!("{:?}", Bar));
        assert_eq!(
"[
    [
        true,
        10/20
    ],
    \"world\"
]",
                   format!("{:#?}", Bar));
    }
}
