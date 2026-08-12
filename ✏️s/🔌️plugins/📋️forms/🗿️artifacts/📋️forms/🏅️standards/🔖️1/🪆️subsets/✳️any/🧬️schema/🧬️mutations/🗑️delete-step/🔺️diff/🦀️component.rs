//! 🔺️ `delete-step` — sparse diff construction.

use super::mutation::DeleteStep;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_step(payload: &DeleteStep, base: &FormsSnapshot) -> FormsDiff {
    if !base.steps.iter().any(|step| step.id == payload.id) {
        return FormsDiff::default();
    }
    FormsDiff { steps: Some(FormsStepsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
