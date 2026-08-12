//! 🔺️ `update-script-limits` — sparse diff construction.

use super::mutation::UpdateScriptLimits;
use crate::artifacts::iso16757::{part_5::ScriptLimits, Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateScriptLimits, _base: &Iso16757Snapshot) -> Iso16757Diff {
    Iso16757Diff {
        script_limits: Some(ScriptLimits { max_steps: payload.new_max_steps, max_recursion: payload.new_max_recursion, timeout_ms: payload.new_timeout_ms }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
