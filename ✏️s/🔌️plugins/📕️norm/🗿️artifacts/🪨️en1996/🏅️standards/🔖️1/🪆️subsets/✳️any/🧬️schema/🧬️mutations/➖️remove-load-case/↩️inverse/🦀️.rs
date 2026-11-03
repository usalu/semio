//! ↩️ `remove-load-case` inverse: inserts the removed row back at its position.

use crate::remove_load_case::RemoveLoadCase;
use crate::insert_load_case::InsertLoadCase;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveLoadCase, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_load_case::InsertLoadCase> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveLoadCase, base: &En1996Snapshot) -> Option<InsertLoadCase> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.load_cases.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertLoadCase { wall_id: payload.wall_id.clone(), index, load_case: row.clone() })
}
