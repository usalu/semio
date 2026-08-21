//! 🎛️ Process 3d play app commands — the engagement command-line input (a separate system from the
//! utility bar switcher): submit / edit / abort.

use crate::artifacts::process3d::mutations::change_cursor::mutation::ChangeCursor;
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::editor::process3d::set_active_utility_effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {}

    pub async fn handle(
        _payload: &EngagementSubmit,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        let command_word = config.engagement_input.trim().to_lowercase();
        // 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `steps` is a composed
        // CHILD HANDLE now (no `.len()` — see `ProcessWorkingScene`'s doc comment); `forward`'s
        // upper clamp against the real step count is dropped honestly, matching `⏱️cursor`'s own
        // commands.
        let current = fixture.resolved_up_to.unwrap_or(0);
        let clear_input = Process3dConfigMutation::SetEngagementInput { value: String::new() };
        match command_word.split_whitespace().next() {
            Some("cut") => Ok(Emit { config_mutations: vec![clear_input], effects: vec![set_active_utility_effect("cut")], ..Default::default() }),
            Some("drill") => Ok(Emit { config_mutations: vec![clear_input], effects: vec![set_active_utility_effect("drill")], ..Default::default() }),
            Some("attach") => Ok(Emit { config_mutations: vec![clear_input], effects: vec![set_active_utility_effect("attach")], ..Default::default() }),
            Some("back") => Ok(Emit { artifact_mutations: vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(current.saturating_sub(1)) })], config_mutations: vec![clear_input], ..Default::default() }),
            Some("forward") => Ok(Emit { artifact_mutations: vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(current + 1) })], config_mutations: vec![clear_input], ..Default::default() }),
            Some("all") => Ok(Emit { artifact_mutations: vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: None })], config_mutations: vec![clear_input], ..Default::default() }),
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

    pub async fn handle(
        payload: &EngagementInput,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementAbort
pub mod engagement_abort {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-abort")]
    pub struct EngagementAbort {}

    pub async fn handle(
        _payload: &EngagementAbort,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit { config_mutations: vec![Process3dConfigMutation::SetEngagementInput { value: String::new() }], effects: vec![set_active_utility_effect("select")], ..Default::default() })
    }
}
//#endregion 🔖️EngagementAbort
