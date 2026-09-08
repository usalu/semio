//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-down`.

use crate::op::WiresMutation;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_NODE};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "pointer-down")]
pub struct CanvasPointerDown {
    pub id: Option<String>,
    pub x: f64,
    pub y: f64,
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
/// a hit no longer writes `WiresConfigMutation::SetSelection` directly, it asks the host to
/// redispatch `interactionSelect` for the "graph" domain's "node" granularity — the in-flight drag
/// state (`SetDrag`) stays a plain config mutation since it is genuinely app-specific.
pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let document = doc.snapshot;
    match payload.id.as_deref().filter(|id| find_board_node(document, id).is_some()) {
        Some(id) => Ok(Emit {
            config_mutations: vec![WiresConfigMutation::SetDrag(crate::editor::wires::config::SetDrag { node_id: Some(id.to_string()), last_x: payload.x, last_y: payload.y })],
            effects: vec![wires_select_effect(&[id.to_string()], WIRES_GRANULARITY_NODE, "replace")],
            ..Default::default()
        }),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
