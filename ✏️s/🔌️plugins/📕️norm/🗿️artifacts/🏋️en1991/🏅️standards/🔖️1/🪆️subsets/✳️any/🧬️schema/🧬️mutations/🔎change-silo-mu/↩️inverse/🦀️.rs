//! Inverse for `change-silo-mu`.
use super::ChangeSiloMu;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloMu, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloMu(ChangeSiloMu { new_silo_mu: base.silo_mu })]
}
