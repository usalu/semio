//! 🔺️ Diff constructor for `CreateZone`: one created zone entry. The id must be free in every collection and the occupancy density a finite,
//! non-negative number of persons per square metre. The spaces of the zone are not listed here: each space names its zone.

use super::super::elements;
use super::CreateZone;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateZone, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !(payload.zone.occupancy_density.is_finite() && payload.zone.occupancy_density >= 0.0) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "An occupancy density is a finite number of persons per square metre, zero or more.", ["zone", "occupancy_density"]);
    }
    MutationOutcome::new(ModelDiff::zones(payload.id.clone(), Entry::Created(payload.zone.clone())))
}
