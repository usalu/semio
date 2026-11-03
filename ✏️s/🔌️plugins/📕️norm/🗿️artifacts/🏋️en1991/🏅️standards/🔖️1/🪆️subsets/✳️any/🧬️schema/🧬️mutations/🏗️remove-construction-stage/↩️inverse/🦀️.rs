//! ↩️ `remove-construction-stage` inverse: inserts the removed row back at its position.

use crate::remove_construction_stage::RemoveConstructionStage;
use crate::insert_construction_stage::InsertConstructionStage;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveConstructionStage, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_construction_stage::InsertConstructionStage> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveConstructionStage, base: &En1991Snapshot) -> Option<InsertConstructionStage> {
    let (index, row) = base.construction_stages.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertConstructionStage { index, item: row.clone() })
}
