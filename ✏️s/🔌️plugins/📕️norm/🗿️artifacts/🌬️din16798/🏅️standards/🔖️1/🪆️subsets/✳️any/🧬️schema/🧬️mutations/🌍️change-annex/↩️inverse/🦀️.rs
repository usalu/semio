//! ↩️ `change-annex` inverse.
use super::ChangeAnnex;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeAnnex, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
