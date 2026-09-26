use crate::mutations::change_annex::ChangeAnnex;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(_payload: &ChangeAnnex, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
