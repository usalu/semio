//! 🔺️ Diff constructor for `CreateBuilding`: one created building entry; the site must exist, the id must be free.

use super::super::elements;
use super::CreateBuilding;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateBuilding, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.sites.contains_key(&payload.building.site) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Site \"{}\" does not exist.", payload.building.site), ["building", "site"]);
    }
    MutationOutcome::new(ModelDiff::buildings(payload.id.clone(), Entry::Created(payload.building.clone())))
}
