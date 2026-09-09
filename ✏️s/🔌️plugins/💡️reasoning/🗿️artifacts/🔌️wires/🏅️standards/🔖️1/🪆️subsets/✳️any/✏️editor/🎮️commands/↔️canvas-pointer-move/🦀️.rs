//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-move`.

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
}

pub fn handle(_payload: &CanvasPointerMove, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("wires-pointer-move-requires-retained-window-owner"))
}
