//! 🎬️ `s.fem.fem3d` World3d scene: primitives, solid surfaces and one pickable instance per document entity.

#![allow(clippy::result_large_err)]



extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🪟️viewport/🦀️.rs"]
pub mod viewport;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../🦀️.rs"]
                pub mod scene;
            }
        }
    }
}
