//! 🕹️ Handles: viewport grips for the building dimensions, storey count, site altitude, wind zone and terrain category, the roof pitch, parapet and obstruction heights, floor area and
//! self-weight thickness, wind face height and loaded area, bridge span, lane width and lane, crane hoisting speed, silo height and radius, and the fire compartment height, plus one gauge grip ([`gauges`]) for every other numeric field.
//! The drag parameter `s` is the displacement in metres (radians for angular grips) from the anchor along the grip direction; every field value is `base + k * s`.

use super::accidental::{BLAST_HEIGHT, IMPACT_HEIGHT, IMPACT_SPACING};
use super::checks;
use super::envelope::RoofShape;
use super::gravity::{plate, STACK_SIDE};
use super::kit::*;
use super::layout::{deck_width, silo_radius, Layout, Placement, DECK_Z, FOOTWAY, RUNWAY_HEIGHT};
use super::model::{En1991Snapshot, StructureKind};
use super::scene::*;
use super::thermal_fire::{compartment, FIRE_MARGIN, WALL_THICKNESS_PER_B};
use super::silo::{hopper_depth, OUTLET_RADIUS};
use super::wave3;
use super::wave3_rows;

#[path = "🎚️gauges/🦀️.rs"]
pub mod gauges;

/// 🕹️ How a written field value is rounded.
#[derive(Clone, Copy)]
enum Round {
    Free,
    Step(f64),
    Int,
}

struct Grip<'a> {
    owner: &'a EntityRef,
    tag: &'a str,
    kind: HandleKind,
    anchor: Vec3,
    style: GripStyle,
    path: String,
    unit: &'a str,
    base: f64,
    k: f64,
    round: Round,
    range: (f64, f64),
    names: (&'a str, &'a str),
}

impl Grip<'_> {
    fn build(self) -> Handle {
        let (a, b) = ((self.range.0 - self.base) / self.k, (self.range.1 - self.base) / self.k);
        let (lo, hi) = (a.min(b), a.max(b));
        let quantise = match self.round {
            Round::Free => None,
            Round::Step(step) => Some(Quantise::Round { step }),
            Round::Int => Some(Quantise::Int),
        };
        Handle {
            id: format!("{}:{}", self.owner.id, self.tag),
            entity: self.owner.clone(),
            kind: self.kind,
            anchor: self.anchor,
            grip: self.style,
            writes: vec![FieldBinding { path: self.path, map: Affine { base: self.base, k: self.k }, unit: self.unit.to_string(), quantise }],
            snap: SnapRule::NONE,
            limits: (lo < hi).then(|| Interval::new(lo, hi)),
            accessible_name: loc(self.names.0, self.names.1),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn grip<'a>(owner: &'a EntityRef, tag: &'a str, kind: HandleKind, anchor: Vec3, style: GripStyle, path: &str, unit: &'a str, base: f64, k: f64, round: Round, range: (f64, f64), names: (&'a str, &'a str)) -> Handle {
    Grip { owner, tag, kind, anchor, style, path: path.to_string(), unit, base, k, round, range, names }.build()
}

fn building(d: &En1991Snapshot, layout: &Layout) -> Vec<Handle> {
    let owner = entity_ref(EntityKind::Solid, "building", "@site");
    let pose = &layout.building;
    let (depth, width, height) = (d.depth, d.width, d.height);
    vec![
        grip(&owner, "height", HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, pose.at(depth, width, height), GripStyle::Arrow, "height", "m", height, 1.0, Round::Step(0.1), (2.0, 150.0), ("Building height", "Gebäudehöhe")),
        grip(&owner, "width", HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Y) }, pose.at(depth / 2.0, width, 0.0), GripStyle::Arrow, "width", "m", width, 1.0, Round::Step(0.1), (2.0, 150.0), ("Building width b", "Gebäudebreite b")),
        grip(&owner, "depth", HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, pose.at(depth, width / 2.0, 0.0), GripStyle::Arrow, "depth", "m", depth, 1.0, Round::Step(0.1), (2.0, 150.0), ("Building depth d", "Gebäudetiefe d")),
        grip(&owner, "storeys", HandleKind::CountStepper, pose.at(depth, 0.0, height / 2.0), GripStyle::Diamond, "storeyCount", "1", f64::from(d.storey_count), 1.0, Round::Int, (1.0, 40.0), ("Number of storeys", "Anzahl der Geschosse")),
        grip(&owner, "altitude", HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, pose.at(-3.0, -3.0, 0.0), GripStyle::Arrow, "altitude", "m", d.altitude, 25.0, Round::Step(10.0), (0.0, 1500.0), ("Site altitude above sea level", "Geländehöhe über NHN")),
        grip(&owner, "wind-zone", HandleKind::CountStepper, pose.at(-6.0, width / 2.0, height + 3.0), GripStyle::Diamond, "windZone", "1", f64::from(d.wind_zone), 1.0, Round::Int, (1.0, 4.0), ("Wind zone", "Windzone")),
        grip(&owner, "terrain", HandleKind::CountStepper, pose.at(-6.0, width / 2.0 + 3.0, height + 3.0), GripStyle::Diamond, "terrainCategory", "1", f64::from(d.terrain_category), 1.0, Round::Int, (0.0, 4.0), ("Terrain category", "Geländekategorie")),
    ]
}

fn roof(d: &En1991Snapshot, layout: &Layout, index: usize) -> Vec<Handle> {
    let Some(r) = d.roofs.get(index) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, &r.id, "roofs");
    let pose = layout.roof(index);
    let shape = RoofShape::of(d, Some(r));
    let tip = if shape.kind == super::envelope::RoofKind::Mono { [shape.d, shape.b / 2.0] } else { [shape.d / 2.0, shape.b / 2.0] };
    let lifted = shape.lift(tip, 0.0);
    let mut out = vec![Grip {
        owner: &owner,
        tag: "pitch",
        kind: HandleKind::Angular { axis: pose.dir(&Vec3::Y), centre: pose.at(0.0, shape.b / 2.0, shape.h) },
        anchor: pose.at(lifted.x, lifted.y, lifted.z),
        style: GripStyle::Ring,
        path: format!("roofs[id={}].pitchDeg", r.id),
        unit: "deg",
        base: r.pitch_deg,
        k: 180.0 / std::f64::consts::PI,
        round: Round::Step(0.5),
        range: (0.0, 60.0),
        names: ("Roof pitch", "Dachneigung"),
    }
    .build()];
    let spans = [("false".to_string(), "Single span", "Einfeld"), ("true".to_string(), "Multi-span", "Mehrfeld")];
    let valley = shape.lift([shape.d / 2.0, shape.b / 4.0], 0.5);
    out.push(super::scene::projection_kit::handle_kit::choice_handle(&owner, "multi-span", pose.at(valley.x, valley.y, valley.z), format!("roofs[id={}].multiSpan", r.id), &spans, LocalizedCopy::new("Roof span arrangement", "Dachfeldanordnung")));
    if r.has_parapet {
        let corner = shape.lift([0.0, 0.0], r.parapet_height);
        out.push(Grip { owner: &owner, tag: "parapet", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(corner.x, corner.y, corner.z), style: GripStyle::Arrow, path: format!("roofs[id={}].parapetHeight", r.id), unit: "m", base: r.parapet_height, k: 1.0, round: Round::Step(0.05), range: (0.0, 3.0), names: ("Parapet height", "Attikahöhe") }.build());
    }
    if let Some(limit) = shape.planar_limit() {
        let x = 0.3 * limit;
        let base = shape.lift([x, shape.b / 2.0], r.drift_obstruction_height);
        out.push(Grip { owner: &owner, tag: "obstruction", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(base.x, base.y, base.z), style: GripStyle::Arrow, path: format!("roofs[id={}].driftObstructionHeight", r.id), unit: "m", base: r.drift_obstruction_height, k: 1.0, round: Round::Step(0.05), range: (0.0, 5.0), names: ("Drift obstruction height", "Höhe des Dachaufbaus") }.build());
    }
    out
}

fn floor(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, f)) = d.floors.iter().enumerate().find(|(_, f)| f.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "floors");
    let (x0, y0, sx, sy, z) = plate(d, index, f);
    vec![Grip { owner: &owner, tag: "area", kind: HandleKind::ScalarAlong { axis: layout.building.dir(&Vec3::X) }, anchor: layout.building.at(x0 + sx, y0 + sy / 2.0, z), style: GripStyle::Arrow, path: format!("floors[id={id}].area"), unit: "m2", base: f.area, k: sy, round: Round::Step(1.0), range: (1.0, 5000.0), names: ("Floor area", "Geschossfläche") }.build()]
}

fn self_weight(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(index) = d.self_weight_elements.iter().position(|e| e.id == id) else { return Vec::new() };
    let element = &d.self_weight_elements[index];
    let owner = entity_ref(EntityKind::Solid, id, "selfWeightElements");
    let below: f64 = d.self_weight_elements[..index].iter().map(|e| e.thickness.max(0.01)).sum();
    let thickness = element.thickness.max(0.01);
    let pose = &layout.self_weight;
    vec![Grip { owner: &owner, tag: "thickness", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(STACK_SIDE / 2.0, STACK_SIDE / 2.0, below + thickness), style: GripStyle::Arrow, path: format!("selfWeightElements[id={id}].thickness"), unit: "m", base: thickness, k: 1.0, round: Round::Step(0.005), range: (0.01, 2.0), names: ("Layer thickness", "Schichtdicke") }.build()]
}

fn face(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(f) = d.wind_faces.iter().find(|f| f.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "windFaces");
    let pose = &layout.building;
    vec![
        Grip { owner: &owner, tag: "z", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(0.0, d.width / 2.0, f.z), style: GripStyle::Diamond, path: format!("windFaces[id={id}].z"), unit: "m", base: f.z, k: 1.0, round: Round::Step(0.1), range: (0.5, (d.height * 1.5).max(10.0)), names: ("Reference height z_e", "Bezugshöhe z_e") }.build(),
        Grip { owner: &owner, tag: "area", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Y) }, anchor: pose.at(0.0, d.width / 2.0 + 1.0, f.z), style: GripStyle::Arrow, path: format!("windFaces[id={id}].loadedArea"), unit: "m2", base: f.loaded_area, k: 1.0, round: Round::Step(0.1), range: (0.1, 100.0), names: ("Loaded area A", "Lasteinzugsfläche A") }.build(),
    ]
}

fn bridge(d: &En1991Snapshot, layout: &Layout) -> Vec<Handle> {
    if d.structure_kind != StructureKind::Bridge {
        return Vec::new();
    }
    let owner = entity_ref(EntityKind::Solid, "bridge", "@site");
    let pose = &layout.bridge;
    let (span, carriageway, deck) = (d.bridge_span.max(1.0), d.bridge_lane_width.max(0.0), deck_width(d));
    let (bands, ..) = super::layout::bands_of(d);
    let count = u8::try_from(bands.len()).unwrap_or(u8::MAX).max(1);
    let lane = d.bridge_lane.clamp(1, count);
    let lane_y = bands.iter().find(|b| b.number == lane).map_or(FOOTWAY + 1.5, |b| (b.y0 + b.y1) / 2.0);
    vec![
        Grip { owner: &owner, tag: "span", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(span, deck / 2.0, DECK_Z), style: GripStyle::Arrow, path: "bridgeSpan".into(), unit: "m", base: d.bridge_span, k: 1.0, round: Round::Step(0.5), range: (2.0, 200.0), names: ("Bridge span L", "Brückenspannweite L") }.build(),
        Grip { owner: &owner, tag: "tandem-x", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(d.bridge_tandem_x, lane_y, DECK_Z + 0.5), style: GripStyle::Diamond, path: "bridgeTandemX".into(), unit: "m", base: d.bridge_tandem_x, k: 1.0, round: Round::Step(0.25), range: (0.6, (span - 0.6).max(0.6)), names: ("LM1 tandem position x", "LM1-Tandemlage x") }.build(),
        Grip { owner: &owner, tag: "rail-alpha", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Y) }, anchor: pose.at(span / 2.0, FOOTWAY + carriageway / 2.0, DECK_Z + 1.0), style: GripStyle::Diamond, path: "railAlpha".into(), unit: "1", base: d.rail_alpha, k: 0.1, round: Round::Step(0.01), range: (0.75, 1.46), names: ("Rail classification factor α", "Klassifizierungsbeiwert α der Bahnlasten") }.build(),
        Grip { owner: &owner, tag: "rail-lm71", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(span / 2.0, FOOTWAY + carriageway / 2.0, DECK_Z + 2.0), style: GripStyle::Arrow, path: "assumedRailLm71".into(), unit: "N", base: d.assumed_rail_lm71, k: 50_000.0, round: Round::Step(5_000.0), range: (0.0, 800_000.0), names: ("Assumed LM71 axle load", "Angesetzte LM71-Achslast") }.build(),
        Grip { owner: &owner, tag: "lane-width", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Y) }, anchor: pose.at(span / 2.0, FOOTWAY + carriageway, DECK_Z), style: GripStyle::Arrow, path: "bridgeLaneWidth".into(), unit: "m", base: d.bridge_lane_width, k: 1.0, round: Round::Step(0.1), range: (2.5, 30.0), names: ("Carriageway width w", "Fahrbahnbreite w") }.build(),
        Grip { owner: &owner, tag: "lane", kind: HandleKind::CountStepper, anchor: pose.at(span * 0.9, lane_y, DECK_Z), style: GripStyle::Diamond, path: "bridgeLane".into(), unit: "1", base: f64::from(d.bridge_lane), k: 1.0, round: Round::Int, range: (1.0, f64::from(count)), names: ("Selected notional lane", "Gewählter Fahrstreifen") }.build(),
    ]
}

fn lane(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.bridge_lanes.iter().find(|l| l.id == id).filter(|_| d.structure_kind == StructureKind::Bridge) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "bridgeLanes");
    let pose = &layout.bridge;
    let (span, carriageway) = (d.bridge_span.max(1.0), d.bridge_lane_width.max(0.0));
    let centre = FOOTWAY + carriageway / 2.0 + row.offset;
    vec![
        Grip { owner: &owner, tag: "width", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Y) }, anchor: pose.at(span / 2.0, centre + row.width / 2.0, DECK_Z), style: GripStyle::Arrow, path: format!("bridgeLanes[id={id}].width"), unit: "m", base: row.width, k: 2.0, round: Round::Step(0.05), range: (1.0, 4.0), names: ("Lane width", "Fahrstreifenbreite") }.build(),
        Grip { owner: &owner, tag: "offset", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Y) }, anchor: pose.at(span / 4.0, centre, DECK_Z), style: GripStyle::Diamond, path: format!("bridgeLanes[id={id}].offset"), unit: "m", base: row.offset, k: 1.0, round: Round::Step(0.1), range: (-carriageway / 2.0, carriageway / 2.0), names: ("Lane offset", "Fahrstreifenversatz") }.build(),
        Grip { owner: &owner, tag: "number", kind: HandleKind::CountStepper, anchor: pose.at(span * 0.9, centre, DECK_Z), style: GripStyle::Diamond, path: format!("bridgeLanes[id={id}].lm1Lane"), unit: "1", base: f64::from(row.lm1_lane), k: 1.0, round: Round::Int, range: (1.0, 4.0), names: ("LM1 lane number", "LM1-Fahrstreifennummer") }.build(),
    ]
}

fn stage(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(index) = d.construction_stages.iter().position(|s| s.id == id) else { return Vec::new() };
    let row = &d.construction_stages[index];
    let owner = entity_ref(EntityKind::Load, id, "constructionStages");
    let pose = &layout.building;
    let (x0, x1) = super::collections::stage_slots(d)[index];
    let y = super::collections::stage_row_y(d) + 1.0;
    vec![
        Grip { owner: &owner, tag: "duration", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(x1, y, 0.0), style: GripStyle::Arrow, path: format!("constructionStages[id={id}].durationDays"), unit: "d", base: row.duration_days, k: super::collections::days_per_metre(d), round: Round::Step(1.0), range: (1.0, 3650.0), names: ("Stage duration", "Dauer des Bauzustands") }.build(),
        Grip { owner: &owner, tag: "load", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at((x0 + x1) / 2.0, y, 1.0), style: GripStyle::Diamond, path: format!("constructionStages[id={id}].assumedQk"), unit: "Pa", base: row.assumed_qk, k: 1_000.0, round: Round::Step(50.0), range: (0.0, 20_000.0), names: ("Assumed construction load", "Angenommene Bauzustandslast") }.build(),
    ]
}

fn surface(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(index) = d.roof_surfaces.iter().position(|s| s.id == id) else { return Vec::new() };
    let row = &d.roof_surfaces[index];
    let owner = entity_ref(EntityKind::Surface, id, "roofSurfaces");
    let pose = &layout.building;
    let (x0, x1) = super::collections::surface_span(d, index);
    let z = |x: f64| super::collections::surface_z(d, index, x);
    vec![
        Grip { owner: &owner, tag: "pitch", kind: HandleKind::Angular { axis: pose.dir(&Vec3::Y), centre: pose.at(x0, d.width / 2.0, z(x0)) }, anchor: pose.at(x1, d.width / 2.0, z(x1)), style: GripStyle::Ring, path: format!("roofSurfaces[id={id}].pitchDeg"), unit: "deg", base: row.pitch_deg, k: 180.0 / std::f64::consts::PI, round: Round::Step(0.5), range: (0.0, 60.0), names: ("Roof surface pitch", "Neigung der Dachfläche") }.build(),
        Grip { owner: &owner, tag: "extent", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(x1, d.width / 2.0, z(x1)), style: GripStyle::Arrow, path: format!("roofSurfaces[id={id}].extent"), unit: "m", base: row.extent, k: 1.0, round: Round::Step(0.1), range: (0.5, d.depth.max(1.0)), names: ("Roof surface extent", "Ausdehnung der Dachfläche") }.build(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn axis_grip<'a>(owner: &'a EntityRef, tag: &'a str, axis: Vec3, anchor: Vec3, style: GripStyle, path: String, unit: &'a str, base: f64, k: f64, step: f64, range: (f64, f64), names: (&'a str, &'a str)) -> Handle {
    Grip { owner, tag, kind: HandleKind::ScalarAlong { axis }, anchor, style, path, unit, base, k, round: Round::Step(step), range, names }.build()
}

fn vehicle(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.special_vehicles.iter().find(|v| v.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "specialVehicles");
    let pose = &layout.bridge;
    let (length, _, _) = wave3::vehicle_footprint(&row.designation).unwrap_or((0.0, 0.0, 0));
    let span = d.bridge_span.max(1.0);
    vec![
        axis_grip(&owner, "x", pose.dir(&Vec3::X), pose.at(row.x, FOOTWAY + row.y, DECK_Z + 0.5), GripStyle::Diamond, format!("specialVehicles[id={id}].x"), "m", row.x, 1.0, 0.25, (length / 2.0, (span - length / 2.0).max(length / 2.0 + 0.5)), ("Vehicle position along the span", "Fahrzeugposition entlang der Spannweite")),
        axis_grip(&owner, "y", pose.dir(&Vec3::Y), pose.at(row.x, FOOTWAY + row.y, DECK_Z + 1.0), GripStyle::Diamond, format!("specialVehicles[id={id}].y"), "m", row.y, 1.0, 0.1, (0.0, d.bridge_lane_width.max(0.2)), ("Vehicle axis across the carriageway", "Fahrzeugachse quer zur Fahrbahn")),
    ]
}

fn bridge_wind(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((row, (z0, z1, _))) = d.wind_bridges.iter().find(|r| r.id == id).and_then(|r| wave3::wind_face(r).map(|f| (r, f))) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "windBridges");
    let pose = &layout.bridge;
    vec![
        axis_grip(&owner, "depth", pose.dir(&Vec3::Z), pose.at(row.span / 2.0, 0.0, z0), GripStyle::Arrow, format!("windBridges[id={id}].deckDepth"), "m", row.deck_depth, -1.0, 0.05, (0.2, 10.0), ("Deck depth d", "Überbauhöhe d")),
        axis_grip(&owner, "width", pose.dir(&Vec3::Y), pose.at(row.span / 2.0, row.deck_width, DECK_Z), GripStyle::Arrow, format!("windBridges[id={id}].deckWidth"), "m", row.deck_width, 1.0, 0.1, (1.0, 60.0), ("Deck width b", "Überbaubreite b")),
        axis_grip(&owner, "height", pose.dir(&Vec3::Z), pose.at(row.span, 0.0, z1), GripStyle::Diamond, format!("windBridges[id={id}].heightAboveGround"), "m", row.height_above_ground, 1.0, 0.5, (1.0, 200.0), ("Height above ground z_e", "Höhe über Grund z_e")),
        axis_grip(&owner, "span", pose.dir(&Vec3::X), pose.at(row.span, row.deck_width / 2.0, DECK_Z), GripStyle::Arrow, format!("windBridges[id={id}].span"), "m", row.span, 1.0, 0.5, (2.0, 300.0), ("Loaded length L", "Belastete Länge L")),
    ]
}

fn dynamic_element(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(index) = d.dynamic_elements.iter().position(|e| e.id == id) else { return Vec::new() };
    let row = &d.dynamic_elements[index];
    let owner = entity_ref(EntityKind::Solid, id, "dynamicElements");
    let pose = &layout.building;
    let (x, y) = wave3::element_origin(d, index);
    vec![
        axis_grip(&owner, "height", pose.dir(&Vec3::Z), pose.at(x, y, row.height), GripStyle::Arrow, format!("dynamicElements[id={id}].height"), "m", row.height, 1.0, 0.5, (2.0, 300.0), ("Element height h", "Bauteilhöhe h")),
        axis_grip(&owner, "width", pose.dir(&Vec3::Y), pose.at(x, y + row.width / 2.0, row.height / 2.0), GripStyle::Arrow, format!("dynamicElements[id={id}].width"), "m", row.width, 2.0, 0.05, (0.1, 60.0), ("Element width b", "Bauteilbreite b")),
        axis_grip(&owner, "mass", pose.dir(&Vec3::Y), pose.at(x, y - 1.0, row.height / 2.0), GripStyle::Diamond, format!("dynamicElements[id={id}].massPerLength"), "kg/m", row.mass_per_length, 100.0, 1.0, (1.0, 1_000_000.0), ("Mass per length m_e", "Masse je Länge m_e")),
        axis_grip(&owner, "decrement", pose.dir(&Vec3::Y), pose.at(x, y, row.height + 1.0), GripStyle::Diamond, format!("dynamicElements[id={id}].logDecrement"), "1", row.log_decrement, 0.01, 0.005, (0.005, 0.5), ("Logarithmic decrement δ_s", "Logarithmisches Dekrement δ_s")),
    ]
}

fn impact(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.impact_actions.iter().find(|r| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "impactActions");
    let pose = &layout.building;
    vec![
        axis_grip(&owner, "x", pose.dir(&Vec3::X), pose.at(row.x, row.y, row.height), GripStyle::Ring, format!("impactActions[id={id}].x"), "m", row.x, 1.0, 0.25, (-200.0, 400.0), ("Impact position x", "Anprallposition x")),
        axis_grip(&owner, "y", pose.dir(&Vec3::Y), pose.at(row.x, row.y + 1.0, row.height), GripStyle::Ring, format!("impactActions[id={id}].y"), "m", row.y, 1.0, 0.25, (-200.0, 400.0), ("Impact position y", "Anprallposition y")),
        axis_grip(&owner, "height", pose.dir(&Vec3::Z), pose.at(row.x, row.y, row.height + 0.5), GripStyle::Arrow, format!("impactActions[id={id}].height"), "m", row.height, 1.0, 0.05, (0.0, 10.0), ("Impact height", "Anprallhöhe")),
        axis_grip(&owner, "force", pose.dir(&Vec3::X), pose.at(row.x - 1.0, row.y, row.height), GripStyle::Diamond, format!("impactActions[id={id}].assumedForce"), "N", row.assumed_force, 100_000.0, 10_000.0, (0.0, 20_000_000.0), ("Assumed impact force", "Angenommene Anprallkraft")),
    ]
}

const SPEED_PER_METRE: f64 = 10.0;
const CHARGE_PER_METRE: f64 = 100.0;

/// 💥 The grips of the nested rows of an accidental case: the vehicle speed of every impact row (the arrow length reads 10 m/s per metre) and the stand-off and charge mass of every explosion row; the rows have no ids, so the paths address them by position.
fn accidental(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.accidental_cases.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "accidentalCases");
    let pose = &layout.building;
    let y = d.width * (index + 1) as f64 / (d.accidental_cases.len() + 1) as f64;
    let mut out = Vec::new();
    for (j, impact) in row.impact.iter().enumerate() {
        let (tag, en, de) = (format!("impact{j}-speed"), format!("Vehicle speed of impact {j} of {id}"), format!("Fahrzeuggeschwindigkeit des Anpralls {j} von {id}"));
        let at = pose.at(-impact.vehicle_speed / SPEED_PER_METRE, y + IMPACT_SPACING * j as f64, IMPACT_HEIGHT);
        out.push(axis_grip(&owner, &tag, pose.dir(&v3(-1.0, 0.0, 0.0)), at, GripStyle::Arrow, format!("accidentalCases[id={id}].impact[{j}].vehicleSpeed"), "m/s", impact.vehicle_speed, SPEED_PER_METRE, 0.5, (0.0, 60.0), (&en, &de)));
    }
    for (j, blast) in row.explosion.iter().enumerate() {
        let lateral = y + IMPACT_SPACING * j as f64;
        let centre = pose.at(-blast.standoff.max(0.0), lateral, BLAST_HEIGHT);
        let (tag, en, de) = (format!("explosion{j}-standoff"), format!("Stand-off of explosion {j} of {id}"), format!("Abstand der Explosion {j} von {id}"));
        out.push(axis_grip(&owner, &tag, pose.dir(&v3(-1.0, 0.0, 0.0)), centre.clone(), GripStyle::Ring, format!("accidentalCases[id={id}].explosion[{j}].standoff"), "m", blast.standoff, 1.0, 0.1, (0.0, 500.0), (&en, &de)));
        let (tag, en, de) = (format!("explosion{j}-mass"), format!("Charge mass of explosion {j} of {id}"), format!("Ladungsmasse der Explosion {j} von {id}"));
        let top = pose.at(-blast.standoff.max(0.0), lateral, BLAST_HEIGHT + blast.explosion_mass / CHARGE_PER_METRE);
        out.push(axis_grip(&owner, &tag, pose.dir(&Vec3::Z), top, GripStyle::Arrow, format!("accidentalCases[id={id}].explosion[{j}].explosionMass"), "kg", blast.explosion_mass, CHARGE_PER_METRE, 1.0, (0.0, 20_000.0), (&en, &de)));
    }
    out
}

fn helicopter(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.helicopter_landings.iter().find(|r| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "helicopterLandings");
    let pose = &layout.building;
    vec![
        axis_grip(&owner, "x", pose.dir(&Vec3::X), pose.at(row.x, row.y, row.height), GripStyle::Ring, format!("helicopterLandings[id={id}].x"), "m", row.x, 1.0, 0.5, (-200.0, 400.0), ("Pad position x", "Landeplatz x")),
        axis_grip(&owner, "y", pose.dir(&Vec3::Y), pose.at(row.x, row.y + 1.0, row.height), GripStyle::Ring, format!("helicopterLandings[id={id}].y"), "m", row.y, 1.0, 0.5, (-200.0, 400.0), ("Pad position y", "Landeplatz y")),
        axis_grip(&owner, "height", pose.dir(&Vec3::Z), pose.at(row.x, row.y, row.height + 1.0), GripStyle::Arrow, format!("helicopterLandings[id={id}].height"), "m", row.height, 1.0, 0.5, (0.0, 300.0), ("Pad height", "Landeplatzhöhe")),
        axis_grip(&owner, "force", pose.dir(&Vec3::Z), pose.at(row.x + 1.0, row.y, row.height + 1.0), GripStyle::Diamond, format!("helicopterLandings[id={id}].assumedForce"), "N", row.assumed_force, 50_000.0, 1_000.0, (0.0, 5_000_000.0), ("Assumed landing force", "Angenommene Landekraft")),
    ]
}

fn tie(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.tie_runs.iter().find(|r| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "tieRuns");
    let pose = &layout.building;
    vec![
        axis_grip(&owner, "x", pose.dir(&Vec3::X), pose.at(row.x, row.y, 3.0), GripStyle::Ring, format!("tieRuns[id={id}].x"), "m", row.x, 1.0, 0.5, (-200.0, 400.0), ("Tie start x", "Zuggliedanfang x")),
        axis_grip(&owner, "y", pose.dir(&Vec3::Y), pose.at(row.x, row.y, 3.0), GripStyle::Ring, format!("tieRuns[id={id}].y"), "m", row.y, 1.0, 0.5, (-200.0, 400.0), ("Tie position y", "Zugglied y")),
        axis_grip(&owner, "span", pose.dir(&Vec3::X), pose.at(row.x + row.span, row.y, 3.0), GripStyle::Arrow, format!("tieRuns[id={id}].span"), "m", row.span, 1.0, 0.5, (0.5, 200.0), ("Tie span L", "Zuggliedlänge L")),
        axis_grip(&owner, "spacing", pose.dir(&Vec3::Y), pose.at(row.x + row.span / 2.0, row.y + row.spacing, 3.0), GripStyle::Diamond, format!("tieRuns[id={id}].spacing"), "m", row.spacing, 1.0, 0.5, (0.5, 50.0), ("Tie spacing s", "Zuggliedabstand s")),
        axis_grip(&owner, "force", pose.dir(&Vec3::Z), pose.at(row.x + row.span, row.y, 4.0), GripStyle::Diamond, format!("tieRuns[id={id}].providedForce"), "N", row.provided_force, 20_000.0, 1_000.0, (0.0, 5_000_000.0), ("Provided tie force", "Vorhandene Zugkraft")),
    ]
}

fn construction_load(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(index) = d.construction_loads.iter().position(|r| r.id == id) else { return Vec::new() };
    let row = &d.construction_loads[index];
    let Some(slot) = d.construction_stages.iter().position(|s| s.id == row.stage) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "constructionLoads");
    let pose = &layout.building;
    let x0 = super::collections::stage_slots(d)[slot].0;
    vec![axis_grip(&owner, "load", pose.dir(&Vec3::Z), pose.at(x0, super::collections::stage_row_y(d), super::layout::stack_level(d, index)), GripStyle::Diamond, format!("constructionLoads[id={id}].magnitude"), "Pa", row.magnitude, 1_000.0, 50.0, (0.0, 50_000.0), ("Construction load Q_c,i", "Bauzustandslast Q_c,i"))]
}

fn falling(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.falling_loads.iter().find(|r| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "fallingLoads");
    let pose = &layout.building;
    vec![
        axis_grip(&owner, "x", pose.dir(&Vec3::X), pose.at(row.x, row.y, 0.0), GripStyle::Ring, format!("fallingLoads[id={id}].x"), "m", row.x, 1.0, 0.5, (-200.0, 400.0), ("Impact position x", "Aufprallposition x")),
        axis_grip(&owner, "y", pose.dir(&Vec3::Y), pose.at(row.x, row.y + 1.0, 0.0), GripStyle::Ring, format!("fallingLoads[id={id}].y"), "m", row.y, 1.0, 0.5, (-200.0, 400.0), ("Impact position y", "Aufprallposition y")),
        axis_grip(&owner, "drop", pose.dir(&Vec3::Z), pose.at(row.x, row.y, row.drop_height), GripStyle::Arrow, format!("fallingLoads[id={id}].dropHeight"), "m", row.drop_height, 1.0, 0.1, (0.0, 100.0), ("Drop height h", "Fallhöhe h")),
        axis_grip(&owner, "force", pose.dir(&Vec3::X), pose.at(row.x - 1.0, row.y, 0.0), GripStyle::Diamond, format!("fallingLoads[id={id}].assumedForce"), "N", row.assumed_force, 10_000.0, 1_000.0, (0.0, 20_000_000.0), ("Assumed force", "Angenommene Kraft")),
    ]
}

fn imperfection(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.execution_imperfections.iter().find(|r| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "executionImperfections");
    let pose = &layout.building;
    vec![
        axis_grip(&owner, "height", pose.dir(&Vec3::Z), pose.at(row.x + row.tolerance, row.y, row.height), GripStyle::Arrow, format!("executionImperfections[id={id}].height"), "m", row.height, 1.0, 0.1, (0.5, 100.0), ("Member height h", "Bauteilhöhe h")),
        axis_grip(&owner, "tolerance", pose.dir(&Vec3::X), pose.at(row.x + row.tolerance, row.y, row.height), GripStyle::Diamond, format!("executionImperfections[id={id}].tolerance"), "m", row.tolerance, 1.0, 0.001, (0.0, 0.5), ("Tolerance Δ", "Toleranz Δ")),
        axis_grip(&owner, "x", pose.dir(&Vec3::X), pose.at(row.x, row.y, 0.0), GripStyle::Ring, format!("executionImperfections[id={id}].x"), "m", row.x, 1.0, 0.5, (-200.0, 400.0), ("Member position x", "Bauteilposition x")),
    ]
}

fn derailment(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.derailment_cases.iter().find(|r| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "derailmentCases");
    let pose = &layout.building;
    vec![
        axis_grip(&owner, "distance", pose.dir(&Vec3::Y), pose.at(row.x, row.y, 0.0), GripStyle::Arrow, format!("derailmentCases[id={id}].distance"), "m", row.distance, 1.0, 0.1, (0.0, 100.0), ("Distance from the track axis", "Abstand von der Gleisachse")),
        axis_grip(&owner, "x", pose.dir(&Vec3::X), pose.at(row.x, row.y, 1.8), GripStyle::Ring, format!("derailmentCases[id={id}].x"), "m", row.x, 1.0, 0.5, (-200.0, 400.0), ("Position x", "Position x")),
        axis_grip(&owner, "force", pose.dir(&Vec3::X), pose.at(row.x - 1.0, row.y, 1.8), GripStyle::Diamond, format!("derailmentCases[id={id}].assumedForce"), "N", row.assumed_force, 100_000.0, 10_000.0, (0.0, 50_000_000.0), ("Assumed resistance", "Angenommener Widerstand")),
    ]
}

fn hoist(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.crane_hoists.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "craneHoists");
    let pose = &layout.crane;
    let y = HOIST_SPACING * index as f64;
    let device = [("grab".to_string(), "Grab or similar", "Greifer o. ä."), ("magnet".to_string(), "Magnet or spreader", "Magnet oder Traverse")];
    vec![
        axis_grip(&owner, "rated-load", pose.dir(&Vec3::Z), pose.at(d.crane_x, y, RUNWAY_HEIGHT + 1.0), GripStyle::Arrow, format!("craneHoists[id={id}].ratedLoad"), "N", row.rated_load, 100_000.0, 10_000.0, (0.0, 20_000_000.0), ("Rated hoist load Q_H", "Hublast Q_H")),
        axis_grip(&owner, "release-fraction", pose.dir(&Vec3::X), pose.at(d.crane_x + 1.0, y, RUNWAY_HEIGHT - 0.5), GripStyle::Diamond, format!("craneHoists[id={id}].releaseFraction"), "-", row.release_fraction, 0.5, 0.05, (0.0, 1.0), ("Released payload fraction Δm/m", "Abgeworfener Lastanteil Δm/m")),
        super::scene::projection_kit::handle_kit::choice_handle(&owner, "release-device", pose.at(d.crane_x - 1.0, y, RUNWAY_HEIGHT - 0.5), format!("craneHoists[id={id}].releaseDevice"), &device, LocalizedCopy::new("Release device of the hoist", "Abwurfeinrichtung des Hubwerks")),
    ]
}

fn footbridge(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.footbridges.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "footbridges");
    let (pose, y) = (&layout.bridge, gallery_y(d, "footbridges", index));
    vec![
        axis_grip(&owner, "loaded-length", pose.dir(&Vec3::X), pose.at(row.loaded_length, y + row.deck_width / 2.0, 0.3), GripStyle::Arrow, format!("footbridges[id={id}].loadedLength"), "m", row.loaded_length, 1.0, 1.0, (1.0, 500.0), ("Loaded length L", "Belastete Länge L")),
        axis_grip(&owner, "deck-width", pose.dir(&Vec3::Y), pose.at(row.loaded_length / 2.0, y + row.deck_width, 0.3), GripStyle::Arrow, format!("footbridges[id={id}].deckWidth"), "m", row.deck_width, 1.0, 0.1, (0.5, 20.0), ("Deck width b", "Deckbreite b")),
        axis_grip(&owner, "natural-frequency", pose.dir(&Vec3::Z), pose.at(row.loaded_length / 2.0, y + row.deck_width / 2.0, 2.0), GripStyle::Diamond, format!("footbridges[id={id}].naturalFrequency"), "Hz", row.natural_frequency, 1.0, 0.05, (0.5, 20.0), ("First vertical natural frequency f_v", "Erste vertikale Eigenfrequenz f_v")),
        axis_grip(&owner, "service-axle", pose.dir(&Vec3::Z), pose.at(row.loaded_length / 4.0, y + row.deck_width / 2.0, 1.2), GripStyle::Diamond, format!("footbridges[id={id}].assumedServiceAxle"), "N", row.assumed_service_axle, 20_000.0, 1_000.0, (0.0, 200_000.0), ("Assumed service vehicle axle load", "Angesetzte Dienstfahrzeug-Achslast")),
    ]
}

fn fatigue_model(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.fatigue_models.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let Some((axles, _)) = wave3_rows::fatigue_axles(row, d.annex) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "fatigueModels");
    let (pose, y) = (&layout.bridge, gallery_y(d, "fatigueModels", index));
    let (length, width) = (axles.last().copied().unwrap_or(0.0), checks::part_2::fatigue_flm3_track_m());
    let mut out = vec![
        axis_grip(&owner, "axle-load", pose.dir(&Vec3::Z), pose.at(0.0, y + width / 2.0, 2.0), GripStyle::Diamond, format!("fatigueModels[id={id}].assumedAxleLoad"), "N", row.assumed_axle_load, 50_000.0, 5_000.0, (0.0, 1_000_000.0), ("Assumed fatigue axle load", "Angesetzte Ermüdungsachslast")),
        Grip { owner: &owner, tag: "lane", kind: HandleKind::CountStepper, anchor: pose.at(length, y + width / 2.0, 0.3), style: GripStyle::Diamond, path: format!("fatigueModels[id={id}].lane"), unit: "1", base: f64::from(row.lane), k: 1.0, round: Round::Int, range: (1.0, 3.0), names: ("Lane number", "Fahrstreifennummer") }.build(),
    ];
    if row.model == "flm1" {
        out.push(axis_grip(&owner, "udl", pose.dir(&Vec3::Z), pose.at(length / 2.0, y + width, 1.0), GripStyle::Arrow, format!("fatigueModels[id={id}].assumedUdl"), "Pa", row.assumed_udl, 1_000.0, 50.0, (0.0, 20_000.0), ("Assumed fatigue distributed load", "Angesetzte Ermüdungsflächenlast")));
    }
    out
}

fn support_collision(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.support_collisions.iter().find(|r| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "supportCollisions");
    let (pose, deck) = (&layout.bridge, deck_width(d));
    let mut out = vec![
        axis_grip(&owner, "position-x", pose.dir(&Vec3::X), pose.at(row.position_x, deck / 2.0, 0.3), GripStyle::Ring, format!("supportCollisions[id={id}].positionX"), "m", row.position_x, 1.0, 0.25, (0.0, d.bridge_span.max(1.0)), ("Position along the bridge", "Lage entlang der Brücke")),
        axis_grip(&owner, "height", pose.dir(&Vec3::Z), pose.at(row.position_x, deck / 2.0, row.height + 0.5), GripStyle::Arrow, format!("supportCollisions[id={id}].height"), "m", row.height, 1.0, 0.05, (0.0, 10.0), ("Height of the line of action", "Höhe der Wirkungslinie")),
        axis_grip(&owner, "force-x", pose.dir(&Vec3::X), pose.at(row.position_x - 1.0, deck / 2.0, row.height), GripStyle::Diamond, format!("supportCollisions[id={id}].assumedForceX"), "N", row.assumed_force_x, 100_000.0, 10_000.0, (0.0, 5_000_000.0), ("Assumed force in driving direction", "Angesetzte Kraft in Fahrtrichtung")),
    ];
    if row.location == "deck" {
        out.push(axis_grip(&owner, "force-y", pose.dir(&Vec3::Y), pose.at(row.position_x, -1.0, row.height), GripStyle::Diamond, format!("supportCollisions[id={id}].assumedForceY"), "N", row.assumed_force_y, 100_000.0, 10_000.0, (0.0, 5_000_000.0), ("Assumed force across the driving direction", "Angesetzte Kraft quer zur Fahrtrichtung")));
    }
    out
}

fn local_fire(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.local_fires.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Solid, id, "localFires");
    let (pose, (x, y)) = (&layout.building, wave3_rows::fire_origin(d, index));
    let radius = row.fire_diameter / 2.0;
    let flame = checks::part_1_2::local_fire(row.fire_diameter, row.heat_release_rate, 1.0).0.max(0.1);
    vec![
        axis_grip(&owner, "diameter", pose.dir(&Vec3::X), pose.at(x + radius, y, 0.3), GripStyle::Arrow, format!("localFires[id={id}].fireDiameter"), "m", row.fire_diameter, 2.0, 0.1, (0.1, 30.0), ("Fire diameter D", "Branddurchmesser D")),
        axis_grip(&owner, "heat-release", pose.dir(&Vec3::Z), pose.at(x, y, flame + 0.3), GripStyle::Diamond, format!("localFires[id={id}].heatReleaseRate"), "W", row.heat_release_rate, 500_000.0, 100_000.0, (100_000.0, 500_000_000.0), ("Heat release rate Q", "Wärmefreisetzungsrate Q")),
        axis_grip(&owner, "ceiling", pose.dir(&Vec3::Z), pose.at(x + 2.0, y, row.ceiling_height), GripStyle::Arrow, format!("localFires[id={id}].ceilingHeight"), "m", row.ceiling_height, 1.0, 0.1, (0.0, 20.0), ("Ceiling height H", "Deckenhöhe H")),
        axis_grip(&owner, "distance", pose.dir(&Vec3::X), pose.at(x + row.horizontal_distance, y + 1.0, row.ceiling_height), GripStyle::Ring, format!("localFires[id={id}].horizontalDistance"), "m", row.horizontal_distance, 1.0, 0.1, (0.0, 20.0), ("Horizontal distance r", "Horizontaler Abstand r")),
        axis_grip(&owner, "flux", pose.dir(&Vec3::Z), pose.at(x - 2.0, y, row.ceiling_height + 0.5), GripStyle::Diamond, format!("localFires[id={id}].assumedFlux"), "W/m2", row.assumed_flux, 10_000.0, 1_000.0, (0.0, 500_000.0), ("Assumed heat flux", "Angesetzter Wärmestrom")),
    ]
}

fn snow_guard(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.snow_guards.iter().find(|r| r.id == id) else { return Vec::new() };
    let Some((index, roof)) = d.roofs.iter().enumerate().find(|(_, r)| r.id == row.roof_id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "snowGuards");
    let (shape, pose) = (RoofShape::of(d, Some(roof)), layout.roof(index));
    let (eaves, down) = wave3_rows::guard_eaves(&shape);
    let at = shape.lift([eaves, shape.b / 2.0], 0.5);
    vec![
        axis_grip(&owner, "slope-length", pose.dir(&v3(-down, 0.0, 0.0)), pose.at(at.x, at.y, at.z), GripStyle::Arrow, format!("snowGuards[id={id}].slopeLength"), "m", row.slope_length, 1.0, 0.1, (0.1, 60.0), ("Slope length b", "Dachflächenlänge b")),
        axis_grip(&owner, "force", pose.dir(&Vec3::Z), pose.at(at.x, at.y, at.z + 0.5), GripStyle::Diamond, format!("snowGuards[id={id}].assumedForce"), "N/m", row.assumed_force, 1_000.0, 100.0, (0.0, 100_000.0), ("Assumed line force", "Angesetzte Linienkraft")),
    ]
}

fn rail_derailment(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.rail_derailments.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "railDerailments");
    let (pose, y) = (&layout.bridge, gallery_y(d, "railDerailments", index));
    let line_y = y + TRACK_GAUGE / 2.0 + row.offset;
    vec![
        axis_grip(&owner, "line-load", pose.dir(&Vec3::Z), pose.at(row.loaded_length / 2.0, line_y, 2.0), GripStyle::Diamond, format!("railDerailments[id={id}].assumedLineLoad"), "N/m", row.assumed_line_load, 10_000.0, 1_000.0, (0.0, 500_000.0), ("Assumed derailment line load", "Angesetzte Entgleisungs-Linienlast")),
        axis_grip(&owner, "loaded-length", pose.dir(&Vec3::X), pose.at(row.loaded_length, line_y, 1.0), GripStyle::Arrow, format!("railDerailments[id={id}].loadedLength"), "m", row.loaded_length, 1.0, 0.5, (0.5, 200.0), ("Loaded length", "Belastete Länge")),
        axis_grip(&owner, "offset", pose.dir(&Vec3::Y), pose.at(row.loaded_length / 2.0, line_y, 1.0), GripStyle::Arrow, format!("railDerailments[id={id}].offset"), "m", row.offset, 1.0, 0.05, (0.0, 10.0), ("Offset from the track axis", "Abstand von der Gleisachse")),
    ]
}

fn rail_group(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.rail_load_groups.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "railLoadGroups");
    let (pose, y) = (&layout.bridge, gallery_y(d, "railLoadGroups", index));
    let (mid, end) = (y + TRACK_GAUGE / 2.0, 3.0 * LM71_AXLE_SPACING);
    vec![
        axis_grip(&owner, "axle-load", pose.dir(&Vec3::Z), pose.at(0.0, mid, 2.5), GripStyle::Diamond, format!("railLoadGroups[id={id}].assumedAxleLoad"), "N", row.assumed_axle_load, 50_000.0, 5_000.0, (0.0, 1_000_000.0), ("Assumed group axle load", "Angesetzte Gruppen-Achslast")),
        axis_grip(&owner, "udl", pose.dir(&Vec3::Z), pose.at(end + 2.0, mid, 1.0), GripStyle::Diamond, format!("railLoadGroups[id={id}].assumedUdl"), "N/m", row.assumed_udl, 10_000.0, 1_000.0, (0.0, 500_000.0), ("Assumed group distributed load", "Angesetzte Gruppen-Streckenlast")),
        axis_grip(&owner, "longitudinal", pose.dir(&Vec3::X), pose.at(end, mid, 0.6), GripStyle::Arrow, format!("railLoadGroups[id={id}].assumedLongitudinal"), "N", row.assumed_longitudinal, 100_000.0, 10_000.0, (0.0, 6_000_000.0), ("Assumed group longitudinal force", "Angesetzte Gruppen-Längskraft")),
        axis_grip(&owner, "lateral", pose.dir(&Vec3::Y), pose.at(end / 2.0, y + TRACK_GAUGE, 0.6), GripStyle::Arrow, format!("railLoadGroups[id={id}].assumedLateral"), "N", row.assumed_lateral, 10_000.0, 1_000.0, (0.0, 1_000_000.0), ("Assumed group lateral force", "Angesetzte Gruppen-Querkraft")),
        axis_grip(&owner, "factor-vertical", pose.dir(&Vec3::Z), pose.at(0.5, mid, 1.5), GripStyle::Diamond, format!("railLoadGroups[id={id}].factorVertical"), "1", row.factor_vertical, 0.5, 0.05, (0.0, 1.0), ("Vertical component factor", "Faktor der Vertikalkomponente")),
        axis_grip(&owner, "factor-longitudinal", pose.dir(&Vec3::Z), pose.at(end - 0.5, mid, 1.5), GripStyle::Diamond, format!("railLoadGroups[id={id}].factorLongitudinal"), "1", row.factor_longitudinal, 0.5, 0.05, (0.0, 1.0), ("Longitudinal component factor", "Faktor der Längskomponente")),
        axis_grip(&owner, "factor-lateral", pose.dir(&Vec3::Z), pose.at(end / 2.0, y + TRACK_GAUGE, 1.5), GripStyle::Diamond, format!("railLoadGroups[id={id}].factorLateral"), "1", row.factor_lateral, 0.5, 0.05, (0.0, 1.0), ("Lateral component factor", "Faktor der Querkomponente")),
    ]
}

fn dust_explosion(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.dust_explosions.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Load, id, "dustExplosions");
    let (pose, y) = (&layout.silo, gallery_y(d, "dustExplosions", index));
    let side = row.volume.max(1.0).cbrt().clamp(1.0, 30.0);
    let (mid, top) = (y + side / 2.0, side + 0.5);
    vec![
        axis_grip(&owner, "kst", pose.dir(&Vec3::Z), pose.at(0.0, y, top), GripStyle::Diamond, format!("dustExplosions[id={id}].kst"), "1", row.kst, 50.0, 5.0, (0.0, 600.0), ("Dust constant K_St", "Staubkennzahl K_St")),
        axis_grip(&owner, "p-max", pose.dir(&Vec3::Z), pose.at(side, y, top), GripStyle::Diamond, format!("dustExplosions[id={id}].pMax"), "Pa", row.p_max, 100_000.0, 10_000.0, (0.0, 1_500_000.0), ("Maximum explosion pressure p_max", "Maximaler Explosionsdruck p_max")),
        axis_grip(&owner, "volume", pose.dir(&Vec3::X), pose.at(side, mid, side / 2.0), GripStyle::Arrow, format!("dustExplosions[id={id}].volume"), "m3", row.volume, 3.0 * side * side, 1.0, (0.1, 20_000.0), ("Volume V", "Volumen V")),
        axis_grip(&owner, "vent-area", pose.dir(&Vec3::Z), pose.at(side / 2.0, mid, top), GripStyle::Arrow, format!("dustExplosions[id={id}].ventArea"), "m2", row.vent_area, 2.0, 0.1, (0.01, 200.0), ("Vent area A_v", "Entlastungsfläche A_v")),
        axis_grip(&owner, "static-opening-pressure", pose.dir(&Vec3::Z), pose.at(side / 2.0, y, top), GripStyle::Diamond, format!("dustExplosions[id={id}].staticOpeningPressure"), "Pa", row.static_opening_pressure, 10_000.0, 1_000.0, (0.0, 200_000.0), ("Static opening pressure p_stat", "Statischer Ansprechdruck p_stat")),
        axis_grip(&owner, "design-pressure", pose.dir(&Vec3::Z), pose.at(side / 2.0, y + side, top), GripStyle::Diamond, format!("dustExplosions[id={id}].designPressure"), "Pa", row.design_pressure, 50_000.0, 5_000.0, (0.0, 500_000.0), ("Reduced explosion pressure p_red", "Reduzierter Explosionsdruck p_red")),
    ]
}

fn rail_span(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.rail_dynamics.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "railDynamics");
    let (pose, y) = (&layout.bridge, gallery_y(d, "railDynamics", index));
    let maintenance = [("careful".to_string(), "Careful maintenance (Φ₂)", "Sorgfältige Unterhaltung (Φ₂)"), ("standard".to_string(), "Standard maintenance (Φ₃)", "Normale Unterhaltung (Φ₃)")];
    vec![
        axis_grip(&owner, "determinant-length", pose.dir(&Vec3::X), pose.at(row.determinant_length, y + TRACK_WIDTH / 2.0, 0.3), GripStyle::Arrow, format!("railDynamics[id={id}].determinantLength"), "m", row.determinant_length, 1.0, 0.5, (0.5, 500.0), ("Determinant length L_Φ", "Maßgebende Länge L_Φ")),
        super::scene::projection_kit::handle_kit::choice_handle(&owner, "maintenance", pose.at(row.determinant_length / 2.0, y + TRACK_WIDTH, 1.0), format!("railDynamics[id={id}].maintenance"), &maintenance, LocalizedCopy::new("Track maintenance", "Gleisunterhaltung")),
    ]
}

fn dispersal(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.load_dispersals.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "loadDispersals");
    let (pose, y) = (&layout.bridge, gallery_y(d, "loadDispersals", index));
    let a = checks::part_2::dispersed_side_m(row.wheel_length, row.pavement_thickness, row.slab_thickness);
    let b = checks::part_2::dispersed_side_m(row.wheel_width, row.pavement_thickness, row.slab_thickness);
    let (cx, cy) = (a / 2.0, y + b / 2.0);
    vec![
        axis_grip(&owner, "wheel-length", pose.dir(&Vec3::X), pose.at(cx + row.wheel_length / 2.0, cy, 0.3), GripStyle::Arrow, format!("loadDispersals[id={id}].wheelLength"), "m", row.wheel_length, 1.0, 0.05, (0.05, 5.0), ("Wheel contact length", "Radaufstandslänge")),
        axis_grip(&owner, "wheel-width", pose.dir(&Vec3::Y), pose.at(cx, cy + row.wheel_width / 2.0, 0.3), GripStyle::Arrow, format!("loadDispersals[id={id}].wheelWidth"), "m", row.wheel_width, 1.0, 0.05, (0.05, 5.0), ("Wheel contact width", "Radaufstandsbreite")),
        axis_grip(&owner, "pavement-thickness", pose.dir(&Vec3::Z), pose.at(0.0, y, 0.0), GripStyle::Diamond, format!("loadDispersals[id={id}].pavementThickness"), "m", row.pavement_thickness, 0.1, 0.01, (0.0, 2.0), ("Pavement thickness t_p", "Belagdicke t_p")),
        axis_grip(&owner, "slab-thickness", pose.dir(&Vec3::Z), pose.at(a, y + b, 0.0), GripStyle::Diamond, format!("loadDispersals[id={id}].slabThickness"), "m", row.slab_thickness, 0.1, 0.01, (0.0, 3.0), ("Slab thickness t_s", "Plattendicke t_s")),
    ]
}

fn bulk_solid(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.bulk_solids.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "bulkSolids");
    let (pose, y) = (&layout.silo, gallery_y(d, "bulkSolids", index));
    vec![axis_grip(&owner, "internal-friction", pose.dir(&Vec3::Z), pose.at(PLATE_SIDE / 2.0, y + PLATE_SIDE / 2.0, 1.0), GripStyle::Diamond, format!("bulkSolids[id={id}].internalFriction"), "rad", row.internal_friction, 0.5, 0.0174532925, (0.087, 1.5), ("Angle of internal friction φ_i", "Innerer Reibungswinkel φ_i"))]
}

fn machine(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some((index, row)) = d.rotating_machines.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let owner = entity_ref(EntityKind::Surface, id, "rotatingMachines");
    let (pose, y) = (&layout.silo, gallery_y(d, "rotatingMachines", index));
    let x = PLATE_SIDE * 2.0;
    vec![
        axis_grip(&owner, "mass", pose.dir(&Vec3::Z), pose.at(x, y + PLATE_SIDE / 2.0, 2.5), GripStyle::Arrow, format!("rotatingMachines[id={id}].mass"), "kg", row.mass, 1000.0, 10.0, (1.0, 1_000_000.0), ("Rotating mass m", "Rotierende Masse m")),
        axis_grip(&owner, "eccentricity", pose.dir(&Vec3::Y), pose.at(x, y + PLATE_SIDE, 0.5), GripStyle::Diamond, format!("rotatingMachines[id={id}].eccentricity"), "m", row.eccentricity, 0.05, 0.0005, (0.0, 0.5), ("Eccentricity e", "Exzentrizität e")),
        axis_grip(&owner, "frequency", pose.dir(&Vec3::X), pose.at(PLATE_SIDE * 2.5, y + PLATE_SIDE / 2.0, 0.5), GripStyle::Diamond, format!("rotatingMachines[id={id}].frequency"), "Hz", row.frequency, 5.0, 0.5, (0.1, 200.0), ("Rotation frequency f", "Drehfrequenz f")),
    ]
}

fn explosion(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(index) = d.internal_explosions.iter().position(|e| e.id == id) else { return Vec::new() };
    let row = &d.internal_explosions[index];
    let owner = entity_ref(EntityKind::Solid, id, "internalExplosions");
    let pose = &layout.building;
    let (length, depth) = wave3::room_size(row);
    let (x0, y0) = wave3::room_origin(d, index);
    let side = row.vent_area.max(0.01).sqrt();
    vec![
        axis_grip(&owner, "volume", pose.dir(&Vec3::X), pose.at(x0 + length, y0 + depth / 2.0, 1.0), GripStyle::Arrow, format!("internalExplosions[id={id}].volume"), "m3", row.volume, depth * 3.0, 1.0, (5.0, 1500.0), ("Enclosure volume V", "Raumvolumen V")),
        axis_grip(&owner, "vent", pose.dir(&Vec3::Z), pose.at(x0 + length, y0 + depth / 2.0, 1.5 + side / 2.0), GripStyle::Diamond, format!("internalExplosions[id={id}].ventArea"), "m2", row.vent_area, 2.0 * side, 0.05, (0.01, 50.0), ("Vent area A_v (tangent)", "Entlastungsfläche A_v (Tangente)")),
    ]
}

fn section(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(index) = d.thermal_sections.iter().position(|e| e.id == id) else { return Vec::new() };
    let row = &d.thermal_sections[index];
    let owner = entity_ref(EntityKind::Solid, id, "thermalSections");
    let pose = &layout.bridge;
    let x0 = wave3::section_x(index);
    vec![
        axis_grip(&owner, "depth", pose.dir(&Vec3::Z), pose.at(x0, 0.0, DECK_Z - row.depth), GripStyle::Arrow, format!("thermalSections[id={id}].depth"), "m", row.depth, -1.0, 0.05, (0.2, 10.0), ("Section depth h", "Querschnittshöhe h")),
        axis_grip(&owner, "length", pose.dir(&Vec3::X), pose.at(row.length, deck_width(d) / 2.0, DECK_Z), GripStyle::Arrow, format!("thermalSections[id={id}].length"), "m", row.length, 1.0, 1.0, (1.0, 1000.0), ("Expansion length L", "Dehnlänge L")),
    ]
}

fn crane(d: &En1991Snapshot, layout: &Layout) -> Vec<Handle> {
    if !d.crane_claimed {
        return Vec::new();
    }
    let owner = entity_ref(EntityKind::Member, "crane", "@site");
    let pose = &layout.crane;
    let (length, wheel_base) = (d.crane_runway_length.max(1.0), d.crane_wheel_base.max(0.1));
    vec![
        Grip { owner: &owner, tag: "hoisting-speed", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(d.crane_x, 0.0, RUNWAY_HEIGHT + 2.0), style: GripStyle::Arrow, path: "hoistingSpeed".into(), unit: "m/s", base: d.hoisting_speed, k: 0.1, round: Round::Step(0.05), range: (0.0, 2.0), names: ("Hoisting speed v_h", "Hubgeschwindigkeit v_h") }.build(),
        Grip { owner: &owner, tag: "x", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(d.crane_x, 0.0, RUNWAY_HEIGHT + 1.0), style: GripStyle::Diamond, path: "craneX".into(), unit: "m", base: d.crane_x, k: 1.0, round: Round::Step(0.25), range: (0.0, length), names: ("Crane position", "Kranposition") }.build(),
        Grip { owner: &owner, tag: "runway-length", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(length, 0.0, RUNWAY_HEIGHT), style: GripStyle::Arrow, path: "craneRunwayLength".into(), unit: "m", base: d.crane_runway_length, k: 1.0, round: Round::Step(0.5), range: (1.0, 100.0), names: ("Runway length", "Kranbahnlänge") }.build(),
        Grip { owner: &owner, tag: "wheel-base", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(d.crane_x + wheel_base / 2.0, 0.6, RUNWAY_HEIGHT), style: GripStyle::Arrow, path: "craneWheelBase".into(), unit: "m", base: d.crane_wheel_base, k: 2.0, round: Round::Step(0.05), range: (0.5, 10.0), names: ("Wheel base", "Radstand") }.build(),
    ]
}

fn silo(d: &En1991Snapshot, layout: &Layout) -> Vec<Handle> {
    if !d.silo_claimed {
        return Vec::new();
    }
    let owner = entity_ref(EntityKind::Shell, "silo", "@site");
    let pose = &layout.silo;
    let (radius, height) = (silo_radius(d), d.silo_height.max(0.5));
    let outlet = OUTLET_RADIUS.min(radius);
    vec![
        Grip { owner: &owner, tag: "height", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(0.0, 0.0, height), style: GripStyle::Arrow, path: "siloHeight".into(), unit: "m", base: d.silo_height, k: 1.0, round: Round::Step(0.1), range: (1.0, 60.0), names: ("Silo height", "Silohöhe") }.build(),
        Grip { owner: &owner, tag: "diameter", kind: HandleKind::Radial { centre: pose.at(0.0, 0.0, height / 2.0) }, anchor: pose.at(radius, 0.0, height / 2.0), style: GripStyle::Ring, path: "siloDiameter".into(), unit: "m", base: d.silo_diameter, k: 2.0, round: Round::Step(0.1), range: (0.5, 40.0), names: ("Silo diameter", "Silodurchmesser") }.build(),
        Grip { owner: &owner, tag: "eccentricity", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(d.silo_eccentricity, 0.0, height + 0.4), style: GripStyle::Diamond, path: "siloEccentricity".into(), unit: "m", base: d.silo_eccentricity, k: 1.0, round: Round::Step(0.05), range: (0.0, radius), names: ("Filling eccentricity e_f", "Befüllungsausmitte e_f") }.build(),
        Grip { owner: &owner, tag: "hopper-angle", kind: HandleKind::Angular { axis: pose.dir(&Vec3::Y), centre: pose.at(radius, 0.0, 0.0) }, anchor: pose.at((radius + outlet) / 2.0, 0.0, -hopper_depth(radius, d.silo_hopper_angle_deg) / 2.0), style: GripStyle::Ring, path: "siloHopperAngleDeg".into(), unit: "deg", base: d.silo_hopper_angle_deg, k: 180.0 / std::f64::consts::PI, round: Round::Step(1.0), range: (1.0, 85.0), names: ("Hopper wall angle", "Trichterneigung") }.build(),
    ]
}

fn fire(d: &En1991Snapshot, layout: &Layout) -> Vec<Handle> {
    if d.fire_mode == super::model::FireMode::None {
        return Vec::new();
    }
    let owner = entity_ref(EntityKind::Solid, "fire", "@site");
    let pose = &layout.building;
    let room = compartment(d);
    let per_metre_of_width = room.window_height * room.height.sqrt() / room.total_area;
    vec![
        Grip { owner: &owner, tag: "height", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(FIRE_MARGIN, FIRE_MARGIN, room.height), style: GripStyle::Arrow, path: "fireCompartmentHeight".into(), unit: "m", base: d.fire_compartment_height, k: 1.0, round: Round::Step(0.1), range: (2.0, 12.0), names: ("Fire compartment height", "Brandabschnittshöhe") }.build(),
        Grip { owner: &owner, tag: "opening", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(FIRE_MARGIN + room.window_offset + room.window_width, FIRE_MARGIN, room.window_sill + room.window_height / 2.0), style: GripStyle::Arrow, path: "fireOpeningFactor".into(), unit: "m½", base: d.fire_opening_factor, k: per_metre_of_width, round: Round::Step(0.001), range: (0.02_f64.min(d.fire_opening_factor), 0.20_f64.max(d.fire_opening_factor)), names: ("Opening factor O (window width)", "Öffnungsfaktor O (Fensterbreite)") }.build(),
        Grip { owner: &owner, tag: "inertia", kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(FIRE_MARGIN + room.side + room.wall, FIRE_MARGIN + room.side / 2.0, room.height / 2.0), style: GripStyle::Diamond, path: "fireThermalInertia".into(), unit: "J/(m²·s½·K)", base: d.fire_thermal_inertia, k: 1.0 / WALL_THICKNESS_PER_B, round: Round::Step(10.0), range: (100.0_f64.min(d.fire_thermal_inertia), 2200.0_f64.max(d.fire_thermal_inertia)), names: ("Thermal inertia b (wall thickness)", "Wärmeträgheit b (Wanddicke)") }.build(),
    ]
}

/// 🧲️ Translate grips of every placeable entity; a drag writes `placements[id=ENTITY].x|y`, and `PlacementCommit::of_writes` turns the first drag of an entity without a row into one insert (one undo step).
fn moves(d: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (kind, collection, pose) = match id {
        "building" => (EntityKind::Solid, "@site".to_string(), &layout.building),
        "bridge" => (EntityKind::Solid, "@site".to_string(), &layout.bridge),
        "crane" => (EntityKind::Member, "@site".to_string(), &layout.crane),
        "silo" => (EntityKind::Shell, "@site".to_string(), &layout.silo),
        _ => match d.roofs.iter().position(|r| r.id == id) {
            Some(index) if index > 0 => (EntityKind::Surface, "roofs".to_string(), &layout.roofs[index]),
            _ => return Vec::new(),
        },
    };
    let owner = entity_ref(kind, id, &collection);
    let stored: Vec<_> = d.placements.iter().map(|p| super::scene::projection_kit::Placement { entity_id: p.id.clone(), origin: Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    let base_of = |leaf: &str| super::scene::projection_kit::grip_base(&stored, &pose.origin, id, leaf);
    [("move-x", Vec3::X, base_of("x"), "x", "Move along X", "Entlang X verschieben"), ("move-y", Vec3::Y, base_of("y"), "y", "Move along Y", "Entlang Y verschieben")]
        .into_iter()
        .map(|(tag, axis, base, leaf, en, de)| Grip { owner: &owner, tag, kind: HandleKind::Translate { axis }, anchor: pose.origin.clone(), style: GripStyle::Ring, path: format!("placements[id={id}].{leaf}"), unit: "m", base, k: 1.0, round: Round::Step(0.05), range: (base - 1.0e3, base + 1.0e3), names: (en, de) }.build())
        .collect()
}

/// 🕹️ The handles of one document entity, anchored in the given layout.
pub fn build(document: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let base = super::scene::projection_kit::base_id(id);
    if let (Some(row), Some(pose)) = (document.cases.iter().find(|c| c.id == base), layout.case(base)) {
        return super::scene::projection_kit::design_case::forms::handles::<super::checks::catalogue::En1991Cases>(row, pose, "cases");
    }
    let mut out = dimensions(document, layout, id);
    out.extend(moves(document, layout, id));
    out
}

fn dimensions(document: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let mut out = geometric(document, layout, id);
    out.extend(gauges::of(document, layout, id, &out));
    out
}

fn geometric(document: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    match owner_of(document, id) {
        Some(Owner::Building) => building(document, layout),
        Some(Owner::Roof(index, _)) => roof(document, layout, index),
        Some(Owner::Floor(f)) => floor(document, layout, &f.id),
        Some(Owner::SelfWeight(e)) => self_weight(document, layout, &e.id),
        Some(Owner::Face(f)) => face(document, layout, &f.id),
        Some(Owner::Bridge) => bridge(document, layout),
        Some(Owner::Crane) => crane(document, layout),
        Some(Owner::Silo) => silo(document, layout),
        Some(Owner::Fire) => fire(document, layout),
        Some(Owner::Lane(l)) => lane(document, layout, &l.id),
        Some(Owner::Stage(s)) => stage(document, layout, &s.id),
        Some(Owner::Surface(s)) => surface(document, layout, &s.id),
        Some(Owner::Vehicle(v)) => vehicle(document, layout, &v.id),
        Some(Owner::BridgeWind(r)) => bridge_wind(document, layout, &r.id),
        Some(Owner::Dynamic(r)) => dynamic_element(document, layout, &r.id),
        Some(Owner::Impact(r)) => impact(document, layout, &r.id),
        Some(Owner::Explosion(r)) => explosion(document, layout, &r.id),
        Some(Owner::Section(r)) => section(document, layout, &r.id),
        Some(Owner::Helicopter(r)) => helicopter(document, layout, &r.id),
        Some(Owner::Tie(r)) => tie(document, layout, &r.id),
        Some(Owner::ConstructionLoad(r)) => construction_load(document, layout, &r.id),
        Some(Owner::Falling(r)) => falling(document, layout, &r.id),
        Some(Owner::Imperfection(r)) => imperfection(document, layout, &r.id),
        Some(Owner::Derailment(r)) => derailment(document, layout, &r.id),
        Some(Owner::CraneHoist(r)) => hoist(document, layout, &r.id),
        Some(Owner::Footbridge(r)) => footbridge(document, layout, &r.id),
        Some(Owner::RailSpan(r)) => rail_span(document, layout, &r.id),
        Some(Owner::Dispersal(r)) => dispersal(document, layout, &r.id),
        Some(Owner::BulkSolid(r)) => bulk_solid(document, layout, &r.id),
        Some(Owner::Machine(r)) => machine(document, layout, &r.id),
        Some(Owner::FatigueModel(r)) => fatigue_model(document, layout, &r.id),
        Some(Owner::Collision(r)) => support_collision(document, layout, &r.id),
        Some(Owner::LocalFire(r)) => local_fire(document, layout, &r.id),
        Some(Owner::SnowGuard(r)) => snow_guard(document, layout, &r.id),
        Some(Owner::RailDerailment(r)) => rail_derailment(document, layout, &r.id),
        Some(Owner::RailGroup(r)) => rail_group(document, layout, &r.id),
        Some(Owner::Dust(r)) => dust_explosion(document, layout, &r.id),
        Some(Owner::Accidental(r)) => accidental(document, layout, &r.id),
        Some(Owner::Thermal) | None => Vec::new(),
    }
}

/// 🆔️ The handle ids a scene entity advertises.
pub fn ids(document: &En1991Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🕹️ The handles of every selected entity (derived scene ids resolve to their document entity), in selection order, without duplicates.
pub fn handles(document: &En1991Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        for handle in build(document, &layout, &entity.id) {
            if !out.iter().any(|h| h.id == handle.id) {
                out.push(handle);
            }
        }
    }
    out
}
