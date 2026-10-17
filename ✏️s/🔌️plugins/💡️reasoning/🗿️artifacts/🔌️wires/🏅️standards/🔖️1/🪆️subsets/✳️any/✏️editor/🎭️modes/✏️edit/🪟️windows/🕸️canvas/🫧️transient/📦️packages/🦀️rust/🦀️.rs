//! 🫧️ window-transient of `semio-s-artifact-reasoning-wires` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wires {
        #[path = "../../🦀️.rs"]
        pub mod window_transient;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_reasoning_wires_windows::editor::wires::modes::edit::windows;
            }
        }
    }
}
