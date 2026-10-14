//! 🎭️ Generation3d viewer modes: the view mode and its preview window.
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod view {
    #[path = "👁️view/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod windows {
        #[path = "."]
        pub mod preview {
            #[path = "👁️view/🪟️windows/👁️preview/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "👁️view/🪟️windows/👁️preview/🫧️transient/🦀️.rs"]
            pub mod transient;
        }
    }
}
