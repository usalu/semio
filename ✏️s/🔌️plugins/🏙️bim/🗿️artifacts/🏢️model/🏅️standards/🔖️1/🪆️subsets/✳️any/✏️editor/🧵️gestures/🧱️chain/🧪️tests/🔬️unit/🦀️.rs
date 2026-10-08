use crate::editor::bim::gestures::session::{Shape, TYPE_MISSING};
use crate::editor::bim::gestures::tests::fixture::{model, Rig};
use crate::{Axis, ModelMutation, Point2, TopConstraint};

fn line(start: (f64, f64), end: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } }
}

fn created_wall(mutations: &[ModelMutation]) -> &crate::Wall {
    match mutations {
        [ModelMutation::CreateWall(create)] => &create.wall,
        other => panic!("one create-wall expected, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn a_wall_chain_writes_one_wall_per_click_after_the_first_and_closes_on_its_first_point() {
    let mut rig = Rig::plan("wall", model());
    assert!(rig.down(0.0, 0.0).mutations.is_empty(), "the first click only sets the start");
    let second = rig.down(4.0, 0.0);
    let wall = created_wall(&second.mutations);
    assert_eq!((wall.axis.clone(), wall.wall_type.as_str(), wall.storey.as_str()), (line((0.0, 0.0), (4.0, 0.0)), "wt-300", "st-ground"));
    assert_eq!(wall.top, TopConstraint::StoreyTop { offset: 0.0 });
    assert_eq!(wall.name, "Wall 1");
    assert_eq!(created_wall(&rig.down(4.0, 3.0).mutations).axis, line((4.0, 0.0), (4.0, 3.0)));
    assert_eq!(created_wall(&rig.down(0.0, 3.0).mutations).axis, line((4.0, 3.0), (0.0, 3.0)));
    let closing = rig.down(0.004, 0.003);
    assert_eq!(created_wall(&closing.mutations).axis, line((0.0, 3.0), (0.0, 0.0)), "the click near the first point snaps onto it and closes the loop");
    assert_eq!(rig.snapshot.walls.len(), 4);
    assert!(rig.down(1.0, 1.0).mutations.is_empty(), "a closed chain is over: the next click starts a new one");
    assert_eq!(created_wall(&rig.down(2.0, 1.0).mutations).axis, line((1.0, 1.0), (2.0, 1.0)));
}

#[semio_framework_async_macros::async_test]
async fn moving_shows_the_rubber_band_and_its_length_but_never_writes() {
    let mut rig = Rig::plan("wall", model());
    rig.down(0.0, 0.0);
    let before = rig.snapshot.clone();
    assert!(rig.mv(3.0, 4.0).mutations.is_empty());
    assert_eq!(rig.snapshot, before);
    assert!(rig.shows(Shape::Path) && rig.shows(Shape::Snap));
    let label = rig.preview.marks.iter().find(|mark| mark.shape == Shape::Label).expect("a length label");
    assert_eq!(label.text, "5.00 m");
}

#[semio_framework_async_macros::async_test]
async fn the_end_of_a_segment_snaps_to_the_orthogonal_direction_and_to_existing_endpoints() {
    let mut rig = Rig::plan("wall", model());
    rig.down(0.0, 0.0);
    rig.down(3.0, 0.04);
    let wall = rig.snapshot.walls.values().next().expect("a wall");
    let Axis::Line { end, .. } = &wall.axis else { panic!("a line") };
    assert_eq!(end.y, 0.0, "a direction within reach of the horizontal snaps onto it");
    assert!((end.x - 3.0).abs() < 1e-3);
    rig.escape();
    let mut rig = Rig::plan("wall", rig.snapshot.clone());
    rig.down(10.0, 10.0);
    rig.down(0.03, 0.02);
    let Axis::Line { end, .. } = &rig.snapshot.walls.values().last().expect("a wall").axis else { panic!("a line") };
    assert_eq!((end.x, end.y), (0.0, 0.0), "the end snaps onto the endpoint of the wall before it");
}

#[semio_framework_async_macros::async_test]
async fn an_arc_wall_takes_its_bulge_from_the_third_click() {
    let mut rig = Rig::plan("wall-arc", model());
    rig.down(0.0, 0.0);
    assert!(rig.down(4.0, 0.0).mutations.is_empty(), "the second click only fixes the chord");
    rig.mv(2.0, -1.0);
    assert!(rig.shows(Shape::Path));
    let third = rig.down(2.0, -1.0);
    let wall = created_wall(&third.mutations);
    let Axis::Arc { start, end, bulge } = &wall.axis else { panic!("an arc, got {:?}", wall.axis) };
    assert_eq!(((start.x, start.y), (end.x, end.y)), ((0.0, 0.0), (4.0, 0.0)));
    assert!((bulge - 0.5).abs() < 1e-9, "a chord of 4 with a sagitta of 1 is bulge 2h/c = 0.5, got {bulge}");
}

#[semio_framework_async_macros::async_test]
async fn escape_and_finish_end_a_chain_and_what_was_written_stays() {
    let mut rig = Rig::plan("wall", model());
    rig.down(0.0, 0.0);
    rig.down(2.0, 0.0);
    rig.escape();
    assert_eq!(rig.snapshot.walls.len(), 1);
    assert!(rig.down(5.0, 5.0).mutations.is_empty(), "the chain began again");
    rig.finish();
    assert!(rig.down(6.0, 5.0).mutations.is_empty(), "finishing ended the chain again");
    assert_eq!(rig.snapshot.walls.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn the_wall_type_is_the_selected_library_entry_else_the_first_and_a_missing_type_is_refused() {
    let mut snapshot = model();
    let mut thin = snapshot.wall_types["wt-300"].clone();
    thin.name = "Thin".into();
    snapshot.wall_types.insert("wt-100".into(), thin);
    let mut rig = Rig::plan("wall", snapshot.clone());
    rig.library = vec!["wt-100".into()];
    rig.down(0.0, 0.0);
    assert_eq!(created_wall(&rig.down(1.0, 0.0).mutations).wall_type, "wt-100");
    snapshot.wall_types.clear();
    let mut rig = Rig::plan("wall", snapshot);
    rig.down(0.0, 0.0);
    assert_eq!(rig.down(1.0, 0.0).refused, Some(TYPE_MISSING));
    assert!(rig.snapshot.walls.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_beam_is_written_by_two_clicks_and_a_curtain_wall_by_each_segment() {
    let mut rig = Rig::plan("beam", model());
    rig.down(0.0, 0.0);
    let step = rig.down(5.0, 0.0);
    let [ModelMutation::CreateBeam(create)] = step.mutations.as_slice() else { panic!("one create-beam") };
    assert_eq!((create.beam.beam_type.as_str(), create.beam.top_offset, create.beam.end), ("bm-20", 0.0, Point2 { x: 5.0, y: 0.0 }));
    assert!(rig.down(1.0, 1.0).mutations.is_empty(), "one beam per two clicks: the third click starts the next beam");
    let mut rig = Rig::plan("curtain-wall", model());
    rig.down(0.0, 0.0);
    let step = rig.down(6.0, 0.0);
    let [ModelMutation::CreateCurtainWall(create)] = step.mutations.as_slice() else { panic!("one create-curtain-wall") };
    assert_eq!((create.curtain_wall.panel_material.as_str(), create.curtain_wall.mullion_material.as_str()), ("m-glass", "m-steel"));
}

#[semio_framework_async_macros::async_test]
async fn grid_lines_take_the_next_free_label_letters_for_horizontal_numbers_for_vertical() {
    let mut rig = Rig::plan("grid", model());
    rig.down(0.0, 0.0);
    let step = rig.down(10.0, 0.0);
    let [ModelMutation::CreateGridLine(first)] = step.mutations.as_slice() else { panic!("one create-grid-line") };
    assert_eq!((first.grid_line.label.as_str(), first.grid_line.building.as_str()), ("A", "bldg-1"));
    rig.down(0.0, 3.0);
    let step = rig.down(10.0, 3.0);
    let [ModelMutation::CreateGridLine(second)] = step.mutations.as_slice() else { panic!("one create-grid-line") };
    assert_eq!(second.grid_line.label, "B");
    rig.down(1.0, -2.0);
    let step = rig.down(1.0, 8.0);
    let [ModelMutation::CreateGridLine(third)] = step.mutations.as_slice() else { panic!("one create-grid-line") };
    assert_eq!(third.grid_line.label, "1");
}

#[semio_framework_async_macros::async_test]
async fn a_railing_is_written_whole_when_the_polyline_finishes() {
    let mut rig = Rig::plan("railing", model());
    for (x, y) in [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0)] {
        assert!(rig.click(x, y).mutations.is_empty());
    }
    let step = rig.double(2.0, 2.0);
    let [ModelMutation::CreateRailing(create)] = step.mutations.as_slice() else { panic!("one create-railing") };
    assert_eq!(create.railing.path.len(), 3);
    assert_eq!((create.railing.height, create.railing.material.as_str()), (1.0, "m-steel"));
    let mut rig = Rig::plan("railing", model());
    rig.click(0.0, 0.0);
    rig.click(1.0, 0.0);
    rig.escape();
    assert!(rig.finish().mutations.is_empty() && rig.snapshot.railings.is_empty(), "escape drops the polyline whole");
}

#[semio_framework_async_macros::async_test]
async fn the_measure_shows_a_distance_and_never_writes_anything() {
    let mut rig = Rig::plan("measure", model());
    let before = rig.snapshot.clone();
    for (x, y) in [(0.0, 0.0), (3.0, 4.0), (9.0, 9.0)] {
        let step = rig.down(x, y);
        assert!(step.mutations.is_empty() && step.pick.is_none() && step.refused.is_none());
    }
    rig.down(9.0, 12.0);
    let label = rig.preview.marks.iter().find(|mark| mark.shape == Shape::Label).expect("the measured distance");
    assert_eq!(label.text, "3.00 m");
    assert_eq!(rig.snapshot, before);
}
