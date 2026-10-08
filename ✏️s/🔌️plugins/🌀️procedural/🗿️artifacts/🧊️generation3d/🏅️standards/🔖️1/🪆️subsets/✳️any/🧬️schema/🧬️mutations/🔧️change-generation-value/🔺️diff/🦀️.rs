//! 🔺️ `change-generation-value` sparse diff construction.

use crate::standards::v1::subsets::any::schema::diff::{Generation3dDiff, Generation3dGenerationPatch, Generation3dGenerationModification, Generation3dGenerationsDelta, Generation3dValueRow, Generation3dValuesDelta};
use crate::standards::v1::subsets::any::schema::mutations::change_generation_value::ChangeGenerationValue;
use crate::Generation3dSnapshot;

pub fn diff(payload: &ChangeGenerationValue, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let Some(existing) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.values.get(&payload.question_id) == Some(&payload.new_value) {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", format!("Generation \"{}\" question \"{}\" already has the requested value.", payload.id, payload.question_id));
    }
    let row = Generation3dValueRow { question_id: payload.question_id.clone(), value: payload.new_value.clone() };
    let values = if existing.values.contains_key(&payload.question_id) { Generation3dValuesDelta { patched: vec![row], ..Default::default() } } else { Generation3dValuesDelta { added: vec![row], ..Default::default() } };
    protocol::MutationOutcome::new(Generation3dDiff { generations: Some(Generation3dGenerationsDelta { modified: vec![Generation3dGenerationModification { id: payload.id.clone(), patch: Generation3dGenerationPatch { values: Some(values), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}
