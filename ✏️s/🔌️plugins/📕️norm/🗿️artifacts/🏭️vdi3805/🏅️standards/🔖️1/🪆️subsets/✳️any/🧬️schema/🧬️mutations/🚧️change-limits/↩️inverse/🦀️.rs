//! ↩️ `change-limits` — undo restores BASE's whole limits facet.

use crate::change_limits::ChangeLimits;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeLimits, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_limits::ChangeLimits> {
    Ok((|| {
    vec![Op::from(ChangeLimits { new_limits: base.limits })]

    })())
}
//#endregion 🔖️Inverse
