//! Inverse for `change-fire-opening-factor`.
use super::ChangeFireOpeningFactor;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeFireOpeningFactor, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireOpeningFactor(ChangeFireOpeningFactor { new_fire_opening_factor: base.fire_opening_factor })]
}
