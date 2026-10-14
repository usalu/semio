//! ⚙️ `s.fem.fem2d` engine bridge: meshing, mesh preview and modal/buckling over the shared fem engine.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod fem2d_engine {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../🗺️mesh-preview/🦀️.rs"]
    pub mod mesh_preview;
    #[path = "../../🕸️meshing/🦀️.rs"]
    pub mod meshing;
    #[path = "../../🎵️modal-buckling/🦀️.rs"]
    pub mod modal_buckling;
}
