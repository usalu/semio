//! Diff for `change-silo-hydraulic-radius`.
use super::ChangeSiloHydraulicRadius;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeSiloHydraulicRadius, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_hydraulic_radius == payload.new_silo_hydraulic_radius {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_hydraulic_radius: Some(payload.new_silo_hydraulic_radius), ..Default::default() })
}
