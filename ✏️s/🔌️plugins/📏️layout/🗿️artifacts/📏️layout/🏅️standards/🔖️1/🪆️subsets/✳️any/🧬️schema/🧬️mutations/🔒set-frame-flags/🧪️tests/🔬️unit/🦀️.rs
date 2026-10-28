use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use super::*;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use protocol::{ColdMutation, Mutation, MutationDiff};

#[test]
fn set_frame_flags_locks_a_frame_and_inverse_unlocks_it() {
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let mutation = LayoutMutation::SetFrameFlags(SetFrameFlags { page_id: "page-1".into(), frame_id: "frame-1".into(), locked: Some(true), visible: Some(false) });
    let next = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("flags apply");
    let frame = next.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
    assert!(frame.locked());
    assert!(!frame.visible());
    let restored = protocol::apply_diff_cold(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    let frame = restored.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
    assert!(!frame.locked());
    assert!(frame.visible());
}
