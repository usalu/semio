//! ↩️ `change-annex` inverse.

use super::ChangeAnnex;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeAnnex, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
