//! ⏱️ Hidden dispatch surface for one bounded duplicate-widget continuation step.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::commands::duplicate_widget;
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
pub use duplicate_widget::DuplicateWidgetStep;
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &DuplicateWidgetStep, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, eval: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    duplicate_widget::handle_step(payload, doc, cfg, eval)
}
//#endregion 🔖️Handler
