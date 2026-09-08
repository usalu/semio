//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-up`.

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "pointer-up")]
pub struct CanvasPointerUp {}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::config(vec![WiresConfigMutation::SetDrag(crate::editor::wires::config::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 })]))
}
