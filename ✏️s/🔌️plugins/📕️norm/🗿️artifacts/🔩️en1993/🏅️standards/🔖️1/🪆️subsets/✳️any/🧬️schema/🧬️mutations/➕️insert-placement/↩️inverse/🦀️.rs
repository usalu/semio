use crate::insert_placement::InsertPlacement;
use crate::remove_placement;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertPlacement, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_placement::RemovePlacement> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.placements.len());
    vec![Op::from(remove_placement::RemovePlacement { index: at })]

    })())
}
