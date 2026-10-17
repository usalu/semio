//! 🧊️ WFC 3d artifact — an ARBITRARY slot graph in space (`s.wfc.wfc3d`): slots are boxes placed
//! freely at `x`/`y`/`z`, adjacency is an explicit edge list carrying a named `relation`, and the
//! placeable catalogue is mesh tiles. No grid assumption anywhere; the regular-box sibling is
//! `🧱️grid3d`. The SOLVE is an inference over this spec (`🧬️schema/💡️inferences/🦀️.rs`), never
//! persisted state — the editor's preview window renders the inferred assignment and the document
//! keeps only the problem.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::WFC3D_DOCUMENT_SCHEMA;

use crate::schema::snapshot::{Color, TileMedia3d};

pub use semio_s_artifact_wfc_3d_wfc3d::{WFC3D_DIALECT};

pub use semio_s_artifact_wfc_3d_wfc3d::{artifact_kind};

pub use semio_s_artifact_wfc_3d_snapshot::{mesh_child_handle, unit_box_media, unit_wedge_media};

pub use semio_s_artifact_wfc_3d_tests::{ArtifactApps, artifact, definition, wfc3d_languages};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_wfc_3d_tests::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_wfc_3d_tests::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_3d_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod create_slot {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_slot::inverse;

                        }
                        #[path = "."]
                        pub mod delete_slot {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_slot::inverse;

                        }
                        #[path = "."]
                        pub mod move_slot {
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::*;
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::diff;
                            pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::mutations::move_slot::inverse;

                        }
                        #[path = "."]
                        pub mod drag_slots {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::inverse;

                        }
                        #[path = "."]
                        pub mod set_slot_positions {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::set_slot_positions::inverse;

                        }
                        #[path = "."]
                        pub mod resize_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::resize_slot::inverse;

                        }
                        #[path = "."]
                        pub mod connect_slots {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::connect_slots::inverse;

                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::disconnect_slots::inverse;

                        }
                        #[path = "."]
                        pub mod pin_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::pin_slot::inverse;

                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::unpin_slot::inverse;

                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::inverse;

                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_3d_set_slot_positions::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;

                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;

                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::inverse;

                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;

                        }
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::change_seed::inverse;

                        }
                    }
                    pub use semio_s_artifact_wfc_3d_inferences::standards::v1::subsets::any::schema::inferences;
                }

                pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::examples;

                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_wfc_3d_tests::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_wfc_3d_schema::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::import;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::io::export;
                }
            }
        }
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc3d {
        pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::*;

        pub use semio_s_artifact_wfc_3d_schema::editor::wfc3d::presence;

        pub mod config {
    pub use semio_s_artifact_wfc_3d_config::editor::wfc3d::config::*;
    pub use semio_s_artifact_wfc_3d_schema::editor::wfc3d::config::schema;
}
        pub use semio_s_artifact_wfc_3d_transient::editor::wfc3d::transient;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::*;

                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::tools::fill;
                    pub use semio_s_artifact_wfc_3d_inferences::editor::wfc3d::modes::edit::tools::drag;
                }

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_3d_schema::editor::wfc3d::modes::edit::windows::graph;
                    pub use semio_s_artifact_wfc_3d_wfc3d::editor::wfc3d::modes::edit::windows::preview;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wfc3d {
        pub use semio_s_artifact_wfc_3d_tests::viewer::wfc3d::*;

        pub use semio_s_artifact_wfc_3d_schema::viewer::wfc3d::presence;
        pub use semio_s_artifact_wfc_3d_schema::viewer::wfc3d::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_3d_wfc3d::viewer::wfc3d::modes::view::*;

                pub use semio_s_artifact_wfc_3d_wfc3d::viewer::wfc3d::modes::view::windows;
            }
        }
    }
}

//#region 🔗️FlatShims
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
    #[cfg(test)] pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
}

pub use crate::standards::v1::subsets::any::schema::diff::Wfc3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Wfc3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc3dSnapshot;

//#endregion 🔗️FlatShims

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;

#[cfg(test)]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🗄️store-fixture/🦀️.rs"]
mod store_fixture;

#[cfg(test)]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
mod examples_outcome;
//#endregion 🧪️Tests

pub use crate::standards::v1::subsets::any::io::{Wfc3dBuilderConstruction, Wfc3dParts, Wfc3dAnalyzerAnalysis, Wfc3dBuilderFacets, Wfc3dBuilder, Wfc3dAnalyzer, Wfc3dComposer};

#[path="🔨️modules/🏠️host/🦀️.rs"]
pub mod host;

pub use host::inferences::*;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
