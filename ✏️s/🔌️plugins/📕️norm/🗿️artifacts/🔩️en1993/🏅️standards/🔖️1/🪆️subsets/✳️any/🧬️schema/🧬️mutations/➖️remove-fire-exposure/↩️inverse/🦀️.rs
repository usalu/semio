use super::RemoveFireExposure;
use crate::mutations::{insert_fire_exposure, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveFireExposure, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.fire_exposures.len() { return Vec::new(); }
    vec![En1993Mutation::InsertFireExposure(insert_fire_exposure::InsertFireExposure { index: Some(payload.index), fire_exposure: base.fire_exposures[payload.index].clone() })]

    })())
}
