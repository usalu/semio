//! 🏋️ Loads: the vertical resultants at the top and the bottom with their eccentricities, concentrated loads with the 60° spreading zone, the wind pressure and the indicative yield-line pattern of the laterally loaded wall.

use super::scene::projection_kit::{loc};
use super::kit::*;
use super::layout::{Layout, WallPose, EPS};
use super::model::{ConcentratedLoad, MasonryWall, WallLoadCase};
use super::scene::*;
use super::{handles, profile, SceneInput};

const FRONT: f64 = -0.006;
const SPREAD_DEGREES: f64 = 60.0;
pub(crate) const YIELD_FIELD: &str = "wall-yield-deflection";

fn case(lc: &str, kind: LoadKind) -> LoadCaseRef {
    LoadCaseRef { id: lc.to_string(), kind, psi: None }
}

fn point(id: String, at: Vec3, pose: &WallPose, force: f64, lc: &str, kind: LoadKind, name: LocalizedCopy) -> Spec {
    let glyph = LoadGlyph::Point { at, direction: pose.dir(0.0, 0.0, -1.0), magnitude: Quantity::new(QuantityKind::Force, force.max(1.0)), case: case(lc, kind) };
    let mut spec = Spec::new(EntityKind::Load, id, Geometry::Load(glyph), MaterialRole::Load { kind }, LAYER_LOADS, name);
    spec.detail = spec.name.clone();
    spec
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        if wall.length_m <= EPS || wall.height_m <= EPS || wall.thickness_m <= EPS {
            continue;
        }
        resultants(input, layout, wall, pose, out);
        for lc in &wall.load_cases {
            concentrated(input, layout, wall, lc, pose, out);
        }
        lateral(input, wall, pose, out);
    }
}

fn resultants(input: &SceneInput, layout: &Layout, wall: &MasonryWall, pose: &WallPose, out: &mut Out) {
    let Some(p) = profile::governing(input.document, wall) else { return };
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let stations = [("resultant-top", 0usize, h, p.e_top, "Design axial force at the top", "Bemessungsnormalkraft am Kopf"), ("resultant-bottom", 2, 0.0, p.e_bottom, "Design axial force at the base", "Bemessungsnormalkraft am Fuß")];
    for (tag, station, z, e, en, de) in stations {
        let e = e.max(0.05 * t).min(0.5 * t);
        let at = pose.at(0.5 * l, 0.5 * t + e, z);
        let id = format!("{}:{tag}", wall.id);
        let mut spec = point(id.clone(), at.clone(), pose, p.n[station], &p.load_case, LoadKind::Permanent, loc(en, de));
        spec.detail = loc(&format!("{en}: N_Ed = {:.0} N at eccentricity {:.1} mm", p.n[station], e * 1000.0), &format!("{de}: N_Ed = {:.0} N bei Ausmitte {:.1} mm", p.n[station], e * 1000.0));
        spec.binding = Some(computed_binding(compression_id(&wall.id, &p.load_case, station), p.utilisation[station]));
        spec.handles = handles::ids(input.document, layout, &id);
        out.push(spec, &[at]);
        let from = pose.at(0.5 * l, 0.5 * t, z + if station == 0 { 0.15 } else { -0.15 });
        let to = pose.at(0.5 * l, 0.5 * t + e, z + if station == 0 { 0.15 } else { -0.15 });
        out.annotate(Annotation::Dimension { id: format!("{}:{tag}-eccentricity", wall.id), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, e), binding: None }, &from);
    }
    let mid = pose.at(0.5 * l, 0.5 * t, 0.5 * h);
    let shifted = pose.at(0.5 * l, 0.5 * t + p.e_mk.abs().min(0.5 * t), 0.5 * h);
    out.annotate(Annotation::Dimension { id: format!("{}:resultant-mid-eccentricity", wall.id), from: mid.clone(), to: shifted, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, p.e_mk.abs().min(0.5 * t)), binding: None }, &mid);
    let base = pose.at(0.0, FRONT, 0.0);
    let h_ef = super::masonry::h_ef_m(wall);
    out.annotate(Annotation::Dimension { id: format!("{}:effective-height", wall.id), from: base.clone(), to: pose.at(0.0, FRONT, h_ef), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, h_ef), binding: None }, &base);
}

fn concentrated(input: &SceneInput, layout: &Layout, wall: &MasonryWall, lc: &WallLoadCase, pose: &WallPose, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    for c in lc.concentrated.iter() {
        let x = c.position_m.clamp(0.0, l);
        let id = format!("{}:{CONCENTRATED_TAG}{}-{}", wall.id, lc.id, c.id);
        let nested = format!(".loadCases[id={}].concentrated[id={}]", lc.id, c.id);
        let binding = binding_of(input.report, &format!("en1996.6.1.3.concentrated.{}.{}.{}", wall.id, lc.id, c.id));
        let at = pose.at(x, 0.5 * t, h);
        let mut spec = point(id.clone(), at.clone(), pose, c.force_n, &lc.id, LoadKind::Permanent, loc(&format!("Concentrated load {}", c.id), &format!("Einzellast {}", c.id)));
        spec.nested = nested.clone();
        spec.detail = loc(&format!("F_k = {:.0} N on {:.0} mm bearing length", c.force_n, c.bearing_length_m * 1000.0), &format!("F_k = {:.0} N auf {:.0} mm Auflagerlänge", c.force_n, c.bearing_length_m * 1000.0));
        spec.binding = binding.clone();
        spec.handles = handles::ids(input.document, layout, &id);
        out.push(spec, &[at]);
        let polygon = spreading(wall, c, x);
        let points: Vec<Vec3> = polygon.iter().map(|(px, pz)| pose.at(*px, FRONT, *pz)).collect();
        let mut zone = Spec::new(EntityKind::Surface, format!("{}:spread-{}-{}", wall.id, lc.id, c.id), surface(&points), MaterialRole::Load { kind: LoadKind::Permanent }, LAYER_PATTERNS, loc("Load spreading at 60°", "Lastausbreitung unter 60°"));
        zone.nested = nested;
        zone.detail = loc(&format!("Effective length {:.2} m at mid-height", polygon[2].0.max(polygon[3].0) - polygon[2].0.min(polygon[3].0)), &format!("Wirksame Länge {:.2} m in Wandmitte", polygon[2].0.max(polygon[3].0) - polygon[2].0.min(polygon[3].0)));
        zone.binding = binding;
        zone.opacity = 0.4;
        zone.hatch = Some(Hatch::Dots);
        out.push(zone, &points);
    }
}

/// 📐️ The spreading zone `(x, z)` of a concentrated load on the wall face: the bearing length at the top widening at 60° to the horizontal down to mid-height, clamped to the wall.
pub fn spreading(wall: &MasonryWall, load: &ConcentratedLoad, x: f64) -> [(f64, f64); 4] {
    let (l, h) = (wall.length_m, wall.height_m);
    let half = 0.5 * load.bearing_length_m.max(0.0);
    let spread = 0.5 * h / SPREAD_DEGREES.to_radians().tan();
    [((x - half).max(0.0), h), ((x + half).min(l), h), ((x + half + spread).min(l), 0.5 * h), ((x - half - spread).max(0.0), 0.5 * h)]
}

/// 📐️ The normalised out-of-plane displacement of the Johansen mechanism at a region vertex at the height `z`: 1 on the ridge at mid-height, 0 on the held edges, linear inside every rigid region.
pub fn yield_deflection(z: f64, h: f64) -> f64 {
    if (z - 0.5 * h).abs() <= EPS.max(1e-9 * h) { 1.0 } else { 0.0 }
}

/// 🧱️ The yield-line regions `(x, z)` of a wall panel with `sides` held sides and corner offset `a`, ridge at mid-height; the free vertical edges of three- and two-sided walls carry no diagonal.
pub fn yield_regions(sides: u8, l: f64, h: f64, a: f64) -> Vec<Vec<(f64, f64)>> {
    let r = 0.5 * h;
    match sides {
        2 => vec![vec![(0.0, 0.0), (l, 0.0), (l, r), (0.0, r)], vec![(0.0, r), (l, r), (l, h), (0.0, h)]],
        3 => vec![vec![(0.0, 0.0), (a, r), (0.0, h)], vec![(0.0, 0.0), (l, 0.0), (l, r), (a, r)], vec![(0.0, h), (a, r), (l, r), (l, h)]],
        _ => vec![vec![(0.0, 0.0), (a, r), (0.0, h)], vec![(l, 0.0), (l, h), (l - a, r)], vec![(0.0, 0.0), (l, 0.0), (l - a, r), (a, r)], vec![(0.0, h), (a, r), (l - a, r), (l, h)]],
    }
}

fn lateral(input: &SceneInput, wall: &MasonryWall, pose: &WallPose, out: &mut Out) {
    let (l, h) = (wall.length_m, wall.height_m);
    let Some(lc) = wall.load_cases.iter().filter(|lc| lc.q_p_wind_pa * lc.c_pe.abs() > 0.0).max_by(|a, b| (a.q_p_wind_pa * a.c_pe.abs()).total_cmp(&(b.q_p_wind_pa * b.c_pe.abs()))) else { return };
    let w_e = lc.q_p_wind_pa * lc.c_pe.abs();
    let face = [pose.at(0.0, FRONT, 0.0), pose.at(l, FRONT, 0.0), pose.at(l, FRONT, h), pose.at(0.0, FRONT, h)];
    let glyph = LoadGlyph::Pressure { face: wall_ref(&wall.id), zones: vec![PressureZone { polygon: ring(&face), value: Quantity::new(QuantityKind::Pressure, w_e), label: loc(&format!("w_e = {w_e:.0} Pa"), &format!("w_e = {w_e:.0} Pa")) }], case: case(&lc.id, LoadKind::Wind) };
    let mut wind = Spec::new(EntityKind::Load, format!("{}:wind-{}", wall.id, lc.id), Geometry::Load(glyph), MaterialRole::Load { kind: LoadKind::Wind }, LAYER_LOADS, loc("Wind pressure on the wall", "Winddruck auf die Wand"));
    wind.detail = loc(&format!("w_e = q_p c_pe = {w_e:.0} Pa"), &format!("w_e = q_p c_pe = {w_e:.0} Pa"));
    wind.nested = format!(".loadCases[id={}]", lc.id);
    out.push(wind, &face);
    let (m1, m2) = profile::lateral(input.document, wall).map_or((1.0, 1.0), |x| (x.m1, x.m2));
    let a = (0.5 * l).min(0.5 * h * (m2 / m1.max(1e-12)).sqrt()).max(0.05 * l);
    let binding = binding_of(input.report, &format!("en1996.m-31.w-rd.{}.{}", wall.id, lc.id));
    for (k, region) in yield_regions(wall.support_sides, l, h, a).into_iter().enumerate() {
        let points: Vec<Vec3> = region.iter().map(|(px, pz)| pose.at(*px, FRONT, *pz)).collect();
        let mut spec = Spec::new(EntityKind::Surface, format!("{}:yield-{}", wall.id, k + 1), surface(&points), MaterialRole::Load { kind: LoadKind::Wind }, LAYER_PATTERNS, loc("Yield-line region", "Bruchlinienfeld"));
        spec.detail = loc(&format!("Indicative collapse mechanism of the {}-sided wall under {w_e:.0} Pa", wall.support_sides), &format!("Richtungsweisender Bruchmechanismus der {}-seitig gehaltenen Wand unter {w_e:.0} Pa", wall.support_sides));
        spec.binding = binding.clone();
        spec.opacity = 0.35;
        spec.hatch = Some(Hatch::Diagonal);
        let values = region.iter().map(|(_, pz)| yield_deflection(*pz, h)).collect();
        spec.channel = Some(ColourChannel::Fields { active: YIELD_FIELD.into(), fields: vec![ScalarField { id: YIELD_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } }] });
        field_legend(&mut out.legends, YIELD_FIELD, loc("Yield-line mechanism: normalised out-of-plane displacement", "Bruchlinienmechanismus: normierte Verschiebung aus der Ebene"));
        out.push(spec, &points);
    }
}
