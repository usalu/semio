//! 🎛️ Process 3d play app commands — the engagement command-line input (a separate system from the
//! utility bar switcher): submit / edit / abort.

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation, Process3dConfigSetEngagementInput};
use crate::editor::process3d::set_active_utility_effect;
use crate::editor::process3d::commands::cursor::{process3d_cursor, process3d_cursor_moves};
use crate::standards::v1::subsets::any::schema::mutations::Process3dMutation;
use crate::{Process3dSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {}

    pub fn handle(
        _payload: &EngagementSubmit,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        let command_word = config.engagement_input.trim().to_lowercase();
        let current = process3d_cursor(fixture, config);
        let clear_input = Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: String::new() });
        let replay = |next: Option<usize>| Emit::config(std::iter::once(clear_input.clone()).chain(process3d_cursor_moves(fixture, config, next)).collect());
        match command_word.split_whitespace().next() {
            Some("cut") => Ok(Emit { config_mutations: vec![clear_input], effects: vec![set_active_utility_effect("cut")], ..Default::default() }),
            Some("drill") => Ok(Emit { config_mutations: vec![clear_input], effects: vec![set_active_utility_effect("drill")], ..Default::default() }),
            Some("attach") => Ok(Emit { config_mutations: vec![clear_input], effects: vec![set_active_utility_effect("attach")], ..Default::default() }),
            Some("back") => Ok(replay(Some(current.saturating_sub(1)))),
            Some("forward") => Ok(replay(Some(current + 1))),
            Some("all") => Ok(replay(None)),
            _ => Ok(Emit::config(vec![clear_input])),
        }
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub fn handle(
        payload: &EngagementInput,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: payload.value.clone() })]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementAbort
pub mod engagement_abort {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "engagement-abort")]
    pub struct EngagementAbort {}

    pub fn handle(
        _payload: &EngagementAbort,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit { config_mutations: vec![Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: String::new() })], effects: vec![set_active_utility_effect("select")], ..Default::default() })
    }
}
//#endregion 🔖️EngagementAbort
