//! 🔺️ `delete-step` — sparse diff construction.

use super::mutation::DeleteStep;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::schema::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_step(payload: &DeleteStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    if !forms_steps(base).iter().any(|step| step.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(forms_diff_from_delta(FormsStepsDelta { removed: vec![payload.id.clone()], ..Default::default() }, base))
}
//#endregion 🔖️Diff
