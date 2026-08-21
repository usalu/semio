//! 🔺️ `create-block` — sparse diff construction: clones only the touched step's own `blocks` Vec
//! (bounded, single-step scope — never the whole document).

use super::mutation::CreateBlock;
use crate::artifacts::forms::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub async fn diff_create_block(payload: &CreateBlock, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    if step.blocks.iter().any(|block| block.id == payload.block.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A block with id \"{}\" already exists in step \"{}\".", payload.block.id, payload.step_id), [payload.step_id.clone(), payload.block.id.clone()]);
    }
    let mut blocks = step.blocks.clone();
    let at = payload.index.unwrap_or(blocks.len()).min(blocks.len());
    blocks.insert(at, payload.block.clone());
    let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.step_id.clone(), patch }], ..Default::default() }, base))
}
//#endregion 🔖️Diff
