use super::*;
use crate::mutations::modify::cut::{cut_loop, offset_axis, trim_extend, TrimFlaw};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout;
use crate::{Axis, Layer, LayerFunction, ModelSnapshot, Vertex, WallType};
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;
use serde_json::Value;
use std::f64::consts::TAU;

const CASES: &str = include_str!("../../../../../🧫️fixtures/🧙️modify/🔣️.json");
const JOINS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🔗️wall-joins/📸️snapshot/🔣️.json");

fn cases() -> Value {
    serde_json::from_str(CASES).expect("the committed cases are JSON")
}

fn number(value: &Value) -> f64 {
    value.as_f64().expect("a number")
}

fn near(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance * a.abs().max(b.abs()).max(1.0)
}

fn point(value: &Value) -> Point2 {
    Point2 { x: number(&value[0]), y: number(&value[1]) }
}

fn assert_point(found: Point2, expected: &Value, tolerance: f64, what: &str) {
    assert!(near(found.x, number(&expected[0]), tolerance) && near(found.y, number(&expected[1]), tolerance), "{what}: expected {expected}, found ({}, {})", found.x, found.y);
}

fn loop_of(value: &Value) -> Vec<Vertex> {
    value.as_array().expect("a loop").iter().map(|vertex| Vertex { point: point(vertex), bulge: number(&vertex[2]) }).collect()
}

fn inner(vertices: &[Vertex]) -> Vec<loops::Vertex> {
    vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect()
}

fn axis_of(value: &Value) -> Axis {
    let (start, end, bulge) = (point(&value["start"]), point(&value["end"]), number(&value["bulge"]));
    if bulge == 0.0 { Axis::Line { start, end } } else { Axis::Arc { start, end, bulge } }
}

fn map_of(kind: &str, case: &Value) -> Map {
    match kind {
        "translate" => Map::Translate(point(&case["vector"])),
        "rotate" => Map::Rotate { pivot: point(&case["pivot"]), angle: number(&case["angle"]) },
        _ => Map::Mirror { start: point(&case["line"][0]), end: point(&case["line"][1]) },
    }
}

fn same_direction(a: f64, b: f64) -> bool {
    let turn = (a - b).rem_euclid(TAU);
    turn < 1e-9 || TAU - turn < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn the_maps_send_points_and_directions_where_shapely_sends_them() {
    for (kind, rows) in cases()["maps"].as_object().expect("maps") {
        for case in rows.as_array().expect("cases") {
            let map = map_of(kind, case);
            for (at, expected) in case["points"].as_array().expect("points").iter().zip(case["images"].as_array().expect("images")) {
                assert_point(map.point(point(at)), expected, 1e-9, kind);
            }
            for (theta, expected) in case["directions"].as_array().expect("directions").iter().zip(case["turned"].as_array().expect("turned")) {
                assert!(same_direction(map.turn(number(theta)), number(expected)), "{kind}: direction {theta} turns to {expected}, not {}", map.turn(number(theta)));
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_mirrored_loop_keeps_its_area_runs_counter_clockwise_and_its_centroid_is_mirrored() {
    for case in cases()["loops"].as_array().expect("loops") {
        let (loop_, map) = (loop_of(&case["loop"]), map_of("mirror", case));
        let image = map::loop_image(&loop_, &map);
        assert!(near(loops::signed_area(&inner(&loop_)), number(&case["area"]), 1e-9), "the exact area of the loop");
        assert!(near(loops::signed_area(&inner(&image)), number(&case["image_area"]), 1e-5), "the image keeps the area");
        assert_eq!(loops::is_ccw(&inner(&image)), case["image_counter_clockwise"].as_bool().expect("flag"), "the image runs counter-clockwise");
        let centroid = loops::centroid(&inner(&image));
        assert_point(Point2 { x: centroid.x, y: centroid.y }, &case["image_centroid"], 1e-5, "the centroid of the image");
    }
}

#[semio_framework_async_macros::async_test]
async fn parallel_axes_lie_where_the_offset_curves_of_shapely_lie() {
    for case in cases()["offsets"].as_array().expect("offsets") {
        let found = offset_axis(&axis_of(&case["axis"]), number(&case["distance"])).expect("the offset exists");
        let (Axis::Line { start, end } | Axis::Arc { start, end, .. }) = found;
        assert_point(start, &case["image"]["start"], 1e-9, "offset start");
        assert_point(end, &case["image"]["end"], 1e-9, "offset end");
        if let Axis::Arc { bulge, .. } = found {
            assert!(near(bulge, number(&case["image"]["bulge"]), 1e-12), "an offset arc keeps its sweep");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_trimmed_or_extended_end_lands_on_the_intersection_of_the_carriers() {
    for case in cases()["trims"].as_array().expect("trims") {
        let end = if case["end"] == "Start" { WallEnd::Start } else { WallEnd::End };
        let trimmed = trim_extend(&axis_of(&case["axis"]), end, &axis_of(&case["target"])).expect("the end moves");
        let (Axis::Line { start, end: finish } | Axis::Arc { start, end: finish, .. }) = trimmed.axis;
        assert_point(if end == WallEnd::Start { start } else { finish }, &case["point"], 1e-9, "the moved end");
    }
    let wall = Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 8.0, y: 0.0 } };
    let parallel = Axis::Line { start: Point2 { x: 0.0, y: 5.0 }, end: Point2 { x: 8.0, y: 5.0 } };
    assert_eq!(trim_extend(&wall, WallEnd::End, &parallel).unwrap_err(), TrimFlaw::Parallel);
    let behind = Axis::Line { start: Point2 { x: -3.0, y: -1.0 }, end: Point2 { x: -3.0, y: 1.0 } };
    assert_eq!(trim_extend(&wall, WallEnd::End, &behind).unwrap_err(), TrimFlaw::Reversed);
    let there = Axis::Line { start: Point2 { x: 8.0, y: -1.0 }, end: Point2 { x: 8.0, y: 1.0 } };
    assert_eq!(trim_extend(&wall, WallEnd::End, &there).unwrap_err(), TrimFlaw::Unchanged);
}

#[semio_framework_async_macros::async_test]
async fn a_cut_leaves_the_two_halves_shapely_leaves() {
    for case in cases()["splits"].as_array().expect("splits") {
        let cut = cut_loop(&loop_of(&case["loop"]), point(&case["line"][0]), point(&case["line"][1])).expect("the loop is cut");
        for (side, half) in [("left", &cut.left), ("right", &cut.right)] {
            let found = inner(half);
            assert!(loops::is_ccw(&found), "{side}: the half runs counter-clockwise");
            assert!(near(loops::signed_area(&found), number(&case[format!("{side}_area")]), 1e-5), "{side}: area");
            let centroid = loops::centroid(&found);
            assert_point(Point2 { x: centroid.x, y: centroid.y }, &case[format!("{side}_centroid")], 1e-5, side);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_copies_of_a_radial_array_are_the_turns_of_the_source() {
    for case in cases()["arrays"].as_array().expect("arrays") {
        let pattern = ArrayPattern::Radial { count: case["count"].as_u64().expect("count") as u32, center: point(&case["center"]), step: number(&case["step"]) };
        for (copy, expected) in case["images"].as_array().expect("images").iter().enumerate() {
            assert_point(pattern.map_of(copy as u32 + 1).point(point(&case["point"])), expected, 1e-9, "array copy");
        }
    }
}

fn joined_model(case: &Value) -> ModelSnapshot {
    let mut snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(JOINS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the joins model decodes");
    let template = snapshot.walls.values().next().expect("the joins model has a wall").clone();
    let material = snapshot.materials.keys().next().expect("the joins model has a material").clone();
    snapshot.walls.clear();
    snapshot.openings.clear();
    for wall in case["walls"].as_array().expect("walls") {
        let thickness = number(&wall["thickness"]);
        let kind = format!("wt-{thickness}");
        snapshot.wall_types.insert(kind.clone(), WallType { name: kind.clone(), layers: vec![Layer { material: material.clone(), thickness, function: LayerFunction::Structure }] });
        let join = |key: &str| match wall[key].as_str() {
            Some("Miter") => Some(crate::EndJoin::Miter),
            Some("Butt") => Some(crate::EndJoin::Butt),
            Some("None") => Some(crate::EndJoin::None),
            _ => None,
        };
        let axis = Axis::Line { start: point(&wall["start"]), end: point(&wall["end"]) };
        snapshot.walls.insert(wall["id"].as_str().expect("id").to_string(), crate::Wall { wall_type: kind, axis, location: crate::LocationLine::Center, start_join: join("start_join"), end_join: join("end_join"), ..template.clone() });
    }
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn walls_joined_by_authored_preferences_have_the_footprints_shapely_derives() {
    for case in cases()["joins"].as_array().expect("joins") {
        let layouts = compute_wall_layout(&joined_model(case));
        for (id, expected) in case["footprints"].as_object().expect("footprints") {
            let found = &layouts[id].footprint;
            let want = expected.as_array().expect("corners");
            assert_eq!(found.len(), want.len(), "{}: {id} footprint corners", case["name"]);
            for (corner, expected) in found.iter().zip(want) {
                assert_point(corner.point, expected, 1e-9, &format!("{}: {id}", case["name"]));
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_authored_preference_changes_only_the_walls_that_meet_at_the_end() {
    let case = &cases()["joins"][0];
    let mut model = joined_model(case);
    let before = compute_wall_layout(&model);
    model.walls.insert("far".into(), crate::Wall { axis: Axis::Line { start: Point2 { x: 20.0, y: 0.0 }, end: Point2 { x: 25.0, y: 0.0 } }, start_join: None, end_join: None, ..model.walls["a"].clone() });
    let with_far = compute_wall_layout(&model);
    assert_eq!(with_far["a"], before["a"], "a wall that touches nothing changes nothing");
    assert_eq!(with_far["b"], before["b"]);
    model.walls.get_mut("a").expect("a").end_join = None;
    let auto = compute_wall_layout(&model);
    assert_ne!(auto["a"].footprint, before["a"].footprint, "clearing the preference returns the corner to the geometry");
    assert_eq!(auto["far"], with_far["far"]);
}
