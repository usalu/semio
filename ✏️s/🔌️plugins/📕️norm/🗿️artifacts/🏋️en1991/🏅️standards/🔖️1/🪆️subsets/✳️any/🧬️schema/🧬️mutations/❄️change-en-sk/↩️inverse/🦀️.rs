//! Inverse for `change-en-sk`.
use super::ChangeEnSk;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeEnSk, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeEnSk(ChangeEnSk { new_en_sk: base.en_sk })]
}
