//! 🔺️ `delete-block` — sparse diff construction: clones only the touched step's own `blocks` Vec.

use super::mutation::DeleteBlock;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::schema::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_delete_block(payload: &DeleteBlock, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    if !step.blocks.iter().any(|block| block.id == payload.id) {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Block \"{}\" does not exist in step \"{}\".", payload.id, payload.step_id),
            [payload.step_id.clone(), payload.id.clone()],
        );
    }
    let blocks: Vec<_> = step.blocks.iter().filter(|block| block.id != payload.id).cloned().collect();
    let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.step_id.clone(), patch }], ..Default::default() }, base))
}
//#endregion 🔖️Diff
