use super::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn update_text_frame_sets_inset_and_thread_and_inverse_restores_them() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
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
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("inset applies");
    let semio_s_artifact_layout_layout_model::Frame::Text { inset, .. } = next.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap() else { panic!("text") };
    assert_eq!(inset.x, 4.0);
    assert_eq!(inset.y, 6.0);
    assert_eq!(inset.width, 72.0);
    assert_eq!(inset.height, 28.0);
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    let semio_s_artifact_layout_layout_model::Frame::Text { inset: restored_inset, .. } = restored.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap() else { panic!("text") };
    let semio_s_artifact_layout_layout_model::Frame::Text { inset: base_inset, .. } = base.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap() else { panic!("text") };
    assert_eq!(restored_inset, base_inset);
}
