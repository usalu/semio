//! ✏️ wires of `semio-s-artifact-reasoning-wires` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::mutations::WiresMutation;
pub use crate::schema::snapshot::WiresSnapshot;
pub use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphNodeId;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphEdgeId;
pub use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
pub use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphEdge;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphNode;
pub use semio_s_artifact_stdio_semio_graph::schema::snapshot::STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_reasoning_wires_schema::{MINDMAP_BOARD_SCHEMA, MINDMAP_WIRES_SCHEMA, WIRES_CONTENT_SLOT, WIRES_DIALECT, WIRES_RELATIONSHIP_PROPERTY, WiresComposed, empty_wires_content, empty_wires_snapshot, genesis_wires_child_pack, semio_value_from_dsl, wires_bundled_contents, wires_child_emit, wires_composed, wires_composed_from_children, wires_content_fault, wires_content_handle, wires_content_snapshot};
pub use semio_s_artifact_reasoning_wires_viewer::{wires_child_restore_projection};
pub use crate::editor::wires::artifact_items_4::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
