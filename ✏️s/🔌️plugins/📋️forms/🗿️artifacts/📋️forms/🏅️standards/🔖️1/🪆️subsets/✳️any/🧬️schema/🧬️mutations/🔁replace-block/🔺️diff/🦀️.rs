//! 🔺️ `replace-block` — sparse diff construction: the question's differing fields as one field patch.

use super::mutation::ReplaceBlock;
use crate::schema::diff::{FormsQuestionPatch, FormsQuestionsDelta, FormsStepPatch, FormsStepsDelta};
use crate::schema::mutations::change_block_field::mutation::BlockField;
use crate::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_replace_block(payload: &ReplaceBlock, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let Some(existing) = step.blocks.iter().find(|block| block.id == payload.block.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist in step \"{}\".", payload.block.id, payload.step_id), [payload.step_id.clone(), payload.block.id.clone()]);
    };
    if existing == &payload.block {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" is already unchanged.", payload.block.id));
    }
    let question = FormsQuestionPatch { kind: (existing.kind != payload.block.kind).then(|| payload.block.kind.clone()), changes: BlockField::changes(existing, &payload.block) };
    let blocks = FormsQuestionsDelta::modification(payload.block.id.clone(), question);
    protocol::MutationOutcome::new(FormsDiff { steps: Some(FormsStepsDelta::modification(payload.step_id.clone(), FormsStepPatch { blocks: Some(blocks), ..Default::default() })), ..Default::default() })
}
//#endregion 🔖️Diff
