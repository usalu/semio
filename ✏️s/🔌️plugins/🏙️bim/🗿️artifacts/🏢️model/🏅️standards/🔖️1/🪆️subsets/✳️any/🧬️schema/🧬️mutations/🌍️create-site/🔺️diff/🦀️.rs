//! 🔺️ Diff constructor for `CreateSite`: one created site entry; a taken id is a fatal `mutation.duplicate-id`.

use super::super::elements;
use super::CreateSite;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSite, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::sites(payload.id.clone(), Entry::Created(payload.site.clone())))
}
