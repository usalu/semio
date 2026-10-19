//! 🔺️ Diff constructor for `SetMaterial`: a sparse material patch of exactly the provided fields. Physical values stay finite and
//! non-negative, colour channels in 0..1; a patch that changes nothing is a no-op. Everything derived from the material follows by inference.

use crate::standards::v1::subsets::any::schema::mutations::set_material::SetMaterial;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::MaterialPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Rgb;
use protocol::{MutationOutcome, OutcomeCode};

fn unit(color: &Rgb) -> bool {
    [color.r, color.g, color.b].into_iter().all(|channel| channel.is_finite() && (0.0..=1.0).contains(&channel))
}

fn fault(change: &MaterialPatch) -> Option<&'static str> {
    [("density", change.density), ("conductivity", change.conductivity), ("specific_heat", change.specific_heat)]
        .into_iter()
        .find_map(|(field, value)| value.filter(|value| !(value.is_finite() && *value >= 0.0)).map(|_| field))
        .or_else(|| change.color.filter(|color| !unit(color)).map(|_| "color"))
}

pub fn diff(payload: &SetMaterial, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(material) = base.materials.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let change = payload.patch();
    if let Some(field) = fault(&change) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Material {field} is out of range."), [field]);
    }
    let change = change.minimal(material);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Material \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::materials(payload.id.clone(), Entry::Patched(change)))
}
