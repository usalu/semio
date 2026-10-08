//! 🔺️ Diff constructor for `CreateMaterial`: one created material entry. Density, conductivity and specific heat are finite and non-negative; every colour channel lies in 0..1.

use super::super::elements;
use super::CreateMaterial;
use crate::{Entry, ModelDiff, ModelSnapshot, Rgb};
use protocol::{MutationOutcome, OutcomeCode};

fn unit(color: &Rgb) -> bool {
    [color.r, color.g, color.b].into_iter().all(|channel| channel.is_finite() && (0.0..=1.0).contains(&channel))
}

pub fn diff(payload: &CreateMaterial, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let material = &payload.material;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let physical = [("density", material.density), ("conductivity", material.conductivity), ("specific_heat", material.specific_heat)];
    if let Some((field, _)) = physical.into_iter().find(|(_, value)| !(value.is_finite() && *value >= 0.0)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Material {field} must be a finite, non-negative number."), ["material", field]);
    }
    if !unit(&material.color) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "Every colour channel must lie between 0 and 1.", ["material", "color"]);
    }
    MutationOutcome::new(ModelDiff::materials(payload.id.clone(), Entry::Created(material.clone())))
}
