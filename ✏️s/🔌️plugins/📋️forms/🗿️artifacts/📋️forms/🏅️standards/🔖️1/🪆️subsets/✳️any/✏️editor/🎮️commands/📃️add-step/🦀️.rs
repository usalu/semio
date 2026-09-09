//! 📃️ 📃️ Forms play app commands command — `add-step`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use crate::schema::create_form_id;
use crate::{forms_steps, op::FormMutation, FormStep, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-step")]
pub struct AddStep {}

pub fn handle(_payload: &AddStep, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let step = FormStep { id: create_form_id("step"), title: format!("Step {}", forms_steps(spec).len() + 1), description: None, blocks: Vec::new() };
    Ok(Emit { artifact_mutations: vec![FormMutation::CreateStep(crate::mutations::create_step::mutation::CreateStep { step, index: None })], config_mutations: reset_try_config_mutations(), ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
