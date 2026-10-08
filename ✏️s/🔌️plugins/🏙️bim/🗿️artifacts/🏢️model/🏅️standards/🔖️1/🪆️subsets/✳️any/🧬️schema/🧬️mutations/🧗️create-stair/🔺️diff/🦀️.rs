//! 🔺️ Diff constructor for `CreateStair`: one created stair entry. The storey must exist, width, maximum riser and minimum tread must
//! be positive, the flight must be sensible and a storey top constraint must name a storey of the same building. Riser count and
//! tread depth are never stored: they are inferred from the storey levels.

use super::super::elements;
use super::super::placement::{refuse_stair, stair_issue};
use super::CreateStair;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateStair, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let stair = &payload.stair;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&stair.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", stair.storey), ["stair", "storey"]);
    }
    if let Some(issue) = stair_issue(base, stair) {
        return refuse_stair(issue, &["stair"]);
    }
    MutationOutcome::new(ModelDiff::stairs(payload.id.clone(), Entry::Created(stair.clone())))
}
