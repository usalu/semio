use crate::editor::bim::gestures::tests::fixture::{model, room, Rig};
use crate::{ModelMutation, Point2, SpaceBoundary, StairFlight, TopConstraint};

#[semio_framework_async_macros::async_test]
async fn one_click_stands_a_column_of_the_first_type_on_the_storey() {
    let mut rig = Rig::plan("column", model());
    let step = rig.down(2.0, 3.0);
    let [ModelMutation::CreateColumn(create)] = step.mutations.as_slice() else { panic!("one create-column") };
    assert_eq!((create.column.position, create.column.column_type.as_str(), create.column.rotation), (Point2 { x: 2.0, y: 3.0 }, "col-30", 0.0));
    assert_eq!(create.column.top, TopConstraint::StoreyTop { offset: 0.0 });
    assert!(rig.snapshot.columns.len() == 1 && create.column.name == "Column 1");
}

#[semio_framework_async_macros::async_test]
async fn a_space_is_seeded_by_the_click_and_numbered_with_the_next_free_number() {
    let mut rig = Rig::plan("space", room());
    let step = rig.down(4.0, 3.0);
    let [ModelMutation::CreateSpace(first)] = step.mutations.as_slice() else { panic!("one create-space") };
    assert_eq!(first.space.boundary, SpaceBoundary::Bounded { seed: Point2 { x: 4.0, y: 3.0 } });
    assert_eq!(first.space.number, "1");
    let step = rig.down(5.0, 2.0);
    let [ModelMutation::CreateSpace(second)] = step.mutations.as_slice() else { panic!("one create-space") };
    assert_eq!(second.space.number, "2");
    rig.snapshot.spaces.values_mut().for_each(|space| space.number = "104".into());
    let step = rig.down(2.0, 2.0);
    let [ModelMutation::CreateSpace(third)] = step.mutations.as_slice() else { panic!("a create-space") };
    assert_eq!(third.space.number, "105", "the next number goes past the highest");
}

#[semio_framework_async_macros::async_test]
async fn a_stair_is_pressed_at_its_foot_and_dragged_in_its_direction() {
    let mut rig = Rig::plan("stair", model());
    assert!(rig.down(1.0, 1.0).mutations.is_empty());
    rig.mv(1.0, 4.0);
    let step = rig.up(1.0, 4.0);
    let [ModelMutation::CreateStair(create)] = step.mutations.as_slice() else { panic!("one create-stair") };
    assert_eq!(create.stair.start, Point2 { x: 1.0, y: 1.0 });
    assert!((create.stair.direction - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    assert_eq!((create.stair.flight.clone(), create.stair.top.clone(), create.stair.width), (StairFlight::Straight, TopConstraint::StoreyTop { offset: 0.0 }, 1.0));
}

#[semio_framework_async_macros::async_test]
async fn a_stair_can_also_be_clicked_foot_then_direction() {
    let mut rig = Rig::plan("stair", model());
    rig.click(0.0, 0.0);
    assert!(rig.snapshot.stairs.is_empty(), "a click without a drag only fixes the foot");
    let step = rig.click(3.0, 0.0);
    let [ModelMutation::CreateStair(create)] = step.mutations.as_slice() else { panic!("one create-stair") };
    assert!(create.stair.direction.abs() < 1e-9);
    rig.click(5.0, 5.0);
    rig.escape();
    assert!(rig.click(6.0, 5.0).mutations.is_empty(), "escape forgot the foot");
}
