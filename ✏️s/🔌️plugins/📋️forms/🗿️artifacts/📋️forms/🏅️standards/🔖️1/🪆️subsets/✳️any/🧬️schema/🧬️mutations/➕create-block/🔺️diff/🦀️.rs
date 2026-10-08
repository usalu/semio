//! 🔺️ `create-block` — sparse diff construction: one added question (and the order when it is not appended) in the touched step.

use super::mutation::CreateBlock;
use crate::schema::diff::forms_diff_from_delta;
use crate::schema::diff::{FormsQuestionsDelta, FormsStepPatch, FormsStepsDelta};
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_create_block(payload: &CreateBlock, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    if step.blocks.iter().any(|block| block.id == payload.block.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A block with id \"{}\" already exists in step \"{}\".", payload.block.id, payload.step_id), [payload.step_id.clone(), payload.block.id.clone()]);
    }
    let reordered = payload.index.filter(|index| *index < step.blocks.len()).map(|index| {
        let mut order: Vec<String> = step.blocks.iter().map(|block| block.id.clone()).collect();
        order.insert(index, payload.block.id.clone());
        order
    });
    let blocks = FormsQuestionsDelta { added: vec![payload.block.clone()], reordered, ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(&FormsStepsDelta { patched: vec![FormsStepPatch { id: payload.step_id.clone(), blocks: Some(blocks), ..Default::default() }], ..Default::default() }, base))
}
//#endregion 🔖️Diff
