use crate::insert_fire_exposure::InsertFireExposure;
use crate::remove_fire_exposure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFireExposure, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fire_exposure::RemoveFireExposure> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.fire_exposures.len());
    vec![Op::from(remove_fire_exposure::RemoveFireExposure { index: at })]

    })())
}
