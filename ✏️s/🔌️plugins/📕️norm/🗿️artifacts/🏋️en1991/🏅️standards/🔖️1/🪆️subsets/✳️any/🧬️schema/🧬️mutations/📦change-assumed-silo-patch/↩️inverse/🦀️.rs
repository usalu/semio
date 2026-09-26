//! Inverse for `change-assumed-silo-patch`.
use super::ChangeAssumedSiloPatch;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedSiloPatch, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedSiloPatch(ChangeAssumedSiloPatch { new_assumed_silo_patch: base.assumed_silo_patch })]
}
