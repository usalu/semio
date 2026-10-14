use crate::remove_crane_runway::RemoveCraneRunway;
use crate::insert_crane_runway;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveCraneRunway, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_crane_runway::InsertCraneRunway> {
    Ok((|| {
    if payload.index >= base.crane_runways.len() { return Vec::new(); }
    vec![Op::from(insert_crane_runway::InsertCraneRunway { index: Some(payload.index), crane_runway: base.crane_runways[payload.index].clone() })]

    })())
}
