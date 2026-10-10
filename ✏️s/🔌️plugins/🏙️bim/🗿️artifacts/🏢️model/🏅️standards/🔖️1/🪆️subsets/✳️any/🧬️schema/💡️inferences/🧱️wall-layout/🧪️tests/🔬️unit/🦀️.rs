use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelNode};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{compute_storey_levels, target_of};
use crate::{Entry, ModelDiff, StoreyPatch, TopConstraint, WallPatch};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");
const JOINS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🔗️wall-joins/📸️snapshot/🔣️.json");

const CASES: [(&str, &str); 5] = [
    ("l-miter-equal", include_str!("../../../../../🧫️fixtures/💡️inferences/🧱️wall-layout/📐️l-miter-equal/🔣️.json")),
    ("t-butt", include_str!("../../../../../🧫️fixtures/💡️inferences/🧱️wall-layout/🔱️t-butt/🔣️.json")),
    ("x-cross", include_str!("../../../../../🧫️fixtures/💡️inferences/🧱️wall-layout/❌️x-cross/🔣️.json")),
    ("location-lines", include_str!("../../../../../🧫️fixtures/💡️inferences/🧱️wall-layout/📏️location-lines/🔣️.json")),
    ("arc-tangent", include_str!("../../../../../🧫️fixtures/💡️inferences/🧱️wall-layout/🌀️arc-tangent/🔣️.json")),
];

fn decode(text: &str) -> ModelSnapshot {
    from_json_str(text, JsonMemberPolicy::Reject).expect("the snapshot decodes")
}

fn house() -> ModelSnapshot {
    decode(HOUSE)
}

fn joins() -> ModelSnapshot {
    decode(JOINS)
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

fn raised(delta: f64) -> ModelSnapshot {
    let height = house().storeys["st-ground"].height + delta;
    protocol::apply_diff(&ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(height), ..Default::default() })), &house()).expect("the height edit applies")
}

fn layout_json(layout: &WallLayout) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(layout)).expect("the layout encodes as JSON")
}

fn differences(expected: &serde_json::Value, actual: &serde_json::Value, path: &str) -> Vec<String> {
    use serde_json::Value;
    match (expected, actual) {
        (Value::Object(want), Value::Object(got)) => want.iter().flat_map(|(key, value)| got.get(key).map_or_else(|| vec![format!("{path}.{key}: missing")], |found| differences(value, found, &format!("{path}.{key}")))).collect(),
        (Value::Array(want), Value::Array(got)) if want.len() == got.len() => want.iter().zip(got).enumerate().flat_map(|(index, (value, found))| differences(value, found, &format!("{path}[{index}]"))).collect(),
        (Value::Array(want), Value::Array(got)) => vec![format!("{path}: {} items expected, {} found", want.len(), got.len())],
        (Value::Number(want), Value::Number(got)) => {
            let (want, got) = (want.as_f64().expect("number"), got.as_f64().expect("number"));
            if (want - got).abs() <= 1e-9 * want.abs().max(1.0) { Vec::new() } else { vec![format!("{path}: expected {want}, found {got}")] }
        }
        _ if expected == actual => Vec::new(),
        _ => vec![format!("{path}: expected {expected}, found {actual}")],
    }
}

#[semio_framework_async_macros::async_test]
async fn the_language_agnostic_cases_are_reproduced() {
    for (name, text) in CASES {
        let case: serde_json::Value = serde_json::from_str(text).expect("the case is JSON");
        let layouts = compute_wall_layout(&decode(&case["before"].to_string()));
        for (id, expected) in case["expected"].as_object().expect("expected walls") {
            let problems = differences(expected, &layout_json(&layouts[id]), id);
            assert!(problems.is_empty(), "{name}: {problems:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn walls_resolve_base_top_thickness_and_quantities() {
    let layouts = compute_wall_layout(&house());
    let south = &layouts["w-ground-south"];
    assert!(close(south.base_z, 0.0) && close(south.top_z, 3.0) && close(south.height, 3.0));
    assert!(close(south.thickness, 0.3) && close(south.length, 8.0));
    assert!(close(south.side_area, 24.0), "the centreline area stays length times height");
    assert!(close(south.footprint_area, 2.3775) && close(south.volume, 7.1325), "the east wall mitres the end: (8 + 7.85) / 2 * 0.3");
    let basement = &layouts["w-basement-west"];
    assert!(close(basement.base_z, -2.6) && close(basement.top_z, -0.6) && close(basement.height, 2.0));
    assert!(basement.joins.is_empty() && close(basement.footprint_area, 6.0 * 0.3), "a free wall keeps its square footprint");
    let east = &layouts["w-ground-east"];
    assert!(close(east.base_z, 0.1) && close(east.height, 2.4) && close(east.thickness, 0.15));
    assert!(close(east.offset_left, 0.15) && close(east.offset_right, 0.0), "Exterior puts the axis on the right face");
    let arc = &layouts["w-ground-arc"];
    assert!(close(arc.top_z, 2.8) && close(arc.height, 2.8), "constrained to the first storey's elevation minus 0.2");
    assert!((arc.length - 9.272952180016122).abs() < 1e-9, "arc length of bulge 0.5 over a chord of 8");
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_re_infers_exactly_what_depends_on_it() {
    let (before, after) = (compute_wall_layout(&house()), compute_wall_layout(&raised(0.4)));
    let moved = |id: &str, field: fn(&WallLayout) -> f64| field(&after[id]) - field(&before[id]);
    assert!(close(moved("w-ground-south", |w| w.height), 0.4), "a StoreyTop wall grows with its storey");
    assert!(close(moved("w-ground-south", |w| w.left_area), 0.4 * before["w-ground-south"].left_length), "the side areas follow the height");
    assert!(close(moved("w-ground-east", |w| w.height), 0.0), "an Unconnected wall keeps its height");
    assert!(close(moved("w-ground-arc", |w| w.height), 0.4), "a wall constrained to the storey above follows that storey's elevation");
    assert!(close(moved("w-first-west", |w| w.base_z), 0.4) && close(moved("w-first-west", |w| w.height), 0.0), "walls above are lifted, not stretched");
    assert!(close(moved("w-first-north", |w| w.top_z), 0.4));
    assert_eq!(after["w-basement-west"], before["w-basement-west"], "walls below are untouched");
    assert_eq!(after["w-shed-south"], before["w-shed-south"], "walls of another building are untouched");
    for id in before.keys() {
        assert_eq!(after[id].footprint, before[id].footprint, "{id}: the plan geometry does not depend on a storey height");
        assert_eq!(after[id].joins, before[id].joins);
    }
    let (levels_before, levels_after) = (compute_storey_levels_for_test(&house()), compute_storey_levels_for_test(&raised(0.4)));
    assert!(close(levels_after["st-first"].0 - levels_before["st-first"].0, 0.4));
    assert!(close(levels_after["st-shed"].0 - levels_before["st-shed"].0, 0.0));
}

fn compute_storey_levels_for_test(snapshot: &ModelSnapshot) -> BTreeMap<String, (f64, f64)> {
    crate::standards::v1::subsets::any::schema::inferences::storey_levels::compute_storey_levels(snapshot).into_iter().map(|(id, level)| (id, (level.elevation, level.top_elevation))).collect()
}

#[semio_framework_async_macros::async_test]
async fn freeing_a_wall_top_makes_it_independent_of_storey_heights() {
    let freed = protocol::apply_diff(&ModelDiff::walls("w-ground-south", Entry::Patched(WallPatch { top: Some(TopConstraint::Unconnected { height: 3.0 }), ..Default::default() })), &house()).expect("the top edit applies");
    let after = protocol::apply_diff(&ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() })), &freed).expect("the height edit applies");
    assert!(close(compute_wall_layout(&after)["w-ground-south"].height, 3.0));
}

#[semio_framework_async_macros::async_test]
async fn the_layout_is_deterministic() {
    assert_eq!(compute_wall_layout(&joins()), compute_wall_layout(&joins()));
    assert!(compute_wall_layout(&ModelSnapshot::default()).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn every_join_is_seen_from_both_walls() {
    let layouts = compute_wall_layout(&joins());
    let mirror = |kind: JoinKind| match kind {
        JoinKind::Miter => JoinKind::Miter,
        JoinKind::Butt => JoinKind::Through,
        JoinKind::Through => JoinKind::Butt,
        JoinKind::Cross => JoinKind::Cross,
    };
    let mut count = std::collections::BTreeMap::<&str, usize>::new();
    for (id, layout) in &layouts {
        for join in &layout.joins {
            *count.entry(match join.kind { JoinKind::Miter => "miter", JoinKind::Butt => "butt", JoinKind::Through => "through", JoinKind::Cross => "cross" }).or_default() += 1;
            let back = layouts[&join.other].joins.iter().find(|other| other.other == *id && other.kind == mirror(join.kind) && other.end == join.other_end && other.other_end == join.end);
            let back = back.unwrap_or_else(|| panic!("{id} -> {} ({:?}) has no mirror", join.other, join.kind));
            assert!((back.point.x - join.point.x).abs() < 1e-6 && (back.point.y - join.point.y).abs() < 1e-6 && close(back.overlap_area, join.overlap_area));
        }
    }
    assert_eq!(count["butt"], count["through"]);
    assert!(count["miter"] > 0 && count["butt"] > 0 && count["cross"] > 0);
}

#[semio_framework_async_macros::async_test]
async fn walls_of_another_storey_never_join() {
    let layouts = compute_wall_layout(&joins());
    assert!(layouts["w-up-a"].joins.is_empty(), "w-up-a ends where two ground walls meet but lives on the first storey");
    assert!(layouts["w-l-a"].joins.iter().all(|join| join.other != "w-up-a"));
    assert!(close(layouts["w-up-a"].footprint_area, 4.0 * 0.3));
}

#[semio_framework_async_macros::async_test]
async fn the_footprints_of_joined_walls_tile_without_overlap_and_cover_the_union() {
    let layouts = compute_wall_layout(&joins());
    for id in ["w-l-a", "w-ac-a", "w-n3-a", "w-n4-a", "w-ra-arc", "w-ct-arc"] {
        let layout = &layouts[id];
        assert!(layout.footprint_area > 0.0 && layout.footprint.len() == 4, "{id} has a footprint loop");
        assert!(close(layout.volume, layout.footprint_area * layout.height), "{id}: volume = footprint area * height");
        assert!(close(layout.left_area, layout.left_length * layout.height) && close(layout.right_area, layout.right_length * layout.height));
    }
    let l = layouts["w-l-a"].footprint_area + layouts["w-l-b"].footprint_area;
    assert!(close(l, 0.3 * (4.0 + 3.0)), "a mitered L of equal thickness moves area between its walls: thickness times the centreline lengths, {l}");
    let acute = layouts["w-ac-a"].footprint_area + layouts["w-ac-b"].footprint_area;
    assert!(close(acute, 0.3 * (4.0 + 4.0)), "the same holds at 60 degrees: {acute}");
    let n3 = ["w-n3-a", "w-n3-b", "w-n3-c"].iter().map(|id| layouts[*id].footprint_area).sum::<f64>();
    assert!(n3 > 0.0 && layouts["w-n3-a"].joins.len() == 2 && layouts["w-n3-c"].joins.len() == 2, "a node of three walls joins every pair");
    let n4 = ["w-n4-a", "w-n4-b", "w-n4-c", "w-n4-d"].iter().all(|id| layouts[*id].joins.len() == 3);
    assert!(n4, "a node of four walls joins every pair");
}

#[semio_framework_async_macros::async_test]
async fn a_free_wall_footprint_equals_the_geometry_kernel_band() {
    use semio_framework_geometry::bulge::band_loop;
    let snapshot = joins();
    let layouts = compute_wall_layout(&snapshot);
    for id in ["w-loc-center", "w-loc-interior", "w-loc-exterior", "w-loc-core", "w-up-a"] {
        let wall = &snapshot.walls[id];
        let offsets = offsets_of(&snapshot, wall);
        let expected = band_loop(&seg(&wall.axis), offsets.left, offsets.right, None, None).expect("a band");
        let found = &layouts[id].footprint;
        for (vertex, (point, bulge)) in found.iter().zip(expected) {
            assert!(close(vertex.point.x, point.x) && close(vertex.point.y, point.y) && close(vertex.bulge, bulge), "{id}: {vertex:?} vs {point:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn layer_offsets_run_from_the_left_face_to_the_right_face() {
    let snapshot = joins();
    for (id, layout) in compute_wall_layout(&snapshot) {
        let layers = snapshot.wall_types[&snapshot.walls[&id].wall_type].layers.len();
        assert_eq!(layout.layer_offsets.len(), layers + 1, "{id}");
        assert!(close(layout.layer_offsets[0], layout.offset_left) && close(layout.layer_offsets[layers], -layout.offset_right), "{id}");
        assert!(close(layout.offset_left + layout.offset_right, layout.thickness), "{id}");
        assert!(layout.layer_offsets.windows(2).all(|pair| pair[0] >= pair[1] - 1e-12), "{id}: interfaces descend from left to right");
    }
}

#[semio_framework_async_macros::async_test]
async fn changing_one_wall_type_thickness_moves_its_neighbours_footprints_but_not_unrelated_walls() {
    let before = joins();
    let mut after = before.clone();
    after.wall_types.get_mut("wt-150").expect("type").layers[0].thickness = 0.3;
    let (old, new) = (compute_wall_layout(&before), compute_wall_layout(&after));
    for id in ["w-ll-b", "w-ll-a", "w-t1-stem", "w-x-b", "w-ra-line", "w-ra-arc", "w-n3-a", "w-n3-b"] {
        assert_ne!(old[id].footprint, new[id].footprint, "{id} is a wt-150 wall or mitered to one");
    }
    assert_eq!(old["w-x-a"].footprint, new["w-x-a"].footprint, "a crossing trims nothing");
    assert_ne!(old["w-x-a"].joins, new["w-x-a"].joins, "but the overlap with the thicker crossing wall grows");
    assert_eq!(old["w-t1-through"], new["w-t1-through"], "a through wall is not trimmed by the stem that butts into it");
    for id in ["w-l-a", "w-l-b", "w-ac-a", "w-ac-b", "w-t2-through", "w-t2-stem", "w-ta-arc", "w-ta-line", "w-loc-center", "w-up-a"] {
        assert_eq!(old[id], new[id], "{id} neither uses wt-150 nor touches a wall that does");
    }
}

#[semio_framework_async_macros::async_test]
async fn degenerate_walls_get_no_footprint() {
    let mut snapshot = joins();
    snapshot.walls.get_mut("w-loc-center").expect("wall").axis = Axis::Line { start: crate::Point2 { x: 1.0, y: 1.0 }, end: crate::Point2 { x: 1.0, y: 1.0 } };
    snapshot.walls.get_mut("w-loc-interior").expect("wall").wall_type = "no-such-type".into();
    let layouts = compute_wall_layout(&snapshot);
    let flat = &layouts["w-loc-center"];
    assert!(flat.footprint.is_empty() && flat.joins.is_empty() && flat.length == 0.0 && flat.footprint_area == 0.0 && flat.volume == 0.0);
    let typeless = &layouts["w-loc-interior"];
    assert!(close(typeless.thickness, 0.0) && close(typeless.footprint_area, 0.0) && typeless.layer_offsets == vec![0.0]);
}

#[semio_framework_async_macros::async_test]
async fn a_wall_is_resolved_by_its_storeys_and_the_bands_of_the_walls_it_touches() {
    let snapshot = house();
    let steps = plan::build(&snapshot, kinds::closure(kinds::LAYOUTS));
    let parents = |wall: &str| steps.iter().find(|step| step.key == ModelNode::WallLayout(wall.into())).map(|step| step.parents.clone()).expect("planned");
    assert_eq!(parents("w-ground-south")[0], ModelNode::Storey("st-ground".into()));
    assert_eq!(parents("w-ground-arc")[..2], [ModelNode::Storey("st-ground".into()), ModelNode::Storey("st-first".into())]);
    let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).expect("planned");
    for step in &steps {
        for parent in &step.parents {
            assert!(position(parent) < position(&step.key), "parents come first");
        }
        if let ModelNode::WallLayout(id) = &step.key {
            assert!(step.parents.iter().all(|parent| matches!(parent, ModelNode::Storey(_) | ModelNode::Band(_))), "neighbours enter as bands, never as layouts");
            assert!(step.parents.contains(&ModelNode::Band(id.clone())), "{id}: a wall always has its own band");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn moving_a_wall_into_contact_makes_its_band_a_parent_of_the_other() {
    let before = joins();
    let mut after = before.clone();
    let wall = after.walls.get_mut("w-loc-center").expect("wall");
    wall.axis = Axis::Line { start: crate::Point2 { x: 4.0, y: 0.0 }, end: crate::Point2 { x: 4.0, y: -3.0 } };
    wall.storey = "st-ground".into();
    let parents = |snapshot: &ModelSnapshot, wall: &str| plan::build(snapshot, kinds::closure(kinds::LAYOUTS)).into_iter().find(|step| step.key == ModelNode::WallLayout(wall.into())).expect("planned").parents;
    let band = ModelNode::Band("w-loc-center".into());
    assert!(!parents(&before, "w-l-b").contains(&band) && parents(&after, "w-l-b").contains(&band), "a wall that starts touching becomes a parent");
    assert_eq!(parents(&before, "w-ll-a"), parents(&after, "w-ll-a"), "an unrelated wall keeps its parents");
    assert!(compute_wall_layout(&after)["w-l-b"].joins.iter().any(|join| join.other == "w-loc-center"));
}

#[semio_framework_async_macros::async_test]
async fn a_layout_over_the_neighbourhood_equals_the_layout_over_the_whole_storey() {
    for snapshot in [house(), joins()] {
        let (layouts, levels) = (compute_wall_layout(&snapshot), compute_storey_levels(&snapshot));
        for (id, wall) in &snapshot.walls {
            let whole = layout_of(&snapshot, id, wall, &levels[&wall.storey], target_of(&wall.top, &levels), &storey_bands(&snapshot, &wall.storey));
            assert_eq!(layouts[id], whole, "{id}");
        }
    }
}
