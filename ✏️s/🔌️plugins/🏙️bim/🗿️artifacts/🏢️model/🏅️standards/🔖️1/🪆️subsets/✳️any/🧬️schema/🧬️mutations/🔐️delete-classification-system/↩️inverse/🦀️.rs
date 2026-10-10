//! ↩️ Inverse of `DeleteClassificationSystem`: one concrete `SetElementClassification` per removed classification and, last in storage order so that the store (which replays the vector reversed) recreates the
//! system first, the `CreateClassificationSystem` carrying the full removed record; none when the system is absent or the cascade is refused.

use super::super::cascade;
use super::super::create_classification_system::CreateClassificationSystem;
use super::super::set_element_classification::SetElementClassification;
use super::DeleteClassificationSystem;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteClassificationSystem, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(system) = base.classification_systems.get(&payload.id) else {
        return Vec::new();
    };
    let holders = cascade::classified_by(base, &payload.id);
    if holders.len() > cascade::INVERSE_ROWS {
        return Vec::new();
    }
    holders
        .into_iter()
        .map(|(holder, code)| ModelMutation::SetElementClassification(SetElementClassification { id: holder, system: payload.id.clone(), code }))
        .chain(std::iter::once(ModelMutation::CreateClassificationSystem(CreateClassificationSystem { id: payload.id.clone(), system: system.clone() })))
        .collect()
}
