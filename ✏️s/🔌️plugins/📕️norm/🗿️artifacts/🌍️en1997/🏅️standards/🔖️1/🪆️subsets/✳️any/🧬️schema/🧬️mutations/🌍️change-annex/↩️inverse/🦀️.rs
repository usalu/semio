use super::ChangeAnnex;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(_payload: &ChangeAnnex, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
