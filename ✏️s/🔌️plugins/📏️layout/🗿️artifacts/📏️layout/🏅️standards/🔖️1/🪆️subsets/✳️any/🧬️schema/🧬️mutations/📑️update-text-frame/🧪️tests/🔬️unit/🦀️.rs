use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_text_frame_sets_inset_and_thread_and_inverse_restores_them() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let mutation = LayoutMutation::UpdateTextFrame(UpdateTextFrame {
        page_id: "page-1".into(),
        frame_id: "frame-text-1".into(),
        story_id: "story-1".into(),
        thread_next: None,
        inset_x: 4.0,
        inset_y: 6.0,
        inset_width: 72.0,
        inset_height: 28.0,
    });
    let next = mutation.diff(&base).diff().apply(&base).expect("inset applies");
    let crate::Frame::Text { inset, .. } = next.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap() else { panic!("text") };
    assert_eq!(inset.x, 4.0);
    assert_eq!(inset.y, 6.0);
    assert_eq!(inset.width, 72.0);
    assert_eq!(inset.height, 28.0);
    let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff().apply(&next).expect("inverse applies");
    let crate::Frame::Text { inset: restored_inset, .. } = restored.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap() else { panic!("text") };
    let crate::Frame::Text { inset: base_inset, .. } = base.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap() else { panic!("text") };
    assert_eq!(restored_inset, base_inset);
}
