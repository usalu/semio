use super::*;
use crate::editor::bim::gestures::session::{Modifiers, Shape, ToolEvent};
use crate::editor::bim::gestures::tests::fixture::{room, Rig};
use crate::{Axis, ModelMutation, Point2};

fn only(mutations: &[ModelMutation]) -> &ModelMutation {
    match mutations {
        [mutation] => mutation,
        other => panic!("one mutation expected, got {other:?}"),
    }
}

fn line(start: (f64, f64), end: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } }
}

#[semio_framework_async_macros::async_test]
async fn the_reflection_of_a_point_is_the_point_on_the_other_side_of_the_line() {
    assert_eq!(reflect([3.0, 2.0], [5.0, 0.0], [5.0, 1.0]), [7.0, 2.0]);
    assert_eq!(reflect([1.0, 4.0], [0.0, 0.0], [1.0, 1.0]), [4.0, 1.0]);
    assert_eq!(reflect([2.0, 2.0], [1.0, 1.0], [1.0, 1.0]), [2.0, 2.0], "a line without length reflects nothing");
}

#[semio_framework_async_macros::async_test]
async fn an_array_counts_the_whole_spacings_up_to_the_pointer_and_a_radial_one_the_turns_that_fill_the_circle() {
    assert_eq!(array_count([0.0, 0.0], [2.0, 0.0], [6.1, 0.4]), 3);
    assert_eq!(array_count([0.0, 0.0], [2.0, 0.0], [-4.0, 0.0]), 1, "behind the base is one copy");
    assert_eq!(array_count([0.0, 0.0], [0.0, 0.0], [5.0, 0.0]), 1);
    assert_eq!(array_count([0.0, 0.0], [0.001, 0.0], [1000.0, 0.0]), MAX_COPIES);
    let step = radial_step([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], false);
    assert!((step - PI / 2.0).abs() < 1e-12);
    assert_eq!(radial_count(step), 3);
    assert_eq!(radial_count(-PI / 3.0), 5);
    assert_eq!(radial_count(PI), 1);
    assert_eq!(radial_count(0.0), 0);
    assert!((radial_step([0.0, 0.0], [1.0, 0.0], [1.0, 0.3], true) - PI / 12.0).abs() < 1e-12, "shift quantises to fifteen degrees");
}

#[semio_framework_async_macros::async_test]
async fn clicking_the_base_and_the_target_writes_one_copy_and_selects_it() {
    let mut rig = Rig::plan("copy", room());
    rig.selected = vec!["w-south".into()];
    assert!(rig.click(1.0, 1.0).mutations.is_empty(), "the base click only arms the copy");
    rig.mv(1.0, 4.0);
    assert!(rig.shows(Shape::Path) && rig.shows(Shape::Label), "the ghost of the copy and its distance follow the pointer");
    let step = rig.click(1.0, 4.0);
    let ModelMutation::CopyElements(copy) = only(&step.mutations) else { panic!("a copy-elements") };
    assert_eq!((copy.ids.clone(), copy.vector.x, copy.vector.y), (vec!["w-south".to_string()], 0.0, 3.0));
    let minted = crate::mutations::modify::mint(&copy.prefix, 1, 0);
    assert_eq!(rig.snapshot.walls[&minted].axis, line((0.0, 3.0), (8.0, 3.0)));
    assert!(rig.snapshot.walls.contains_key("w-south"), "the original stays");
    assert_eq!(step.pick.map(|pick| pick.targets), Some(vec![("wall".to_string(), minted)]), "the copy ends up selected");
}

#[semio_framework_async_macros::async_test]
async fn dragging_copies_too_and_escape_forgets_the_base() {
    let mut rig = Rig::plan("copy", room());
    rig.selected = vec!["w-south".into()];
    rig.down(1.0, 1.0);
    rig.mv(3.0, 1.0);
    let step = rig.up(3.0, 1.0);
    let ModelMutation::CopyElements(copy) = only(&step.mutations) else { panic!("a copy-elements") };
    assert_eq!((copy.vector.x, copy.vector.y), (2.0, 0.0));
    let mut rig = Rig::plan("copy", room());
    rig.selected = vec!["w-south".into()];
    rig.click(1.0, 1.0);
    rig.escape();
    assert!(rig.click(2.0, 2.0).mutations.is_empty(), "escape forgot the base");
}

#[semio_framework_async_macros::async_test]
async fn with_nothing_selected_the_press_selects_the_element_under_it_and_copies_it() {
    let mut rig = Rig::plan("copy", room());
    let press = rig.down(4.0, 0.0);
    assert_eq!(press.pick.map(|pick| pick.targets), Some(vec![("wall".to_string(), "w-south".to_string())]));
    rig.mv(4.0, 2.0);
    let step = rig.up(4.0, 2.0);
    let ModelMutation::CopyElements(copy) = only(&step.mutations) else { panic!("a copy-elements") };
    assert_eq!(copy.ids, vec!["w-south".to_string()]);
    let mut rig = Rig::plan("copy", room());
    assert!(rig.down(4.0, 3.0).pick.is_none() && rig.up(5.0, 4.0).mutations.is_empty(), "nothing under the press: nothing is copied");
}

#[semio_framework_async_macros::async_test]
async fn the_mirror_line_of_two_clicks_mirrors_in_place_and_control_makes_copies() {
    let mut rig = Rig::plan("mirror", room());
    rig.selected = vec!["w-south".into()];
    assert!(rig.click(10.0, -1.0).mutations.is_empty());
    rig.mv(10.0, 7.0);
    assert!(rig.shows(Shape::Path), "the line and the mirrored ghost show");
    let step = rig.click(10.0, 7.0);
    let ModelMutation::MirrorElements(mirror) = only(&step.mutations) else { panic!("a mirror-elements") };
    assert_eq!((mirror.line_start.x, mirror.line_end.y, mirror.prefix.clone()), (10.0, 7.0, None));
    assert_eq!(rig.snapshot.walls["w-south"].axis, line((12.0, 0.0), (20.0, 0.0)), "the wall stands mirrored and runs the other way");
    let mut rig = Rig::plan("mirror", room());
    rig.selected = vec!["w-south".into()];
    rig.click(10.0, -1.0);
    let step = rig.send(ToolEvent::Down(rig.pointer(10.0, 7.0, Modifiers { shift: false, ctrl: true, meta: false })));
    let ModelMutation::MirrorElements(mirror) = only(&step.mutations) else { panic!("a mirror-elements") };
    let minted = crate::mutations::modify::mint(mirror.prefix.as_deref().expect("control makes copies"), 1, 0);
    assert_eq!(rig.snapshot.walls[&minted].axis, line((12.0, 0.0), (20.0, 0.0)));
    assert_eq!(rig.snapshot.walls["w-south"].axis, line((0.0, 0.0), (8.0, 0.0)), "the original stays");
}

#[semio_framework_async_macros::async_test]
async fn the_array_takes_a_base_a_spacing_and_the_count_the_pointer_reaches() {
    let mut rig = Rig::plan("array", room());
    rig.selected = vec!["w-south".into()];
    rig.click(1.0, 2.0);
    assert!(rig.click(1.0, 3.0).mutations.is_empty(), "the spacing click only arms the count");
    rig.mv(1.0, 5.1);
    assert!(rig.preview.marks.iter().filter(|mark| mark.shape == Shape::Path).count() >= 3, "a ghost per copy shows");
    let step = rig.down(1.0, 5.1);
    let ModelMutation::ArrayElements(array) = only(&step.mutations) else { panic!("an array-elements") };
    assert_eq!(array.pattern, ArrayPattern::Linear { count: 3, spacing: Point2 { x: 0.0, y: 1.0 } });
    for copy in 1..=3 {
        let minted = crate::mutations::modify::mint(&array.prefix, copy, 0);
        let shift = f64::from(copy);
        assert_eq!(rig.snapshot.walls[&minted].axis, line((0.0, shift), (8.0, shift)));
    }
    assert_eq!(step.pick.map(|pick| pick.targets.len()), Some(3), "every copy ends up selected");
}

#[semio_framework_async_macros::async_test]
async fn the_radial_array_fills_the_circle_with_the_step_the_pointer_turns() {
    let mut rig = Rig::plan("array-radial", room());
    rig.selected = vec!["w-south".into()];
    rig.click(4.0, 3.0);
    assert!(rig.click(5.0, 3.0).mutations.is_empty());
    rig.mv(4.0, 4.0);
    assert!(rig.shows(Shape::Label), "the count and the step show");
    let step = rig.down(4.0, 4.0);
    let ModelMutation::ArrayElements(array) = only(&step.mutations) else { panic!("an array-elements") };
    let ArrayPattern::Radial { count, center, step: turn } = array.pattern else { panic!("a radial pattern") };
    assert_eq!((count, center), (3, Point2 { x: 4.0, y: 3.0 }));
    assert!((turn - PI / 2.0).abs() < 1e-12);
    assert_eq!(rig.snapshot.walls.len(), 4 + 3, "three turned copies of the wall");
    let mut rig = Rig::plan("array-radial", room());
    rig.selected = vec!["w-south".into()];
    rig.click(4.0, 3.0);
    rig.click(5.0, 3.0);
    assert!(rig.down(5.0, 3.0).mutations.is_empty(), "a pointer on the reference is no turn");
}

#[semio_framework_async_macros::async_test]
async fn a_refused_result_writes_nothing_and_the_gesture_starts_over() {
    let mut rig = Rig::plan("copy", room());
    rig.selected = vec!["st-ground".into()];
    assert!(rig.click(1.0, 1.0).mutations.is_empty(), "a storey has no placement: nothing is selected to copy");
    let before = rig.snapshot.clone();
    rig.click(1.0, 4.0);
    assert_eq!(rig.snapshot, before);
}
