//! 🔺️ Diff constructor for `SetSlabBoundary`: the boundary and the holes are validated together as one outline and patched as far
//! as they differ. Both loops follow the create rules (counter-clockwise, no self-intersection, holes strictly inside); an outline
//! equal to the current one is a no-op.
//! Whole-list ruling: the boundary and the holes are ONE planar region that only validates together (every hole strictly inside
//! the boundary); holes have no identity, so the region replaces as the boundary and holes it names.

use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::horizontal_rules::{holes_fault, loop_fault};
use crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::columns_beams_slabs::SlabPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSlabBoundary, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(slab) = base.slabs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = loop_fault(&payload.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["boundary"]);
    }
    if let Some(fault) = holes_fault(&payload.boundary, &payload.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["holes"]);
    }
    let patch = SlabPatch {
        boundary: (payload.boundary != slab.boundary).then(|| payload.boundary.clone()),
        holes: (payload.holes != slab.holes).then(|| payload.holes.clone()),
        ..Default::default()
    };
    if patch == SlabPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Slab \"{}\" already has this outline.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::slabs(payload.id.clone(), Entry::Patched(patch)))
}
