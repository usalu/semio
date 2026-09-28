use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn set_page_parent_clears_the_master_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let mutation = LayoutMutation::SetPageParent(SetPageParent { id: "page-1".into(), parent_page_id: None });
    let next = mutation.diff(&base).diff().apply(&base).expect("parent clears");
    assert_eq!(next.pages[0].parent_page_id, None);
    let restored = mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse");
    assert_eq!(restored.pages[0].parent_page_id.as_deref(), Some("parent-1"));
}
