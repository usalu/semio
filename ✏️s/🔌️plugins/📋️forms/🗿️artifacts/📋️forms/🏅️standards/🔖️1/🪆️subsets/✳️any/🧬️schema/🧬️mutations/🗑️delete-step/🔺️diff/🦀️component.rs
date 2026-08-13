//! 🔺️ `delete-step` — sparse diff construction.

use super::mutation::DeleteStep;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::schema::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_step(payload: &DeleteStep, base: &FormsSnapshot) -> FormsDiff {
    if !forms_steps(base).iter().any(|step| step.id == payload.id) {
        return FormsDiff::default();
    }
    forms_diff_from_delta(FormsStepsDelta { removed: vec![payload.id.clone()], ..Default::default() }, base)
}
//#endregion 🔖️Diff
