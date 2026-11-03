//! ↩️ `insert-load-case` inverse: removes the inserted row again.

use crate::insert_load_case::InsertLoadCase;
use crate::remove_load_case::RemoveLoadCase;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertLoadCase, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_load_case::RemoveLoadCase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertLoadCase, base: &En1996Snapshot) -> Option<RemoveLoadCase> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemoveLoadCase { wall_id: payload.wall_id.clone(), id: payload.load_case.id.clone() })
}
