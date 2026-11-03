//! 📈️ Diagrams: locale-free `Diagram2d`s of every RC member, each with its table twin: the M-N capacity polygon with design point and utilisation ray, the section stress-strain block, tendon eccentricity and force, deflection against the span limits, the fire section and the material laws.

extern crate semio_s_artifact_norm_en1992_checks as checks;
extern crate semio_s_artifact_norm_en1992_model as model;
extern crate semio_s_artifact_norm_en1992_scene_foundation as foundation;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

use foundation::{kit, layout, mechanics, reinforcement, tracks, SceneInput};

#[path = "🔗️bond/🦀️.rs"]
pub mod bond;

#[path = "🧷️ties/🦀️.rs"]
pub mod ties;

#[path = "🔐️integrity/🦀️.rs"]
pub mod integrity;

#[path = "📊️ledger/🦀️.rs"]
pub mod ledger;

#[path = "🧲️prestress-limit/🦀️.rs"]
pub mod prestress_limit;

#[path = "🧵️material-laws/🦀️.rs"]
pub mod material_laws;

#[path = "📏️crack-width-moment/🦀️.rs"]
pub mod crack_width_moment;

#[path = "🌀️creep-curve/🦀️.rs"]
pub mod creep_curve;

#[path = "📳️fatigue-range/🦀️.rs"]
pub mod fatigue_range;

#[path = "🔥️fire-capacity/🦀️.rs"]
pub mod fire_capacity;

#[path = "🛢️liquid-pressure/🦀️.rs"]
pub mod liquid_pressure;

#[path = "🧮️second-order/🦀️.rs"]
pub mod second_order;

#[path = "🌀️torsion-interaction/🦀️.rs"]
pub mod torsion_interaction;

#[path = "🕸️strut-and-tie/🦀️.rs"]
pub mod strut_and_tie;

#[path = "✂️shear/🦀️.rs"]
pub mod shear;

#[path = "🎨️grades/🦀️.rs"]
pub mod grades;

#[path = "🪨️crack-pattern/🦀️.rs"]
pub mod crack_pattern;

#[path = "📉️prestress-loss/🦀️.rs"]
pub mod prestress_loss;

use kit::*;
use layout::{Class, Layout};
use checks::fire_field;
use model::{RcMember, TendonPoint};
use scene::diagram_kit::{interaction_polygon::{interaction_polygon, utilisation_ray}, stress_blocks::parabola_rectangle_curve};
use scene::*;
use scene::projection_kit::diagram_parts::{column};
use scene::projection_kit::diagram_parts;
use std::f64::consts::PI;

const SAMPLES: usize = 20;

pub fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, None)
}

pub fn stroke(width_px: f64, dash: Dash, tone: Tone) -> Stroke {
    Stroke { width_px, dash, tone }
}

/// 🏷️ The marker id: `id` when a check draws the point, else `reference:<id>` so a point no check ties to is explicitly a reference.
pub fn marker_id(id: &str, check: &Option<String>) -> String {
    if check.is_some() { id.to_string() } else { format!("{REFERENCE_MARKER_KIND}:{id}") }
}

pub fn xy(points: &[[f64; 2]]) -> Vec<Vec2> {
    points.iter().map(|p| Vec2::new(p[0], p[1])).collect()
}

/// 📋️ The diagram frame of a member. The table twin starts as one empty column per axis, typed by the axis (label and unit), so a caller that forgets to replace it ships a twin without rows and fails the table-twin law instead of carrying a twin without columns.
pub fn base(m: &RcMember, key: &str, title: LocalizedCopy, description: LocalizedCopy, axes: Vec<Axis>, kind: EntityKind, path: &str) -> Diagram2d {
    let table_twin = DataTable { columns: axes.iter().map(|a| DataColumn { header: a.label.clone(), unit: a.unit.clone(), values: Vec::new() }).collect() };
    Diagram2d {
        id: format!("{key}:{}", m.id),
        title,
        description,
        axes,
        series: Vec::new(),
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(entity_ref(kind, &m.id, path)),
        legend: true,
        table_twin,
    }
}

pub fn titled(m: &RcMember, en: &str, de: &str) -> LocalizedCopy {
    loc(&format!("{} - {en}", m.label_en), &format!("{} - {de}", m.label_de))
}

/// 🔷️ The convex M-N capacity hull of the general section in (kN·m, kN), compression positive: the strain planes about the y axis from a very small to a very large neutral-axis depth on both sides and the centric tension point.
pub fn capacity_hull(sec: &checks::Sec) -> Vec<[f64; 2]> {
    let (lo, hi) = sec.extent((0.0, 1.0));
    let mut points = vec![[0.0, sec.n_tension() / 1e3]];
    for i in 0..40 {
        let x = (hi - lo) * 0.02 * (100.0_f64 / 0.02).powf(i as f64 / 39.0);
        for alpha in [PI / 2.0, -PI / 2.0] {
            let r = sec.state(alpha, x);
            points.push([r.1 / 1e6, r.0 / 1e3]);
        }
    }
    interaction_polygon(&points)
}

fn interaction(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let hull = capacity_hull(sec);
    if hull.len() < 3 {
        return None;
    }
    let d = mechanics::reported_effects(input.report, m).unwrap_or_else(|| mechanics::design_effects(m, &input.document.action_factors()));
    let demand = [d.my / 1e3, d.n / 1e3];
    let hit = utilisation_ray(&hull, [0.0, 0.0], demand);
    let binding = binding(input.report, &m.id, &["en1992.6.1.flexure."]);
    let tone = tone_of(&binding);
    let mut g = base(m, "interaction-my-n", titled(m, "M-N interaction", "M-N-Interaktion"), loc("Uniaxial bending capacity polygon with the design point and the utilisation ray from the origin.", "Biegetragfähigkeits-Polygon mit Bemessungspunkt und Ausnutzungsstrahl vom Ursprung."), vec![axis("Bending moment My", "Biegemoment My", QuantityKind::Moment, "kN·m"), axis("Axial force N (compression positive)", "Normalkraft N (Druck positiv)", QuantityKind::Force, "kN")], kind, path);
    g.series.push(Series::Polygon { id: "capacity".into(), label: loc("Capacity MRd-NRd", "Tragfähigkeit MRd-NRd"), ring: xy(&hull), fill: Fill { tone: Tone::Info, opacity: 0.2 }, hatch: None });
    g.series.push(Series::Scatter { id: "design-point".into(), label: loc("Design point", "Bemessungspunkt"), points: vec![ScatterPoint { at: Vec2::new(demand[0], demand[1]), tone, label: Some(loc("MEd, NEd", "MEd, NEd")) }] });
    if demand[0].hypot(demand[1]) > 1e-9 {
        g.series.push(Series::Ray { id: "ray".into(), label: loc("Utilisation ray", "Ausnutzungsstrahl"), origin: Vec2::ZERO, through: Vec2::new(demand[0], demand[1]), to_boundary: true, style: stroke(1.5, Dash::Dashed, tone) });
    }
    g.markers.push(Marker { id: "design-marker".into(), at: Vec2::new(demand[0], demand[1]), shape: MarkerShape::Diamond, tone, label: hit.map_or_else(|| loc("Design point", "Bemessungspunkt"), |h| loc(&format!("Design point, geometric ηu = {:.2}", h.utilisation), &format!("Bemessungspunkt, geometrisch ηu = {:.2}", h.utilisation))), utilisation: hit.map(|h| h.utilisation), bound_check: binding.map(|b| b.check_id), binding: None });
    g.table_twin = DataTable { columns: vec![column("MRd", "MRd", "kN·m", hull.iter().map(|p| p[0]).collect()), column("NRd", "NRd", "kN", hull.iter().map(|p| p[1]).collect())] };
    Some(g)
}

fn stress_block(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let d = mechanics::reported_effects(input.report, m).unwrap_or_else(|| mechanics::design_effects(m, &input.document.action_factors()));
    let x = sec.solve_x(PI / 2.0, d.n)?;
    let g = sec.gross_props();
    let top = sec.top_strain(PI / 2.0, x);
    let depths: Vec<f64> = (0..=10).map(|i| (g.z_top - g.z_bot) * i as f64 / 10.0).collect();
    let strain: Vec<f64> = depths.iter().map(|u| top * (x - u) / x).collect();
    let stress: Vec<f64> = strain.iter().map(|e| checks::sigma_c_pr(&sec.c, sec.fcd, *e)).collect();
    let metres: Vec<f64> = depths.iter().map(|u| u * 1e-3).collect();
    let mut g = base(m, "stress-block", titled(m, "Section strains and stresses", "Querschnittsdehnungen und -spannungen"), loc("Strain plane and parabola-rectangle concrete stress of the polygon section at the displayed design point (EN 1992-1-1 3.1.7 and 6.1).", "Dehnungsebene und Parabel-Rechteck-Betonspannung des Polygonquerschnitts am dargestellten Bemessungspunkt (EN 1992-1-1 3.1.7 und 6.1)."), vec![axis("Depth below the compression face", "Tiefe unter dem Druckrand", QuantityKind::Length, "m"), axis("Strain (per mille) and stress (MPa)", "Dehnung (Promille) und Spannung (MPa)", QuantityKind::Dimensionless, "1")], kind, path);
    g.series.push(Series::Section {
        id: "section".into(),
        label: loc("Section", "Querschnitt"),
        shape: member_section(m, m.width, m.height),
        strain: Some(SectionProfile { points: metres.iter().zip(&strain).map(|(t, e)| Vec2::new(*t, *e)).collect(), unit: "‰".into() }),
        stress: Some(SectionProfile { points: metres.iter().zip(&stress).map(|(t, s)| Vec2::new(*t, *s)).collect(), unit: "MPa".into() }),
        neutral_axis: Some(Line2 { origin: Vec2::new(x * 1e-3, 0.0), direction: Vec2::new(0.0, 1.0) }),
    });
    g.table_twin = DataTable { columns: vec![column("Depth", "Tiefe", "m", metres), column("Strain", "Dehnung", "‰", strain), column("Concrete stress", "Betonspannung", "MPa", stress)] };
    Some(g)
}

fn moment_curvature(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let d = mechanics::reported_effects(input.report, m).unwrap_or_else(|| mechanics::design_effects(m, &input.document.action_factors()));
    let cracked = sec.moment_curvature(d.n, 48, false);
    if cracked.len() < 3 {
        return None;
    }
    let (m_cr, k_cr) = sec.cracking(d.n);
    let state_one: Vec<[f64; 2]> = sec.moment_curvature(d.n, 48, true).into_iter().take_while(|p| p.1 <= m_cr).map(|p| [p.0, p.1 / 1e6]).chain(std::iter::once([k_cr, m_cr / 1e6])).collect();
    let curve: Vec<[f64; 2]> = cracked.iter().map(|p| [p.0, p.1 / 1e6]).collect();
    let mut g = base(m, "moment-curvature", titled(m, "Moment-curvature", "Moment-Krümmung"), loc("Cracked-section moment-curvature relation at the displayed normal force with the uncracked branch up to the cracking moment (parabola-rectangle concrete, bilinear steel, tension in concrete neglected).", "Moment-Krümmungs-Beziehung des gerissenen Querschnitts bei der angezeigten Normalkraft mit dem ungerissenen Ast bis zum Rissmoment (Parabel-Rechteck-Beton, bilinearer Stahl, Betonzugfestigkeit vernachlässigt)."), vec![axis("Curvature κ", "Krümmung κ", QuantityKind::Dimensionless, "1/m"), axis("Moment My", "Moment My", QuantityKind::Moment, "kN·m")], kind, path);
    g.series.push(Series::Polyline { id: "state-i".into(), label: loc("Uncracked (state I)", "Ungerissen (Zustand I)"), points: xy(&state_one), style: stroke(2.0, Dash::Dashed, Tone::Info) });
    g.series.push(Series::Polyline { id: "state-ii".into(), label: loc("Cracked (state II)", "Gerissen (Zustand II)"), points: xy(&curve), style: stroke(2.0, Dash::Solid, Tone::Info) });
    let cracking_check = input.report.checks.iter().find(|c| c.id.starts_with("en1992.m-14.") && c.id.ends_with(&format!(".{}", m.id))).map(|c| c.id.clone());
    g.markers.push(Marker { id: marker_id("cracking", &cracking_check), at: Vec2::new(k_cr, m_cr / 1e6), shape: MarkerShape::Diamond, tone: Tone::Warning, label: loc("Cracking moment Mcr", "Rissmoment Mcr"), utilisation: None, bound_check: cracking_check, binding: None });
    g.table_twin = DataTable { columns: vec![column("Curvature κ", "Krümmung κ", "1/m", curve.iter().map(|p| p[0]).collect()), column("Moment My", "Moment My", "kN·m", curve.iter().map(|p| p[1]).collect())] };
    Some(g)
}

fn tendon(m: &RcMember, kind: EntityKind, path: &str) -> Vec<Diagram2d> {
    let Some(p) = m.prestress.as_ref().filter(|p| p.area > 0.0) else { return Vec::new() };
    let mut control: Vec<&TendonPoint> = m.tendon_points.iter().collect();
    control.sort_by(|a, b| a.x.total_cmp(&b.x));
    let (xs, ecc): (Vec<f64>, Vec<f64>) = if control.len() >= 2 {
        (control.iter().map(|t| t.x).collect(), control.iter().map(|t| -t.z * 1000.0).collect())
    } else {
        let xs: Vec<f64> = (0..=SAMPLES).map(|i| m.span * i as f64 / SAMPLES as f64).collect();
        let ecc = xs.iter().map(|x| 4.0 * p.eccentricity * (x / m.span) * (1.0 - x / m.span) * 1000.0).collect();
        (xs, ecc)
    };
    let mut e = base(m, "tendon-eccentricity", titled(m, "Tendon eccentricity", "Spanngliedexzentrizität"), loc("Parabolic tendon profile below the centroid; drag the marker to change the midspan eccentricity.", "Parabolischer Spanngliedverlauf unter dem Schwerpunkt; Marker ziehen ändert die Exzentrizität in Feldmitte."), vec![axis("Position along the member", "Lage entlang des Bauteils", QuantityKind::Length, "m"), axis("Eccentricity e", "Exzentrizität e", QuantityKind::Length, "mm")], kind, path);
    e.series.push(Series::Polyline { id: "profile".into(), label: loc("Eccentricity e(x)", "Exzentrizität e(x)"), points: xs.iter().zip(&ecc).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(2.0, Dash::Solid, Tone::Info) });
    e.markers.push(Marker { id: "reference:midspan".into(), at: Vec2::new(m.span / 2.0, m.tendon_eccentricity_mid().unwrap_or(p.eccentricity) * 1000.0), shape: MarkerShape::Diamond, tone: Tone::Info, label: loc("Midspan eccentricity", "Exzentrizität in Feldmitte"), utilisation: None, bound_check: None, binding: Some(FieldBinding::new(format!("{path}.prestress.eccentricity"), Affine { base: 0.0, k: 0.001 }, "m")) });
    e.table_twin = DataTable { columns: vec![column("Position", "Lage", "m", xs.clone()), column("Eccentricity", "Exzentrizität", "mm", ecc)] };
    let (p0, pinf) = (p.force / 1e3, p.force * (1.0 - p.loss_ratio) / 1e3);
    let mut f = base(m, "tendon-force", titled(m, "Prestressing force", "Vorspannkraft"), loc("Initial force P0 and long-term force Pm,inf along the tendon (no friction model in the document).", "Anfangskraft P0 und Langzeitkraft Pm,inf entlang des Spannglieds (kein Reibungsmodell im Dokument)."), vec![axis("Position along the member", "Lage entlang des Bauteils", QuantityKind::Length, "m"), axis("Prestressing force", "Vorspannkraft", QuantityKind::Force, "kN")], kind, path);
    f.series.push(Series::Polyline { id: "p0".into(), label: loc("P0", "P0"), points: vec![Vec2::new(0.0, p0), Vec2::new(m.span, p0)], style: stroke(2.0, Dash::Solid, Tone::Info) });
    f.series.push(Series::Polyline { id: "pinf".into(), label: loc("Pm,inf", "Pm,inf"), points: vec![Vec2::new(0.0, pinf), Vec2::new(m.span, pinf)], style: stroke(2.0, Dash::Dashed, Tone::Warning) });
    f.table_twin = DataTable { columns: vec![column("Position", "Lage", "m", vec![0.0, m.span]), column("P0", "P0", "kN", vec![p0, p0]), column("Pm,inf", "Pm,inf", "kN", vec![pinf, pinf])] };
    vec![e, f]
}

fn deflection(input: &SceneInput, m: &RcMember, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let id = format!("en1992.7.4.deflection.{}", m.id);
    let check = input.report.checks.iter().find(|c| c.id == id && !c.stations.is_empty())?;
    let stations = projection_kit::station_values(check);
    let xs: Vec<f64> = check.stations.iter().filter(|s| s.at.len() == 1).map(|s| s.at[0] * m.span).collect();
    let w: Vec<f64> = check.stations.iter().filter(|s| s.at.len() == 1).map(|s| 1000.0 * s.values.first().map_or(0.0, |q| q.value)).collect();
    let peak = w.iter().copied().fold(0.0, f64::max);
    let tone = tone_of(&Some(projection_kit::binding_of_check(check)));
    let limit = 1000.0 * check.limit.value;
    let mut g = base(m, "deflection", titled(m, "Deflection", "Durchbiegung"), loc("Quasi-permanent deflection along the span from the check stations (elastic line of the actual loads, creep and cracking interpolation of EN 1992-1-1 7.4.3) against the limits L/250 and L/500.", "Quasi-ständige Durchbiegung entlang der Spannweite aus den Prüfstellen (Biegelinie der tatsächlichen Lasten, Kriechen und Rissinterpolation nach EN 1992-1-1 7.4.3) gegen die Grenzen L/250 und L/500."), vec![axis("Position along the member", "Lage entlang des Bauteils", QuantityKind::Length, "m"), axis("Deflection w (downwards)", "Durchbiegung w (nach unten)", QuantityKind::Length, "mm")], kind, path);
    g.series.push(Series::Polyline { id: "w".into(), label: loc("Deflection w(x)", "Durchbiegung w(x)"), points: xs.iter().zip(&w).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(2.0, Dash::Solid, tone) });
    g.series.push(Series::Limit { id: "l250".into(), label: loc("L/250", "L/250"), axis: AxisRef::Y, value: limit, style: stroke(1.0, Dash::Dashed, Tone::Warning) });
    g.series.push(Series::Limit { id: "l500".into(), label: loc("L/500", "L/500"), axis: AxisRef::Y, value: limit / 2.0, style: stroke(1.0, Dash::Dotted, Tone::Info) });
    let at_peak = w.iter().position(|v| *v == peak).map_or(0.0, |i| xs[i]);
    g.markers.push(Marker { id: "wmax".into(), at: Vec2::new(at_peak, peak), shape: MarkerShape::Triangle, tone, label: loc(&format!("wmax = {peak:.1} mm"), &format!("wmax = {peak:.1} mm")), utilisation: Some(stations.iter().map(|s| s.value).fold(0.0, f64::max)), bound_check: Some(check.id.clone()), binding: None });
    g.table_twin = DataTable { columns: vec![column("Position", "Lage", "m", xs), column("Deflection", "Durchbiegung", "mm", w)] };
    Some(g)
}

fn fire(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let fire = m.fire.as_ref()?;
    let minutes = fire_field::rating_minutes(fire.rating);
    let (outline, faces) = (m.outline(), fire_field::exposed_faces(m.kind));
    let field = fire_field::solve_outline(&outline, faces, minutes, fire_field::MOISTURE_DEFAULT);
    let contour = field.isotherm_ring(fire_field::LEVEL_500, 72);
    if contour.len() < 3 {
        return None;
    }
    let zone = fire_field::zone_section(&outline, faces, &field, fire_field::zones());
    let (depth, a_z) = (field.ring_depth(fire_field::LEVEL_500, 72), zone.damage.iter().copied().fold(0.0, f64::max));
    let tone = tone_of(&binding(input.report, &m.id, &["en1992.1-2."]));
    let ring = |points: &[(f64, f64)]| points.iter().map(|&(y, z)| Vec2::new(y, z)).collect::<Vec<_>>();
    let z_bot = sec.gross_props().z_bot * 1e-3;
    let mut g = base(m, "fire-section", titled(m, "Fire section", "Brandquerschnitt"), loc("Section with the 500 degC isotherm after the rated fire duration from a 2D transient conduction solve (EN 1992-1-2 3.3 properties), the reduced section inside the isotherm (Annex B.1) and the zone-method section moved in by the damage a_z per exposed edge (Annex B.2); both reduced sections feed the N-M fire check.", "Querschnitt mit der 500-°C-Isotherme nach der Feuerwiderstandsdauer aus einer instationären 2D-Wärmeleitungsrechnung (Stoffwerte EN 1992-1-2 3.3), reduzierter Querschnitt innerhalb der Isotherme (Anhang B.1) und Zonenmethoden-Querschnitt, um die Schädigung a_z je beflammter Kante eingerückt (Anhang B.2); beide reduzierten Querschnitte speisen den N-M-Brandnachweis."), vec![axis("Section width y", "Querschnittsbreite y", QuantityKind::Length, "m"), axis("Section height z", "Querschnittshöhe z", QuantityKind::Length, "m")], kind, path);
    g.series.push(Series::Polygon { id: "section".into(), label: loc("Cold section", "Kalter Querschnitt"), ring: ring(&outline), fill: Fill { tone: Tone::Neutral, opacity: 0.1 }, hatch: None });
    g.series.push(Series::Polygon { id: "isotherm".into(), label: loc(&format!("Reduced section, 500 degC isotherm, {} mm deep", mm(depth)), &format!("Reduzierter Querschnitt, 500-°C-Isotherme, {} mm tief", mm(depth))), ring: ring(&contour), fill: Fill { tone, opacity: 0.35 }, hatch: Some(Hatch::Diagonal) });
    if zone.contour.len() >= 3 {
        g.series.push(Series::Polygon { id: "zone".into(), label: loc(&format!("Zone method section, a_z up to {} mm", mm(a_z)), &format!("Zonenmethoden-Querschnitt, a_z bis {} mm", mm(a_z))), ring: ring(&zone.contour), fill: Fill { tone: Tone::Warning, opacity: 0.15 }, hatch: Some(Hatch::CrossDiagonal) });
    }
    let temperatures: Vec<f64> = sec.bars.iter().map(|b| field.at(b.y * 1e-3, b.z * 1e-3)).collect();
    let factors: Vec<f64> = temperatures.iter().map(|t| checks::ks_theta(*t)).collect();
    g.series.push(Series::Scatter {
        id: "bars".into(),
        label: loc("Bars", "Stäbe"),
        points: sec.bars.iter().zip(temperatures.iter().zip(&factors)).map(|(b, (t, k))| ScatterPoint { at: Vec2::new(b.y * 1e-3, b.z * 1e-3), tone: Tone::Neutral, label: Some(loc(&format!("{t:.0} °C, ks = {k:.2}"), &format!("{t:.0} °C, ks = {k:.2}"))) }).collect(),
    });
    g.markers.push(Marker { id: "reference:axis-distance".into(), at: Vec2::new(0.0, z_bot + fire.axis_distance), shape: MarkerShape::Cross, tone: Tone::Info, label: loc(&format!("a = {} mm", mm(fire.axis_distance)), &format!("a = {} mm", mm(fire.axis_distance))), utilisation: None, bound_check: None, binding: Some(FieldBinding::new(format!("{path}.fire.axisDistance"), Affine { base: -z_bot, k: 1.0 }, "m")) });
    let column_of = |values: Vec<f64>| if values.is_empty() { vec![0.0] } else { values };
    let rows = sec.bars.len().max(1);
    g.table_twin = DataTable { columns: vec![column("Bar y", "Stab y", "m", column_of(sec.bars.iter().map(|b| b.y * 1e-3).collect())), column("Bar z", "Stab z", "m", column_of(sec.bars.iter().map(|b| b.z * 1e-3).collect())), column("Bar temperature", "Stabtemperatur", "°C", column_of(temperatures)), column("Strength factor ks", "Festigkeitsfaktor ks", "1", column_of(factors)), column("Isotherm depth", "Isothermentiefe", "m", vec![depth; rows])] };
    Some(g)
}

fn section_geometry(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Diagram2d {
    let g = sec.gross_props();
    let ring = |points: &[(f64, f64)], scale: f64| points.iter().map(|&(y, z)| Vec2::new(y * scale, z * scale)).collect::<Vec<_>>();
    let outline = m.outline();
    let torsion = sec.torsion_section(checks::torsion_t_min_mm(m));
    let crack = mechanics::crack_state(sec, m, &input.document.action_factors());
    let mut d = base(m, "section-geometry", titled(m, "Section geometry", "Querschnittsgeometrie"), loc("The one general section every check reads: outline, torsion wall centre line (6.3.1), effective tension area A_c,eff (7.3.2) and the derived web width, effective depth and lever arm.", "Der eine allgemeine Querschnitt, den jeder Nachweis liest: Kontur, Wandmittellinie der Torsion (6.3.1), wirksame Zugfläche A_c,eff (7.3.2) sowie abgeleitete Stegbreite, Nutzhöhe und innerer Hebelarm."), vec![axis("Section width y", "Querschnittsbreite y", QuantityKind::Length, "m"), axis("Section height z", "Querschnittshöhe z", QuantityKind::Length, "m")], kind, path);
    d.series.push(Series::Polygon { id: "section".into(), label: loc("Section", "Querschnitt"), ring: ring(&outline, 1.0), fill: Fill { tone: Tone::Neutral, opacity: 0.1 }, hatch: None });
    if let Some(t) = torsion {
        let centre = checks::fire_field::offset_polygon(&outline, &vec![t.t_ef * 1e-3 / 2.0; outline.len()]);
        if centre.len() >= 3 {
            d.series.push(Series::Polygon { id: "torsion-wall".into(), label: loc(&format!("Torsion wall centre line, t_ef = {} mm", mm(t.t_ef * 1e-3)), &format!("Torsionswand-Mittellinie, t_ef = {} mm", mm(t.t_ef * 1e-3))), ring: ring(&centre, 1.0), fill: Fill { tone: Tone::Info, opacity: 0.05 }, hatch: None });
        }
    }
    if let Some(c) = crack {
        for (i, band) in sec.band(g.z_bot, g.z_bot + c.hc_eff * 1e3).iter().enumerate() {
            d.series.push(Series::Polygon { id: format!("ac-eff-{i}"), label: loc(&format!("Effective tension area, h_c,eff = {} mm", mm(c.hc_eff)), &format!("Wirksame Zugfläche, h_c,eff = {} mm", mm(c.hc_eff))), ring: ring(band, 1e-3), fill: Fill { tone: Tone::Warning, opacity: 0.3 }, hatch: Some(Hatch::Horizontal) });
        }
    }
    let one = |en: &str, de: &str, unit: &str, value: f64| column(en, de, unit, vec![value]);
    let (depth, web) = (sec.effective_depth().unwrap_or(0.0) * 1e-3, sec.web_width() * 1e-3);
    d.table_twin = DataTable { columns: vec![
        one("Area A_c", "Fläche A_c", "m²", g.area * 1e-6),
        one("Second moment I_y", "Flächenmoment I_y", "m⁴", g.i_y * 1e-12),
        one("Web width b_w", "Stegbreite b_w", "m", web),
        one("Effective depth d", "Nutzhöhe d", "m", depth),
        one("Lever arm z = 0.9 d", "Hebelarm z = 0,9 d", "m", 0.9 * depth),
        one("Effective wall t_ef", "Wirksame Wanddicke t_ef", "m", torsion.map_or(0.0, |t| t.t_ef * 1e-3)),
        one("Enclosed area A_k", "Eingeschlossene Fläche A_k", "m²", torsion.map_or(0.0, |t| t.a_k * 1e-6)),
        one("Effective tension height h_c,eff", "Wirksame Zugzonenhöhe h_c,eff", "m", crack.map_or(0.0, |c| c.hc_eff)),
    ] };
    d
}

fn materials(m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Diagram2d {
    let concrete: Vec<[f64; 2]> = parabola_rectangle_curve(sec.c.fck, sec.fcd, 40);
    let yield_strain = 1000.0 * sec.fyd / sec.es;
    let steel = [[0.0, 0.0], [yield_strain, sec.fyd], [sec.eps_ud, sec.k * sec.fyd]];
    let mut g = base(m, "material-laws", titled(m, "Material laws", "Werkstoffgesetze"), loc("Design stress-strain laws of concrete (parabola-rectangle) and reinforcing steel (inclined top branch to k f_yd at the design strain).", "Bemessungs-Spannungs-Dehnungs-Linien von Beton (Parabel-Rechteck) und Betonstahl (geneigter oberer Ast bis k f_yd bei der Bemessungsdehnung)."), vec![axis("Strain", "Dehnung", QuantityKind::Dimensionless, "‰"), axis("Stress", "Spannung", QuantityKind::Stress, "MPa")], kind, path);
    g.series.push(Series::Polyline { id: "concrete".into(), label: loc("Concrete", "Beton"), points: xy(&concrete), style: stroke(2.0, Dash::Solid, Tone::Info) });
    g.series.push(Series::Polyline { id: "steel".into(), label: loc("Reinforcing steel", "Betonstahl"), points: xy(&steel), style: stroke(2.0, Dash::Dashed, Tone::Warning) });
    g.table_twin = DataTable { columns: vec![column("Concrete strain", "Betondehnung", "‰", concrete.iter().map(|p| p[0]).collect()), column("Concrete stress", "Betonspannung", "MPa", concrete.iter().map(|p| p[1]).collect())] };
    g
}

pub fn diagrams(input: &SceneInput, layout: &Layout) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for m in &input.document.members {
        let Some(pose) = layout.member(&m.id) else { continue };
        if m.span <= 1e-6 || m.height <= 1e-6 || m.width <= 1e-6 {
            continue;
        }
        let kind = if pose.class == Class::Slab { EntityKind::Plate } else { EntityKind::Member };
        let path = format!("members[id={}]", m.id);
        let Some(sec) = mechanics::member_sec(input.document, m) else { continue };
        if !sec.bars.is_empty() {
            out.extend(interaction(input, m, &sec, kind, &path));
            out.extend(moment_curvature(input, m, &sec, kind, &path));
            out.extend(stress_block(input, m, &sec, kind, &path));
        }
        out.extend(tendon(m, kind, &path));
        out.extend(prestress_loss::diagram(input.document, m, kind, &path));
        if let Some(grade) = input.document.concrete(&m.concrete_grade_id) {
            out.push(creep_curve::diagram(m, grade.f_ck / 1.0e6, kind, &path));
        }
        if pose.class != Class::Upright {
            out.extend(deflection(input, m, kind, &path));
            let width = if m.kind == model::MemberKind::FlatSlab { m.span } else { m.width };
            out.extend(fatigue_range::diagram(input.document, m, width, kind, &path));
            out.extend(crack_pattern::diagram(input, m, &sec, kind, &path, pose.class));
        }
        out.extend(ledger::member(input, m, kind, &path));
        out.extend(bond::diagrams(input, m, kind, &path));
        out.extend(prestress_limit::diagram(input, m, kind, &path));
        out.extend(torsion_interaction::diagram(input, m, &sec, kind, &path));
        out.extend(second_order::diagram(input, m, &sec, kind, &path));
        out.extend(crack_width_moment::diagram(input, m, &sec, kind, &path));
        out.push(section_geometry(input, m, &sec, kind, &path));
        out.extend(fire(input, m, &sec, kind, &path));
        out.extend(fire_capacity::diagram(input, m, &sec, kind, &path));
        out.extend(liquid_pressure::diagram(input, m, kind, &path));
        out.push(materials(m, &sec, kind, &path));
    }
    out.extend(shear::diagrams(input, layout));
    out.extend(strut_and_tie::diagrams(input, layout));
    out.extend(grades::legend_diagram(input.document));
    out.extend(material_laws::diagrams(input));
    out.extend(ties::diagrams(input));
    out.extend(integrity::diagram(input));
    out.extend(ledger::rows(input));
    out.extend(input.document.anchors.iter().filter_map(|a| projection_kit::design_case::case_diagram(&a.id, &loc(&format!("Anchor {}", a.id), &format!("Dübel {}", a.id)), input.report, None)));
    out.extend(placements(input.document));
    out
}

/// 📍️ The plan of the placed entities: one diamond per placement at its (x, y) that drags both leaves, the yaw in the label, with a table twin of id, x, y, z and yaw.
fn placements(document: &model::En1992Snapshot) -> Option<Diagram2d> {
    if document.placements.is_empty() {
        return None;
    }
    let markers = document
        .placements
        .iter()
        .map(|p| Marker {
            id: format!("reference:placement-{}", p.id),
            at: Vec2::new(p.x, p.y),
            shape: MarkerShape::Diamond,
            tone: Tone::Info,
            label: loc(&format!("{} at ({:.2}, {:.2}) m, yaw {:.0} deg", p.id, p.x, p.y, p.yaw.to_degrees()), &format!("{} bei ({:.2}, {:.2}) m, Drehung {:.0} Grad", p.id, p.x, p.y, p.yaw.to_degrees())),
            utilisation: None,
            bound_check: None,
            binding: Some(FieldBinding { path: format!("placements[id={}].x", p.id), map: Affine::IDENTITY, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.01 }) }),
        })
        .collect();
    let notes = document.placements.iter().map(|p| DiagramNote { id: format!("name-{}", p.id), at: Vec2::new(p.x, p.y + 0.3), text: loc(&p.id, &p.id), tone: Tone::Neutral }).collect();
    let column_of = |value: &dyn Fn(&model::Placement) -> f64| document.placements.iter().map(value).collect::<Vec<f64>>();
    Some(Diagram2d {
        id: "placements:document".into(),
        title: loc("Placement plan", "Platzierungsplan"),
        description: loc("The persisted position of every placed entity in plan; a diamond drags the x position.", "Die gespeicherte Lage jeder platzierten Entität im Grundriss; eine Raute verschiebt die x-Lage."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m"), axis("Position y", "Lage y", QuantityKind::Length, "m")],
        series: Vec::new(),
        markers,
        annotations: notes,
        focus: None,
        legend: false,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", column_of(&|p| p.x)), column("Position y", "Lage y", "m", column_of(&|p| p.y)), column("Position z", "Position z", "m", column_of(&|p| p.z)), column("Yaw", "Drehung", "rad", column_of(&|p| p.yaw))] },
    })
}
