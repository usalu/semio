//! 🎛️ Process 3d play app commands — the engagement command-line input (a separate system from the
//! utility bar switcher): submit / edit / abort.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::apps::process3d::set_active_utility_effect;
use crate::artifacts::process3d::{op::Process3dOperation, Process3dDocument};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {}

    pub fn handle(_payload: &EngagementSubmit, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        let command_word = config.engagement_input.trim().to_lowercase();
        let len = fixture.steps.len();
        let current = fixture.resolved_up_to.unwrap_or(len);
        let clear_input = Process3dConfigOperation::SetEngagementInput { value: String::new() };
        match command_word.split_whitespace().next() {
            Some("cut") => Ok(Emit { config_operations: vec![clear_input], effects: vec![set_active_utility_effect("cut")], ..Default::default() }),
            Some("drill") => Ok(Emit { config_operations: vec![clear_input], effects: vec![set_active_utility_effect("drill")], ..Default::default() }),
            Some("attach") => Ok(Emit { config_operations: vec![clear_input], effects: vec![set_active_utility_effect("attach")], ..Default::default() }),
            Some("back") => Ok(Emit { document_operations: vec![Process3dOperation::SetCursor { resolved_up_to: Some(current.saturating_sub(1)) }], config_operations: vec![clear_input], ..Default::default() }),
            Some("forward") => Ok(Emit { document_operations: vec![Process3dOperation::SetCursor { resolved_up_to: Some((current + 1).min(len)) }], config_operations: vec![clear_input], ..Default::default() }),
            Some("all") => Ok(Emit { document_operations: vec![Process3dOperation::SetCursor { resolved_up_to: None }], config_operations: vec![clear_input], ..Default::default() }),
            _ => Ok(Emit::config(vec![clear_input])),
        }
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &EngagementInput, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigOperation::SetEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementAbort
pub mod engagement_abort {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-abort")]
    pub struct EngagementAbort {}

    pub fn handle(_payload: &EngagementAbort, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        Ok(Emit { config_operations: vec![Process3dConfigOperation::SetEngagementInput { value: String::new() }], effects: vec![set_active_utility_effect("select")], ..Default::default() })
    }
}
//#endregion 🔖️EngagementAbort
