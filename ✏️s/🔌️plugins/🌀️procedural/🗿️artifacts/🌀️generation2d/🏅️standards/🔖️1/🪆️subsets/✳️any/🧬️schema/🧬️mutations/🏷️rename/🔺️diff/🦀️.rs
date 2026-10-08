//! 🔺️ Sparse diff for `RenameGeneration`, built directly from `(payload, base)`.
use super::RenameGeneration;
use crate::standards::v1::subsets::any::schema::diff::{Generation2dGenerationPatch, Generation2dGenerationModification, Generation2dGenerationsDelta};
use crate::{Generation2dDiff, Generation2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let Some(entry) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.name == payload.name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Generation \"{}\" is already named \"{}\".", payload.id, payload.name));
    }
    protocol::MutationOutcome::new(Generation2dDiff { generations: Some(Generation2dGenerationsDelta { modified: vec![Generation2dGenerationModification { id: payload.id.clone(), patch: Generation2dGenerationPatch { name: Some(payload.name.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
