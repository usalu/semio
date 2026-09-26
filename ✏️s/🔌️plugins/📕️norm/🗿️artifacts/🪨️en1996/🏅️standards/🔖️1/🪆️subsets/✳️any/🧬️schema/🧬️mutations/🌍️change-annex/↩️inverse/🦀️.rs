use super::ChangeAnnex;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeAnnex, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
