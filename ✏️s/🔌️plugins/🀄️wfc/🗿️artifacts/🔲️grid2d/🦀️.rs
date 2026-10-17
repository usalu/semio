//! 🔲️ WFC 2D grid artifact — the `s.wfc.grid2d` document schema: a regular rectangular grid whose
//! cells collapse onto an authored tile catalogue under directional adjacency rules. The persisted
//! document is the PROBLEM only (extent, cell size, periodicity, tiles, rules, pins, mask); the
//! solved assignment, the contradiction verdict and the entropy map are an INFERENCE
//! (`🧬️schema/💡️inferences/🦀️.rs`) driven by the shared `semio_s_plugin_wfc_engine` crate.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

use {semio_framework_artifact_reference::SubsetId};

pub use crate::standards::v1::subsets::any::schema::snapshot::WFC_GRID2D_DOCUMENT_SCHEMA;

pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::{WFC_GRID2D_DIALECT};

pub use semio_s_artifact_wfc_grid2d_v1::{artifact_kind};

pub use semio_s_artifact_wfc_grid2d_v1::{ArtifactApps, artifact, definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_wfc_grid2d_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_wfc_grid2d_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_grid2d_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::schema::snapshot;

                    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::diff;

                    pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::inferences;

                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::inverse;

                        }
                        #[path = "."]
                        pub mod resize_grid {
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::resize_grid::*;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::resize_grid::diff;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::resize_grid::inverse;

                        }
                        #[path = "."]
                        pub mod change_cell_size {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::inverse;

                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::inverse;

                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::inverse;

                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;

                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_rule::inverse;

                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;

                        }
                        #[path = "."]
                        pub mod pin_cell {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::pin_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::pin_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::pin_cell::inverse;

                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::unpin_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::unpin_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::unpin_cell::inverse;

                        }
                        #[path = "."]
                        pub mod mask_cell {
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::mask_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::mask_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::mask_cell::inverse;

                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::unmask_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::unmask_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::unmask_cell::inverse;

                        }
                    }
                }

                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::binary::diff;
}
pub mod sqlite {
    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::text::diff;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

//#region 📚️Examples
/// 📚️ The bundled tile sets this subset ships — a five-tile vector PIPES set (rotation variants as
/// separate tiles, exactly the way an authored tiled model states them) and a three-tile bitmap
/// TERRAIN set with grass/sand/water transitions.
pub use semio_s_artifact_wfc_grid2d_diff::examples;
//#endregion 📚️Examples

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid2d {
        pub use semio_s_artifact_wfc_grid2d_v1::editor::grid2d::*;

        pub use semio_s_artifact_wfc_grid2d_diff::editor::grid2d::presence;
        pub use semio_s_artifact_wfc_grid2d_diff::editor::grid2d::config;

        pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::window;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::*;

                pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::windows;

                pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::tools;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid2d {
        pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::viewer::grid2d::*;

        pub use semio_s_artifact_wfc_grid2d_diff::viewer::grid2d::presence;
        pub use semio_s_artifact_wfc_grid2d_diff::viewer::grid2d::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_grid2d_create_tile::viewer::grid2d::modes::view::*;

                pub use semio_s_artifact_wfc_grid2d_diff::viewer::grid2d::modes::view::windows;
            }
        }
    }
}

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
pub use crate::standards::v1::subsets::any::schema::diff::Grid2dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Grid2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::{
    Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcColor, WfcDirection2d, WfcPathSegment, WfcPinnedCell2d, WfcPoint2, WfcTile2d, WfcTileMedia2d, WfcVectorPath,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]
mod store_fixture;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;

//#endregion 🧪️Tests

#[path="🔨️modules/🏠️host/🦀️.rs"]
pub mod host;

pub use host::inferences::*;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
