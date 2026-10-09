//! 🔺️ Diff constructor for `CreateLeader`: one created leader entry. The id is free in every collection. The storey and the style must exist, the anchor names an existing element or a finite point, the offset is finite and the text is not blank.
//! Everything the leader shows is inferred from the current geometry of what it names, never stored.

use super::super::annotating;
use super::super::elements;
use super::CreateLeader;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateLeader, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = annotating::leader_fault(base, &payload.leader) {
        let path = fault.path(Some("leader"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::leaders(payload.id.clone(), Entry::Created(payload.leader.clone())))
}
