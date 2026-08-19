//! ↩️ `update-script-limits` — undo restores BASE's whole limits facet.

use super::mutation::UpdateScriptLimits;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &UpdateScriptLimits, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::UpdateScriptLimits(UpdateScriptLimits { new_max_steps: base.script_limits.max_steps, new_max_recursion: base.script_limits.max_recursion, new_timeout_ms: base.script_limits.timeout_ms })]
}
//#endregion 🔖️Inverse
