//! 🔺️ Diff constructor for `SetElementPhase`: a one-field patch of the phase slot in whichever collection holds the element. An opening carries no phase of its own, it takes the phase of its host, so the host is the one to set.
//! Nothing derived is written: which elements a view phase shows and the quantities per phase follow by inference.

use super::super::elements;
use super::SetElementPhase;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetElementPhase, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let id = &payload.id;
    let Some(current) = elements::phase_of(base, id) else {
        return if base.openings.contains_key(id) {
            MutationOutcome::refuse(OutcomeCode::Invariant, format!("Opening \"{id}\" takes the phase of its host; set the host."), [id.clone()])
        } else if elements::exists(base, id) {
            MutationOutcome::refuse(OutcomeCode::Invariant, format!("Element \"{id}\" carries no phase."), [id.clone()])
        } else {
            MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{id}\" does not exist."), [id.clone()])
        };
    };
    if current == payload.phase {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Element \"{id}\" already is in phase {:?}.", current), [id.clone()]);
    }
    let Some(diff) = elements::rephase(base, id, payload.phase) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{id}\" does not exist."), [id.clone()]);
    };
    let outcome = MutationOutcome::new(diff);
    let hosted = base.openings.values().filter(|opening| &opening.host == id).count();
    if hosted == 0 {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("Element \"{id}\" took {hosted} hosted opening(s) into the phase."))
    }
}
