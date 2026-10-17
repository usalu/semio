//! ✂️ mut-any-2 of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_shooting_shooting_inferences::{ShootingCamera, ShootingSavedCamera};
pub use semio_s_artifact_shooting_shooting_schema::{ShootingSavedCameraPatch, ShootingShotPatch};

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
                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_shot_shape {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod reorder_shots {
                            #[path = "../../../🔃️reorder-shots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔃️reorder-shots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔃️reorder-shots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_shot_camera {
                            #[path = "../../../📷️replace-shot-camera/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📷️replace-shot-camera/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📷️replace-shot-camera/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_saved_camera {
                            #[path = "../../../🎥️create-saved-camera/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🎥️create-saved-camera/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🎥️create-saved-camera/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_saved_camera {
                            #[path = "../../../🧹️delete-saved-camera/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧹️delete-saved-camera/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧹️delete-saved-camera/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rename_saved_camera {
                            #[path = "../../../🪪️rename-saved-camera/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🪪️rename-saved-camera/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🪪️rename-saved-camera/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_saved_camera_view {
                            #[path = "../../../🎞️replace-saved/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🎞️replace-saved/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🎞️replace-saved/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod reorder_saved_cameras {
                            #[path = "../../../🔁️reorder-saved-cameras/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔁️reorder-saved-cameras/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔁️reorder-saved-cameras/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_active_shot {
                            #[path = "../../../🎯️set-active-shot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🎯️set-active-shot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🎯️set-active-shot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_shooting_shooting_schema::diff;
