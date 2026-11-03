//! ↩️ `change-tie-beam-foundation-b-id` inverse: restores the previous `foundation_b_id`.

use crate::change_tie_beam_foundation_b_id::ChangeTieBeamFoundationBId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTieBeamFoundationBId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_beam_foundation_b_id::ChangeTieBeamFoundationBId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieBeamFoundationBId, base: &En1998Snapshot) -> Option<ChangeTieBeamFoundationBId> {
    let r0 = base.tie_beams.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieBeamFoundationBId { id: payload.id.clone(), new_foundation_b_id: r0.foundation_b_id.clone() })
}
