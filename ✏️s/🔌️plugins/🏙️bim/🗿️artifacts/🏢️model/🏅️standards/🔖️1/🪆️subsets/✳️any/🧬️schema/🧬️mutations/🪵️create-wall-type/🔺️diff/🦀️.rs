//! 🔺️ Diff constructor for `CreateWallType`: one created wall type entry. Every layer names an existing material and has a positive finite
//! thickness; a wall type has at least one layer.

use super::super::elements;
use super::CreateWallType;
use crate::{Entry, Layer, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

fn fault(layers: &[Layer], base: &ModelSnapshot) -> Option<(OutcomeCode, String, Option<&'static str>)> {
    if let Some(layer) = layers.iter().find(|layer| !base.materials.contains_key(&layer.material)) {
        return Some((OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", layer.material), Some("material")));
    }
    if layers.is_empty() {
        return Some((OutcomeCode::Invariant, "A wall type needs at least one layer.".to_string(), None));
    }
    if layers.iter().any(|layer| !(layer.thickness.is_finite() && layer.thickness > 0.0)) {
        return Some((OutcomeCode::Invariant, "Every layer needs a positive thickness.".to_string(), Some("thickness")));
    }
    None
}

pub fn diff(payload: &CreateWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some((code, message, field)) = fault(&payload.wall_type.layers, base) {
        return MutationOutcome::refuse(code, message, ["wall_type", "layers"].into_iter().chain(field));
    }
    MutationOutcome::new(ModelDiff::wall_types(payload.id.clone(), Entry::Created(payload.wall_type.clone())))
}
