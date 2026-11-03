//! 📈️ Diagrams: plastic stress blocks, the partial-connection curve M(η), the stud load–slip curve, the deflection line and the shear flow of each beam; the M–N interaction polygon and the buckling curve of each column; the moment-versus-span curve of each slab; the plan of the placements.

use super::checks::{catalogue as cat, part_1_1, part_1_2, part_column, part_en1990, part_slab};
use super::facts::{BeamFacts, ColumnFacts, SlabFacts};
use super::kit::{check_with, worst};
use super::statics;
use super::model::{CompositeColumn, En1994Snapshot};
use super::norm::CheckReport;
use super::scene::diagram_kit::{curves, interaction_polygon, Xy};
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{column, stroke};
use super::scene::projection_kit::diagram_parts;

pub(crate) const SAMPLES: usize = 40;
const STRAIN_POINTS: usize = 120;
const SLIP_MAX_MM: f64 = 8.0;
const SLIP_RATE_PER_MM: f64 = 0.709;
const SLIP_EXPONENT: f64 = 0.4;
const LAMBDA_MAX: f64 = 3.0;

pub(crate) fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(crate) fn p(point: Xy) -> Vec2 {
    Vec2::new(point[0], point[1])
}

pub(crate) fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, domain: Option<(f64, f64)>) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, domain)
}

pub(crate) fn tone_of_ratio(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

pub(crate) fn polyline(id: &str, en: &str, de: &str, points: &[Xy], style: Stroke) -> Series {
    Series::Polyline { id: id.into(), label: loc(en, de), points: points.iter().copied().map(p).collect(), style }
}

pub(crate) fn limit(id: &str, en: &str, de: &str, axis: AxisRef, value: f64, style: Stroke) -> Series {
    Series::Limit { id: id.into(), label: loc(en, de), axis, value, style }
}

fn field(path: String, map: Affine, unit: &str, quantise: Option<Quantise>) -> FieldBinding {
    FieldBinding { path, map, unit: unit.to_string(), quantise }
}

pub(crate) fn beam_entity(id: &str) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Member, id, format!("beams[id={id}]")))
}

/// 🧱️ Stress distribution over the composite depth for plastic bending: `(depth below the slab top, stress)` in m and MPa, compression positive.
pub fn plastic_stress_points(f: &BeamFacts) -> Vec<Xy> {
    let fc = 0.85 * f.beam.concrete_f_ck_pa / f.params.gamma_c / 1e6;
    let fy = f.fy / f.params.gamma_m0 / 1e6;
    let total = f.depth + f.steel.height_m;
    let mut points: Vec<Xy> = vec![[0.0, fc], [f.pna.x_c, fc], [f.pna.x_c, 0.0], [f.depth, 0.0]];
    if f.pna.z_s > 1e-9 {
        points.extend([[f.depth, fy], [f.depth + f.pna.z_s, fy], [f.depth + f.pna.z_s, -fy]]);
    } else {
        points.push([f.depth, -fy]);
    }
    points.push([total, -fy]);
    points
}

fn stress_blocks(f: &BeamFacts, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let steel = super::beams::steel_shape(&f.steel);
    let slab = SectionShape::parametric(ProfileKind::Rect, &[("h", f.hc), ("b", f.b_eff)]);
    let shape = SectionShape::Composite {
        parts: vec![
            CompositePart { shape: Box::new(steel), role: MaterialRole::Steel, offset: Vec2::new(0.0, -f.steel.height_m / 2.0) },
            CompositePart { shape: Box::new(slab), role: MaterialRole::Concrete, offset: Vec2::new(0.0, f.depth - f.hc / 2.0) },
        ],
        rebar: Vec::new(),
    };
    let stress = plastic_stress_points(f);
    let check = worst(report, id, &["en1994.6.2.1.3.mrd."]);
    let in_steel = f.pna.z_s > 1e-9;
    let m_pl_eta = part_1_1::plastic_moment_partial_nm(f.m_pla, f.m_pl_rd, f.eta.min(1.0));
    let fy = f.fy / f.params.gamma_m0 / 1e6;
    Diagram2d {
        id: format!("stress-blocks:{id}"),
        title: loc(&format!("Plastic stress blocks – {id}"), &format!("Plastische Spannungsblöcke – {id}")),
        description: loc(
            &format!("Rectangular stress blocks of the composite section at η = {:.2} (EN 1994-1-1, 6.2.1.2 and 6.2.1.3): concrete 0.85 f_cd over x_c = {:.1} mm, steel ±f_yd, plastic neutral axis {} (N_c = {:.0} kN, N_a = {:.0} kN, residual {:.2} kN).", f.eta, f.pna.x_c * 1e3, if in_steel { format!("{:.1} mm below the steel top", f.pna.z_s * 1e3) } else { "in the slab".to_string() }, f.pna.n_c / 1e3, f.pna.n_a / 1e3, f.pna.residual / 1e3),
            &format!("Rechteckige Spannungsblöcke des Verbundquerschnitts bei η = {:.2} (EN 1994-1-1, 6.2.1.2 und 6.2.1.3): Beton 0,85 f_cd über x_c = {:.1} mm, Stahl ±f_yd, plastische Nulllinie {} (N_c = {:.0} kN, N_a = {:.0} kN, Rest {:.2} kN).", f.eta, f.pna.x_c * 1e3, if in_steel { format!("{:.1} mm unter Stahloberkante", f.pna.z_s * 1e3) } else { "in der Platte".to_string() }, f.pna.n_c / 1e3, f.pna.n_a / 1e3, f.pna.residual / 1e3),
        ),
        axes: vec![axis("Stress σ", "Spannung σ", QuantityKind::Stress, "MPa", Some((-1.3 * fy, 1.3 * fy))), axis("Height z", "Höhe z", QuantityKind::Length, "m", Some((-f.steel.height_m * 1.1, f.depth * 1.1)))],
        series: vec![Series::Section { id: "section".into(), label: loc(&format!("{} with slab b_eff = {:.2} m", f.steel.designation, f.b_eff), &format!("{} mit Platte b_eff = {:.2} m", f.steel.designation, f.b_eff)), shape, strain: None, stress: Some(SectionProfile { points: stress.iter().copied().map(p).collect(), unit: "MPa".into() }), neutral_axis: Some(Line2 { origin: Vec2::new(0.0, f.pna_z()), direction: Vec2::new(1.0, 0.0) }) }],
        markers: vec![Marker {
            id: "design-point".into(),
            at: Vec2::new(0.0, f.pna_z()),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("M_Ed = {:.0} kN·m against M_pl,Rd = {:.0} kN·m at η = {:.2}, plastic neutral axis x_pl = {:.0} mm", f.m_ed / 1e3, m_pl_eta / 1e3, f.eta, (f.depth - f.pna_z()) * 1e3), &format!("M_Ed = {:.0} kN·m gegen M_pl,Rd = {:.0} kN·m bei η = {:.2}, plastische Nulllinie x_pl = {:.0} mm", f.m_ed / 1e3, m_pl_eta / 1e3, f.eta, (f.depth - f.pna_z()) * 1e3)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: vec![DiagramNote {
            id: "pna".into(),
            at: Vec2::new(0.0, f.pna_z()),
            text: loc(&format!("x_pl = {:.0} mm, η = {:.2}", (f.depth - f.pna_z()) * 1e3, f.eta), &format!("x_pl = {:.0} mm, η = {:.2}", (f.depth - f.pna_z()) * 1e3, f.eta)),
            tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)),
        }],
        focus: beam_entity(id),
        legend: false,
        table_twin: DataTable { columns: vec![column("Depth below slab top", "Tiefe unter Plattenoberkante", "m", stress.iter().map(|q| q[0]).collect()), column("Stress (compression positive)", "Spannung (Druck positiv)", "MPa", stress.iter().map(|q| q[1]).collect())] },
    }
}

/// ↔️ The effective width b_eff(x) along the girder (EN 1994-1-1, 5.4.1.2, Figure 5.1): b_eff,0 with β at an end support, b_eff,1 at the middle of every span from its L_e, b_eff,2 over an internal support from L_e = 0.25·(L_left + L_right), joined linearly, against the beam spacing.
fn effective_width(f: &BeamFacts, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let half = f.beam.spacing_m / 2.0;
    let eff = |le: f64, end: bool| cat::effective_width_m(le, 0.0, half, half, end);
    let lengths = &f.layout.lengths;
    let last = lengths.len() - 1;
    let position = |k: usize| match (lengths.len(), k) {
        (1, _) => cat::SpanPosition::Simple,
        (_, k) if f.layout.cantilever && k == last => cat::SpanPosition::Cantilever,
        (_, k) if k == 0 || k == last => cat::SpanPosition::End,
        _ => cat::SpanPosition::Interior,
    };
    let (mut points, mut x, mut mid) = (Vec::<Xy>::new(), 0.0, (0.0, 0.0, 0.0));
    for (k, &length) in lengths.iter().enumerate() {
        let le = cat::le_sagging_m(position(k), length);
        let left = if k == 0 { eff(le, true) } else { eff(cat::le_hogging_m(lengths[k - 1], length), false) };
        let right = if k == last { eff(le, true) } else { eff(cat::le_hogging_m(length, lengths[k + 1]), false) };
        if k == 0 {
            points.push([x, left]);
        }
        if !(f.layout.cantilever && k == last) {
            points.push([x + length / 2.0, eff(le, false)]);
        }
        points.push([x + length, right]);
        if k == f.layout.own {
            mid = (x + length / 2.0, eff(le, false), le);
        }
        x += length;
    }
    let check = worst(report, id, &["en1994.5.4.1.2.beff."]);
    let spacing = f.beam.spacing_m;
    Diagram2d {
        id: format!("effective-width:{id}"),
        title: loc(&format!("Effective width b_eff along the span – {id}"), &format!("Mitwirkende Breite b_eff entlang der Spannweite – {id}")),
        description: loc(
            "Effective width of the concrete flange along the girder (EN 1994-1-1, 5.4.1.2, Figure 5.1): b_ei = min(L_e/8, b_i) at mid-span from the equivalent span L_e of the span, over internal supports from L_e = 0.25 (L_left + L_right), with the factor β at end supports; the dashed line is the beam spacing that b_eff must not exceed. Schematic linear transition between the three characteristic values.",
            "Mitwirkende Breite des Betongurts entlang des Trägers (EN 1994-1-1, 5.4.1.2, Bild 5.1): b_ei = min(L_e/8, b_i) in Feldmitte aus der Ersatzstützweite L_e des Feldes, über Innenstützen aus L_e = 0,25 (L_links + L_rechts), mit dem Faktor β an Endauflagern; die gestrichelte Linie ist der Trägerabstand, den b_eff nicht überschreiten darf. Schematischer linearer Übergang zwischen den drei charakteristischen Werten.",
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", Some((0.0, x.max(f.l_total)))), axis("Effective width b_eff", "Mitwirkende Breite b_eff", QuantityKind::Length, "m", Some((0.0, 1.1 * points.iter().map(|q| q[1]).fold(spacing, f64::max))))],
        series: vec![
            polyline("b-eff", "b_eff(x)", "b_eff(x)", &points, stroke(Tone::Info, 2.0, Dash::Solid)),
            limit("spacing", "Beam spacing", "Trägerabstand", AxisRef::Y, spacing, stroke(Tone::Warning, 1.5, Dash::Dashed)),
        ],
        markers: vec![Marker {
            id: "design-point".into(),
            at: Vec2::new(mid.0, mid.1),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("b_eff = {:.3} m from L_e = {:.2} m against the spacing {:.2} m", mid.1, mid.2, spacing), &format!("b_eff = {:.3} m aus L_e = {:.2} m gegen den Trägerabstand {:.2} m", mid.1, mid.2, spacing)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: beam_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", points.iter().map(|q| q[0]).collect()), column("Effective width b_eff", "Mitwirkende Breite b_eff", "m", points.iter().map(|q| q[1]).collect())] },
    }
}

fn partial_connection(f: &BeamFacts, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let moment = |eta: f64| part_1_1::plastic_moment_partial_nm(f.m_pla, f.m_pl_rd, eta) / 1e3;
    let curve: Vec<Xy> = curves::sample(moment, 0.0, 1.0, SAMPLES);
    let check = worst(report, id, &["en1994.6.2.1.3.mrd."]);
    let eta_now = f.eta.min(1.0);
    let span = (f.m_pl_rd - f.m_pla).max(1e-9);
    let eta_req = ((f.m_ed - f.m_pla) / span).clamp(0.0, 1.0);
    Diagram2d {
        id: format!("partial-connection:{id}"),
        title: loc(&format!("Partial shear connection M_Rd(η) – {id}"), &format!("Teilverdübelung M_Rd(η) – {id}")),
        description: loc(
            "Plastic bending resistance between the steel section alone (η = 0) and full shear connection (η = 1) against the degree of shear connection (EN 1994-1-1, 6.2.1.3); the marker shows the degree of the persisted studs.",
            "Plastische Biegetragfähigkeit zwischen reinem Stahlquerschnitt (η = 0) und voller Verdübelung (η = 1) über dem Verdübelungsgrad (EN 1994-1-1, 6.2.1.3); der Marker zeigt den Grad der gespeicherten Kopfbolzen.",
        ),
        axes: vec![axis("Degree of shear connection η", "Verdübelungsgrad η", QuantityKind::Dimensionless, "1", Some((0.0, 1.0))), axis("Resistance M_Rd", "Widerstand M_Rd", QuantityKind::Moment, "kN·m", None)],
        series: vec![
            polyline("m-rd", "M_Rd(η)", "M_Rd(η)", &curve, stroke(Tone::Info, 2.0, Dash::Solid)),
            limit("m-ed", "Design moment M_Ed", "Bemessungsmoment M_Ed", AxisRef::Y, f.m_ed / 1e3, stroke(Tone::Warning, 1.5, Dash::Dashed)),
            limit("eta-min", "Minimum degree η_min", "Mindestverdübelungsgrad η_min", AxisRef::X, f.eta_min.clamp(0.0, 1.0), stroke(Tone::Error, 1.5, Dash::Dotted)),
        ],
        markers: vec![Marker {
            id: "eta".into(),
            at: Vec2::new(eta_now, moment(eta_now)),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("η = {:.2} ({} of {} studs), M_Rd = {:.0} kN·m", f.eta, f.n_f, f.n_req, moment(eta_now)), &format!("η = {:.2} ({} von {} Dübeln), M_Rd = {:.0} kN·m", f.eta, f.n_f, f.n_req, moment(eta_now))),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: vec![DiagramNote { id: "eta-required".into(), at: Vec2::new(eta_req, f.m_ed / 1e3), text: loc(&format!("η required {:.2}", eta_req), &format!("η erforderlich {:.2}", eta_req)), tone: Tone::Info }],
        focus: beam_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Degree of shear connection η", "Verdübelungsgrad η", "1", curve.iter().map(|q| q[0]).collect()), column("Resistance M_Rd", "Widerstand M_Rd", "kN·m", curve.iter().map(|q| q[1]).collect())] },
    }
}

/// 📈️ Non-linear moment-curvature curve of the composite section (EN 1994-1-1, 6.2.1.4) with the design moment and the ultimate point.
fn moment_curvature(f: &BeamFacts, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let sec = part_1_1::isection(&f.steel);
    let fcc = cat::fcc_beam_pa(f.beam.concrete_f_ck_pa, f.params.gamma_c);
    let input = cat::NonlinearInput::standard(f.fy / f.params.gamma_m0, fcc, 0.0, 0.0);
    let points = cat::moment_curvature(&sec, f.beam.slab_thickness_m, f.b_eff, f.hc, &input, SAMPLES);
    let curve: Vec<Xy> = points.iter().map(|q| [q.kappa * 1e3, q.m_nm / 1e3]).collect();
    let peak = points.iter().max_by(|a, b| a.m_nm.total_cmp(&b.m_nm)).copied().unwrap_or(cat::NonlinearPoint { kappa: 0.0, m_nm: 0.0, eps_top: 0.0, eps_steel_max: 0.0 });
    let check = worst(report, id, &["en1994.6.2.1.4.mkappa."]);
    Diagram2d {
        id: format!("moment-curvature:{id}"),
        title: loc(&format!("Moment-curvature M-κ – {id}"), &format!("Momenten-Krümmungs-Beziehung M-κ – {id}")),
        description: loc(
            "Strain-compatible bending response of the composite section with parabola-rectangle concrete and bilinear steel up to the concrete or steel strain limit (EN 1994-1-1, 6.2.1.4); the horizontal line is the design moment.",
            "Dehnungskompatibles Biegeverhalten des Verbundquerschnitts mit Parabel-Rechteck-Beton und bilinearem Stahl bis zur Beton- oder Stahldehnungsgrenze (EN 1994-1-1, 6.2.1.4); die waagerechte Linie ist das Bemessungsmoment.",
        ),
        axes: vec![axis("Curvature κ", "Krümmung κ", QuantityKind::Dimensionless, "1/km", None), axis("Moment M", "Moment M", QuantityKind::Moment, "kN·m", None)],
        series: vec![
            polyline("m-kappa", "M(κ)", "M(κ)", &curve, stroke(Tone::Info, 2.0, Dash::Solid)),
            limit("m-ed", "Design moment M_Ed", "Bemessungsmoment M_Ed", AxisRef::Y, f.m_ed / 1e3, stroke(Tone::Warning, 1.5, Dash::Dashed)),
            limit("m-pl", "Plastic resistance M_pl,Rd", "Plastische Tragfähigkeit M_pl,Rd", AxisRef::Y, f.m_pl_rd / 1e3, stroke(Tone::Neutral, 1.0, Dash::Dotted)),
        ],
        markers: vec![Marker {
            id: "ultimate".into(),
            at: Vec2::new(peak.kappa * 1e3, peak.m_nm / 1e3),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("M_Rd,nl = {:.0} kN·m at κ = {:.1} 1/km", peak.m_nm / 1e3, peak.kappa * 1e3), &format!("M_Rd,nl = {:.0} kN·m bei κ = {:.1} 1/km", peak.m_nm / 1e3, peak.kappa * 1e3)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: vec![],
        focus: beam_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Curvature κ", "Krümmung κ", "1/km", curve.iter().map(|q| q[0]).collect()), column("Moment M", "Moment M", "kN·m", curve.iter().map(|q| q[1]).collect()), column("Top-fibre strain ε_c", "Randdehnung ε_c", "‰", points.iter().map(|q| q.eps_top * 1e3).collect()), column("Largest steel tension strain", "Größte Stahlzugdehnung", "‰", points.iter().map(|q| q.eps_steel_max * 1e3).collect())] },
    }
}

/// 🔩️ Ollgaard–Slutter–Fisher stud load–slip law `P = P_Rk·(1 − e^{−0.709·δ})^{0.4}` with `δ` in mm.
pub fn stud_load(p_rk: f64, slip_mm: f64) -> f64 {
    p_rk * (1.0 - (-SLIP_RATE_PER_MM * slip_mm).exp()).powf(SLIP_EXPONENT)
}

/// 🔩️ Slip in mm at which the law reaches `load`; `None` at or above the plateau.
pub fn stud_slip_at(p_rk: f64, load: f64) -> Option<f64> {
    let ratio = (load / p_rk).powf(1.0 / SLIP_EXPONENT);
    (ratio < 1.0 && ratio >= 0.0).then(|| -(1.0 - ratio).ln() / SLIP_RATE_PER_MM)
}

fn stud_slip(f: &BeamFacts, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let p_rd = f.p_rd / 1e3;
    let p_rk = p_rd * f.params.gamma_v;
    let curve: Vec<Xy> = curves::sample(|d| stud_load(p_rk, d), 0.0, SLIP_MAX_MM, SAMPLES);
    let check = worst(report, id, &["en1994.6.6.3.1.prd."]);
    let slip = stud_slip_at(p_rk, p_rd).unwrap_or(SLIP_MAX_MM);
    Diagram2d {
        id: format!("stud-slip:{id}"),
        title: loc(&format!("Stud load–slip curve – {id}"), &format!("Dübel-Last-Schlupf-Kurve – {id}")),
        description: loc(
            &format!("Load per headed stud over slip (push-out law, EN 1994-1-1, 6.6.1.1): plateau P_Rk = {p_rk:.0} kN, design resistance P_Rd = {p_rd:.0} kN reached at {slip:.1} mm, characteristic slip capacity δ_uk = 6 mm."),
            &format!("Last je Kopfbolzendübel über dem Schlupf (Ausdrückversuch, EN 1994-1-1, 6.6.1.1): Plateau P_Rk = {p_rk:.0} kN, Bemessungswiderstand P_Rd = {p_rd:.0} kN bei {slip:.1} mm, charakteristische Schlupfkapazität δ_uk = 6 mm."),
        ),
        axes: vec![axis("Slip δ", "Schlupf δ", QuantityKind::Length, "mm", Some((0.0, SLIP_MAX_MM))), axis("Load per stud P", "Last je Dübel P", QuantityKind::Force, "kN", Some((0.0, (p_rk * 1.15).max(1.0))))],
        series: vec![
            polyline("load-slip", "P(δ)", "P(δ)", &curve, stroke(Tone::Info, 2.0, Dash::Solid)),
            limit("p-rd", "Design resistance P_Rd", "Bemessungswiderstand P_Rd", AxisRef::Y, p_rd, stroke(Tone::Warning, 1.5, Dash::Dashed)),
            limit("p-rk", "Characteristic resistance P_Rk", "Charakteristischer Widerstand P_Rk", AxisRef::Y, p_rk, stroke(Tone::Neutral, 1.0, Dash::Dotted)),
            limit("delta-uk", "Slip capacity δ_uk = 6 mm", "Schlupfkapazität δ_uk = 6 mm", AxisRef::X, 6.0, stroke(Tone::Error, 1.0, Dash::Dotted)),
        ],
        markers: vec![Marker {
            id: "p-rd-point".into(),
            at: Vec2::new(slip, p_rd),
            shape: MarkerShape::Diamond,
            tone: check.map_or(Tone::Info, |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("P_Rd = {p_rd:.0} kN at δ = {slip:.1} mm"), &format!("P_Rd = {p_rd:.0} kN bei δ = {slip:.1} mm")),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: beam_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Slip δ", "Schlupf δ", "mm", curve.iter().map(|q| q[0]).collect()), column("Load per stud P", "Last je Dübel P", "kN", curve.iter().map(|q| q[1]).collect())] },
    }
}

/// 🔩️ Interface slip along the studs: the elastic shear flow `v(x) = v₀·|1 − 2u|` shared by the `n_f` studs of a half span gives the load per stud, which the push-out law turns into the slip `δ(x)` (signed by the half span; plateau loads saturate at the 8 mm axis end).
pub fn slip_at(f: &BeamFacts, peak_kn_per_m: f64, p_rk_kn: f64, x: f64) -> f64 {
    let studs_per_m = f64::from(f.n_f.max(1)) / (f.beam.span_m.max(1e-9) / 2.0);
    let load = peak_kn_per_m * f.flow_share(x) / studs_per_m;
    let slip = stud_slip_at(p_rk_kn, load).unwrap_or(SLIP_MAX_MM);
    if f.span_fraction(x) < 0.5 { slip } else { -slip }
}

/// 🔩️ Peak elastic shear flow in kN/m at the support, `4·η_req·N_cf/L`, and the characteristic stud resistance P_Rk in kN: the inputs of [`slip_at`].
pub fn slip_inputs(f: &BeamFacts) -> (f64, f64) {
    let n_cf = (f.beam.steel.a_m2 * f.fy / f.params.gamma_m0).min(0.85 * f.beam.concrete_f_ck_pa / f.params.gamma_c * f.b_eff * f.hc);
    let eta_req = ((f.m_ed - f.m_pla) / (f.m_pl_rd - f.m_pla).max(1e-9)).clamp(0.0, 1.0);
    (4.0 * eta_req * n_cf / f.beam.span_m.max(1e-9) / 1e3, f.p_rd / 1e3 * f.params.gamma_v)
}

/// 🔩️ The slip capacity δ_uk in mm (EN 1994-1-1, 6.6.1.1).
pub const SLIP_CAPACITY_MM: f64 = 6.0;

fn slip_distribution(f: &BeamFacts, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let (peak, p_rk) = slip_inputs(f);
    let line: Vec<Xy> = curves::sample(|x| slip_at(f, peak, p_rk, x), 0.0, f.l_total, 2 * SAMPLES);
    let support = line.first().map_or(0.0, |q| q[1]);
    let check = worst(report, id, &["en1994.6.6.3.1.prd."]);
    Diagram2d {
        id: format!("slip-distribution:{id}"),
        title: loc(&format!("Slip along the studs – {id}"), &format!("Schlupf entlang der Dübel – {id}")),
        description: loc(
            &format!("Interface slip δ(x) between steel section and slab from the elastic shear flow and the push-out law (EN 1994-1-1, 6.6.1.1, 6.6.2); the largest slip {support:.2} mm at the support is checked against the slip capacity δ_uk = 6 mm; the slip is zero at midspan."),
            &format!("Fugenschlupf δ(x) zwischen Stahlprofil und Platte aus dem elastischen Schubfluss und dem Ausdrückversuch (EN 1994-1-1, 6.6.1.1, 6.6.2); der größte Schlupf {support:.2} mm am Auflager wird gegen die Schlupfkapazität δ_uk = 6 mm geprüft; in Feldmitte ist der Schlupf null."),
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", Some((0.0, f.l_total))), axis("Slip δ", "Schlupf δ", QuantityKind::Length, "mm", Some((-SLIP_MAX_MM, SLIP_MAX_MM)))],
        series: vec![
            polyline("slip", "δ(x)", "δ(x)", &line, stroke(Tone::Info, 2.0, Dash::Solid)),
            limit("delta-uk", "Slip capacity δ_uk = 6 mm", "Schlupfkapazität δ_uk = 6 mm", AxisRef::Y, 6.0, stroke(Tone::Error, 1.0, Dash::Dotted)),
            limit("delta-uk-negative", "Slip capacity −δ_uk", "Schlupfkapazität −δ_uk", AxisRef::Y, -6.0, stroke(Tone::Error, 1.0, Dash::Dotted)),
        ],
        markers: vec![Marker {
            id: "support-slip".into(),
            at: Vec2::new(0.0, support),
            shape: MarkerShape::Triangle,
            tone: check.map_or(tone_of_ratio(support.abs() / 6.0), |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("δ = {support:.2} mm at the support"), &format!("δ = {support:.2} mm am Auflager")),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: beam_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", line.iter().map(|q| q[0]).collect()), column("Slip δ", "Schlupf δ", "mm", line.iter().map(|q| q[1]).collect())] },
    }
}

/// 📐️ Deflection of a uniformly loaded simply supported span from the mechanics kernel, relative to its midspan value at relative position `u`.
pub fn deflection_shape(u: f64) -> f64 {
    let layout = cat::continuous::Layout::of(cat::SpanPosition::Simple, 10.0, 0.0);
    statics::uniform_load_line(&layout, cat::continuous::Stiffness::uniform(1.0)).map_or(0.0, |l| statics::interpolate(&l.deflection, u))
}

fn deflection(f: &BeamFacts, report: &CheckReport) -> Option<Diagram2d> {
    let id = f.beam.id.as_str();
    let check = check_with(report, &format!("en1994.7.3.1.deflection.{id}"))?;
    let delta = check.computed.value * 1e3;
    if !delta.is_finite() || delta <= 0.0 {
        return None;
    }
    let limit_mm = part_1_1::deflection_limit_m(f.beam.span_m, f.annex) * 1e3;
    let shape = statics::uniform_load_line(&f.layout, f.stiffness)?;
    let origin = f.layout.origin(f.layout.own);
    let (at_x, at_map) = if f.layout.cantilever { (f.l_total, Affine { base: -origin, k: 1.0 }) } else { (origin + f.beam.span_m / 2.0, Affine { base: -2.0 * origin, k: 2.0 }) };
    let line: Vec<Xy> = curves::sample(|x| delta * statics::interpolate(&shape.deflection, x / f.l_total), 0.0, f.l_total, 2 * SAMPLES);
    let binding = field(format!("beams[id={id}].spanM"), at_map, "m", Some(Quantise::Round { step: 0.05 }));
    Some(Diagram2d {
        id: format!("deflection:{id}"),
        title: loc(&format!("Deflection line – {id}"), &format!("Biegelinie – {id}")),
        description: loc(
            &format!("Frequent-combination deflection δ = {delta:.1} mm against the limit {limit_mm:.1} mm (EN 1994-1-1, 7.3.1); drag the marker to change the span."),
            &format!("Durchbiegung der häufigen Kombination δ = {delta:.1} mm gegen die Grenze {limit_mm:.1} mm (EN 1994-1-1, 7.3.1); den Marker ziehen, um die Spannweite zu ändern."),
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", Some((0.0, f.l_total))), axis("Deflection w", "Durchbiegung w", QuantityKind::Length, "mm", None)],
        series: vec![polyline("w", "w(x)", "w(x)", &line, stroke(Tone::Info, 2.0, Dash::Solid)), limit("w-limit", "Limit", "Grenzwert", AxisRef::Y, limit_mm, stroke(Tone::Warning, 1.5, Dash::Dashed))],
        markers: vec![Marker {
            id: "midspan".into(),
            at: Vec2::new(at_x, delta),
            shape: MarkerShape::Circle,
            tone: tone_of_ratio(check.utilization),
            label: loc(&format!("δ = {delta:.1} mm at x = {at_x:.2} m"), &format!("δ = {delta:.1} mm bei x = {at_x:.2} m")),
            utilisation: Some(check.utilization),
            bound_check: Some(check.id.clone()),
            binding: Some(binding),
        }],
        annotations: Vec::new(),
        focus: beam_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", line.iter().map(|q| q[0]).collect()), column("Deflection w", "Durchbiegung w", "mm", line.iter().map(|q| q[1]).collect())] },
    })
}

/// 📍️ Shape of |V(x)| of the beam (simple, end, interior or cantilever span) under its composite-stage actions (permanent γ_G, variable γ_Q of the annex registry, every span loaded, neighbours carrying the load at the same relative position) from the shared mechanics statics: area loads as uniform line loads, each point force at its position `x_m`; normalised to the largest shear force, `None` when the beam carries no load.
pub(crate) fn shear_shape(f: &BeamFacts) -> Option<Box<dyn Fn(f64) -> f64>> {
    let l = f.beam.span_m.max(1e-9);
    let factor = |kind: semio_s_artifact_norm_en1994_model::ActionKind| if kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent { part_en1990::gamma_g(f.annex, f.reliability_class) } else { part_en1990::gamma_q(f.annex, f.reliability_class) };
    let acting = || f.beam.actions.iter().filter(|a| a.stage == semio_s_artifact_norm_en1994_model::ActionStage::Composite && a.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue);
    let loads: Vec<Vec<cat::continuous::Load>> = f
        .layout
        .lengths
        .iter()
        .map(|_| {
            acting()
                .filter_map(|a| {
                    if a.q_area_pa.abs() >= 1e-9 {
                        Some(cat::continuous::Load::Udl(factor(a.kind) * a.q_area_pa * f.beam.spacing_m))
                    } else {
                        (a.f_k_n.abs() > 1.0).then(|| cat::continuous::Load::Point(factor(a.kind) * a.f_k_n, part_en1990::point_fraction(a, l)))
                    }
                })
                .collect()
        })
        .collect();
    let diagram = cat::continuous::shear_diagram(cat::span_position(f.beam.support), f.beam.span_m, f.beam.adjacent_span_m, &loads, cat::continuous::Stiffness::uniform(1.0));
    let peak = diagram.iter().fold(0.0_f64, |m, p| m.max(p.1.abs()));
    (peak > 1e-9).then(|| Box::new(move |x: f64| cat::continuous::shear_at(&diagram, x).abs() / peak) as Box<dyn Fn(f64) -> f64>)
}

fn shear_flow(f: &BeamFacts, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let n_cf = (f.beam.steel.a_m2 * f.fy / f.params.gamma_m0).min(0.85 * f.beam.concrete_f_ck_pa / f.params.gamma_c * f.b_eff * f.hc);
    let span_m = f.beam.span_m.max(1e-9);
    let eta_req = ((f.m_ed - f.m_pla) / (f.m_pl_rd - f.m_pla).max(1e-9)).clamp(0.0, 1.0);
    let demand_total = eta_req * n_cf;
    let peak = 4.0 * demand_total / span_m / 1e3;
    let resistance = f.studs.capacity_n() / (span_m / 2.0) / 1e3;
    let shape = shear_shape(f);
    let flow: Vec<Xy> = curves::sample(|x| peak * shape.as_ref().map_or_else(|| (1.0 - 2.0 * f.span_fraction(x)).abs(), |v| v(x)), 0.0, f.l_total, 2 * SAMPLES);
    let staircase: Vec<Xy> = f.studs.cells().iter().flat_map(|cell| [[cell.a_m, cell.resistance_n_per_m() / 1e3], [cell.b_m, cell.resistance_n_per_m() / 1e3]]).collect();
    let check = worst(report, id, &["en1994.6.6."]);
    let ratio = if resistance > 0.0 { (peak / resistance).min(1.0e6) } else { 0.0 };
    Diagram2d {
        id: format!("shear-flow:{id}"),
        title: loc(&format!("Longitudinal shear flow – {id}"), &format!("Längsschubfluss – {id}")),
        description: loc(
            &format!("Elastic shear flow v(x) ∝ |V(x)| for the force {:.0} kN to be transferred (η required {eta_req:.2}) against the uniform stud resistance {resistance:.0} kN/m (EN 1994-1-1, 6.6.2 and 6.6.1.2); the stud zones of the 3D view are coloured by this flow.", demand_total / 1e3),
            &format!("Elastischer Schubfluss v(x) ∝ |V(x)| für die zu übertragende Kraft {:.0} kN (η erforderlich {eta_req:.2}) gegen den gleichmäßigen Dübelwiderstand {resistance:.0} kN/m (EN 1994-1-1, 6.6.2 und 6.6.1.2); die Dübelzonen der 3D-Ansicht sind nach diesem Fluss gefärbt.", demand_total / 1e3),
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", Some((0.0, f.l_total))), axis("Shear flow v", "Schubfluss v", QuantityKind::ForcePerLength, "kN/m", Some((0.0, (peak.max(resistance).max(staircase.iter().map(|q| q[1]).fold(0.0, f64::max)) * 1.15).max(1.0))))],
        series: vec![
            polyline("v-elastic", "Elastic shear flow", "Elastischer Schubfluss", &flow, stroke(Tone::Info, 2.0, Dash::Solid)),
            limit("v-plastic", "Uniform plastic demand", "Gleichmäßiger plastischer Bedarf", AxisRef::Y, demand_total / (span_m / 2.0) / 1e3, stroke(Tone::Neutral, 1.0, Dash::Dotted)),
            limit("v-rd", "Stud resistance n·P_Rd per length", "Dübelwiderstand n·P_Rd je Länge", AxisRef::Y, resistance, stroke(Tone::Warning, 1.5, Dash::Dashed)),
            polyline("v-rd-staircase", "Resistance staircase ΣP_Rd per station cell", "Widerstandstreppe ΣP_Rd je Stationszelle", &staircase, stroke(Tone::Info, 1.5, Dash::Dashed)),
        ],
        markers: vec![Marker {
            id: "support-peak".into(),
            at: Vec2::new(0.0, peak),
            shape: MarkerShape::Triangle,
            tone: tone_of_ratio(check.map_or(ratio, |c| c.utilization)),
            label: loc(&format!("v = {peak:.0} kN/m at the support, {:.0} % of the resistance", ratio * 100.0), &format!("v = {peak:.0} kN/m am Auflager, {:.0} % des Widerstands", ratio * 100.0)),
            utilisation: Some(check.map_or(ratio, |c| c.utilization)),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: beam_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", flow.iter().map(|q| q[0]).collect()), column("Shear flow v", "Schubfluss v", "kN/m", flow.iter().map(|q| q[1]).collect())] },
    }
}

fn buckling_alpha(curve: &str) -> f64 {
    match curve {
        "a" => 0.21,
        "b" => 0.34,
        "c" => 0.49,
        _ => 0.76,
    }
}

fn mn_polygon(f: &ColumnFacts) -> Vec<Xy> {
    std::iter::once([0.0, 0.0]).chain(f.polygon.iter().map(|q| [q.n_n / 1e3, q.m_nm / 1e3])).collect()
}

fn column_entity(id: &str) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Member, id, format!("columns[id={id}]")))
}

fn interaction(f: &ColumnFacts, report: &CheckReport) -> Diagram2d {
    let id = f.column.id.as_str();
    let ring = mn_polygon(f);
    let reduced: Vec<Xy> = ring.iter().map(|q| [q[0], q[1] * f.alpha_m]).collect();
    let check = worst(report, id, &["en1994.6.7.3.mn."]);
    let demand = [f.n_ed.max(0.0) / 1e3, f.m_ed.abs() / 1e3];
    let hull = interaction_polygon::interaction_polygon(&ring);
    let mut series = vec![
        Series::Polygon { id: "capacity".into(), label: loc("Interaction polygon A–B–C–D", "Interaktionspolygon A–B–C–D"), ring: ring.iter().copied().map(p).collect(), fill: Fill { tone: Tone::Info, opacity: 0.15 }, hatch: None },
        Series::Polygon { id: "capacity-alpha".into(), label: loc("Reduced by α_M", "Abgemindert um α_M"), ring: reduced.iter().copied().map(p).collect(), fill: Fill { tone: Tone::Neutral, opacity: 0.1 }, hatch: Some(Hatch::Diagonal) },
    ];
    let strain: Vec<(f64, f64)> = part_column::strain_curve(&f.column, f.annex, STRAIN_POINTS).into_iter().filter(|q| q.n_n >= 0.0).map(|q| (q.n_n, q.m_nm.abs())).collect();
    let strain_line: Vec<Xy> = strain.iter().map(|q| [q.0 / 1e3, q.1 / 1e3]).collect();
    series.push(polyline("strain-compatibility", "Strain-compatibility curve (reference, not governing)", "Dehnungskompatibilitätskurve (Referenz, nicht maßgebend)", &strain_line, stroke(Tone::Neutral, 1.5, Dash::Dashed)));
    let mut notes = Vec::new();
    let polygon_m = part_column::mu_d(&f.column, f.annex, demand[0] * 1e3) * f.polygon[3].m_nm;
    let strain_m = cat::strain_curve::moment_at(&strain, demand[0] * 1e3);
    if polygon_m > 1.0 && strain_m > 0.0 {
        let margin = 100.0 * (1.0 - strain_m / polygon_m);
        notes.push(DiagramNote { id: "strain-margin".into(), at: p([demand[0], strain_m / 1e3]), text: loc(&format!("Strain-compatible M at N_Ed: {margin:.1} % below the polygon (reference)"), &format!("Dehnungskompatibles M bei N_Ed: {margin:.1} % unter dem Polygon (Referenz)")), tone: Tone::Neutral });
    }
    if let Some(hit) = interaction_polygon::utilisation_ray(&hull, [0.0, 0.0], demand) {
        series.push(Series::Ray { id: "ray".into(), label: loc("Utilisation ray", "Ausnutzungsstrahl"), origin: Vec2::ZERO, through: p(demand), to_boundary: true, style: stroke(Tone::Warning, 1.5, Dash::Dotted) });
        notes.push(DiagramNote { id: "ray-ratio".into(), at: p(hit.boundary), text: loc(&format!("|OA|/|OB| = {:.2}", hit.utilisation), &format!("|OA|/|OB| = {:.2}", hit.utilisation)), tone: tone_of_ratio(hit.utilisation) });
    }
    let utilisation = check.map(|c| c.utilization);
    Diagram2d {
        id: format!("mn:{id}"),
        title: loc(&format!("M–N interaction – {id}"), &format!("M–N-Interaktion – {id}")),
        description: loc(
            &format!("Interaction polygon of the composite cross-section (EN 1994-1-1, 6.7.3.2) with the design point N_Ed = {:.0} kN, M_Ed = {:.1} kN·m; the hatched polygon is reduced by α_M = {:.2}. The dashed strain-compatibility curve is a reference only; the polygon governs the verdict.", demand[0], demand[1], f.alpha_m),
            &format!("Interaktionspolygon des Verbundquerschnitts (EN 1994-1-1, 6.7.3.2) mit dem Bemessungspunkt N_Ed = {:.0} kN, M_Ed = {:.1} kN·m; das schraffierte Polygon ist um α_M = {:.2} abgemindert. Die gestrichelte Dehnungskompatibilitätskurve ist nur eine Referenz; das Polygon ist maßgebend.", demand[0], demand[1], f.alpha_m),
        ),
        axes: vec![axis("Axial force N", "Normalkraft N", QuantityKind::Force, "kN", Some((0.0, (ring.iter().map(|q| q[0]).fold(0.0, f64::max).max(demand[0]) * 1.1).max(1.0)))), axis("Bending moment M", "Biegemoment M", QuantityKind::Moment, "kN·m", Some((0.0, (ring.iter().map(|q| q[1]).fold(0.0, f64::max).max(demand[1]) * 1.1).max(1.0))))],
        series,
        markers: vec![Marker {
            id: "demand".into(),
            at: p(demand),
            shape: MarkerShape::Diamond,
            tone: utilisation.map_or(Tone::Neutral, tone_of_ratio),
            label: loc(&format!("N_Ed = {:.0} kN, M_Ed = {:.1} kN·m", demand[0], demand[1]), &format!("N_Ed = {:.0} kN, M_Ed = {:.1} kN·m", demand[0], demand[1])),
            utilisation,
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: notes,
        focus: column_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![
            column("Axial force N", "Normalkraft N", "kN", ring.iter().map(|q| q[0]).collect()),
            column("Bending moment M", "Biegemoment M", "kN·m", ring.iter().map(|q| q[1]).collect()),
            column("Strain-compatible M at the same N (reference)", "Dehnungskompatibles M bei gleichem N (Referenz)", "kN·m", ring.iter().map(|q| cat::strain_curve::moment_at(&strain, q[0] * 1e3) / 1e3).collect()),
        ] },
    }
}

fn column_buckling(f: &ColumnFacts, report: &CheckReport) -> Diagram2d {
    let c = &f.column;
    let id = c.id.as_str();
    let curve_names = [("a", 0.21), ("b", 0.34), ("c", 0.49), ("d", 0.76)];
    let mut series: Vec<Series> = curve_names
        .iter()
        .map(|(name, alpha)| {
            let selected = *name == c.buckling_curve;
            polyline(&format!("chi-{name}"), &format!("Curve {name}"), &format!("Linie {name}"), &curves::buckling_curve(*alpha, LAMBDA_MAX, 60), stroke(if selected { Tone::Info } else { Tone::Neutral }, if selected { 2.5 } else { 1.0 }, Dash::Solid))
        })
        .collect();
    series.push(polyline("euler", "Euler hyperbola 1/λ̄²", "Euler-Hyperbel 1/λ̄²", &curves::sample(|l| 1.0 / (l * l), 1.0, LAMBDA_MAX, 60), stroke(Tone::Neutral, 1.0, Dash::Dashed)));
    let lambda = f.lambda.min(LAMBDA_MAX);
    let check = worst(report, id, &["en1994.6.7.3.npl."]);
    let binding = field(format!("columns[id={id}].lengthM"), Affine { base: 0.0, k: c.length_m / f.lambda.max(1e-9) }, "m", Some(Quantise::Round { step: 0.05 }));
    let alpha = buckling_alpha(c.buckling_curve.as_str());
    let lambdas: Vec<f64> = curves::buckling_curve(alpha, LAMBDA_MAX, 60).iter().map(|q| q[0]).collect();
    Diagram2d {
        id: format!("buckling:{id}"),
        title: loc(&format!("Buckling curve – {id}"), &format!("Knicklinie – {id}")),
        description: loc(
            &format!("Reduction factor χ over the relative slenderness λ̄ (EN 1994-1-1, 6.7.3.5) for curve {}; the marker is this column (λ̄ = {:.2}, χ = {:.3}); drag it to change the length.", c.buckling_curve, f.lambda, f.chi),
            &format!("Abminderungsfaktor χ über dem bezogenen Schlankheitsgrad λ̄ (EN 1994-1-1, 6.7.3.5) für Linie {}; der Marker ist diese Stütze (λ̄ = {:.2}, χ = {:.3}); ziehen, um die Länge zu ändern.", c.buckling_curve, f.lambda, f.chi),
        ),
        axes: vec![axis("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄", QuantityKind::Dimensionless, "1", Some((0.0, LAMBDA_MAX))), axis("Reduction factor χ", "Abminderungsfaktor χ", QuantityKind::Dimensionless, "1", Some((0.0, 1.1)))],
        series,
        markers: vec![Marker {
            id: "column".into(),
            at: Vec2::new(lambda, f.chi),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |x| tone_of_ratio(x.utilization)),
            label: loc(&format!("λ̄ = {:.2}, χ = {:.3}", f.lambda, f.chi), &format!("λ̄ = {:.2}, χ = {:.3}", f.lambda, f.chi)),
            utilisation: check.map(|x| x.utilization),
            bound_check: check.map(|x| x.id.clone()),
            binding: Some(binding),
        }],
        annotations: Vec::new(),
        focus: column_entity(id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄", "1", lambdas.clone()), column(&format!("χ curve {}", c.buckling_curve), &format!("χ Linie {}", c.buckling_curve), "1", curves::buckling_curve(alpha, LAMBDA_MAX, 60).iter().map(|q| q[1]).collect())] },
    }
}

/// 🧱️ Stress distribution of the hogging section over the composite depth: `(depth below the slab top, stress)` in m and MPa, compression positive, with the bars in tension at their depth and the steel around the plastic neutral axis `y_na`.
pub fn hogging_stress_points(f: &BeamFacts, y_na: f64) -> Vec<Xy> {
    let fy = f.fy / f.params.gamma_m0 / 1e6;
    let fs = cat::constants::REBAR_FSK / f.params.gamma_s / 1e6;
    let y_r = cat::constants::HOGGING_REBAR_DEPTH_M;
    vec![[0.0, 0.0], [y_r - 0.003, 0.0], [y_r, -fs], [y_r + 0.003, 0.0], [f.depth, 0.0], [f.depth, -fy], [y_na, -fy], [y_na, fy], [f.depth + f.steel.height_m, fy]]
}

fn hogging_stress_blocks(f: &BeamFacts, report: &CheckReport) -> Option<Diagram2d> {
    if !f.hogging {
        return None;
    }
    let id = f.beam.id.as_str();
    let sec = part_1_1::isection(&f.steel);
    let as_total = f.beam.as_hogging_m2_per_m * f.b_eff;
    let (m_pl, y_na) = cat::plastic_hogging(&sec, f.fy / f.params.gamma_m0, f.depth, as_total, cat::constants::REBAR_FSK / f.params.gamma_s, cat::constants::HOGGING_REBAR_DEPTH_M);
    let stress = hogging_stress_points(f, y_na);
    let steel = super::beams::steel_shape(&f.steel);
    let shape = SectionShape::Composite { parts: vec![CompositePart { shape: Box::new(steel), role: MaterialRole::Steel, offset: Vec2::new(0.0, -f.steel.height_m / 2.0) }], rebar: Vec::new() };
    let check = worst(report, id, &["en1994.6.2.1.2.hogging."]);
    let fy = f.fy / f.params.gamma_m0 / 1e6;
    let z_na = f.depth - y_na;
    Some(Diagram2d {
        id: format!("stress-blocks-hogging:{id}"),
        title: loc(&format!("Hogging stress blocks – {id}"), &format!("Spannungsblöcke im Stützbereich – {id}")),
        description: loc(
            &format!("Plastic stress blocks of the cracked section over the support (EN 1994-1-1, 6.2.1.2): concrete neglected, bars A_s = {:.0} mm² in tension, steel ±f_yd about the neutral axis {:.0} mm below the slab top, M_pl,Rd = {:.1} kNm.", as_total * 1e6, y_na * 1e3, m_pl / 1e3),
            &format!("Plastische Spannungsblöcke des gerissenen Querschnitts über der Stütze (EN 1994-1-1, 6.2.1.2): Beton vernachlässigt, Bewehrung A_s = {:.0} mm² auf Zug, Stahl ±f_yd um die Nulllinie {:.0} mm unter Plattenoberkante, M_pl,Rd = {:.1} kNm.", as_total * 1e6, y_na * 1e3, m_pl / 1e3),
        ),
        axes: vec![axis("Stress σ", "Spannung σ", QuantityKind::Stress, "MPa", Some((-1.3 * fy, 1.3 * fy))), axis("Height z", "Höhe z", QuantityKind::Length, "m", Some((-f.steel.height_m * 1.1, f.depth * 1.1)))],
        series: vec![Series::Section { id: "section".into(), label: loc(&format!("{} over the support", f.steel.designation), &format!("{} über der Stütze", f.steel.designation)), shape, strain: None, stress: Some(SectionProfile { points: stress.iter().copied().map(p).collect(), unit: "MPa".into() }), neutral_axis: Some(Line2 { origin: Vec2::new(0.0, z_na), direction: Vec2::new(1.0, 0.0) }) }],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "pna".into(), at: Vec2::new(0.0, z_na), text: loc(&format!("x = {:.0} mm", y_na * 1e3), &format!("x = {:.0} mm", y_na * 1e3)), tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)) }],
        focus: beam_entity(id),
        legend: false,
        table_twin: DataTable { columns: vec![column("Depth below slab top", "Tiefe unter Plattenoberkante", "m", stress.iter().map(|q| q[0]).collect()), column("Stress (compression positive)", "Spannung (Druck positiv)", "MPa", stress.iter().map(|q| q[1]).collect())] },
    })
}

fn fire_temperature(c: &CompositeColumn, document: &En1994Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let id = c.id.as_str();
    let g = part_column::true_geometry(c)?;
    let end = part_1_2::rating_minutes(document.fire_rating.rated()?);
    let times: Vec<f64> = [30.0, 60.0, 90.0, 120.0].into_iter().filter(|t| *t <= end + 1e-9).collect();
    let fields = cat::column_fire::temperature_fields(&g, &times.iter().map(|t| t * 60.0).collect::<Vec<_>>());
    let last = fields.last()?;
    let dashes = [Dash::Solid, Dash::Dashed, Dash::Dotted, Dash::Solid];
    let tones = [Tone::Info, Tone::Info, Tone::Warning, Tone::Error];
    let lines: Vec<Vec<Xy>> = fields.iter().map(|field| (0..field.nx).map(|i| [(i as f64 + 0.5) * field.dx, field.at(i, 0)]).collect()).collect();
    let check = worst(report, id, &["en1994.1-2.4.3.5.column."]);
    let mean = last.steel_mean();
    Some(Diagram2d {
        id: format!("fire-temperature:{id}"),
        title: loc(&format!("Fire temperature across the section – {id}"), &format!("Brandtemperatur über den Querschnitt – {id}")),
        description: loc(
            &format!("Temperature of the mid-height line from the centre to the surface after {} min of the ISO 834 fire (two-dimensional heat conduction, EN 1992-1-2 concrete and EN 1993-1-2 steel properties; values listed as unverified in the NDP registry); the marker is the mean steel temperature {mean:.0} °C at the rating.", times.iter().map(|t| format!("{t:.0}")).collect::<Vec<_>>().join(", ")),
            &format!("Temperatur der Linie auf halber Höhe vom Zentrum zur Oberfläche nach {} min Normbrand ISO 834 (zweidimensionale Wärmeleitung, Betoneigenschaften nach EN 1992-1-2 und Stahleigenschaften nach EN 1993-1-2; im NDP-Register als ungeprüft geführt); der Marker ist die mittlere Stahltemperatur {mean:.0} °C bei der Feuerwiderstandsdauer.", times.iter().map(|t| format!("{t:.0}")).collect::<Vec<_>>().join(", ")),
        ),
        axes: vec![axis("Distance from the centre", "Abstand vom Zentrum", QuantityKind::Length, "m", Some((0.0, last.nx as f64 * last.dx))), axis("Temperature θ", "Temperatur θ", QuantityKind::Temperature, "°C", None)],
        series: lines.iter().zip(&times).enumerate().map(|(k, (line, t))| polyline(&format!("t{t:.0}"), &format!("R{t:.0}"), &format!("R{t:.0}"), line, stroke(tones[k.min(3)], 2.0, dashes[k.min(3)]))).collect(),
        markers: vec![Marker {
            id: "steel-mean".into(),
            at: Vec2::new(g.tw / 2.0, mean),
            shape: MarkerShape::Diamond,
            tone: check.map_or(Tone::Neutral, |x| tone_of_ratio(x.utilization)),
            label: loc(&format!("Steel mean {mean:.0} °C"), &format!("Stahl im Mittel {mean:.0} °C")),
            utilisation: check.map(|x| x.utilization),
            bound_check: check.map(|x| x.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: column_entity(id),
        legend: true,
        table_twin: DataTable { columns: std::iter::once(column("Distance from the centre", "Abstand vom Zentrum", "m", lines[0].iter().map(|q| q[0]).collect())).chain(lines.iter().zip(&times).map(|(line, t)| column(&format!("θ after {t:.0} min"), &format!("θ nach {t:.0} min"), "°C", line.iter().map(|q| q[1]).collect()))).collect() },
    })
}

fn slab_moment(f: &SlabFacts, report: &CheckReport) -> Diagram2d {
    let s = &f.slab;
    let id = s.id.as_str();
    let m_rd = part_slab::bending_resistance_nm_per_m(s, f.annex) / 1e3;
    let demand = |span: f64| part_en1990::uls_composite(&s.statics_actions(), span, span, s.support, 1.0, f.annex, f.reliability_class).m_nm / 1e3;
    let upper = (s.span_m * 2.0).max(4.0);
    let curve: Vec<Xy> = curves::sample(demand, 0.5, upper, SAMPLES);
    let check = worst(report, id, &["en1994.9.7.2."]);
    let now = demand(s.span_m);
    let binding = field(format!("slabs[id={id}].spanM"), Affine::IDENTITY, "m", Some(Quantise::Round { step: 0.05 }));
    Diagram2d {
        id: format!("slab-span:{id}"),
        title: loc(&format!("Slab moment over span – {id}"), &format!("Deckenmoment über der Spannweite – {id}")),
        description: loc(
            "Design moment per metre width over the span against the bending resistance of the composite slab (EN 1994-1-1, 9.7.2); drag the marker to change the span.",
            "Bemessungsmoment je Meter Breite über der Spannweite gegen die Biegetragfähigkeit der Verbunddecke (EN 1994-1-1, 9.7.2); den Marker ziehen, um die Spannweite zu ändern.",
        ),
        axes: vec![axis("Span L", "Spannweite L", QuantityKind::Length, "m", Some((0.5, upper))), axis("Moment m", "Moment m", QuantityKind::Moment, "kN·m", None)],
        series: vec![polyline("m-ed", "m_Ed(L)", "m_Ed(L)", &curve, stroke(Tone::Info, 2.0, Dash::Solid)), limit("m-rd", "Resistance m_Rd", "Widerstand m_Rd", AxisRef::Y, m_rd, stroke(Tone::Warning, 1.5, Dash::Dashed))],
        markers: vec![Marker {
            id: "span".into(),
            at: Vec2::new(s.span_m, now),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("L = {:.2} m, m_Ed = {now:.1} kN·m/m", s.span_m), &format!("L = {:.2} m, m_Ed = {now:.1} kN·m/m", s.span_m)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: Some(binding),
        }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, id, format!("slabs[id={id}]"))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Span L", "Spannweite L", "m", curve.iter().map(|q| q[0]).collect()), column("Design moment m_Ed", "Bemessungsmoment m_Ed", "kN·m", curve.iter().map(|q| q[1]).collect())] },
    }
}

/// 📍️ The plan of the placed entities: one diamond per placement at its (x, y) that drags the x leaf, the yaw in the label, with a table twin of id, x, y, z and yaw.
fn placements(document: &En1994Snapshot) -> Option<Diagram2d> {
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
            binding: Some(field(format!("placements[id={}].x", p.id), Affine::IDENTITY, "m", Some(Quantise::Round { step: 0.01 }))),
        })
        .collect();
    let notes = document.placements.iter().map(|p| DiagramNote { id: format!("name-{}", p.id), at: Vec2::new(p.x, p.y + 0.3), text: loc(&p.id, &p.id), tone: Tone::Neutral }).collect();
    let column_of = |value: &dyn Fn(&super::model::Placement) -> f64| document.placements.iter().map(value).collect::<Vec<f64>>();
    Some(Diagram2d {
        id: "placements:document".into(),
        title: loc("Placement plan", "Platzierungsplan"),
        description: loc("The persisted position of every placed entity in plan; a diamond drags the x position.", "Die gespeicherte Lage jeder platzierten Entität im Grundriss; eine Raute verschiebt die x-Lage."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", None), axis("Position y", "Lage y", QuantityKind::Length, "m", None)],
        series: Vec::new(),
        markers,
        annotations: notes,
        focus: None,
        legend: false,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", column_of(&|p| p.x)), column("Position y", "Lage y", "m", column_of(&|p| p.y)), column("Position z", "Position z", "m", column_of(&|p| p.z)), column("Yaw", "Drehung", "rad", column_of(&|p| p.yaw))] },
    })
}

/// 📊️ The utilisation of every named result of a design case as one marker per result over the limit 1, with the table twin; a marker is bound to its check.
fn case_results(report: &CheckReport, c: &super::model::CaseRow) -> Option<Diagram2d> {
    let rows: Vec<(&str, f64, &LocalizedCopy)> = report.checks.iter().filter(|k| k.subject.entity_id == c.id && k.utilization.is_finite() && k.utilization > 0.0).map(|k| (k.id.as_str(), k.utilization, &k.title)).collect();
    if rows.is_empty() {
        return None;
    }
    let markers = rows
        .iter()
        .enumerate()
        .map(|(i, (id, ratio, title))| Marker {
            id: format!("marker-result-{i}"),
            at: Vec2::new(i as f64 + 1.0, *ratio),
            shape: MarkerShape::Diamond,
            tone: tone_of_ratio(*ratio),
            label: loc(&format!("{}: {:.0} %", title.en, ratio * 100.0), &format!("{}: {:.0} %", title.de, ratio * 100.0)),
            utilisation: Some(*ratio),
            bound_check: Some((*id).to_string()),
            binding: None,
        })
        .collect();
    let top = rows.iter().map(|r| r.1).fold(1.2_f64, f64::max);
    Some(Diagram2d {
        id: format!("case-results:{}", c.id),
        title: loc(&format!("Utilisation of {} ({})", c.label_en, c.entry_id), &format!("Ausnutzung von {} ({})", c.label_de, c.entry_id)),
        description: loc("Demand over resistance of every named result of the catalogue verification; a marker above 1 fails.", "Beanspruchung zu Beanspruchbarkeit jedes benannten Ergebnisses des Katalognachweises; ein Marker über 1 versagt."),
        axes: vec![axis("Result", "Ergebnis", QuantityKind::Dimensionless, "1", Some((0.4, rows.len() as f64 + 0.6))), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", Some((0.0, top)))],
        series: vec![limit("limit", "Limit 1", "Grenze 1", AxisRef::Y, 1.0, stroke(Tone::Warning, 1.5, Dash::Dashed))],
        markers,
        annotations: Vec::new(),
        focus: Some(super::kit::case_ref(&c.id)),
        legend: false,
        table_twin: DataTable { columns: vec![column("Result", "Ergebnis", "1", (1..=rows.len()).map(|i| i as f64).collect()), column("Utilisation", "Ausnutzung", "1", rows.iter().map(|r| r.1).collect())] },
    })
}

/// 📈️ Every diagram of the document; with `focus` set to an entity id only the diagrams of that entity.
pub fn diagrams(document: &En1994Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1994, document.annex))
}

fn diagrams_unbased(document: &En1994Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let wanted = |id: &str| focus.is_none_or(|f| super::kit::base_id(f) == id);
    let mut out = Vec::new();
    for beam in document.beams.iter().filter(|b| wanted(&b.id)) {
        let facts = BeamFacts::of(document, beam);
        if facts.l_total <= 1e-6 || facts.steel.height_m <= 1e-6 {
            continue;
        }
        out.push(stress_blocks(&facts, report));
        out.push(effective_width(&facts, report));
        out.push(partial_connection(&facts, report));
        out.push(moment_curvature(&facts, report));
        out.push(stud_slip(&facts, report));
        out.push(slip_distribution(&facts, report));
        out.extend(deflection(&facts, report));
        out.extend(hogging_stress_blocks(&facts, report));
        out.push(shear_flow(&facts, report));
        out.extend(super::fire_section::diagrams(&facts, document, report));
        out.extend(super::fatigue::diagrams(&facts, document, report));
    }
    for column in document.columns.iter().filter(|c| wanted(&c.id)) {
        let facts = ColumnFacts::of(document, column);
        out.push(interaction(&facts, report));
        out.push(column_buckling(&facts, report));
        out.extend(fire_temperature(column, document, report));
    }
    for slab in document.slabs.iter().filter(|s| wanted(&s.id)) {
        out.push(slab_moment(&SlabFacts::of(document, slab), report));
    }
    for case in document.cases.iter().filter(|c| wanted(&c.id)) {
        out.extend(case_results(report, case));
    }
    if focus.is_none() {
        out.extend(placements(document));
    }
    out
}
