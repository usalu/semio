//! Inverse for `change-assumed-construction-qk`.
use super::ChangeAssumedConstructionQk;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedConstructionQk, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedConstructionQk(ChangeAssumedConstructionQk { new_assumed_construction_qk: base.assumed_construction_qk })]
}
