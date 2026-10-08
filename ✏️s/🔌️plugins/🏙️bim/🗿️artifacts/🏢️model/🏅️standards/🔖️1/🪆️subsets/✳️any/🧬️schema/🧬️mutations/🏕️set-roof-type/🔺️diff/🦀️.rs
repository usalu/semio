//! 🔺️ Diff constructor for `SetRoofType`: a sparse roof type patch of exactly the provided fields. A provided layer stack replaces the whole
//! stack: every layer names an existing material and has a positive finite thickness, and the stack is not empty. A patch that changes nothing is a no-op.
//! Whole-list ruling: the layer stack is ONE owned value, an ordered build-up whose order and thicknesses only mean something
//! together; its layers have no identity and nothing references one, so a provided stack replaces the stack as one field.

use super::SetRoofType;
use crate::{Entry, Layer, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

fn fault(layers: &[Layer], base: &ModelSnapshot) -> Option<(OutcomeCode, String, Option<&'static str>)> {
    if let Some(layer) = layers.iter().find(|layer| !base.materials.contains_key(&layer.material)) {
        return Some((OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", layer.material), Some("material")));
    }
    if layers.is_empty() {
        return Some((OutcomeCode::Invariant, "A roof type needs at least one layer.".to_string(), None));
    }
    if layers.iter().any(|layer| !(layer.thickness.is_finite() && layer.thickness > 0.0)) {
        return Some((OutcomeCode::Invariant, "Every layer needs a positive thickness.".to_string(), Some("thickness")));
    }
    None
}

pub fn diff(payload: &SetRoofType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(current) = base.roof_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some((code, message, field)) = payload.layers.as_deref().and_then(|layers| fault(layers, base)) {
        return MutationOutcome::refuse(code, message, ["layers"].into_iter().chain(field));
    }
    let change = payload.patch().minimal(current);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Roof type \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::roof_types(payload.id.clone(), Entry::Patched(change)))
}
