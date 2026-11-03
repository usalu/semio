use crate::insert_placement::InsertPlacement;
use crate::remove_placement;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertPlacement, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_placement::RemovePlacement> {
    let at = payload.index.min(base.placements.len());
    Ok(vec![Op::from(remove_placement::RemovePlacement { index: at })])
}
