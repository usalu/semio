//! Inverse for `change-assumed-crane-wheel`.
use super::ChangeAssumedCraneWheel;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedCraneWheel, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedCraneWheel(ChangeAssumedCraneWheel { new_assumed_crane_wheel: base.assumed_crane_wheel })]
}
