//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-down`.

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "pointer-down")]
pub struct CanvasPointerDown {
    pub id: Option<String>,
    pub x: f64,
    pub y: f64,
}

/// 🪟️ Canvas hits are resolved incrementally by the retained concrete-window owner.
pub fn handle(_payload: &CanvasPointerDown, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("wires-pointer-down-requires-retained-window-owner"))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
