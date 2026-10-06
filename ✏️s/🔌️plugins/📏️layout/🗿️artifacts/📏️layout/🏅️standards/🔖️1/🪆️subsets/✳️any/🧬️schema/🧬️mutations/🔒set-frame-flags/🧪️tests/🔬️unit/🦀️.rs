use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn set_frame_flags_locks_a_frame_and_inverse_unlocks_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::SetFrameFlags(SetFrameFlags { page_id: "page-1".into(), frame_id: "frame-1".into(), locked: Some(true), visible: Some(false) });
    let next = mutation.diff(&base).diff().apply(&base).expect("flags apply");
    let frame = next.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
    assert!(frame.locked());
    assert!(!frame.visible());
    let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff().apply(&next).expect("inverse applies");
    let frame = restored.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
    assert!(!frame.locked());
    assert!(frame.visible());
}
