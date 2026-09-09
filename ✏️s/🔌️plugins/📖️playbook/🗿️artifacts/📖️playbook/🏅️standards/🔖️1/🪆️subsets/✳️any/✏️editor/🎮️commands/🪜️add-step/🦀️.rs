//! 🪜️ 🪜️ Playbook play app commands command — `add-step`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::op::{add_step_operation, PlaybookMutation};
use crate::PlaybookSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-step")]
pub struct AddStep {}

pub fn handle(_payload: &AddStep, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let operation_id = doc.operation()?.operation_id;
    Ok(Emit::mutations(vec![add_step_operation(format!("step-op-{operation_id}"), format!("Step {operation_id}"))]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
