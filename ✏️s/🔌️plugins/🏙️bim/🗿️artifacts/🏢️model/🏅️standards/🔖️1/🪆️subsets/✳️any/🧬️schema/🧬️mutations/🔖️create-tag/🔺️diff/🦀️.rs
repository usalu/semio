//! 🔺️ Diff constructor for `CreateTag`: one created tag entry. The id is free in every collection. The storey and the style must exist, the element is one that can be tagged and the offset is finite.
//! Everything the tag shows is inferred from the current geometry of what it names, never stored.

use super::super::annotating;
use super::super::elements;
use super::CreateTag;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateTag, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = annotating::tag_fault(base, &payload.tag) {
        let path = fault.path(Some("tag"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::tags(payload.id.clone(), Entry::Created(payload.tag.clone())))
}
