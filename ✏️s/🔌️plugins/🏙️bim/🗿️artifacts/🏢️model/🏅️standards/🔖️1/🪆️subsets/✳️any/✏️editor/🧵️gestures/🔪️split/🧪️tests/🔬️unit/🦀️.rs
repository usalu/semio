use crate::editor::bim::gestures::tests::fixture::{room, Rig};
use crate::editor::bim::gestures::session::Shape;
use crate::{Axis, ModelMutation};

fn ends(rig: &Rig, wall: &str) -> ([f64; 2], [f64; 2]) {
    let (Axis::Line { start, end } | Axis::Arc { start, end, .. }) = &rig.snapshot.walls[wall].axis;
    ([start.x, start.y], [end.x, end.y])
}

#[semio_framework_async_macros::async_test]
async fn a_click_on_a_wall_parts_it_at_the_foot_of_the_pointer_in_one_split_wall() {
    let mut rig = Rig::plan("split-wall", room());
    let step = rig.down(2.0, 0.05);
    let [ModelMutation::SplitWall(split)] = step.mutations.as_slice() else { panic!("one split-wall, got {:?}", step.mutations) };
    assert!((split.t - 0.25).abs() < 1e-9 && split.id == "w-south", "{split:?}");
    assert_eq!(rig.snapshot.walls.len(), 5);
    assert_eq!(ends(&rig, "w-south"), ([0.0, 0.0], [2.0, 0.0]));
    assert_eq!(ends(&rig, &split.new_id), ([2.0, 0.0], [8.0, 0.0]), "the new half continues the old one");
}

#[semio_framework_async_macros::async_test]
async fn hovering_a_wall_shows_the_cut_and_leaves_the_model_alone() {
    let mut rig = Rig::plan("split-wall", room());
    let before = rig.snapshot.clone();
    rig.mv(4.0, 0.0);
    assert!(rig.shows(Shape::Path), "the cut across the wall is drawn");
    assert_eq!(rig.snapshot, before);
    rig.mv(4.0, 3.0);
    assert!(rig.preview.is_empty(), "away from every wall nothing shows");
}

#[semio_framework_async_macros::async_test]
async fn a_cut_at_an_end_is_refused_and_a_click_on_nothing_does_nothing() {
    let mut rig = Rig::plan("split-wall", room());
    let end = rig.down(0.0, 0.0);
    assert!(end.mutations.is_empty() && end.refused.is_some(), "an end of the wall is no place to split");
    let nothing = rig.down(4.0, 3.0);
    assert!(nothing.mutations.is_empty() && nothing.refused.is_none());
    assert_eq!(rig.snapshot.walls.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn two_splits_of_one_session_mint_different_ids() {
    let mut rig = Rig::plan("split-wall", room());
    rig.down(2.0, 0.0);
    rig.down(6.0, 0.0);
    assert_eq!(rig.snapshot.walls.len(), 6);
}
