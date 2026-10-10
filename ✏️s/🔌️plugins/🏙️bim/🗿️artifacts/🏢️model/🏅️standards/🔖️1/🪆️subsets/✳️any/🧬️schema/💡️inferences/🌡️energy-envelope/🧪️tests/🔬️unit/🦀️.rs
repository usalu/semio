//! 🧪️ The energy envelope of a closed room, a two-room house and a basement, and the aggregates, against closed-form values (ISO 6946 resistances, areas by orientation, the GEG coefficient).

use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, ModelInferenceSession};
use crate::{Axis, Building, Entry, Layer, LayerFunction, LocationLine, Material, MaterialCategory, ModelDiff, Opening, Phase, Point2, Rgb, Site, Slab, SlabType, Space, SpaceBoundary, SpaceConditionsPatch, Storey, TopConstraint, Wall, WallType, WindowType, Zone};
use crate::{Assigned, ModelInference};
use protocol::Inference;

fn near(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-6 * right.abs().max(1.0)
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

fn layer(material: &str, thickness: f64) -> Layer {
    Layer { material: material.into(), thickness, function: LayerFunction::Structure }
}

fn material(name: &str, conductivity: f64) -> Material {
    Material { name: name.into(), category: MaterialCategory::Masonry, color: Rgb { r: 0.7, g: 0.3, b: 0.2 }, density: 1800.0, conductivity, specific_heat: 900.0 }
}

fn space(storey: &str, seed: (f64, f64)) -> Space {
    Space { phase: Phase::New, storey: storey.into(), number: "1".into(), name: "Room".into(), boundary: SpaceBoundary::Bounded { seed: point(seed.0, seed.1) }, usage: "office".into(), zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None }
}

fn heated(setpoint: Option<f64>) -> SpaceConditions {
    SpaceConditions { heating_setpoint: setpoint, cooling_setpoint: setpoint.map(|value| value + 6.0), ..SpaceConditions::empty() }
}

fn base() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.materials.insert("m".into(), material("Brick", 0.8));
    snapshot.materials.insert("ins".into(), material("Insulation", 0.04));
    snapshot.wall_types.insert("wt".into(), WallType { name: "Insulated brick".into(), layers: vec![layer("m", 0.2), layer("ins", 0.1)] });
    snapshot.slab_types.insert("st".into(), SlabType { name: "Slab 250".into(), layers: vec![layer("m", 0.25)] });
    snapshot.window_types.insert("win".into(), WindowType { name: "Window".into(), width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.05, frame_depth: 0.08, panes: 2, material: "m".into(), u_value: Some(1.0), g_value: Some(0.5), frame_fraction: Some(0.2) });
    snapshot.sites.insert("site".into(), Site { name: "Site".into(), latitude: 0.0, longitude: 0.0, elevation: 0.0, true_north: 0.0, boundary: Vec::new() });
    snapshot.buildings.insert("bldg".into(), Building { site: "site".into(), name: "Building".into(), origin: point(0.0, 0.0), rotation: 0.0, elevation: 0.0 });
    snapshot.storeys.insert("st-0".into(), Storey { building: "bldg".into(), name: "Ground".into(), level: 0, height: 3.0, cut_height: None });
    snapshot
}

fn rectangle(snapshot: &mut ModelSnapshot, storey: &str, width: f64, depth: f64) {
    let corners = [(0.0, 0.0), (width, 0.0), (width, depth), (0.0, depth)];
    for (index, name) in ["south", "east", "north", "west"].into_iter().enumerate() {
        snapshot.walls.insert(format!("w-{name}"), wall(storey, line(corners[index], corners[(index + 1) % 4])));
    }
    snapshot.slabs.insert("sl".into(), Slab { storey: storey.into(), slab_type: "st".into(), boundary: vec![corner(0.0, 0.0), corner(width, 0.0), corner(width, depth), corner(0.0, depth)], holes: Vec::new(), offset: 0.0, slope: None, phase: Phase::New, name: "Slab".into() });
}

fn window(snapshot: &mut ModelSnapshot, host: &str, offset: f64) {
    snapshot.openings.insert("o-win".into(), Opening { host: host.into(), kind: crate::OpeningKind::Window { window_type: "win".into() }, offset, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, reveal_depth: None, reveal_material: None, name: "Window".into() });
}

fn room() -> ModelSnapshot {
    let mut snapshot = base();
    rectangle(&mut snapshot, "st-0", 4.0, 3.0);
    window(&mut snapshot, "w-south", 2.0);
    snapshot.spaces.insert("sp".into(), space("st-0", (2.0, 1.5)));
    snapshot.space_conditions.insert("sp".into(), heated(Some(20.0)));
    snapshot
}

fn two_rooms(other: Option<f64>) -> ModelSnapshot {
    let mut snapshot = base();
    rectangle(&mut snapshot, "st-0", 8.0, 3.0);
    snapshot.walls.insert("w-part".into(), wall("st-0", line((4.0, 0.0), (4.0, 3.0))));
    snapshot.spaces.insert("sp-a".into(), space("st-0", (2.0, 1.5)));
    snapshot.spaces.insert("sp-b".into(), space("st-0", (6.0, 1.5)));
    snapshot.space_conditions.insert("sp-a".into(), heated(Some(20.0)));
    snapshot.space_conditions.insert("sp-b".into(), heated(other));
    snapshot
}

fn infer(snapshot: &ModelSnapshot) -> ModelInference {
    ModelInference::infer(snapshot).expect("an inference")
}

fn wall_u() -> f64 {
    1.0 / (0.13 + 0.2 / 0.8 + 0.1 / 0.04 + 0.04)
}

fn floor_u() -> f64 {
    1.0 / (0.17 + 0.25 / 0.8)
}

fn surfaces<'a>(envelope: &'a EnvelopeSpace, kind: SurfaceKind) -> Vec<&'a EnvelopeSurface> {
    envelope.surfaces.iter().filter(|surface| surface.kind == kind).collect()
}

#[test]
fn a_model_without_conditions_has_no_envelope() {
    let mut snapshot = room();
    snapshot.space_conditions.clear();
    let inference = infer(&snapshot);
    assert!(inference.energy_envelopes.is_empty() && inference.energy_totals.is_empty());
}

#[test]
fn a_closed_room_has_four_walls_a_window_a_floor_and_a_ceiling() {
    let inference = infer(&room());
    let envelope = &inference.energy_envelopes["sp"];
    assert!(envelope.conditioned && envelope.heated && envelope.open_length < 1e-6, "{envelope:?}");
    assert_eq!(envelope.surfaces.len(), 7, "{:?}", envelope.surfaces.iter().map(|surface| (&surface.id, surface.kind)).collect::<Vec<_>>());
    assert!(near(envelope.floor_area, 3.7 * 2.7) && near(envelope.height, 3.0) && near(envelope.volume, 3.7 * 2.7 * 3.0));
    let walls = surfaces(envelope, SurfaceKind::Wall);
    assert_eq!(walls.len(), 4);
    assert!(walls.iter().all(|surface| surface.boundary == Boundary::Exterior && near(surface.tilt, 90.0)));
    assert!(walls.iter().all(|surface| near(surface.u_value.expect("a wall U-value"), wall_u())));
}

#[test]
fn walls_face_the_compass_by_their_outward_normal() {
    let envelope = infer(&room()).energy_envelopes["sp"].clone();
    let azimuth = |element: &str| surfaces(&envelope, SurfaceKind::Wall).into_iter().find(|surface| surface.element == element).map(|surface| surface.azimuth).expect("a wall");
    assert!(near(azimuth("w-north"), 0.0) || near(azimuth("w-north"), 360.0));
    assert!(near(azimuth("w-east"), 90.0) && near(azimuth("w-south"), 180.0) && near(azimuth("w-west"), 270.0));
}

#[test]
fn the_building_rotation_and_the_true_north_turn_the_azimuths() {
    let mut snapshot = room();
    snapshot.buildings.get_mut("bldg").expect("building").rotation = std::f64::consts::FRAC_PI_2;
    let turned = infer(&snapshot).energy_envelopes["sp"].clone();
    let south = surfaces(&turned, SurfaceKind::Wall).into_iter().find(|surface| surface.element == "w-south").map(|surface| surface.azimuth).expect("a wall");
    assert!(near(south, 270.0), "the south wall of a building turned a quarter counter-clockwise faces west: {south}");
    snapshot.buildings.get_mut("bldg").expect("building").rotation = 0.0;
    snapshot.sites.get_mut("site").expect("site").true_north = std::f64::consts::FRAC_PI_2;
    let north = infer(&snapshot).energy_envelopes["sp"].clone();
    let south = surfaces(&north, SurfaceKind::Wall).into_iter().find(|surface| surface.element == "w-south").map(|surface| surface.azimuth).expect("a wall");
    assert!(near(south, 270.0), "{south}");
}

#[test]
fn a_window_takes_its_area_out_of_the_wall_and_carries_its_type_data() {
    let envelope = infer(&room()).energy_envelopes["sp"].clone();
    let windows = surfaces(&envelope, SurfaceKind::Window);
    assert_eq!(windows.len(), 1);
    let window = windows[0];
    assert!(near(window.area, 1.44) && near(window.azimuth, 180.0) && window.u_value == Some(1.0) && window.g_value == Some(0.5) && near(window.frame_fraction, 0.2));
    assert_eq!(window.parent, surfaces(&envelope, SurfaceKind::Wall).into_iter().find(|surface| surface.element == "w-south").expect("the host wall").id);
    let south = surfaces(&envelope, SurfaceKind::Wall).into_iter().find(|surface| surface.element == "w-south").expect("a wall");
    assert!(near(south.gross_area, 3.7 * 3.0) && near(south.area, 3.7 * 3.0 - 1.44), "{} {}", south.gross_area, south.area);
}

#[test]
fn the_floor_lies_on_the_ground_and_the_ceiling_without_a_roof_has_no_data() {
    let envelope = infer(&room()).energy_envelopes["sp"].clone();
    let floors = surfaces(&envelope, SurfaceKind::Floor);
    assert_eq!(floors.len(), 1);
    assert!(floors[0].boundary == Boundary::Ground && near(floors[0].area, 3.7 * 2.7) && near(floors[0].tilt, 180.0) && near(floors[0].u_value.expect("a floor U-value"), floor_u()));
    let ceilings = surfaces(&envelope, SurfaceKind::Ceiling);
    assert_eq!(ceilings.len(), 1);
    assert!(ceilings[0].boundary == Boundary::Exterior && ceilings[0].u_value.is_none() && near(ceilings[0].tilt, 0.0));
    assert!(envelope.issues.iter().any(|issue| issue.code == EnergyCode::ThermalDataMissing && issue.detail == "ceiling"));
}

#[test]
fn the_totals_follow_the_closed_form() {
    let inference = infer(&room());
    let totals = &inference.energy_totals["project"];
    let walls = 3.7 * 3.0 * 2.0 + 2.7 * 3.0 * 2.0 - 1.44;
    let area = walls + 1.44 + 3.7 * 2.7 + 3.7 * 2.7;
    assert!(near(totals.envelope_area, area), "{} {area}", totals.envelope_area);
    let loss = wall_u() * walls + 1.0 * 1.44 + GROUND_FACTOR * floor_u() * 3.7 * 2.7;
    assert!(near(totals.transmission, loss), "{} {loss}", totals.transmission);
    assert!(near(totals.h_t_prime, (loss + BRIDGE_ALLOWANCE * area) / area));
    assert!(near(totals.a_over_v, area / (3.7 * 2.7 * 3.0)) && totals.missing == 1);
    assert!(near(totals.window_by_sector[4], 1.44) && near(totals.glazing_area, 1.44) && near(totals.solar_aperture, 0.5 * 0.8 * 1.44));
    assert!(near(totals.opaque_by_sector[0], 3.7 * 3.0) && near(totals.opaque_by_sector[2], 2.7 * 3.0) && near(totals.opaque_by_sector[6], 2.7 * 3.0));
    assert!(near(totals.opaque_by_sector[4], 3.7 * 3.0 - 1.44));
    assert!(near(totals.roof_area, 3.7 * 2.7) && near(totals.floor_envelope_area, 3.7 * 2.7));
    assert_eq!(totals.spaces, ["sp"]);
}

#[test]
fn equal_set_points_make_a_partition_adiabatic_and_different_ones_a_neighbour() {
    let same = infer(&two_rooms(Some(20.0)));
    let wall_of = |inference: &ModelInference, space: &str| surfaces(&inference.energy_envelopes[space], SurfaceKind::Wall).into_iter().find(|surface| surface.element == "w-part").map(|surface| (surface.boundary, surface.adjacent.clone())).expect("the partition");
    assert_eq!(wall_of(&same, "sp-a"), (Boundary::Adiabatic, "sp-b".to_string()));
    assert_eq!(wall_of(&same, "sp-b"), (Boundary::Adiabatic, "sp-a".to_string()));
    let unheated = infer(&two_rooms(None));
    assert_eq!(wall_of(&unheated, "sp-a"), (Boundary::Adjacent, "sp-b".to_string()));
    let building = &unheated.energy_totals["building:bldg"];
    let all = &same.energy_totals["building:bldg"];
    let total = |inference: &ModelInference, space: &str| inference.energy_envelopes[space].surfaces.iter().map(|surface| surface.area).sum::<f64>();
    assert!(near(building.envelope_area, total(&unheated, "sp-a")), "the partition to an unheated room joins the envelope");
    let partition = 3.0 * 2.7;
    assert!(near(all.envelope_area, total(&same, "sp-a") + total(&same, "sp-b") - 2.0 * partition), "the adiabatic partition is no envelope");
    assert_eq!(building.spaces, ["sp-a"]);
    assert_eq!(all.spaces, ["sp-a", "sp-b"]);
}

#[test]
fn a_zone_counts_the_partition_to_another_zone_with_the_neighbour_factor() {
    let mut snapshot = two_rooms(Some(22.0));
    snapshot.zones.insert("z-a".into(), Zone { name: "A".into(), category: "Thermal".into(), occupancy_density: 0.0 });
    snapshot.zones.insert("z-b".into(), Zone { name: "B".into(), category: "Thermal".into(), occupancy_density: 0.0 });
    snapshot.spaces.get_mut("sp-a").expect("space").zone = Some("z-a".into());
    snapshot.spaces.get_mut("sp-b").expect("space").zone = Some("z-b".into());
    let inference = infer(&snapshot);
    let zone = &inference.energy_totals["zone:z-a"];
    let building = &inference.energy_totals["building:bldg"];
    assert!(zone.envelope_area > 0.0 && zone.spaces == ["sp-a"]);
    assert!(near(building.envelope_area, zone.envelope_area + inference.energy_totals["zone:z-b"].envelope_area - 2.0 * 3.0 * 2.7), "{} {} {}", building.envelope_area, zone.envelope_area, inference.energy_totals["zone:z-b"].envelope_area);
}

#[test]
fn walls_below_the_datum_lie_against_the_ground() {
    let mut snapshot = base();
    snapshot.storeys.insert("st-b".into(), Storey { building: "bldg".into(), name: "Cellar".into(), level: -1, height: 3.0, cut_height: None });
    rectangle(&mut snapshot, "st-b", 4.0, 3.0);
    snapshot.spaces.insert("sp".into(), space("st-b", (2.0, 1.5)));
    snapshot.space_conditions.insert("sp".into(), heated(Some(15.0)));
    let envelope = infer(&snapshot).energy_envelopes["sp"].clone();
    let walls = surfaces(&envelope, SurfaceKind::Wall);
    assert_eq!(walls.len(), 4);
    assert!(walls.iter().all(|surface| surface.boundary == Boundary::Ground && near(surface.u_value.expect("a U-value"), 1.0 / (0.13 + 0.2 / 0.8 + 0.1 / 0.04))));
    assert_eq!(surfaces(&envelope, SurfaceKind::Floor)[0].boundary, Boundary::Ground);
}

#[test]
fn a_layer_without_conductivity_leaves_the_transmittance_open_and_is_reported() {
    let mut snapshot = room();
    snapshot.materials.get_mut("ins").expect("material").conductivity = 0.0;
    let inference = infer(&snapshot);
    let envelope = &inference.energy_envelopes["sp"];
    assert!(surfaces(envelope, SurfaceKind::Wall).iter().all(|surface| surface.u_value.is_none()));
    assert!(envelope.issues.iter().any(|issue| issue.code == EnergyCode::ThermalDataMissing && issue.element == "w-south" && issue.detail == "ins"), "{:?}", envelope.issues);
    assert_eq!(inference.energy_totals["project"].missing, 6);
}

#[test]
fn a_window_type_without_a_u_value_is_reported() {
    let mut snapshot = room();
    snapshot.window_types.get_mut("win").expect("type").u_value = None;
    let envelope = infer(&snapshot).energy_envelopes["sp"].clone();
    assert!(envelope.issues.iter().any(|issue| issue.code == EnergyCode::ThermalDataMissing && issue.element == "o-win"));
}

#[test]
fn spaces_without_conditions_are_reported_once_when_others_state_them() {
    let mut snapshot = two_rooms(Some(20.0));
    snapshot.space_conditions.remove("sp-b");
    let inference = infer(&snapshot);
    let found: Vec<_> = inference.diagnostics.iter().filter(|finding| finding.code == crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode::EnergyConditionsMissing).collect();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].elements, ["sp-b"]);
    assert!(found[0].text("en").is_some() && found[0].text("de").is_some());
}

#[test]
fn the_diagnostics_name_the_surfaces_without_thermal_data_in_both_languages() {
    let inference = infer(&room());
    let found: Vec<_> = inference.diagnostics.iter().filter(|finding| finding.code == crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode::EnergyThermalMissing).collect();
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].text("en").expect("en").contains("no thermal data") && found[0].text("de").expect("de").contains("keine thermischen Daten"));
}

#[test]
fn editing_the_conditions_recomputes_the_envelope_and_the_totals_only() {
    let mut snapshot = room();
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    let before = session.inference().energy_totals["project"].clone();
    let patch = SpaceConditionsPatch { heating_setpoint: Some(Assigned::new(Some(21.0))), ..Default::default() };
    let diff = ModelDiff::space_conditions("sp", Entry::Patched(patch));
    snapshot = protocol::apply_diff(&diff, &snapshot).expect("the diff applies");
    let after = session.update(&snapshot, &diff).clone();
    let report = session.report();
    assert!(!report.gated && report.computed <= 4, "{report:?}");
    assert!(report.computed_by_kind.keys().all(|kind| matches!(*kind, "envelope" | "energy-totals" | "diagnostics")), "{:?}", report.computed_by_kind);
    assert!(near(after.energy_totals["project"].transmission, before.transmission));
    assert_eq!(after, infer(&snapshot));
    assert!(kinds::ENERGY != 0);
}

#[test]
fn the_session_equals_a_cold_inference_for_the_energy_fields() {
    let snapshot = two_rooms(None);
    let mut session = ModelInferenceSession::new();
    let warm = session.refresh(&snapshot).clone();
    let cold = infer(&snapshot);
    assert_eq!(warm.energy_envelopes, cold.energy_envelopes);
    assert_eq!(warm.energy_totals, cold.energy_totals);
    assert!(!cold.energy_envelopes.is_empty());
}
