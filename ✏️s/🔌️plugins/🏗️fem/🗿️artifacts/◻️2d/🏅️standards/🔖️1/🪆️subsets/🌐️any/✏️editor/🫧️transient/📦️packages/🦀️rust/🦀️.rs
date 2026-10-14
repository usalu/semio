//! 🫧️ `s.fem.fem2d` editor transient state: selection, hover and pending gestures.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🦀️.rs"]
        pub mod transient;
    }
}
