//! 💡️ inferences of `semio-s-artifact-wfc-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::mutations::Wfc2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc2dSnapshot;
pub use crate::schema::snapshot::WFC_2D_DOCUMENT_SCHEMA;
pub use crate::schema::snapshot::WFC_2D_DEFAULT_RELATION;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub use crate::viewer::wfc2d::artifact_items::{WFC_2D_DIALECT};

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
                    #[path = "../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod drag_slots {
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::*;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::diff;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🎨️change-tile-media/🧪️tests/🎨️repaints/🦀️.rs"]
                            pub mod tests_repaints_the_roof_tile_as_a_raster;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🚦️create-rule/🧪️tests/⛔️forbids-roof-over-roof/🦀️.rs"]
                            pub mod tests_forbids_roof_over_roof;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/❌delete-rule/🧪️tests/🚫️removes-the-wall-wall-rule/🦀️.rs"]
                            pub mod tests_removes_the_wall_wall_rule;
                        }
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
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc2d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod tools {
                    #[path = "../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs"]
                    pub mod drag;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wfc2d {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_2d_pin_slot::viewer::wfc2d::modes::view::*;
                pub use semio_s_artifact_wfc_2d_pin_slot::viewer::wfc2d::modes::view::windows;
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::*;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::two_room_corridor;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::wall_roof_facade_strip;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::hex_ring;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::terrain_ring;
    #[cfg(test)]
    #[path = "../../../../📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    pub mod tests;
}
