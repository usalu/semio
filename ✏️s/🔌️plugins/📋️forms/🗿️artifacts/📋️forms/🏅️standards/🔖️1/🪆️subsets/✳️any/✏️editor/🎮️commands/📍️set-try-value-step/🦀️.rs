//! ⏱️ Hidden bounded Forms vector-value continuation.

use crate::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::commands::set_try_value;
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️SetTryValueStep
pub use set_try_value::SetTryValueStep;

/// ⏱️ Delegates one generation-checked vector-growth chunk.
pub fn handle(payload: &SetTryValueStep, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    set_try_value::handle_step(payload, doc, cfg)
}
//#endregion 🔖️SetTryValueStep
