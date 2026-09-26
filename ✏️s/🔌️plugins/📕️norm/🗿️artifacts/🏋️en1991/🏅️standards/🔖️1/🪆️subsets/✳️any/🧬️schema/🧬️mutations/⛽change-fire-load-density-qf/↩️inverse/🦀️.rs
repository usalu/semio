//! Inverse for `change-fire-load-density-qf`.
use super::ChangeFireLoadDensityQf;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireLoadDensityQf, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireLoadDensityQf(ChangeFireLoadDensityQf { new_fire_load_density_qf: base.fire_load_density_qf })]
}
