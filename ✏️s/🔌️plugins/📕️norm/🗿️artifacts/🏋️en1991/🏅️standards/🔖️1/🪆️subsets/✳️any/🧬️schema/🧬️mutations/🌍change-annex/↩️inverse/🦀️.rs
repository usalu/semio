//! Inverse for `change-annex`.
use super::ChangeAnnex;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAnnex, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
