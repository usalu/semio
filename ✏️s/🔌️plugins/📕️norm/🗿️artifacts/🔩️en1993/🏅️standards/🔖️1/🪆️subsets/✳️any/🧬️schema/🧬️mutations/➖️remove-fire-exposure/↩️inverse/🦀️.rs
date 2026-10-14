use crate::remove_fire_exposure::RemoveFireExposure;
use crate::insert_fire_exposure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFireExposure, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fire_exposure::InsertFireExposure> {
    Ok((|| {
    if payload.index >= base.fire_exposures.len() { return Vec::new(); }
    vec![Op::from(insert_fire_exposure::InsertFireExposure { index: Some(payload.index), fire_exposure: base.fire_exposures[payload.index].clone() })]

    })())
}
