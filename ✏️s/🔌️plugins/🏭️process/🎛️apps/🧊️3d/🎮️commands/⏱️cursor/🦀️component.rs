//! ⏱️ Process 3d play app commands — the process timeline cursor (`resolved_up_to`), NOT framework
//! History — these move the replay cursor.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCursor
pub mod set_cursor {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "cursor")]
    pub struct SetCursor {
        pub value: Option<u64>,
    }

    pub fn handle(payload: &SetCursor, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let resolved = payload.value.map(|n| (n as usize).min(fixture.steps.len()));
        Ok(Emit::mutations(vec![Process3dMutation::SetCursor { resolved_up_to: resolved }]))
    }
}
//#endregion 🔖️SetCursor

//#region 🔖️StepCursor
pub mod step_cursor {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor")]
    pub struct StepCursor {
        pub delta: i64,
    }

    pub fn handle(payload: &StepCursor, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let len = fixture.steps.len();
        let current = fixture.resolved_up_to.unwrap_or(len) as i64;
        Ok(Emit::mutations(vec![Process3dMutation::SetCursor { resolved_up_to: Some((current + payload.delta).clamp(0, len as i64) as usize) }]))
    }
}
//#endregion 🔖️StepCursor

//#region 🔖️StepCursorBack
pub mod step_cursor_back {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor-back")]
    pub struct StepCursorBack {}

    pub fn handle(_payload: &StepCursorBack, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let len = fixture.steps.len();
        let current = fixture.resolved_up_to.unwrap_or(len) as i64;
        Ok(Emit::mutations(vec![Process3dMutation::SetCursor { resolved_up_to: Some((current - 1).clamp(0, len as i64) as usize) }]))
    }
}
//#endregion 🔖️StepCursorBack

//#region 🔖️StepCursorForward
pub mod step_cursor_forward {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor-forward")]
    pub struct StepCursorForward {}

    pub fn handle(_payload: &StepCursorForward, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let len = fixture.steps.len();
        let current = fixture.resolved_up_to.unwrap_or(len) as i64;
        Ok(Emit::mutations(vec![Process3dMutation::SetCursor { resolved_up_to: Some((current + 1).clamp(0, len as i64) as usize) }]))
    }
}
//#endregion 🔖️StepCursorForward
