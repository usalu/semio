//! 🔺️ Diff constructor for `SetOpening`: a sparse opening patch naming only the provided fields that differ from the base. A new kind
//! must name existing types, a sill override must not be negative, size overrides must be positive, and a change of kind or width must
//! still fit the host axis and clear the neighbours. Clearing an override (`width: {"value": null}`) falls back to the kind's size, clearing the sill override to the sill of the type.

use super::super::placement::{host_length, kind_issue, placement_issue, positive, refuse_kind, width_of};
use super::SetOpening;
use crate::{Entry, ModelDiff, ModelSnapshot, OpeningPatch, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetOpening, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(opening) = base.openings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Opening \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(issue) = payload.kind.as_ref().and_then(|kind| kind_issue(base, kind)) {
        return refuse_kind(issue, ["kind"]);
    }
    if payload.sill_override.as_ref().and_then(|assigned| assigned.value).is_some_and(|sill| !(sill.is_finite() && sill >= 0.0)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "An opening sill must not be negative.", ["sill_override"]);
    }
    for (field, size) in [("width", &payload.width), ("height", &payload.height)] {
        if size.as_ref().and_then(|assigned| assigned.value).is_some_and(|value| !positive(value)) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, format!("An opening {field} must be a positive length."), [field]);
        }
    }
    let patch = OpeningPatch {
        kind: payload.kind.clone().filter(|kind| *kind != opening.kind),
        sill_override: payload.sill_override.clone().filter(|sill| sill.value != opening.sill_override),
        width: payload.width.clone().filter(|width| width.value != opening.width),
        height: payload.height.clone().filter(|height| height.value != opening.height),
        flip_hand: payload.flip_hand.filter(|flip| *flip != opening.flip_hand),
        flip_facing: payload.flip_facing.filter(|flip| *flip != opening.flip_facing),
        name: payload.name.clone().filter(|name| *name != opening.name),
        ..Default::default()
    };
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Opening \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if patch.kind.is_some() || patch.width.is_some() {
        let next = patch.write(opening);
        let field = if patch.width.is_some() { "width" } else { "kind" };
        let Some(length) = host_length(base, &next.host) else {
            return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Host \"{}\" is neither a wall nor a curtain wall.", next.host), ["host"]);
        };
        let Some(width) = width_of(base, &next) else {
            return MutationOutcome::refuse(OutcomeCode::Invariant, "An opening needs a resolvable width.", [field]);
        };
        if let Some(message) = placement_issue(base, Some(&payload.id), &next.host, length, next.offset, width) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
        }
    }
    MutationOutcome::new(ModelDiff::openings(payload.id.clone(), Entry::Patched(patch)))
}
