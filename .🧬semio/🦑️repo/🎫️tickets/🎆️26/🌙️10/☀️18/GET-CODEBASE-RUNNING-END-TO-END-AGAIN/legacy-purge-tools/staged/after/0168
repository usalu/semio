//! 🔀️ DAG artifact — the document entity this plugin's app edits.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`reasoning/dag→C:graph`): the old inline
//! `nodes`/`edges` fields are replaced by a composed `s.stdio.semio.graph` CHILD slot
//! (`🔖️ContentBridge` below) — this plugin no longer defines its own persisted node/edge model, it
//! composes stdio's neutral `graph` subset instead. The rich live editing types
//! (`semio_framework_os_infinite_directed_presentation::DagNodeSpec`/`DagNodeKind`/`DagHostSnapshotEdge`) still flow
//! through the app exactly as before; only the PERSISTED shape changed. They now bridge through the
//! composed child's exact local owner rather than plain struct fields.

extern crate infinite_canvas as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_dag_demo_tests;
extern crate semio_framework_schema as framework_schema;

pub use semio_s_artifact_dag_dag_inferences::{DAG_DOCUMENT_SCHEMA};

pub use semio_s_artifact_dag_dag_schema::{DAG_DIALECT};

pub use crate::snapshot::schema::{default_snapshot, empty_snapshot};
pub use semio_framework_artifact_infinite_dag::{DagEdgePatch, DagNodePatch};
pub use semio_framework_os_infinite_directed_presentation::{DagHostSnapshotEdge};

pub use semio_framework_os_infinite_directed_presentation::{DagExpandedPaths, DagNodeKind, DagNodeSpec, DagPreviewContent, IoPortSpec};

pub use semio_s_artifact_dag_dag_inferences::{DAG_EDGE_KIND, DAG_EDGE_NATIVE_FIELDS, DAG_NODE_NATIVE_FIELDS, DagContentChild, DagScene, SemioGraphMutation, dag_content_child_handle, dag_content_snapshot, dag_edge_of_graph, dag_graph_edge, dag_graph_node, dag_node_of_graph, dag_record_properties, dag_scene_of_content, dsl_value_of, semio_value_of};
pub use semio_s_artifact_dag_dag_schema::{dag_scene, dag_scene_from_children};
pub use semio_s_artifact_dag_dag_schema::{dag_child_emit};
pub use semio_s_artifact_dag_dag_inferences::{dag_derivable_scene, genesis_dag_child_pack};

pub use semio_s_artifact_dag_dag_schema::{create_edge_leaf, create_node_leaf, delete_edge_leaf, drag_nodes_leaf, rename_node_leaf};

pub use semio_s_artifact_dag_dag_schema::{DagCamera};

pub use semio_s_artifact_dag_dag_dag::{artifact_kind};

pub use semio_s_artifact_dag_dag_v1::{DagApplication, artifact, definition};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_dag_dag_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_dag_dag_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_dag_dag_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_dag_dag_config::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_dag_dag_diff::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::io::binary::mutations;
    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_dag_dag_config::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub use semio_s_artifact_dag_dag_diff::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_dag_dag_inferences::snapshot;
pub use crate::standards::v1::subsets::any::schema::diff::DagDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::DagMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;

pub use semio_s_artifact_dag_dag_inferences::examples;

/// ✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the mutation-capable surface, migrated
/// wholesale from the retired `🎛️apps/🕸️dag/` app tree into the owned subset's `✏️editor/` facet.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        pub use semio_s_artifact_dag_dag_dag::editor::dag::*;
pub use semio_s_artifact_dag_dag_config::editor::dag::demo_session;

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_dag_dag_config::editor::dag::config::*;
pub mod io {
    pub use semio_s_artifact_dag_dag_inferences::editor::dag::config::io::*;
    pub mod binary {
        pub use semio_s_artifact_dag_dag_diff::editor::dag::config::io::binary::*;
        pub use semio_s_artifact_dag_dag_config::editor::dag::config::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_dag_dag_diff::editor::dag::config::io::text::*;
        pub use semio_s_artifact_dag_dag_config::editor::dag::config::io::text::mutations;
    }
}

            pub use semio_s_artifact_dag_dag_diff::editor::dag::config::schema;
        }

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_dag_dag_diff::editor::dag::presence::*;
pub mod io {
    pub use semio_s_artifact_dag_dag_config::editor::dag::presence::io::*;
    pub mod binary {
        pub use semio_s_artifact_dag_dag_inferences::editor::dag::presence::io::binary::*;
        pub use semio_s_artifact_dag_dag_diff::editor::dag::presence::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_dag_dag_inferences::editor::dag::presence::io::text::*;
        pub use semio_s_artifact_dag_dag_diff::editor::dag::presence::io::text::mutations;
    }
}

            pub use semio_s_artifact_dag_dag_diff::editor::dag::presence::schema;
        }
        pub use semio_s_artifact_dag_dag_diff::editor::dag::terminology;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::add_node;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::connect_media_ports;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::delete_selection;
            pub use semio_s_artifact_dag_dag_schema::editor::dag::commands::disconnect;
            pub use semio_s_artifact_dag_dag_schema::editor::dag::commands::graph_pointer_down;
            pub use semio_s_artifact_dag_dag_dag::editor::dag::commands::move_media_node;
            pub use semio_s_artifact_dag_dag_dag::editor::dag::commands::node_graph_edit;
            pub use semio_s_artifact_dag_dag_schema::editor::dag::commands::node_graph_viewport;
            pub use semio_s_artifact_dag_dag_dag::editor::dag::commands::set_active_example;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::patch_dag_nodes;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::remove_node;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::rename_dag_node;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::*;

                pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::tools;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_dag_dag_diff::editor::dag::modes::edit::windows::compiled;
                    pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::windows::main;
                }
            }
        }

        pub use semio_s_artifact_dag_dag_dag::editor::dag::panels;
    }
}

/// 👁️ The read-only surface (contract §2.2/§2.6) — a genuinely independent module tree from
/// `editor` above, never `#[path]`-mounting anything under `✏️editor/`.
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dag {
        pub use semio_s_artifact_dag_dag_document_behavior_tests::viewer::dag::*;

        pub use semio_s_artifact_dag_dag_diff::viewer::dag::presence;
        pub use semio_s_artifact_dag_dag_diff::viewer::dag::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_dag_dag_document_behavior_tests::viewer::dag::modes::view::*;

                pub use semio_s_artifact_dag_dag_document_behavior_tests::viewer::dag::modes::view::windows;
            }
        }
    }
}

pub use semio_s_artifact_dag_dag_document_behavior_tests::{dag_child_restore_projection};

pub use semio_s_artifact_dag_dag_document_behavior_tests::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
