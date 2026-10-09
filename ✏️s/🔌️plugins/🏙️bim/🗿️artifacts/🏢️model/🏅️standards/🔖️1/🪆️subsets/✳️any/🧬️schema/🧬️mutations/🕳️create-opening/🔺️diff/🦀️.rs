//! 🔺️ Diff constructor for `CreateOpening`: one created opening entry. The host (a wall or curtain wall), the window or door type it
//! names and a non-negative sill override (when authored) must hold, the centre offset must keep the opening inside the host axis and clear of its
//! neighbours. An authored reveal (depth not negative, material existing) is checked as well. Only the offset, the optional sill override (which replaces the sill of the window type) and the optional reveal are stored; the frame follows the host parametrically by inference.

use super::super::elements;
use super::super::wall_depth::reveal_flaw;
use super::super::placement::{host_length, kind_issue, placement_issue, positive, refuse_kind, width_of};
use super::CreateOpening;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateOpening, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let opening = &payload.opening;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let Some(length) = host_length(base, &opening.host) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Host \"{}\" is neither a wall nor a curtain wall.", opening.host), ["opening", "host"]);
    };
    if let Some(issue) = kind_issue(base, &opening.kind) {
        return refuse_kind(issue, ["opening", "kind"]);
    }
    if opening.sill_override.is_some_and(|sill| !(sill.is_finite() && sill >= 0.0)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "An opening sill must not be negative.", ["opening", "sill_override"]);
    }
    if let Some(flaw) = reveal_flaw(base, opening.reveal_depth, opening.reveal_material.as_deref()) {
        return flaw.under(&["opening"]).refuse();
    }
    for (field, size) in [("width", opening.width), ("height", opening.height)] {
        if size.is_some_and(|value| !positive(value)) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, format!("An opening {field} must be a positive length."), ["opening", field]);
        }
    }
    let Some(width) = width_of(base, opening) else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "An opening needs a resolvable width.", ["opening", "width"]);
    };
    if let Some(message) = placement_issue(base, None, &opening.host, length, opening.offset, width) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["opening", "offset"]);
    }
    MutationOutcome::new(ModelDiff::openings(payload.id.clone(), Entry::Created(opening.clone())))
}
