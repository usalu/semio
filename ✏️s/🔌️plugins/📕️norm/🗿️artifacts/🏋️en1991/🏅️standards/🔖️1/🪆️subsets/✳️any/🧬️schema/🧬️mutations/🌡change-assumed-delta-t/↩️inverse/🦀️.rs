//! Inverse for `change-assumed-delta-t`.
use super::ChangeAssumedDeltaT;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedDeltaT, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedDeltaT(ChangeAssumedDeltaT { new_assumed_delta_t: base.assumed_delta_t })]
}
