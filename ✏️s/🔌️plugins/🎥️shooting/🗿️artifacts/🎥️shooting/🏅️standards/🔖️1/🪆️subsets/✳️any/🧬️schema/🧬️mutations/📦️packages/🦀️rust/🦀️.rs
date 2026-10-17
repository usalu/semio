//! 🧬️ mut-any of `semio-s-artifact-shooting-shooting` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_shooting_shooting_inferences::{ShootingShot, quat_from_axis_angle, quat_mul, shooting_asset_scale};
pub use semio_s_artifact_shooting_shooting_schema::{ShootingAssetPatch, ShootingAssigned, ShootingShotPatch};

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
                        pub mod drag_assets {
                            #[path = "../../↔️drag-assets/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../↔️drag-assets/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↔️drag-assets/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rotate_assets {
                            #[path = "../../🔄️rotate-assets/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔄️rotate-assets/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔄️rotate-assets/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod scale_assets {
                            #[path = "../../↕️scale-assets/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../↕️scale-assets/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↕️scale-assets/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_shot {
                            #[path = "../../📸️create-shot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../📸️create-shot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📸️create-shot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_shot {
                            #[path = "../../🚮️delete-shot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🚮️delete-shot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🚮️delete-shot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rename_shot {
                            #[path = "../../🏷️rename-shot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🏷️rename-shot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏷️rename-shot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_shot_width {
                            #[path = "../../📏️change-shot-width/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../📏️change-shot-width/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📏️change-shot-width/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_shot_height {
                            #[path = "../../📐️change-shot-height/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../📐️change-shot-height/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📐️change-shot-height/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_shot_format {
                            #[path = "../../🖼️change-shot-format/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🖼️change-shot-format/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🖼️change-shot-format/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_shooting_shooting_schema::diff;
