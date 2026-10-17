//! 🔬️ tests of `semio-s-artifact-wfc-grid3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::snapshot::Grid3dSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::Grid3dMutation;
pub use crate::schema::snapshot::WFC_GRID3D_DOCUMENT_SCHEMA;
pub use crate::standards::v1::subsets::any::schema::diff::Grid3dDiff;
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_wfc_grid3d_snapshot::{mesh_child_handle};
pub use semio_s_artifact_wfc_grid3d_editor::{WFC_GRID3D_DIALECT};
pub use crate::standards::v1::artifact_items_4::{ArtifactApps, artifact, definition, pilot_languages};

#[path = "."]
pub mod host {
    pub use semio_s_artifact_wfc_grid3d_inferences::host::inferences;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_grid3d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod scene_internals {
                        pub use semio_s_artifact_wfc_grid3d_inferences::standards::v1::subsets::any::schema::scene_internals::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod unpin_cell {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/📍️unpin-cell/🧪️tests/📍️releases-the-origin-cell/🦀️.rs"]
                            pub mod tests_releases_the_origin_cell;
                        }
                        #[path = "."]
                        pub mod mask_cell {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🚫️mask-cell/🧪️tests/🚫️carves/🦀️.rs"]
                            pub mod tests_carves_out_the_far_edge_cell;
                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            #[cfg(test)]
                            #[path = "../../../../../🧬️mutations/🔓️unmask-cell/🧪️tests/🔓️restores/🦀️.rs"]
                            pub mod tests_restores_the_masked_corner;
                        }
                    }
                    pub use semio_s_artifact_wfc_grid3d_inferences::standards::v1::subsets::any::schema::inferences;
                }
                pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples;
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
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
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::*;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::edit::*;
                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::edit::tools::*;
                    #[path = "."]
                    pub mod fill {
                        pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::edit::tools::fill::*;
                        #[cfg(test)]
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                pub use semio_s_artifact_wfc_grid3d_editor::editor::grid3d::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid3d {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
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
