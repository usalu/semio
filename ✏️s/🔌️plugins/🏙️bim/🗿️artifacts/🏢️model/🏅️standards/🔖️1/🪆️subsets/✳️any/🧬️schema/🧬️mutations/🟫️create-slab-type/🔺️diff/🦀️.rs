//! 🔺️ Diff constructor for `CreateSlabType`: one created slab type entry. Every layer names an existing material and has a positive finite
//! thickness; a slab type has at least one layer.

use crate::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Layer;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

fn fault(layers: &[Layer], base: &ModelSnapshot) -> Option<(OutcomeCode, String, Option<&'static str>)> {
    if let Some(layer) = layers.iter().find(|layer| !base.materials.contains_key(&layer.material)) {
        return Some((OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", layer.material), Some("material")));
    }
    if layers.is_empty() {
        return Some((OutcomeCode::Invariant, "A slab type needs at least one layer.".to_string(), None));
    }
    if layers.iter().any(|layer| !(layer.thickness.is_finite() && layer.thickness > 0.0)) {
        return Some((OutcomeCode::Invariant, "Every layer needs a positive thickness.".to_string(), Some("thickness")));
    }
    None
}

pub fn diff(payload: &CreateSlabType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some((code, message, field)) = fault(&payload.slab_type.layers, base) {
        return MutationOutcome::refuse(code, message, ["slab_type", "layers"].into_iter().chain(field));
    }
    MutationOutcome::new(ModelDiff::slab_types(payload.id.clone(), Entry::Created(payload.slab_type.clone())))
}
