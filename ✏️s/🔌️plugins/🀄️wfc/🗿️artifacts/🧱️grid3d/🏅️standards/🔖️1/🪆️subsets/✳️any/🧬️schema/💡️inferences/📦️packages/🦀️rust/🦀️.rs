//! 💡️ inferences of `semio-s-artifact-wfc-grid3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::Grid3dMutation;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

#[path = "."]
pub mod host {
    #[path="../../../../../../../../🔨️modules/🏠️host/💡️inferences/🦀️.rs"]
    pub mod inferences;
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
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path="../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🌐️scene-internals/🦀️.rs"]
                    pub mod scene_internals;
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
