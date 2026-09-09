//! 🔵️ 🔵️ Wires play app commands command — `add-node`.

use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_NODE};
use crate::op::WiresMutation;
use crate::schema::fixture_nodes;
use crate::WiresSnapshot;
use dsl::DslValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub kind: String,
}

/// 🕹️ Creates the node and selects it through the framework interaction effect.
pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let kind = if payload.kind.is_empty() { "identity" } else { payload.kind.as_str() };
    let id = format!("node-{}", fixture_nodes(&crate::wires_working_board(document)).len() + 1);
    let node = DslValue::object([
        ("id".into(), DslValue::String(id.clone())),
        ("nodeKind".into(), DslValue::String(kind.into())),
        ("shape".into(), DslValue::String("circle".into())),
        ("x".into(), DslValue::float(0.0)),
        ("y".into(), DslValue::float(0.0)),
        ("radius".into(), DslValue::float(24.0)),
        ("text".into(), DslValue::String(id.clone())),
        ("handles".into(), DslValue::Array(vec![])),
    ]);
    Ok(Emit { artifact_mutations: vec![crate::mutations::create_node(node)], effects: vec![wires_select_effect(&[id], WIRES_GRANULARITY_NODE, "replace")], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
