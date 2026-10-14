//! ↩️ `change-night-setback` inverse.
use crate::change_night_setback::ChangeNightSetback;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeNightSetback, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_night_setback::ChangeNightSetback> {
    Ok((|| {
    vec![Op::from(ChangeNightSetback { new_night_setback_k: base.night_setback_k })]

    })())
}
