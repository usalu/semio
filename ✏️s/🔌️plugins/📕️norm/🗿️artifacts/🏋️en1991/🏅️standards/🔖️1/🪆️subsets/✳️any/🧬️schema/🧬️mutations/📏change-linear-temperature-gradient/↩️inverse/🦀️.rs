//! Inverse for `change-linear-temperature-gradient`.
use super::ChangeLinearTemperatureGradient;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeLinearTemperatureGradient, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeLinearTemperatureGradient(ChangeLinearTemperatureGradient { new_delta_t_m: base.delta_t_m })]
}
