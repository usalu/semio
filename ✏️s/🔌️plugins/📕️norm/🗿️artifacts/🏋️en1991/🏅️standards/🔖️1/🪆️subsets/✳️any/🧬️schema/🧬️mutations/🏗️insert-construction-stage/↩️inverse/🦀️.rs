//! ↩️ `insert-construction-stage` inverse: removes the inserted row again.

use crate::insert_construction_stage::InsertConstructionStage;
use crate::remove_construction_stage::RemoveConstructionStage;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertConstructionStage, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_construction_stage::RemoveConstructionStage> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertConstructionStage, base: &En1991Snapshot) -> Option<RemoveConstructionStage> {
    Some(RemoveConstructionStage { id: payload.item.id.clone() })
}
