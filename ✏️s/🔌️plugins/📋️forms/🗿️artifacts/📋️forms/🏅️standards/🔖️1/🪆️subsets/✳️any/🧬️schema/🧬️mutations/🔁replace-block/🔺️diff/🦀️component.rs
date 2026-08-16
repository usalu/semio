//! 🔺️ `replace-block` — sparse diff construction: clones only the touched step's own `blocks` Vec.

use super::mutation::ReplaceBlock;
use crate::artifacts::forms::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::schema::diff::text::forms_diff_from_delta;
use crate::artifacts::forms::{forms_steps, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_replace_block(payload: &ReplaceBlock, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.step_id), [payload.step_id.clone()]);
    };
    let Some(existing) = step.blocks.iter().find(|block| block.id == payload.block.id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Block \"{}\" does not exist in step \"{}\".", payload.block.id, payload.step_id),
            [payload.step_id.clone(), payload.block.id.clone()],
        );
    };
    if existing == &payload.block {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" is already unchanged.", payload.block.id));
    }
    let blocks: Vec<_> = step.blocks.iter().map(|block| if block.id == payload.block.id { payload.block.clone() } else { block.clone() }).collect();
    let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: payload.step_id.clone(), patch }], ..Default::default() }, base))
}
//#endregion 🔖️Diff
