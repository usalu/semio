//! 📈️ Entity diagrams: the joint moment-rotation characteristic with its classification limits, χ_LT(λ̄_LT) and the M_cr mode shape of a restraint, the weld stress vectors of a weld group, the hollow joint resistance over the chord wall and the parametric section outline against its section row.

use super::checks::clauses::{hollow_joints as hj, joint_components as jc, weld_group as wg};
use super::checks::{fire_ltb_summary, frame_eigen, joint_frame, joint_row_summary, restraint_summary, torsional_summary};
use super::model::{En1993Snapshot, FireExposure, HollowJoint, ParametricSection, Restraint, SteelMember, WeldGroup};
use super::norm::CheckReport;
use super::scene::diagram_kit::{curves, Xy};
use super::scene::projection_kit::diagram_parts::{axis, column, stroke};
use super::scene::*;

const SAMPLES: usize = 60;
const LAMBDA_MAX: f64 = 3.0;
const VECTORS_PER_LINE: usize = 9;
const PSI_END_PLATE: f64 = 2.7;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn point(q: Xy) -> Vec2 {
    Vec2::new(q[0], q[1])
}

fn worst_of(report: &CheckReport, entity: &str) -> f64 {
    report.checks.iter().filter(|c| c.subject.entity_id == entity && c.utilization.is_finite()).map(|c| c.utilization).fold(0.0, f64::max)
}

/// 🎚️ The stiffness classification zones of EN 1993-1-8 5.2.2.1 as bands over the M–φ plane: nominally pinned below S_j,ini = 0.5 EI_b/L_b, rigid above the rigid boundary, semi-rigid between (slopes in kN·m/mrad).
pub(crate) fn classification_bands(pinned: f64, rigid: f64, x_max: f64, y_max: f64) -> Vec<Series> {
    const STEPS: usize = 16;
    let line = |k: f64| -> Vec<Vec2> { (0..=STEPS).map(|i| { let x = x_max * i as f64 / STEPS as f64; Vec2::new(x, (k * x).min(y_max)) }).collect() };
    let flat = |y: f64| -> Vec<Vec2> { (0..=STEPS).map(|i| Vec2::new(x_max * i as f64 / STEPS as f64, y)).collect() };
    let band = |id: &str, en: &str, de: &str, lower: Vec<Vec2>, upper: Vec<Vec2>, tone: Tone| Series::Band { id: id.into(), label: loc(en, de), lower, upper, fill: Fill { tone, opacity: 0.12 } };
    vec![
        band("zone-pinned", "Nominally pinned zone", "Zone gelenkig", flat(0.0), line(pinned), Tone::Warning),
        band("zone-semi-rigid", "Semi-rigid zone", "Zone verformbar", line(pinned), line(rigid), Tone::Info),
        band("zone-rigid", "Rigid zone", "Zone starr", line(rigid), flat(y_max), Tone::Neutral),
    ]
}

/// 📈️ M–φ of an end plate joint: the characteristic of EN 1993-1-8 6.3.1 up to M_j,Rd, the 2/3 M_j,Rd limit, the rigid and nominally pinned boundaries of 5.2.2.1 and the operating point of M_j,Ed.
pub(crate) fn joint_moment_rotation(d: &En1993Snapshot, report: &CheckReport, joint_id: &str) -> Option<Diagram2d> {
    let s = joint_row_summary(d, joint_id)?;
    let phi_max = s.curve.last().map_or(1e-3, |(phi, _)| *phi).max(1e-6);
    let m_rd_k = s.m_j_rd / 1e3;
    let points: Vec<Vec2> = s.curve.iter().map(|(phi, m)| Vec2::new(phi * 1e3, m / 1e3)).collect();
    let phi_ed = if s.m_j_ed > 0.0 { s.m_j_ed / (s.s_ini / jc::mu_stiffness(s.m_j_ed, s.m_j_rd, PSI_END_PLATE)) } else { 0.0 };
    let utilisation = if s.m_j_rd > 0.0 { s.m_j_ed / s.m_j_rd } else { 0.0 };
    let check_tone = tone(utilisation.max(worst_of(report, joint_id)));
    let (x_max, y_max) = (phi_max * 1.3e3, m_rd_k.max(s.m_j_ed / 1e3) * 1.25);
    let bands = classification_bands(s.pinned_limit / 1e6, s.rigid_limit / 1e6, x_max, y_max);
    let ray = |id: &str, en: &str, de: &str, stiffness: f64, style: Stroke| Series::Ray { id: id.into(), label: loc(en, de), origin: Vec2::ZERO, through: Vec2::new(1.0, stiffness / 1e6), to_boundary: true, style };
    Some(Diagram2d {
        id: format!("mphi:{joint_id}"),
        title: loc(&format!("Moment–rotation characteristic – {joint_id}"), &format!("Momenten-Rotations-Kennlinie – {joint_id}")),
        description: loc(
            &format!("φ(M) of the end plate joint from the component method (EN 1993-1-8 6.3.1): S_j,ini = {:.1} MNm/rad is {}, M_j,Rd = {:.1} kNm is {} strength; rigid limit {:.1}, pinned limit {:.1} MNm/rad.", s.s_ini / 1e6, s.stiffness_class, m_rd_k, s.strength_class, s.rigid_limit / 1e6, s.pinned_limit / 1e6),
            &format!("φ(M) des Stirnplattenanschlusses nach der Komponentenmethode (EN 1993-1-8 6.3.1): S_j,ini = {:.1} MNm/rad ist {}, M_j,Rd = {:.1} kNm ist {} Tragfähigkeit; Grenze starr {:.1}, gelenkig {:.1} MNm/rad.", s.s_ini / 1e6, s.stiffness_class, m_rd_k, s.strength_class, s.rigid_limit / 1e6, s.pinned_limit / 1e6),
        ),
        axes: vec![axis("Rotation φ", "Verdrehung φ", QuantityKind::Angle, "mrad", AxisScale::Linear, Some((0.0, phi_max * 1.3e3))), axis("Moment M", "Moment M", QuantityKind::Moment, "kN·m", AxisScale::Linear, Some((0.0, m_rd_k.max(s.m_j_ed / 1e3) * 1.25)))],
        series: bands.into_iter().chain([
            Series::Polyline { id: "m-phi".into(), label: loc("φ(M)", "φ(M)"), points: points.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "m-rd".into(), label: loc("M_j,Rd", "M_j,Rd"), axis: AxisRef::Y, value: m_rd_k, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
            Series::Limit { id: "m-23".into(), label: loc("2/3 M_j,Rd", "2/3 M_j,Rd"), axis: AxisRef::Y, value: m_rd_k * 2.0 / 3.0, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
            ray("rigid", "Rigid limit", "Grenze starr", s.rigid_limit, stroke(Tone::Neutral, 1.0, Dash::Dashed)),
            ray("pinned", "Nominally pinned limit", "Grenze gelenkig", s.pinned_limit, stroke(Tone::Neutral, 1.0, Dash::Dotted)),
        ]).collect(),
        markers: vec![Marker { id: "m-ed".into(), at: Vec2::new(phi_ed * 1e3, s.m_j_ed / 1e3), shape: MarkerShape::Circle, tone: check_tone, label: loc(&format!("M_j,Ed = {:.1} kNm", s.m_j_ed / 1e3), &format!("M_j,Ed = {:.1} kNm", s.m_j_ed / 1e3)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-8.6.3.mj.{joint_id}")), binding: None }],
        annotations: vec![DiagramNote { id: "class".into(), at: Vec2::new(phi_max * 0.05e3, m_rd_k * 1.1), text: loc(&format!("{} / {}", s.stiffness_class, s.strength_class), &format!("{} / {}", s.stiffness_class, s.strength_class)), tone: Tone::Neutral }],
        focus: Some(EntityRef::new(EntityKind::Group, joint_id, format!("joints[id={joint_id}]"))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Rotation φ", "Verdrehung φ", "mrad", points.iter().map(|q| q.x).collect()), column("Moment M", "Moment M", "kN·m", points.iter().map(|q| q.y).collect())] },
    })
}

/// 📊️ Component resistances of an end plate joint (EN 1993-1-8 6.2.6): per bolt row the column flange in bending (6.2.6.4), the end plate in bending (6.2.6.5) and the column web in tension (6.2.6.3), then the column web in compression (6.2.6.2), the web panel in shear (6.2.6.1) and the beam flange and web in compression (6.2.6.7); the governing component of each row and the governing compression component are drawn in the error tone, the plastic row force F_tr,Rd as a limit.
pub(crate) fn joint_components(d: &En1993Snapshot, report: &CheckReport, joint_id: &str) -> Option<Diagram2d> {
    let s = joint_row_summary(d, joint_id)?;
    let kn = |n: f64| n / 1e3;
    let x_max = s.component_forces.iter().flatten().copied().chain([s.column_web_compression, s.panel_shear, s.beam_compression]).filter(|v| v.is_finite()).fold(0.0_f64, f64::max).max(1.0) * 1.1 / 1e3;
    let name = |index: usize| s.row_ids.get(index).cloned().unwrap_or_else(|| format!("row-{}", index + 1));
    let mut bars: Vec<(String, LocalizedCopy, f64, bool)> = Vec::new();
    for (index, forces) in s.component_forces.iter().enumerate() {
        let (row, governing) = (name(index), s.governing[index]);
        let entries = [(jc::RowComponent::ColumnFlangeBending, "6.2.6.4", forces[0]), (jc::RowComponent::EndPlateBending, "6.2.6.5", forces[1]), (jc::RowComponent::ColumnWebTension, "6.2.6.3", forces[2]), (jc::RowComponent::BeamWebTension, "6.2.6.8", forces[3])];
        for (component, clause, force) in entries.into_iter().filter(|(_, _, force)| force.is_finite()) {
            bars.push((format!("{row}:{clause}"), loc(&format!("{row}: {} ({clause})", component.en()), &format!("{row}: {} ({clause})", component.de())), force, component == governing));
        }
    }
    let cap = s.compression_cap;
    bars.push(("compression:6.2.6.2".into(), loc("Column web in compression (6.2.6.2)", "Stützensteg auf Druck (6.2.6.2)"), s.column_web_compression, s.column_web_compression == cap));
    bars.push(("compression:6.2.6.1".into(), loc("Column web panel in shear (6.2.6.1)", "Stützenstegfeld auf Schub (6.2.6.1)"), s.panel_shear, s.panel_shear == cap));
    bars.push(("compression:6.2.6.7".into(), loc("Beam flange and web in compression (6.2.6.7)", "Trägerflansch und -steg auf Druck (6.2.6.7)"), s.beam_compression, s.beam_compression == cap));
    let utilisation = worst_of(report, joint_id).max(report.checks.iter().filter(|c| c.id == format!("en1993.1-8.6.2.comp.{joint_id}") && c.utilization.is_finite()).map(|c| c.utilization).fold(0.0, f64::max));
    let series: Vec<Series> = bars
        .iter()
        .enumerate()
        .map(|(index, (id, label, force, governing))| {
            let y = index as f64;
            let fill = if *governing { Tone::Error } else { Tone::Info };
            Series::Polygon { id: id.clone(), label: label.clone(), ring: vec![Vec2::new(0.0, y), Vec2::new(kn(*force), y), Vec2::new(kn(*force), y + 0.8), Vec2::new(0.0, y + 0.8)], fill: Fill { tone: fill, opacity: 0.55 }, hatch: None }
        })
        .collect();
    let row_total: f64 = s.row_forces.iter().sum();
    Some(Diagram2d {
        id: format!("components:{joint_id}"),
        title: loc(&format!("Component resistances – {joint_id}"), &format!("Komponententragfähigkeiten – {joint_id}")),
        description: loc(
            &format!("Resistance of every joint component (EN 1993-1-8 6.2.6) per bolt row and in the compression zone; the governing component is highlighted. Compression cap {:.0} kN, plastic row forces sum to {:.0} kN, M_j,Rd = {:.1} kNm (utilisation {:.2}).", kn(cap), kn(row_total), s.m_j_rd / 1e3, utilisation),
            &format!("Tragfähigkeit jeder Anschlusskomponente (EN 1993-1-8 6.2.6) je Schraubenreihe und in der Druckzone; die maßgebende Komponente ist hervorgehoben. Druckgrenze {:.0} kN, plastische Reihenkräfte summieren sich auf {:.0} kN, M_j,Rd = {:.1} kNm (Ausnutzung {:.2}).", kn(cap), kn(row_total), s.m_j_rd / 1e3, utilisation),
        ),
        axes: vec![axis("Resistance F_Rd", "Tragfähigkeit F_Rd", QuantityKind::Force, "kN", AxisScale::Linear, Some((0.0, x_max))), axis("Component", "Komponente", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, bars.len() as f64)))],
        series: series.into_iter().chain([Series::Limit { id: "compression-cap".into(), label: loc("Compression cap", "Druckgrenze"), axis: AxisRef::X, value: kn(cap), style: stroke(Tone::Error, 1.5, Dash::Dashed) }]).collect(),
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, joint_id, format!("joints[id={joint_id}]"))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Component", "Komponente", "1", (0..bars.len()).map(|i| i as f64).collect()), column("Resistance F_Rd", "Tragfähigkeit F_Rd", "kN", bars.iter().map(|b| kn(b.2)).collect())] },
    })
}

fn restraint_curves(d: &En1993Snapshot, report: &CheckReport, r: &Restraint) -> Vec<Diagram2d> {
    let Some(s) = restraint_summary(d, &r.id) else { return Vec::new() };
    let curve = curves::buckling_curve(s.alpha_lt, LAMBDA_MAX, SAMPLES);
    let utilisation = if s.m_b_rd > 0.0 { s.m_ed / s.m_b_rd } else { 0.0 };
    let focus = Some(EntityRef::new(EntityKind::Group, &r.id, format!("restraints[id={}]", r.id)));
    let chi = Diagram2d {
        id: format!("chi-lt:{}", r.id),
        title: loc(&format!("Lateral-torsional buckling curve – {}", r.id), &format!("Biegedrillknicklinie – {}", r.id)),
        description: loc(
            &format!("χ_LT over λ̄_LT (EN 1993-1-1 6.3.2.2, α_LT = {:.2}) with the operating point of the real restraints: M_cr = {:.1} kNm, λ̄_LT = {:.3}, χ_LT = {:.3}.", s.alpha_lt, s.mcr / 1e3, s.lambda_lt, s.chi_lt),
            &format!("χ_LT über λ̄_LT (EN 1993-1-1 6.3.2.2, α_LT = {:.2}) mit dem Arbeitspunkt der tatsächlichen Halterungen: M_cr = {:.1} kNm, λ̄_LT = {:.3}, χ_LT = {:.3}.", s.alpha_lt, s.mcr / 1e3, s.lambda_lt, s.chi_lt),
        ),
        axes: vec![axis("Slenderness λ̄_LT", "Schlankheit λ̄_LT", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction χ_LT", "Abminderung χ_LT", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.05)))],
        series: vec![Series::Polyline { id: "chi-lt".into(), label: loc("χ_LT", "χ_LT"), points: curve.iter().copied().map(point).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) }],
        markers: vec![Marker { id: "operating".into(), at: Vec2::new(s.lambda_lt.min(LAMBDA_MAX), s.chi_lt), shape: MarkerShape::Circle, tone: tone(utilisation.max(worst_of(report, &r.id))), label: loc(&format!("M_Ed/M_b,Rd = {utilisation:.2}"), &format!("M_Ed/M_b,Rd = {utilisation:.2}")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-1.6.3.2.mcrfe.{}", r.id)), binding: None }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: false,
        table_twin: DataTable { columns: vec![column("Slenderness λ̄_LT", "Schlankheit λ̄_LT", "1", curve.iter().map(|q| q[0]).collect()), column("Reduction χ_LT", "Abminderung χ_LT", "1", curve.iter().map(|q| q[1]).collect())] },
    };
    let peak = |values: &[f64]| values.iter().fold(0.0_f64, |a, b| a.max(b.abs())).max(1e-30);
    let (pv, pp) = (peak(&s.mode_v), peak(&s.mode_phi));
    let at = |values: &[f64], scale: f64| -> Vec<Vec2> { s.stations.iter().zip(values).map(|(x, v)| Vec2::new(*x, v / scale)).collect() };
    let mode = Diagram2d {
        id: format!("mcr-mode:{}", r.id),
        title: loc(&format!("M_cr mode shape – {}", r.id), &format!("M_cr-Eigenform – {}", r.id)),
        description: loc("Lateral displacement v and twist φ of the first lateral-torsional buckling mode along the member, normalised to their maxima (Hermite finite elements with the actual supports, springs and load height).", "Seitliche Verschiebung v und Verdrehung φ der ersten Biegedrillknickform entlang des Bauteils, auf ihre Maxima normiert (Hermite-Finite-Elemente mit den tatsächlichen Lagern, Bettungen und Lastangriffshöhe)."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, s.stations.last().copied().unwrap_or(1.0)))), axis("Normalised amplitude", "Normierte Amplitude", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((-1.1, 1.1)))],
        series: vec![
            Series::Polyline { id: "v".into(), label: loc("Lateral displacement v", "Seitliche Verschiebung v"), points: at(&s.mode_v, pv), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "phi".into(), label: loc("Twist φ", "Verdrehung φ"), points: at(&s.mode_phi, pp), style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
        ],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "mcr".into(), at: Vec2::new(s.stations.last().copied().unwrap_or(1.0) * 0.5, 1.0), text: loc(&format!("M_cr = {:.1} kNm", s.mcr / 1e3), &format!("M_cr = {:.1} kNm", s.mcr / 1e3)), tone: Tone::Neutral }],
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", s.stations.clone()), column("Lateral displacement v", "Seitliche Verschiebung v", "1", s.mode_v.iter().map(|v| v / pv).collect()), column("Twist φ", "Verdrehung φ", "1", s.mode_phi.iter().map(|v| v / pp).collect())] },
    };
    vec![chi, mode]
}

fn weld_pattern(g: &WeldGroup) -> wg::WeldPattern {
    match g.pattern {
        semio_s_artifact_norm_en1993_model::WeldPattern::Line => wg::WeldPattern::Line,
        semio_s_artifact_norm_en1993_model::WeldPattern::Box => wg::WeldPattern::Box,
        _ => wg::WeldPattern::TwoLines,
    }
}

fn weld_vectors(d: &En1993Snapshot, report: &CheckReport, g: &WeldGroup) -> Option<Diagram2d> {
    if g.depth <= 0.0 || g.shear_force == 0.0 {
        return None;
    }
    let (b, depth, f, e) = (g.width * 1e3, g.depth * 1e3, g.shear_force.abs(), g.eccentricity * 1e3);
    let props = wg::group_properties(weld_pattern(g), b, depth);
    let points = wg::sample_points(weld_pattern(g), b, depth, VECTORS_PER_LINE);
    let vectors: Vec<(f64, f64, f64, f64)> = points.iter().map(|(x, y)| { let (qx, qy) = wg::vector_at(&props, f, e, *x, *y); (*x, *y, qx, qy) }).collect();
    let q_max = wg::max_force_per_length(&props, f, e).max(1e-9);
    let scale = 0.18 * depth.max(b) / q_max;
    let rd = wg::resistance_per_length(g.throat * 1e3, g.steel_fu / 1e6, g.beta_w, super::checks::AnnexParams::for_choice(d.annex).gamma_m2);
    let utilisation = q_max / rd.max(1e-9);
    let line_tone = tone(utilisation.max(worst_of(report, &g.id)));
    let outline: Vec<Series> = points.chunks(VECTORS_PER_LINE).enumerate().map(|(i, line)| Series::Polyline { id: format!("weld-{i}"), label: loc("Weld line", "Schweißnaht"), points: line.iter().map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Neutral, 3.0, Dash::Solid) }).collect();
    let arrows = vectors.iter().enumerate().map(|(i, (x, y, qx, qy))| Series::Ray { id: format!("q-{i}"), label: loc("Force per length", "Kraft je Länge"), origin: Vec2::new(*x, *y), through: Vec2::new(x + qx * scale, y + qy * scale), to_boundary: false, style: stroke(line_tone, 1.5, Dash::Solid) });
    Some(Diagram2d {
        id: format!("weld-vectors:{}", g.id),
        title: loc(&format!("Weld stress vectors – {}", g.id), &format!("Nahtspannungsvektoren – {}", g.id)),
        description: loc(
            &format!("Elastic vector method (EN 1993-1-8 4.5.3.3): direct shear F/L_w plus torsion T r/I_p along the weld lines; largest F_w,Ed = {q_max:.1} N/mm against F_w,Rd = {rd:.1} N/mm."),
            &format!("Elastisches Vektorverfahren (EN 1993-1-8 4.5.3.3): Querkraftanteil F/L_w plus Torsion T r/I_p entlang der Nähte; größte Beanspruchung F_w,Ed = {q_max:.1} N/mm gegenüber F_w,Rd = {rd:.1} N/mm."),
        ),
        axes: vec![axis("Width x", "Breite x", QuantityKind::Length, "mm", AxisScale::Linear, Some((-0.75 * depth.max(b), 0.75 * depth.max(b)))), axis("Depth y", "Tiefe y", QuantityKind::Length, "mm", AxisScale::Linear, Some((-0.75 * depth, 0.75 * depth)))],
        series: outline.into_iter().chain(arrows).collect(),
        markers: vec![Marker { id: "corner".into(), at: Vec2::new(props.x_max, props.y_max), shape: MarkerShape::Circle, tone: line_tone, label: loc(&format!("F_w,Ed = {q_max:.1} N/mm"), &format!("F_w,Ed = {q_max:.1} N/mm")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-8.4.5.group.{}", g.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &g.id, format!("weldGroups[id={}]", g.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Point x", "Punkt x", "mm", vectors.iter().map(|v| v.0).collect()), column("Point y", "Punkt y", "mm", vectors.iter().map(|v| v.1).collect()), column("q_x", "q_x", "N/mm", vectors.iter().map(|v| v.2).collect()), column("q_y", "q_y", "N/mm", vectors.iter().map(|v| v.3).collect())] },
    })
}

fn hollow_resistance(j: &HollowJoint) -> Option<Diagram2d> {
    if j.chord_thickness <= 0.0 || j.chord_width <= 0.0 || j.brace1_angle <= 0.0 || j.brace2_angle <= 0.0 || j.brace1_thickness <= 0.0 || j.chord_fy <= 0.0 {
        return None;
    }
    let gm5 = 1.0;
    let at = |t0_mm: f64| hj::GapJoint { b0: j.chord_width * 1e3, t0: t0_mm, fy0: j.chord_fy / 1e6, sigma0: j.chord_stress / 1e6, b1: j.brace1_width * 1e3, h1: j.brace1_height * 1e3, t1: j.brace1_thickness * 1e3, theta1: j.brace1_angle, b2: j.brace2_width * 1e3, h2: j.brace2_height * 1e3, t2: j.brace2_thickness * 1e3, theta2: j.brace2_angle, gap: j.gap * 1e3, fyi: j.brace_fy / 1e6 };
    let t0 = j.chord_thickness * 1e3;
    let ts: Vec<f64> = (0..=SAMPLES).map(|i| 0.5 * t0 + 1.5 * t0 * i as f64 / SAMPLES as f64).collect();
    let face: Vec<Vec2> = ts.iter().map(|t| Vec2::new(*t, hj::chord_face_rd(&at(*t), gm5).0 / 1e3)).collect();
    let brace: Vec<Vec2> = ts.iter().map(|t| { let g = at(*t); Vec2::new(*t, hj::brace_effective_width_rd(&g, g.h1, g.b1, g.t1, gm5) / 1e3) }).collect();
    let force = j.brace1_force.abs() / 1e3;
    let n_rd = hj::chord_face_rd(&at(t0), gm5).0.min(hj::brace_effective_width_rd(&at(t0), j.brace1_height * 1e3, j.brace1_width * 1e3, j.brace1_thickness * 1e3, gm5)) / 1e3;
    let utilisation = if n_rd > 0.0 { force / n_rd } else { 0.0 };
    let top = face.iter().chain(&brace).map(|p| p.y).fold(force, f64::max) * 1.1;
    Some(Diagram2d {
        id: format!("hollow:{}", j.id),
        title: loc(&format!("Hollow joint resistance over the chord wall – {}", j.id), &format!("Hohlprofilknoten-Tragfähigkeit über der Gurtwand – {}", j.id)),
        description: loc("Brace 1 resistance of the gap joint over the chord wall thickness t_0: chord face failure and brace effective width (EN 1993-1-8 Table 7.11, unverified coefficients) against the brace force.", "Tragfähigkeit der Strebe 1 des Knotens mit Spalt über der Gurtwanddicke t_0: Gurtflanschversagen und mittragende Breite der Strebe (EN 1993-1-8 Tabelle 7.11, nicht verifizierte Beiwerte) gegenüber der Strebenkraft."),
        axes: vec![axis("Chord wall t_0", "Gurtwand t_0", QuantityKind::Length, "mm", AxisScale::Linear, Some((0.5 * t0, 2.0 * t0))), axis("Resistance N_1,Rd", "Tragfähigkeit N_1,Rd", QuantityKind::Force, "kN", AxisScale::Linear, Some((0.0, top)))],
        series: vec![
            Series::Polyline { id: "face".into(), label: loc("Chord face failure", "Gurtflanschversagen"), points: face.clone(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "brace".into(), label: loc("Brace effective width", "Mittragende Breite der Strebe"), points: brace.clone(), style: stroke(Tone::Neutral, 2.0, Dash::Dashed) },
            Series::Limit { id: "force".into(), label: loc("N_1,Ed", "N_1,Ed"), axis: AxisRef::Y, value: force, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "t0".into(), at: Vec2::new(t0, n_rd), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("t_0 = {t0:.1} mm: N_1,Rd = {n_rd:.0} kN"), &format!("t_0 = {t0:.1} mm: N_1,Rd = {n_rd:.0} kN")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-8.7.4.gap.{}", j.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &j.id, format!("hollowJoints[id={}]", j.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Chord wall t_0", "Gurtwand t_0", "mm", ts.clone()), column("Chord face failure", "Gurtflanschversagen", "kN", face.iter().map(|p| p.y).collect()), column("Brace effective width", "Mittragende Breite der Strebe", "kN", brace.iter().map(|p| p.y).collect())] },
    })
}

fn parametric_outline(d: &En1993Snapshot, p: &ParametricSection) -> Option<Diagram2d> {
    let section = d.sections.iter().find(|s| s.id == p.section_id)?;
    let shape = super::kit::section_shape(section);
    let extent = section.h.max(p.height).max(0.05);
    let ratio = if section.area > 0.0 { (area_of(p) / section.area - 1.0).abs() } else { 0.0 };
    Some(Diagram2d {
        id: format!("parametric:{}", p.id),
        title: loc(&format!("Parametric section against its section row – {}", p.id), &format!("Parametrischer Querschnitt gegen die Querschnittszeile – {}", p.id)),
        description: loc(&format!("Outline of the section row {} with the area of the parametric {} section: deviation {:.1} % (tolerance {:.1} %).", section.designation, p.shape, ratio * 100.0, p.tolerance * 100.0), &format!("Umriss der Querschnittszeile {} mit der Fläche des parametrischen Querschnitts ({}): Abweichung {:.1} % (Toleranz {:.1} %).", section.designation, p.shape, ratio * 100.0, p.tolerance * 100.0)),
        axes: vec![axis("Width y", "Breite y", QuantityKind::Length, "m", AxisScale::Linear, Some((-0.7 * extent, 0.7 * extent))), axis("Height z", "Höhe z", QuantityKind::Length, "m", AxisScale::Linear, Some((-0.7 * extent, 0.7 * extent)))],
        series: vec![Series::Section { id: "section".into(), label: loc(&section.designation, &section.designation), shape, strain: None, stress: None, neutral_axis: None }],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "deviation".into(), at: Vec2::new(0.0, 0.62 * extent), text: loc(&format!("ΔA = {:.1} %", ratio * 100.0), &format!("ΔA = {:.1} %", ratio * 100.0)), tone: if ratio > p.tolerance { Tone::Error } else { Tone::Neutral } }],
        focus: Some(EntityRef::new(EntityKind::Member, &p.id, format!("parametricSections[id={}]", p.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Parametric area", "Parametrische Fläche", "m²", vec![area_of(p)]), column("Row area", "Querschnittsfläche", "m²", vec![section.area])] },
    })
}

fn area_of(p: &ParametricSection) -> f64 {
    let (h, b, t, tf) = (p.height, p.width, p.web_thickness, p.flange_thickness);
    match p.shape {
        semio_s_artifact_norm_en1993_model::ParametricShape::Rhs => b * h - (b - 2.0 * t).max(0.0) * (h - 2.0 * t).max(0.0),
        semio_s_artifact_norm_en1993_model::ParametricShape::Chs => std::f64::consts::PI * (h * h - (h - 2.0 * t).max(0.0).powi(2)) / 4.0,
        semio_s_artifact_norm_en1993_model::ParametricShape::Angle => (h + b - t).max(0.0) * t,
        semio_s_artifact_norm_en1993_model::ParametricShape::WeldedI => 2.0 * b * tf + (h - 2.0 * tf).max(0.0) * t,
    }
}

fn frame_modes(d: &En1993Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let anchor = d.frame_bars.first()?;
    let eigen = joint_frame::solve(d).ok()?;
    let mode = eigen.sway().or_else(|| eigen.modes.first())?;
    let node = |id: &str| d.frame_nodes.iter().find(|n| n.id == id).map(|n| (n.x, n.z));
    let shifts = |id: &str| mode.shape.iter().find(|s| s.0 == id).map_or((0.0, 0.0), |s| (s.1, s.2));
    let extent = d.frame_nodes.iter().fold(1.0_f64, |m, n| m.max(n.x.abs()).max(n.z.abs()));
    let scale = 0.12 * extent;
    let ratio = 3.0 / mode.alpha_cr;
    let mut series = Vec::new();
    let mut columns = Vec::new();
    for bar in &d.frame_bars {
        let ids = frame_eigen::bar_nodes(bar);
        let base: Vec<Vec2> = [&bar.start_node_id, &bar.end_node_id].iter().filter_map(|id| node(id)).map(|(x, z)| Vec2::new(x, z)).collect();
        let (Some(first), Some(last)) = (base.first().cloned(), base.last().cloned()) else { continue };
        let n = ids.len() - 1;
        let deformed: Vec<Vec2> = ids.iter().enumerate().map(|(k, id)| {
            let t = k as f64 / n as f64;
            let (ux, uz) = shifts(id);
            Vec2::new(first.x + t * (last.x - first.x) + scale * ux, first.y + t * (last.y - first.y) + scale * uz)
        }).collect();
        series.push(Series::Polyline { id: format!("{}:frame", bar.id), label: loc(&format!("Bar {}", bar.id), &format!("Stab {}", bar.id)), points: base, style: stroke(Tone::Neutral, 1.5, Dash::Dashed) });
        columns.push((bar.id.clone(), deformed.clone()));
        series.push(Series::Polyline { id: format!("{}:mode", bar.id), label: loc(&format!("Buckling mode of {}", bar.id), &format!("Knickfigur von {}", bar.id)), points: deformed, style: stroke(tone(ratio), 2.5, Dash::Solid) });
    }
    let all: Vec<&Vec2> = columns.iter().flat_map(|(_, p)| p.iter()).collect();
    let (xs, zs): (Vec<f64>, Vec<f64>) = (all.iter().map(|p| p.x).collect(), all.iter().map(|p| p.y).collect());
    let range = |v: &[f64]| (v.iter().cloned().fold(f64::INFINITY, f64::min) - 0.5, v.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 0.5);
    Some(Diagram2d {
        id: format!("frame-mode:{}", anchor.id),
        title: loc("Buckling mode of the frame", "Knickfigur des Rahmens"),
        description: loc(
            &format!("In-plane buckling mode {} of the frame from the mechanics kernel: α_cr = {:.2}, shown with the nodes magnified by {:.2} m for the largest translation; the dashed lines are the undeformed bars.", if mode.sway { "(sway)" } else { "(non-sway)" }, mode.alpha_cr, scale),
            &format!("Knickfigur {} in der Rahmenebene aus dem Mechanik-Kern: α_cr = {:.2}, Knoten um {:.2} m für die größte Verschiebung überhöht; gestrichelt die unverformten Stäbe.", if mode.sway { "(verschieblich)" } else { "(unverschieblich)" }, mode.alpha_cr, scale),
        ),
        axes: vec![axis("Coordinate x", "Koordinate x", QuantityKind::Length, "m", AxisScale::Linear, Some(range(&xs))), axis("Coordinate z", "Koordinate z", QuantityKind::Length, "m", AxisScale::Linear, Some(range(&zs)))],
        series,
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "alpha".into(), at: Vec2::new(range(&xs).0 + 0.3, range(&zs).1 - 0.3), text: loc(&format!("α_cr = {:.2}", mode.alpha_cr), &format!("α_cr = {:.2}", mode.alpha_cr)), tone: tone(ratio.max(worst_of(report, &anchor.id))) }],
        focus: Some(EntityRef::new(EntityKind::Member, &anchor.id, format!("frameBars[id={}]", anchor.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Coordinate x", "Koordinate x", "m", xs), column("Coordinate z", "Koordinate z", "m", zs)] },
    })
}

/// 🌀 χ_T over λ̄_T on the z-z buckling curve with the torsional operating point and the flexural slenderness (EN 1993-1-1 6.3.1.4).
fn torsional_buckling(d: &En1993Snapshot, report: &CheckReport, member: &SteelMember) -> Option<Diagram2d> {
    let s = torsional_summary(d, &member.id)?;
    let curve = curves::buckling_curve(s.alpha, LAMBDA_MAX, SAMPLES);
    let utilisation = s.n_ed / s.n_b_rd_t.max(1e-9);
    let chi_z = curves::buckling_chi(s.lambda_z, s.alpha);
    Some(Diagram2d {
        id: format!("tf-buckling:{}", member.id),
        title: loc(&format!("Torsional buckling curve – {}", member.id), &format!("Drillknicklinie – {}", member.id)),
        description: loc(
            &format!("χ over λ̄ on the z-z curve (α = {:.2}) with the torsional operating point: N_cr,T = {:.0} kN, λ̄_T = {:.3}, χ_T = {:.3}; the diamond marks the flexural slenderness λ̄_z = {:.3}.", s.alpha, s.n_cr_t / 1e3, s.lambda_t, s.chi_t, s.lambda_z),
            &format!("χ über λ̄ auf der z-z-Linie (α = {:.2}) mit dem Drillknick-Arbeitspunkt: N_cr,T = {:.0} kN, λ̄_T = {:.3}, χ_T = {:.3}; die Raute zeigt die Biegeknickschlankheit λ̄_z = {:.3}.", s.alpha, s.n_cr_t / 1e3, s.lambda_t, s.chi_t, s.lambda_z),
        ),
        axes: vec![axis("Slenderness λ̄", "Schlankheit λ̄", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction χ", "Abminderung χ", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.05)))],
        series: vec![Series::Polyline { id: "chi".into(), label: loc("χ(λ̄)", "χ(λ̄)"), points: curve.iter().copied().map(point).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) }],
        markers: vec![
            Marker { id: "torsional".into(), at: Vec2::new(s.lambda_t.min(LAMBDA_MAX), s.chi_t), shape: MarkerShape::Circle, tone: tone(utilisation.max(worst_of(report, &member.id))), label: loc(&format!("N_Ed/N_b,Rd,T = {utilisation:.2}"), &format!("N_Ed/N_b,Rd,T = {utilisation:.2}")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.6.3.1.tf.{}", member.id)), binding: None },
            Marker { id: "reference:flexural".into(), at: Vec2::new(s.lambda_z.min(LAMBDA_MAX), chi_z), shape: MarkerShape::Diamond, tone: Tone::Neutral, label: loc(&format!("λ̄_z = {:.3}", s.lambda_z), &format!("λ̄_z = {:.3}", s.lambda_z)), utilisation: None, bound_check: None, binding: None },
        ],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Slenderness λ̄", "Schlankheit λ̄", "1", curve.iter().map(|q| q[0]).collect()), column("Reduction χ", "Abminderung χ", "1", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 🔥 M_b,fi,θ,Rd over the steel temperature with the design moment M_fi,Ed and the operating temperature θ_a (EN 1993-1-2 4.2.3.3).
fn fire_ltb_curve(d: &En1993Snapshot, report: &CheckReport, fire: &FireExposure) -> Option<Diagram2d> {
    let s = fire_ltb_summary(d, &fire.id)?;
    let utilisation = s.m_fi_ed / s.m_b_fi_rd.max(1e-9);
    let points: Vec<Vec2> = s.curve.iter().map(|(theta, m)| Vec2::new(*theta, m / 1e3)).collect();
    Some(Diagram2d {
        id: format!("fire-ltb:{}", fire.id),
        title: loc(&format!("Lateral-torsional buckling resistance in fire – {}", fire.id), &format!("Biegedrillknickwiderstand im Brandfall – {}", fire.id)),
        description: loc(
            &format!("M_b,fi,θ,Rd = χ_LT,fi·W_y·k_y,θ·f_y over the steel temperature (λ̄_LT = {:.3}); the dashed line is M_fi,Ed = {:.1} kNm, the marker the operating temperature θ_a = {:.0} °C with {:.1} kNm.", s.lambda_lt, s.m_fi_ed / 1e3, s.theta_a, s.m_b_fi_rd / 1e3),
            &format!("M_b,fi,θ,Rd = χ_LT,fi·W_y·k_y,θ·f_y über der Stahltemperatur (λ̄_LT = {:.3}); gestrichelt M_fi,Ed = {:.1} kNm, die Marke zeigt die Betriebstemperatur θ_a = {:.0} °C mit {:.1} kNm.", s.lambda_lt, s.m_fi_ed / 1e3, s.theta_a, s.m_b_fi_rd / 1e3),
        ),
        axes: vec![axis("Steel temperature θ_a", "Stahltemperatur θ_a", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((20.0, 1000.0))), axis("Resistance M_b,fi,θ,Rd", "Widerstand M_b,fi,θ,Rd", QuantityKind::Moment, "kN·m", AxisScale::Linear, Some((0.0, points.first().map_or(1.0, |p| p.y * 1.05))))],
        series: vec![
            Series::Polyline { id: "m-b-fi".into(), label: loc("M_b,fi,θ,Rd", "M_b,fi,θ,Rd"), points: points.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "m-fi-ed".into(), label: loc("M_fi,Ed", "M_fi,Ed"), axis: AxisRef::Y, value: s.m_fi_ed / 1e3, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "theta-a".into(), at: Vec2::new(s.theta_a.clamp(20.0, 1000.0), s.m_b_fi_rd / 1e3), shape: MarkerShape::Circle, tone: tone(utilisation.max(worst_of(report, &fire.id))), label: loc(&format!("M_fi,Ed/M_b,fi,Rd = {utilisation:.2}"), &format!("M_fi,Ed/M_b,fi,Rd = {utilisation:.2}")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-2.4.2.3.ltb.{}", fire.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &fire.id, format!("fireExposures[id={}]", fire.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Steel temperature θ_a", "Stahltemperatur θ_a", "°C", points.iter().map(|q| q.x).collect()), column("Resistance M_b,fi,θ,Rd", "Widerstand M_b,fi,θ,Rd", "kN·m", points.iter().map(|q| q.y).collect())] },
    })
}

/// 📈️ The diagrams of the entities added in wave 2; `focus` limits them to one entity id.
pub(crate) fn entity_diagrams(d: &En1993Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let wanted = |id: &str, related: &str| focus.is_none_or(|f| f == id || f == related);
    let mut out = Vec::new();
    for joint in d.joints.iter().filter(|j| wanted(&j.id, &j.member_id)) {
        out.extend(joint_moment_rotation(d, report, &joint.id));
        out.extend(joint_components(d, report, &joint.id));
    }
    for restraint in d.restraints.iter().filter(|r| wanted(&r.id, &r.member_id)) {
        out.extend(restraint_curves(d, report, restraint));
    }
    for weld in d.weld_groups.iter().filter(|w| wanted(&w.id, &w.member_id)) {
        out.extend(weld_vectors(d, report, weld));
    }
    for joint in d.hollow_joints.iter().filter(|j| wanted(&j.id, &j.member_id)) {
        out.extend(hollow_resistance(joint));
    }
    for section in d.parametric_sections.iter().filter(|p| wanted(&p.id, &p.section_id)) {
        out.extend(parametric_outline(d, section));
    }
    for member in d.members.iter().filter(|m| wanted(&m.id, &m.id)) {
        out.extend(torsional_buckling(d, report, member));
    }
    for fire in d.fire_exposures.iter().filter(|f| wanted(&f.id, &f.member_id)) {
        out.extend(fire_ltb_curve(d, report, fire));
    }
    if d.frame_bars.first().is_some_and(|b| wanted(&b.id, &b.member_id)) {
        out.extend(frame_modes(d, report));
    }
    out
}
