//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-up`.

use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "pointer-up")]
pub struct CanvasPointerUp {}

pub async fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::config(vec![WiresConfigMutation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }]))
}
