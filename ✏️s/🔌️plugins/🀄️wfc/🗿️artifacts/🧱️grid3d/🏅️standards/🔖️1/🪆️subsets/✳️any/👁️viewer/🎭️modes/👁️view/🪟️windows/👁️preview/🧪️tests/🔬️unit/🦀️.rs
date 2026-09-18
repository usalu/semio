//! 🔬️ The viewer's preview renders the solved grid over the same pure projection the editor uses.

use super::*;

#[test]
fn every_example_renders_a_non_empty_solved_scene() {
    for document in [crate::examples::blocks::snapshot(), crate::examples::pipes_3d::snapshot()] {
        let node = render(&document).expect("viewer preview renders");
        assert!(!format!("{node:?}").is_empty());
    }
}

#[test]
fn the_opening_pose_frames_the_document() {
    let document = crate::examples::blocks::snapshot();
    let (position, target) = framed_camera(&document);
    assert_ne!(position, target);
}
