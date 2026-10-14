//! 📊️ `s.fem.fem2d` results window: its config, playback transient and the results canvas scene.

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
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "../../🦀️.rs"]
                    pub mod results;
                }
            }
        }
    }
}
