//! 🔺️ Diff for `change-fire-compartment-area`.
use crate::change_fire_compartment_area::ChangeFireCompartmentArea;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeFireCompartmentArea, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_compartment_area == payload.new_fire_compartment_area {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_compartment_area: Some(payload.new_fire_compartment_area), ..Default::default() })
}
