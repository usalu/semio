//! 👁️ viewer of `semio-s-artifact-reasoning-wires` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphNodeId;
pub use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
pub use crate::schema::snapshot::WiresSnapshot;
pub use crate::schema::mutations::WiresMutation;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphEdgeId;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphEdge;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphNode;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_reasoning_wires_schema::{MINDMAP_WIRES_SCHEMA, WIRES_CONTENT_SLOT, WIRES_DEMO_CONTENT_ID, WIRES_DIALECT, WiresComposed, empty_wires_content, empty_wires_snapshot, genesis_wires_child_pack, wires_bundled_contents, wires_composed, wires_composed_from_children, wires_content_fault};
pub use crate::viewer::wires::artifact_items_6::{wires_child_restore_projection};

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
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path="../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_reasoning_wires_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wires {
        #[path = "."]
        pub mod window_transient {
            pub use semio_s_artifact_reasoning_wires_window_transient::editor::wires::window_transient::*;
            #[cfg(test)]
            #[path = "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️contract-vectors/🦀️.rs"]
            pub mod contract_vectors;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod tools {
                    #[path = "../../../✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"]
                    pub mod reorganize;
                }
                pub use semio_s_artifact_reasoning_wires_windows::editor::wires::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wires {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../🎭️modes/👁️view/🪟️windows/🕸️canvas/🦀️.rs"]
                    pub mod canvas;
                }
            }
        }
    }
}
