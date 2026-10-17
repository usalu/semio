//! 🎚️ config of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        #[path = "."]
        pub mod config {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
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
    }
}
