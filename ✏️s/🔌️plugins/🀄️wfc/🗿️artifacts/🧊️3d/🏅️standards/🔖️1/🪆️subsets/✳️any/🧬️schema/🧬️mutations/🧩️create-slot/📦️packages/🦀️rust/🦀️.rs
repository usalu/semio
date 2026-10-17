//! 🧩️ mut-family of `semio-s-artifact-wfc-3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
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
                pub mod examples {
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples::two_room_corridor;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_slot {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            #[path = "../../../🕳️delete-slot/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🕳️delete-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🕳️delete-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod drag_slots {
                            #[path = "../../../✋️drag-slots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../✋️drag-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../✋️drag-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_slot_positions {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::inverse;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            #[path = "../../../🖼️change-tile-media/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖼️change-tile-media/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖼️change-tile-media/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[path = "../../../🚦️create-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🚦️create-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🚦️create-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "../../../🚫️delete-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🚫️delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🚫️delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "../../../🎲️change-seed/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc3d {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_wfc_3d_config::editor::wfc3d::config::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
}
