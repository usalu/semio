//! 🔺️ `rename-step` / `change-step-description` — sparse diff construction.

use super::mutation::ChangeStepDescription;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeStepDescription, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(existing) = steps.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.description == payload.new_description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" description is already unchanged.", payload.id));
    }
    let patch = FormsStepPatch { description: Some(payload.new_description.clone()), ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, base))
}
