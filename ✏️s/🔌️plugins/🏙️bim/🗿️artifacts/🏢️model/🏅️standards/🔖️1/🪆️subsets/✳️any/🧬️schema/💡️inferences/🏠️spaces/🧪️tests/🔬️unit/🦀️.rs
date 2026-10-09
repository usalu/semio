use crate::standards::v1::subsets::any::io::text::inferences::spaces::table_json;
use super::*;
use crate::{Axis, Building, Column, ColumnType, Entry, Layer, LayerFunction, LocationLine, Material, MaterialCategory, ModelDiff, Phase, Profile, Rgb, Site, Slab, SlabType, Space, Storey, TopConstraint, Wall, WallPatch, WallType};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{compute, kinds, plan, ModelInferenceSession, ModelNode};
use protocol::Inference;

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

fn near(left: f64, right: f64, tolerance: f64) -> bool {
    (left - right).abs() <= tolerance
}

fn point(x: f64, y: f64) -> Point2 {
    Point2 { x, y }
}

fn corner(x: f64, y: f64) -> Vertex {
    Vertex { point: point(x, y), bulge: 0.0 }
}

fn line(a: (f64, f64), b: (f64, f64)) -> Axis {
    Axis::Line { start: point(a.0, a.1), end: point(b.0, b.1) }
}

fn wall(storey: &str, axis: Axis) -> Wall {
    Wall { storey: storey.into(), wall_type: "wt".into(), axis, location: LocationLine::Center, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, start_join: None, end_join: None, base_slab: None, name: "Wall".into() }
}

fn space(storey: &str, boundary: SpaceBoundary) -> Space {
    Space { phase: crate::Phase::New, storey: storey.into(), number: "1".into(), name: "Room".into(), boundary, usage: "office".into(), zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None }
}

fn base() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.materials.insert("m".into(), Material { name: "Brick".into(), category: MaterialCategory::Masonry, color: Rgb { r: 0.7, g: 0.3, b: 0.2 }, density: 1800.0, conductivity: 0.8, specific_heat: 900.0 });
    snapshot.wall_types.insert("wt".into(), WallType { name: "Brick 200".into(), layers: vec![Layer { material: "m".into(), thickness: 0.2, function: LayerFunction::Structure }] });
    snapshot.slab_types.insert("st".into(), SlabType { name: "Slab 250".into(), layers: vec![Layer { material: "m".into(), thickness: 0.25, function: LayerFunction::Structure }] });
    snapshot.sites.insert("site".into(), Site { name: "Site".into(), latitude: 0.0, longitude: 0.0, elevation: 0.0, true_north: 0.0, boundary: Vec::new() });
    snapshot.buildings.insert("bldg".into(), Building { site: "site".into(), name: "Building".into(), origin: point(0.0, 0.0), rotation: 0.0, elevation: 0.0 });
    snapshot.storeys.insert("st-0".into(), Storey { building: "bldg".into(), name: "Ground".into(), level: 0, height: 3.0, cut_height: None });
    snapshot.storeys.insert("st-1".into(), Storey { building: "bldg".into(), name: "First".into(), level: 1, height: 3.0, cut_height: None });
    snapshot
}

fn rectangle(snapshot: &mut ModelSnapshot, width: f64, depth: f64) {
    let corners = [(0.0, 0.0), (width, 0.0), (width, depth), (0.0, depth)];
    for (index, name) in ["south", "east", "north", "west"].into_iter().enumerate() {
        snapshot.walls.insert(format!("w-{name}"), wall("st-0", line(corners[index], corners[(index + 1) % 4])));
    }
}

fn room(width: f64, depth: f64) -> ModelSnapshot {
    let mut snapshot = base();
    rectangle(&mut snapshot, width, depth);
    snapshot.spaces.insert("sp".into(), space("st-0", SpaceBoundary::Bounded { seed: point(width / 2.0, depth / 2.0) }));
    snapshot
}

fn rooms(snapshot: &ModelSnapshot) -> BTreeMap<String, SpaceRoom> {
    compute_spaces(snapshot)
}

#[semio_framework_async_macros::async_test]
async fn a_closed_wall_loop_bounds_the_room_between_the_inner_faces() {
    let found = rooms(&room(4.0, 3.0))["sp"].clone();
    assert_eq!(found.status, SpaceStatus::Inferred);
    assert!(near(found.area, 3.8 * 2.8, 1e-9) && near(found.perimeter, 2.0 * (3.8 + 2.8), 1e-9), "area {} perimeter {}", found.area, found.perimeter);
    assert!(near(found.net_floor_area, found.area, 1e-12));
    assert!(close(found.floor_z, 0.0) && close(found.clear_height, 3.0) && near(found.volume, 3.8 * 2.8 * 3.0, 1e-9));
    assert_eq!(found.bounding_walls, ["w-east", "w-north", "w-south", "w-west"]);
    assert_eq!(found.outline.len(), 4);
    assert!(found.holes.is_empty() && found.ceiling_slab.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn moving_a_wall_changes_the_room_area() {
    let before = rooms(&room(4.0, 3.0))["sp"].area;
    let mut wider = room(4.0, 3.0);
    wider.walls.get_mut("w-east").expect("wall").axis = line((6.0, 0.0), (6.0, 3.0));
    wider.walls.get_mut("w-south").expect("wall").axis = line((0.0, 0.0), (6.0, 0.0));
    wider.walls.get_mut("w-north").expect("wall").axis = line((6.0, 3.0), (0.0, 3.0));
    let after = rooms(&wider)["sp"].area;
    assert!(near(before, 3.8 * 2.8, 1e-9) && near(after, 5.8 * 2.8, 1e-9), "{before} -> {after}");
    let mut thicker = room(4.0, 3.0);
    thicker.wall_types.get_mut("wt").expect("type").layers[0].thickness = 0.4;
    assert!(near(rooms(&thicker)["sp"].area, 3.6 * 2.6, 1e-9), "a thicker wall type shrinks the room");
}

#[semio_framework_async_macros::async_test]
async fn a_gap_in_the_walls_leaves_the_room_open() {
    let mut snapshot = room(4.0, 3.0);
    snapshot.walls.remove("w-west");
    let found = rooms(&snapshot)["sp"].clone();
    assert_eq!(found.status, SpaceStatus::NotEnclosed);
    assert!(found.area == 0.0 && found.outline.is_empty() && found.bounding_walls.is_empty());
    snapshot.walls.clear();
    assert_eq!(rooms(&snapshot)["sp"].status, SpaceStatus::NotEnclosed, "no walls at all is open too");
}

#[semio_framework_async_macros::async_test]
async fn a_seed_inside_a_wall_or_outside_the_building_is_reported() {
    let mut snapshot = room(4.0, 3.0);
    snapshot.spaces.insert("sp-wall".into(), space("st-0", SpaceBoundary::Bounded { seed: point(2.0, 0.05) }));
    snapshot.spaces.insert("sp-out".into(), space("st-0", SpaceBoundary::Bounded { seed: point(9.0, 9.0) }));
    let found = rooms(&snapshot);
    assert_eq!(found["sp-wall"].status, SpaceStatus::SeedInsideWall);
    assert_eq!(found["sp-out"].status, SpaceStatus::NotEnclosed);
    assert_eq!(found["sp"].status, SpaceStatus::Inferred);
}

#[semio_framework_async_macros::async_test]
async fn an_inner_partition_splits_a_room_in_two() {
    let mut snapshot = room(6.0, 3.0);
    snapshot.walls.insert("w-partition".into(), wall("st-0", line((3.0, 0.0), (3.0, 3.0))));
    snapshot.spaces.insert("sp".into(), space("st-0", SpaceBoundary::Bounded { seed: point(1.5, 1.5) }));
    snapshot.spaces.insert("sp-b".into(), space("st-0", SpaceBoundary::Bounded { seed: point(4.5, 1.5) }));
    let found = rooms(&snapshot);
    assert!(near(found["sp"].area, 2.8 * 2.8, 1e-9) && near(found["sp-b"].area, 2.8 * 2.8, 1e-9));
    assert!(found["sp"].bounding_walls.contains(&"w-partition".to_string()) && found["sp-b"].bounding_walls.contains(&"w-partition".to_string()));
    assert!(!found["sp"].bounding_walls.contains(&"w-east".to_string()), "a wall that does not touch the room does not bound it");
}

#[semio_framework_async_macros::async_test]
async fn an_island_wall_is_a_hole_of_the_room() {
    let mut snapshot = room(6.0, 4.0);
    snapshot.walls.insert("w-island".into(), wall("st-0", line((2.0, 2.0), (4.0, 2.0))));
    snapshot.spaces.insert("sp".into(), space("st-0", SpaceBoundary::Bounded { seed: point(1.0, 1.0) }));
    let found = rooms(&snapshot)["sp"].clone();
    assert_eq!(found.holes.len(), 1);
    assert!(near(found.area, 5.8 * 3.8 - 2.0 * 0.2, 1e-9), "{}", found.area);
    assert!(near(found.perimeter, 2.0 * (5.8 + 3.8) + 2.0 * (2.0 + 0.2), 1e-9));
}

#[semio_framework_async_macros::async_test]
async fn a_curved_wall_closes_a_half_disc_room() {
    let mut snapshot = base();
    snapshot.walls.insert("w-chord".into(), wall("st-0", line((-2.0, 0.0), (2.0, 0.0))));
    snapshot.walls.insert("w-arc".into(), wall("st-0", Axis::Arc { start: point(2.0, 0.0), end: point(-2.0, 0.0), bulge: 1.0 }));
    snapshot.spaces.insert("sp".into(), space("st-0", SpaceBoundary::Bounded { seed: point(0.0, 1.0) }));
    let found = rooms(&snapshot)["sp"].clone();
    let (radius, distance) = (1.9f64, 0.1f64);
    let expected = radius * radius * (distance / radius).acos() - distance * (radius * radius - distance * distance).sqrt();
    assert_eq!(found.status, SpaceStatus::Inferred);
    assert!(near(found.area, expected, 1e-3), "area {} against the circular segment {expected}", found.area);
}

#[semio_framework_async_macros::async_test]
async fn explicit_outlines_are_exact_for_arcs() {
    let mut snapshot = base();
    snapshot.spaces.insert("sp".into(), space("st-0", SpaceBoundary::Explicit { outline: vec![Vertex { point: point(-1.0, 0.0), bulge: 1.0 }, Vertex { point: point(1.0, 0.0), bulge: 1.0 }] }));
    snapshot.spaces.insert("sq".into(), space("st-0", SpaceBoundary::Explicit { outline: vec![corner(0.0, 0.0), corner(0.0, 2.0), corner(2.0, 2.0), corner(2.0, 0.0)] }));
    snapshot.spaces.insert("sx".into(), space("st-0", SpaceBoundary::Explicit { outline: vec![corner(0.0, 0.0), corner(1.0, 0.0)] }));
    let found = rooms(&snapshot);
    assert_eq!(found["sp"].status, SpaceStatus::Explicit);
    assert!(near(found["sp"].area, std::f64::consts::PI, 1e-12) && near(found["sp"].perimeter, 2.0 * std::f64::consts::PI, 1e-12));
    assert!(near(found["sq"].area, 4.0, 1e-12) && loops::signed_area(&plan_loop(&found["sq"].outline)) > 0.0, "a clockwise outline is normalised to counter-clockwise");
    assert_eq!(found["sx"].status, SpaceStatus::InvalidOutline);
    assert!(found["sq"].bounding_walls.is_empty() && close(found["sq"].volume, 12.0));
}

#[semio_framework_async_macros::async_test]
async fn columns_reduce_the_net_floor_area_only() {
    let mut snapshot = room(4.0, 3.0);
    snapshot.column_types.insert("ct".into(), ColumnType { name: "Square".into(), profile: Profile::Rectangle { width: 0.4, depth: 0.4 }, material: "m".into() });
    snapshot.columns.insert("c-in".into(), Column { phase: crate::Phase::New, storey: "st-0".into(), column_type: "ct".into(), position: point(2.0, 1.5), rotation: 0.3, tilt: None, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, name: "Column".into() });
    snapshot.columns.insert("c-out".into(), Column { phase: crate::Phase::New, storey: "st-0".into(), column_type: "ct".into(), position: point(9.0, 9.0), rotation: 0.0, tilt: None, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, name: "Far".into() });
    let found = rooms(&snapshot)["sp"].clone();
    assert!(near(found.area, 3.8 * 2.8, 1e-9));
    assert!(near(found.net_floor_area, 3.8 * 2.8 - 0.16, 1e-9), "{}", found.net_floor_area);
}

#[semio_framework_async_macros::async_test]
async fn the_slab_above_lowers_the_clear_height() {
    let mut snapshot = room(4.0, 3.0);
    let square = vec![corner(-1.0, -1.0), corner(5.0, -1.0), corner(5.0, 4.0), corner(-1.0, 4.0)];
    snapshot.slabs.insert("sl-thin".into(), Slab { phase: crate::Phase::New, storey: "st-1".into(), slab_type: "st".into(), boundary: square.clone(), holes: Vec::new(), offset: 0.0, slope: None, name: "Floor".into() });
    snapshot.slab_types.insert("st-thick".into(), SlabType { name: "Thick".into(), layers: vec![Layer { material: "m".into(), thickness: 0.4, function: LayerFunction::Structure }] });
    snapshot.slabs.insert("sl-thick".into(), Slab { phase: crate::Phase::New, storey: "st-1".into(), slab_type: "st-thick".into(), boundary: square, holes: vec![vec![corner(1.5, 1.0), corner(2.5, 1.0), corner(2.5, 2.0), corner(1.5, 2.0)]], offset: 0.1, slope: None, name: "Hole".into() });
    snapshot.slabs.insert("sl-ground".into(), Slab { phase: crate::Phase::New, storey: "st-0".into(), slab_type: "st".into(), boundary: vec![corner(-1.0, -1.0), corner(5.0, -1.0), corner(5.0, 4.0)], holes: Vec::new(), offset: 0.0, slope: None, name: "Own floor".into() });
    let found = rooms(&snapshot)["sp"].clone();
    assert_eq!(found.ceiling_slab, "sl-thin", "the seed lies in the hole of the thick slab, so only the thin slab covers it");
    assert!(close(found.clear_height, 3.0 - 0.25) && near(found.volume, found.area * 2.75, 1e-9));
    snapshot.spaces.insert("sp".into(), space("st-0", SpaceBoundary::Bounded { seed: point(0.5, 0.5) }));
    let found = rooms(&snapshot)["sp"].clone();
    assert_eq!(found.ceiling_slab, "sl-thick");
    assert!(close(found.clear_height, 3.0 + 0.1 - 0.4), "the offset raises the underside: {}", found.clear_height);
}

#[semio_framework_async_macros::async_test]
async fn the_clear_height_follows_the_storey_height() {
    let mut snapshot = room(4.0, 3.0);
    let before = rooms(&snapshot)["sp"].clone();
    snapshot.storeys.get_mut("st-0").expect("storey").height = 3.4;
    let after = rooms(&snapshot)["sp"].clone();
    assert!(close(after.clear_height - before.clear_height, 0.4) && close(after.area, before.area));
    assert!(near(after.volume - before.volume, 0.4 * before.area, 1e-9));
}

#[semio_framework_async_macros::async_test]
async fn curtain_walls_close_a_room_with_their_mullion_depth() {
    let mut snapshot = room(4.0, 3.0);
    snapshot.walls.remove("w-north");
    snapshot.curtain_wall_types.insert("cwt".into(), crate::CurtainWallType { name: "Facade".into(), u_grid: crate::CurtainGrid::Spacing { spacing: 1.0 }, v_grid: crate::CurtainGrid::Spacing { spacing: 1.0 }, interior_mullion: Profile::Rectangle { width: 0.05, depth: 0.1 }, border_mullion: Profile::Rectangle { width: 0.05, depth: 0.1 }, panel: crate::CurtainPanel::Glass, panel_material: "m".into(), mullion_material: "m".into() });
    snapshot.curtain_walls.insert("cw".into(), crate::CurtainWall { phase: crate::Phase::New, storey: "st-0".into(), curtain_wall_type: "cwt".into(), axis: line((4.0, 3.0), (0.0, 3.0)), base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, u_grid: None, v_grid: None, name: "Curtain".into() });
    let found = rooms(&snapshot)["sp"].clone();
    assert_eq!(found.status, SpaceStatus::Inferred);
    assert!(near(found.area, 3.8 * (3.0 - 0.1 - 0.05), 1e-9), "{}", found.area);
    assert!(found.bounding_walls.contains(&"cw".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_model_inference_carries_the_rooms() {
    let snapshot = room(4.0, 3.0);
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred.spaces, rooms(&snapshot));
    assert_eq!(inferred, crate::ModelInference::infer(&snapshot).expect("infers"), "determinism");
    assert!(compute_spaces(&ModelSnapshot::default()).is_empty(), "default");
}

const ROOMS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🛋️spaces/🏡️rooms/📸️snapshot/🔣️.json");
const ROOMS_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🛋️spaces/🏡️rooms/💡️inference/🏠️spaces/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_third_party_oracle_table() {
    let snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(ROOMS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the rooms model decodes");
    let problems = crate::standards::v1::subsets::any::schema::inferences::storey_levels::table_problems(ROOMS_TABLE, &table_json(&compute_spaces(&snapshot)));
    assert!(problems.is_empty(), "{} disagreements with the shapely oracle: {:?}", problems.len(), &problems[..problems.len().min(12)]);
}

#[semio_framework_async_macros::async_test]
async fn the_dependency_covers_what_the_rooms_read_besides_their_parents() {
    let before = room(4.0, 3.0);
    let key = ModelNode::Room("st-0".into());
    let read = |snapshot: &ModelSnapshot| compute::dependency(snapshot, &key);
    let mut moved = before.clone();
    moved.walls.get_mut("w-east").expect("wall").axis = line((5.0, 0.0), (5.0, 3.0));
    assert_eq!(read(&before), read(&moved), "a wall move enters through the WallLayout parent, not through the dependency");
    let steps = plan::build(&before, kinds::closure(kinds::ROOMS));
    let parents = &steps.iter().find(|step| step.key == key).expect("planned").parents;
    assert!(parents.contains(&ModelNode::WallLayout("w-east".into())) && parents.contains(&ModelNode::Storey("st-0".into())));
    let mut seed = before.clone();
    seed.spaces.get_mut("sp").expect("space").boundary = SpaceBoundary::Bounded { seed: point(1.0, 1.0) };
    assert_ne!(read(&before), read(&seed), "the boundary of a space is read");
    let mut renamed = before.clone();
    renamed.materials.get_mut("m").expect("material").density = 2400.0;
    renamed.storeys.get_mut("st-0").expect("storey").name = "Renamed".into();
    assert_eq!(read(&before), read(&renamed), "a density or a storey rename must not invalidate the rooms");
    let mut other = before.clone();
    other.walls.insert("w-up".into(), wall("st-1", line((0.0, 0.0), (1.0, 0.0))));
    assert_eq!(read(&before), read(&other), "walls of another storey are not read");
}

#[semio_framework_async_macros::async_test]
async fn a_diff_touching_the_walls_re_infers_the_rooms_and_a_material_diff_does_not() {
    let snapshot = room(4.0, 3.0);
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).spaces.clone();
    let paint = ModelDiff::materials("m", Entry::Patched(crate::MaterialPatch { density: Some(2000.0), ..Default::default() }));
    let untouched = session.update(&snapshot, &paint).spaces.clone();
    assert_eq!(first, untouched);
    assert_eq!(session.report().computed_by_kind.get("room"), None, "a density edit recomputes no room");
    let moved = ModelDiff::walls("w-east", Entry::Patched(WallPatch { axis: Some(line((5.0, 0.0), (5.0, 3.0))), ..Default::default() }));
    let edited = protocol::apply_diff(&moved, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &moved).spaces.clone();
    assert!(near(first["sp"].area, 3.8 * 2.8, 1e-9));
    assert!(recomputed["sp"].area != first["sp"].area, "the moved east wall opens or resizes the room: {}", recomputed["sp"].area);
    assert_eq!(session.report().computed_by_kind.get("room"), Some(&1));
}

#[semio_framework_async_macros::async_test]
async fn the_rooms_are_deterministic_and_empty_by_default() {
    let snapshot = room(4.0, 3.0);
    assert_eq!(compute_spaces(&snapshot), compute_spaces(&snapshot));
    assert!(compute_spaces(&ModelSnapshot::default()).is_empty());
}

fn hung(snapshot: &mut ModelSnapshot, offset: f64, thickness: f64) {
    snapshot.ceiling_types.insert("ct".into(), crate::CeilingType { name: "Board".into(), layers: vec![Layer { material: "m".into(), thickness, function: LayerFunction::Finish }] });
    let boundary = vec![corner(-1.0, -1.0), corner(5.0, -1.0), corner(5.0, 4.0), corner(-1.0, 4.0)];
    snapshot.ceilings.insert("ce".into(), crate::Ceiling { storey: "st-0".into(), ceiling_type: "ct".into(), boundary, holes: Vec::new(), offset, slope: None, name: "Hung".into() });
}

#[semio_framework_async_macros::async_test]
async fn a_hung_ceiling_only_lowers_the_clear_height_and_its_hole_lets_the_room_open_up() {
    let mut snapshot = room(4.0, 3.0);
    hung(&mut snapshot, 0.3, 0.0625);
    let found = rooms(&snapshot)["sp"].clone();
    assert_eq!(found.ceiling, "ce");
    assert!(close(found.clear_height, 3.0 - 0.3 - 0.0625) && near(found.volume, found.area * found.clear_height, 1e-9), "{}", found.clear_height);
    snapshot.ceilings.get_mut("ce").expect("ceiling").holes = vec![vec![corner(1.0, 0.5), corner(3.0, 0.5), corner(3.0, 2.5), corner(1.0, 2.5)]];
    let found = rooms(&snapshot)["sp"].clone();
    assert!(found.ceiling.is_empty() && close(found.clear_height, 3.0), "the seed (2, 1.5) lies in the hole, so no ceiling hangs there");
    hung(&mut snapshot, -0.5, 0.0625);
    snapshot.ceilings.get_mut("ce").expect("ceiling").holes.clear();
    assert!(close(rooms(&snapshot)["sp"].clone().clear_height, 3.0), "a ceiling hung above the storey top never raises the room");
}

#[semio_framework_async_macros::async_test]
async fn editing_a_ceiling_changes_the_clear_height_through_the_session_and_a_rename_recomputes_no_room() {
    let mut snapshot = room(4.0, 3.0);
    hung(&mut snapshot, 0.3, 0.0625);
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).spaces.clone();
    assert!(close(first["sp"].clear_height, 3.0 - 0.3 - 0.0625));
    let rename = ModelDiff::ceilings("ce", Entry::Patched(crate::CeilingPatch { name: Some("Renamed".into()), ..Default::default() }));
    let renamed = protocol::apply_diff(&rename, &snapshot).expect("applies");
    let same = session.update(&renamed, &rename).spaces.clone();
    assert_eq!(same, first);
    assert_eq!(session.report().computed_by_kind.get("room"), None, "a name is not read by the rooms: {:?}", session.report());
    let drop = ModelDiff::ceilings("ce", Entry::Patched(crate::CeilingPatch { offset: Some(0.6), ..Default::default() }));
    let edited = protocol::apply_diff(&drop, &renamed).expect("applies");
    let lowered = session.update(&edited, &drop).clone();
    assert!(!session.report().gated && session.report().computed_by_kind.get("room") == Some(&1), "{:?}", session.report());
    assert!(close(lowered.spaces["sp"].clear_height, 3.0 - 0.6 - 0.0625) && lowered.spaces["sp"].clear_height < first["sp"].clear_height);
    assert_eq!(lowered, crate::ModelInference::infer(&edited).expect("infers"), "the incremental result equals a fresh inference");
    let retyped = ModelDiff::ceiling_types("ct", Entry::Patched(crate::CeilingTypePatch { layers: Some(vec![Layer { material: "m".into(), thickness: 0.2, function: LayerFunction::Finish }]), ..Default::default() }));
    let thick = protocol::apply_diff(&retyped, &edited).expect("applies");
    let deeper = session.update(&thick, &retyped).spaces.clone();
    assert!(close(deeper["sp"].clear_height, 3.0 - 0.6 - 0.2), "the layers of the type are read as well");
}

const HUNG_EDIT: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🛋️spaces/🔲️hung-edit/📸️snapshot/🔣️.json");
const HUNG_EDIT_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🛋️spaces/🔲️hung-edit/💡️inference/🏠️spaces/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_oracle_table_of_the_rooms_after_the_ceiling_edits() {
    let snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(HUNG_EDIT, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the edited rooms model decodes");
    let found = compute_spaces(&snapshot);
    let problems = crate::standards::v1::subsets::any::schema::inferences::storey_levels::table_problems(HUNG_EDIT_TABLE, &table_json(&found));
    assert!(problems.is_empty(), "{} disagreements with the shapely oracle: {:?}", problems.len(), &problems[..problems.len().min(12)]);
    assert_eq!((found["sp-living"].ceiling.as_str(), found["sp-kitchen"].ceiling.as_str()), ("ce-bulkhead", ""), "the lowest ceiling over the seed governs, a hole over the seed removes the kitchen ceiling");
    assert!(close(found["sp-living"].clear_height, 3.0 - 1.25) && close(found["sp-kitchen"].clear_height, 3.0));
}
