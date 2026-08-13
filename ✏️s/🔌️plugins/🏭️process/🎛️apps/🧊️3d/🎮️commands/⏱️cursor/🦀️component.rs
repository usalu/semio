//! ⏱️ Process 3d play app commands — the process timeline cursor (`resolved_up_to`), NOT framework
//! History — these move the replay cursor.
//!
//! 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `steps` is a composed
//! `s.stdio.semio.flow` CHILD HANDLE now (no `.len()` — see `ProcessWorkingScene`'s doc comment in
//! the artifact root file), so the cursor can no longer be clamped against the real step count from
//! a bare snapshot; each handler below clamps only against `0` (never negative), documenting the
//! dropped upper bound honestly rather than guessing at an unknown length.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::mutations::change_cursor::mutation::ChangeCursor;
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
        let _ = doc;
        let resolved = payload.value.map(|n| n as usize);
        Ok(Emit::mutations(vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: resolved })]))
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
        let current = fixture.resolved_up_to.unwrap_or(0) as i64;
        Ok(Emit::mutations(vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some((current + payload.delta).max(0) as usize) })]))
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
        let current = fixture.resolved_up_to.unwrap_or(0) as i64;
        Ok(Emit::mutations(vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some((current - 1).max(0) as usize) })]))
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
        let current = fixture.resolved_up_to.unwrap_or(0) as i64;
        Ok(Emit::mutations(vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some((current + 1).max(0) as usize) })]))
    }
}
//#endregion 🔖️StepCursorForward
