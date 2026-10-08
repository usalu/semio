//! 🔺️ Diff constructor for `SetProjectInfo`: a sparse project patch of exactly the provided fields. An empty patch, or one that
//! restates the current values, is a `mutation.no-op`; phase names must be non-blank and distinct.
//! Whole-list ruling: the phase names are ONE ordered vocabulary of the project (exported as one IFC phase string, referenced by
//! no element: a wall's phase is the fixed `Phase` enum), so the list replaces as one field.

use super::SetProjectInfo;
use crate::{ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

fn sound(names: &[String]) -> bool {
    names.iter().enumerate().all(|(index, name)| !name.trim().is_empty() && !names[..index].contains(name))
}

pub fn diff(payload: &SetProjectInfo, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let project = &base.project;
    if payload.phase_names.as_deref().is_some_and(|names| !sound(names)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "Phase names must be non-blank and distinct.", ["phase_names"]);
    }
    let patch = payload.patch().minimal(project);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, "The project info already holds these values.", ["project"]);
    }
    MutationOutcome::new(ModelDiff { project: Some(patch), ..ModelDiff::default() })
}
