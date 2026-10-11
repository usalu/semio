//! 🖱️ 🖱️ VCS play app commands command — `canvas-pointer-up`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {
    /// 🚫️ `true` when the host closed the gesture without a release (pointer left the canvas,
    /// capture lost — design §2 D). This app keeps no drag gesture on its canvas, so a cancel and
    /// a release are both no-ops; the flag is required on the wire and carried so a future gesture honours it.
    pub cancelled: bool,
}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(Emit::default())
}
