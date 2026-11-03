//! 🎞️ Tracks: the fire-time and concrete-age parameter tracks of the EN 1992 scene with the scalar fields their keyframes bind (EN 1992-1-2 section temperatures, EN 1992-1-1 3.1.4 and Annex B creep coefficient and shrinkage strain).

use super::checks::fire_field;
use super::kit::*;
use super::layout::Layout;
use super::model::{MemberKind, RcMember};
use super::scene::projection_kit::tracks as kit_tracks;
use super::scene::*;
use super::SceneInput;

pub const FIRE_MINUTES: [f64; 4] = [30.0, 60.0, 90.0, 120.0];
pub const CREEP_DAYS: [f64; 5] = [28.0, 90.0, 365.0, 1825.0, 18250.0];
pub const SHRINKAGE_DAYS: [f64; 6] = [7.0, 28.0, 90.0, 365.0, 1825.0, 18250.0];
pub const LOADING_AGE_DAYS: f64 = 28.0;
pub const DRYING_START_DAYS: f64 = 3.0;
pub const RELATIVE_HUMIDITY: f64 = 50.0;
const FIRE_NODES: usize = 13;
const SIDE_COLUMNS: usize = 13;
const SIDE_ROWS: usize = 5;
const LIFT: f64 = 0.006;
pub const DAY: f64 = 86_400.0;

/// 🧵️ Creep coefficient φ(t,t0) of EN 1992-1-1 Annex B.1 for normal-hardening cement; `fcm` in MPa, `rh` in %, `h0`, `t0` and `t` in mm and days.
pub fn creep_phi(fcm: f64, rh: f64, h0: f64, t0: f64, t: f64) -> f64 {
    let t0 = t0.max(0.5);
    let (a1, a2, a3) = ((35.0 / fcm).powf(0.7), (35.0 / fcm).powf(0.2), (35.0 / fcm).powf(0.5));
    let high = fcm > 35.0;
    let drying = (1.0 - rh / 100.0) / (0.1 * h0.powf(1.0 / 3.0));
    let phi_rh = if high { (1.0 + drying * a1) * a2 } else { 1.0 + drying };
    let phi0 = phi_rh * 16.8 / fcm.sqrt() / (0.1 + t0.powf(0.2));
    let growth = 1.5 * (1.0 + (0.012 * rh).powi(18)) * h0;
    let beta_h = if high { (growth + 250.0 * a3).min(1500.0 * a3) } else { (growth + 250.0).min(1500.0) };
    phi0 * ((t - t0).max(0.0) / (beta_h + (t - t0).max(0.0))).powf(0.3)
}

/// 📉️ Total shrinkage strain ε_cs(t) of EN 1992-1-1 3.1.4(6) and Annex B.2 for normal-hardening cement; `fck` and `rh` in MPa and %, `h0` in mm, `t` and `ts` in days.
pub fn shrinkage_strain(fck: f64, rh: f64, h0: f64, t: f64, ts: f64) -> f64 {
    let fcm = fck + 8.0;
    let beta_rh = 1.55 * (1.0 - (rh / 100.0).powi(3));
    let eps_cd0 = 0.85 * (220.0 + 110.0 * 4.0) * (-0.12 * fcm / 10.0).exp() * 1e-6 * beta_rh;
    let table = [(100.0, 1.0), (200.0, 0.85), (300.0, 0.75), (500.0, 0.70)];
    let kh = match table.partition_point(|p| p.0 <= h0) {
        0 => 1.0,
        4 => 0.70,
        i => table[i - 1].1 + (table[i].1 - table[i - 1].1) * (h0 - table[i - 1].0) / (table[i].0 - table[i - 1].0),
    };
    let beta_ds = (t - ts).max(0.0) / ((t - ts).max(0.0) + 0.04 * h0.powf(1.5));
    let beta_as = 1.0 - (-0.2 * t.sqrt()).exp();
    beta_ds * kh * eps_cd0 + beta_as * 2.5 * (fck - 10.0) * 1e-6
}

/// 📐️ Notional size h0 = 2 Ac / u of a rectangular section in mm.
pub fn notional_size(b: f64, h: f64) -> f64 {
    1000.0 * b * h / (b + h)
}

pub fn side_grid(f: &Frame, span: f64, b: f64, h: f64, offset: f64) -> (Vec<Vec3>, PointGrid) {
    let points: Vec<Vec3> = (0..SIDE_ROWS).flat_map(|r| (0..SIDE_COLUMNS).map(move |c| (r, c))).map(|(r, c)| at(f, span * c as f64 / (SIDE_COLUMNS - 1) as f64, b / 2.0 + offset, -h / 2.0 + h * r as f64 / (SIDE_ROWS - 1) as f64)).collect();
    let grid = PointGrid { rows: SIDE_ROWS as u32, columns: SIDE_COLUMNS as u32, points: points.clone() };
    (points, grid)
}

pub fn spec(m: &RcMember, sub: &str, field: &str, grid: PointGrid, layer: &'static str, texts: [(&str, &str); 2], fields: Vec<ScalarField>) -> Spec {
    let active = fields[0].id.clone();
    let path = format!("members[id={}].{field}", m.id);
    let named = |en: &str, de: &str| loc(&format!("{} {en}", m.label_en), &format!("{} {de}", m.label_de));
    Spec::new(EntityKind::Surface, format!("{}:{sub}", m.id), path, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }), MaterialRole::Neutral, layer, named(texts[0].0, texts[0].1), named(texts[1].0, texts[1].1)).translucent(0.75).channel(ColourChannel::Fields { active, fields })
}

fn fire(m: &RcMember, width: f64, pose: &super::layout::MemberPose, out: &mut Out) {
    if m.fire.is_none() {
        return;
    }
    let f = &pose.frame;
    let faces = fire_field::exposed_faces(m.kind);
    let outline = if m.kind == MemberKind::FlatSlab { fire_field::rect_outline(width, m.height) } else { m.outline() };
    let fields: Vec<ScalarField> = FIRE_MINUTES.iter().map(|&min| {
        let field = fire_field::solve_outline(&outline, faces, min, fire_field::MOISTURE_DEFAULT);
        let values = (0..FIRE_NODES).flat_map(|r| (0..FIRE_NODES).map(move |c| (r, c))).map(|(r, c)| field.at(-width / 2.0 + width * c as f64 / (FIRE_NODES - 1) as f64, -m.height / 2.0 + m.height * r as f64 / (FIRE_NODES - 1) as f64)).collect();
        ScalarField { id: format!("temperature@{min}"), unit: QuantityKind::Temperature, samples: FieldSamples::Vertex { values } }
    }).collect();
    let points: Vec<Vec3> = (0..FIRE_NODES).flat_map(|r| (0..FIRE_NODES).map(move |c| (r, c))).map(|(r, c)| at(f, -LIFT, -width / 2.0 + width * c as f64 / (FIRE_NODES - 1) as f64, -m.height / 2.0 + m.height * r as f64 / (FIRE_NODES - 1) as f64)).collect();
    let grid = PointGrid { rows: FIRE_NODES as u32, columns: FIRE_NODES as u32, points: points.clone() };
    out.push(spec(m, "fire-field", "fire", grid, LAYER_FIRE, [("section temperature", "Querschnittstemperatur"), ("temperature field of the ISO 834 fire", "Temperaturfeld des ISO-834-Brandes")], fields), &points);
}

fn age(m: &RcMember, fck: f64, pose: &super::layout::MemberPose, width: f64, out: &mut Out) {
    let (fcm, h0) = (fck + 8.0, notional_size(width, m.height));
    let uniform = |value: f64| FieldSamples::Vertex { values: vec![value; SIDE_ROWS * SIDE_COLUMNS] };
    let creep = CREEP_DAYS.iter().map(|&d| ScalarField { id: format!("creep@{d}"), unit: QuantityKind::Dimensionless, samples: uniform(creep_phi(fcm, RELATIVE_HUMIDITY, h0, LOADING_AGE_DAYS, d)) });
    let shrinkage = SHRINKAGE_DAYS.iter().map(|&d| ScalarField { id: format!("shrinkage@{d}"), unit: QuantityKind::Dimensionless, samples: uniform(1000.0 * shrinkage_strain(fck, RELATIVE_HUMIDITY, h0, d, DRYING_START_DAYS)) });
    let fields: Vec<ScalarField> = creep.chain(shrinkage).collect();
    let (points, grid) = side_grid(&pose.frame, m.span, width, m.height, LIFT);
    out.push(spec(m, "age-field", "concreteGradeId", grid, LAYER_AGE, [("creep and shrinkage", "Kriechen und Schwinden"), ("creep coefficient and shrinkage strain in permille over the concrete age", "Kriechzahl und Schwinddehnung in Promille über dem Betonalter")], fields), &points);
}

fn once(tracks: &mut Vec<ParameterTrack>, track: Option<ParameterTrack>) {
    if let Some(t) = track.filter(|t| !tracks.iter().any(|o| o.id == t.id)) {
        tracks.push(t);
    }
}

/// 🎬️ Adds the fire-time field entity of every member with a fire spec, the creep and shrinkage field entity of every member, and the tracks `fire`, `creep` and `shrinkage` that scrub them.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in &input.document.members {
        if !out.wants(&m.id) {
            continue;
        }
        let Some(pose) = layout.member(&m.id) else { continue };
        if m.span <= 1e-6 || m.height <= 1e-6 || m.width <= 1e-6 {
            continue;
        }
        let width = if m.kind == MemberKind::FlatSlab { m.span } else { m.width };
        fire(m, width, pose, out);
        if let Some(grade) = input.document.concrete_grades.iter().find(|g| g.id == m.concrete_grade_id) {
            age(m, grade.f_ck / 1.0e6, pose, width, out);
        }
    }
}

/// 🎞️ Adds the tracks `fire`, `creep` and `shrinkage` to the merged scene once any member carries the field they scrub; they belong to no single member.
pub fn finish(scene: &mut StructuralScene) {
    let ids = |prefix: &str| scene.entities.iter().any(|e| matches!(&e.appearance.channel, ColourChannel::Fields { fields, .. } if fields.iter().any(|f| f.id.starts_with(prefix))));
    let (fire_on, age_on) = (ids("temperature@"), ids("creep@"));
    let mut tracks = Vec::new();
    if fire_on {
        let stops: Vec<(f64, String, String)> = FIRE_MINUTES.iter().map(|m| (m * 60.0, format!("temperature@{m}"), format!("R{m}"))).collect();
        let rows: Vec<(f64, &str, &str, &str)> = stops.iter().map(|(at, id, name)| (*at, id.as_str(), name.as_str(), name.as_str())).collect();
        once(&mut tracks, kit_tracks::timed("fire", TrackKind::Time, ("Fire exposure time", "Branddauer"), &rows));
    }
    if age_on {
        let stops: Vec<(f64, String, String, String)> = CREEP_DAYS.iter().map(|d| (d * DAY, format!("creep@{d}"), format!("{} d", d), format!("{} d", d))).collect();
        let rows: Vec<(f64, &str, &str, &str)> = stops.iter().map(|(at, id, en, de)| (*at, id.as_str(), en.as_str(), de.as_str())).collect();
        once(&mut tracks, kit_tracks::timed("creep", TrackKind::Age, ("Concrete age (creep coefficient)", "Betonalter (Kriechzahl)"), &rows));
        let stops: Vec<(f64, String, String, String)> = SHRINKAGE_DAYS.iter().map(|d| (d * DAY, format!("shrinkage@{d}"), format!("{} d", d), format!("{} d", d))).collect();
        let rows: Vec<(f64, &str, &str, &str)> = stops.iter().map(|(at, id, en, de)| (*at, id.as_str(), en.as_str(), de.as_str())).collect();
        once(&mut tracks, kit_tracks::timed("shrinkage", TrackKind::Age, ("Concrete age (shrinkage)", "Betonalter (Schwinden)"), &rows));
    }
    scene.tracks.extend(tracks);
}
