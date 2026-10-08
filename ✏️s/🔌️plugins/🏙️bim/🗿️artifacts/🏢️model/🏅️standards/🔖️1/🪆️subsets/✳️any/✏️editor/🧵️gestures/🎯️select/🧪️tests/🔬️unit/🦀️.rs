use super::*;
use crate::editor::bim::gestures::session::{Modifiers, Pick, Shape, Surface, ToolEvent};
use crate::editor::bim::gestures::tests::fixture::{room, Rig};
use crate::{Opening, OpeningKind, Point2};

fn target(kind: &str, id: &str) -> (String, String) {
    (kind.to_string(), id.to_string())
}

fn room_with_window() -> ModelSnapshot {
    let mut snapshot = room();
    snapshot.openings.insert("o-1".into(), Opening { host: "w-south".into(), kind: OpeningKind::Window { window_type: "win-12".into() }, offset: 2.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: "Window 1".into() });
    snapshot
}

fn only_mutation(step: &Step) -> &ModelMutation {
    match step.mutations.as_slice() {
        [mutation] => mutation,
        other => panic!("one mutation expected, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn a_press_on_an_element_selects_it_with_the_framework_merge_modes() {
    let mut rig = Rig::plan("select", room());
    let step = rig.down(4.0, 0.0);
    assert_eq!(step.pick, Some(Pick { targets: vec![target("wall", "w-south")], merge: "replace" }));
    assert!(step.mutations.is_empty(), "selection is never a mutation");
    let shifted = rig.send(ToolEvent::Down(rig.pointer(4.0, 0.0, Modifiers { shift: true, ctrl: false, meta: false })));
    assert_eq!(shifted.pick.map(|pick| pick.merge), Some("additive"));
    let subtracted = rig.send(ToolEvent::Down(rig.pointer(4.0, 0.0, Modifiers { shift: false, ctrl: true, meta: false })));
    assert_eq!(subtracted.pick.map(|pick| pick.merge), Some("subtractive"));
}

#[semio_framework_async_macros::async_test]
async fn a_press_on_nothing_clears_the_selection_and_a_drag_selects_what_the_marquee_contains() {
    let mut rig = Rig::plan("select", room());
    let clear = rig.down(-1.0, -1.0);
    assert_eq!(clear.pick, Some(Pick { targets: Vec::new(), merge: "replace" }));
    rig.mv(9.0, 7.0);
    assert!(rig.shows(Shape::Path), "the marquee is drawn");
    let step = rig.up(9.0, 7.0);
    let mut ids: Vec<String> = step.pick.expect("a marquee selects").targets.into_iter().map(|(_, id)| id).collect();
    ids.sort();
    assert_eq!(ids, vec!["w-east", "w-north", "w-south", "w-west"], "left to right selects what the rectangle contains");
    let mut rig = Rig::plan("select", room());
    rig.down(9.0, 7.0);
    let touching = rig.up(4.0, 2.0);
    assert!(touching.pick.expect("a crossing marquee selects").targets.iter().all(|(kind, _)| kind == "wall"));
    let mut rig = Rig::plan("select", room());
    rig.down(2.0, 1.0);
    let contained = rig.up(6.0, 5.0);
    assert!(contained.pick.expect("a marquee selects").targets.is_empty(), "left to right selects only what is fully inside");
}

#[semio_framework_async_macros::async_test]
async fn dragging_the_end_handle_of_the_selected_wall_sets_its_axis() {
    let mut rig = Rig::plan("select", room());
    rig.selected = vec!["w-south".into()];
    assert!(rig.down(0.02, 0.01).pick.is_none(), "a press on a handle picks nothing");
    rig.mv(-1.0, 0.5);
    assert!(rig.shows(Shape::Path), "the dragged wall is ghosted");
    let step = rig.up(-1.0, 0.5);
    let ModelMutation::SetWallAxis(set) = only_mutation(&step) else { panic!("a set-wall-axis") };
    assert_eq!((set.id.as_str(), set.axis.clone()), ("w-south", Axis::Line { start: Point2 { x: -1.0, y: 0.5 }, end: Point2 { x: 8.0, y: 0.0 } }));
    assert_eq!(rig.snapshot.walls["w-south"].axis, set.axis);
}

#[semio_framework_async_macros::async_test]
async fn dragging_the_midpoint_handle_bends_the_wall_through_the_pointer() {
    let mut rig = Rig::plan("select", room());
    rig.selected = vec!["w-south".into()];
    rig.down(4.0, 0.01);
    rig.mv(4.0, -1.0);
    let step = rig.up(4.0, -1.0);
    let ModelMutation::SetWallAxis(set) = only_mutation(&step) else { panic!("a set-wall-axis") };
    let Axis::Arc { start, end, bulge } = &set.axis else { panic!("an arc, got {:?}", set.axis) };
    assert_eq!(((start.x, start.y), (end.x, end.y)), ((0.0, 0.0), (8.0, 0.0)));
    assert!((bulge - 0.25).abs() < 1e-9, "a sagitta of 1 over a chord of 8 is bulge 0.25, got {bulge}");
    rig.selected = vec!["w-south".into()];
    rig.down(4.0, -1.0);
    let straight = rig.up(4.0, 0.0);
    let ModelMutation::SetWallAxis(set) = only_mutation(&straight) else { panic!("a set-wall-axis") };
    assert!(matches!(set.axis, Axis::Line { .. }), "dragging the curve handle back onto the chord straightens the wall");
}

#[semio_framework_async_macros::async_test]
async fn the_handles_show_only_for_exactly_one_selected_wall() {
    let snapshot = room();
    assert_eq!(plan_marks(&snapshot, &["w-south".into()]).len(), 3);
    assert!(plan_marks(&snapshot, &["w-south".into(), "w-east".into()]).is_empty() && plan_marks(&snapshot, &[]).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn an_opening_slides_along_its_wall_and_onto_another_one() {
    let mut rig = Rig::plan("select", room_with_window());
    let press = rig.down(2.0, 0.0);
    assert_eq!(press.pick.map(|pick| pick.targets), Some(vec![target("opening", "o-1")]));
    rig.mv(5.0, 0.0);
    assert!(rig.shows(Shape::Path), "the cut ghost slides");
    let step = rig.up(5.0, 0.0);
    let ModelMutation::MoveOpening(slide) = only_mutation(&step) else { panic!("a move-opening") };
    assert_eq!((slide.id.as_str(), slide.offset, slide.host.clone()), ("o-1", 5.0, None));
    rig.down(5.0, 0.0);
    let step = rig.up(3.0, 6.0);
    let ModelMutation::MoveOpening(rehost) = only_mutation(&step) else { panic!("a move-opening") };
    assert_eq!((rehost.offset, rehost.host.clone()), (5.0, Some("w-north".to_string())), "onto the north wall, 5 m from its start");
}

#[semio_framework_async_macros::async_test]
async fn an_opening_that_would_overlap_or_not_move_is_left_alone() {
    let mut snapshot = room_with_window();
    snapshot.openings.insert("o-2".into(), Opening { host: "w-south".into(), kind: OpeningKind::Window { window_type: "win-12".into() }, offset: 5.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: "Window 2".into() });
    let mut rig = Rig::plan("select", snapshot);
    rig.down(2.0, 0.0);
    assert!(rig.up(5.2, 0.0).mutations.is_empty(), "onto its neighbour: refused before it is written");
    rig.down(2.0, 0.0);
    assert!(rig.up(2.0, 0.0).mutations.is_empty(), "released where it was: nothing to write");
}

#[semio_framework_async_macros::async_test]
async fn the_storey_height_handle_of_the_section_sets_the_storey_height() {
    let surface = Surface::Section { start: [0.0, 0.0], end: [8.0, 0.0] };
    let mut rig = Rig::on("select", room(), surface.clone());
    assert!(rig.down(4.0, 1.0).mutations.is_empty(), "away from a storey top nothing is grabbed");
    rig.up(4.0, 1.0);
    rig.down(4.0, 3.0);
    rig.mv(4.0, 3.6);
    assert!(rig.shows(Shape::Path) && rig.shows(Shape::Label), "the new top line and the height are shown");
    let step = rig.up(4.0, 3.62);
    let ModelMutation::SetStoreyHeight(set) = only_mutation(&step) else { panic!("a set-storey-height") };
    assert_eq!(set.id, "st-ground");
    assert!((set.height - 3.6).abs() < 1e-9, "snapped to 5 cm: {}", set.height);
    assert!((rig.snapshot.storeys["st-ground"].height - 3.6).abs() < 1e-9);
    let marks = section_marks(&crate::editor::bim::inference::with_inference(None, &rig.snapshot, |inference| inference.clone()), [0.0, 0.0], [8.0, 0.0]);
    assert_eq!(marks.len(), 4, "a guide and a handle for each of the two storeys");
}

#[semio_framework_async_macros::async_test]
async fn a_lowered_storey_keeps_at_least_half_a_metre_and_an_unchanged_height_writes_nothing() {
    let mut rig = Rig::on("select", room(), Surface::Section { start: [0.0, 0.0], end: [8.0, 0.0] });
    rig.down(4.0, 3.0);
    let step = rig.up(4.0, 0.1);
    let ModelMutation::SetStoreyHeight(set) = only_mutation(&step) else { panic!("a set-storey-height") };
    assert_eq!(set.height, 0.5);
    let mut rig = Rig::on("select", room(), Surface::Section { start: [0.0, 0.0], end: [8.0, 0.0] });
    rig.down(4.0, 3.0);
    assert!(rig.up(4.0, 3.01).mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn escape_drops_a_drag_without_a_trace() {
    let mut rig = Rig::plan("select", room());
    rig.selected = vec!["w-south".into()];
    rig.down(0.02, 0.01);
    rig.mv(-1.0, 0.5);
    rig.escape();
    assert!(rig.up(-1.0, 0.5).mutations.is_empty());
    assert_eq!(rig.snapshot, room());
}
