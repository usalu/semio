//! Inverse for `change-annex`.
use super::ChangeAnnex;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &ChangeAnnex, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex.clone() })]
}
