use super::*;
use crate::editor::bim::gestures::session::Shape;
use crate::editor::bim::gestures::tests::fixture::{room, Rig};
use crate::{Axis, Beam, ModelMutation, Phase, Point2, Slab, Vertex, Wall};

fn only(mutations: &[ModelMutation]) -> &ModelMutation {
    match mutations {
        [mutation] => mutation,
        other => panic!("one mutation expected, got {other:?}"),
    }
}

fn line(start: (f64, f64), end: (f64, f64)) -> Axis {
    Axis::Line { start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } }
}

fn wall_on(snapshot: &crate::ModelSnapshot, id: &str, axis: Axis) -> Wall {
    Wall { axis, name: id.into(), ..snapshot.walls["w-south"].clone() }
}

fn corner(x: f64, y: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge: 0.0 }
}

#[semio_framework_async_macros::async_test]
async fn the_offset_distance_is_the_side_distance_of_the_pointer_in_steps_of_five_centimetres() {
    let axis = line((0.0, 0.0), (8.0, 0.0));
    assert_eq!(offset_distance(&axis, [4.0, 1.52]), Some(1.5));
    assert_eq!(offset_distance(&axis, [4.0, -0.98]), Some(-1.0));
    assert_eq!(offset_distance(&axis, [4.0, 0.01]), None, "a hair from the axis is no offset");
}

#[semio_framework_async_macros::async_test]
async fn a_wall_pressed_and_a_side_clicked_writes_one_offset_wall_and_selects_it() {
    let mut rig = Rig::plan("offset", room());
    assert!(rig.down(4.0, 0.0).mutations.is_empty(), "the press only holds the wall");
    rig.mv(4.0, 1.52);
    assert!(rig.shows(Shape::Path) && rig.shows(Shape::Label), "the parallel wall and its distance show");
    let step = rig.down(4.0, 1.52);
    let ModelMutation::OffsetWall(offset) = only(&step.mutations) else { panic!("an offset-wall") };
    assert_eq!((offset.id.as_str(), offset.distance), ("w-south", 1.5));
    assert_eq!(rig.snapshot.walls[&offset.new_id].axis, line((0.0, 1.5), (8.0, 1.5)));
    assert_eq!(step.pick.map(|pick| pick.targets), Some(vec![("wall".to_string(), offset.new_id.clone())]));
}

#[semio_framework_async_macros::async_test]
async fn trim_keeps_the_part_under_the_pointer_and_cuts_the_other_end_back_to_the_target() {
    let mut snapshot = room();
    snapshot.walls.insert("w-long".into(), wall_on(&snapshot, "w-long", line((0.0, 3.0), (10.0, 3.0))));
    let mut rig = Rig::plan("trim", snapshot);
    let target = rig.down(8.0, 1.5);
    assert_eq!(target.pick.map(|pick| pick.targets), Some(vec![("wall".to_string(), "w-east".to_string())]), "the first press chooses the target");
    let step = rig.down(3.0, 3.0);
    let ModelMutation::TrimExtendWall(trim) = only(&step.mutations) else { panic!("a trim-extend-wall") };
    assert_eq!((trim.id.as_str(), trim.end, trim.target.as_str()), ("w-long", WallEnd::End, "w-east"));
    assert_eq!(rig.snapshot.walls["w-long"].axis, line((0.0, 3.0), (8.0, 3.0)));
}

#[semio_framework_async_macros::async_test]
async fn extend_moves_the_end_nearest_the_pointer_out_to_the_target() {
    let mut snapshot = room();
    snapshot.walls.insert("w-short".into(), wall_on(&snapshot, "w-short", line((1.0, 4.0), (5.0, 4.0))));
    let mut rig = Rig::plan("extend", snapshot);
    rig.down(8.0, 1.5);
    let step = rig.down(4.9, 4.0);
    let ModelMutation::TrimExtendWall(trim) = only(&step.mutations) else { panic!("a trim-extend-wall") };
    assert_eq!(trim.end, WallEnd::End);
    assert_eq!(rig.snapshot.walls["w-short"].axis, line((1.0, 4.0), (8.0, 4.0)));
    assert!(rig.shows(Shape::Path), "the target stays highlighted");
}

#[semio_framework_async_macros::async_test]
async fn the_end_a_trim_or_an_extend_moves_follows_the_pointer_and_parallel_walls_are_refused() {
    let wall = line((0.0, 3.0), (10.0, 3.0));
    let target = line((8.0, 0.0), (8.0, 6.0));
    assert_eq!(end_to_move(Kind::Trim, &wall, &target, [2.0, 3.0]), Some(WallEnd::End));
    assert_eq!(end_to_move(Kind::Trim, &wall, &target, [9.5, 3.0]), Some(WallEnd::Start));
    assert_eq!(end_to_move(Kind::Extend, &wall, &target, [2.0, 3.0]), Some(WallEnd::Start));
    assert_eq!(end_to_move(Kind::Trim, &wall, &line((0.0, 5.0), (10.0, 5.0)), [2.0, 3.0]), None);
    let mut snapshot = room();
    snapshot.walls.insert("w-par".into(), wall_on(&snapshot, "w-par", line((0.0, 3.0), (6.0, 3.0))));
    snapshot.walls.insert("w-par-2".into(), wall_on(&snapshot, "w-par-2", line((0.0, 4.0), (6.0, 4.0))));
    let mut rig = Rig::plan("trim", snapshot);
    rig.down(3.0, 3.0);
    assert!(rig.down(3.0, 4.0).refused.is_some(), "walls that never meet are refused and write nothing");
}

#[semio_framework_async_macros::async_test]
async fn align_sets_the_selection_onto_the_edge_of_the_reference_nearest_the_pointer() {
    let mut rig = Rig::plan("align", room());
    rig.selected = vec!["w-north".into()];
    let step = rig.down(0.5, 0.0);
    let ModelMutation::AlignElements(align) = only(&step.mutations) else { panic!("an align-elements") };
    assert_eq!((align.ids.clone(), align.axis, align.edge, align.target), (vec!["w-north".to_string()], AlignAxis::Y, AlignEdge::Min, 0.0));
    assert_eq!(rig.snapshot.walls["w-north"].axis, line((8.0, 0.0), (0.0, 0.0)));
    assert_eq!(reference_line([0.0, 0.0, 8.0, 6.0], [7.9, 3.0]), (AlignAxis::X, AlignEdge::Max, 8.0));
    assert_eq!(reference_line([0.0, 0.0, 8.0, 6.0], [4.0, 3.1]), (AlignAxis::X, AlignEdge::Center, 4.0), "the middle line wins at the middle");
}

#[semio_framework_async_macros::async_test]
async fn with_nothing_selected_align_first_selects_the_pressed_element() {
    let mut rig = Rig::plan("align", room());
    let step = rig.down(0.5, 0.0);
    assert_eq!(step.pick.map(|pick| pick.targets), Some(vec![("wall".to_string(), "w-south".to_string())]));
    assert!(step.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn split_parts_a_beam_where_it_is_pressed() {
    let mut snapshot = room();
    let beam = Beam { storey: "st-ground".into(), beam_type: "bm-20".into(), start: Point2 { x: 0.0, y: 2.0 }, end: Point2 { x: 8.0, y: 2.0 }, top_offset: 0.0, phase: Phase::New, name: "Beam".into() };
    snapshot.beams.insert("b-1".into(), beam);
    let mut rig = Rig::plan("split", snapshot);
    rig.mv(2.0, 2.0);
    assert!(rig.shows(Shape::Path), "the cut shows across the beam");
    let step = rig.down(2.0, 2.0);
    let ModelMutation::SplitBeam(split) = only(&step.mutations) else { panic!("a split-beam") };
    assert_eq!((split.id.as_str(), split.t), ("b-1", 0.25));
    assert_eq!(rig.snapshot.beams["b-1"].end, Point2 { x: 2.0, y: 2.0 });
    assert_eq!(rig.snapshot.beams[&split.new_id].start, Point2 { x: 2.0, y: 2.0 });
}

#[semio_framework_async_macros::async_test]
async fn split_cuts_a_slab_along_the_line_of_two_clicks() {
    let mut snapshot = room();
    let slab = Slab { storey: "st-ground".into(), slab_type: "sl-200".into(), boundary: vec![corner(0.0, 0.0), corner(8.0, 0.0), corner(8.0, 6.0), corner(0.0, 6.0)], holes: Vec::new(), offset: 0.0, slope: None, phase: Phase::New, name: "Floor".into() };
    snapshot.slabs.insert("sl-1".into(), slab);
    let mut rig = Rig::plan("split", snapshot);
    assert!(rig.down(3.0, 2.0).mutations.is_empty(), "the first press holds the slab and starts the cut line");
    rig.mv(3.0, 4.0);
    assert!(rig.shows(Shape::Label), "the cut line and its length show");
    let step = rig.down(3.0, 4.0);
    let ModelMutation::SplitSlab(split) = only(&step.mutations) else { panic!("a split-slab") };
    assert_eq!(split.id, "sl-1");
    assert_eq!(rig.snapshot.slabs.len(), 2);
    assert_eq!(rig.snapshot.slabs["sl-1"].boundary.len(), 4);
}
