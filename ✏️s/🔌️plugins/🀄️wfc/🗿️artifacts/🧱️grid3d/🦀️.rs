//! 🧱️ WFC 3D grid artifact — `s.wfc.grid3d`: a regular box grid with NON-UNIFORM per-axis cell
//! sizes, six fixed face neighbours, mesh tiles and a closed adjacency allow-list. The document
//! persists only the PROBLEM (extent, cell sizes, periodicity, tiles, rules, pins, masks); the
//! SOLVE is an inference (`…/🧬️schema/💡️inferences/🦀️.rs`) driving the shared
//! `semio_s_plugin_wfc_engine` `WfcJob` over a `Grid3dTopology`, republished into the editor's
//! preview window and never written back into the snapshot.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::WFC_GRID3D_DOCUMENT_SCHEMA;

pub use semio_s_artifact_wfc_grid3d_editor::{WFC_GRID3D_DIALECT};

pub use semio_s_artifact_wfc_grid3d_editor::{artifact_kind};

pub use semio_s_artifact_wfc_grid3d_snapshot::{mesh_child_handle};

pub use semio_s_artifact_wfc_grid3d_tests::{ArtifactApps, artifact, definition, pilot_languages};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_wfc_grid3d_tests::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_wfc_grid3d_tests::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_grid3d_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_grid3d_inferences::standards::v1::subsets::any::schema::scene_internals;
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::inverse;

                        }
                        #[path = "."]
                        pub mod resize_grid {
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::resize_grid::*;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::resize_grid::diff;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::resize_grid::inverse;

                        }
                        #[path = "."]
                        pub mod change_cell_sizes {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::inverse;

                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::inverse;

                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::inverse;

                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;

                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::inverse;

                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;

                        }
                        #[path = "."]
                        pub mod pin_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::inverse;

                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unpin_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unpin_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unpin_cell::inverse;

                        }
                        #[path = "."]
                        pub mod mask_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::mask_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::mask_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::mask_cell::inverse;

                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unmask_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unmask_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unmask_cell::inverse;

                        }
                    }
                    pub use semio_s_artifact_wfc_grid3d_inferences::standards::v1::subsets::any::schema::inferences;
                }

                pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples;

                pub mod io {
    pub use semio_s_artifact_wfc_grid3d_tests::standards::v1::subsets::any::io::*;
    pub mod binary {
        pub use semio_s_artifact_wfc_grid3d_tests::standards::v1::subsets::any::io::binary::*;
        pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::io::binary::diff;
    }
    pub mod text {
        pub use semio_s_artifact_wfc_grid3d_tests::standards::v1::subsets::any::io::text::*;
        pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::io::text::diff;
    }
}
            }
        }
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid3d {
        pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::*;

        pub use semio_s_artifact_wfc_grid3d_schema::editor::grid3d::presence;
        pub use semio_s_artifact_wfc_grid3d_schema::editor::grid3d::config;

        pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::window;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::edit::*;

                pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::edit::tools;

                pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid3d {
        pub use semio_s_artifact_wfc_grid3d_tests::viewer::grid3d::*;

        pub use semio_s_artifact_wfc_grid3d_schema::viewer::grid3d::presence;
        pub use semio_s_artifact_wfc_grid3d_schema::viewer::grid3d::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_grid3d_editor::viewer::grid3d::modes::view::*;

                pub use semio_s_artifact_wfc_grid3d_editor::viewer::grid3d::modes::view::windows;
            }
        }
    }
}

/// 🪞️ Flat access from the artifact root, mirroring generation3d: the subset's schema tree reached as
/// `crate::schema`, with the sibling shims below doing the same for diff, mutations and inferences.
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
}
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
}
pub use crate::standards::v1::subsets::any::schema::diff::Grid3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Grid3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Grid3dSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;

#[cfg(test)]
#[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]
mod store_fixture;

#[cfg(test)]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
mod examples_outcome;

//#endregion 🧪️Tests

pub use crate::standards::v1::subsets::any::io::{Grid3dBuilderConstruction, Grid3dParts, Grid3dAnalyzerAnalysis, Grid3dBuilderFacets, Grid3dBuilder, Grid3dAnalyzer, Grid3dComposer};

#[path="🔨️modules/🏠️host/🦀️.rs"]
pub mod host;

pub use host::inferences::*;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
