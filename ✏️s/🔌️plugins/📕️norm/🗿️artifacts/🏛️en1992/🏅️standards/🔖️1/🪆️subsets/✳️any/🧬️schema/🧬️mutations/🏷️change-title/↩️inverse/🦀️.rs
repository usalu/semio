use crate::mutations::change_title::ChangeTitle;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(_payload: &ChangeTitle, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeTitle(ChangeTitle { new_title: base.title.clone() })]
}
