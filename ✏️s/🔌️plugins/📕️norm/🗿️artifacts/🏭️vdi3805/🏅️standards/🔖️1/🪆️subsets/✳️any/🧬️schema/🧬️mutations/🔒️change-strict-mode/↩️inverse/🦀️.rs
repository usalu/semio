//! ↩️ `change-strict-mode` — undo restores BASE's flag value.

use crate::change_strict_mode::ChangeStrictMode;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeStrictMode, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_strict_mode::ChangeStrictMode> {
    Ok((|| {
    vec![Op::from(ChangeStrictMode { new_strict_mode: base.strict_mode })]

    })())
}
//#endregion 🔖️Inverse
