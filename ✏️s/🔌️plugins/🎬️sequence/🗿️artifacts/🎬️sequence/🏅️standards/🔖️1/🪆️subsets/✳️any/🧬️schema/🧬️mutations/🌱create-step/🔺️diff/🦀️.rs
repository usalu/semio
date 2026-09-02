//! 🔺️ Sparse diff builder for `CreateStep` — a real append-only insert (never a whole-snapshot
//! capture). Reads the CURRENT scene off `base` via `sequence_working_scene`, applies the same
//! append semantics against it, then mints a whole new content handle via `diff_replace_content`
//! (the composed child is opaque — a parent's diff never embeds a child diff).
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{diff_replace_content, sequence_working_scene, SequenceSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::CreateStep, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
    let scene = sequence_working_scene(base);
    if scene.steps.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let mut steps = scene.steps;
    steps.push(payload.step.clone());
    protocol::MutationOutcome::new(diff_replace_content(steps, scene.edges))
}
//#endregion 🔖️Diff
