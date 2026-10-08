use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn set_page_overrides_moves_an_inherited_frame_and_inverse_clears_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::SetPageOverrides(SetPageOverrides { id: "page-1".into(), overrides: vec![PageOverride { object_id: "frame-inherited".into(), bounds: Some(LayoutBounds { x: 90.0, y: 50.0, width: 100.0, height: 80.0, rotation: 0.0 }), visible: None, locked: None }] });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("override applies");
    assert_eq!(next.pages[0].overrides[0].bounds.as_ref().unwrap().x, 90.0);
    assert_eq!(next.parent_pages[0].frames[0].bounds().x, 50.0);
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert!(restored.pages[0].overrides.is_empty());
}
