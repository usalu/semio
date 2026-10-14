//! 🧵️ `s.fem.fem3d` mounted editor session: one visual job per document revision, recovery and retirement.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod fem3d {
        #[path = "."]
        pub mod session {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
