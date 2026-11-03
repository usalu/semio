//! 🕹️ Handles: viewport grips that bind a drag to the EN 1998 fields behind a selected entity (storey heights, masses, centres, plan dimensions, tower and vessel dimensions).

use super::scene::projection_kit::handle_kit::choice_handle;
use super::kit::{entity_kind_of, assessed_origin, foundation_centre, loc, path_of, region_drawn, region_origin, v3, DECK_WIDTH, FOOTING_DEPTH, PIER_HEIGHT, PIER_SIZE, PIER_X};
use super::ground_and_bridges::{tie_beam_ends, MIN_SOIL_LAYER, SLOPE_WIDTH, WALL_BACKFILL_WIDTH, WALL_LENGTH, WALL_THICKNESS};
use super::horizontal_elements::base_widths;
use super::layout::Layout;
use super::scene::*;
use super::layout::Placement;
use super::schema::{joints as joint_rows, part_3, wall_plan};
use super::schema::horizontal::time_history;
use super::walls;
use std::collections::BTreeMap;

/// 🔩 Upper end of the longitudinal reinforcement ratio of a pier section, the range of `reinforcementRatio` in the schema.
const MAX_REINFORCEMENT_RATIO: f64 = 0.04;

/// 🗂️ Handle ids per owning entity id, so the entity builder lists exactly the grips `all` produces.
#[derive(Default)]
pub struct Index(BTreeMap<String, Vec<String>>);

impl Index {
    pub fn of(&self, entity: &str) -> Vec<String> {
        self.0.get(entity).cloned().unwrap_or_default()
    }
}

pub fn index(handles: &[Handle]) -> Index {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for h in handles {
        map.entry(h.entity.id.clone()).or_default().push(h.id.clone());
    }
    Index(map)
}

/// 🪨 The choices of a stiffness basis grip, in enum order (EN 1998-1 4.3.1(6)).
fn stiffness_basis_options() -> Vec<(String, &'static str, &'static str)> {
    [("simplified", "Simplified (cracked-stiffness factor)", "Vereinfacht (Faktor der gerissenen Steifigkeit)"), ("declared", "Declared (effective share)", "Angegeben (wirksamer Anteil)")]
        .into_iter()
        .map(|(token, en, de)| (token.to_owned(), en, de))
        .collect()
}

fn snap(step: f64) -> SnapRule {
    SnapRule { increment: Some(step), tolerance: 0.0, ..SnapRule::NONE }
}

struct Grip<'a> {
    id: String,
    entity: &'a str,
    entity_kind: EntityKind,
    path: String,
    kind: HandleKind,
    anchor: Vec3,
    style: GripStyle,
    field: String,
    unit: &'static str,
    base: f64,
    k: f64,
    step: f64,
    limits: (f64, f64),
    name: (String, String),
}

fn handle(g: Grip<'_>) -> Handle {
    Handle {
        id: g.id,
        entity: EntityRef::new(g.entity_kind, g.entity, g.path),
        kind: g.kind,
        anchor: g.anchor,
        grip: g.style,
        writes: vec![FieldBinding::new(g.field, Affine { base: g.base, k: g.k }, g.unit)],
        snap: snap(g.step),
        limits: Some(Interval::new((g.limits.0 - g.base) / g.k, (g.limits.1 - g.base) / g.k)),
        accessible_name: loc(&g.name.0, &g.name.1),
    }
}

/// 🧲️ The move grips of every placed entity; a drag writes `placements[id=ENTITY].x|y`, the first drag of an entity without a row inserts it as one undo step.
fn move_handles(document: &model::En1998Snapshot, layout: &Layout) -> Vec<Handle> {
    let stored: Vec<_> = document.placements.iter().map(|p| Placement { entity_id: p.id.clone(), origin: Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    let placed: Vec<(&str, EntityKind, &str)> = document
        .buildings
        .iter()
        .map(|e| (e.id.as_str(), EntityKind::Plate, "buildings"))
        .chain(document.retaining_walls.iter().map(|e| (e.id.as_str(), EntityKind::Solid, "retainingWalls")))
        .chain(document.bridges.iter().map(|e| (e.id.as_str(), EntityKind::Member, "bridges")))
        .chain(document.silos.iter().map(|e| (e.id.as_str(), EntityKind::Shell, "silos")))
        .chain(document.tanks.iter().map(|e| (e.id.as_str(), EntityKind::Shell, "tanks")))
        .chain(document.towers.iter().map(|e| (e.id.as_str(), EntityKind::Member, "towers")))
        .chain(document.slopes.iter().map(|e| (e.id.as_str(), EntityKind::Solid, "slopes")))
        .collect();
    placed.into_iter().flat_map(|(id, kind, collection)| super::scene::projection_kit::handle_kit::move_handles(&EntityRef::new(kind, id, path_of(collection, id)), id, &layout.origin(id), &stored, 0.0, false)).collect()
}

/// 🕹️ Every grip of the document at its layout position.
pub fn all(document: &model::En1998Snapshot, layout: &Layout) -> Vec<Handle> {
    let mut out = move_handles(document, layout);
    for b in &document.buildings {
        let o = layout.origin(&b.id);
        let bp = path_of("buildings", &b.id);
        out.push(handle(Grip { id: format!("{}:lx", b.id), entity: &b.id, entity_kind: EntityKind::Plate, path: bp.clone(), kind: HandleKind::FaceOffset { normal: Vec3::X }, anchor: v3(o.x + b.plan_width_m, o.y + b.plan_length_m / 2.0, o.z), style: GripStyle::Arrow, field: format!("{bp}.planWidthM"), unit: "m", base: b.plan_width_m, k: 1.0, step: 0.1, limits: (3.0, 200.0), name: (format!("Plan width of {}", b.name), format!("Grundrissbreite von {}", b.name)) }));
        out.push(handle(Grip { id: format!("{}:ly", b.id), entity: &b.id, entity_kind: EntityKind::Plate, path: bp.clone(), kind: HandleKind::FaceOffset { normal: Vec3::Y }, anchor: v3(o.x + b.plan_width_m / 2.0, o.y + b.plan_length_m, o.z), style: GripStyle::Arrow, field: format!("{bp}.planLengthM"), unit: "m", base: b.plan_length_m, k: 1.0, step: 0.1, limits: (3.0, 200.0), name: (format!("Plan length of {}", b.name), format!("Grundrissl\u{e4}nge von {}", b.name)) }));
        if b.nonstructural_weight_n > 0.0 {
            let total: f64 = b.storeys.iter().map(|s| s.height_m).sum();
            let id = format!("{}:nonstructural", b.id);
            out.push(handle(Grip { id: format!("{id}:z"), entity: &id, entity_kind: EntityKind::Solid, path: bp.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x + b.plan_width_m / 2.0, o.y + b.plan_length_m / 2.0, o.z + b.nonstructural_height_m + 1.5), style: GripStyle::Arrow, field: format!("{bp}.nonstructuralHeightM"), unit: "m", base: b.nonstructural_height_m, k: 1.0, step: 0.1, limits: (0.0, total.max(0.1)), name: (format!("Height of the non-structural element of {}", b.name), format!("H\u{f6}he des nichttragenden Bauteils von {}", b.name)) }));
        }
        let mut z = 0.0;
        for s in &b.storeys {
            z += s.height_m;
            let sp = format!("{bp}.storeys[id={}]", s.id);
            let slab = format!("{}:{}", b.id, s.id);
            out.push(handle(Grip { id: format!("{slab}:h"), entity: &slab, entity_kind: EntityKind::Plate, path: sp.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x + b.plan_width_m, o.y, o.z + z), style: GripStyle::Arrow, field: format!("{sp}.heightM"), unit: "m", base: s.height_m, k: 1.0, step: 0.05, limits: (2.4, 8.0), name: (format!("Height of storey {}", s.id), format!("H\u{f6}he von Geschoss {}", s.id)) }));
            let at = v3(o.x + s.centre_of_mass_x_m, o.y + s.centre_of_mass_y_m, o.z + z);
            out.extend(super::structure_views::variables::marks(&slab, &sp, &s.variables, s.permanent_gk_n, &at).iter().map(|mark| mark.grip()));
            let mass = format!("{slab}:mass");
            out.push(handle(Grip { id: format!("{mass}:g"), entity: &mass, entity_kind: EntityKind::Load, path: sp.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(at.x, at.y, at.z + 1.0), style: GripStyle::Diamond, field: format!("{sp}.permanentGkN"), unit: "N", base: s.permanent_gk_n, k: (s.permanent_gk_n * 0.25).max(1.0e3), step: 1.0e3, limits: (0.0, (s.permanent_gk_n * 5.0).max(1.0e6)), name: (format!("Permanent action of storey {}", s.id), format!("St\u{e4}ndige Einwirkung von Geschoss {}", s.id)) }));
            let com = format!("{slab}:com");
            let cos = format!("{slab}:cos");
            let driven = wall_plan::stiffness_centre_of(&b.members, s).2;
            for (entity, name, fx, fy, cx, cy) in [(&com, ("centre of mass", "Massenmittelpunkt"), "centreOfMassXM", "centreOfMassYM", s.centre_of_mass_x_m, s.centre_of_mass_y_m), (&cos, ("centre of stiffness", "Steifigkeitsmittelpunkt"), "centreOfStiffnessXM", "centreOfStiffnessYM", s.centre_of_stiffness_x_m, s.centre_of_stiffness_y_m)] {
                if driven && entity == &cos {
                    continue;
                }
                let p = v3(o.x + cx, o.y + cy, o.z + z);
                out.push(handle(Grip { id: format!("{entity}:x"), entity, entity_kind: EntityKind::Solid, path: sp.clone(), kind: HandleKind::Translate { axis: Vec3::X }, anchor: p.clone(), style: GripStyle::Arrow, field: format!("{sp}.{fx}"), unit: "m", base: cx, k: 1.0, step: 0.05, limits: (0.0, b.plan_width_m.max(1.0)), name: (format!("{} of storey {} in x", name.0, s.id), format!("{} von Geschoss {} in x", name.1, s.id)) }));
                out.push(handle(Grip { id: format!("{entity}:y"), entity, entity_kind: EntityKind::Solid, path: sp.clone(), kind: HandleKind::Translate { axis: Vec3::Y }, anchor: p, style: GripStyle::Arrow, field: format!("{sp}.{fy}"), unit: "m", base: cy, k: 1.0, step: 0.05, limits: (0.0, b.plan_length_m.max(1.0)), name: (format!("{} of storey {} in y", name.0, s.id), format!("{} von Geschoss {} in y", name.1, s.id)) }));
            }
        }
        let basis_entity = EntityRef::new(EntityKind::Plate, &b.id, bp.clone());
        out.push(choice_handle(&basis_entity, "stiffness-basis", v3(o.x + b.plan_width_m, o.y, o.z + z), format!("{bp}.storeyStiffnessBasis"), &stiffness_basis_options(), loc(&format!("Stiffness basis of building {}", b.id), &format!("Steifigkeitsbasis des Gebäudes {}", b.id))));
    }
    for b in &document.buildings {
        let o = layout.origin(&b.id);
        let bp = path_of("buildings", &b.id);
        let reach = b.plan_width_m.hypot(b.plan_length_m).max(1.0);
        for (m, index) in walls::rows_of(b) {
            let storey = &b.storeys[index];
            let z = o.z + b.storeys.iter().take(index).map(|s| s.height_m).sum::<f64>() + 0.5 * storey.height_m;
            let (id, p) = (walls::body_id(&b.id, &m.id, &storey.id), format!("{bp}.members[id={}]", m.id));
            let (s, c) = m.wall_direction_rad.sin_cos();
            let start = v3(o.x + m.wall_start_x_m, o.y + m.wall_start_y_m, z);
            let end = v3(start.x + m.wall_length_m * c, start.y + m.wall_length_m * s, z);
            let grip = |tag: &str, kind: HandleKind, anchor: Vec3, style: GripStyle, field: &str, unit: &'static str, base: f64, limits: (f64, f64), step: f64, name: (String, String)| {
                handle(Grip { id: format!("{id}:{tag}"), entity: &id, entity_kind: EntityKind::Solid, path: p.clone(), kind, anchor, style, field: format!("{p}.{field}"), unit, base, k: 1.0, step, limits, name })
            };
            let label = |en: &str, de: &str| (format!("{en} of wall {} at storey {}", m.id, storey.id), format!("{de} der Wand {} im Geschoss {}", m.id, storey.id));
            out.push(grip("start-x", HandleKind::Translate { axis: Vec3::X }, start.clone(), GripStyle::Arrow, "wallStartXM", "m", m.wall_start_x_m, (0.0, b.plan_width_m.max(1.0)), 0.05, label("start x", "Anfang x")));
            out.push(grip("start-y", HandleKind::Translate { axis: Vec3::Y }, start.clone(), GripStyle::Arrow, "wallStartYM", "m", m.wall_start_y_m, (0.0, b.plan_length_m.max(1.0)), 0.05, label("start y", "Anfang y")));
            out.push(grip("length", HandleKind::ScalarAlong { axis: v3(c, s, 0.0) }, end.clone(), GripStyle::Arrow, "wallLengthM", "m", m.wall_length_m, (0.1, reach), 0.05, label("length", "Länge")));
            out.push(grip("direction", HandleKind::Angular { axis: Vec3::Z, centre: start }, end, GripStyle::Ring, "wallDirectionRad", "rad", m.wall_direction_rad, (-std::f64::consts::PI, std::f64::consts::PI), 0.01, label("direction", "Richtung")));
            let (basis_en, basis_de) = label("stiffness basis", "Steifigkeitsbasis");
            out.push(choice_handle(&EntityRef::new(EntityKind::Solid, &id, p.clone()), "stiffness-basis", v3(o.x + m.wall_start_x_m, o.y + m.wall_start_y_m, z + 0.5), format!("{p}.effectiveStiffnessBasis"), &stiffness_basis_options(), loc(&basis_en, &basis_de)));
        }
    }
    for j in &document.joints {
        let Some(b) = document.buildings.iter().find(|b| b.id == j.building_id) else { continue };
        let Some(top) = joint_rows::elevation_m(b, j) else { continue };
        let (o, p) = (layout.origin(&b.id), path_of("joints", &j.id));
        let at = v3(o.x + j.x_m, o.y + j.y_m, o.z + top);
        for (tag, axis, field, base, limit, en, de) in [("x", Vec3::X, "xM", j.x_m, b.plan_width_m, "x", "x"), ("y", Vec3::Y, "yM", j.y_m, b.plan_length_m, "y", "y")] {
            out.push(handle(Grip { id: format!("{}:{tag}", j.id), entity: &j.id, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::Translate { axis }, anchor: at.clone(), style: GripStyle::Arrow, field: format!("{p}.{field}"), unit: "m", base, k: 1.0, step: 0.05, limits: (0.0, limit.max(1.0)), name: (format!("Plan {en} of joint {}", j.id), format!("Grundriss-{de} von Knoten {}", j.id)) }));
        }
    }
    for b in &document.buildings {
        let o = layout.origin(&b.id);
        for (index, a) in document.assessments.iter().filter(|a| a.supported_building_id == b.id).enumerate() {
            let Some(member) = part_3::RcMember::of(a) else { continue };
            let base = assessed_origin(&o, b, index);
            let (id, p) = (format!("{}:member", a.id), path_of("assessments", &a.id));
            out.push(handle(Grip { id: format!("{id}:depth"), entity: &id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x + member.l_v, base.y, base.z + member.h / 2.0), style: GripStyle::Arrow, field: format!("{p}.sectionDepthM"), unit: "m", base: member.h, k: 2.0, step: 0.01, limits: (0.1, 3.0), name: (format!("Section depth of {}", a.id), format!("Querschnittsh\u{f6}he von {}", a.id)) }));
            out.push(handle(Grip { id: format!("{id}:span"), entity: &id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: v3(base.x + member.l_v, base.y, base.z), style: GripStyle::Arrow, field: format!("{p}.shearSpanM"), unit: "m", base: member.l_v, k: 1.0, step: 0.05, limits: (0.2, 15.0), name: (format!("Shear span of {}", a.id), format!("Schubspannweite von {}", a.id)) }));
            let top = base.z + member.h / 2.0;
            let steel_z = top - member.d;
            let section = |tag: &str, kind: HandleKind, anchor: Vec3, field: &str, unit: &'static str, value: f64, k: f64, step: f64, limits: (f64, f64), name: (String, String)| handle(Grip { id: format!("{id}:{tag}"), entity: &id, entity_kind: EntityKind::Member, path: p.clone(), kind, anchor, style: GripStyle::Arrow, field: format!("{p}.{field}"), unit, base: value, k, step, limits, name });
            out.push(section("width", HandleKind::ScalarAlong { axis: Vec3::Y }, v3(base.x + member.l_v, base.y + member.b / 2.0, base.z), "sectionWidthM", "m", member.b, 2.0, 0.01, (0.1, 3.0), (format!("Section width of {}", a.id), format!("Querschnittsbreite von {}", a.id))));
            out.push(section("effective-depth", HandleKind::ScalarAlong { axis: v3(0.0, 0.0, -1.0) }, v3(base.x + 0.25 * member.l_v, base.y, steel_z), "effectiveDepthM", "m", member.d, 1.0, 0.005, (0.05, (member.h - 0.005).max(0.06)), (format!("Effective depth of {}", a.id), format!("Nutzh\u{f6}he von {}", a.id))));
            out.push(section("bar-diameter", HandleKind::ScalarAlong { axis: Vec3::Y }, v3(base.x + 0.5 * member.l_v, base.y + member.b / 4.0, steel_z), "barDiameterMm", "mm", a.bar_diameter_mm, 100.0, 1.0, (6.0, 50.0), (format!("Bar diameter of {}", a.id), format!("St\u{e4}bdurchmesser von {}", a.id))));
            out.push(section("tension-steel", HandleKind::ScalarAlong { axis: Vec3::Y }, v3(base.x + 0.75 * member.l_v, base.y + member.b / 4.0, steel_z), "tensionSteelAreaM2", "m2", member.a_s1, 0.004, 0.00001, (0.00001, 0.1), (format!("Tension steel area of {}", a.id), format!("Zugbewehrungsfl\u{e4}che von {}", a.id))));
            out.push(section("shear-demand", HandleKind::ScalarAlong { axis: Vec3::Z }, v3(base.x + member.l_v, base.y, base.z + member.h / 2.0 + 0.3), "shearForceDemandN", "N", a.shear_force_demand_n, 1.0e5, 1000.0, (0.0, 1.0e8), (format!("Shear force demand of {}", a.id), format!("Querkraftbeanspruchung von {}", a.id))));
        }
    }
    for b in &document.buildings {
        let o = layout.origin(&b.id);
        for (index, r) in document.critical_regions.iter().filter(|r| r.building_id == b.id && region_drawn(r)).enumerate() {
            let base = region_origin(&o, index);
            let (id, p) = (format!("{}:region", r.id), path_of("criticalRegions", &r.id));
            let top = base.z + r.section_depth_m / 2.0;
            let grip = |tag: &str, axis: Vec3, anchor: Vec3, field: &str, value: f64, k: f64, step: f64, limits: (f64, f64), name: (String, String)| handle(Grip { id: format!("{id}:{tag}"), entity: &id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis }, anchor, style: GripStyle::Arrow, field: format!("{p}.{field}"), unit: "m", base: value, k, step, limits, name });
            out.push(grip("hoop-spacing", Vec3::X, v3(base.x + r.hoop_spacing_m, base.y, top + 0.3), "hoopSpacingM", r.hoop_spacing_m, 1.0, 0.005, (0.03, 0.5), (format!("Hoop spacing of {}", r.id), format!("B\u{fc}gelabstand von {}", r.id))));
            out.push(grip("hoop-diameter", Vec3::Z, v3(base.x + 0.5 * r.hoop_spacing_m, base.y, base.z + r.core_depth_m / 2.0 + r.hoop_diameter_m), "hoopDiameterM", r.hoop_diameter_m, 0.05, 0.001, (0.006, 0.05), (format!("Hoop diameter of {}", r.id), format!("B\u{fc}geldurchmesser von {}", r.id))));
            out.push(grip("hooped-length", Vec3::X, v3(base.x + r.hooped_length_m, base.y, base.z), "hoopedLengthM", r.hooped_length_m, 1.0, 0.05, (0.05, r.clear_length_m.max(0.1)), (format!("Hooped length of {}", r.id), format!("B\u{fc}gell\u{e4}nge von {}", r.id))));
        }
    }
    for b in document.buildings.iter().filter(|b| b.plan_width_m > 0.0 && b.plan_length_m > 0.0 && !b.storeys.is_empty()) {
        let o = layout.origin(&b.id);
        let (bx, by) = base_widths(b);
        let (bp, base) = (path_of("buildings", &b.id), format!("{}:base", b.id));
        let (x0, y0) = (o.x + (b.plan_width_m - bx) / 2.0, o.y + (b.plan_length_m - by) / 2.0);
        out.push(handle(Grip { id: format!("{base}:bx"), entity: &base, entity_kind: EntityKind::Plate, path: bp.clone(), kind: HandleKind::FaceOffset { normal: Vec3::X }, anchor: v3(x0 + bx, y0 + by / 2.0, o.z - 0.2), style: GripStyle::Arrow, field: format!("{bp}.overturningBaseXM"), unit: "m", base: bx, k: 1.0, step: 0.1, limits: (0.5, 200.0), name: (format!("Base width in x of {}", b.name), format!("Fundamentbreite in x von {}", b.name)) }));
        out.push(handle(Grip { id: format!("{base}:by"), entity: &base, entity_kind: EntityKind::Plate, path: bp.clone(), kind: HandleKind::FaceOffset { normal: Vec3::Y }, anchor: v3(x0 + bx / 2.0, y0 + by, o.z - 0.2), style: GripStyle::Arrow, field: format!("{bp}.overturningBaseYM"), unit: "m", base: by, k: 1.0, step: 0.1, limits: (0.5, 200.0), name: (format!("Base width in y of {}", b.name), format!("Fundamentbreite in y von {}", b.name)) }));
    }
    for d in &document.diaphragms {
        let Some(b) = document.buildings.iter().find(|b| b.id == d.building_id) else { continue };
        let Some(index) = b.storeys.iter().position(|s| s.id == d.storey_id) else { continue };
        let o = layout.origin(&b.id);
        let top: f64 = b.storeys.iter().take(index + 1).map(|s| s.height_m).sum();
        let p = path_of("diaphragms", &d.id);
        out.push(handle(Grip { id: format!("{}:t", d.id), entity: &d.id, entity_kind: EntityKind::Plate, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x + b.plan_width_m, o.y + b.plan_length_m, o.z + top + d.thickness_m.max(0.05)), style: GripStyle::Arrow, field: format!("{p}.thicknessM"), unit: "m", base: d.thickness_m, k: 1.0, step: 0.01, limits: (0.05, 1.0), name: (format!("Thickness of diaphragm {}", d.id), format!("Dicke der Scheibe {}", d.id)) }));
    }
    for b in &document.bridges {
        let anchor = super::ground_and_bridges::variables_anchor(document, b, &layout.origin(&b.id));
        out.extend(super::structure_views::variables::marks(&b.id, &path_of("bridges", &b.id), &b.variables, b.permanent_gk_n, &anchor).iter().map(|mark| mark.grip()));
    }
    for b in &document.bridges {
        let deck = super::ground_and_bridges::deck_origin(document, b, &layout.origin(&b.id));
        let p = path_of("bridges", &b.id);
        let class = b.importance_class.round().clamp(1.0, 3.0);
        out.push(handle(Grip { id: format!("{}:importance", b.id), entity: &b.id, entity_kind: EntityKind::Plate, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(deck.x, deck.y + DECK_WIDTH / 2.0, deck.z + super::ground_and_bridges::DECK_THICKNESS + super::ground_and_bridges::BADGE_RISE + super::ground_and_bridges::BADGE_PER_CLASS * class), style: GripStyle::Diamond, field: format!("{p}.importanceClass"), unit: "1", base: class, k: 1.0 / super::ground_and_bridges::BADGE_PER_CLASS, step: 1.0, limits: (1.0, 3.0), name: (format!("Importance class of {}", b.id), format!("Bedeutungsklasse von {}", b.id)) }));
        if b.vertical_period_s > 0.0 {
            out.push(handle(Grip { id: format!("{}:vertical-period", b.id), entity: &b.id, entity_kind: EntityKind::Plate, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(deck.x, deck.y + DECK_WIDTH / 2.0, deck.z + super::ground_and_bridges::DECK_THICKNESS + super::ground_and_bridges::VERTICAL_RISE + super::ground_and_bridges::VERTICAL_PER_SECOND * b.vertical_period_s), style: GripStyle::Diamond, field: format!("{p}.verticalPeriodS"), unit: "s", base: b.vertical_period_s, k: 1.0 / super::ground_and_bridges::VERTICAL_PER_SECOND, step: 0.01, limits: (0.01, 2.0), name: (format!("Vertical period T_v of {}", b.id), format!("Vertikalperiode T_v von {}", b.id)) }));
        }
    }
    for b in &document.bridges {
        if b.pier_height_m <= 0.0 || document.piers.iter().any(|p| p.bridge_id == b.id) || document.bearings.iter().any(|k| k.bridge_id == b.id) {
            continue;
        }
        let o = layout.origin(&b.id);
        let p = path_of("bridges", &b.id);
        for (tag, x) in PIER_X {
            let base = v3(o.x + x, o.y + DECK_WIDTH / 2.0, o.z);
            let pier = format!("{}:pier-{tag}", b.id);
            out.push(handle(Grip { id: format!("{pier}:h"), entity: &pier, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x, base.y, base.z + b.pier_height_m), style: GripStyle::Arrow, field: format!("{p}.pierHeightM"), unit: "m", base: b.pier_height_m, k: 1.0, step: 0.1, limits: (1.0, 60.0), name: (format!("Pier height of {}", b.id), format!("Pfeilerh\u{f6}he von {}", b.id)) }));
            if b.pier_moment_rd_nm > 0.0 {
                let moment = format!("{pier}:mrd");
                out.push(handle(Grip { id: format!("{moment}:g"), entity: &moment, entity_kind: EntityKind::Load, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x, base.y, base.z + 1.0), style: GripStyle::Diamond, field: format!("{p}.pierMomentRdNm"), unit: "N*m", base: b.pier_moment_rd_nm, k: (b.pier_moment_rd_nm * 0.25).max(1.0e3), step: 1.0e3, limits: (0.0, (b.pier_moment_rd_nm * 5.0).max(1.0e6)), name: (format!("Plastic hinge moment of {}", b.id), format!("Plastisches Gelenkmoment von {}", b.id)) }));
            }
            if b.pier_shear_rd_n > 0.0 {
                let shear = format!("{pier}:vrd");
                out.push(handle(Grip { id: format!("{shear}:g"), entity: &shear, entity_kind: EntityKind::Load, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x, base.y + 1.2, base.z + b.pier_height_m * 0.5 + 1.0), style: GripStyle::Diamond, field: format!("{p}.pierShearRdN"), unit: "N", base: b.pier_shear_rd_n, k: (b.pier_shear_rd_n * 0.25).max(1.0e3), step: 1.0e3, limits: (0.0, (b.pier_shear_rd_n * 5.0).max(1.0e6)), name: (format!("Shear resistance of {}", b.id), format!("Querkrafttragf\u{e4}higkeit von {}", b.id)) }));
            }
        }
    }
    for pier in &document.piers {
        let Some(b) = document.bridges.iter().find(|b| b.id == pier.bridge_id) else { continue };
        let o = layout.origin(&b.id);
        let p = path_of("piers", &pier.id);
        let height = if pier.height_m > 0.0 { pier.height_m } else { PIER_HEIGHT };
        let base = v3(o.x + pier.x_m, o.y + DECK_WIDTH / 2.0, o.z);
        out.push(handle(Grip { id: format!("{}:h", pier.id), entity: &pier.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x, base.y, base.z + height), style: GripStyle::Arrow, field: format!("{p}.heightM"), unit: "m", base: pier.height_m, k: 1.0, step: 0.1, limits: (1.0, 60.0), name: (format!("Height of pier {}", pier.id), format!("Pfeilerh\u{f6}he von {}", pier.id)) }));
        out.push(handle(Grip { id: format!("{}:x", pier.id), entity: &pier.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::Translate { axis: Vec3::X }, anchor: v3(base.x, base.y - 1.0, base.z + 0.5), style: GripStyle::Arrow, field: format!("{p}.xM"), unit: "m", base: pier.x_m, k: 1.0, step: 0.1, limits: (0.0, 400.0), name: (format!("Position of pier {}", pier.id), format!("Lage von Pfeiler {}", pier.id)) }));
        let bp = path_of("bridges", &b.id);
        if pier.stiffness_basis == model::PierStiffnessBasis::Computed {
            out.push(handle(Grip { id: format!("{}:rho", pier.id), entity: &pier.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x + pier.section_b_m * 0.5, base.y, base.z + height * (pier.reinforcement_ratio / MAX_REINFORCEMENT_RATIO).clamp(0.01, 1.0)), style: GripStyle::Diamond, field: format!("{p}.reinforcementRatio"), unit: "1", base: pier.reinforcement_ratio, k: MAX_REINFORCEMENT_RATIO / height, step: 0.001, limits: (0.0, MAX_REINFORCEMENT_RATIO), name: (format!("Longitudinal reinforcement ratio of pier {}", pier.id), format!("L\u{e4}ngsbewehrungsgrad von Pfeiler {}", pier.id)) }));
        } else {
            out.push(handle(Grip { id: format!("{}:cracked", pier.id), entity: &pier.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x + pier.section_b_m * 0.5, base.y, base.z + height * b.pier_cracked_inertia_ratio.clamp(0.01, 1.0)), style: GripStyle::Diamond, field: format!("{bp}.pierCrackedInertiaRatio"), unit: "1", base: b.pier_cracked_inertia_ratio, k: 1.0 / height, step: 0.01, limits: (0.01, 1.0), name: (format!("Cracked stiffness share I_eff / I_g of the piers of {}", b.id), format!("Gerissener Steifigkeitsanteil I_eff / I_g der Pfeiler von {}", b.id)) }));
        }
        if pier.moment_rd_nm > 0.0 {
            let moment = format!("{}:mrd", pier.id);
            out.push(handle(Grip { id: format!("{moment}:g"), entity: &moment, entity_kind: EntityKind::Load, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x, base.y, base.z + 1.0), style: GripStyle::Diamond, field: format!("{p}.momentRdNm"), unit: "N*m", base: pier.moment_rd_nm, k: (pier.moment_rd_nm * 0.25).max(1.0e3), step: 1.0e3, limits: (0.0, (pier.moment_rd_nm * 5.0).max(1.0e6)), name: (format!("Plastic hinge moment of {}", pier.id), format!("Plastisches Gelenkmoment von {}", pier.id)) }));
        }
        if pier.shear_rd_n > 0.0 {
            let shear = format!("{}:vrd", pier.id);
            let depth = if pier.section_h_m > 0.0 { pier.section_h_m } else { PIER_SIZE };
            out.push(handle(Grip { id: format!("{shear}:g"), entity: &shear, entity_kind: EntityKind::Load, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(base.x, base.y + depth, base.z + height * 0.5 + 1.0), style: GripStyle::Diamond, field: format!("{p}.shearRdN"), unit: "N", base: pier.shear_rd_n, k: (pier.shear_rd_n * 0.25).max(1.0e3), step: 1.0e3, limits: (0.0, (pier.shear_rd_n * 5.0).max(1.0e6)), name: (format!("Shear resistance of {}", pier.id), format!("Querkrafttragf\u{e4}higkeit von {}", pier.id)) }));
        }
    }
    for bearing in &document.bearings {
        let Some(b) = document.bridges.iter().find(|b| b.id == bearing.bridge_id) else { continue };
        let o = layout.origin(&b.id);
        let pier = document.piers.iter().find(|p| p.id == bearing.pier_id && p.bridge_id == b.id);
        let x = pier.map_or(bearing.x_m, |p| p.x_m);
        let z = o.z + pier.map_or_else(|| document.piers.iter().filter(|p| p.bridge_id == b.id).map(|p| if p.height_m > 0.0 { p.height_m } else { PIER_HEIGHT }).fold(PIER_HEIGHT, f64::max), |p| if p.height_m > 0.0 { p.height_m } else { PIER_HEIGHT });
        let p = path_of("bearings", &bearing.id);
        out.push(handle(Grip { id: format!("{}:d", bearing.id), entity: &bearing.id, entity_kind: EntityKind::Support, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: v3(o.x + x + bearing.d_rd_m, o.y + DECK_WIDTH / 2.0, z), style: GripStyle::Arrow, field: format!("{p}.dRdM"), unit: "m", base: bearing.d_rd_m, k: 1.0, step: 0.005, limits: (0.0, 2.0), name: (format!("Displacement capacity of bearing {}", bearing.id), format!("Verschiebungskapazit\u{e4}t von Lager {}", bearing.id)) }));
    }
    for f in &document.foundations {
        let centre = foundation_centre(document, layout, f);
        let p = path_of("foundations", &f.id);
        let top = centre.z - FOOTING_DEPTH;
        if f.pile_diameter_m > 0.0 && f.pile_length_m > 0.0 {
            let pile = format!("{}:pile", f.id);
            out.push(handle(Grip { id: format!("{pile}:l"), entity: &pile, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::ScalarAlong { axis: v3(0.0, 0.0, -1.0) }, anchor: v3(centre.x, centre.y, top - f.pile_length_m), style: GripStyle::Arrow, field: format!("{p}.pileLengthM"), unit: "m", base: f.pile_length_m, k: 1.0, step: 0.1, limits: (1.0, 80.0), name: (format!("Pile length of {}", f.id), format!("Pfahll\u{e4}nge von {}", f.id)) }));
            if f.pile_interface_depth_m > 0.0 {
                let interface = format!("{}:pile-interface", f.id);
                out.push(handle(Grip { id: format!("{interface}:z"), entity: &interface, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::ScalarAlong { axis: v3(0.0, 0.0, -1.0) }, anchor: v3(centre.x, centre.y, top - f.pile_interface_depth_m), style: GripStyle::Arrow, field: format!("{p}.pileInterfaceDepthM"), unit: "m", base: f.pile_interface_depth_m, k: 1.0, step: 0.1, limits: (0.5, f.pile_length_m.max(1.0)), name: (format!("Interface depth of {}", f.id), format!("Tiefe des Steifigkeitssprungs von {}", f.id)) }));
            }
        }
        let layered = document.soil_layers.iter().any(|l| l.foundation_id == f.id);
        if f.liq_depth_m > 0.0 && f.liq_n1_60 > 0.0 && !layered {
            let layer = format!("{}:liquefiable-layer", f.id);
            let thickness = if f.liq_layer_thickness_m > 0.0 { f.liq_layer_thickness_m } else { 0.5 };
            out.push(handle(Grip { id: format!("{layer}:t"), entity: &layer, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(centre.x, centre.y, centre.z - f.liq_depth_m + thickness / 2.0), style: GripStyle::Arrow, field: format!("{p}.liqLayerThicknessM"), unit: "m", base: thickness, k: 2.0, step: 0.1, limits: (0.1, 30.0), name: (format!("Layer thickness of {}", f.id), format!("Schichtdicke von {}", f.id)) }));
        }
    }
    for f in &document.foundations {
        let centre = foundation_centre(document, layout, f);
        let mut bottom = 0.0;
        for row in document.soil_layers.iter().filter(|l| l.foundation_id == f.id) {
            let thickness = row.thickness_m.max(MIN_SOIL_LAYER);
            bottom += thickness;
            let p = path_of("soilLayers", &row.id);
            out.push(handle(Grip { id: format!("{}:interface", row.id), entity: &row.id, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::ScalarAlong { axis: v3(0.0, 0.0, -1.0) }, anchor: v3(centre.x, centre.y, centre.z - bottom), style: GripStyle::Arrow, field: format!("{p}.thicknessM"), unit: "m", base: row.thickness_m, k: 1.0, step: 0.1, limits: (MIN_SOIL_LAYER, 60.0), name: (format!("Lower interface of soil layer {}", row.id), format!("Untere Schichtgrenze von Bodenschicht {}", row.id)) }));
        }
    }
    for t in &document.towers {
        let o = layout.origin(&t.id);
        let p = path_of("towers", &t.id);
        out.extend(super::structure_views::variables::marks(&t.id, &p, &t.variables, t.permanent_gk_n, &v3(o.x, o.y, o.z + t.height_m.max(0.1))).iter().map(|mark| mark.grip()));
        out.push(handle(Grip { id: format!("{}:h", t.id), entity: &t.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x, o.y, o.z + t.height_m), style: GripStyle::Arrow, field: format!("{p}.heightM"), unit: "m", base: t.height_m, k: 1.0, step: 0.5, limits: (3.0, 300.0), name: (format!("Height of tower {}", t.id), format!("H\u{f6}he von Turm {}", t.id)) }));
        let base = format!("{}:base", t.id);
        out.push(handle(Grip { id: format!("{base}:k"), entity: &base, entity_kind: EntityKind::Support, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: v3(o.x + 1.0, o.y, o.z), style: GripStyle::Diamond, field: format!("{p}.baseRotationalStiffnessNm"), unit: "N*m", base: t.base_rotational_stiffness_nm, k: (t.base_rotational_stiffness_nm * 0.25).max(1.0e7), step: 1.0e6, limits: (0.0, (t.base_rotational_stiffness_nm * 5.0).max(1.0e11)), name: (format!("Foundation rotational stiffness of {}", t.id), format!("Drehfedersteifigkeit der Gr\u{fc}ndung von {}", t.id)) }));
    }
    for (collection, items) in [("silos", document.silos.iter().map(|s| (s.id.as_str(), s.height_m, s.radius_m)).collect::<Vec<_>>()), ("tanks", document.tanks.iter().map(|s| (s.id.as_str(), s.height_m, s.radius_m)).collect())] {
        for (id, height, radius) in items {
            let o = layout.origin(id);
            let p = path_of(collection, id);
            out.push(handle(Grip { id: format!("{id}:h"), entity: id, entity_kind: EntityKind::Shell, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x, o.y, o.z + height), style: GripStyle::Arrow, field: format!("{p}.heightM"), unit: "m", base: height, k: 1.0, step: 0.1, limits: (1.0, 100.0), name: (format!("Height of {id}"), format!("H\u{f6}he von {id}")) }));
            if collection == "tanks" {
                let thickness = document.tanks.iter().find(|t| t.id == id).map_or(0.0, |t| t.shell_thickness_m);
                out.push(handle(Grip { id: format!("{id}:t"), entity: id, entity_kind: EntityKind::Shell, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: v3(o.x - radius - 0.5, o.y, o.z), style: GripStyle::Diamond, field: format!("{p}.shellThicknessM"), unit: "m", base: thickness, k: 0.02, step: 0.0005, limits: (0.002, 0.1), name: (format!("Shell thickness of {id}"), format!("Schalendicke von {id}")) }));
            }
            out.push(handle(Grip { id: format!("{id}:r"), entity: id, entity_kind: EntityKind::Shell, path: p.clone(), kind: HandleKind::Radial { centre: v3(o.x, o.y, o.z) }, anchor: v3(o.x + radius, o.y, o.z + height / 2.0), style: GripStyle::Ring, field: format!("{p}.radiusM"), unit: "m", base: radius, k: 1.0, step: 0.05, limits: (0.5, 50.0), name: (format!("Radius of {id}"), format!("Radius von {id}")) }));
        }
    }
    for b in &document.buildings {
        let o = layout.origin(&b.id);
        for (slot, rec) in document.accelerograms.iter().filter(|x| x.building_id == b.id).enumerate() {
            let Some(raw) = time_history::parse_samples(&rec.samples).filter(|s| s.len() >= 2 && rec.dt_s > 0.0 && rec.scale_factor > 0.0).map(|s| s.iter().fold(0.0_f64, |p, a| p.max(a.abs()))).filter(|p| *p > 0.0) else { continue };
            let at = super::horizontal_elements::trace_base(&o, slot);
            let p = path_of("accelerograms", &rec.id);
            out.push(handle(Grip { id: format!("{}:scale", rec.id), entity: &rec.id, entity_kind: EntityKind::Surface, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(at.x, at.y, at.z + raw * rec.scale_factor * super::horizontal_elements::TRACE_METRES_PER_MS2), style: GripStyle::Arrow, field: format!("{p}.scaleFactor"), unit: "1", base: rec.scale_factor, k: 1.0 / (raw * super::horizontal_elements::TRACE_METRES_PER_MS2), step: 0.01, limits: (0.05, 10.0), name: (format!("Scale factor of {}", rec.label), format!("Skalierungsfaktor von {}", rec.label)) }));
        }
    }
    for w in &document.retaining_walls {
        let o = layout.origin(&w.id);
        let p = path_of("retainingWalls", &w.id);
        out.push(handle(Grip { id: format!("{}:h", w.id), entity: &w.id, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x, o.y, o.z + w.height_m), style: GripStyle::Arrow, field: format!("{p}.heightM"), unit: "m", base: w.height_m, k: 1.0, step: 0.1, limits: (0.5, 30.0), name: (format!("Height of wall {}", w.id), format!("H\u{f6}he von Wand {}", w.id)) }));
        if w.water_height_m > 0.0 {
            let water_id = format!("{}:water", w.id);
            out.push(handle(Grip { id: water_id.clone(), entity: &water_id, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x + 0.5 * WALL_LENGTH, o.y + WALL_THICKNESS + 0.5 * WALL_BACKFILL_WIDTH, o.z + w.water_height_m.min(w.height_m)), style: GripStyle::Arrow, field: format!("{p}.waterHeightM"), unit: "m", base: w.water_height_m, k: 1.0, step: 0.05, limits: (0.0, w.height_m.max(0.5)), name: (format!("Retained water height of wall {}", w.id), format!("Angestaute Wasserh\u{f6}he von Wand {}", w.id)) }));
        }
    }
    for t in &document.tie_beams {
        let Some((start, end)) = tie_beam_ends(document, layout, t) else { continue };
        let p = path_of("tieBeams", &t.id);
        let length = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2) + (end.z - start.z).powi(2)).sqrt().max(1e-9);
        let direction = v3((end.x - start.x) / length, (end.y - start.y) / length, (end.z - start.z) / length);
        let middle = v3(0.5 * (start.x + end.x), 0.5 * (start.y + end.y), 0.5 * (start.z + end.z));
        let side = v3(-direction.y, direction.x, 0.0);
        out.push(handle(Grip { id: format!("{}:w", t.id), entity: &t.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: side.clone() }, anchor: v3(middle.x + 0.5 * t.width * side.x, middle.y + 0.5 * t.width * side.y, middle.z), style: GripStyle::Arrow, field: format!("{p}.width"), unit: "m", base: t.width, k: 2.0, step: 0.05, limits: (0.1, 3.0), name: (format!("Section width of tie beam {}", t.id), format!("Querschnittsbreite von Fundamentbalken {}", t.id)) }));
        out.push(handle(Grip { id: format!("{}:d", t.id), entity: &t.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: v3(0.0, 0.0, -1.0) }, anchor: v3(middle.x, middle.y, middle.z - 0.5 * t.depth), style: GripStyle::Arrow, field: format!("{p}.depth"), unit: "m", base: t.depth, k: 2.0, step: 0.05, limits: (0.1, 3.0), name: (format!("Section depth of tie beam {}", t.id), format!("Querschnittshöhe von Fundamentbalken {}", t.id)) }));
        out.push(handle(Grip { id: format!("{}:u", t.id), entity: &t.id, entity_kind: EntityKind::Member, path: p.clone(), kind: HandleKind::ScalarAlong { axis: direction.clone() }, anchor: v3(end.x + t.relative_displacement * direction.x, end.y + t.relative_displacement * direction.y, end.z + t.relative_displacement * direction.z), style: GripStyle::Diamond, field: format!("{p}.relativeDisplacement"), unit: "m", base: t.relative_displacement, k: 1.0, step: 0.001, limits: (0.0, 0.5), name: (format!("Relative displacement of tie beam {}", t.id), format!("Relativverschiebung von Fundamentbalken {}", t.id)) }));
    }
    for sl in &document.slopes {
        let o = layout.origin(&sl.id);
        let p = path_of("slopes", &sl.id);
        let y = o.y + 0.5 * SLOPE_WIDTH;
        out.push(handle(Grip { id: format!("{}:h", sl.id), entity: &sl.id, entity_kind: EntityKind::Solid, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(o.x, y, o.z + sl.height), style: GripStyle::Arrow, field: format!("{p}.height"), unit: "m", base: sl.height, k: 1.0, step: 0.1, limits: (0.5, 60.0), name: (format!("Height of slope {}", sl.id), format!("Höhe von Böschung {}", sl.id)) }));
        if sl.circle_radius > 0.0 {
            let circle = format!("{}:slip-circle", sl.id);
            let centre = v3(o.x + sl.circle_centre_x, y, o.z + sl.circle_centre_z);
            out.push(handle(Grip { id: format!("{circle}:x"), entity: &circle, entity_kind: EntityKind::Surface, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: centre.clone(), style: GripStyle::Diamond, field: format!("{p}.circleCentreX"), unit: "m", base: sl.circle_centre_x, k: 1.0, step: 0.1, limits: (-200.0, 200.0), name: (format!("Circle centre x of slope {}", sl.id), format!("Kreismittelpunkt x von Böschung {}", sl.id)) }));
            out.push(handle(Grip { id: format!("{circle}:z"), entity: &circle, entity_kind: EntityKind::Surface, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: centre.clone(), style: GripStyle::Diamond, field: format!("{p}.circleCentreZ"), unit: "m", base: sl.circle_centre_z, k: 1.0, step: 0.1, limits: (0.0, 200.0), name: (format!("Circle centre z of slope {}", sl.id), format!("Kreismittelpunkt z von Böschung {}", sl.id)) }));
            out.push(handle(Grip { id: format!("{circle}:r"), entity: &circle, entity_kind: EntityKind::Surface, path: p.clone(), kind: HandleKind::Radial { centre: centre.clone() }, anchor: v3(centre.x + sl.circle_radius, centre.y, centre.z), style: GripStyle::Ring, field: format!("{p}.circleRadius"), unit: "m", base: sl.circle_radius, k: 1.0, step: 0.1, limits: (0.5, 300.0), name: (format!("Circle radius of slope {}", sl.id), format!("Kreisradius von Böschung {}", sl.id)) }));
        }
    }
    scalar_inputs(document, layout, &mut out);
    out
}

/// 🎚️ One scalar input without geometry of its own, dragged as a diamond grip next to its owner.
struct Scalar {
    field: &'static str,
    unit: &'static str,
    value: f64,
    floor: f64,
    lo: f64,
    hi: f64,
    step: f64,
    en: &'static str,
    de: &'static str,
}

/// 🎚️ Diamond grips for the numeric inputs of an owner that have no drawn dimension: side by side at `anchor`, each writing `field` with a gain of a quarter of its value (at least `floor`) per metre dragged.
fn scalar_grips(out: &mut Vec<Handle>, owner: &str, kind: EntityKind, collection: &str, anchor: &Vec3, scalars: &[Scalar]) {
    let p = path_of(collection, owner);
    for (index, s) in scalars.iter().enumerate() {
        let k = (s.value.abs() * 0.25).max(s.floor);
        out.push(handle(Grip { id: format!("{owner}:in:{}", s.field), entity: owner, entity_kind: kind, path: p.clone(), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(anchor.x + 0.6 * index as f64, anchor.y, anchor.z), style: GripStyle::Diamond, field: format!("{p}.{}", s.field), unit: s.unit, base: s.value, k, step: s.step, limits: (s.lo.min(s.value), s.hi.max(s.value)), name: (format!("{} of {owner}", s.en), format!("{} von {owner}", s.de)) }));
    }
}

/// 🎚️ The numeric inputs of buildings, foundations, piers, bridges, silos and tanks that no other grip binds.
fn scalar_inputs(document: &model::En1998Snapshot, layout: &Layout, out: &mut Vec<Handle>) {
    for b in &document.buildings {
        let o = layout.origin(&b.id);
        scalar_grips(out, &b.id, EntityKind::Plate, "buildings", &v3(o.x, o.y + b.plan_length_m + 1.0, o.z + 0.5), &[
            Scalar { field: "nonstructuralWeightN", unit: "N", value: b.nonstructural_weight_n, floor: 1000.0, lo: 0.0, hi: 1.0e7, step: 100.0, en: "Non-structural weight W_a", de: "Gewicht W_a des nichttragenden Bauteils" },
            Scalar { field: "nonstructuralPeriodS", unit: "s", value: b.nonstructural_period_s, floor: 0.05, lo: 0.0, hi: 5.0, step: 0.01, en: "Non-structural period T_a", de: "Eigenperiode T_a des nichttragenden Bauteils" },
            Scalar { field: "nonstructuralQ", unit: "1", value: b.nonstructural_q, floor: 0.5, lo: 1.0, hi: 6.0, step: 0.1, en: "Non-structural behaviour factor q_a", de: "Verhaltensbeiwert q_a des nichttragenden Bauteils" },
            Scalar { field: "nonstructuralResistanceN", unit: "N", value: b.nonstructural_resistance_n, floor: 1000.0, lo: 0.0, hi: 1.0e7, step: 100.0, en: "Anchorage resistance of the non-structural element", de: "Verankerungswiderstand des nichttragenden Bauteils" },
            Scalar { field: "columnMomentSumNm", unit: "N*m", value: b.column_moment_sum_nm, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Column resistance sum at the joint", de: "Stützenwiderstandssumme am Knoten" },
            Scalar { field: "beamMomentSumNm", unit: "N*m", value: b.beam_moment_sum_nm, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Beam resistance sum at the joint", de: "Balkenwiderstandssumme am Knoten" },
            Scalar { field: "tieDownMomentNm", unit: "N*m", value: b.tie_down_moment_nm, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Tie-down stabilising moment", de: "Rückhaltemoment der Zuganker" },
            Scalar { field: "masonryWallAreaRatio", unit: "1", value: b.masonry_wall_area_ratio, floor: 0.01, lo: 0.0, hi: 1.0, step: 0.001, en: "Masonry wall area ratio", de: "Mauerwerks-Wandflächenanteil" },
            Scalar { field: "planEnvelopeGapRatio", unit: "1", value: b.plan_envelope_gap_ratio, floor: 0.01, lo: 0.0, hi: 1.0, step: 0.001, en: "Plan envelope gap ratio", de: "Hüllflächen-Verhältnis des Grundrisses" },
            Scalar { field: "storeyStiffnessRatio", unit: "1", value: b.storey_stiffness_ratio, floor: 0.05, lo: 0.01, hi: 1.0, step: 0.01, en: "Effective storey stiffness share", de: "Wirksamer Anteil der Geschosssteifigkeit" },
        ]);
    }
    for f in &document.foundations {
        let c = foundation_centre(document, layout, f);
        scalar_grips(out, &f.id, EntityKind::Solid, "foundations", &v3(c.x - 3.0, c.y - 3.0, c.z), &[
            Scalar { field: "footingWidthM", unit: "m", value: f.footing_width_m, floor: 0.1, lo: 0.0, hi: 30.0, step: 0.05, en: "Footing width B", de: "Fundamentbreite B" },
            Scalar { field: "bearingAxialN", unit: "N", value: f.bearing_axial_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Footing axial force N_Ed", de: "Normalkraft N_Ed des Fundaments" },
            Scalar { field: "bearingNMaxN", unit: "N", value: f.bearing_n_max_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Ultimate bearing force N_max", de: "Grenztragkraft N_max" },
            Scalar { field: "soilFrictionAngleDeg", unit: "deg", value: f.soil_friction_angle_deg, floor: 1.0, lo: 0.0, hi: 50.0, step: 0.5, en: "Soil friction angle φ'", de: "Reibungswinkel φ' des Bodens" },
            Scalar { field: "pRdPa", unit: "Pa", value: f.p_rd_pa, floor: 10000.0, lo: 0.0, hi: 5.0e7, step: 1000.0, en: "Bearing resistance p_Rd", de: "Sohldruckwiderstand p_Rd" },
            Scalar { field: "passiveResistanceN", unit: "N", value: f.passive_resistance_n, floor: 1000.0, lo: 0.0, hi: 1.0e8, step: 1000.0, en: "Passive resistance", de: "Passiver Widerstand" },
            Scalar { field: "kFoundation", unit: "1", value: f.k_foundation, floor: 1000.0, lo: 0.0, hi: 1.0e10, step: 1000.0, en: "Foundation stiffness", de: "Fundamentsteifigkeit" },
            Scalar { field: "kSoil", unit: "1", value: f.k_soil, floor: 1000.0, lo: 0.0, hi: 1.0e10, step: 1000.0, en: "Soil stiffness", de: "Bodensteifigkeit" },
            Scalar { field: "liqN160", unit: "1", value: f.liq_n1_60, floor: 1.0, lo: 0.0, hi: 60.0, step: 0.5, en: "Corrected SPT count (N1)60", de: "Korrigierte SPT-Schlagzahl (N1)60" },
            Scalar { field: "liqFinesPercent", unit: "1", value: f.liq_fines_percent, floor: 1.0, lo: 0.0, hi: 100.0, step: 1.0, en: "Fines content", de: "Feinkornanteil" },
            Scalar { field: "liqWaterDepthM", unit: "m", value: f.liq_water_depth_m, floor: 0.5, lo: 0.0, hi: 50.0, step: 0.1, en: "Groundwater depth", de: "Grundwassertiefe" },
            Scalar { field: "liqUnitWeightNM3", unit: "N/m3", value: f.liq_unit_weight_n_m3, floor: 1000.0, lo: 0.0, hi: 30000.0, step: 100.0, en: "Soil unit weight", de: "Bodenwichte" },
            Scalar { field: "liqMagnitude", unit: "1", value: f.liq_magnitude, floor: 0.5, lo: 0.0, hi: 9.5, step: 0.1, en: "Surface-wave magnitude Ms", de: "Oberflächenwellenmagnitude Ms" },
            Scalar { field: "pileDiameterM", unit: "m", value: f.pile_diameter_m, floor: 0.1, lo: 0.0, hi: 3.0, step: 0.05, en: "Pile diameter", de: "Pfahldurchmesser" },
            Scalar { field: "pileModulusPa", unit: "Pa", value: f.pile_modulus_pa, floor: 1000000000.0, lo: 0.0, hi: 1.0e11, step: 100000000.0, en: "Pile modulus E_p", de: "Pfahl-Elastizitätsmodul E_p" },
            Scalar { field: "pileSubgradeModulusNM3", unit: "N/m3", value: f.pile_subgrade_modulus_n_m3, floor: 1000000.0, lo: 0.0, hi: 1.0e9, step: 100000.0, en: "Subgrade modulus k_h", de: "Bettungsmodul k_h" },
            Scalar { field: "pileHeadShearN", unit: "N", value: f.pile_head_shear_n, floor: 10000.0, lo: 0.0, hi: 1.0e7, step: 1000.0, en: "Pile head shear", de: "Pfahlkopfquerkraft" },
            Scalar { field: "pileAxialLoadN", unit: "N", value: f.pile_axial_load_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Pile axial load N_Ed", de: "Pfahlnormalkraft N_Ed" },
            Scalar { field: "pileUpperVsMS", unit: "m/s", value: f.pile_upper_vs_m_s, floor: 10.0, lo: 0.0, hi: 1000.0, step: 5.0, en: "Upper layer shear wave velocity", de: "Scherwellengeschwindigkeit oben" },
            Scalar { field: "pileLowerVsMS", unit: "m/s", value: f.pile_lower_vs_m_s, floor: 10.0, lo: 0.0, hi: 1500.0, step: 5.0, en: "Lower layer shear wave velocity", de: "Scherwellengeschwindigkeit unten" },
            Scalar { field: "pileMomentRdNm", unit: "N*m", value: f.pile_moment_rd_nm, floor: 10000.0, lo: 0.0, hi: 1.0e8, step: 1000.0, en: "Pile moment resistance M_Rd", de: "Pfahl-Momententragfähigkeit M_Rd" },
            Scalar { field: "baseFrictionAngleDeg", unit: "deg", value: f.base_friction_angle_deg, floor: 1.0, lo: 0.0, hi: 50.0, step: 0.5, en: "Base friction angle δ", de: "Sohlreibungswinkel δ" },
            Scalar { field: "basePorePressurePa", unit: "Pa", value: f.base_pore_pressure_pa, floor: 1000.0, lo: 0.0, hi: 5.0e6, step: 100.0, en: "Base pore pressure u", de: "Porenwasserdruck u" },
            Scalar { field: "areaM2", unit: "m2", value: f.area_m2, floor: 1.0, lo: 0.0, hi: 1000.0, step: 0.1, en: "Foundation area", de: "Fundamentfläche" },
        ]);
    }
    for pier in &document.piers {
        let Some(b) = document.bridges.iter().find(|b| b.id == pier.bridge_id) else { continue };
        let o = layout.origin(&b.id);
        scalar_grips(out, &pier.id, EntityKind::Member, "piers", &v3(o.x + pier.x_m + 1.5, o.y + DECK_WIDTH / 2.0, o.z + 0.5), &[
            Scalar { field: "sectionBM", unit: "m", value: pier.section_b_m, floor: 0.1, lo: 0.1, hi: 6.0, step: 0.05, en: "Section width b", de: "Querschnittsbreite b" },
            Scalar { field: "sectionHM", unit: "m", value: pier.section_h_m, floor: 0.1, lo: 0.1, hi: 6.0, step: 0.05, en: "Section depth h", de: "Querschnittshöhe h" },
            Scalar { field: "modulusMpa", unit: "MPa", value: pier.modulus_mpa, floor: 1000.0, lo: 0.0, hi: 60000.0, step: 500.0, en: "Effective modulus", de: "Wirksamer Elastizitätsmodul" },
            Scalar { field: "coverM", unit: "m", value: pier.cover_m, floor: 0.01, lo: 0.0, hi: 0.3, step: 0.005, en: "Cover to bar centre", de: "Randabstand der Bewehrungsachse" },
            Scalar { field: "concreteFckMpa", unit: "MPa", value: pier.concrete_fck_mpa, floor: 5.0, lo: 0.0, hi: 100.0, step: 1.0, en: "Concrete strength f_ck", de: "Betonfestigkeit f_ck" },
            Scalar { field: "steelFykMpa", unit: "MPa", value: pier.steel_fyk_mpa, floor: 50.0, lo: 0.0, hi: 700.0, step: 10.0, en: "Steel yield f_yk", de: "Stahlstreckgrenze f_yk" },
            Scalar { field: "axialLoadN", unit: "N", value: pier.axial_load_n, floor: 100000.0, lo: 0.0, hi: 1.0e9, step: 10000.0, en: "Axial load N", de: "Normalkraft N" },
        ]);
    }
    for b in &document.bridges {
        let o = layout.origin(&b.id);
        scalar_grips(out, &b.id, EntityKind::Plate, "bridges", &v3(o.x, o.y - 1.5, o.z + 0.5), &[
            Scalar { field: "periodRatio", unit: "1", value: b.period_ratio, floor: 0.1, lo: 1.0, hi: 6.0, step: 0.05, en: "Isolation period ratio", de: "Periodenverhältnis der Isolierung" },
            Scalar { field: "fundamentalPeriodS", unit: "s", value: b.fundamental_period_s, floor: 0.1, lo: 0.0, hi: 6.0, step: 0.01, en: "Fundamental period T", de: "Grundschwingzeit T" },
            Scalar { field: "vRdN", unit: "N", value: b.v_rd_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Shear resistance V_Rd", de: "Querkrafttragfähigkeit V_Rd" },
            Scalar { field: "deckLengthM", unit: "m", value: b.deck_length_m, floor: 1.0, lo: 0.0, hi: 500.0, step: 0.5, en: "Deck length", de: "Überbaulänge" },
            Scalar { field: "deckEiNm2", unit: "1", value: b.deck_ei_nm2, floor: 1000000.0, lo: 0.0, hi: 1.0e13, step: 1000000.0, en: "Deck bending stiffness EI", de: "Biegesteifigkeit EI des Überbaus" },
        ]);
    }
    for system in &document.isolation_systems {
        let Some(at) = super::ground_and_bridges::isolation_anchor(document, layout, system, 0.4) else { continue };
        scalar_grips(out, &system.id, EntityKind::Support, "isolationSystems", &v3(at.x + 1.5, at.y, at.z + 0.5), &[
            Scalar { field: "strengthN", unit: "N", value: system.strength_n, floor: 1000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Characteristic strength Q_d", de: "Charakteristische Festigkeit Q_d" },
            Scalar { field: "stiffnessNPerM", unit: "N/m", value: system.stiffness_n_per_m, floor: 10000.0, lo: 0.0, hi: 1.0e10, step: 10000.0, en: "Post-yield stiffness K_d", de: "Steifigkeit nach dem Fließen K_d" },
            Scalar { field: "yieldDisplacementM", unit: "m", value: system.yield_displacement_m, floor: 0.001, lo: 0.0, hi: 0.2, step: 0.0005, en: "Yield displacement d_y", de: "Fließverschiebung d_y" },
        ]);
    }
    for base in &document.pier_bases {
        let (Some(bridge), Some(pier)) = (document.bridges.iter().find(|b| b.id == base.bridge_id), document.piers.iter().find(|p| p.id == base.pier_id && p.bridge_id == base.bridge_id)) else { continue };
        let o = layout.origin(&bridge.id);
        scalar_grips(out, &base.id, EntityKind::Support, "pierBases", &v3(o.x + pier.x_m + 2.5, o.y + DECK_WIDTH / 2.0, o.z + 0.5), &[
            Scalar { field: "soilVsMS", unit: "m/s", value: base.soil_vs_m_s, floor: 10.0, lo: 0.0, hi: 1500.0, step: 5.0, en: "Shear wave velocity v_s", de: "Scherwellengeschwindigkeit v_s" },
            Scalar { field: "soilUnitWeightNM3", unit: "N/m3", value: base.soil_unit_weight_n_m3, floor: 1000.0, lo: 0.0, hi: 30000.0, step: 100.0, en: "Soil unit weight γ", de: "Wichte des Bodens γ" },
            Scalar { field: "poissonRatio", unit: "1", value: base.poisson_ratio, floor: 0.02, lo: 0.0, hi: 0.49, step: 0.01, en: "Poisson ratio ν", de: "Querdehnzahl ν" },
        ]);
    }
    for v in &document.silos {
        let o = layout.origin(&v.id);
        scalar_grips(out, &v.id, EntityKind::Shell, "silos", &v3(o.x - v.radius_m, o.y - v.radius_m - 1.0, o.z + 0.5), &[
            Scalar { field: "contentQkN", unit: "N", value: v.content_qk_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Content action Q_k", de: "Füllgutlast Q_k" },
            Scalar { field: "nRdN", unit: "N", value: v.n_rd_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Axial resistance N_Rd", de: "Normalkraftwiderstand N_Rd" },
            Scalar { field: "vRdN", unit: "N", value: v.v_rd_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Shear resistance V_Rd", de: "Schubwiderstand V_Rd" },
            Scalar { field: "qNominal", unit: "1", value: v.q_nominal, floor: 0.5, lo: 1.0, hi: 6.0, step: 0.1, en: "Behaviour factor q", de: "Verhaltensbeiwert q" },
        ]);
    }
    for v in &document.tanks {
        let o = layout.origin(&v.id);
        scalar_grips(out, &v.id, EntityKind::Shell, "tanks", &v3(o.x - v.radius_m, o.y - v.radius_m - 1.0, o.z + 0.5), &[
            Scalar { field: "contentQkN", unit: "N", value: v.content_qk_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Content action Q_k", de: "Füllgutlast Q_k" },
            Scalar { field: "vRdN", unit: "N", value: v.v_rd_n, floor: 10000.0, lo: 0.0, hi: 1.0e9, step: 1000.0, en: "Shear resistance V_Rd", de: "Schubwiderstand V_Rd" },
            Scalar { field: "shellStrengthMpa", unit: "MPa", value: v.shell_strength_mpa, floor: 20.0, lo: 0.0, hi: 1000.0, step: 5.0, en: "Shell design strength", de: "Bemessungsfestigkeit der Schale" },
        ]);
    }
    for w in &document.retaining_walls {
        let o = layout.origin(&w.id);
        let mut scalars = vec![
            Scalar { field: "phiDeg", unit: "deg", value: w.phi_deg, floor: 1.0, lo: 0.0, hi: 50.0, step: 0.5, en: "Soil friction angle φ'", de: "Reibungswinkel φ' des Bodens" },
            Scalar { field: "soilGamma", unit: "N/m3", value: w.soil_gamma, floor: 1000.0, lo: 0.0, hi: 30000.0, step: 100.0, en: "Soil unit weight γ", de: "Bodenwichte γ" },
            Scalar { field: "r", unit: "1", value: w.r, floor: 0.1, lo: 1.0, hi: 3.0, step: 0.05, en: "Displacement factor r", de: "Verschiebungsbeiwert r" },
            Scalar { field: "hRdNPerM", unit: "N/m", value: w.h_rd_n_per_m, floor: 10000.0, lo: 0.0, hi: 1.0e8, step: 1000.0, en: "Sliding resistance H_Rd per metre", de: "Gleitwiderstand H_Rd je Meter" },
            Scalar { field: "saturatedGamma", unit: "N/m3", value: w.saturated_gamma, floor: 1000.0, lo: 0.0, hi: 30000.0, step: 100.0, en: "Saturated unit weight", de: "Wichte unter Auftrieb gesättigt" },
        ];
        if w.water_height_m <= 0.0 {
            scalars.push(Scalar { field: "waterHeightM", unit: "m", value: w.water_height_m, floor: 0.5, lo: 0.0, hi: 30.0, step: 0.05, en: "Water height", de: "Wasserstandshöhe" });
        }
        scalar_grips(out, &w.id, EntityKind::Solid, "retainingWalls", &v3(o.x + 0.5 * WALL_LENGTH, o.y - 1.0, o.z + 0.5), &scalars);
    }
    for f in &document.foundations {
        let centre = foundation_centre(document, layout, f);
        let mut bottom = 0.0;
        for row in document.soil_layers.iter().filter(|l| l.foundation_id == f.id) {
            let thickness = row.thickness_m.max(MIN_SOIL_LAYER);
            bottom += thickness;
            scalar_grips(out, &row.id, EntityKind::Solid, "soilLayers", &v3(centre.x + 2.0, centre.y, centre.z - bottom + 0.5 * thickness), &[
                Scalar { field: "shearWaveVelocityMS", unit: "m/s", value: row.shear_wave_velocity_m_s, floor: 10.0, lo: 0.0, hi: 1500.0, step: 5.0, en: "Shear wave velocity v_s", de: "Scherwellengeschwindigkeit v_s" },
                Scalar { field: "n160", unit: "1", value: row.n1_60, floor: 1.0, lo: 0.0, hi: 60.0, step: 0.5, en: "Corrected SPT count (N1)60", de: "Korrigierte SPT-Schlagzahl (N1)60" },
                Scalar { field: "finesPercent", unit: "1", value: row.fines_percent, floor: 1.0, lo: 0.0, hi: 100.0, step: 1.0, en: "Fines content", de: "Feinkornanteil" },
            ]);
        }
    }
    for row in &document.placements {
        let Some(spec) = entity_kind_of(document, &row.id) else { continue };
        let p = format!("placements[id={}]", row.id);
        out.push(handle(Grip { id: format!("{}:in:z", row.id), entity: &row.id, entity_kind: spec.scene_kind, path: path_of(spec.collection, &row.id), kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: v3(row.x, row.y, row.z), style: GripStyle::Diamond, field: format!("{p}.z"), unit: "m", base: row.z, k: 1.0, step: 0.05, limits: (-50.0, 500.0), name: (format!("Elevation of {}", row.id), format!("Höhenlage von {}", row.id)) }));
    }
}

/// 🎚️ The numeric fields no grip writes, with the reason: the permeability spans orders of magnitude, a linear drag is meaningless; no mutation exists for the soil unit weight; the scene stands every entity upright (`yaw` is not read by the layout).
pub const EXEMPT: &[(&str, &str)] = &[("retainingWalls", "permeabilityMS"), ("soilLayers", "unitWeightNM3"), ("placements", "yaw")];

/// 🕹️ The grips of the selected entities (a derived scene id selects the grips of its own entity).
pub fn handles(document: &model::En1998Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    all(document, &layout).into_iter().filter(|h| selection.iter().any(|s| h.entity.id == s.id || h.entity.id.starts_with(&format!("{}:", s.id)))).collect()
}
