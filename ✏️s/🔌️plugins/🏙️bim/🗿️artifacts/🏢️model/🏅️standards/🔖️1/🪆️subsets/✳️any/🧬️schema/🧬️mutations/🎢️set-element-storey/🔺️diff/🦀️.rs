//! 🔺️ Diff constructor for `SetElementStorey`: a one-field patch of the storey slot in whichever collection holds the element. The target storey must exist and belong to the building the element stands in. Nothing derived is written:
//! hosted openings follow their host by reference, the components mounted on a wall move to its new storey with it (a mounted component itself cannot change its storey), and the heights, layouts, solids and quantities of the element follow by inference. The move is refused when the resolved top of the element would no longer lie above its base, or when
//! an opening that fits its host today would rise above the host at its new height.

use super::super::{component_rules, elements, placement};
use super::SetElementStorey;
use crate::{Component, ComponentPatch, Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

fn resolved(base: &ModelSnapshot, storey: &str, id: &str) -> Option<f64> {
    elements::vertical_of(base, id).and_then(|(base_offset, top)| placement::rise(base, storey, base_offset, top))
}

pub fn diff(payload: &SetElementStorey, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let id = &payload.id;
    let Some(current) = elements::storey_of(base, id) else {
        return if base.openings.contains_key(id) {
            MutationOutcome::refuse(OutcomeCode::Invariant, format!("Opening \"{id}\" stands on the storey of its host; move the host."), [id.clone()])
        } else if elements::exists(base, id) {
            MutationOutcome::refuse(OutcomeCode::Invariant, format!("Element \"{id}\" stands on no storey."), [id.clone()])
        } else {
            MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{id}\" does not exist."), [id.clone()])
        };
    };
    if let Some(wall) = base.components.get(id).and_then(|row| row.host.as_ref()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Component \"{id}\" is mounted on wall \"{wall}\" and stands on its storey; move the wall."), [id.clone()]);
    }
    let Some(target) = base.storeys.get(&payload.storey) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.storey), ["storey"]);
    };
    if current == payload.storey {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Element \"{id}\" already stands on storey \"{current}\"."), [id.clone()]);
    }
    if base.storeys.get(&current).map(|row| &row.building) != Some(&target.building) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Storey \"{}\" belongs to another building.", payload.storey), ["storey"]);
    }
    if elements::vertical_of(base, id).is_some() && !resolved(base, &payload.storey, id).is_some_and(|rise| rise.is_finite() && rise > 0.0) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("The top of element \"{id}\" would no longer lie above its base on storey \"{}\".", payload.storey), ["storey"]);
    }
    let hosted = base.openings.values().filter(|opening| &opening.host == id).count();
    if let (Some(height), Some(was)) = (resolved(base, &payload.storey, id), resolved(base, &current, id)) {
        if placement::overflowing_opening(base, id, was).is_none() {
            if let Some(opening) = placement::overflowing_opening(base, id, height) {
                return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Opening \"{opening}\" would rise above the host \"{id}\" on storey \"{}\".", payload.storey), ["storey"]);
            }
        }
    }
    let Some(mut diff) = elements::restorey(base, id, &payload.storey) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{id}\" does not exist."), [id.clone()]);
    };
    let along: BTreeMap<String, Entry<Component, ComponentPatch>> = component_rules::mounted_on(base, id).map(|component| (component.clone(), Entry::Patched(ComponentPatch { storey: Some(payload.storey.clone()), ..Default::default() }))).collect();
    let carried = hosted + along.len();
    if !along.is_empty() {
        diff.components = Some(KeyedDelta(along));
    }
    let outcome = MutationOutcome::new(diff);
    if carried == 0 {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("Element \"{id}\" took {carried} hosted opening(s) and component(s) along."))
    }
}
