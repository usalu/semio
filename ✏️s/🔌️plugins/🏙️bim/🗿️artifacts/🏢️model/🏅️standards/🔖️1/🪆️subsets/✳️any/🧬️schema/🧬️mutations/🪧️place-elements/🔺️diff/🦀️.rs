//! 🔺️ Diff constructor for `PlaceElements`: every listed element receives its absolute placement, one sparse patch per element holding
//! exactly its placement fields. Placements equal to the current one are left out. Refused: an empty map, an id that is no element or
//! has no placement, a placement of another element kind, a non-finite number; nothing to change is `mutation.no-op`.

use super::super::elements::{self, Change, Refusal};
use super::PlaceElements;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

fn changes(payload: &PlaceElements, base: &ModelSnapshot) -> Result<Change, Refusal> {
    if payload.placements.is_empty() {
        return Err(Refusal::new(OutcomeCode::Invariant, "No placement is given.", ["placements"]));
    }
    let mut change = Change { before: Default::default(), after: Default::default() };
    for (id, target) in &payload.placements {
        let Some(current) = elements::state(base, id) else {
            let code = if elements::exists(base, id) { OutcomeCode::Invariant } else { OutcomeCode::TargetMissing };
            return Err(Refusal::new(code, format!("Element \"{id}\" has no placement to set."), [id.clone()]));
        };
        if !current.same_kind(target) {
            return Err(Refusal::new(OutcomeCode::Invariant, format!("The placement does not fit the kind of element \"{id}\"."), [id.clone()]));
        }
        if !target.numbers().iter().all(|number| number.is_finite()) {
            return Err(Refusal::new(OutcomeCode::Invariant, format!("The placement of \"{id}\" must be finite."), ["placements".to_string(), id.clone()]));
        }
        if *target != current {
            change.before.insert(id.clone(), current);
            change.after.insert(id.clone(), target.clone());
        }
    }
    Ok(change)
}

pub fn diff(payload: &PlaceElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    elements::placed(changes(payload, base), "placements")
}
