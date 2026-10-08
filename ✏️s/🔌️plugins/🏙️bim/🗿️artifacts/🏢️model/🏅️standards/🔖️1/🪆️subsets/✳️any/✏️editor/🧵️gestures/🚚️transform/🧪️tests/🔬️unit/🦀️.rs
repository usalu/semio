use crate::editor::bim::gestures::session::{Modifiers, Shape, ToolEvent};
use crate::editor::bim::gestures::tests::fixture::{room, Rig};
use crate::ModelMutation;

fn only(mutations: &[ModelMutation]) -> &ModelMutation {
    match mutations {
        [mutation] => mutation,
        other => panic!("one mutation expected, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn dragging_the_selection_writes_one_move_with_the_vector() {
    let mut rig = Rig::plan("move", room());
    rig.selected = vec!["w-south".into(), "w-north".into()];
    assert!(rig.down(1.0, 1.0).mutations.is_empty());
    rig.mv(3.0, 1.0);
    assert!(rig.shows(Shape::Path) && rig.shows(Shape::Label), "the ghost and the distance follow the pointer");
    let step = rig.up(3.0, 1.0);
    let ModelMutation::MoveElements(moved) = only(&step.mutations) else { panic!("a move-elements") };
    assert_eq!((moved.ids.clone(), moved.vector.x, moved.vector.y), (vec!["w-south".to_string(), "w-north".to_string()], 2.0, 0.0));
    assert_eq!(rig.snapshot.walls["w-south"].axis, crate::Axis::Line { start: crate::Point2 { x: 2.0, y: 0.0 }, end: crate::Point2 { x: 10.0, y: 0.0 } });
}

#[semio_framework_async_macros::async_test]
async fn clicking_the_base_and_the_target_moves_too_and_shift_locks_the_direction() {
    let mut rig = Rig::plan("move", room());
    rig.selected = vec!["w-south".into()];
    rig.click(1.0, 1.0);
    assert!(rig.snapshot == room(), "the base click only arms the move");
    let step = rig.send(ToolEvent::Down(rig.pointer(1.0, 4.0, Modifiers { shift: true, ctrl: false, meta: false })));
    let ModelMutation::MoveElements(moved) = only(&step.mutations) else { panic!("a move-elements") };
    assert!(moved.vector.x.abs() < 1e-9 && (moved.vector.y - 3.0).abs() < 1e-9);
    let mut rig = Rig::plan("move", room());
    rig.selected = vec!["w-south".into()];
    rig.click(1.0, 1.0);
    rig.escape();
    assert!(rig.click(2.0, 2.0).mutations.is_empty(), "escape forgot the base");
}

#[semio_framework_async_macros::async_test]
async fn with_nothing_selected_the_element_under_the_press_is_selected_and_moved() {
    let mut rig = Rig::plan("move", room());
    let press = rig.down(4.0, 0.0);
    assert_eq!(press.pick.map(|pick| pick.targets), Some(vec![("wall".to_string(), "w-south".to_string())]));
    rig.mv(4.0, 2.0);
    let step = rig.up(4.0, 2.0);
    let ModelMutation::MoveElements(moved) = only(&step.mutations) else { panic!("a move-elements") };
    assert_eq!(moved.ids, vec!["w-south".to_string()]);
    let mut rig = Rig::plan("move", room());
    assert!(rig.down(4.0, 3.0).pick.is_none() && rig.up(5.0, 4.0).mutations.is_empty(), "nothing under the press: nothing moves");
}

#[semio_framework_async_macros::async_test]
async fn rotating_takes_the_pivot_the_reference_and_the_target_direction() {
    let mut rig = Rig::plan("rotate", room());
    rig.selected = vec!["w-south".into()];
    rig.click(0.0, 0.0);
    rig.click(1.0, 0.0);
    rig.mv(0.0, 1.0);
    assert!(rig.shows(Shape::Label), "the angle is shown");
    let step = rig.click(0.0, 1.0);
    let ModelMutation::RotateElements(turned) = only(&step.mutations) else { panic!("a rotate-elements") };
    assert_eq!((turned.pivot.x, turned.pivot.y), (0.0, 0.0));
    assert!((turned.angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn shift_quantises_the_turn_to_fifteen_degrees_and_a_zero_turn_writes_nothing() {
    let mut rig = Rig::plan("rotate", room());
    rig.selected = vec!["w-south".into()];
    rig.click(0.0, 0.0);
    rig.click(1.0, 0.0);
    let angle = 20.0_f64.to_radians();
    let step = rig.send(ToolEvent::Down(rig.pointer(angle.cos(), angle.sin(), Modifiers { shift: true, ctrl: false, meta: false })));
    let ModelMutation::RotateElements(turned) = only(&step.mutations) else { panic!("a rotate-elements") };
    assert!((turned.angle - 15.0_f64.to_radians()).abs() < 1e-9);
    let mut rig = Rig::plan("rotate", room());
    rig.selected = vec!["w-south".into()];
    rig.click(0.0, 0.0);
    rig.click(1.0, 0.0);
    assert!(rig.click(2.0, 0.0).mutations.is_empty(), "the same direction is no turn");
}
