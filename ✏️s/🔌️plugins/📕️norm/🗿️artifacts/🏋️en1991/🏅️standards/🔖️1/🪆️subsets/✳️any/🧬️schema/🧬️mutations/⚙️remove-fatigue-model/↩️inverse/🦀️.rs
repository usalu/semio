//! ↩️ `remove-fatigue-model` inverse: inserts the removed row back at its position.

use crate::remove_fatigue_model::RemoveFatigueModel;
use crate::insert_fatigue_model::InsertFatigueModel;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveFatigueModel, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fatigue_model::InsertFatigueModel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFatigueModel, base: &En1991Snapshot) -> Option<InsertFatigueModel> {
    let (index, row) = base.fatigue_models.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertFatigueModel { index, item: row.clone() })
}
