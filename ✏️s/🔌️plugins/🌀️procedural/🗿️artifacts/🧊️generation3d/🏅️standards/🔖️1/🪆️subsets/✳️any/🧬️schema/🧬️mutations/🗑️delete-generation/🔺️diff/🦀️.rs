//! 🔺️ `delete-generation` sparse diff construction. `FormGeneration` carries no widget/synapse
//! reference, so removing a generation never cascades into the fixture.

use crate::standards::v1::subsets::any::schema::diff::{Generation3dDiff, Generation3dGenerationsDelta, Generation3dSelectionChange};
use crate::standards::v1::subsets::any::schema::mutations::delete_generation::DeleteGeneration;
use crate::Generation3dSnapshot;

pub fn diff(payload: &DeleteGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if !base.generation.generations.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let selection = (base.generation.selected_generation_id.as_deref() == Some(payload.id.as_str())).then(|| Generation3dSelectionChange { id: base.generation.generations.iter().find(|entry| entry.id != payload.id).map(|entry| entry.id.clone()) });
    protocol::MutationOutcome::new(Generation3dDiff { generations: Some(Generation3dGenerationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), selected_generation: selection, ..Default::default() })
}
