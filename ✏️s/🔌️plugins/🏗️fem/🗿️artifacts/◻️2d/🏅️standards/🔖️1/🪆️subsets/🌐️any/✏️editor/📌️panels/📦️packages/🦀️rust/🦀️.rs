//! 📌️ `s.fem.fem2d` editor panels: artifact, inspection and results.

#![allow(clippy::result_large_err)]



extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod fem2d {
        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "../../🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "../../📊️results/🦀️.rs"]
            pub mod results;
        }
    }
}
