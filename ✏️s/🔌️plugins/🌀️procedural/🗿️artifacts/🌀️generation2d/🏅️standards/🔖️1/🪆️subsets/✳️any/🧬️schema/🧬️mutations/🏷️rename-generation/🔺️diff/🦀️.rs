//! 🔺️ Sparse diff for `RenameGeneration`, built directly from `(payload, base)`.
use super::RenameGeneration;
use crate::standards::v1::subsets::any::schema::diff::diff_generation_from_ops;
use crate::{Generation2dDiff, Generation2dSnapshot};
use semio_framework_artifact_playbook_playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &RenameGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let Some(entry) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.name == payload.name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Generation \"{}\" is already named \"{}\".", payload.id, payload.name));
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Rename { id: payload.id.clone(), name: payload.name.clone() }]))
}
//#endregion 🔖️Diff
