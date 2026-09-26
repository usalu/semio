//! Inverse for `change-assumed-crane-horizontal`.
use super::ChangeAssumedCraneHorizontal;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedCraneHorizontal, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedCraneHorizontal(ChangeAssumedCraneHorizontal { new_assumed_crane_horizontal: base.assumed_crane_horizontal })]
}
