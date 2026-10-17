//! 🧬️ schema of `semio-s-artifact-reasoning-wires` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::WiresSnapshot;
pub use crate::schema::mutations::WiresMutation;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_reasoning_wires_snapshot::{GraphEdgeId, GraphNodeId, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA, SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot, WiresContentChild};
pub use crate::standards::v1::subsets::any::schema::artifact_items::{MINDMAP_BOARD_SCHEMA, MINDMAP_WIRES_SCHEMA, WIRES_CONTENT_SLOT, WIRES_DIALECT, WIRES_RELATIONSHIP_PROPERTY, WiresDiff};
pub use crate::standards::v1::subsets::any::schema::artifact_items_3::{SemioGraphMutation, SemioValue, SemioValueEntry, WIRES_DEMO_CONTENT_ID, WIRES_NATIVE_EDGE_KEYS, WIRES_NATIVE_NODE_KEYS, WiresComposed, canonical_board_value, dsl_from_semio_value, empty_camera, empty_wires_content, empty_wires_identity_snapshot, empty_wires_snapshot, genesis_wires_child_pack, semio_value_from_dsl, wires_board_edge, wires_board_node, wires_bundled_contents, wires_child_emit, wires_composed, wires_composed_from_children, wires_content, wires_content_fault, wires_content_handle, wires_content_snapshot, wires_graph_edge, wires_graph_node, wires_properties};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::binary::*;
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::io::binary::snapshot::*;
                            #[cfg(test)]
                            #[path="../../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
pub use semio_s_artifact_reasoning_wires_inferences::examples;
