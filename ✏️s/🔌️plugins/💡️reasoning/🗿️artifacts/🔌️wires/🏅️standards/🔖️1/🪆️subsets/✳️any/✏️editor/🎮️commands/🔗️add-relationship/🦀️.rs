//! 🔗️ 🔗️ Wires play app commands command — `add-relationship`.

use crate::op::WiresMutation;
use crate::schema::fixture_edges;
use crate::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_EDGE};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-relationship")]
pub struct AddRelationship {
    pub kind: String,
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
/// the newly created edge is selected via a requested `interactionSelect` effect instead of a
/// `WiresConfigMutation::SetSelection`.
pub fn handle(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let document = doc.snapshot;
    let kind = if payload.kind.is_empty() { "owns" } else { payload.kind.as_str() };
    let edge_id = format!("edge-{}", fixture_edges(&crate::wires_working_board(document)).len() + 1);
    let edge = DslValue::object([
        ("id".into(), DslValue::String(edge_id.clone())),
        ("edgeKind".into(), DslValue::String(format!("wires.{kind}"))),
        ("source".into(), DslValue::String("node-1".into())),
        ("target".into(), DslValue::String("node-2".into())),
    ]);
    let relationship = DslValue::object([
        ("edgeId".into(), DslValue::String(edge_id.clone())),
        ("kind".into(), DslValue::String(kind.into())),
        ("sourceIdentityId".into(), DslValue::uint(1)),
        ("targetIdentityId".into(), DslValue::uint(2)),
    ]);
    Ok(Emit { artifact_mutations: vec![crate::mutations::connect_nodes(edge, relationship)], effects: vec![wires_select_effect(&[edge_id], WIRES_GRANULARITY_EDGE, "replace")], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
