//! ↩️ `change-script-limits` — undo restores BASE's whole limits facet.

use crate::change_script_limits::mutation::ChangeScriptLimits;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeScriptLimits, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_script_limits::mutation::ChangeScriptLimits> {
    Ok((|| {
    vec![Op::from(ChangeScriptLimits { new_max_steps: base.script_limits.max_steps, new_max_recursion: base.script_limits.max_recursion, new_timeout_ms: base.script_limits.timeout_ms })]

    })())
}
//#endregion 🔖️Inverse
