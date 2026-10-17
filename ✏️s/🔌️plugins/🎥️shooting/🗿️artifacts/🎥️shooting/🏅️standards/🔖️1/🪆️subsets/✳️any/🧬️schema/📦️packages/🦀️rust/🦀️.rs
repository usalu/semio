//! 🧬️ schema of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
pub use semio_s_artifact_shooting_shooting_inferences::{SHOOTING_DOCUMENT_SCHEMA, ShootingAsset, ShootingCamera, ShootingEmblemChild, ShootingSavedCamera, ShootingSceneLighting, ShootingShot};
pub use crate::standards::v1::subsets::any::schema::artifact_items_8::{ShootingAssetPatch, ShootingAssigned, ShootingSavedCameraPatch, ShootingScenePatch, ShootingShotPatch};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod hexagonal_cut_concrete_forest_left {
        #[cfg(test)]
        #[path = "../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config::*;
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config::*;
            #[cfg(test)]
            #[path = "../../../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
            #[cfg(test)]
            #[path = "../../../✏️editor/🎚️config/🧪️tests/🔬️contract-vectors/🦀️.rs"]
            pub mod contract_vectors;
        }
    }
}
