//! 🔺️ `update-script-limits` — sparse diff construction.

use super::mutation::UpdateScriptLimits;
use crate::artifacts::iso16757::{part_5::ScriptLimits, Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateScriptLimits, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let new_limits = ScriptLimits { max_steps: payload.new_max_steps, max_recursion: payload.new_max_recursion, timeout_ms: payload.new_timeout_ms };
    if base.script_limits == new_limits {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Script limits already have these values.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { script_limits: Some(new_limits), ..Default::default() })
}
//#endregion 🔖️Diff
