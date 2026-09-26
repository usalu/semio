//! Diff for `change-fire-load-density-qf`.
use super::ChangeFireLoadDensityQf;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireLoadDensityQf, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_load_density_qf == payload.new_fire_load_density_qf {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_load_density_qf: Some(payload.new_fire_load_density_qf), ..Default::default() })
}
