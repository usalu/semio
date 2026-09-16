//! 🖱️ 🖱️ Layout play app commands command — `canvas-pointer-up`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {
    /// 🚫️ `true` when the host closed the gesture without a release (pointer left the canvas,
    /// capture lost — design §2 D). Layout keeps no drag gesture on this surface, so a cancel and
    /// a release are both no-ops; the flag is carried so a future gesture honours it.
    #[value(default)]
    pub cancelled: bool,
}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
