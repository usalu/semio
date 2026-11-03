//! ↩️ `change-tie-beam-foundation-a-id` inverse: restores the previous `foundation_a_id`.

use crate::change_tie_beam_foundation_a_id::ChangeTieBeamFoundationAId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamFoundationAId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_foundation_a_id::ChangeTieBeamFoundationAId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamFoundationAId, base: &En1998Snapshot) -> Option<ChangeTieBeamFoundationAId> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamFoundationAId { id: payload.id.clone(), new_foundation_a_id: r0.foundation_a_id.clone() })
}
