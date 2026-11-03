use crate::remove_placement::RemovePlacement;
use crate::insert_placement;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemovePlacement, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_placement::InsertPlacement> {
    Ok(match base.placements.get(payload.index) {
        Some(item) => vec![Op::from(insert_placement::InsertPlacement { index: payload.index, placement: item.clone() })],
        None => Vec::new(),
    })
}
