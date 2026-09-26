//! Diff for `change-fire-thermal-inertia`.
use super::ChangeFireThermalInertia;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireThermalInertia, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_thermal_inertia == payload.new_fire_thermal_inertia {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_thermal_inertia: Some(payload.new_fire_thermal_inertia), ..Default::default() })
}
