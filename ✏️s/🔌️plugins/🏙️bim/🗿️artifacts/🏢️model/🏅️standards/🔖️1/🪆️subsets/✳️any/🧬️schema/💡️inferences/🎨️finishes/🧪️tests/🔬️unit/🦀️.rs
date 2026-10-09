use super::*;
use super::super::element_solids::ElementSolid;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::{Assigned, CeilingPatch, Entry, ModelDiff, SpacePatch};
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const ROOMS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🎨️finishes/🏡️rooms/📸️snapshot/🔣️.json");
const ROOMS_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🎨️finishes/🏡️rooms/💡️inference/🎨️finishes/🔣️.json");
const ONE_ROOM: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🎨️finishes/🏠️one-room/📸️snapshot/🔣️.json");
const ONE_ROOM_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🎨️finishes/🏠️one-room/💡️inference/🎨️finishes/🔣️.json");

fn load(text: &str) -> ModelSnapshot {
    from_json_str(text, JsonMemberPolicy::Reject).expect("the fixture decodes")
}

fn infer(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("infers")
}

fn quantity<'a>(inferred: &'a ModelInference, space: &str) -> &'a ElementQuantity {
    inferred.quantities.elements.get(space).unwrap_or_else(|| panic!("no take-off row for {space}"))
}

fn area(inferred: &ModelInference, space: &str, surface: FinishSurface) -> f64 {
    area_of(quantity(inferred, space), surface)
}

fn disagreements(expected: &serde_json::Value, actual: &serde_json::Value, path: &str) -> Vec<String> {
    use serde_json::Value;
    match (expected, actual) {
        (Value::Object(want), Value::Object(got)) => {
            let mut problems: Vec<String> = if want.len() == got.len() { Vec::new() } else { vec![format!("{path}: {} members expected, {} found", want.len(), got.len())] };
            problems.extend(want.iter().flat_map(|(key, value)| got.get(key).map_or_else(|| vec![format!("{path}.{key}: missing")], |found| disagreements(value, found, &format!("{path}.{key}")))));
            problems
        }
        (Value::Number(want), Value::Number(got)) => {
            let (want, got) = (want.as_f64().expect("number"), got.as_f64().expect("number"));
            if (want - got).abs() <= 1e-9 * want.abs().max(1.0) { Vec::new() } else { vec![format!("{path}: oracle {want}, subject {got}")] }
        }
        _ if expected == actual => Vec::new(),
        _ => vec![format!("{path}: oracle {expected}, subject {actual}")],
    }
}

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_third_party_oracle_tables() {
    for (snapshot, table) in [(ROOMS, ROOMS_TABLE), (ONE_ROOM, ONE_ROOM_TABLE)] {
        let produced: serde_json::Value = serde_json::from_str(&table_json(&infer(&load(snapshot)).quantities)).expect("JSON table");
        let expected: serde_json::Value = serde_json::from_str(table).expect("JSON table");
        let problems = disagreements(&expected, &produced, "");
        assert!(problems.is_empty(), "{problems:#?}");
    }
}

fn triangle(solid: &ElementSolid, index: usize) -> [[f64; 3]; 3] {
    let corner = |at: u32| {
        let start = 3 * at as usize;
        [solid.positions[start], solid.positions[start + 1], solid.positions[start + 2]]
    };
    [corner(solid.indices[3 * index]), corner(solid.indices[3 * index + 1]), corner(solid.indices[3 * index + 2])]
}

fn clipped(polygon: Vec<[f64; 3]>, plane: f64, keep_above: bool) -> Vec<[f64; 3]> {
    let inside = |point: &[f64; 3]| if keep_above { point[2] >= plane } else { point[2] <= plane };
    let mut out = Vec::new();
    for index in 0..polygon.len() {
        let (a, b) = (polygon[index], polygon[(index + 1) % polygon.len()]);
        if inside(&a) {
            out.push(a);
        }
        if inside(&a) != inside(&b) {
            let along = (plane - a[2]) / (b[2] - a[2]);
            out.push([a[0] + along * (b[0] - a[0]), a[1] + along * (b[1] - a[1]), plane]);
        }
    }
    out
}

fn polygon_area(polygon: &[[f64; 3]]) -> f64 {
    let mut sum = [0.0; 3];
    for index in 1..polygon.len().saturating_sub(1) {
        let (u, v) = ([polygon[index][0] - polygon[0][0], polygon[index][1] - polygon[0][1], polygon[index][2] - polygon[0][2]], [polygon[index + 1][0] - polygon[0][0], polygon[index + 1][1] - polygon[0][1], polygon[index + 1][2] - polygon[0][2]]);
        sum = [sum[0] + u[1] * v[2] - u[2] * v[1], sum[1] + u[2] * v[0] - u[0] * v[2], sum[2] + u[0] * v[1] - u[1] * v[0]];
    }
    (sum[0] * sum[0] + sum[1] * sum[1] + sum[2] * sum[2]).sqrt() / 2.0
}

fn inside(rings: &[Vec<[f64; 2]>], point: [f64; 2]) -> bool {
    let mut crossings = false;
    for ring in rings {
        for index in 0..ring.len() {
            let (a, b) = (ring[index], ring[(index + 1) % ring.len()]);
            if (a[1] > point[1]) != (b[1] > point[1]) && point[0] < a[0] + (point[1] - a[1]) / (b[1] - a[1]) * (b[0] - a[0]) {
                crossings = !crossings;
            }
        }
    }
    crossings
}

fn facing_the_room(snapshot: &ModelSnapshot, inferred: &ModelInference, space: &str) -> f64 {
    let room = &inferred.spaces[space];
    let rings = rings_of(room);
    let level = inferred.storey_levels[&snapshot.spaces[space].storey];
    let datum = level.absolute_elevation - level.elevation;
    let (floor, ceiling) = (room.floor_z, room.floor_z + room.clear_height);
    let mut total = 0.0;
    for (id, solid) in inferred.element_solids.iter().filter(|(id, _)| snapshot.walls.contains_key(*id)) {
        let _ = id;
        for index in 0..solid.indices.len() / 3 {
            let [a, b, c] = triangle(solid, index);
            let cross = [(b[1] - a[1]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[1] - a[1]), (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2]), (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])];
            let length = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
            if length < 1e-12 || cross[2].abs() > 1e-9 * length {
                continue;
            }
            let centre = [(a[0] + b[0] + c[0]) / 3.0, (a[1] + b[1] + c[1]) / 3.0];
            let normal = [cross[0] / length, cross[1] / length];
            if !on_boundary(&rings, centre) || !inside(&rings, [centre[0] + normal[0] * 1e-3, centre[1] + normal[1] * 1e-3]) {
                continue;
            }
            let shifted: Vec<[f64; 3]> = [a, b, c].iter().map(|point| [point[0], point[1], point[2] - datum]).collect();
            total += polygon_area(&clipped(clipped(shifted, floor, true), ceiling, false));
        }
    }
    total
}

#[semio_framework_async_macros::async_test]
async fn the_wall_finish_of_a_closed_room_is_the_room_facing_area_of_its_wall_solids() {
    let snapshot = load(ONE_ROOM);
    let inferred = infer(&snapshot);
    let finish = area(&inferred, "sp-room", FinishSurface::Wall);
    let solids = facing_the_room(&snapshot, &inferred, "sp-room");
    assert!((finish - solids).abs() < 1e-6, "finish {finish}, wall solids {solids}");
    assert!((finish - (inferred.spaces["sp-room"].perimeter * inferred.spaces["sp-room"].clear_height - (1.2 * 1.2 * 2.0 + 0.9 * 2.1))).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn a_door_between_two_rooms_is_subtracted_from_both() {
    let mut snapshot = load(ROOMS);
    let with = infer(&snapshot);
    snapshot.openings.remove("o-door-part");
    let without = infer(&snapshot);
    for space in ["sp-west", "sp-east"] {
        let gained = area(&without, space, FinishSurface::Wall) - area(&with, space, FinishSurface::Wall);
        assert!((gained - 0.9 * 2.1).abs() < 1e-9, "{space} gained {gained}");
    }
    assert_eq!(area(&without, "sp-up", FinishSurface::Wall), area(&with, "sp-up", FinishSurface::Wall), "another storey is untouched");
}

#[semio_framework_async_macros::async_test]
async fn an_island_counts_both_of_its_faces_in_the_perimeter_and_the_openings_on_it() {
    let snapshot = load(ROOMS);
    let inferred = infer(&snapshot);
    let room = &inferred.spaces["sp-east"];
    assert_eq!(room.holes.len(), 1, "the island wall is a hole of the room");
    let finish = area(&inferred, "sp-east", FinishSurface::Wall);
    assert!((finish - (room.perimeter * room.clear_height - 3.33)).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn an_unfinished_surface_keeps_its_area_with_an_empty_material() {
    let inferred = infer(&load(ROOMS));
    let rows = &quantity(&inferred, "sp-east").finishes;
    assert_eq!(rows.iter().map(|row| (row.surface, row.material.as_str())).collect::<Vec<_>>(), vec![(FinishSurface::Floor, "m-wood"), (FinishSurface::Wall, "m-paint"), (FinishSurface::Ceiling, "")]);
    assert!(rows[2].area > 0.0);
    assert_eq!(inferred.quantities.project.finishes.get("wall:m-paint").map(|totals| totals.count), Some(2), "the project sums the finished surfaces per material");
    assert_eq!(inferred.quantities.project.finishes.get("floor:m-wood").map(|totals| totals.count), Some(2));
    assert!(!inferred.quantities.project.finishes.keys().any(|key| key.ends_with(':')), "an unfinished surface is not a total");
}

#[semio_framework_async_macros::async_test]
async fn an_unresolved_room_has_no_finish_rows() {
    let inferred = infer(&load(ROOMS));
    assert!(!inferred.quantities.elements.contains_key("sp-out"), "the open space has no take-off row");
    let snapshot = load(ROOMS);
    assert!(finish_rows(&snapshot, &snapshot.spaces["sp-out"], &inferred.spaces["sp-out"], &[]).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn an_opening_the_wall_solid_does_not_cut_is_not_subtracted() {
    let mut snapshot = load(ONE_ROOM);
    let before = area(&infer(&snapshot), "sp-room", FinishSurface::Wall);
    snapshot.openings.get_mut("o-win-south").expect("the window").offset = 0.5;
    let inferred = infer(&snapshot);
    assert!(!inferred.opening_frames["o-win-south"].valid, "a window across the wall end is invalid");
    let after = area(&inferred, "sp-room", FinishSurface::Wall);
    assert!((after - before - 1.2 * 1.2).abs() < 1e-9, "the invalid window is no longer subtracted: {before} -> {after}");
}

#[semio_framework_async_macros::async_test]
async fn the_ceiling_is_the_soffit_of_the_slab_that_closes_the_room() {
    let mut snapshot = load(ROOMS);
    snapshot.ceilings.clear();
    let inferred = infer(&snapshot);
    let net = quantity(&inferred, "sp-east").net_area;
    assert!((area(&inferred, "sp-east", FinishSurface::Ceiling) - net / 0.2f64.cos()).abs() < 1e-9, "a sloped slab has a larger soffit");
    assert!((area(&inferred, "sp-west", FinishSurface::Ceiling) - quantity(&inferred, "sp-west").net_area).abs() < 1e-9, "a flat slab has the room area");
    snapshot.slabs.get_mut("sl-east").expect("the slab").slope = None;
    let flat = infer(&snapshot);
    assert!((area(&flat, "sp-east", FinishSurface::Ceiling) - net).abs() < 1e-9);
    assert!((area(&infer(&snapshot), "sp-up", FinishSurface::Ceiling) - 12.0).abs() < 1e-9, "a room under no slab keeps its floor area");
}

#[semio_framework_async_macros::async_test]
async fn the_ceiling_under_a_hung_ceiling_is_the_covered_floor_over_the_slope_of_the_ceiling_plus_the_rest_under_the_soffit() {
    let snapshot = load(ROOMS);
    let inferred = infer(&snapshot);
    assert_eq!(inferred.spaces["sp-west"].ceiling, "ce-living");
    let (covered, tilt) = (2.85 * 5.7, 0.05f64.cos());
    let expected = covered / tilt + (quantity(&inferred, "sp-west").net_area - covered);
    assert!((area(&inferred, "sp-west", FinishSurface::Ceiling) - expected).abs() < 1e-9, "the hung ceiling covers x 0.15 to 3 of the living room, the flat slab the rest");
    assert_eq!(inferred.spaces["sp-east"].ceiling, "ce-kitchen");
    let covered = quantity(&inferred, "sp-east").net_area - 0.5;
    let expected = covered / 0.1f64.cos() + 0.5 / 0.2f64.cos();
    assert!((area(&inferred, "sp-east", FinishSurface::Ceiling) - expected).abs() < 1e-9, "the hole of the ceiling (0.5) is closed by the sloped soffit, the island and the column are not counted: {expected}");
    assert!((area(&inferred, "sp-up", FinishSurface::Ceiling) - 12.0).abs() < 1e-9, "a room of another storey is untouched");
}

#[semio_framework_async_macros::async_test]
async fn without_authored_ceilings_the_finish_falls_back_to_the_soffit() {
    let mut snapshot = load(ROOMS);
    snapshot.ceilings.clear();
    let inferred = infer(&snapshot);
    assert_eq!(inferred.spaces["sp-west"].ceiling, "");
    assert!((area(&inferred, "sp-west", FinishSurface::Ceiling) - quantity(&inferred, "sp-west").net_area).abs() < 1e-9);
    assert!((area(&inferred, "sp-east", FinishSurface::Ceiling) - quantity(&inferred, "sp-east").net_area / 0.2f64.cos()).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn a_ceiling_that_does_not_reach_the_room_leaves_the_soffit_ceiling() {
    let mut snapshot = load(ROOMS);
    let kitchen = snapshot.ceilings.get_mut("ce-kitchen").expect("the kitchen ceiling");
    let corner = |x: f64, y: f64| Vertex { point: crate::Point2 { x, y }, bulge: 0.0 };
    kitchen.boundary = vec![corner(4.2, 0.2), corner(5.0, 0.2), corner(5.0, 0.8), corner(4.2, 0.8)];
    kitchen.holes.clear();
    let inferred = infer(&snapshot);
    assert_eq!(inferred.spaces["sp-east"].ceiling, "", "the seed of the room lies outside the ceiling");
    assert!((area(&inferred, "sp-east", FinishSurface::Ceiling) - quantity(&inferred, "sp-east").net_area / 0.2f64.cos()).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn a_ceiling_edit_re_infers_only_the_take_off_of_the_spaces_it_hangs_over() {
    let snapshot = load(ROOMS);
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let diff = ModelDiff::ceilings("ce-kitchen", Entry::Patched(CeilingPatch { holes: Some(Vec::new()), ..Default::default() }));
    let edited = protocol::apply_diff(&diff, &snapshot).expect("applies");
    let ceiling = session.update(&edited, &diff).quantities.elements["sp-east"].finishes[2].area;
    assert!((ceiling - quantity(&infer(&snapshot), "sp-east").net_area / 0.1f64.cos()).abs() < 1e-9, "without the hole the ceiling covers the whole kitchen floor");
    let computed = &session.report().computed_by_kind;
    for kind in ["wall-layout", "opening-frame"] {
        assert_eq!(computed.get(kind), None, "a ceiling edit recomputes no {kind}");
    }
    assert_eq!(session.inference(), &infer(&edited), "the session agrees with a fresh inference");
}

#[semio_framework_async_macros::async_test]
async fn a_finish_edit_re_infers_only_the_take_off_of_its_space() {
    let snapshot = load(ROOMS);
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let diff = ModelDiff::spaces("sp-west", Entry::Patched(SpacePatch { wall_finish: Some(Assigned::new(Some("m-wood".into()))), ..Default::default() }));
    let edited = protocol::apply_diff(&diff, &snapshot).expect("applies");
    let wall = session.update(&edited, &diff).quantities.elements["sp-west"].finishes.clone();
    assert_eq!(wall[1].material, "m-wood");
    let computed = &session.report().computed_by_kind;
    for kind in ["room", "wall-layout", "solid", "plan", "opening-frame"] {
        assert_eq!(computed.get(kind), None, "a finish edit recomputes no {kind}");
    }
    assert_eq!(computed.get("quantity"), Some(&1));
    assert_eq!(session.inference(), &infer(&edited), "the session agrees with a fresh inference");
}

#[semio_framework_async_macros::async_test]
async fn the_finishes_are_deterministic_and_empty_by_default() {
    let snapshot = load(ROOMS);
    assert_eq!(infer(&snapshot).quantities, infer(&snapshot).quantities);
    assert_eq!(table_json(&infer(&ModelSnapshot::default()).quantities), "{}");
}
