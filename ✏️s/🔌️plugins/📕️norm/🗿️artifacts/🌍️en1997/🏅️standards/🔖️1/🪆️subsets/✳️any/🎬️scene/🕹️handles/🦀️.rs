//! 🕹️ Handles: viewport grips for layer interfaces, groundwater, footing size, embedment and position, wall dimensions, slope geometry, pile length, diameter, count and position, embedded-wall height, embedment and prop level, anchor lengths and inclination, pile-group spacing and counts, and the piezometric level of a layer.
//! The drag parameter `s` is the displacement in metres (radians for angular grips) from the anchor along the grip direction; every field value is `base + k * s`.

use super::kit::*;
use super::scene::projection_kit::handle_kit::choice_handle;
use super::layout::{wall_stem_x, Layout, Placement};
use super::mechanics as mech;
use super::cases as case_scene;
use super::model::{CharacteristicParameter, En1997Snapshot, ParameterBasis, PileConcreteClass};
use super::checks::catalogue::En1997Cases;
use super::scene::*;
use super::Ctx;

pub const LOAD_GAIN: f64 = 100_000.0;
pub const GRID: f64 = 0.05;
const MIN_LENGTH: f64 = 0.1;

#[path = "🎚️gauges/🦀️.rs"]
pub mod gauges;

struct Grip {
    id: String,
    entity: EntityRef,
    kind: HandleKind,
    anchor: Vec3,
    style: GripStyle,
    writes: Vec<FieldBinding>,
    limits: (f64, f64),
    names: (String, String),
}

impl Grip {
    fn build(self) -> Handle {
        Handle {
            id: self.id,
            entity: self.entity,
            kind: self.kind,
            anchor: self.anchor,
            grip: self.style,
            writes: self.writes,
            snap: SnapRule { grid: Some(GRID), increment: None, targets: Vec::new(), catalogue: Vec::new(), tolerance: 0.02 },
            limits: (self.limits.0 < self.limits.1).then(|| Interval::new(self.limits.0, self.limits.1)),
            accessible_name: LocalizedCopy::new(self.names.0, self.names.1),
        }
    }
}

fn write(path: String, base: f64, k: f64, unit: &str, step: Option<f64>) -> FieldBinding {
    FieldBinding { path, map: Affine { base, k }, unit: unit.into(), quantise: step.map(|step| Quantise::Round { step }) }
}

fn metres(path: String, base: f64, k: f64) -> FieldBinding {
    write(path, base, k, "m", Some(GRID))
}

fn range(base: f64, k: f64, lo: f64, hi: f64) -> (f64, f64) {
    let (a, b) = ((lo - base) / k, (hi - base) / k);
    (a.min(b), a.max(b))
}

fn named(en: &str, de: &str, id: &str) -> (String, String) {
    (format!("{en} {id}"), format!("{de} {id}"))
}

fn layers(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    let mut top = 0.0_f64;
    for (i, l) in d.layers.iter().enumerate() {
        let bottom = l.depth_bottom;
        let next = d.layers.get(i + 1);
        let mut writes = vec![metres(format!("layers[id={}].depthBottom", l.id), bottom, -1.0)];
        let mut lo = top + MIN_LENGTH;
        let mut hi = next.map_or(layout.depth.max(bottom + 1.0), |n| n.depth_bottom - MIN_LENGTH);
        if let Some(n) = next {
            writes.push(metres(format!("layers[id={}].depthTop", n.id), bottom, -1.0));
        } else {
            hi = hi.max(lo + MIN_LENGTH);
        }
        lo = lo.min(hi - MIN_LENGTH);
        out.push(
            Grip {
                id: format!("{}:interface", l.id),
                entity: entity_ref(EntityKind::Solid, &l.id, "layers"),
                kind: HandleKind::ScalarAlong { axis: Vec3::Z },
                anchor: v3(layout.min[0], layout.centre()[1], -bottom),
                style: GripStyle::Arrow,
                writes,
                limits: range(bottom, -1.0, lo, hi),
                names: named("Lower interface of layer", "Untere Schichtgrenze von Schicht", &l.id),
            }
            .build(),
        );
        let layer = entity_ref(EntityKind::Solid, &l.id, "layers");
        gauges::rack(gauges::LAYERS, l, "layers", &l.id, &layer, |dy| v3(layout.min[0] + 0.3, layout.centre()[1] + dy, -(top + bottom) / 2.0), out);
        top = bottom;
    }
}

fn groundwater(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    let gw = d.groundwater_level;
    out.push(
        Grip {
            id: "groundwater:level".into(),
            entity: EntityRef::new(EntityKind::Surface, "groundwater", "groundwaterLevel"),
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: v3(layout.max[0], layout.centre()[1], -gw),
            style: GripStyle::Diamond,
            writes: vec![metres("groundwaterLevel".into(), gw, -1.0)],
            limits: range(gw, -1.0, 0.0, layout.depth),
            names: ("Groundwater level".into(), "Grundwasserstand".into()),
        }
        .build(),
    );
}

fn position(id: &str, entity: &EntityRef, origin: &Vec3, anchor: &Vec3, out: &mut Vec<Handle>) {
    for (tag, axis, base, field, en, de) in [("move-x", Vec3::X, origin.x, "x", "Move along X", "Entlang X verschieben"), ("move-y", Vec3::Y, origin.y, "y", "Move along Y", "Entlang Y verschieben")] {
        out.push(
            Grip { id: format!("{id}:{tag}"), entity: entity.clone(), kind: HandleKind::Translate { axis }, anchor: anchor.clone(), style: GripStyle::Ring, writes: vec![metres(format!("placements[id={id}].{field}"), base, 1.0)], limits: (-1.0e4, 1.0e4), names: named(en, de, id) }.build(),
        );
    }
}

fn footings(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for f in d.footings.iter().filter(|f| f.width > 1e-6 && f.length > 1e-6) {
        let pose = layout.pose(&f.id);
        let entity = entity_ref(EntityKind::Solid, &f.id, "footings");
        let base = -f.embedment;
        out.push(Grip { id: format!("{}:width", f.id), entity: entity.clone(), kind: HandleKind::FaceOffset { normal: pose.dir(&Vec3::X) }, anchor: pose.at(f.width / 2.0, 0.0, base), style: GripStyle::Arrow, writes: vec![metres(format!("footings[id={}].width", f.id), f.width, 2.0)], limits: range(f.width, 2.0, 0.3, 50.0), names: named("Width of footing", "Breite des Fundaments", &f.id) }.build());
        out.push(Grip { id: format!("{}:length", f.id), entity: entity.clone(), kind: HandleKind::FaceOffset { normal: pose.dir(&Vec3::Y) }, anchor: pose.at(0.0, f.length / 2.0, base), style: GripStyle::Arrow, writes: vec![metres(format!("footings[id={}].length", f.id), f.length, 2.0)], limits: range(f.length, 2.0, 0.3, 50.0), names: named("Length of footing", "Länge des Fundaments", &f.id) }.build());
        out.push(Grip { id: format!("{}:embedment", f.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(0.0, 0.0, base), style: GripStyle::Diamond, writes: vec![metres(format!("footings[id={}].embedment", f.id), f.embedment, -1.0)], limits: range(f.embedment, -1.0, MIN_LENGTH, layout.depth), names: named("Embedment of footing", "Einbindetiefe des Fundaments", &f.id) }.build());
        if let Some(case) = mech::governing_case(f) {
            out.push(Grip { id: format!("{}:load-h", f.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(0.0, 0.0, 0.3), style: GripStyle::Arrow, writes: vec![write(format!("footings[id={}].loadCases[id={}].horizontalVariable", f.id, case.id), case.horizontal_variable, LOAD_GAIN, "N", Some(1000.0))], limits: range(case.horizontal_variable, LOAD_GAIN, 0.0, 1.0e8), names: named("Horizontal load tip of footing", "Horizontallast-Spitze des Fundaments", &f.id) }.build());
        }
        gauges::rack(gauges::FOOTINGS, f, "footings", &f.id, &entity, |dy| pose.at(-f.width / 2.0 - 0.6, dy, 0.3), out);
        position(&f.id, &entity, &pose.origin, &pose.at(0.0, 0.0, 0.3), out);
    }
}

fn walls(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for w in d.retaining_walls.iter().filter(|w| w.height > 1e-6) {
        let pose = layout.pose(&w.id);
        let entity = entity_ref(EntityKind::Solid, &w.id, "retainingWalls");
        let t = w.stem_thickness.max(0.05);
        let path = |field: &str| format!("retainingWalls[id={}].{field}", w.id);
        out.push(Grip { id: format!("{}:height", w.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(t / 2.0, 0.0, w.height), style: GripStyle::Arrow, writes: vec![metres(path("height"), w.height, 1.0)], limits: range(w.height, 1.0, 0.5, 30.0), names: named("Height of wall", "Höhe der Wand", &w.id) }.build());
        out.push(Grip { id: format!("{}:embedment", w.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(t / 2.0, 0.0, -w.embedment), style: GripStyle::Diamond, writes: vec![metres(path("embedment"), w.embedment, -1.0)], limits: range(w.embedment, -1.0, 0.0, 15.0), names: named("Embedment of wall", "Einbindetiefe der Wand", &w.id) }.build());
        out.push(Grip { id: format!("{}:base-width", w.id), entity: entity.clone(), kind: HandleKind::FaceOffset { normal: pose.dir(&Vec3::X) }, anchor: pose.at(w.base_width - wall_stem_x(w), 0.0, -w.embedment), style: GripStyle::Arrow, writes: vec![metres(path("baseWidth"), w.base_width, 1.0)], limits: range(w.base_width, 1.0, 0.5, 30.0), names: named("Base width of wall", "Fußbreite der Wand", &w.id) }.build());
        if w.reinforcement_spacing > 1e-6 {
            out.push(Grip { id: format!("{}:reinforcement", w.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(w.base_width, 0.0, w.reinforcement_spacing), style: GripStyle::Diamond, writes: vec![metres(path("reinforcementSpacing"), w.reinforcement_spacing, 1.0)], limits: range(w.reinforcement_spacing, 1.0, 0.2, 2.0), names: named("Reinforcement spacing of wall", "Bewehrungsabstand der Wand", &w.id) }.build());
        }
        out.push(Grip { id: format!("{}:stem", w.id), entity, kind: HandleKind::FaceOffset { normal: pose.dir(&Vec3::X) }, anchor: pose.at(t, 0.0, w.height / 2.0), style: GripStyle::Arrow, writes: vec![metres(path("stemThickness"), w.stem_thickness, 1.0)], limits: range(w.stem_thickness, 1.0, 0.1, 5.0), names: named("Stem thickness of wall", "Schaftdicke der Wand", &w.id) }.build());
        gauges::rack(gauges::RETAINING_WALLS, w, "retainingWalls", &w.id, &entity_ref(EntityKind::Solid, &w.id, "retainingWalls"), |dy| pose.at(-0.8, dy, 0.3), out);
    }
}

fn slopes(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for s in d.slopes.iter().filter(|s| s.height > 1e-6 && s.angle_deg > 1.0 && s.angle_deg < 89.0) {
        let pose = layout.pose(&s.id);
        let entity = entity_ref(EntityKind::Solid, &s.id, "slopes");
        let run = mech::slope_run(s.height, s.angle_deg);
        out.push(Grip { id: format!("{}:height", s.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(0.0, 0.0, s.height), style: GripStyle::Arrow, writes: vec![metres(format!("slopes[id={}].height", s.id), s.height, 1.0)], limits: range(s.height, 1.0, 0.5, 60.0), names: named("Height of slope", "Höhe der Böschung", &s.id) }.build());
        let to_rad = std::f64::consts::PI / 180.0;
        out.push(Grip { id: format!("{}:angle", s.id), entity, kind: HandleKind::Angular { axis: pose.dir(&Vec3::Y), centre: pose.at(run, 0.0, 0.0) }, anchor: pose.at(run / 2.0, 0.0, s.height / 2.0), style: GripStyle::Ring, writes: vec![write(format!("slopes[id={}].angleDeg", s.id), s.angle_deg, 1.0 / to_rad, "deg", Some(0.5))], limits: range(s.angle_deg, 1.0 / to_rad, 5.0, 85.0), names: named("Angle of slope", "Neigung der Böschung", &s.id) }.build());
        gauges::rack(gauges::SLOPES, s, "slopes", &s.id, &entity_ref(EntityKind::Solid, &s.id, "slopes"), |dy| pose.at(-1.0, dy, s.height), out);
    }
}

fn piles(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for p in d.piles.iter().filter(|p| p.diameter > 1e-6 && p.length > 1e-6) {
        let pose = layout.pose(&p.id);
        let entity = entity_ref(EntityKind::Group, &p.id, "piles");
        let path = |field: &str| format!("piles[id={}].{field}", p.id);
        let count = f64::from(p.count.max(1));
        out.push(Grip { id: format!("{}:length", p.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(0.0, 0.0, -p.length), style: GripStyle::Arrow, writes: vec![metres(path("length"), p.length, -1.0)], limits: range(p.length, -1.0, 1.0, 80.0), names: named("Length of pile", "Länge des Pfahls", &p.id) }.build());
        out.push(Grip { id: format!("{}:diameter", p.id), entity: entity.clone(), kind: HandleKind::Radial { centre: pose.at(0.0, 0.0, -p.length / 2.0) }, anchor: pose.at(p.diameter / 2.0, 0.0, -p.length / 2.0), style: GripStyle::Ring, writes: vec![metres(path("diameter"), p.diameter, 2.0)], limits: range(p.diameter, 2.0, 0.1, 3.0), names: named("Diameter of pile", "Durchmesser des Pfahls", &p.id) }.build());
        out.push(Grip { id: format!("{}:count", p.id), entity: entity.clone(), kind: HandleKind::CountStepper, anchor: pose.at(0.0, 0.0, 0.2), style: GripStyle::Diamond, writes: vec![FieldBinding { path: path("count"), map: Affine { base: count, k: 1.0 }, unit: "-".into(), quantise: Some(Quantise::Int) }], limits: range(count, 1.0, 1.0, 64.0), names: named("Number of piles in", "Pfahlanzahl in", &p.id) }.build());
        if p.settling_thickness > 1e-6 {
            out.push(Grip { id: format!("{}:settling", p.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(-p.diameter, 0.0, -p.settling_thickness.min(p.length)), style: GripStyle::Diamond, writes: vec![metres(path("settlingThickness"), p.settling_thickness, -1.0)], limits: range(p.settling_thickness, -1.0, 0.5, p.length), names: named("Settling ground along", "Setzender Boden entlang", &p.id) }.build());
        }
        out.push(choice_handle(&entity, "concrete-class", pose.at(p.diameter, 0.0, -0.3), path("concreteClass"), &concrete_class_options(), LocalizedCopy::new(format!("Concrete class of pile {}", p.id), format!("Betonfestigkeitsklasse von Pfahl {}", p.id))));
        gauges::rack(gauges::PILES, p, "piles", &p.id, &entity, |dy| pose.at(-p.diameter - 0.8, dy, 0.2), out);
        position(&p.id, &entity, &pose.origin, &pose.at(0.0, 0.0, 0.2), out);
    }
}

fn concrete_class_options() -> Vec<(String, &'static str, &'static str)> {
    PileConcreteClass::ALL.iter().map(|class| (class.token().to_string(), class.designation().unwrap_or("Not given (explicit modulus)"), class.designation().unwrap_or("Nicht angegeben (expliziter Modul)"))).collect()
}

fn embedded_walls(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for w in d.embedded_walls.iter().filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6) {
        let pose = layout.pose(&w.id);
        let entity = entity_ref(EntityKind::Solid, &w.id, "embeddedWalls");
        let t = w.thickness.max(0.05);
        let path = |field: &str| format!("embeddedWalls[id={}].{field}", w.id);
        out.push(Grip { id: format!("{}:height", w.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(t / 2.0, 0.0, w.retained_height), style: GripStyle::Arrow, writes: vec![metres(path("retainedHeight"), w.retained_height, 1.0)], limits: range(w.retained_height, 1.0, 0.5, 20.0), names: named("Retained height of", "Geländesprung von", &w.id) }.build());
        out.push(Grip { id: format!("{}:embedment", w.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(t / 2.0, 0.0, -w.embedment), style: GripStyle::Diamond, writes: vec![metres(path("embedment"), w.embedment, -1.0)], limits: range(w.embedment, -1.0, 0.5, 30.0), names: named("Embedment of", "Einbindetiefe von", &w.id) }.build());
        if w.kind != semio_s_artifact_norm_en1997_model::EmbeddedWallKind::Cantilever {
            out.push(Grip { id: format!("{}:prop", w.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(t, 0.0, w.retained_height - w.prop_depth), style: GripStyle::Ring, writes: vec![metres(path("propDepth"), w.prop_depth, -1.0)], limits: range(w.prop_depth, -1.0, 0.1, (w.retained_height - 0.1).max(0.2)), names: named("Prop level of", "Stützenlage von", &w.id) }.build());
        }
        gauges::rack(gauges::EMBEDDED_WALLS, w, "embeddedWalls", &w.id, &entity, |dy| pose.at(-0.8, dy, 0.3), out);
        position(&w.id, &entity, &pose.origin, &pose.at(t / 2.0, 0.0, w.retained_height), out);
    }
}

fn anchors(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for a in d.anchors.iter().filter(|a| a.free_length > 1e-6 && a.fixed_length > 1e-6) {
        let pose = layout.pose(&a.wall_id);
        let entity = entity_ref(EntityKind::Group, &a.id, "anchors");
        let (top, t) = d.embedded_walls.iter().find(|w| w.id == a.wall_id).map_or((0.0, 0.3), |w| (w.retained_height, w.thickness.max(0.05)));
        let theta = a.inclination_deg.to_radians();
        let (c, s) = (theta.cos(), theta.sin());
        let head = [t, top - a.head_depth];
        let dir = pose.dir(&v3(c, 0.0, -s));
        let path = |field: &str| format!("anchors[id={}].{field}", a.id);
        let along = |l: f64| pose.at(head[0] + c * l, 0.0, head[1] - s * l);
        out.push(Grip { id: format!("{}:free", a.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: dir.clone() }, anchor: along(a.free_length), style: GripStyle::Arrow, writes: vec![metres(path("freeLength"), a.free_length, 1.0)], limits: range(a.free_length, 1.0, 1.0, 40.0), names: named("Free length of", "Freie Länge von", &a.id) }.build());
        out.push(Grip { id: format!("{}:fixed", a.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: dir }, anchor: along(a.free_length + a.fixed_length), style: GripStyle::Diamond, writes: vec![metres(path("fixedLength"), a.fixed_length, 1.0)], limits: range(a.fixed_length, 1.0, 1.0, 30.0), names: named("Fixed length of", "Verpresslänge von", &a.id) }.build());
        let to_rad = std::f64::consts::PI / 180.0;
        out.push(Grip { id: format!("{}:inclination", a.id), entity, kind: HandleKind::Angular { axis: pose.dir(&Vec3::Y), centre: pose.at(head[0], 0.0, head[1]) }, anchor: along(a.free_length / 2.0), style: GripStyle::Ring, writes: vec![write(path("inclinationDeg"), a.inclination_deg, 1.0 / to_rad, "deg", Some(0.5))], limits: range(a.inclination_deg, 1.0 / to_rad, 0.0, 60.0), names: named("Inclination of", "Neigung von", &a.id) }.build());
        gauges::rack(gauges::ANCHORS, a, "anchors", &a.id, &entity_ref(EntityKind::Group, &a.id, "anchors"), |dy| pose.at(head[0] + 0.6, dy, head[1] + 0.3), out);
    }
}

fn pile_groups(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for g in d.pile_groups.iter().filter(|g| g.rows > 0 && g.columns > 0) {
        let pose = layout.pose(&g.id);
        let entity = entity_ref(EntityKind::Group, &g.id, "pileGroups");
        let path = |field: &str| format!("pileGroups[id={}].{field}", g.id);
        let half = (f64::from(g.columns) - 1.0) * g.spacing / 2.0;
        let (rows, columns) = (f64::from(g.rows), f64::from(g.columns));
        out.push(Grip { id: format!("{}:spacing", g.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::X) }, anchor: pose.at(half, 0.0, 0.2), style: GripStyle::Arrow, writes: vec![metres(path("spacing"), g.spacing, 2.0 / (columns - 1.0).max(1.0))], limits: range(g.spacing, 2.0 / (columns - 1.0).max(1.0), 0.5, 10.0), names: named("Spacing of", "Achsabstand von", &g.id) }.build());
        out.push(Grip { id: format!("{}:rows", g.id), entity: entity.clone(), kind: HandleKind::CountStepper, anchor: pose.at(0.0, half, 0.2), style: GripStyle::Diamond, writes: vec![FieldBinding { path: path("rows"), map: Affine { base: rows, k: 1.0 }, unit: "-".into(), quantise: Some(Quantise::Int) }], limits: range(rows, 1.0, 1.0, 12.0), names: named("Rows of", "Reihen von", &g.id) }.build());
        out.push(Grip { id: format!("{}:columns", g.id), entity: entity.clone(), kind: HandleKind::CountStepper, anchor: pose.at(half, 0.0, 0.4), style: GripStyle::Diamond, writes: vec![FieldBinding { path: path("columns"), map: Affine { base: columns, k: 1.0 }, unit: "-".into(), quantise: Some(Quantise::Int) }], limits: range(columns, 1.0, 1.0, 12.0), names: named("Columns of", "Spalten von", &g.id) }.build());
        gauges::rack(gauges::PILE_GROUPS, g, "pileGroups", &g.id, &entity, |dy| pose.at(-half - 1.2, dy, 0.2), out);
        position(&g.id, &entity, &pose.origin, &pose.at(0.0, 0.0, 0.2), out);
    }
}

fn layer_waters(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for w in d.layer_waters.iter().filter(|w| w.well_radius > 1e-6 && w.well_water_depth > 0.0) {
        let [x, y] = layout.well_at();
        out.push(Grip { id: format!("{}:well:level", w.id), entity: entity_ref(EntityKind::Surface, &w.id, "layerWaters"), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(x, y, -w.well_water_depth), style: GripStyle::Diamond, writes: vec![metres(format!("layerWaters[id={}].wellWaterDepth", w.id), w.well_water_depth, -1.0)], limits: range(w.well_water_depth, -1.0, 0.5, layout.depth), names: named("Water level in the well of", "Wasserspiegel im Brunnen von", &w.id) }.build());
    }
    for w in &d.layer_waters {
        out.push(
            Grip {
                id: format!("{}:piezometric", w.id),
                entity: entity_ref(EntityKind::Surface, &w.id, "layerWaters"),
                kind: HandleKind::ScalarAlong { axis: Vec3::Z },
                anchor: v3(layout.max[0], layout.min[1], -w.piezometric_depth),
                style: GripStyle::Diamond,
                writes: vec![metres(format!("layerWaters[id={}].piezometricDepth", w.id), w.piezometric_depth, -1.0)],
                limits: range(w.piezometric_depth, -1.0, 0.0, layout.depth),
                names: named("Piezometric level of", "Standrohrspiegel von", &w.id),
            }
            .build(),
        );
        let water = entity_ref(EntityKind::Surface, &w.id, "layerWaters");
        gauges::rack(gauges::LAYER_WATERS, w, "layerWaters", &w.id, &water, |dy| v3(layout.max[0] - 0.3, layout.centre()[1] + dy, -w.piezometric_depth), out);
    }
}

const PRESSURE_GAIN: f64 = 10_000.0;

fn uplift_cases(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for u in &d.uplift_cases {
        let pose = layout.pose(&u.id);
        let entity = entity_ref(EntityKind::Plate, &u.id, "upliftCases");
        let (below, top) = (-super::uplift::SLAB_DEPTH, -super::uplift::SLAB_DEPTH + super::uplift::SLAB_THICKNESS);
        let path = |field: &str| format!("upliftCases[id={}].{field}", u.id);
        for (tag, field, value, up, z, en, de) in [
            ("stabilising", "permanentStabilizing", u.permanent_stabilizing, false, top, "Stabilising permanent action of uplift case", "Stabilisierende ständige Einwirkung des Auftriebsfalls"),
            ("destabilising-g", "permanentDestabilizing", u.permanent_destabilizing, true, below, "Destabilising permanent action of uplift case", "Destabilisierende ständige Einwirkung des Auftriebsfalls"),
            ("destabilising-q", "variableDestabilizing", u.variable_destabilizing, true, below, "Destabilising variable action of uplift case", "Destabilisierende veränderliche Einwirkung des Auftriebsfalls"),
        ] {
            let reach = match tag { "stabilising" => -0.6, "destabilising-g" => 0.0, _ => 0.6 };
            let axis = pose.dir(&Vec3::Z);
            out.push(Grip { id: format!("{}:{tag}", u.id), entity: entity.clone(), kind: HandleKind::ScalarAlong { axis: if up { axis.clone() } else { v3(-axis.x, -axis.y, -axis.z) } }, anchor: pose.at(reach, 0.0, z), style: GripStyle::Arrow, writes: vec![write(path(field), value, LOAD_GAIN, "N", Some(1000.0))], limits: range(value, LOAD_GAIN, 0.0, 1.0e8), names: named(en, de, &u.id) }.build());
        }
        let h = super::layout::UPLIFT_SIDE / 2.0;
        out.push(Grip { id: format!("{}:pore-pressure", u.id), entity, kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(h * 0.6, h * 0.6, below), style: GripStyle::Diamond, writes: vec![write(path("porePressure"), u.pore_pressure, PRESSURE_GAIN, "Pa", Some(100.0))], limits: range(u.pore_pressure, PRESSURE_GAIN, 0.0, 1.0e7), names: named("Pore pressure of uplift case", "Porenwasserdruck des Auftriebsfalls", &u.id) }.build());
        gauges::rack(gauges::UPLIFT_CASES, u, "upliftCases", &u.id, &entity_ref(EntityKind::Plate, &u.id, "upliftCases"), |dy| pose.at(h * 0.6 + 0.6, dy, below), out);
    }
}

fn lab_tests(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for t in &d.lab_tests {
        let Some(layer) = d.layers.iter().find(|l| l.id == t.layer_id) else { continue };
        let pose = layout.pose(&t.id);
        let entity = entity_ref(EntityKind::Solid, &t.id, "labTests");
        let top = layer.depth_top;
        gauges::rack(gauges::LAB_TESTS, t, "labTests", &t.id, &entity, |dy| pose.at(-1.2, dy, -top), out);
        out.push(Grip { id: format!("{}:stress-increment", t.id), entity, kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(0.0, 0.0, -top), style: GripStyle::Diamond, writes: vec![write(format!("labTests[id={}].stressIncrement", t.id), t.stress_increment, PRESSURE_GAIN, "Pa", Some(100.0))], limits: range(t.stress_increment, PRESSURE_GAIN, 0.0, 1.0e7), names: named("Stress increment of laboratory series", "Spannungszuwachs der Laborversuchsreihe", &t.id) }.build());
    }
}

fn investigations(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for i in &d.investigations {
        let pose = layout.pose(&i.id);
        let entity = entity_ref(EntityKind::Solid, &i.id, "investigations");
        let path = |field: &str| format!("investigations[id={}].{field}", i.id);
        let top = pose.at(0.0, 0.0, 0.0);
        out.push(Grip { id: format!("{}:position-x", i.id), entity: entity.clone(), kind: HandleKind::Translate { axis: pose.dir(&Vec3::X) }, anchor: top.clone(), style: GripStyle::Ring, writes: vec![metres(path("positionX"), i.position_x, 1.0)], limits: (-1.0e4, 1.0e4), names: named("Position x of ground investigation", "Position x der Baugrunderkundung", &i.id) }.build());
        out.push(Grip { id: format!("{}:position-y", i.id), entity: entity.clone(), kind: HandleKind::Translate { axis: pose.dir(&Vec3::Y) }, anchor: top, style: GripStyle::Ring, writes: vec![metres(path("positionY"), i.position_y, 1.0)], limits: (-1.0e4, 1.0e4), names: named("Position y of ground investigation", "Position y der Baugrunderkundung", &i.id) }.build());
        out.push(Grip { id: format!("{}:depth", i.id), entity, kind: HandleKind::ScalarAlong { axis: pose.dir(&Vec3::Z) }, anchor: pose.at(0.0, 0.0, -i.depth), style: GripStyle::Arrow, writes: vec![metres(path("depth"), i.depth, -1.0)], limits: range(i.depth, -1.0, 0.5, 200.0), names: named("Depth of ground investigation", "Tiefe der Baugrunderkundung", &i.id) }.build());
        gauges::rack(gauges::INVESTIGATIONS, i, "investigations", &i.id, &entity_ref(EntityKind::Solid, &i.id, "investigations"), |dy| pose.at(-0.8, dy, 0.0), out);
    }
}

fn characteristic_bases(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for (i, b) in d.characteristic_bases.iter().enumerate() {
        let mid = d.layers.iter().find(|l| l.id == b.layer_id).map_or(0.5, |l| (l.depth_top + l.depth_bottom) / 2.0);
        let pose = super::entities::basis_pose(layout, i);
        let entity = entity_ref(EntityKind::Solid, &b.id, "characteristicBases");
        let path = |field: &str| format!("characteristicBases[id={}].{field}", b.id);
        let parameters: Vec<(String, &str, &str)> = [CharacteristicParameter::UndrainedStrength, CharacteristicParameter::FrictionAngle].into_iter().map(|p| (p.to_string(), super::entities::parameter_words(p).0, super::entities::parameter_words(p).1)).collect();
        let bases: Vec<(String, &str, &str)> = [ParameterBasis::Declared, ParameterBasis::Derived].into_iter().map(|p| (p.to_string(), super::entities::basis_words(p).0, super::entities::basis_words(p).1)).collect();
        out.push(choice_handle(&entity, "parameter", pose.at(0.0, 0.0, -mid + 0.35), path("parameter"), &parameters, LocalizedCopy::new(format!("Parameter of characteristic value basis {}", b.id), format!("Parameter der Grundlage {} des charakteristischen Werts", b.id))));
        out.push(choice_handle(&entity, "basis", pose.at(0.0, 0.0, -mid + 0.15), path("basis"), &bases, LocalizedCopy::new(format!("Basis of characteristic value basis {}", b.id), format!("Herkunft der Grundlage {} des charakteristischen Werts", b.id))));
    }
}

fn slip_circles(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for s in d.slopes.iter().filter(|s| s.height > 1e-6 && s.angle_deg > 1.0 && s.angle_deg < 89.0 && s.length > 1e-6) {
        let Some((circle0, tan_phi, c)) = super::slopes::proto(d, s) else { continue };
        let Some((circle, _)) = super::slopes::current_circle(s, &circle0, tan_phi, c) else { continue };
        let pose = layout.pose(&s.id);
        let entity = entity_ref(EntityKind::Solid, &s.id, "slopes");
        let path = |field: &str| format!("slopes[id={}].{field}", s.id);
        let hold = |field: &str, value: f64| write(path(field), value, 0.0, "m", Some(GRID));
        let centre = pose.at(circle.xc, mech::slope_span(s) / 2.0, circle.yc);
        let span = mech::slope_span(s);
        out.push(Grip { id: format!("{}:width", s.id), entity: entity.clone(), kind: HandleKind::FaceOffset { normal: pose.dir(&Vec3::Y) }, anchor: pose.at(0.0, span / 2.0, s.height / 2.0), style: GripStyle::Arrow, writes: vec![metres(path("width"), span, 2.0)], limits: range(span, 2.0, 1.0, 500.0), names: named("Width of slope", "Breite der Böschung", &s.id) }.build());
        out.push(Grip { id: format!("{}:circle-x", s.id), entity: entity.clone(), kind: HandleKind::Translate { axis: pose.dir(&Vec3::X) }, anchor: centre.clone(), style: GripStyle::Ring, writes: vec![metres(path("circleCentreX"), circle.xc, 1.0), hold("circleCentreZ", circle.yc), hold("circleRadius", circle.r)], limits: (-1.0e3, 1.0e3), names: named("Slip circle centre along the slope of", "Gleitkreismittelpunkt entlang der Böschung von", &s.id) }.build());
        out.push(Grip { id: format!("{}:circle-z", s.id), entity: entity.clone(), kind: HandleKind::Translate { axis: pose.dir(&Vec3::Z) }, anchor: centre.clone(), style: GripStyle::Ring, writes: vec![hold("circleCentreX", circle.xc), metres(path("circleCentreZ"), circle.yc, 1.0), hold("circleRadius", circle.r)], limits: (-1.0e3, 1.0e3), names: named("Slip circle centre height of", "Gleitkreismittelpunkt Höhe von", &s.id) }.build());
        out.push(Grip { id: format!("{}:circle-r", s.id), entity, kind: HandleKind::Radial { centre: centre.clone() }, anchor: pose.at(circle.xc, mech::slope_span(s) / 2.0, circle.yc - circle.r), style: GripStyle::Diamond, writes: vec![hold("circleCentreX", circle.xc), hold("circleCentreZ", circle.yc), metres(path("circleRadius"), circle.r, 1.0)], limits: range(circle.r, 1.0, 0.5, 4.0 * s.height.max(1.0) + 50.0), names: named("Slip circle radius of", "Gleitkreisradius von", &s.id) }.build());
    }
}

fn cases(d: &En1997Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for row in d.cases.iter().filter(|row| case_scene::declared(row)) {
        out.extend(projection_kit::design_case::forms::handles::<En1997Cases>(row, &case_scene::case_pose(&layout.pose(&row.id)), "cases"));
    }
}

/// 🕹️ Every handle of one document revision, in entity order.
pub fn all(document: &En1997Snapshot, layout: &Layout) -> Vec<Handle> {
    let mut out = Vec::new();
    layers(document, layout, &mut out);
    groundwater(document, layout, &mut out);
    footings(document, layout, &mut out);
    walls(document, layout, &mut out);
    slopes(document, layout, &mut out);
    piles(document, layout, &mut out);
    slip_circles(document, layout, &mut out);
    embedded_walls(document, layout, &mut out);
    anchors(document, layout, &mut out);
    pile_groups(document, layout, &mut out);
    layer_waters(document, layout, &mut out);
    uplift_cases(document, layout, &mut out);
    lab_tests(document, layout, &mut out);
    investigations(document, layout, &mut out);
    characteristic_bases(document, layout, &mut out);
    cases(document, layout, &mut out);
    out
}

/// 🕹️ The handles of one document entity (a derived scene id resolves to its document entity), anchored in the given layout.
pub fn build(document: &En1997Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let id = base_id(id);
    all(document, layout).into_iter().filter(|h| h.entity.id == id).collect()
}

/// 🕹️ The handles of every selected entity, in selection order, without duplicates.
pub fn handles(document: &En1997Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
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

impl Ctx<'_> {
    pub fn handles_of(&self, entity: &str) -> Vec<String> {
        self.handles.iter().filter(|h| h.entity.id == entity).map(|h| h.id.clone()).collect()
    }
}
