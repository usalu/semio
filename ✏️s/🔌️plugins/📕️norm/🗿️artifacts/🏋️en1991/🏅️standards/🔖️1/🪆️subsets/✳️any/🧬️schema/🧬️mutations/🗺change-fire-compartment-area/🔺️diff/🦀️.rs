//! Diff for `change-fire-compartment-area`.
use super::ChangeFireCompartmentArea;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireCompartmentArea, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_compartment_area == payload.new_fire_compartment_area {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_compartment_area: Some(payload.new_fire_compartment_area), ..Default::default() })
}
