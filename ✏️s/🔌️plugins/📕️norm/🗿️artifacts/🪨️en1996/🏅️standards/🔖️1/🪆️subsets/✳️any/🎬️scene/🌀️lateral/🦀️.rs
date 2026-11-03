//! 🌀️ Lateral: the wall under out-of-plane load as a deflected one-metre strip from the shared mechanics kernel ([`mechanics`]), the resultant eccentricity at mid-height, the cracked zones where the eccentricity leaves the middle third,
//! the shear flanges at the wall and the bearing stress block under each concentrated load.

use super::kit::*;
use super::layout::{solid_rects, Layout, WallPose, EPS};
use super::mechanics::member_modes::{deflection, Curve, Ends, Loads, Props};
use super::model::MasonryWall;
use super::scene::projection_kit::{member_frame, polyline};
use super::scene::*;
use super::{masonry as m, profile, SceneInput};

const STRIP_WIDTH: f64 = 1.0;
const SHEAR_MODULUS_RATIO: f64 = 2.4;
const DISPLAY_FRACTION: f64 = 0.05;
const MAX_SCALE: f64 = 500.0;
const E_OVER_F_K: f64 = 1000.0;
const PROUD: f64 = 0.004;
const STRESS_BLOCK_MAX_HEIGHT: f64 = 0.6;
pub(crate) const STRIP_TAG: &str = "lateral-strip";
pub(crate) const FLANGE_TAG: &str = "flange-";
pub(crate) const CRACKED_TAG: &str = "cracked-";
pub(crate) const STRESS_TAG: &str = "stress-block-";

/// 📐️ The depth of the cracked zone of a rectangular section of thickness `t` under the eccentricity `e` of the resultant: zero inside the middle third `e ≤ t/6`, otherwise `t − 3·(t/2 − e) = 3e − t/2` (triangular stress block of length `3·(t/2 − e)`), at most `t`.
pub fn cracked_depth(t: f64, e: f64) -> f64 {
    if e <= t / 6.0 { 0.0 } else { (3.0 * e - 0.5 * t).min(t) }
}

/// 🧮️ The out-of-plane deflection line of a one-metre strip of height `h` and thickness `t` of the elastic modulus `e_modulus` under the pressure `w` (N/m² on a one-metre strip), pinned at the floor and the base.
pub fn strip_line(h: f64, t: f64, e_modulus: f64, w: f64) -> Option<Curve> {
    let inertia = STRIP_WIDTH * t.powi(3) / 12.0;
    let props = Props { length: h, e: e_modulus, g: e_modulus / SHEAR_MODULUS_RATIO, a: STRIP_WIDTH * t, iy: inertia, iz: inertia, j: inertia };
    deflection(&props, Ends::Pinned, &Loads { udl: w, ..Default::default() }).ok()
}

fn overlay(curve: &Curve, direction: &Vec3, height: f64) -> Option<DeformationOverlay> {
    let samples: Vec<DisplacementSample> = curve.iter().map(|(at, u)| DisplacementSample { at: *at, delta: mul_by(direction, -u[2]) }).collect();
    let peak = samples.iter().map(|s| s.delta.length()).fold(0.0_f64, f64::max);
    (peak > 0.0).then(|| DeformationOverlay { surface: Vec::new(), motion: Motion::Ramp, load_factor: Some(1.0), kind: DeformationKind::Deformed, index: 0, frequency: None, participation: 1.0, scale: (DISPLAY_FRACTION * height / peak).clamp(1.0, MAX_SCALE), samples })
}

fn mul_by(v: &Vec3, k: f64) -> Vec3 {
    v3(v.x * k, v.y * k, v.z * k)
}

fn strip(input: &SceneInput, wall: &MasonryWall, pose: &WallPose, prof: &profile::Profile, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    if prof.wind <= 0.0 {
        return;
    }
    let e_modulus = E_OVER_F_K * m::f_k_pa(input.document.annex, wall);
    let Some(curve) = strip_line(h, t, e_modulus, prof.wind * STRIP_WIDTH) else { return };
    let (start, end) = (pose.at(0.5 * l, 0.5 * t, 0.0), pose.at(0.5 * l, 0.5 * t, h));
    let mut frame = member_frame(&start, &Vec3::Z);
    frame.y = pose.axis_x();
    frame.z = pose.axis_y();
    let geometry = Geometry::Member(MemberGeometry {
        axis: polyline(&[start.clone(), end.clone()]),
        frame,
        section: SectionShape::parametric(ProfileKind::Rect, &[("h", t), ("b", STRIP_WIDTH)]),
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Pinned,
        end_release: EndRelease::Pinned,
        stations: Vec::new(),
    });
    let id = format!("{}:{STRIP_TAG}", wall.id);
    let mut spec = Spec::new(EntityKind::Member, id, geometry, MaterialRole::Masonry, LAYER_PATTERNS, loc("Wall strip under wind (indicative)", "Wandstreifen unter Wind (indikativ)"));
    let peak = overlay(&curve, &pose.axis_y(), h).map_or(0.0, |o| o.samples.iter().map(|s| s.delta.length()).fold(0.0, f64::max));
    spec.detail = loc(&format!("One-metre strip of {:.0} mm masonry, w = {:.0} Pa, E = {:.1} GPa, largest deflection {:.2} mm", t * 1000.0, prof.wind, e_modulus / 1e9, peak * 1000.0), &format!("Ein-Meter-Streifen aus {:.0} mm Mauerwerk, w = {:.0} Pa, E = {:.1} GPa, größte Durchbiegung {:.2} mm", t * 1000.0, prof.wind, e_modulus / 1e9, peak * 1000.0));
    spec.binding = binding_of(input.report, &format!("en1996.m-31.w-rd.{}.{}", wall.id, prof.load_case));
    spec.deformation = overlay(&curve, &pose.axis_y(), h);
    out.push(spec, &[start, end]);
}

fn eccentricity_mid(wall: &MasonryWall, pose: &WallPose, prof: &profile::Profile, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let e = prof.e_mk.min(0.5 * t);
    let from = pose.at(0.5 * l, 0.5 * t, 0.5 * h);
    out.annotate(Annotation::Dimension { id: format!("{}:resultant-mid-eccentricity", wall.id), from: from.clone(), to: pose.at(0.5 * l, 0.5 * t + e, 0.5 * h), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, e), binding: None }, &from);
}

fn cracked(input: &SceneInput, wall: &MasonryWall, pose: &WallPose, prof: &profile::Profile, out: &mut Out) {
    let (h, t) = (wall.height_m, wall.thickness_m);
    let stations = [("top", 2.0 * h / 3.0, h, prof.e_top + prof.e_init, 0usize), ("mid", h / 3.0, 2.0 * h / 3.0, prof.e_mk, 1), ("bottom", 0.0, h / 3.0, prof.e_bottom + prof.e_init, 2)];
    for (tag, z0, z1, e, station) in stations {
        let depth = cracked_depth(t, e);
        if depth <= EPS {
            continue;
        }
        let mut mesh = MeshBuilder::default();
        for (x0, x1, a, b) in solid_rects(wall, z0, z1) {
            mesh.cuboid(pose, (x0, x1), (-PROUD, depth), (a, b));
        }
        if mesh.is_empty() {
            continue;
        }
        let points = mesh.points().to_vec();
        let mut spec = Spec::new(EntityKind::Solid, format!("{}:{CRACKED_TAG}{tag}", wall.id), mesh.solid(), MaterialRole::Neutral, LAYER_PATTERNS, loc("Cracked zone", "Gerissene Zone"));
        spec.detail = loc(&format!("Eccentricity {:.1} mm exceeds t/6 = {:.1} mm: the section is cracked over {:.0} mm of {:.0} mm", e * 1000.0, t / 6.0 * 1000.0, depth * 1000.0, t * 1000.0), &format!("Ausmitte {:.1} mm überschreitet t/6 = {:.1} mm: der Querschnitt ist auf {:.0} mm von {:.0} mm gerissen", e * 1000.0, t / 6.0 * 1000.0, depth * 1000.0, t * 1000.0));
        spec.binding = binding_of(input.report, &super::walls::compression_id(&wall.id, &prof.load_case, station));
        spec.opacity = 0.55;
        spec.hatch = Some(Hatch::Diagonal);
        spec.channel = Some(ColourChannel::Role);
        out.push(spec, &points);
    }
}

fn flanges(wall: &MasonryWall, pose: &WallPose, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    for flange in wall.flanges.iter().filter(|f| f.length_m > EPS && f.thickness_m > EPS) {
        let x0 = flange.position_m.clamp(0.0, l);
        let x1 = x0 + flange.thickness_m;
        let mut mesh = MeshBuilder::default();
        mesh.cuboid(pose, (x0, x1), (t, t + flange.length_m), (0.0, h));
        let points = mesh.points().to_vec();
        let mut spec = Spec::new(EntityKind::Solid, format!("{}:{FLANGE_TAG}{}", wall.id, flange.id), mesh.solid(), MaterialRole::Masonry, LAYER_MASONRY, loc(&format!("Flange {}", flange.id), &format!("Flansch {}", flange.id)));
        spec.nested = format!(".flanges[id={}]", flange.id);
        spec.detail = loc(&format!("Flange wall {:.2} m long, {:.0} mm thick at x = {:.2} m", flange.length_m, flange.thickness_m * 1000.0, x0), &format!("Flanschwand {:.2} m lang, {:.0} mm dick bei x = {:.2} m", flange.length_m, flange.thickness_m * 1000.0, x0));
        out.push(spec, &points);
    }
}

fn stress_blocks(input: &SceneInput, wall: &MasonryWall, pose: &WallPose, prof: &profile::Profile, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    for lc in &wall.load_cases {
        for c in lc.concentrated.iter() {
            if c.bearing_length_m <= EPS {
                continue;
            }
            let x = c.position_m.clamp(0.0, l);
            let sigma = c.force_n / (c.bearing_length_m * t).max(EPS);
            let ratio = sigma / prof.f_d.max(1.0);
            let depth = (STRESS_BLOCK_MAX_HEIGHT * ratio).clamp(0.02, STRESS_BLOCK_MAX_HEIGHT);
            let half = 0.5 * c.bearing_length_m;
            let mut mesh = MeshBuilder::default();
            mesh.cuboid(pose, ((x - half).max(0.0), (x + half).min(l)), (0.0, t), ((h - depth).max(0.0), h));
            let points = mesh.points().to_vec();
            let nested = format!(".loadCases[id={}].concentrated[id={}]", lc.id, c.id);
            let mut spec = Spec::new(EntityKind::Solid, format!("{}:{STRESS_TAG}{}-{}", wall.id, lc.id, c.id), mesh.solid(), MaterialRole::Load { kind: LoadKind::Permanent }, LAYER_PATTERNS, loc("Bearing stress block", "Lagerpressungs-Spannungsblock"));
            spec.nested = nested;
            spec.detail = loc(&format!("Rectangular bearing stress σ = {:.2} MPa over {:.0} mm, {:.0} % of f_d = {:.2} MPa", sigma / 1e6, c.bearing_length_m * 1000.0, ratio * 100.0, prof.f_d / 1e6), &format!("Rechteckige Lagerpressung σ = {:.2} MPa auf {:.0} mm, {:.0} % von f_d = {:.2} MPa", sigma / 1e6, c.bearing_length_m * 1000.0, ratio * 100.0, prof.f_d / 1e6));
            spec.binding = binding_of(input.report, &format!("en1996.6.1.3.concentrated.{}.{}.{}", wall.id, lc.id, c.id));
            spec.opacity = 0.5;
            spec.channel = Some(ColourChannel::Utilisation);
            out.push(spec, &points);
        }
    }
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
        flanges(wall, pose, out);
        let Some(prof) = profile::governing(input.document, wall) else { continue };
        strip(input, wall, pose, &prof, out);
        eccentricity_mid(wall, pose, &prof, out);
        cracked(input, wall, pose, &prof, out);
        stress_blocks(input, wall, pose, &prof, out);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
