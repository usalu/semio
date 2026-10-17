//! 🧠️ Wires artifact — the document entity this plugin's one app (🔌️wires) edits.
//!
//! The board (nodes and edges) lives ONLY in the composed `s.stdio.semio@v1/graph` child `content` (ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §20.15): every board edit is a child-lane graph leaf in that child's
//! store, the parent owns no leaf (`WiresMutation` is uninhabited), and every reader composes the parent's `wires_snapshot`
//! (identities) with the child on read ([`wires_composed`]). A decoded, reloaded or remote parent therefore needs no
//! materialization step.

#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_wires_demo_tests;
extern crate semio_framework_schema as framework_schema;

use semio_framework_value::DslValue;

pub use semio_s_artifact_reasoning_wires_schema::{MINDMAP_BOARD_SCHEMA, MINDMAP_WIRES_SCHEMA, WIRES_CONTENT_SLOT, WIRES_DIALECT, WIRES_RELATIONSHIP_PROPERTY, WiresDiff, WiresMutation};

//#region 🔖️Types
/// 📸️ Persisted wires snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::schema::snapshot::WiresSnapshot;
pub use crate::schema::WiresArtifact;
//#endregion 🔖️Types

pub use semio_s_artifact_reasoning_wires_schema::{SemioGraphMutation, SemioValue, SemioValueEntry, WIRES_DEMO_CONTENT_ID, WIRES_NATIVE_EDGE_KEYS, WIRES_NATIVE_NODE_KEYS, WiresComposed, canonical_board_value, dsl_from_semio_value, empty_camera, empty_wires_content, empty_wires_identity_snapshot, empty_wires_snapshot, genesis_wires_child_pack, semio_value_from_dsl, wires_board_edge, wires_board_node, wires_bundled_contents, wires_child_emit, wires_composed, wires_composed_from_children, wires_content, wires_content_fault, wires_content_handle, wires_content_snapshot, wires_graph_edge, wires_graph_node, wires_properties};

pub use semio_s_artifact_reasoning_wires_snapshot::{GraphEdgeId, GraphNodeId, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA, SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot, WiresContentChild};

pub use semio_s_artifact_reasoning_wires_wires::{artifact_kind};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
pub use semio_s_artifact_reasoning_wires_v1::{WiresApplication, artifact, definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_reasoning_wires_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_reasoning_wires_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_reasoning_wires_snapshot_2::snapshot;

pub use semio_s_artifact_reasoning_wires_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wires {
        pub use semio_s_artifact_reasoning_wires_wires::editor::wires::*;
pub use semio_s_artifact_reasoning_wires_inferences::editor::wires::demo_session;

        pub use semio_s_artifact_reasoning_wires_inferences::editor::wires::terminology;

        pub use semio_s_artifact_reasoning_wires_wires::editor::wires::commands;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_reasoning_wires_viewer::editor::wires::modes::edit::*;

                pub use semio_s_artifact_reasoning_wires_viewer::editor::wires::modes::edit::tools;

                pub use semio_s_artifact_reasoning_wires_windows::editor::wires::modes::edit::windows;
            }
        }

        pub use semio_s_artifact_reasoning_wires_wires::editor::wires::panels;
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wires {
        pub use semio_s_artifact_reasoning_wires_viewer::viewer::wires::*;

        pub use semio_s_artifact_reasoning_wires_inferences::viewer::wires::presence;
        pub use semio_s_artifact_reasoning_wires_inferences::viewer::wires::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_reasoning_wires_viewer::viewer::wires::modes::view::*;

                pub use semio_s_artifact_reasoning_wires_viewer::viewer::wires::modes::view::windows;
            }
        }
    }
}

pub use semio_s_artifact_reasoning_wires_viewer::{wires_child_restore_projection};

pub use crate::standards::v1::subsets::any::io::{WiresBuilderConstruction};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
