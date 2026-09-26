//! Diff for `change-fire-compartment-height`.
use super::ChangeFireCompartmentHeight;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireCompartmentHeight, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_compartment_height == payload.new_fire_compartment_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_compartment_height: Some(payload.new_fire_compartment_height), ..Default::default() })
}
