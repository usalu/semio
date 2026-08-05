//! ⏱️ Process 3d play app commands — the process timeline cursor (`resolved_up_to`), NOT framework
//! History — these move the replay cursor.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::artifacts::process3d::{op::Process3dOperation, Process3dDocument};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCursor
pub mod set_cursor {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "cursor")]
    pub struct SetCursor {
        pub value: Option<u64>,
    }

    pub fn handle(payload: &SetCursor, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let resolved = payload.value.map(|n| (n as usize).min(fixture.steps.len()));
        Ok(Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: resolved }]))
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

    pub fn handle(payload: &StepCursor, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let len = fixture.steps.len();
        let current = fixture.resolved_up_to.unwrap_or(len) as i64;
        Ok(Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: Some((current + payload.delta).clamp(0, len as i64) as usize) }]))
    }
}
//#endregion 🔖️StepCursor

//#region 🔖️StepCursorBack
pub mod step_cursor_back {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor-back")]
    pub struct StepCursorBack {}

    pub fn handle(_payload: &StepCursorBack, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let len = fixture.steps.len();
        let current = fixture.resolved_up_to.unwrap_or(len) as i64;
        Ok(Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: Some((current - 1).clamp(0, len as i64) as usize) }]))
    }
}
//#endregion 🔖️StepCursorBack

//#region 🔖️StepCursorForward
pub mod step_cursor_forward {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor-forward")]
    pub struct StepCursorForward {}

    pub fn handle(_payload: &StepCursorForward, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let len = fixture.steps.len();
        let current = fixture.resolved_up_to.unwrap_or(len) as i64;
        Ok(Emit::operations(vec![Process3dOperation::SetCursor { resolved_up_to: Some((current + 1).clamp(0, len as i64) as usize) }]))
    }
}
//#endregion 🔖️StepCursorForward
