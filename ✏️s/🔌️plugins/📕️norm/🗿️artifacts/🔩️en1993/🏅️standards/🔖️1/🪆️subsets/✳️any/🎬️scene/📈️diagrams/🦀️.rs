//! 📈️ Diagrams: buckling curves χ(λ̄) with the member markers, LTB χ_LT and M_b,Rd over the restraint spacing, the M–N interaction polygon with its utilisation ray,
//! the cross-section classification stress block, the S–N fatigue curve with the spectrum, and the fire temperature and k_θ curves.

use super::checks::{part_1_1, part_1_2, part_1_9, AnnexParams};
use super::model::{En1993Snapshot, FatigueDetail, FireExposure, SteelMaterial, SteelMember, SteelSection};
use super::norm::{CheckReport, CheckResult};
use super::scene::diagram_kit::{curves, interaction_polygon, sn_curves, Xy};
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{column, stroke, axis};

#[path = "../📈️entity-diagrams/🦀️.rs"]
mod entity_diagrams;

#[path = "../📈️annex-diagrams/🦀️.rs"]
mod annex_diagrams;

#[path = "../📈️design-diagrams/🦀️.rs"]
mod design_diagrams;

#[path = "../📈️norm-diagrams/🦀️.rs"]
mod norm_diagrams;

const LAMBDA_MAX: f64 = 3.0;
const SAMPLES: usize = 60;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn p(point: Xy) -> Vec2 {
    Vec2::new(point[0], point[1])
}

fn utilisation_tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn worst<'a>(report: &'a CheckReport, entity: &str, prefix: &str) -> Option<&'a CheckResult> {
    report.checks.iter().filter(|c| c.subject.entity_id == entity && c.id.starts_with(prefix) && c.utilization.is_finite()).max_by(|a, b| a.utilization.total_cmp(&b.utilization))
}

fn material_of<'a>(d: &'a En1993Snapshot, member: &SteelMember) -> Option<(&'a SteelSection, &'a SteelMaterial)> {
    Some((d.sections.iter().find(|s| s.id == member.section_id)?, d.materials.iter().find(|m| m.id == member.material_id)?))
}

fn curve_name(curve: part_1_1::BucklingCurve) -> &'static str {
    match curve {
        part_1_1::BucklingCurve::A0 => "a₀",
        part_1_1::BucklingCurve::A => "a",
        part_1_1::BucklingCurve::B => "b",
        part_1_1::BucklingCurve::C => "c",
        part_1_1::BucklingCurve::D => "d",
    }
}

const CURVES: [part_1_1::BucklingCurve; 5] = [part_1_1::BucklingCurve::A0, part_1_1::BucklingCurve::A, part_1_1::BucklingCurve::B, part_1_1::BucklingCurve::C, part_1_1::BucklingCurve::D];

fn curve_series(prefix: &str, highlight: part_1_1::BucklingCurve) -> (Vec<Series>, Vec<Vec<f64>>) {
    let mut series = Vec::new();
    let mut columns = Vec::new();
    for curve in CURVES {
        let points = curves::buckling_curve(curve.alpha(), LAMBDA_MAX, SAMPLES);
        columns.push(points.iter().map(|q| q[1]).collect());
        let tone = if curve == highlight { Tone::Info } else { Tone::Neutral };
        series.push(Series::Polyline { id: format!("{prefix}-{}", curve_name(curve)), label: loc(&format!("Curve {}", curve_name(curve)), &format!("Linie {}", curve_name(curve))), points: points.into_iter().map(p).collect(), style: stroke(tone, if curve == highlight { 2.5 } else { 1.0 }, Dash::Solid) });
    }
    (series, columns)
}

fn lambda_column() -> Vec<f64> {
    curves::buckling_curve(0.21, LAMBDA_MAX, SAMPLES).into_iter().map(|q| q[0]).collect()
}

fn compression_ed(report: &CheckReport, member: &str) -> Option<f64> {
    report.checks.iter().filter(|c| c.subject.entity_id == member && c.id.starts_with("en1993.6.2.4.n.")).map(|c| c.computed.value).max_by(f64::total_cmp)
}

fn buckling(d: &En1993Snapshot, report: &CheckReport, member: &SteelMember) -> Option<Diagram2d> {
    let (section, material) = material_of(d, member)?;
    let params = AnnexParams::for_choice(d.annex);
    let n_ed = compression_ed(report, &member.id);
    let nb = worst(report, &member.id, "en1993.6.3.1.nb.");
    let lambda_1 = std::f64::consts::PI * (material.e_modulus / material.fy).sqrt();
    let axes = [('y', member.buckling_length_y, section.iy, "bucklingLengthY", "y–y"), ('z', member.buckling_length_z, section.iz, "bucklingLengthZ", "z–z")];
    let mut markers = Vec::new();
    let mut highlight = part_1_1::buckling_curve_rolled_i(section, 'z', material.fy);
    for (axis_name, length, inertia, field, label) in axes {
        let radius = part_1_1::radius_of_gyration(inertia, section.area);
        let lambda = part_1_1::lambda_bar(length, radius, material.e_modulus, material.fy);
        let curve = part_1_1::buckling_curve_rolled_i(section, axis_name, material.fy);
        if axis_name == 'z' {
            highlight = curve;
        }
        let chi = part_1_1::chi(lambda, curve);
        let utilisation = n_ed.map(|n| n * params.gamma_m1 / (chi * section.area * material.fy));
        let mut binding = FieldBinding::new(format!("members[id={}].{field}", member.id), Affine { base: 0.0, k: lambda_1 * radius }, "m");
        binding.quantise = Some(Quantise::Round { step: 0.05 });
        markers.push(Marker {
            id: format!("marker-{axis_name}"),
            at: Vec2::new(lambda.min(LAMBDA_MAX), chi),
            shape: MarkerShape::Circle,
            tone: utilisation.map_or(Tone::Neutral, utilisation_tone),
            label: loc(&format!("{label}: λ̄ = {lambda:.2}, χ = {chi:.3}"), &format!("{label}: λ̄ = {lambda:.2}, χ = {chi:.3}")),
            utilisation,
            bound_check: nb.map(|c| c.id.clone()),
            binding: Some(binding),
        });
    }
    let (mut series, columns) = curve_series("chi", highlight);
    series.push(Series::Polyline {
        id: "euler".into(),
        label: loc("Euler hyperbola 1/λ̄²", "Euler-Hyperbel 1/λ̄²"),
        points: curves::sample(|l| 1.0 / (l * l), 1.0, LAMBDA_MAX, SAMPLES).into_iter().map(p).collect(),
        style: stroke(Tone::Neutral, 1.0, Dash::Dashed),
    });
    let mut table = vec![column("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄", "1", lambda_column())];
    table.extend(CURVES.iter().zip(columns).map(|(c, v)| column(&format!("χ curve {}", curve_name(*c)), &format!("χ Linie {}", curve_name(*c)), "1", v)));
    Some(Diagram2d {
        id: format!("buckling:{}", member.id),
        title: loc(&format!("Flexural buckling curves – {}", member.label), &format!("Knicklinien – {}", member.label)),
        description: loc("Reduction factor χ over relative slenderness λ̄ for the buckling curves a₀ to d (EN 1993-1-1, 6.3.1.2); markers show this member about both axes.", "Abminderungsfaktor χ über dem bezogenen Schlankheitsgrad λ̄ für die Knicklinien a₀ bis d (EN 1993-1-1, 6.3.1.2); die Marker zeigen dieses Bauteil um beide Achsen."),
        axes: vec![axis("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction factor χ", "Abminderungsfaktor χ", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable { columns: table },
    })
}

struct LtbBasis {
    w_y: f64,
    m_cr_at: Box<dyn Fn(f64) -> f64>,
    curve: part_1_1::BucklingCurve,
}

fn ltb_basis(member: &SteelMember, section: &SteelSection, material: &SteelMaterial) -> LtbBasis {
    let class = part_1_1::section_class_rolled_i(section, material.fy);
    let w_y = if class <= 2 { section.w_pl_y } else { section.w_el_y };
    let (section, material, psi, load, diagram) = (section.clone(), material.clone(), member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
    let curve = part_1_1::ltb_curve_table_6_4(&section);
    LtbBasis { w_y, m_cr_at: Box::new(move |l| part_1_1::m_cr_rolled_nm(&section, &material, l, psi, load, diagram)), curve }
}

fn ltb(d: &En1993Snapshot, report: &CheckReport, member: &SteelMember) -> Option<Vec<Diagram2d>> {
    let (section, material) = material_of(d, member)?;
    let params = AnnexParams::for_choice(d.annex);
    let basis = ltb_basis(member, section, material);
    let l_lt = member.ltb_length.max(member.ltb_restraint_spacing).max(0.1);
    let lambda_lt = part_1_1::lambda_lt(basis.w_y, material.fy, (basis.m_cr_at)(l_lt));
    let chi_lt = part_1_1::chi_lt(lambda_lt, basis.curve);
    let check = worst(report, &member.id, "en1993.6.3.2.mb.");
    let utilisation = check.map(|c| c.utilization);
    let focus = Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id)));
    let (series, columns) = curve_series("chi-lt", basis.curve);
    let chi_diagram = Diagram2d {
        id: format!("ltb:{}", member.id),
        title: loc(&format!("Lateral-torsional buckling curves – {}", member.label), &format!("Biegedrillknicklinien – {}", member.label)),
        description: loc("Reduction factor χ_LT over λ̄_LT for the buckling curves (EN 1993-1-1, 6.3.2.2); the marker is this member.", "Abminderungsfaktor χ_LT über λ̄_LT für die Knicklinien (EN 1993-1-1, 6.3.2.2); der Marker ist dieses Bauteil."),
        axes: vec![axis("Relative slenderness λ̄_LT", "Bezogener Schlankheitsgrad λ̄_LT", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction factor χ_LT", "Abminderungsfaktor χ_LT", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series,
        markers: vec![Marker { id: "marker-ltb".into(), at: Vec2::new(lambda_lt.min(LAMBDA_MAX), chi_lt), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, utilisation_tone), label: loc(&format!("λ̄_LT = {lambda_lt:.2}, χ_LT = {chi_lt:.3}"), &format!("λ̄_LT = {lambda_lt:.2}, χ_LT = {chi_lt:.3}")), utilisation, bound_check: check.map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: true,
        table_twin: DataTable { columns: std::iter::once(column("Relative slenderness λ̄_LT", "Bezogener Schlankheitsgrad λ̄_LT", "1", lambda_column())).chain(CURVES.iter().zip(columns).map(|(c, v)| column(&format!("χ_LT curve {}", curve_name(*c)), &format!("χ_LT Linie {}", curve_name(*c)), "1", v))).collect() },
    };
    let upper = member.length.max(l_lt * 1.5).max(1.0);
    let resistance = |l: f64| {
        let lam = part_1_1::lambda_lt(basis.w_y, material.fy, (basis.m_cr_at)(l));
        part_1_1::ltb_resistance_nm(basis.w_y, material.fy, part_1_1::chi_lt(lam, basis.curve), params) / 1.0e3
    };
    let points = curves::sample(&resistance, 0.25, upper, SAMPLES);
    let demand = report.checks.iter().filter(|c| c.subject.entity_id == member.id && c.id.starts_with("en1993.6.3.2.mb.")).map(|c| c.computed.value / 1.0e3).fold(0.0_f64, f64::max);
    let m_now = resistance(l_lt);
    let mut binding = FieldBinding::new(format!("members[id={}].ltbRestraintSpacing", member.id), Affine::IDENTITY, "m");
    binding.quantise = Some(Quantise::Round { step: 0.05 });
    let spacing_diagram = Diagram2d {
        id: format!("ltb-spacing:{}", member.id),
        title: loc(&format!("Buckling resistance over restraint spacing – {}", member.label), &format!("Biegedrillknickwiderstand über Halterungsabstand – {}", member.label)),
        description: loc("M_b,Rd over the lateral restraint spacing with the design moment as limit; drag the marker to change the spacing.", "M_b,Rd über dem Abstand der seitlichen Halterungen mit dem Bemessungsmoment als Grenze; den Marker ziehen, um den Abstand zu ändern."),
        axes: vec![axis("Restraint spacing", "Halterungsabstand", QuantityKind::Length, "m", AxisScale::Linear, None), axis("Resistance M_b,Rd", "Widerstand M_b,Rd", QuantityKind::Moment, "kN·m", AxisScale::Linear, None)],
        series: [Series::Polyline { id: "mb-rd".into(), label: loc("M_b,Rd", "M_b,Rd"), points: points.iter().copied().map(p).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }].into_iter().chain((demand > 0.0).then(|| Series::Limit { id: "m-ed".into(), label: loc("Design moment M_y,Ed", "Bemessungsmoment M_y,Ed"), axis: AxisRef::Y, value: demand, style: stroke(Tone::Warning, 1.5, Dash::Dashed) })).collect(),
        markers: vec![Marker { id: "marker-spacing".into(), at: Vec2::new(l_lt, m_now), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, utilisation_tone), label: loc(&format!("L = {l_lt:.2} m, M_b,Rd = {m_now:.1} kN·m"), &format!("L = {l_lt:.2} m, M_b,Rd = {m_now:.1} kN·m")), utilisation, bound_check: check.map(|c| c.id.clone()), binding: Some(binding) }],
        annotations: Vec::new(),
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Restraint spacing", "Halterungsabstand", "m", points.iter().map(|q| q[0]).collect()), column("M_b,Rd", "M_b,Rd", "kN·m", points.iter().map(|q| q[1]).collect())] },
    };
    Some(vec![chi_diagram, spacing_diagram])
}

fn combination_of(id: &str, prefix: &str) -> Option<String> {
    id.strip_prefix(prefix).map(str::to_string)
}

fn interaction(d: &En1993Snapshot, report: &CheckReport, member: &SteelMember) -> Option<Diagram2d> {
    let (section, material) = material_of(d, member)?;
    let params = AnnexParams::for_choice(d.annex);
    let class = part_1_1::section_class_rolled_i(section, material.fy);
    let n_pl = part_1_1::axial_resistance_n(section.area, material.fy, params);
    let w = if class <= 2 { section.w_pl_y } else { section.w_el_y };
    let m_pl = part_1_1::bending_resistance_nm(w, material.fy, class, params);
    let a_web = ((section.area - 2.0 * section.b * section.tf) / section.area).clamp(0.0, 0.5);
    let m_n = |n: f64| if class >= 3 { m_pl * (1.0 - n.abs()) } else { (m_pl * (1.0 - n.abs()) / (1.0 - 0.5 * a_web)).min(m_pl) };
    let ns: Vec<f64> = (0..=40).map(|i| -1.0 + 2.0 * i as f64 / 40.0).collect();
    let capacity: Vec<Xy> = ns.iter().flat_map(|n| [[n * n_pl / 1e3, m_n(*n) / 1e3], [n * n_pl / 1e3, -m_n(*n) / 1e3]]).collect();
    let ring = interaction_polygon::interaction_polygon(&capacity);
    let mn = worst(report, &member.id, "en1993.6.2.9.mn.");
    let mut series = vec![Series::Polygon { id: "capacity".into(), label: loc("M–N capacity", "M–N-Tragfähigkeit"), ring: ring.iter().copied().map(p).collect(), fill: Fill { tone: Tone::Info, opacity: 0.15 }, hatch: None }];
    let mut markers = Vec::new();
    let mut notes = Vec::new();
    if let Some(check) = mn {
        let combo = combination_of(&check.id, "en1993.6.2.9.mn.")?;
        let n_ed = report.checks.iter().find(|c| c.subject.entity_id == member.id && c.id == format!("en1993.6.2.4.n.{combo}")).map_or_else(|| report.checks.iter().find(|c| c.subject.entity_id == member.id && c.id == format!("en1993.6.2.3.nt.{combo}")).map_or(0.0, |c| -c.computed.value), |c| c.computed.value) / 1e3;
        let m_ed = report.checks.iter().find(|c| c.subject.entity_id == member.id && c.id == format!("en1993.6.2.5.my.{combo}")).map_or(0.0, |c| c.computed.value) / 1e3;
        let demand = [n_ed, m_ed];
        if let Some(hit) = interaction_polygon::utilisation_ray(&ring, [0.0, 0.0], demand) {
            series.push(Series::Ray { id: "ray".into(), label: loc("Utilisation ray", "Ausnutzungsstrahl"), origin: Vec2::ZERO, through: Vec2::new(demand[0], demand[1]), to_boundary: true, style: stroke(Tone::Warning, 1.5, Dash::Dotted) });
            notes.push(DiagramNote { id: "ray-ratio".into(), at: Vec2::new(hit.boundary[0], hit.boundary[1]), text: loc(&format!("|OA|/|OB| = {:.2}", hit.utilisation), &format!("|OA|/|OB| = {:.2}", hit.utilisation)), tone: utilisation_tone(hit.utilisation) });
        }
        markers.push(Marker { id: "demand".into(), at: Vec2::new(demand[0], demand[1]), shape: MarkerShape::Diamond, tone: utilisation_tone(check.utilization), label: loc(&format!("N_Ed = {n_ed:.0} kN, M_y,Ed = {m_ed:.1} kN·m"), &format!("N_Ed = {n_ed:.0} kN, M_y,Ed = {m_ed:.1} kN·m")), utilisation: Some(check.utilization), bound_check: Some(check.id.clone()), binding: None });
    }
    Some(Diagram2d {
        id: format!("mn:{}", member.id),
        title: loc(&format!("M–N interaction – {}", member.label), &format!("M–N-Interaktion – {}", member.label)),
        description: loc("Plastic bending resistance M_N,y,Rd over the axial force N (EN 1993-1-1, 6.2.9) with the governing design effect and the utilisation ray.", "Plastische Biegetragfähigkeit M_N,y,Rd über der Normalkraft N (EN 1993-1-1, 6.2.9) mit der maßgebenden Beanspruchung und dem Ausnutzungsstrahl."),
        axes: vec![axis("Axial force N", "Normalkraft N", QuantityKind::Force, "kN", AxisScale::Linear, None), axis("Bending moment M_y", "Biegemoment M_y", QuantityKind::Moment, "kN·m", AxisScale::Linear, None)],
        series,
        markers,
        annotations: notes,
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Axial force N", "Normalkraft N", "kN", ring.iter().map(|q| q[0]).collect()), column("Bending moment M_y", "Biegemoment M_y", "kN·m", ring.iter().map(|q| q[1]).collect())] },
    })
}

fn classification(d: &En1993Snapshot, member: &SteelMember) -> Option<Diagram2d> {
    let (section, material) = material_of(d, member)?;
    let fy = material.fy / 1.0e6;
    let class = part_1_1::section_class_rolled_i(section, material.fy);
    let half = section.h / 2.0;
    let stress = if class <= 2 { vec![Vec2::new(fy, half), Vec2::new(fy, 0.0), Vec2::new(-fy, 0.0), Vec2::new(-fy, -half)] } else { vec![Vec2::new(fy, half), Vec2::new(-fy, -half)] };
    let eps = part_1_1::epsilon(material.fy);
    let c_flange = (section.b - section.tw - 2.0 * section.r) / 2.0;
    let c_web = section.h - 2.0 * section.tf - 2.0 * section.r;
    Some(Diagram2d {
        id: format!("class:{}", member.id),
        title: loc(&format!("Cross-section classification – {}", member.label), &format!("Querschnittsklassifizierung – {}", member.label)),
        description: loc(&format!("Class {class} section: {} stress distribution over the height under bending (EN 1993-1-1, 5.5).", if class <= 2 { "plastic" } else { "elastic" }), &format!("Querschnittsklasse {class}: {} Spannungsverteilung über die Höhe bei Biegung (EN 1993-1-1, 5.5).", if class <= 2 { "plastische" } else { "elastische" })),
        axes: vec![axis("Stress σ", "Spannung σ", QuantityKind::Stress, "MPa", AxisScale::Linear, Some((-1.3 * fy, 1.3 * fy))), axis("Height z", "Höhe z", QuantityKind::Length, "m", AxisScale::Linear, Some((-0.7 * section.h, 0.7 * section.h)))],
        series: vec![Series::Section { id: "section".into(), label: loc(&section.designation, &section.designation), shape: super::kit::section_shape(section), strain: None, stress: Some(SectionProfile { points: stress, unit: "MPa".into() }), neutral_axis: Some(Line2 { origin: Vec2::ZERO, direction: Vec2::new(1.0, 0.0) }) }],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "class".into(), at: Vec2::new(0.0, 0.62 * section.h), text: loc(&format!("Class {class}, ε = {eps:.2}"), &format!("Klasse {class}, ε = {eps:.2}")), tone: Tone::Neutral}],
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: false,
        table_twin: DataTable { columns: vec![
            column("Part (1 flange, 2 web)", "Teil (1 Flansch, 2 Steg)", "1", vec![1.0, 2.0]),
            column("Slenderness c/t", "Schlankheit c/t", "1", vec![c_flange / section.tf, c_web / section.tw]),
            column("Class", "Klasse", "1", vec![f64::from(part_1_1::flange_class(c_flange, section.tf, material.fy)), f64::from(part_1_1::web_class(c_web, section.tw, material.fy))]),
        ] },
    })
}

fn fatigue(report: &CheckReport, detail: &FatigueDetail) -> Option<Diagram2d> {
    let category = part_1_9::detail_category_pa(detail.category) / 1.0e6;
    let curve = sn_curves::sn_curve(category, 1.0e4, 1.0e8);
    let spectrum: Vec<(f64, f64)> = super::fatigue_views::blocks(detail).into_iter().map(|(range, cycles)| (range * 1.0e6, cycles)).collect();
    let gamma = part_1_9::gamma_mf(part_1_9::parse_method(detail.method.as_str()));
    let damage = part_1_9::miner_damage(&spectrum, category * 1.0e6, gamma);
    let check = worst(report, &detail.id, "en1993.1-9.fat.");
    let points: Vec<ScatterPoint> = spectrum.iter().map(|(ds, n)| ScatterPoint { at: Vec2::new(*n, ds / 1.0e6), tone: utilisation_tone(damage), label: Some(loc(&format!("Δσ = {:.0} MPa, n = {n:.2e}", ds / 1.0e6), &format!("Δσ = {:.0} MPa, n = {n:.2e}", ds / 1.0e6))) }).collect();
    Some(Diagram2d {
        id: format!("sn:{}", detail.id),
        title: loc(&format!("S–N curve, detail category {} – {}", detail.category, detail.id), &format!("Wöhlerlinie, Kerbfall {} – {}", detail.category, detail.id)),
        description: loc(&format!("Fatigue strength Δσ_R over cycles N in log–log scale (EN 1993-1-9) with the stress-range spectrum; Palmgren–Miner damage D = {damage:.2}."), &format!("Ermüdungsfestigkeit Δσ_R über Lastspielzahl N doppeltlogarithmisch (EN 1993-1-9) mit dem Spannungsschwingbreitenkollektiv; Palmgren-Miner-Schädigung D = {damage:.2}.")),
        axes: vec![axis("Cycles N", "Lastspiele N", QuantityKind::Dimensionless, "1", AxisScale::Log10, Some((1.0e4, 1.0e8))), axis("Stress range Δσ", "Spannungsschwingbreite Δσ", QuantityKind::Stress, "MPa", AxisScale::Log10, Some((10.0, 1000.0)))],
        series: vec![
            Series::Polyline { id: "wohler".into(), label: loc(&format!("Δσ_C = {category:.0} MPa"), &format!("Δσ_C = {category:.0} MPa")), points: curve.iter().copied().map(p).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Limit { id: "delta-d".into(), label: loc("Fatigue limit Δσ_D", "Dauerfestigkeit Δσ_D"), axis: AxisRef::Y, value: sn_curves::fatigue_limit(category), style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
            Series::Limit { id: "delta-l".into(), label: loc("Cut-off limit Δσ_L", "Schwellenwert Δσ_L"), axis: AxisRef::Y, value: sn_curves::cutoff_limit(category), style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
            Series::Scatter { id: "spectrum".into(), label: loc("Stress-range spectrum", "Spannungsschwingbreitenkollektiv"), points },
        ],
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "damage".into(), at: Vec2::new(2.0e4, 700.0), text: loc(&format!("D = {damage:.2}"), &format!("D = {damage:.2}")), tone: check.map_or(utilisation_tone(damage), |c| utilisation_tone(c.utilization)) }],
        focus: Some(EntityRef::new(EntityKind::Group, &detail.id, format!("fatigueDetails[id={}]", detail.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Cycles N", "Lastspiele N", "1", curve.iter().map(|q| q[0]).collect()), column("Strength Δσ_R", "Festigkeit Δσ_R", "MPa", curve.iter().map(|q| q[1]).collect())] },
    })
}

const UTILISATION_CAP: f64 = 2.0;

fn fire(d: &En1993Snapshot, report: &CheckReport, exposure: &FireExposure) -> Option<Vec<Diagram2d>> {
    let member = d.members.iter().find(|m| m.id == exposure.member_id)?;
    let rating = part_1_2::parse_rating(exposure.rating.as_str());
    let minutes = part_1_2::rating_minutes(rating);
    let end = minutes * 1.25;
    let gas = curves::fire_curve(curves::FireCurve::Iso834, end, SAMPLES);
    let history = if exposure.protection_thickness > 1e-6 {
        super::checks::clauses::fatigue_fire::protected_steel_temperature(&super::checks::clauses::fatigue_fire::gas_iso834, exposure.section_factor.max(1.0), exposure.protection_thickness, exposure.protection_conductivity, exposure.protection_density, exposure.protection_specific_heat, end * 60.0, 10.0)
    } else {
        super::checks::clauses::fatigue_fire::unprotected_steel_temperature(&super::checks::clauses::fatigue_fire::gas_iso834, 1.0, exposure.section_factor.max(1.0), end * 60.0, 10.0, 0.7, 1.0)
    };
    let steel: Vec<Xy> = history.iter().step_by(6).map(|(t, theta)| [t / 60.0, *theta]).collect();
    let theta_cr = part_1_2::critical_temperature_c(exposure.mu0);
    let theta_end = history.iter().rev().find(|(t, _)| *t <= minutes * 60.0 + 1e-9).map_or(20.0, |(_, th)| *th);
    let check = worst(report, &exposure.id, "en1993.1-2.theta.");
    let rating_tone = utilisation_tone(exposure.mu0 / part_1_2::k_y_theta(theta_end).max(1e-3));
    let focus = Some(EntityRef::new(EntityKind::Load, &exposure.id, format!("fireExposures[id={}]", exposure.id)));
    let temperature = Diagram2d {
        id: format!("fire:{}", exposure.id),
        title: loc(&format!("Fire temperature curves – {}", member.label), &format!("Brandtemperaturverläufe – {}", member.label)),
        description: loc("ISO 834 gas temperature and steel temperature over time against the critical temperature (EN 1993-1-2, 4.2.4 and 4.2.5).", "ISO-834-Gastemperatur und Stahltemperatur über der Zeit gegenüber der kritischen Temperatur (EN 1993-1-2, 4.2.4 und 4.2.5)."),
        axes: vec![axis("Time t", "Zeit t", QuantityKind::Time, "min", AxisScale::Linear, Some((0.0, end))), axis("Temperature θ", "Temperatur θ", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((0.0, 1100.0)))],
        series: vec![
            Series::Polyline { id: "gas".into(), label: loc("ISO 834 gas", "ISO-834-Gas"), points: gas.iter().copied().map(p).collect(), style: stroke(Tone::Warning, 1.5, Dash::Solid) },
            Series::Polyline { id: "steel".into(), label: loc("Steel θ_a", "Stahl θ_a"), points: steel.iter().copied().map(p).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "theta-cr".into(), label: loc("Critical θ_a,cr", "Kritisch θ_a,cr"), axis: AxisRef::Y, value: theta_cr, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
            Series::Limit { id: "rating".into(), label: loc(&format!("Rating R{minutes:.0}"), &format!("Feuerwiderstand R{minutes:.0}")), axis: AxisRef::X, value: minutes, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
        ],
        markers: vec![Marker { id: "steel-at-rating".into(), at: Vec2::new(minutes, theta_end), shape: MarkerShape::Circle, tone: rating_tone, label: loc(&format!("θ_a = {theta_end:.0} °C at R{minutes:.0}"), &format!("θ_a = {theta_end:.0} °C bei R{minutes:.0}")), utilisation: Some(theta_end / theta_cr.max(1.0)), bound_check: check.map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: true,
        table_twin: DataTable { columns: vec![column("Time t", "Zeit t", "min", steel.iter().map(|q| q[0]).collect()), column("Steel θ_a", "Stahl θ_a", "°C", steel.iter().map(|q| q[1]).collect())] },
    };
    let thetas: Vec<f64> = (0..=SAMPLES).map(|i| 20.0 + 1180.0 * i as f64 / SAMPLES as f64).collect();
    let reduction = Diagram2d {
        id: format!("fire-k:{}", exposure.id),
        title: loc(&format!("Strength and stiffness reduction – {}", member.label), &format!("Festigkeits- und Steifigkeitsabminderung – {}", member.label)),
        description: loc("Reduction factors k_y,θ and k_E,θ of carbon steel over the steel temperature (EN 1993-1-2, Table 3.1) with the critical temperature.", "Abminderungsfaktoren k_y,θ und k_E,θ von Baustahl über der Stahltemperatur (EN 1993-1-2, Tabelle 3.1) mit der kritischen Temperatur."),
        axes: vec![axis("Steel temperature θ_a", "Stahltemperatur θ_a", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((20.0, 1200.0))), axis("Reduction factor k", "Abminderungsfaktor k", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.05)))],
        series: vec![
            Series::Polyline { id: "ky".into(), label: loc("k_y,θ", "k_y,θ"), points: thetas.iter().map(|t| Vec2::new(*t, part_1_2::k_y_theta(*t))).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "ke".into(), label: loc("k_E,θ", "k_E,θ"), points: thetas.iter().map(|t| Vec2::new(*t, part_1_2::k_e_theta(*t))).collect(), style: stroke(Tone::Neutral, 2.0, Dash::Dashed) },
            Series::Limit { id: "theta-cr".into(), label: loc("Critical θ_a,cr", "Kritisch θ_a,cr"), axis: AxisRef::X, value: theta_cr.clamp(20.0, 1200.0), style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "ky-at-end".into(), at: Vec2::new(theta_end.clamp(20.0, 1200.0), part_1_2::k_y_theta(theta_end)), shape: MarkerShape::Circle, tone: rating_tone, label: loc(&format!("k_y,θ = {:.2} at {theta_end:.0} °C", part_1_2::k_y_theta(theta_end)), &format!("k_y,θ = {:.2} bei {theta_end:.0} °C", part_1_2::k_y_theta(theta_end))), utilisation: None, bound_check: check.map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: true,
        table_twin: DataTable { columns: vec![column("Steel temperature θ_a", "Stahltemperatur θ_a", "°C", thetas.clone()), column("k_y,θ", "k_y,θ", "1", thetas.iter().map(|t| part_1_2::k_y_theta(*t)).collect()), column("k_E,θ", "k_E,θ", "1", thetas.iter().map(|t| part_1_2::k_e_theta(*t)).collect())] },
    };
    let utilisation = {
        let curve: Vec<Xy> = history.iter().step_by(6).map(|(t, theta)| [t / 60.0, (exposure.mu0 / part_1_2::k_y_theta(*theta).max(1e-3)).min(UTILISATION_CAP)]).collect();
        let failure = super::checks::clauses::fatigue_fire::time_to_temperature(&history, theta_cr).map(|t| t / 60.0);
        let mut markers = vec![Marker { id: "utilisation-at-rating".into(), at: Vec2::new(minutes, (exposure.mu0 / part_1_2::k_y_theta(theta_end).max(1e-3)).min(UTILISATION_CAP)), shape: MarkerShape::Circle, tone: rating_tone, label: loc(&format!("μ0/k_y,θ = {:.2} at R{minutes:.0}", exposure.mu0 / part_1_2::k_y_theta(theta_end).max(1e-3)), &format!("μ0/k_y,θ = {:.2} bei R{minutes:.0}", exposure.mu0 / part_1_2::k_y_theta(theta_end).max(1e-3))), utilisation: Some(theta_end / theta_cr.max(1.0)), bound_check: check.map(|c| c.id.clone()), binding: None }];
        if let Some(t) = failure {
            markers.push(Marker { id: "critical-time".into(), at: Vec2::new(t, 1.0), shape: MarkerShape::Diamond, tone: utilisation_tone(minutes / t.max(1e-9)), label: loc(&format!("θ_a,cr reached after {t:.1} min"), &format!("θ_a,cr erreicht nach {t:.1} min")), utilisation: Some(minutes / t.max(1e-9)), bound_check: check.map(|c| c.id.clone()), binding: None });
        }
        Diagram2d {
            id: format!("fire-utilisation:{}", exposure.id),
            title: loc(&format!("Utilisation in fire over time – {}", member.label), &format!("Ausnutzung im Brandfall über der Zeit – {}", member.label)),
            description: loc("Degree of utilisation μ0 / k_y,θ(t) of the cross-section resistance over time from the steel temperature history (EN 1993-1-2, 4.2.3.2 with Table 3.1); the member fails when it reaches 1, the critical temperature of 4.2.4 marks the same instant by its fitted formula.", "Ausnutzungsgrad μ0 / k_y,θ(t) des Querschnittswiderstands über der Zeit aus dem Stahltemperaturverlauf (EN 1993-1-2, 4.2.3.2 mit Tabelle 3.1); das Bauteil versagt bei 1, die kritische Temperatur nach 4.2.4 kennzeichnet denselben Zeitpunkt mit ihrer Näherungsformel."),
            axes: vec![axis("Time t", "Zeit t", QuantityKind::Time, "min", AxisScale::Linear, Some((0.0, end))), axis("Utilisation μ0 / k_y,θ", "Ausnutzung μ0 / k_y,θ", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, UTILISATION_CAP * 1.05)))],
            series: vec![
                Series::Polyline { id: "mu".into(), label: loc("μ0 / k_y,θ", "μ0 / k_y,θ"), points: curve.iter().copied().map(p).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
                Series::Limit { id: "limit".into(), label: loc("Failure μ = 1", "Versagen μ = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
                Series::Limit { id: "rating".into(), label: loc(&format!("Rating R{minutes:.0}"), &format!("Feuerwiderstand R{minutes:.0}")), axis: AxisRef::X, value: minutes, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
            ],
            markers,
            annotations: Vec::new(),
            focus: focus.clone(),
            legend: true,
            table_twin: DataTable { columns: vec![column("Time t", "Zeit t", "min", curve.iter().map(|q| q[0]).collect()), column("Utilisation μ0 / k_y,θ", "Ausnutzung μ0 / k_y,θ", "1", curve.iter().map(|q| q[1]).collect())] },
        }
    };
    Some(vec![temperature, reduction, utilisation])
}

/// 📈️ Every diagram of the document; with `focus` set to an entity id only the diagrams of that entity (and, for a member, those of its fatigue and fire details).
pub fn diagrams(document: &En1993Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1993, document.annex))
}

fn diagrams_unbased(document: &En1993Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let matches = |ids: &[&str]| focus.is_none_or(|f| ids.contains(&f));
    let mut out = Vec::new();
    for member in &document.members {
        let related: Vec<&str> = super::kit::dependents(document, &member.id);
        if !matches(&related) {
            continue;
        }
        let own = focus.is_none_or(|f| f == member.id);
        if own {
            out.extend(buckling(document, report, member));
            out.extend(ltb(document, report, member).into_iter().flatten());
            out.extend(interaction(document, report, member));
            out.extend(classification(document, member));
        }
        for detail in document.fatigue_details.iter().filter(|f| f.member_id == member.id && focus.is_none_or(|x| x == f.id || x == member.id)) {
            out.extend(fatigue(report, detail));
        }
        for exposure in document.fire_exposures.iter().filter(|f| f.member_id == member.id && focus.is_none_or(|x| x == f.id || x == member.id)) {
            out.extend(fire(document, report, exposure).into_iter().flatten());
        }
    }
    out.extend(entity_diagrams::entity_diagrams(document, report, focus));
    out.extend(annex_diagrams::annex_diagrams(document, report, focus));
    out.extend(design_diagrams::design_diagrams(document, report, focus));
    out.extend(norm_diagrams::norm_diagrams(document, report, focus));
    out.extend(super::detail_sketches::diagrams(document, report, focus));
    out.extend(super::fatigue_views::diagrams(document, report, focus));
    out
}
