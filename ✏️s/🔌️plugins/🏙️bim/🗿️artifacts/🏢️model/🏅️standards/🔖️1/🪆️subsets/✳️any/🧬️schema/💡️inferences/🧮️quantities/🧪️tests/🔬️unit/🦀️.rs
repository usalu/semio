use super::*;
use crate::TopConstraint;
use crate::{Axis, Beam, BeamType, Building, Column, ColumnType, DoorLeaves, DoorType, LayerFunction, LocationLine, Material, MaterialCategory, Opening, Phase, Point2, Profile, Railing, Rgb, Site, SlabType, Slope, SpaceBoundary, Stair, StairFlight, Storey, Swing, Wall, WallType, WindowType};
use protocol::Inference;

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

fn near(left: f64, right: f64, tolerance: f64) -> bool {
    (left - right).abs() <= tolerance * right.abs().max(1.0)
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

fn layer(material: &str, thickness: f64, function: LayerFunction) -> Layer {
    Layer { material: material.into(), thickness, function }
}

fn wall(axis: Axis) -> Wall {
    Wall { storey: "st-0".into(), wall_type: "wt".into(), axis, location: LocationLine::Center, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, name: "Wall".into() }
}

fn material(density: f64) -> Material {
    Material { name: "Material".into(), category: MaterialCategory::Masonry, color: Rgb { r: 0.5, g: 0.5, b: 0.5 }, density, conductivity: 1.0, specific_heat: 1000.0 }
}

fn base() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.materials.insert("m-brick".into(), material(1800.0));
    snapshot.materials.insert("m-wool".into(), material(40.0));
    snapshot.materials.insert("m-concrete".into(), material(2400.0));
    snapshot.wall_types.insert("wt".into(), crate::WallType { name: "Brick 300".into(), layers: vec![layer("m-brick", 0.2, LayerFunction::Structure), layer("m-wool", 0.1, LayerFunction::Insulation)] });
    snapshot.slab_types.insert("slt".into(), SlabType { name: "Slab 250".into(), layers: vec![layer("m-concrete", 0.25, LayerFunction::Structure)] });
    snapshot.column_types.insert("ct".into(), ColumnType { name: "Column".into(), profile: Profile::Rectangle { width: 0.3, depth: 0.3 }, material: "m-concrete".into() });
    snapshot.beam_types.insert("bt".into(), BeamType { name: "Beam".into(), profile: Profile::Rectangle { width: 0.2, depth: 0.4 }, material: "m-concrete".into() });
    snapshot.window_types.insert("win".into(), WindowType { name: "Window".into(), width: 1.2, height: 1.0, sill: 0.9, frame_width: 0.06, frame_depth: 0.1, panes: 2, material: "m-wool".into() });
    snapshot.door_types.insert("door".into(), DoorType { name: "Door".into(), width: 0.9, height: 2.1, frame_width: 0.06, frame_depth: 0.1, leaves: DoorLeaves::Single, swing: Swing::Left, material: "m-wool".into() });
    snapshot.sites.insert("site".into(), Site { name: "Site".into(), latitude: 0.0, longitude: 0.0, elevation: 0.0, true_north: 0.0, boundary: Vec::new() });
    snapshot.buildings.insert("bldg".into(), Building { site: "site".into(), name: "Building".into(), origin: point(0.0, 0.0), rotation: 0.0, elevation: 0.0 });
    snapshot.storeys.insert("st-0".into(), Storey { building: "bldg".into(), name: "Ground".into(), level: 0, height: 2.5, cut_height: None });
    snapshot.storeys.insert("st-1".into(), Storey { building: "bldg".into(), name: "First".into(), level: 1, height: 3.0, cut_height: None });
    snapshot
}

fn single_wall() -> ModelSnapshot {
    let mut snapshot = base();
    snapshot.walls.insert("w".into(), wall(line((0.0, 0.0), (5.0, 0.0))));
    snapshot
}

fn with_openings() -> ModelSnapshot {
    let mut snapshot = single_wall();
    snapshot.openings.insert("o-win".into(), Opening { host: "w".into(), kind: OpeningKind::Window { window_type: "win".into() }, offset: 1.5, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: "Window".into() });
    snapshot.openings.insert("o-door".into(), Opening { host: "w".into(), kind: OpeningKind::Door { door_type: "door".into() }, offset: 3.8, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: "Door".into() });
    snapshot
}

fn quantities(snapshot: &ModelSnapshot) -> ModelQuantities {
    crate::ModelInference::infer(snapshot).expect("infers").quantities
}

#[semio_framework_async_macros::async_test]
async fn a_wall_measures_length_faces_openings_volume_layers_and_mass() {
    let found = quantities(&with_openings());
    let wall = &found.elements["w"];
    assert_eq!(wall.kind, QuantityKind::Wall);
    assert!(close(wall.length, 5.0) && close(wall.width, 0.3) && close(wall.height, 2.5));
    assert!(close(wall.gross_side_area, 12.5) && close(wall.opening_area, 1.2 * 1.0 + 0.9 * 2.1) && close(wall.net_side_area, 12.5 - 1.2 - 1.89));
    assert!(close(wall.gross_area, 1.5) && close(wall.gross_volume, 3.75) && close(wall.net_volume, 3.75 - 3.09 * 0.3));
    assert_eq!(wall.layers.len(), 2);
    assert!(close(wall.layers[0].volume, 1.0 * 2.5 - 3.09 * 0.2) && close(wall.layers[1].volume, 0.5 * 2.5 - 3.09 * 0.1));
    assert!(close(wall.layers[0].mass, wall.layers[0].volume * 1800.0) && close(wall.mass, wall.layers[0].mass + wall.layers[1].mass));
    assert!(close(wall.layers.iter().map(|row| row.volume).sum::<f64>(), wall.net_volume));
}

#[semio_framework_async_macros::async_test]
async fn the_wall_volume_agrees_with_the_mesh_of_its_solid() {
    let snapshot = with_openings();
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    let solid = &inferred.element_solids["w"];
    assert!(near(inferred.quantities.elements["w"].net_volume, solid.volume, 1e-6), "quantity {} against mesh {}", inferred.quantities.elements["w"].net_volume, solid.volume);
    let rows = solid_rows(&snapshot, solid);
    assert!(rows.iter().any(|row| row.material == "m-brick") && rows.iter().any(|row| row.material == "m-wool"));
}

#[semio_framework_async_macros::async_test]
async fn joined_walls_share_the_corner_without_double_counting() {
    let mut snapshot = base();
    snapshot.walls.insert("w-a".into(), wall(line((0.0, 0.0), (4.0, 0.0))));
    snapshot.walls.insert("w-b".into(), wall(line((4.0, 0.0), (4.0, 3.0))));
    snapshot.wall_types.get_mut("wt").expect("type").layers = vec![layer("m-brick", 0.2, LayerFunction::Structure)];
    let found = quantities(&snapshot);
    let footprint = found.elements["w-a"].gross_area + found.elements["w-b"].gross_area;
    assert!(close(footprint, 4.1 * 0.2 + 2.9 * 0.2), "mitered L footprint {footprint}");
    assert!(close(found.elements["w-a"].gross_volume + found.elements["w-b"].gross_volume, footprint * 2.5));
}

#[semio_framework_async_macros::async_test]
async fn curved_walls_use_the_arc_length_and_the_layer_areas_of_the_annulus() {
    let mut snapshot = base();
    snapshot.wall_types.get_mut("wt").expect("type").layers = vec![layer("m-brick", 0.2, LayerFunction::Structure), layer("m-wool", 0.1, LayerFunction::Insulation)];
    snapshot.walls.insert("w".into(), wall(Axis::Arc { start: point(2.0, 0.0), end: point(-2.0, 0.0), bulge: 1.0 }));
    let wall = &quantities(&snapshot).elements["w"];
    assert!(close(wall.length, std::f64::consts::PI * 2.0));
    let (outer, inner) = (2.15f64, 1.85f64);
    assert!(near(wall.gross_area, std::f64::consts::PI * (outer * outer - inner * inner) / 2.0, 1e-9));
    let layer_area = |from: f64, to: f64| std::f64::consts::PI * (from * from - to * to) / 2.0;
    assert!(near(wall.layers[0].area, layer_area(2.05, 1.85), 1e-9) && near(wall.layers[1].area, layer_area(2.15, 2.05), 1e-9), "left-to-right layers: {:?}", wall.layers);
}

#[semio_framework_async_macros::async_test]
async fn a_slab_with_a_hole_and_a_slope() {
    let mut snapshot = base();
    snapshot.slabs.insert("sl".into(), Slab { storey: "st-0".into(), slab_type: "slt".into(), boundary: vec![corner(0.0, 0.0), corner(6.0, 0.0), corner(6.0, 4.0), corner(0.0, 4.0)], holes: vec![vec![corner(1.0, 1.0), corner(2.0, 1.0), corner(2.0, 2.0), corner(1.0, 2.0)]], offset: 0.0, slope: Some(Slope { direction: 0.0, angle: 0.1 }), name: "Slab".into() });
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    let slab = &inferred.quantities.elements["sl"];
    assert!(close(slab.gross_area, 24.0) && close(slab.net_area, 23.0) && close(slab.perimeter, 20.0 + 4.0) && close(slab.width, 0.25));
    assert!(close(slab.surface_area, 23.0 / 0.1f64.cos()) && close(slab.net_volume, 23.0 * 0.25) && close(slab.gross_volume, 24.0 * 0.25));
    assert!(close(slab.mass, 23.0 * 0.25 * 2400.0));
    assert!(near(slab.net_volume, inferred.element_solids["sl"].volume, 1e-6), "the planar slab volume equals its mesh volume");
}

#[semio_framework_async_macros::async_test]
async fn columns_beams_stairs_railings_and_spaces() {
    let mut snapshot = single_wall();
    snapshot.columns.insert("c".into(), Column { storey: "st-0".into(), column_type: "ct".into(), position: point(1.0, 1.0), rotation: 0.4, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, name: "Column".into() });
    snapshot.beams.insert("b".into(), Beam { storey: "st-0".into(), beam_type: "bt".into(), start: point(0.0, 3.0), end: point(4.0, 0.0), top_offset: 0.0, name: "Beam".into() });
    snapshot.stairs.insert("s".into(), Stair { storey: "st-0".into(), start: point(1.0, 2.0), direction: 0.0, width: 1.0, flight: StairFlight::Straight, top: TopConstraint::StoreyTop { offset: 0.0 }, max_riser: 0.1875, min_tread: 0.25, stringer: crate::STANDARD_STRINGER, nosing: 0.0, tread_thickness: crate::STANDARD_TREAD_THICKNESS, riser: crate::STANDARD_RISER, landing_depth: 1.0, name: "Stair".into() });
    snapshot.railings.insert("r".into(), Railing { storey: "st-0".into(), path: vec![point(0.0, 0.0), point(3.0, 0.0), point(3.0, 4.0)], height: 1.0, post_spacing: 1.0, profile: crate::standard_rail_profile(), post_profile: crate::standard_post_profile(), baluster: None, infill: crate::STANDARD_INFILL, material: "m-concrete".into(), base_offset: 0.0, name: "Railing".into() });
    snapshot.spaces.insert("sp".into(), crate::Space { storey: "st-0".into(), number: "1".into(), name: "Hall".into(), boundary: SpaceBoundary::Explicit { outline: vec![corner(0.0, 0.0), corner(4.0, 0.0), corner(4.0, 3.0), corner(0.0, 3.0)] }, usage: "hall".into() });
    let found = quantities(&snapshot);
    let (column, beam, stair, railing, space) = (&found.elements["c"], &found.elements["b"], &found.elements["s"], &found.elements["r"], &found.elements["sp"]);
    assert!(close(column.length, 2.5) && close(column.gross_area, 0.09) && close(column.net_volume, 0.225) && close(column.mass, 0.225 * 2400.0) && close(column.perimeter, 1.2));
    assert!(close(beam.length, 5.0) && close(beam.net_volume, 0.4) && close(beam.mass, 0.4 * 2400.0));
    assert_eq!((stair.kind, stair.risers), (QuantityKind::Stair, 14));
    assert!(close(stair.height, 2.5) && stair.net_volume > 0.0 && close(stair.length, 13.0 * stair_tread(&snapshot)));
    assert!(close(railing.length, 7.0) && close(railing.height, 1.0) && railing.net_volume > 0.0);
    assert!(railing.layers.iter().all(|row| row.material == "m-concrete") && close(railing.mass, railing.net_volume * 2400.0));
    assert!(close(space.gross_area, 12.0) && close(space.net_area, 12.0 - 0.09) && close(space.net_volume, 12.0 * 2.5) && close(space.height, 2.5));
}

fn stair_tread(snapshot: &ModelSnapshot) -> f64 {
    crate::ModelInference::infer(snapshot).expect("infers").stair_runs["s"].tread
}

#[semio_framework_async_macros::async_test]
async fn totals_add_up_per_kind_type_material_storey_building_and_project() {
    let mut snapshot = with_openings();
    snapshot.columns.insert("c".into(), Column { storey: "st-0".into(), column_type: "ct".into(), position: point(1.0, 1.0), rotation: 0.0, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, name: "Column".into() });
    snapshot.walls.insert("w-up".into(), Wall { storey: "st-1".into(), ..wall(line((0.0, 0.0), (2.0, 0.0))) });
    let found = quantities(&snapshot);
    let walls: Vec<&ElementQuantity> = found.elements.values().filter(|element| element.kind == QuantityKind::Wall).collect();
    let project = &found.project;
    assert_eq!(project.kinds["wall"].count, 2);
    assert!(close(project.kinds["wall"].length, walls.iter().map(|wall| wall.length).sum::<f64>()) && close(project.kinds["wall"].volume, walls.iter().map(|wall| wall.net_volume).sum::<f64>()));
    assert!(close(project.types["wall:wt"].area, walls.iter().map(|wall| wall.net_side_area).sum::<f64>()));
    assert!(project.types.contains_key("window:win") && project.types.contains_key("door:door") && project.types.contains_key("column:ct"));
    assert_eq!(project.kinds["window"].count, 1);
    assert!(close(project.materials["m-brick"].mass, walls.iter().map(|wall| wall.layers[0].mass).sum::<f64>()));
    assert!(project.materials["m-concrete"].mass > 0.0 && close(project.materials["m-concrete"].volume, 0.09 * 2.5));
    assert!(found.storeys["st-0"].kinds["wall"].count == 1 && found.storeys["st-1"].kinds["wall"].count == 1);
    let storey_volume: f64 = found.storeys.values().flat_map(|totals| totals.kinds.values()).map(|totals| totals.volume).sum();
    let kind_volume: f64 = project.kinds.values().map(|totals| totals.volume).sum();
    assert!(close(storey_volume, kind_volume), "the storeys add up to the project");
    assert_eq!(found.buildings["bldg"], found.project, "one building holds everything");
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_follows_into_the_quantities() {
    let mut snapshot = with_openings();
    let before = quantities(&snapshot);
    snapshot.storeys.get_mut("st-0").expect("storey").height = 3.0;
    let after = quantities(&snapshot);
    let (a, b) = (&before.elements["w"], &after.elements["w"]);
    assert!(close(b.height, 3.0) && close(b.gross_side_area, 15.0) && close(b.opening_area, a.opening_area) && close(b.net_side_area, 15.0 - 3.09));
    assert!(close(b.net_volume - a.net_volume, 0.3 * 5.0 * 0.5), "the wall grows by the extra height of its footprint");
}

#[semio_framework_async_macros::async_test]
async fn a_density_edit_changes_masses_only() {
    let mut snapshot = with_openings();
    let before = quantities(&snapshot);
    snapshot.materials.get_mut("m-brick").expect("material").density = 2000.0;
    let after = quantities(&snapshot);
    let (a, b) = (&before.elements["w"], &after.elements["w"]);
    assert_eq!((a.net_volume, a.layers[0].volume, a.net_side_area), (b.net_volume, b.layers[0].volume, b.net_side_area));
    assert!(close(b.layers[0].mass / a.layers[0].mass, 2000.0 / 1800.0));
}

#[semio_framework_async_macros::async_test]
async fn the_default_and_determinism_laws_hold() {
    assert_eq!(quantities(&ModelSnapshot::default()), ModelQuantities::default());
    let snapshot = with_openings();
    assert_eq!(quantities(&snapshot), quantities(&snapshot));
    assert_eq!(quantities(&snapshot), compute_quantities(&snapshot, &crate::ModelInference::infer(&snapshot).expect("infers")));
}

#[semio_framework_async_macros::async_test]
async fn group_volumes_split_a_solid_by_material() {
    let snapshot = single_wall();
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    let solid = &inferred.element_solids["w"];
    let volumes = group_volumes(solid);
    assert!(close(volumes.iter().sum::<f64>(), solid.volume) && volumes.len() == 2);
    assert!(close(volumes[0] + volumes[1], 3.75));
    assert!(upward_area(solid, 0) > 0.0);
}

const BUILDING: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧮️quantities/🏗️building/📸️snapshot/🔣️.json");
const BUILDING_TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧮️quantities/🏗️building/💡️inference/🧮️quantities/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_third_party_oracle_table() {
    let snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(BUILDING, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the building model decodes");
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    let problems = crate::standards::v1::subsets::any::schema::inferences::storey_levels::table_problems(BUILDING_TABLE, &table_json(&snapshot, &inferred.quantities));
    assert!(problems.is_empty(), "{} disagreements with the shapely oracle: {:?}", problems.len(), &problems[..problems.len().min(12)]);
}

#[semio_framework_async_macros::async_test]
async fn openings_stairs_curtain_walls_and_roofs_agree_with_their_solids() {
    let snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(BUILDING, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the building model decodes");
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    for (id, element) in &inferred.quantities.elements {
        if let Some(solid) = inferred.element_solids.get(id) {
            if matches!(element.kind, QuantityKind::Wall) {
                assert!(near(element.net_volume, solid.volume, 1e-4), "{id}: quantity {} against mesh {}", element.net_volume, solid.volume);
            }
            if matches!(element.kind, QuantityKind::Slab | QuantityKind::Column) {
                assert!(near(element.net_volume, solid.volume, 1e-3), "{id}: quantity {} against mesh {}", element.net_volume, solid.volume);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn an_opening_the_wall_does_not_cut_is_not_subtracted() {
    let mut snapshot = with_openings();
    snapshot.openings.get_mut("o-door").expect("opening").offset = 4.9;
    snapshot.openings.get_mut("o-win").expect("opening").kind = OpeningKind::Window { window_type: "gone".into() };
    let found = quantities(&snapshot);
    assert!(close(found.elements["w"].opening_area, 0.0), "the door reaches beyond the end and the window has no type: the solid cuts neither, so the take-off subtracts neither ({})", found.elements["w"].opening_area);
    assert!(close(found.elements["w"].net_volume, found.elements["w"].gross_volume));
}

#[semio_framework_async_macros::async_test]
async fn two_overlapping_windows_are_cut_by_neither_and_subtracted_by_neither() {
    let mut snapshot = with_openings();
    snapshot.openings.get_mut("o-door").expect("opening").kind = OpeningKind::Window { window_type: "win".into() };
    snapshot.openings.get_mut("o-door").expect("opening").offset = 2.0;
    let inferred = crate::ModelInference::infer(&snapshot).expect("infers");
    assert!(inferred.opening_frames.values().all(|frame| !frame.valid), "both overlap each other");
    assert!(close(inferred.quantities.elements["w"].opening_area, 0.0));
    assert!(close(inferred.element_solids["w"].volume, inferred.quantities.elements["w"].net_volume), "the solid and the take-off agree: no hole");
}

#[semio_framework_async_macros::async_test]
async fn the_take_off_subtracts_exactly_the_holes_the_solid_cuts() {
    let inferred = crate::ModelInference::infer(&with_openings()).expect("infers");
    let wall = &inferred.quantities.elements["w"];
    let cut = inferred.element_solids["w"].volume;
    assert!(close(cut, wall.net_volume), "solid volume {cut} against net volume {}", wall.net_volume);
}

#[semio_framework_async_macros::async_test]
async fn a_window_flush_with_the_wall_end_is_reported_and_not_cut() {
    let mut snapshot = single_wall();
    snapshot.openings.insert("o-win".into(), Opening { host: "w".into(), kind: OpeningKind::Window { window_type: "win".into() }, offset: 0.6, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: String::new() });
    let flush = crate::ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(flush.opening_frames["o-win"].issues, vec![crate::standards::v1::subsets::any::schema::inferences::opening_frames::OpeningIssue::OutsideTrimmedExtent]);
    assert!(close(flush.quantities.elements["w"].opening_area, 0.0) && close(flush.element_solids["w"].volume, flush.quantities.elements["w"].gross_volume));
    assert!(flush.diagnostics.iter().any(|row| row.code == crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode::OpeningOutsideTrimmed));
    snapshot.openings.get_mut("o-win").expect("opening").offset = 0.7;
    let clear = crate::ModelInference::infer(&snapshot).expect("infers");
    assert!(clear.opening_frames["o-win"].valid && close(clear.quantities.elements["w"].opening_area, 1.2));
}
