use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn set_page_guides_adds_a_guide_and_inverse_clears_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::SetPageGuides(SetPageGuides { id: "page-1".into(), guides: vec![LayoutRect { x: 12.0, y: 40.0, width: 200.0, height: 0.0 }] });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("guides apply");
    assert_eq!(next.pages[0].guides.len(), 1);
    assert_eq!(next.pages[0].guides[0].x, 12.0);
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert!(restored.pages[0].guides.is_empty());
}
