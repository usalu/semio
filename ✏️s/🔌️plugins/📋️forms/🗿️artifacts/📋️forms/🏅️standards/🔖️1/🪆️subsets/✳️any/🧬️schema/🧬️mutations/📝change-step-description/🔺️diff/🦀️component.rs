//! 🔺️ `rename-step` / `change-step-description` — sparse diff construction.

use super::mutation::ChangeStepDescription;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::schema::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeStepDescription, base: &FormsSnapshot) -> FormsDiff {
    if !forms_steps(base).iter().any(|step| step.id == payload.id) {
        return FormsDiff::default();
    }
    let patch = FormsStepPatch { description: Some(payload.new_description.clone()), ..Default::default() };
    forms_diff_from_delta(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, base)
}
