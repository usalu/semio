use super::fixtures::{case, CASES};
use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelInferenceSession, ModelNode};
use crate::{Entry, MaterialPatch, ModelDiff, StoreyPatch, TopConstraint};
use semio_framework_geometry::placement::ZPlane;

fn square_prism(side: f64, height: f64) -> TriMesh {
    semio_framework_geometry::mesh::extrude(&[Point::new(0.0, 0.0), Point::new(side, 0.0), Point::new(side, side), Point::new(0.0, side)], &[], ZPlane::flat(0.0), ZPlane::flat(height))
}

#[test]
fn a_builder_groups_runs_and_measures_the_union_of_its_parts() {
    let mut builder = SolidBuilder::new(SolidFamily::Column);
    builder.add(parts::BODY, "m-a", 0, &square_prism(1.0, 2.0)).add(parts::BODY, "m-a", 0, &square_prism(0.5, 1.0).translated([5.0, 0.0, 0.0])).add(parts::FRAME, "m-b", 1, &TriMesh::new());
    let solid = builder.build();
    assert_eq!(solid.groups, vec![SolidGroup { part: "body".into(), material: "m-a".into(), layer: 0 }]);
    assert_eq!(solid.face_groups, vec![0; 24]);
    assert!((solid.volume - (2.0 + 0.25)).abs() < 1e-12 && (solid.area - (10.0 + 2.5)).abs() < 1e-12, "{} {}", solid.volume, solid.area);
    assert_eq!((solid.bounds.min, solid.bounds.max), (SolidPoint { x: 0.0, y: 0.0, z: 0.0 }, SolidPoint { x: 5.5, y: 1.0, z: 2.0 }));
    assert_eq!((solid.vertex_count(), solid.triangle_count(), solid.indices.len()), (72, 24, 72));
    assert_eq!(solid.mesh().triangle_count(), 24);
}

#[test]
fn a_solid_converts_into_the_world3d_mesh_arrays_without_re_indexing() {
    let mut builder = SolidBuilder::new(SolidFamily::Wall);
    builder.add(parts::LAYER, "m-a", 0, &square_prism(1.0, 1.0)).add(parts::PANEL, "m-b", 0, &square_prism(1.0, 1.0));
    let solid = builder.build();
    assert_eq!((solid.positions_f32().len(), solid.normals_f32().len()), (solid.positions.len(), solid.positions.len()));
    assert_eq!(solid.face_ids().len(), solid.triangle_count());
    let colours = solid.vertex_colors(|group| if group.part == "layer" { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 0.0, 1.0, 1.0] });
    assert_eq!(colours.len(), solid.vertex_count() * 4);
    assert_eq!(&colours[0..4], &[1.0, 0.0, 0.0, 1.0]);
    assert_eq!(&colours[colours.len() - 4..], &[0.0, 0.0, 1.0, 1.0]);
}

#[test]
fn the_empty_solid_is_the_default_and_is_empty() {
    assert!(ElementSolid::default().is_empty());
    assert!(SolidBuilder::new(SolidFamily::Wall).build().is_empty());
}

#[test]
fn profiles_flatten_to_centred_counter_clockwise_outlines() {
    let rectangle = profile_polygon(&Profile::Rectangle { width: 0.4, depth: 0.2 });
    assert_eq!(profile_extents(&rectangle), (0.4, 0.2));
    let circle = profile_polygon(&Profile::Circle { diameter: 1.0 });
    assert!(circle.iter().all(|p| (p.x.hypot(p.y) - 0.5).abs() < 1e-12) && circle.len() >= 16);
    let area = |ring: &[Point]| (0..ring.len()).map(|i| ring[i].x * ring[(i + 1) % ring.len()].y - ring[(i + 1) % ring.len()].x * ring[i].y).sum::<f64>() / 2.0;
    assert!((area(&profile_polygon(&Profile::IShape { width: 0.2, depth: 0.3, web: 0.01, flange: 0.02 })) - (2.0 * 0.2 * 0.02 + 0.01 * 0.26)).abs() < 1e-12);
    assert_eq!(profile_extents(&[]), (0.0, 0.0));
}

#[test]
fn the_vertical_extent_follows_the_top_constraint() {
    let own = StoreyLevel { elevation: 3.0, top_elevation: 6.0, absolute_elevation: 3.0, absolute_top_elevation: 6.0 };
    let above = StoreyLevel { elevation: 6.0, top_elevation: 9.0, absolute_elevation: 6.0, absolute_top_elevation: 9.0 };
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::storey_levels::vertical_of(0.1, &TopConstraint::Unconnected { height: 2.0 }, &own, None), (3.1, 5.1));
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::storey_levels::vertical_of(0.0, &TopConstraint::StoreyTop { offset: -0.2 }, &own, None), (3.0, 5.8));
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::storey_levels::vertical_of(0.0, &TopConstraint::Storey { storey: "x".into(), offset: 0.5 }, &own, Some(&above)), (3.0, 6.5));
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    for (name, _) in CASES {
        let (snapshot, _) = case(name);
        assert_eq!(compute_element_solids(&snapshot), compute_element_solids(&snapshot), "{name}");
    }
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert!(compute_element_solids(&ModelSnapshot::default()).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn every_element_with_geometry_has_one_placed_solid() {
    let (snapshot, _) = case("straight-openings");
    let solids = compute_element_solids(&snapshot);
    assert_eq!(solids.len(), snapshot.walls.len() + 6, "seven walls plus two doors and four windows; the void and the invalid window are absent");
    for (id, solid) in &solids {
        assert_eq!(solid.storey, "st-1", "{id}");
        assert_eq!(solid.placement, SolidPlacement { x: 0.0, y: 0.0, z: 0.0, rotation: 0.0 }, "{id}");
        assert_eq!(solid.normals.len(), solid.positions.len());
        assert_eq!(solid.face_groups.len(), solid.triangle_count());
        assert!(solid.volume > 0.0 && solid.area > 0.0 && !solid.is_empty(), "{id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_placement_carries_the_building_origin_rotation_and_datum() {
    let (mut snapshot, _) = case("straight-openings");
    snapshot.buildings.get_mut("bldg-1").expect("building").origin = crate::Point2 { x: 20.0, y: -4.0 };
    snapshot.buildings.get_mut("bldg-1").expect("building").rotation = 0.5;
    snapshot.buildings.get_mut("bldg-1").expect("building").elevation = 2.0;
    snapshot.sites.get_mut("site-1").expect("site").elevation = 100.0;
    let solids = compute_element_solids(&snapshot);
    assert_eq!(solids["w-plain"].placement, SolidPlacement { x: 20.0, y: -4.0, z: 102.0, rotation: 0.5 });
    assert_eq!(solids["o-win-1"].placement, solids["w-plain"].placement, "fillers stand in their host's building");
    assert!((solids["w-plain"].volume - 2.5).abs() < 1e-12, "the mesh stays building-local");
}

#[semio_framework_async_macros::async_test]
async fn the_model_inference_carries_the_solids_of_the_house_walls() {
    use protocol::Inference;
    let (snapshot, _) = case("straight-openings");
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred.element_solids, compute_element_solids(&snapshot));
}

fn grid_of_walls(count: usize) -> ModelSnapshot {
    let (mut snapshot, _) = case("straight-openings");
    snapshot.walls.clear();
    snapshot.openings.clear();
    for index in 0..count {
        let (column, row) = ((index % 25) as f64, (index / 25) as f64);
        let wall = crate::Wall {
            storey: "st-1".into(),
            wall_type: if index % 3 == 0 { "wt-300".into() } else { "wt-200".into() },
            axis: crate::Axis::Line { start: crate::Point2 { x: column * 6.0, y: row * 6.0 }, end: crate::Point2 { x: column * 6.0 + 4.0, y: row * 6.0 } },
            location: crate::LocationLine::Center,
            base_offset: 0.0,
            top: TopConstraint::StoreyTop { offset: 0.0 },
            phase: crate::Phase::New,
            name: String::new(),
        };
        snapshot.walls.insert(format!("w-{index:04}"), wall);
        if index % 4 == 0 {
            let opening = crate::Opening { host: format!("w-{index:04}"), kind: crate::OpeningKind::Window { window_type: "wn-1".into() }, offset: 2.0, sill_override: Some(0.9), width: None, height: None, flip_hand: false, flip_facing: false, name: String::new() };
            snapshot.openings.insert(format!("o-{index:04}"), opening);
        }
    }
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn five_hundred_walls_are_tessellated_within_a_generous_bound() {
    let snapshot = grid_of_walls(500);
    let started = std::time::Instant::now();
    let solids = compute_element_solids(&snapshot);
    let elapsed = started.elapsed();
    assert_eq!(solids.len(), 500 + 125);
    let total: f64 = snapshot.walls.keys().map(|id| solids[id].volume).sum();
    assert!(total > 0.0);
    assert!(elapsed < std::time::Duration::from_secs(20), "{elapsed:?} for {} triangles", solids.values().map(ElementSolid::triangle_count).sum::<usize>());
}

/// 🧫️ A case with more triangles than this commits no meshes (a finely tessellated arc would weigh half a megabyte); its measures are checked against the closed forms only.
const MESH_LIMIT: usize = 2000;

const FIXTURES_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/💡️inferences/🧊️element-solids");

fn welded_row(solid: &ElementSolid) -> serde_json::Value {
    let mesh = solid.mesh().welded();
    assert_eq!(mesh.triangle_count(), solid.triangle_count(), "welding must not drop a triangle");
    let round = |value: f64| (value * 1e9).round() / 1e9;
    serde_json::json!({ "positions": mesh.positions.iter().flatten().map(|v| round(*v)).collect::<Vec<f64>>(), "indices": mesh.indices.iter().flatten().collect::<Vec<_>>() })
}

fn blessed_meshes(solids: &BTreeMap<String, ElementSolid>) -> serde_json::Value {
    serde_json::Value::Object(solids.iter().map(|(id, solid)| (id.clone(), welded_row(solid))).collect())
}

fn bless(name: &str, meshes: &serde_json::Value) {
    let path = format!("{FIXTURES_ROOT}/{name}/🔣️.json");
    let text = std::fs::read_to_string(&path).expect("the case file");
    let cut = text.rfind("\n  \"meshes\": ").expect("the case file ends with its meshes");
    std::fs::write(&path, format!("{}\n  \"meshes\": {}\n}}\n", &text[..cut], meshes)).expect("the case file is writable");
}

#[semio_framework_async_macros::async_test]
async fn the_committed_expectations_and_meshes_match_the_inference() {
    for (name, _) in CASES {
        let (snapshot, document) = case(name);
        let solids = compute_element_solids(&snapshot);
        let tolerance = if name == "arc-window" { 1e-4 } else { 1e-9 };
        let close = |want: &serde_json::Value, got: f64| (want.as_f64().expect("a number") - got).abs() <= tolerance * got.abs().max(1.0);
        for (id, row) in document["expected"].as_object().expect("an expected table") {
            let solid = &solids[id];
            assert!(close(&row["volume"], solid.volume), "{name}/{id}: volume {} against {}", row["volume"], solid.volume);
            if let Some(area) = row.get("area") {
                assert!(close(area, solid.area), "{name}/{id}: area {area} against {}", solid.area);
            }
            if let Some(bounds) = row.get("bounds") {
                let (low, high) = ([solid.bounds.min.x, solid.bounds.min.y, solid.bounds.min.z], [solid.bounds.max.x, solid.bounds.max.y, solid.bounds.max.z]);
                for axis in 0..3 {
                    assert!(close(&bounds["min"][axis], low[axis]) && close(&bounds["max"][axis], high[axis]), "{name}/{id}: bounds {bounds} against {low:?} {high:?}");
                }
            }
        }
        let meshes = if solids.values().map(ElementSolid::triangle_count).sum::<usize>() > MESH_LIMIT { serde_json::json!({}) } else { blessed_meshes(&solids) };
        if std::env::var("BIM_BLESS").is_ok() {
            bless(name, &meshes);
        } else {
            assert_eq!(document["meshes"], meshes, "{name}: the committed meshes are stale; run with BIM_BLESS=1 to rewrite them");
        }
    }
}


#[semio_framework_async_macros::async_test]
async fn the_plan_puts_storeys_first_and_gives_elements_their_layouts_frames_and_storeys_as_parents() {
    let (snapshot, _) = case("straight-openings");
    let steps = plan::build(&snapshot, kinds::closure(kinds::SOLIDS));
    let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).expect("planned");
    for step in &steps {
        for parent in &step.parents {
            assert!(position(parent) < position(&step.key), "parents come first");
        }
    }
    let step = |family, id: &str| steps.iter().find(|step| step.key == ModelNode::Solid(SolidKey::of(family, id))).expect("planned");
    let wall = step(SolidFamily::Wall, "w-window");
    assert_eq!(wall.parents[..2], [ModelNode::Storey("st-1".into()), ModelNode::WallLayout("w-window".into())]);
    assert!(wall.parents.contains(&ModelNode::OpeningFrame("o-win-1".into())), "the wall is built from the frames of its hosted openings");
    assert_eq!(step(SolidFamily::Window, "o-win-1").parents, vec![ModelNode::Storey("st-1".into()), ModelNode::OpeningFrame("o-win-1".into())]);
    assert!(steps.iter().all(|step| step.key != ModelNode::Solid(SolidKey::of(SolidFamily::Window, "o-void-1"))), "voids have no filler node");
}

#[semio_framework_async_macros::async_test]
async fn a_wall_constrained_to_another_storey_depends_on_both() {
    let (mut snapshot, _) = case("straight-openings");
    snapshot.storeys.insert("st-2".into(), crate::Storey { building: "bldg-1".into(), name: "Upper".into(), level: 1, height: 3.0, cut_height: None });
    snapshot.walls.get_mut("w-plain").expect("wall").top = TopConstraint::Storey { storey: "st-2".into(), offset: -0.5 };
    let steps = plan::build(&snapshot, kinds::closure(kinds::SOLIDS));
    let parents = steps.iter().find(|step| step.key == ModelNode::WallLayout("w-plain".into())).expect("planned").parents.clone();
    assert_eq!(parents[..2], [ModelNode::Storey("st-1".into()), ModelNode::Storey("st-2".into())]);
    let solids = compute_element_solids(&snapshot);
    assert!((solids["w-plain"].bounds.max.z - 2.5).abs() < 1e-12, "3.0 minus 0.5 above the elevation of the storey above");
}

#[semio_framework_async_macros::async_test]
async fn a_material_edit_leaves_the_solids_alone_and_a_storey_edit_re_infers_them() {
    let (snapshot, _) = case("straight-openings");
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).element_solids.clone();
    let colour = ModelDiff::materials("m-brick", Entry::Patched(MaterialPatch { density: Some(1900.0), ..Default::default() }));
    let untouched = session.update(&snapshot, &colour).element_solids.clone();
    assert_eq!(first, untouched, "a material edit never reaches a solid");
    assert_eq!(session.report().computed_by_kind.get("solid"), None);
    let height = ModelDiff::storeys("st-1", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }));
    let edited = protocol::apply_diff(&height, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &height).element_solids.clone();
    assert_ne!(first, recomputed);
    assert!((recomputed["w-layered"].bounds.max.z - first["w-layered"].bounds.max.z - 0.4).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn a_warm_cache_equals_a_cold_recompute() {
    let (snapshot, _) = case("straight-openings");
    let mut session = ModelInferenceSession::new();
    let cold = session.refresh(&snapshot).element_solids.clone();
    let warm = session.refresh(&snapshot).element_solids.clone();
    assert_eq!(cold, warm);
    assert_eq!(session.report().computed, 0, "the second refresh is all cache hits");
    assert_eq!(cold, compute_element_solids(&snapshot), "and both equal the uncached run");
}

#[semio_framework_async_macros::async_test]
async fn an_opening_edit_re_infers_its_host_and_its_filler_but_a_door_type_edit_leaves_windows_alone() {
    use crate::{DoorTypePatch, OpeningPatch};
    let (snapshot, _) = case("straight-openings");
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).element_solids.clone();
    let moved = ModelDiff::openings("o-win-1", Entry::Patched(OpeningPatch { offset: Some(3.0), ..Default::default() }));
    let edited = protocol::apply_diff(&moved, &snapshot).expect("applies");
    let after = session.update(&edited, &moved).element_solids.clone();
    let same = |id: &str| first[id] == after[id];
    assert!(!same("w-window") && !same("o-win-1"), "the host and the filler of the moved opening change");
    assert!(same("w-door") && same("o-win-2") && same("w-plain"), "everything else is untouched");
    let wider = ModelDiff::door_types("dr-1", Entry::Patched(DoorTypePatch { width: Some(1.0), ..Default::default() }));
    let widened = protocol::apply_diff(&wider, &snapshot).expect("applies");
    let third = ModelInferenceSession::new().update(&widened, &wider).element_solids.clone();
    let same_in = |id: &str| first[id] == third[id];
    assert!(!same_in("o-door-1") && !same_in("w-door"), "a door type edit re-infers its fillers and the walls that host them");
    assert!(same_in("o-win-1") && same_in("w-window"), "windows and their walls are untouched");
}
