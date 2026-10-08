use super::InsertCraneRunway;
use crate::mutations::{remove_crane_runway, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertCraneRunway, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.crane_runways.len());
    vec![En1993Mutation::RemoveCraneRunway(remove_crane_runway::RemoveCraneRunway { index: at })]

    })())
}
