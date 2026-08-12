//! 🔺️ `rename-step` / `change-step-description` — sparse diff construction.

use super::mutation::{ChangeStepDescription, RenameStep};
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_rename_step(payload: &RenameStep, base: &FormsSnapshot) -> FormsDiff {
    if !base.steps.iter().any(|step| step.id == payload.id) {
        return FormsDiff::default();
    }
    let patch = FormsStepPatch { title: Some(payload.new_title.clone()), ..Default::default() };
    FormsDiff {
        steps: Some(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}

pub fn diff_change_step_description(payload: &ChangeStepDescription, base: &FormsSnapshot) -> FormsDiff {
    if !base.steps.iter().any(|step| step.id == payload.id) {
        return FormsDiff::default();
    }
    let patch = FormsStepPatch { description: Some(payload.new_description.clone()), ..Default::default() };
    FormsDiff {
        steps: Some(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
