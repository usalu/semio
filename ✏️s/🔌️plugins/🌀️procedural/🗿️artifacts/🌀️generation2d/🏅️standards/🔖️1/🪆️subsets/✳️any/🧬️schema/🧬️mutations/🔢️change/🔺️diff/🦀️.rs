//! 🔺️ Sparse diff for `ChangeGenerationValue`, built directly from `(payload, base)`.
use super::ChangeGenerationValue;
use crate::standards::v1::subsets::any::schema::diff::{Generation2dGenerationPatch, Generation2dGenerationPatchEntry, Generation2dGenerationsDelta, Generation2dValueRow, Generation2dValuesDelta};
use crate::{Generation2dDiff, Generation2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeGenerationValue, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let Some(entry) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.values.get(&payload.question_id) == Some(&payload.value) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Generation \"{}\" question \"{}\" already has this value.", payload.id, payload.question_id));
    }
    let row = Generation2dValueRow { question_id: payload.question_id.clone(), value: payload.value.clone() };
    let values = if entry.values.contains_key(&payload.question_id) { Generation2dValuesDelta { patched: vec![row], ..Default::default() } } else { Generation2dValuesDelta { added: vec![row], ..Default::default() } };
    protocol::MutationOutcome::new(Generation2dDiff { generations: Some(Generation2dGenerationsDelta { patched: vec![Generation2dGenerationPatchEntry { id: payload.id.clone(), patch: Generation2dGenerationPatch { values: Some(values), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
