use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn set_page_parent_clears_the_master_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::SetPageParent(SetPageParent { id: "page-1".into(), parent_page_id: None });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("parent clears");
    assert_eq!(next.pages[0].parent_page_id, None);
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert_eq!(restored.pages[0].parent_page_id.as_deref(), Some("parent-1"));
}
