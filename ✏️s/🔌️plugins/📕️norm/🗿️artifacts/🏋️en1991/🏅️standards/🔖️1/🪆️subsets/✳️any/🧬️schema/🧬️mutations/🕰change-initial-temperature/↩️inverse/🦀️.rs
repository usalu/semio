//! Inverse for `change-initial-temperature`.
use super::ChangeInitialTemperature;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeInitialTemperature, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeInitialTemperature(ChangeInitialTemperature { new_t_0: base.t_0 })]
}
