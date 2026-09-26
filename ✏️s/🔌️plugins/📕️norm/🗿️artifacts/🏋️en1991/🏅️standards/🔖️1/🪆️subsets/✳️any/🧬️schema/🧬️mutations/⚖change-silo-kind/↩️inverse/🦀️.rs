//! Inverse for `change-silo-kind`.
use super::ChangeSiloKind;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSiloKind, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSiloKind(ChangeSiloKind { new_silo_kind: base.silo_kind.clone() })]
}
