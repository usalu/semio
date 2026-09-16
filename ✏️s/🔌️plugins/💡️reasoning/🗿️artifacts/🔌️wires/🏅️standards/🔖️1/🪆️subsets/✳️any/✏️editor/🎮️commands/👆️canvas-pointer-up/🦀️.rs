//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-up`.

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "pointer-up")]
pub struct CanvasPointerUp {
    /// 🚫️ `true` when the host closed the gesture without a release (pointer left the canvas,
    /// capture lost — design §2 D): a live node drag is cleared and the node is NOT moved.
    #[value(default)]
    pub cancelled: bool,
}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("wires-pointer-up-requires-retained-window-owner"))
}
