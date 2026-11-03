use crate::remove_bracing::RemoveBracing;
use crate::insert_bracing;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveBracing, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bracing::InsertBracing> {
    Ok(match base.bracings.get(payload.index) {
        Some(item) => vec![Op::from(insert_bracing::InsertBracing { index: payload.index, bracing: item.clone() })],
        None => Vec::new(),
    })
}
