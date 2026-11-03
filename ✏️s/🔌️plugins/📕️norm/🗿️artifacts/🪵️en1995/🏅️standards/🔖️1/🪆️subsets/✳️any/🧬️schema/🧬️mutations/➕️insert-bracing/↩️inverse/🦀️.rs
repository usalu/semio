use crate::insert_bracing::InsertBracing;
use crate::remove_bracing;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertBracing, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bracing::RemoveBracing> {
    let at = payload.index.min(base.bracings.len());
    Ok(vec![Op::from(remove_bracing::RemoveBracing { index: at })])
}
