//! 🔺️ Diff constructor for `CreateAreaScheme`: one created area scheme entry. The id must be free in every collection, every counted zone must
//! exist and a counted usage must not be blank (a blank usage could never match a space). An empty list counts everything, so it is valid.

use super::super::elements;
use super::CreateAreaScheme;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateAreaScheme, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let scheme = &payload.area_scheme;
    if let Some((code, message, field)) = elements::scheme_rule_issue(base, &scheme.usages, &scheme.zones) {
        return MutationOutcome::refuse(code, message, ["area_scheme", field]);
    }
    MutationOutcome::new(ModelDiff::area_schemes(payload.id.clone(), Entry::Created(scheme.clone())))
}
