use crate::insert_crane_runway::InsertCraneRunway;
use crate::remove_crane_runway;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertCraneRunway, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_crane_runway::RemoveCraneRunway> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.crane_runways.len());
    vec![Op::from(remove_crane_runway::RemoveCraneRunway { index: at })]

    })())
}
