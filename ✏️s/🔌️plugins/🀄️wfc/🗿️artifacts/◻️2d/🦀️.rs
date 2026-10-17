//! ◻️ WFC 2D artifact — an ARBITRARY slot graph in the plane: slots are free rectangles, adjacency is
//! an explicit edge list with named relation classes, and tiles are flat 2D media. It is the
//! non-rectangular sibling of `grid2d` and the 2D twin of `wfc3d`, and it descends directly from the
//! procedural `assembly` artifact (slots/edges/rules/seed), with kit-module handles replaced by
//! inline tile media and a single implicit relation replaced by named relation classes.
//!
//! The SOLVE is never persisted: it is an inference (`…/🧬️schema/💡️inferences/🦀️.rs`) over this
//! document, driven by the shared `semio-s-plugin-wfc-engine` graph route.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::{WFC_2D_DEFAULT_RELATION, WFC_2D_DOCUMENT_SCHEMA};

pub use semio_s_artifact_wfc_2d_inferences::{WFC_2D_DIALECT};

pub use semio_s_artifact_wfc_2d_v1::{artifact_kind};

pub use semio_s_artifact_wfc_2d_v1::{ArtifactApps, artifact, definition};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_wfc_2d_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_wfc_2d_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::inverse;

                        }
                        #[path = "."]
                        pub mod create_slot {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_slot::inverse;

                        }
                        #[path = "."]
                        pub mod delete_slot {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_slot::inverse;

                        }
                        #[path = "."]
                        pub mod move_slot {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::move_slot::inverse;

                        }
                        #[path = "."]
                        pub mod drag_slots {
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::*;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::diff;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::drag_slots::inverse;

                        }
                        #[path = "."]
                        pub mod set_slot_positions {
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::set_slot_positions::*;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::set_slot_positions::diff;
                            pub use semio_s_artifact_wfc_2d_mutations::standards::v1::subsets::any::schema::mutations::set_slot_positions::inverse;

                        }
                        #[path = "."]
                        pub mod resize_slot {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::resize_slot::inverse;

                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;

                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::*;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::diff;
                            pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::schema::mutations::disconnect_slots::inverse;

                        }
                        #[path = "."]
                        pub mod pin_slot {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::pin_slot::inverse;

                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::unpin_slot::inverse;

                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_tile::inverse;

                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_2d_pin_slot::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;

                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::create_rule::inverse;

                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_2d_mut_any::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;

                        }
                    }
                    pub use semio_s_artifact_wfc_2d_inferences::standards::v1::subsets::any::schema::inferences;
                }

                pub mod io {
    pub use semio_s_artifact_wfc_2d_v1::standards::v1::subsets::any::io::*;
    pub mod sqlite {
        pub use semio_s_artifact_wfc_2d_diff::standards::v1::subsets::any::io::sqlite::*;
        pub use semio_s_artifact_wfc_2d_schema::standards::v1::subsets::any::io::sqlite::snapshot;
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
        pub use semio_s_artifact_wfc_2d_v1::editor::wfc2d::*;

        pub use semio_s_artifact_wfc_2d_diff::editor::wfc2d::presence;

        pub mod config {
    pub use semio_s_artifact_wfc_2d_config::editor::wfc2d::config::*;
    pub use semio_s_artifact_wfc_2d_diff::editor::wfc2d::config::schema;
}
        pub use semio_s_artifact_wfc_2d_transient::editor::wfc2d::transient;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_2d_v1::editor::wfc2d::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_2d_diff::editor::wfc2d::modes::edit::windows::graph;
                    pub use semio_s_artifact_wfc_2d_v1::editor::wfc2d::modes::edit::windows::preview;
                }

                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_wfc_2d_v1::editor::wfc2d::modes::edit::tools::fill;
                    pub use semio_s_artifact_wfc_2d_inferences::editor::wfc2d::modes::edit::tools::drag;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wfc2d {
        pub use semio_s_artifact_wfc_2d_inferences::viewer::wfc2d::*;

        pub use semio_s_artifact_wfc_2d_diff::viewer::wfc2d::presence;
        pub use semio_s_artifact_wfc_2d_diff::viewer::wfc2d::config;

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

//#region 📚️Examples
/// 📚️ The bundled WFC problems this subset ships — a forced path, a cyclic two-relation lattice, a
/// non-rectangular vector ring, and the same ring over RASTER tiles.
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::*;

    pub use semio_s_artifact_wfc_2d_pin_slot::examples::two_room_corridor;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::wall_roof_facade_strip;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::hex_ring;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::terrain_ring;

}
//#endregion 📚️Examples

/// 🪞️ Flat access from the artifact root, mirroring the sibling wfc artifacts: the subset's schema
/// tree reached as `crate::schema`, with the shims below doing the same for diff, mutations and
/// inferences.
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
    pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
}
pub use crate::standards::v1::subsets::any::schema::diff::Wfc2dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Wfc2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc2dSnapshot;

#[cfg(test)]
#[path = "./🧪️tests/🔬️store-fixture/🦀️.rs"]
mod store_fixture;

#[cfg(test)]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;

#[path="🔨️modules/🏠️host/🦀️.rs"]
pub mod host;

pub use host::inferences::*;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🗺️terrain-ring/🧪️tests/🧩️example/🦀️.rs"]
mod hoisted_tests_2;
#[cfg(test)]
#[path = "🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_3;
