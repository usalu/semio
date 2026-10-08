//! 🔺️ `move-block-to-step` — sparse diff construction: a question reorder inside one step, or
//! a question removal in the source step plus an addition in the destination step.

use super::mutation::MoveBlockToStep;
use crate::schema::diff::forms_diff_from_delta;
use crate::schema::diff::{FormsQuestionsDelta, FormsStepPatch, FormsStepsDelta};
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_move_block_to_step(payload: &MoveBlockToStep, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(source_step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let Some(current_index) = source_step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist in step \"{}\".", payload.block_id, payload.step_id), [payload.step_id.clone(), payload.block_id.clone()]);
    };
    let block = source_step.blocks[current_index].clone();
    let placed = |blocks: &[crate::FormQuestion], at: usize| {
        let mut order: Vec<String> = blocks.iter().filter(|candidate| candidate.id != payload.block_id).map(|candidate| candidate.id.clone()).collect();
        order.insert(at.min(order.len()), payload.block_id.clone());
        order
    };

    if payload.step_id == payload.to_step_id {
        let at = payload.index.min(source_step.blocks.len() - 1);
        if at == current_index {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" is already at index {at} in step \"{}\".", payload.block_id, payload.step_id));
        }
        let blocks = FormsQuestionsDelta { reordered: Some(placed(&source_step.blocks, at)), ..Default::default() };
        return protocol::MutationOutcome::new(forms_diff_from_delta(&FormsStepsDelta { patched: vec![FormsStepPatch { id: payload.step_id.clone(), blocks: Some(blocks), ..Default::default() }], ..Default::default() }, base));
    }

    let Some(dest_step) = steps.iter().find(|step| step.id == payload.to_step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.to_step_id), [payload.to_step_id.clone()]);
    };
    let leaving = FormsQuestionsDelta { removed: vec![payload.block_id.clone()], ..Default::default() };
    let arriving = FormsQuestionsDelta { added: vec![block], reordered: (payload.index < dest_step.blocks.len()).then(|| placed(&dest_step.blocks, payload.index)), ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(
        &FormsStepsDelta {
            patched: vec![
                FormsStepPatch { id: payload.step_id.clone(), blocks: Some(leaving), ..Default::default() },
                FormsStepPatch { id: payload.to_step_id.clone(), blocks: Some(arriving), ..Default::default() },
            ],
            ..Default::default()
        },
        base,
    ))
}
//#endregion 🔖️Diff
