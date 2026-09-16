//! 👁️ 👁️ Generation2d play app commands command — `canvas-pointer-up`.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {
    /// 🚫️ `true` when the host closed the gesture without a release (pointer left the canvas,
    /// capture lost — design §2 D). This app keeps no drag gesture on its canvas, so a cancel and
    /// a release are both no-ops; the flag is carried so a future gesture honours it.
    #[value(default)]
    pub cancelled: bool,
}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    Ok(Emit::default())
}
