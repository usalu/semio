use super::InsertFireExposure;
use crate::mutations::{remove_fire_exposure, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertFireExposure, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.fire_exposures.len());
    vec![En1993Mutation::RemoveFireExposure(remove_fire_exposure::RemoveFireExposure { index: at })]

    })())
}
