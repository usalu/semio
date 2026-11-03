//! 🪚️ `change-chase-kind` diff: patches `kind` of the addressed chase row.

use crate::change_chase_kind::ChangeChaseKind;
use semio_s_artifact_norm_en1996_diff::{En1996ChaseDelta, En1996ChasePatch, En1996Diff};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeChaseKind, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(r0) = base.chases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Chase {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The chase already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { chases: En1996ChaseDelta::modification(&payload.id, En1996ChasePatch { kind: Some(payload.new_kind), ..Default::default() }), ..Default::default() })
}
