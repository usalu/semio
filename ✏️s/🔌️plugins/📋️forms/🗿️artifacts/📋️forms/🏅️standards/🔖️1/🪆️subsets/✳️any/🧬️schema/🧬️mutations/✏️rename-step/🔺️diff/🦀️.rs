//! 🔺️ `rename-step` / `change-step-description` — sparse diff construction.

use super::mutation::RenameStep;
use crate::schema::diff::forms_diff_from_delta;
use crate::schema::diff::{FormsStepPatch, FormsStepsDelta};
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(existing) = steps.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.title == payload.new_title {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Step \"{}\" is already titled \"{}\".", payload.id, payload.new_title));
    }
    let patch = FormsStepPatch { id: payload.id.clone(), title: Some(payload.new_title.clone()), ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(&FormsStepsDelta { patched: vec![patch], ..Default::default() }, base))
}
