//! 🎚️ Gauges: one scalar grip for every numeric field of a row that has no geometric grip of its own, listed per row type in one table. The drag parameter `s` is the displacement in metres along the rack's up axis, so the field is `value + 20 * step * s`, quantised to its schema step and held inside physical limits (no normative bound is invented).
//! 🔗️ Handle laws: [`assert_handle_laws`](crate::scene::projection_kit::test_support::assert_handle_laws).

use super::*;
use crate::model::{AluminiumConnection, AluminiumMember, FatigueCategory, FatigueDetail, FireInsulation, MemberAction, PatchLoad, PlateElement, RingStiffener, SheetProfile, WeldGroup, WeldRun};
use crate::scene::projection_kit::handle_kit::field_binding;
use std::collections::BTreeSet;

pub const STEPS_PER_METRE: f64 = 20.0;
const PITCH: f64 = 0.12;
const ROW: f64 = 0.2;
const COLUMNS: usize = 12;
const FORCE: f64 = 1.0e10;
const LINE: f64 = 1.0e8;
const STRESS: f64 = 1.0e9;
const LENGTH: f64 = 1000.0;

pub struct Field<T> {
    pub tag: &'static str,
    pub leaf: &'static str,
    pub get: fn(&T) -> f64,
    pub unit: &'static str,
    pub step: f64,
    pub lo: f64,
    pub hi: f64,
    pub en: &'static str,
    pub de: &'static str,
}

macro_rules! field {
    ($tag:literal, $leaf:literal, $get:expr, $unit:literal, $step:expr, $lo:expr, $hi:expr, $en:literal, $de:literal) => {
        Field { tag: $tag, leaf: $leaf, get: $get, unit: $unit, step: $step, lo: $lo, hi: $hi, en: $en, de: $de }
    };
}

pub const MEMBERS: &[Field<AluminiumMember>] = &[
    field!("buckling-y", "bucklingLengthY", |r| r.buckling_length_y, "m", 0.05, 0.0, LENGTH, "Buckling length L_cr,y", "Knicklänge L_cr,y"),
    field!("buckling-z", "bucklingLengthZ", |r| r.buckling_length_z, "m", 0.05, 0.0, LENGTH, "Buckling length L_cr,z", "Knicklänge L_cr,z"),
    field!("buckling-t", "bucklingLengthT", |r| r.buckling_length_t, "m", 0.05, 0.0, LENGTH, "Torsional buckling length L_cr,T", "Drillknicklänge L_cr,T"),
    field!("ltb-length", "ltbLength", |r| r.ltb_length, "m", 0.05, 0.0, LENGTH, "Unrestrained length L_LT", "Gabellagerungslänge L_LT"),
    field!("c1", "c1", |r| r.c1, "1", 0.01, 0.0, 10.0, "Moment gradient factor C_1", "Momentenbeiwert C_1"),
    field!("bearing-length", "bearingLength", |r| r.bearing_length, "m", 0.005, 0.0, 5.0, "Stiff bearing length s_s", "Lasteinleitungslänge s_s"),
    field!("weld-offset", "transverseWeldOffset", |r| r.transverse_weld_offset, "m", 0.05, 0.0, LENGTH, "Distance x_s of the transverse weld from the end", "Abstand x_s der Quernaht vom Bauteilende"),
];

pub const ELEMENTS: &[Field<PlateElement>] = &[
    field!("width", "width", |r| r.width, "m", 0.001, 0.0, 5.0, "Plate width b", "Blechbreite b"),
    field!("thickness", "thickness", |r| r.thickness, "m", 0.0005, 0.0, 0.1, "Plate thickness t", "Blechdicke t"),
];

pub const LIPS: &[Field<PlateElement>] = &[
    field!("lip-depth", "edgeStiffenerDepth", |r| r.edge_stiffener_depth, "m", 0.001, 0.0, 0.5, "Edge stiffener depth", "Randsteifenhöhe"),
    field!("lip-thickness", "edgeStiffenerThickness", |r| r.edge_stiffener_thickness, "m", 0.0005, 0.0, 0.1, "Edge stiffener thickness", "Randsteifendicke"),
];

pub const ACTIONS: &[Field<MemberAction>] = &[
    field!("gk-line", "gKLine", |r| r.g_k_line, "N/m", 100.0, -LINE, LINE, "Permanent line load g_k", "Ständige Streckenlast g_k"),
    field!("qk-line", "qKLine", |r| r.q_k_line, "N/m", 100.0, -LINE, LINE, "Variable line load q_k", "Veränderliche Streckenlast q_k"),
    field!("n", "nK", |r| r.n_k, "N", 100.0, -FORCE, FORCE, "Axial force N_k", "Normalkraft N_k"),
    field!("vy", "vYK", |r| r.v_y_k, "N", 100.0, -FORCE, FORCE, "Shear force V_y,k", "Querkraft V_y,k"),
    field!("vz", "vZK", |r| r.v_z_k, "N", 100.0, -FORCE, FORCE, "Shear force V_z,k", "Querkraft V_z,k"),
    field!("my", "mYK", |r| r.m_y_k, "Nm", 100.0, -FORCE, FORCE, "Bending moment M_y,k", "Biegemoment M_y,k"),
    field!("mz", "mZK", |r| r.m_z_k, "Nm", 100.0, -FORCE, FORCE, "Bending moment M_z,k", "Biegemoment M_z,k"),
    field!("torsion", "torsionK", |r| r.torsion_k, "Nm", 100.0, -FORCE, FORCE, "Torque T_k", "Torsionsmoment T_k"),
    field!("local-load", "localLoadK", |r| r.local_load_k, "N", 100.0, -FORCE, FORCE, "Transverse patch load F_k", "Quere Einzellast F_k"),
];

pub const WELDS: &[Field<WeldGroup>] = &[field!("weld-beta", "welds.betaW", |r| r.beta_w, "1", 0.01, 0.0, 10.0, "Weld correlation factor β_w", "Korrelationsbeiwert β_w der Naht")];

pub const FATIGUE_DETAILS: &[Field<FatigueDetail>] = &[
    field!("delta-sigma-c", "deltaSigmaC", |r| r.delta_sigma_c, "Pa", 1.0e6, 0.0, STRESS, "Detail category Δσ_c", "Kerbfall Δσ_c"),
    field!("m1", "m1", |r| r.m1, "1", 0.1, 0.0, 20.0, "First S-N slope m_1", "Erste Wöhlerlinienneigung m_1"),
    field!("m2", "m2", |r| r.m2, "1", 0.1, 0.0, 20.0, "Second S-N slope m_2", "Zweite Wöhlerlinienneigung m_2"),
];

pub const FATIGUE_CATEGORIES: &[Field<FatigueCategory>] = &[
    field!("m1", "m1", |r| r.m1, "1", 0.5, 0.0, 20.0, "First S-N slope m_1 of the site", "Erste Wöhlerlinienneigung m_1 der Stelle"),
    field!("m2", "m2", |r| r.m2, "1", 0.5, 0.0, 20.0, "Second S-N slope m_2 of the site", "Zweite Wöhlerlinienneigung m_2 der Stelle"),
];

pub const PATCH_LOADS: &[Field<PatchLoad>] = &[
    field!("web-height", "webHeight", |r| r.web_height, "m", 0.005, 0.0, 5.0, "Web height h_w", "Steghöhe h_w"),
    field!("web-angle", "webAngle", |r| r.web_angle, "rad", 0.0175, 0.0, std::f64::consts::PI, "Web slope φ", "Stegneigung φ"),
    field!("corner-radius", "cornerRadius", |r| r.corner_radius, "m", 0.001, 0.0, 0.2, "Inner corner radius r", "Innerer Eckradius r"),
];

pub const RING_STIFFENERS: &[Field<RingStiffener>] = &[field!("thickness", "thickness", |r| r.thickness, "m", 0.001, 0.0, 0.1, "Ring thickness", "Ringdicke")];

pub const WELD_RUNS: &[Field<WeldRun>] = &[field!("offset", "offset", |r| r.offset, "m", 0.001, -LENGTH, LENGTH, "Weld position on the element", "Nahtlage am Blechelement")];

pub const CONNECTION_ECCENTRICITY: &[Field<AluminiumConnection>] = &[field!("load-eccentricity", "loadEccentricity", |r| r.load_eccentricity, "m", 0.005, 0.0, 5.0, "Load eccentricity e to the group centroid", "Lastexzentrizität e zum Gruppenschwerpunkt")];

pub const CONNECTION_STAGGER: &[Field<AluminiumConnection>] = &[field!("hole-stagger", "holeStagger", |r| r.hole_stagger, "m", 0.005, 0.0, 0.5, "Hole stagger s of every second bolt line", "Lochversatz s jeder zweiten Schraubenlinie")];

pub const SHEET_PROFILES: &[Field<SheetProfile>] = &[
    field!("pitch", "pitch", |r| r.pitch, "m", 0.005, 0.0, 5.0, "Rib pitch p", "Rippenabstand p"),
    field!("top-flange", "topFlange", |r| r.top_flange, "m", 0.001, 0.0, 5.0, "Top flange width b_t", "Obergurtbreite b_t"),
    field!("bottom-flange", "bottomFlange", |r| r.bottom_flange, "m", 0.001, 0.0, 5.0, "Bottom flange width b_b", "Untergurtbreite b_b"),
    field!("web-height", "webHeight", |r| r.web_height, "m", 0.001, 0.0, 1.0, "Profile height h", "Profilhöhe h"),
];

pub const FIRE_INSULATIONS: &[Field<FireInsulation>] = &[
    field!("thickness", "thickness", |r| r.thickness, "m", 0.001, 0.0, 0.5, "Insulation thickness d_p", "Bekleidungsdicke d_p"),
    field!("conductivity", "conductivity", |r| r.conductivity, "1", 0.01, 0.0, 10.0, "Conductivity λ_p [W/(m·K)]", "Wärmeleitfähigkeit λ_p [W/(m·K)]"),
    field!("specific-heat", "specificHeat", |r| r.specific_heat, "1", 10.0, 0.0, 10000.0, "Specific heat c_p [J/(kg·K)]", "Spezifische Wärmekapazität c_p [J/(kg·K)]"),
    field!("density", "density", |r| r.density, "1", 10.0, 0.0, 10000.0, "Density ρ_p [kg/m³]", "Rohdichte ρ_p [kg/m³]"),
];

/// 🚫 The numeric schema fields no viewport grip writes: none. Known limit: the outline vertices beyond the first [`MAX_VERTEX_HANDLES`](super::MAX_VERTEX_HANDLES) of a section have no drag grip and are edited in the properties window.
pub const EXEMPT: &[(&str, &str)] = &[];

/// 📐️ The schema steps the gauges do not take over and why: `members[].bearingLength` is a length in metres whose schema step reads 5 (a millimetre value on a metre field); the schema tool has to correct the step, until then the gauge uses 5 mm.
pub const STEP_OVERRIDES: &[(&str, f64)] = &[("members[].bearingLength", 0.005)];

fn fields<T>(collection: &str, table: &[Field<T>]) -> Vec<(String, f64)> {
    table.iter().map(|f| (format!("{collection}.{}", f.leaf), f.step)).collect()
}

/// 🎚️ The normalised schema paths (`collection[].leaf`) the gauge tables cover with the step each gauge uses: the contract of the grip coverage test.
pub fn coverage() -> Vec<(String, f64)> {
    let mut out = Vec::new();
    out.extend(fields("members[]", MEMBERS));
    out.extend(fields("sections[].elements[]", ELEMENTS));
    out.extend(fields("sections[].elements[]", LIPS));
    for owner in ["members[]", "connections[]", "coldFormed[]", "shells[]"] {
        out.extend(fields(&format!("{owner}.actions[]"), ACTIONS));
    }
    out.extend(fields("connections[]", WELDS));
    out.extend(fields("connections[]", CONNECTION_ECCENTRICITY));
    out.extend(fields("connections[]", CONNECTION_STAGGER));
    out.extend(fields("sheetProfiles[]", SHEET_PROFILES));
    out.extend(fields("fireInsulations[]", FIRE_INSULATIONS));
    out.extend(fields("fatigueDetails[]", FATIGUE_DETAILS));
    out.extend(fields("fatigueCategories[]", FATIGUE_CATEGORIES));
    out.extend(fields("patchLoads[]", PATCH_LOADS));
    out.extend(fields("ringStiffeners[]", RING_STIFFENERS));
    out.extend(fields("weldRuns[]", WELD_RUNS));
    out.push(("placements[].z".into(), 0.1));
    out
}

struct Gauge {
    tag: String,
    path: String,
    base: f64,
    unit: &'static str,
    step: f64,
    lo: f64,
    hi: f64,
    en: String,
    de: String,
}

fn pick<T>(table: &[Field<T>], row: &T, prefix: &str, tag: &str, label: &str) -> Vec<Gauge> {
    table
        .iter()
        .map(|f| Gauge { tag: format!("{tag}{}", f.tag), path: format!("{prefix}.{}", f.leaf), base: (f.get)(row), unit: f.unit, step: f.step, lo: f.lo, hi: f.hi, en: format!("{} of {label}", f.en), de: format!("{} von {label}", f.de) })
        .collect()
}

fn actions_of(owner: &str, collection: &str, actions: &[MemberAction], arrows: &BTreeSet<String>) -> Vec<Gauge> {
    actions
        .iter()
        .flat_map(|a| pick(ACTIONS, a, &format!("{collection}[id={owner}].actions[id={}]", a.id), &format!("act-{}-", a.id), &format!("{owner}/{}", a.id)))
        .filter(|g| !arrows.contains(&g.path))
        .collect()
}

fn arrow_paths(handles: &[Handle]) -> BTreeSet<String> {
    handles.iter().flat_map(|h| h.writes.iter().map(|w| w.path.clone())).collect()
}

fn placement_z(d: &En1999Snapshot, id: &str) -> Vec<Gauge> {
    d.placements
        .iter()
        .filter(|p| p.id == id)
        .map(|p| Gauge { tag: "placement-z".into(), path: format!("placements[id={id}].z"), base: p.z, unit: "m", step: 0.1, lo: -LENGTH, hi: LENGTH, en: format!("Placement height z of {id}"), de: format!("Platzierungshöhe z von {id}") })
        .collect()
}

fn rack(entity: &EntityRef, gauges: Vec<Gauge>, place: &dyn Fn(f64, f64) -> Vec3, up: &Vec3, out: &mut Vec<Handle>) {
    let n = gauges.len();
    let columns = n.clamp(1, COLUMNS);
    for (i, g) in gauges.into_iter().enumerate() {
        let k = STEPS_PER_METRE * g.step;
        let (a, b) = ((g.lo.min(g.base) - g.base) / k, (g.hi.max(g.base) - g.base) / k);
        let quantise = if g.unit == "1" && (g.step - 1.0).abs() < 1e-12 { Some(Quantise::Int) } else { round(g.step) };
        let across = (i % COLUMNS) as f64 - (columns as f64 - 1.0) / 2.0;
        out.push(handle(Spec {
            entity,
            tag: &format!("gauge-{}", g.tag),
            kind: HandleKind::ScalarAlong { axis: up.clone() },
            anchor: place(across * PITCH, (i / COLUMNS) as f64 * ROW),
            grip: GripStyle::Diamond,
            writes: vec![field_binding(g.path, Affine { base: g.base, k }, g.unit, quantise)],
            snap: grid(0.05),
            limits: (a.min(b), a.max(b)),
            name: loc(&g.en, &g.de),
        }));
    }
}

struct Stand {
    origin: Vec3,
    x: Vec3,
    z: Vec3,
    length: f64,
    depth: f64,
}

fn stand_of(d: &En1999Snapshot, layout: &Layout, member_id: &str) -> Stand {
    let depth = d.members.iter().find(|m| m.id == member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id)).map_or(0.2, |s| section_extent(d, s).0);
    match layout.member(member_id) {
        Some(p) => Stand { origin: p.frame.origin.clone(), x: p.frame.x.clone(), z: p.frame.z.clone(), length: p.length, depth },
        None => Stand { origin: Vec3::ZERO, x: Vec3::X, z: Vec3::Z, length: 0.0, depth },
    }
}

fn member_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(member) = d.members.iter().find(|m| m.id == id) else { return };
    let Some(pose) = layout.member(id) else { return };
    let stand = stand_of(d, layout, id);
    let entity = entity_ref(EntityKind::Member, id, "members");
    let mut gauges = pick(MEMBERS, member, &format!("members[id={id}]"), "", id);
    if let Some(section) = d.sections.iter().find(|s| s.id == member.section_id).filter(|s| d.members.iter().find(|m| m.section_id == s.id).is_some_and(|first| first.id == id)) {
        for el in &section.elements {
            let prefix = format!("sections[id={}].elements[id={}]", section.id, el.id);
            let label = format!("{}/{}", section.id, el.id);
            gauges.extend(pick(ELEMENTS, el, &prefix, &format!("el-{}-", el.id), &label));
            if el.outstand {
                gauges.extend(pick(LIPS, el, &prefix, &format!("el-{}-", el.id), &label));
            }
        }
    }
    let arrows = arrow_paths(&crate::members::action_grips(id, "members", &member.actions, &pose.frame, member.length, stand.depth));
    gauges.extend(actions_of(id, "members", &member.actions, &arrows));
    gauges.extend(placement_z(d, id));
    let (o, x, z) = (stand.origin.clone(), stand.x.clone(), stand.z.clone());
    let lift = stand.depth / 2.0 + 0.7;
    rack(&entity, gauges, &|dx, dz| along(&along(&o, &x, stand.length / 2.0 + dx), &z, lift + dz), &z, out);
}

fn connection_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(conn) = d.connections.iter().find(|c| c.id == id) else { return };
    let stand = stand_of(d, layout, &conn.member_id);
    let entity = entity_ref(EntityKind::Group, id, "connections");
    let mut gauges = Vec::new();
    if conn.kind != crate::model::ConnectionKind::Bolted {
        gauges.extend(pick(WELDS, &conn.welds, &format!("connections[id={id}]"), "", id));
    }
    gauges.extend(pick(CONNECTION_ECCENTRICITY, conn, &format!("connections[id={id}]"), "", id));
    gauges.extend(pick(CONNECTION_STAGGER, conn, &format!("connections[id={id}]"), "", id));
    let frame = Frame { origin: stand.origin.clone(), x: stand.x.clone(), y: Vec3::Y, z: stand.z.clone() };
    let arrows = arrow_paths(&crate::members::action_grips(id, "connections", &conn.actions, &frame, stand.length, stand.depth));
    gauges.extend(actions_of(id, "connections", &conn.actions, &arrows));
    let end = along(&stand.origin, &stand.x, stand.length + conn.bolts.plate_thickness);
    let z = stand.z.clone();
    let lift = -stand.depth / 2.0 - 0.6;
    rack(&entity, gauges, &|dy, dz| along(&along(&end, &Vec3::Y, dy), &z, lift - dz), &z, out);
}

fn sheet_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(sheet) = d.cold_formed.iter().find(|s| s.id == id) else { return };
    let o = layout.origin(id);
    let entity = entity_ref(EntityKind::Member, id, "coldFormed");
    let frame = crate::kit::member_frame(&o, &Vec3::X);
    let arrows = arrow_paths(&crate::members::action_grips(id, "coldFormed", &sheet.actions, &frame, sheet.span, 0.2 * sheet.width));
    let mut gauges = actions_of(id, "coldFormed", &sheet.actions, &arrows);
    for profile in d.sheet_profiles.iter().filter(|p| p.sheet_id == id) {
        gauges.extend(pick(SHEET_PROFILES, profile, &format!("sheetProfiles[id={}]", profile.id), &format!("{}-", profile.id), &profile.id));
    }
    gauges.extend(placement_z(d, id));
    let lift = 0.2 * sheet.width + 0.6;
    rack(&entity, gauges, &|dx, dz| v3(o.x + sheet.span / 2.0 + dx, o.y, o.z + lift + dz), &Vec3::Z, out);
}

fn shell_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(shell) = d.shells.iter().find(|s| s.id == id) else { return };
    let o = layout.origin(id);
    let entity = entity_ref(EntityKind::Shell, id, "shells");
    let frame = crate::kit::member_frame(&o, &Vec3::Z);
    let arrows = arrow_paths(&crate::members::action_grips(id, "shells", &shell.actions, &frame, shell.length, 2.0 * shell.radius));
    let mut gauges = actions_of(id, "shells", &shell.actions, &arrows);
    gauges.extend(placement_z(d, id));
    rack(&entity, gauges, &|dy, dz| v3(o.x + shell.radius + 0.6, o.y + dy, o.z + shell.length / 2.0 + dz), &Vec3::Z, out);
}

fn fire_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(fire) = d.fire_scenarios.iter().find(|f| f.id == id) else { return };
    let stand = stand_of(d, layout, &fire.member_id);
    let entity = entity_ref(EntityKind::Load, id, "fireScenarios");
    let gauges: Vec<Gauge> = d.fire_insulations.iter().filter(|i| i.fire_id == id).flat_map(|i| pick(FIRE_INSULATIONS, i, &format!("fireInsulations[id={}]", i.id), &format!("{}-", i.id), &i.id)).collect();
    let (o, x, z) = (stand.origin.clone(), stand.x.clone(), stand.z.clone());
    let lift = -stand.depth / 2.0 - 0.6;
    rack(&entity, gauges, &|dx, dz| along(&along(&o, &x, stand.length / 2.0 + dx), &z, lift - dz), &z, out);
}

fn detail_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(fat) = d.fatigue_details.iter().find(|f| f.id == id) else { return };
    let stand = stand_of(d, layout, &fat.member_id);
    let entity = entity_ref(EntityKind::Group, id, "fatigueDetails");
    let gauges = pick(FATIGUE_DETAILS, fat, &format!("fatigueDetails[id={id}]"), "", id);
    let (o, x, z) = (stand.origin.clone(), stand.x.clone(), stand.z.clone());
    let lift = stand.depth / 2.0 + 0.6;
    rack(&entity, gauges, &|dx, dz| along(&along(&o, &x, stand.length * 0.75 + dx), &z, lift + dz), &z, out);
}

fn site_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(cat) = d.fatigue_categories.iter().find(|c| c.id == id) else { return };
    let stand = stand_of(d, layout, &cat.member_id);
    let entity = entity_ref(EntityKind::Solid, id, "fatigueCategories");
    let gauges = pick(FATIGUE_CATEGORIES, cat, &format!("fatigueCategories[id={id}]"), "", id);
    let (o, x, z) = (stand.origin.clone(), stand.x.clone(), stand.z.clone());
    let lift = stand.depth / 2.0 + 0.6;
    rack(&entity, gauges, &|dx, dz| along(&along(&o, &x, cat.location_x.clamp(0.0, stand.length) + dx), &z, lift + dz), &z, out);
}

fn patch_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(load) = d.patch_loads.iter().find(|l| l.id == id) else { return };
    let entity = entity_ref(EntityKind::Load, id, "patchLoads");
    let gauges = pick(PATCH_LOADS, load, &format!("patchLoads[id={id}]"), "", id);
    let (o, x, z, length, depth) = super::patch_pose(d, layout, load).map_or((Vec3::ZERO, Vec3::X, Vec3::Z, 0.0, 0.2), |p| (p.origin, p.x, p.z, p.length, p.depth));
    let lift = depth / 2.0 + 0.6;
    rack(&entity, gauges, &|dx, dz| along(&along(&o, &x, load.x.clamp(0.0, length) + dx), &z, lift + dz), &z, out);
}

fn ring_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(ring) = d.ring_stiffeners.iter().find(|r| r.id == id) else { return };
    let Some(shell) = d.shells.iter().find(|s| s.id == ring.shell_id) else { return };
    let o = layout.origin(&shell.id);
    let entity = entity_ref(EntityKind::Shell, id, "ringStiffeners");
    let gauges = pick(RING_STIFFENERS, ring, &format!("ringStiffeners[id={id}]"), "", id);
    rack(&entity, gauges, &|dy, dz| v3(o.x + shell.radius + ring.depth + 0.4, o.y + dy, o.z + ring.x + dz), &Vec3::Z, out);
}

fn run_gauges(d: &En1999Snapshot, layout: &Layout, id: &str, out: &mut Vec<Handle>) {
    let Some(run) = d.weld_runs.iter().find(|r| r.id == id) else { return };
    let stand = stand_of(d, layout, &run.member_id);
    let entity = entity_ref(EntityKind::Fastener, id, "weldRuns");
    let gauges = pick(WELD_RUNS, run, &format!("weldRuns[id={id}]"), "", id);
    let (o, x, z) = (stand.origin.clone(), stand.x.clone(), stand.z.clone());
    let lift = stand.depth / 2.0 + 0.5;
    rack(&entity, gauges, &|dx, dz| along(&along(&o, &x, run.start.clamp(0.0, stand.length) + dx), &z, lift + dz), &z, out);
}

/// 🎚️ The gauge grips of one document entity, anchored in the given layout and free of any leaf a geometric grip of the same entity already writes.
pub fn of(d: &En1999Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let mut out = Vec::new();
    member_gauges(d, layout, id, &mut out);
    connection_gauges(d, layout, id, &mut out);
    sheet_gauges(d, layout, id, &mut out);
    shell_gauges(d, layout, id, &mut out);
    fire_gauges(d, layout, id, &mut out);
    detail_gauges(d, layout, id, &mut out);
    site_gauges(d, layout, id, &mut out);
    patch_gauges(d, layout, id, &mut out);
    ring_gauges(d, layout, id, &mut out);
    run_gauges(d, layout, id, &mut out);
    out
}
