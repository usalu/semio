//! 🪚️ `change-chase-width` diff: patches `width_m` of the addressed chase row.

use crate::change_chase_width::ChangeChaseWidth;
use semio_s_artifact_norm_en1996_diff::{En1996ChaseDelta, En1996ChasePatch, En1996Diff};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeChaseWidth, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_width_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.chases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Chase {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.width_m == payload.new_width_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The chase already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { chases: En1996ChaseDelta::modification(&payload.id, En1996ChasePatch { width_m: Some(payload.new_width_m), ..Default::default() }), ..Default::default() })
}
