//! ⏱️ Process 3d play app commands — the replay cursor (`Process3dConfig.resolved_up_to`). The cursor is VIEW
//! state: stepping through the process timeline moves the viewer's config cursor, never the document and never
//! history, so a time-travel session can step through a process while the document is frozen. Consecutive cursor
//! moves fold into one config edit under [`PROCESS3D_CURSOR_COALESCE_KEY`]. Every move clamps to the timeline:
//! `0` shows the bare stock, the step count (or `None`) every step.

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::{op::Process3dMutation, Process3dSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔑️ The one coalesce key every cursor move carries, so stepping through the timeline is one config edit.
pub const PROCESS3D_CURSOR_COALESCE_KEY: &str = "process3d-cursor";

/// 📍️ The resolved step count the viewer shows on `snapshot`: its cursor clamped to the timeline, every step when unset.
pub fn process3d_cursor(snapshot: &Process3dSnapshot, config: &Process3dConfig) -> usize {
    config.resolved_up_to.unwrap_or(snapshot.step_payloads.len()).min(snapshot.step_payloads.len())
}

/// 🎚️ The config writes that move the viewer's cursor to `next` (clamped to the timeline) — none when it already
/// sits there.
pub fn process3d_cursor_moves(snapshot: &Process3dSnapshot, config: &Process3dConfig, next: Option<usize>) -> Vec<Process3dConfigMutation> {
    let next = next.map(|value| value.min(snapshot.step_payloads.len()));
    if next == config.resolved_up_to {
        return Vec::new();
    }
    vec![Process3dConfigMutation::SetCursor { value: next }]
}

/// ⏩️ The view emit of one cursor move: a coalesced config edit, nothing on the document.
fn cursor_emit(doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, next: Option<usize>) -> Emit<Process3dMutation, Process3dConfigMutation> {
    Emit::amend_config(process3d_cursor_moves(doc.snapshot, cfg.snapshot, next), PROCESS3D_CURSOR_COALESCE_KEY)
}

//#region 🔖️SetCursor
pub mod set_cursor {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "cursor")]
    pub struct SetCursor {
        pub value: Option<u64>,
    }

    pub fn handle(payload: &SetCursor, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(cursor_emit(doc, cfg, payload.value.map(|value| usize::try_from(value).unwrap_or(usize::MAX))))
    }
}
//#endregion 🔖️SetCursor

//#region 🔖️StepCursor
pub mod step_cursor {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor")]
    pub struct StepCursor {
        pub delta: i64,
    }

    pub fn handle(payload: &StepCursor, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let current = process3d_cursor(doc.snapshot, cfg.snapshot) as i64;
        Ok(cursor_emit(doc, cfg, Some(current.saturating_add(payload.delta).max(0) as usize)))
    }
}
//#endregion 🔖️StepCursor

//#region 🔖️StepCursorBack
pub mod step_cursor_back {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor-back")]
    pub struct StepCursorBack {}

    pub fn handle(
        _payload: &StepCursorBack,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(cursor_emit(doc, cfg, Some(process3d_cursor(doc.snapshot, cfg.snapshot).saturating_sub(1))))
    }
}
//#endregion 🔖️StepCursorBack

//#region 🔖️StepCursorForward
pub mod step_cursor_forward {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "step-cursor-forward")]
    pub struct StepCursorForward {}

    pub fn handle(
        _payload: &StepCursorForward,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(cursor_emit(doc, cfg, Some(process3d_cursor(doc.snapshot, cfg.snapshot) + 1)))
    }
}
//#endregion 🔖️StepCursorForward

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
