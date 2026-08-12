//! 🔺️ `create-step` — sparse diff construction.

use super::mutation::CreateStep;
use crate::artifacts::forms::schema::diff::FormsStepsDelta;
use crate::artifacts::forms::{FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_create_step(payload: &CreateStep, base: &FormsSnapshot) -> FormsDiff {
    if base.steps.iter().any(|step| step.id == payload.step.id) {
        return FormsDiff::default();
    }
    let mut delta = FormsStepsDelta { added: vec![payload.step.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = base.steps.iter().map(|step| step.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.step.id.clone());
        delta.reordered = Some(order);
    }
    FormsDiff { steps: Some(delta), ..Default::default() }
}
//#endregion 🔖️Diff
