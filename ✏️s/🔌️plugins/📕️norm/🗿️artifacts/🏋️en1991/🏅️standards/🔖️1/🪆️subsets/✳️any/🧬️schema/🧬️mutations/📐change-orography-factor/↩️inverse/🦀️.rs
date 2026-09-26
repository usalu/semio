//! Inverse for `change-orography-factor`.
use super::ChangeOrographyFactor;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeOrographyFactor, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeOrographyFactor(ChangeOrographyFactor { new_orography_factor: base.orography_factor })]
}
