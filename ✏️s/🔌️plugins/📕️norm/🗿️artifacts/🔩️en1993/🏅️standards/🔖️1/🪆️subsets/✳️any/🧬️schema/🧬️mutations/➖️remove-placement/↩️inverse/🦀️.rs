use crate::remove_placement::RemovePlacement;
use crate::insert_placement;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemovePlacement, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_placement::InsertPlacement> {
    Ok((|| {
    if payload.index >= base.placements.len() { return Vec::new(); }
    vec![Op::from(insert_placement::InsertPlacement { index: Some(payload.index), placement: base.placements[payload.index].clone() })]

    })())
}
