//! ↩️ `insert-fatigue-model` inverse: removes the inserted row again.

use crate::insert_fatigue_model::InsertFatigueModel;
use crate::remove_fatigue_model::RemoveFatigueModel;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertFatigueModel, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fatigue_model::RemoveFatigueModel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertFatigueModel, base: &En1991Snapshot) -> Option<RemoveFatigueModel> {
    Some(RemoveFatigueModel { id: payload.item.id.clone() })
}
