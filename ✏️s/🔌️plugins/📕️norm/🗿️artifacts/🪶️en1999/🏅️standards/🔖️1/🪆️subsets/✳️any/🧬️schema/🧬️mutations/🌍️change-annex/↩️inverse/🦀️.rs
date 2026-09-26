//! ↩️ `change-annex` inverse.

use crate::mutations::change_annex::ChangeAnnex;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeAnnex, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
