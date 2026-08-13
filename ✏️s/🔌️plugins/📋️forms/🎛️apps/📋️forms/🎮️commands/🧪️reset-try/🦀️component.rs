//! 🧪️ 🧪️ Forms play app commands command — `reset-try`.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::{effective_try_values, parse_value_json, reset_try_config_mutations, try_values_json_text, try_values_map};
use crate::artifacts::forms::schema::can_advance;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "reset-try")]
pub struct ResetTry {}

pub fn handle(_payload: &ResetTry, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::config(reset_try_config_mutations()))
}
