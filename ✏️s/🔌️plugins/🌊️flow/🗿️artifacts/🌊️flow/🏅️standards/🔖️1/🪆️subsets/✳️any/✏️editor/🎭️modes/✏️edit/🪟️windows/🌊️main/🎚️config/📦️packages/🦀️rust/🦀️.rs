//! 🎚️ config of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

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
                    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::base;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🦀️.rs"]
                        pub mod config;
                    }
                }
            }
        }
    }
}
