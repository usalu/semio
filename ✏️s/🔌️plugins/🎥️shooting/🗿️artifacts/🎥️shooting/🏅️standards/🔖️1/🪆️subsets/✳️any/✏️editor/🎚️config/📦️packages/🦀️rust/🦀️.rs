//! 🎚️ config of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_shooting_shooting_inferences::{ShootingCamera};

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        #[path = "."]
        pub mod config {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🧬️schema/🦀️.rs"]
            pub mod schema;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../../👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../../👥️presence/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../../👥️presence/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
    }
}
