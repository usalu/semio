//! 📈️ Diagrams: aluminium buckling curves by class with the member markers, LTB curves and M_b,Rd over the restraint spacing, the M–N interaction polygon with its utilisation ray,
//! the classification β/ε per element and the effective thickness ρ_c, the HAZ factors ρ_o,haz and ρ_u,haz and the HAZ width over the plate thickness, the S–N curves by detail category,
//! the fire temperature and k_θ curves, the shell buckling reduction, the stress–strain line of each alloy with its HAZ-reduced line, the bolt group in plan and the fillet weld resistance of each connection
//! and the local buckling and utilisation over the thickness of each cold-formed sheet.

use super::checks::part_1_1::{self, AlloyProps, EffCtx};
use super::checks::{na_de, part_1_2, part_1_3, part_1_4_1_5, part_en1990};
use super::facts;
use super::checks;
use super::model::{AluminiumMaterial, AluminiumMember, AluminiumSection, AluminiumShell, En1999Snapshot, FatigueDetail, FireScenario, PatchLoad, WeldRun};
use super::norm::{CheckReport, CheckResult};
use super::scene::diagram_kit::{curves, interaction_polygon, Xy};
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{column, stroke, axis};

const LAMBDA_MAX: f64 = 3.0;
const SAMPLES: usize = 60;
const FATIGUE_CATEGORIES: [f64; 7] = [28.0, 40.0, 56.0, 71.0, 90.0, 112.0, 125.0];

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

fn exact<'a>(report: &'a CheckReport, id: &str) -> Option<&'a CheckResult> {
    report.checks.iter().find(|c| c.id == id && c.utilization.is_finite())
}

fn k_support(member: &AluminiumMember) -> f64 {
    match member.support {
        super::model::SupportCondition::Continuous => 0.7,
        super::model::SupportCondition::Cantilever => 2.0,
        super::model::SupportCondition::SimplySupported => 1.0,
    }
}

fn member_focus(id: &str) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Member, id, format!("members[id={id}]")))
}

struct Ctx<'a> {
    report: &'a CheckReport,
    member: &'a AluminiumMember,
    section: &'a AluminiumSection,
    designation: &'a str,
    alloy: AlloyProps,
    params: na_de::AnnexParams,
    reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    eta: f64,
    haz: Vec<(String, f64)>,
}

impl Ctx<'_> {
    fn base(&self) -> EffCtx {
        EffCtx { f_o_pa: self.alloy.f_o_pa, class_a: self.alloy.buckling_class_a, eta_web: self.eta, local: true, haz_rho: None, haz_extents: &self.haz }
    }

    fn class(&self) -> part_1_1::SectionClass {
        part_1_1::section_class(self.section, self.alloy.f_o_pa, self.alloy.buckling_class_a, self.eta)
    }
}

#[path = "🦴️web-panels/🦀️.rs"]
mod web_panels;

#[path = "🪟️patch-resistance/🦀️.rs"]
mod patch_resistance;

#[path = "📉️stress-strain/🦀️.rs"]
mod stress_strain;

#[path = "🔩️connection-diagrams/🦀️.rs"]
mod connection_diagrams;

#[path = "🥫️sheet-diagrams/🦀️.rs"]
mod sheet_diagrams;

#[path = "🌀️torsion/🦀️.rs"]
mod torsion;

#[path = "📊️outcomes/🦀️.rs"]
mod outcome_diagrams;

fn chi_series(prefix: &str, name: &str, alpha: f64, lambda_0: f64, tone: Tone, width: f64) -> (Series, Vec<f64>) {
    let lambdas: Vec<f64> = (0..=SAMPLES).map(|i| LAMBDA_MAX * i as f64 / SAMPLES as f64).collect();
    let values: Vec<f64> = lambdas.iter().map(|l| part_1_1::chi_from_lambda(*l, alpha, lambda_0)).collect();
    let points = lambdas.iter().zip(&values).map(|(x, y)| Vec2::new(*x, *y)).collect();
    (Series::Polyline { id: format!("{prefix}-{name}"), label: loc(&format!("Class {name}: α = {alpha:.2}, λ̄₀ = {lambda_0:.2}"), &format!("Klasse {name}: α = {alpha:.2}, λ̄₀ = {lambda_0:.2}")), points, style: stroke(tone, width, Dash::Solid) }, values)
}

fn lambda_column(en: &str, de: &str) -> DataColumn {
    column(en, de, "1", (0..=SAMPLES).map(|i| LAMBDA_MAX * i as f64 / SAMPLES as f64).collect())
}

fn buckling(c: &Ctx) -> Option<Diagram2d> {
    let (alpha, lambda_0) = part_1_1::buckling_curve(c.alloy.buckling_class_a);
    let a_loc = part_1_1::props(c.section, &c.base()).a;
    let (_, i_y, i_z, ..) = part_1_1::section_geometry(c.section);
    let k = k_support(c.member);
    let fo = c.alloy.f_o_pa;
    let mut markers = Vec::new();
    for (axis_name, length, inertia, field, id) in [("y", c.member.buckling_length_y, i_y, "bucklingLengthY", format!("en1999.6.3.1.fb.{}", c.member.id)), ("z", c.member.buckling_length_z, i_z, "bucklingLengthZ", format!("en1999.6.3.1.fb.z.{}", c.member.id))] {
        let l_cr = k * if length > 0.0 { length } else { c.member.length };
        let lambda = part_1_1::lambda_bar(l_cr, inertia, a_loc, fo);
        let chi = part_1_1::chi_from_lambda(lambda, alpha, lambda_0);
        let per_metre = part_1_1::lambda_bar(1.0, inertia, a_loc, fo) * k;
        let check = exact(c.report, &id);
        let mut binding = FieldBinding::new(format!("members[id={}].{field}", c.member.id), Affine { base: 0.0, k: if per_metre > 1e-12 { 1.0 / per_metre } else { 1.0 } }, "m");
        binding.quantise = Some(Quantise::Round { step: 0.05 });
        let utilisation = check.map(|x| x.utilization);
        markers.push(Marker {
            id: format!("marker-{axis_name}"),
            at: Vec2::new(lambda.min(LAMBDA_MAX), chi),
            shape: MarkerShape::Circle,
            tone: utilisation.map_or(Tone::Neutral, utilisation_tone),
            label: loc(&format!("{axis_name}–{axis_name}: λ̄ = {lambda:.2}, χ = {chi:.3}"), &format!("{axis_name}–{axis_name}: λ̄ = {lambda:.2}, χ = {chi:.3}")),
            utilisation,
            bound_check: check.map(|x| x.id.clone()),
            binding: (per_metre > 1e-12).then_some(binding),
        });
    }
    let (series_a, values_a) = chi_series("chi", "A", 0.20, 0.10, if c.alloy.buckling_class_a { Tone::Info } else { Tone::Neutral }, if c.alloy.buckling_class_a { 2.5 } else { 1.0 });
    let (series_b, values_b) = chi_series("chi", "B", 0.32, 0.0, if c.alloy.buckling_class_a { Tone::Neutral } else { Tone::Info }, if c.alloy.buckling_class_a { 1.0 } else { 2.5 });
    let mut series = vec![series_a, series_b];
    series.push(Series::Polyline { id: "euler".into(), label: loc("Euler hyperbola 1/λ̄²", "Euler-Hyperbel 1/λ̄²"), points: curves::sample(|l| 1.0 / (l * l), 1.0, LAMBDA_MAX, SAMPLES).into_iter().map(p).collect(), style: stroke(Tone::Neutral, 1.0, Dash::Dashed) });
    Some(Diagram2d {
        id: format!("buckling:{}", c.member.id),
        title: loc(&format!("Aluminium buckling curves – {}", c.member.id), &format!("Aluminium-Knicklinien – {}", c.member.id)),
        description: loc(&format!("Reduction factor χ over the relative slenderness λ̄ for buckling classes A and B (EN 1999-1-1, 6.3.1.2, Table 6.6); {} belongs to class {}.", c.designation, if c.alloy.buckling_class_a { "A" } else { "B" }), &format!("Abminderungsfaktor χ über dem bezogenen Schlankheitsgrad λ̄ für die Knickklassen A und B (EN 1999-1-1, 6.3.1.2, Tabelle 6.6); {} gehört zur Klasse {}.", c.designation, if c.alloy.buckling_class_a { "A" } else { "B" })),
        axes: vec![axis("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction factor χ", "Abminderungsfaktor χ", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: member_focus(&c.member.id),
        legend: true,
        table_twin: DataTable { columns: vec![lambda_column("Relative slenderness λ̄", "Bezogener Schlankheitsgrad λ̄"), column("χ class A", "χ Klasse A", "1", values_a), column("χ class B", "χ Klasse B", "1", values_b)] },
    })
}

fn ltb_length(c: &Ctx) -> f64 {
    if c.member.ltb_length > 0.0 { c.member.ltb_length } else { c.member.length }
}

fn ltb(c: &Ctx) -> Option<Vec<Diagram2d>> {
    let m_rd = exact(c.report, &format!("en1999.6.2.5.m.{}", c.member.id))?.limit.value;
    if m_rd <= 0.0 {
        return None;
    }
    let check = exact(c.report, &format!("en1999.6.3.2.ltb.{}", c.member.id));
    let utilisation = check.map(|x| x.utilization);
    let l_lt = ltb_length(c);
    let (alpha_lt, lambda_0_lt) = part_1_1::ltb_curve(c.class());
    let g1 = c.params.gamma_m1;
    let m_cr_at = |l: f64| part_1_1::m_cr_of(c.section, c.member.c1, l);
    let lambda_at = |l: f64| { let m = m_cr_at(l); if m.is_finite() && m > 0.0 { (m_rd * g1 / m).sqrt() } else { 0.0 } };
    let lambda_lt = if c.member.restrained_ltb { 0.0 } else { lambda_at(l_lt) };
    let chi_lt = part_1_1::chi_from_lambda(lambda_lt, alpha_lt, lambda_0_lt);
    let (series_12, values_12) = chi_series("chi-lt", "12", 0.10, 0.6, if c.class() <= part_1_1::SectionClass::Class2 { Tone::Info } else { Tone::Neutral }, if c.class() <= part_1_1::SectionClass::Class2 { 2.5 } else { 1.0 });
    let (series_34, values_34) = chi_series("chi-lt", "34", 0.20, 0.4, if c.class() <= part_1_1::SectionClass::Class2 { Tone::Neutral } else { Tone::Info }, if c.class() <= part_1_1::SectionClass::Class2 { 1.0 } else { 2.5 });
    let chi_diagram = Diagram2d {
        id: format!("ltb:{}", c.member.id),
        title: loc(&format!("Lateral-torsional buckling curves – {}", c.member.id), &format!("Biegedrillknicklinien – {}", c.member.id)),
        description: loc("Reduction factor χ_LT over λ̄_LT for the section classes 1/2 (α = 0.10, λ̄₀ = 0.6) and 3/4 (α = 0.20, λ̄₀ = 0.4) of EN 1999-1-1, 6.3.2.2; the marker is this member.", "Abminderungsfaktor χ_LT über λ̄_LT für die Querschnittsklassen 1/2 (α = 0,10, λ̄₀ = 0,6) und 3/4 (α = 0,20, λ̄₀ = 0,4) nach EN 1999-1-1, 6.3.2.2; der Marker ist dieses Bauteil."),
        axes: vec![axis("Relative slenderness λ̄_LT", "Bezogener Schlankheitsgrad λ̄_LT", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction factor χ_LT", "Abminderungsfaktor χ_LT", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series: vec![series_12, series_34],
        markers: vec![Marker { id: "marker-ltb".into(), at: Vec2::new(lambda_lt.min(LAMBDA_MAX), chi_lt), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, utilisation_tone), label: loc(&format!("λ̄_LT = {lambda_lt:.2}, χ_LT = {chi_lt:.3}"), &format!("λ̄_LT = {lambda_lt:.2}, χ_LT = {chi_lt:.3}")), utilisation, bound_check: check.map(|x| x.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: member_focus(&c.member.id),
        legend: true,
        table_twin: DataTable { columns: vec![lambda_column("Relative slenderness λ̄_LT", "Bezogener Schlankheitsgrad λ̄_LT"), column("χ_LT classes 1/2", "χ_LT Klassen 1/2", "1", values_12), column("χ_LT classes 3/4", "χ_LT Klassen 3/4", "1", values_34)] },
    };
    let mut out = vec![chi_diagram];
    if !c.member.restrained_ltb {
        let upper = c.member.length.max(l_lt * 1.5).max(1.0);
        let resistance = |l: f64| part_1_1::chi_from_lambda(lambda_at(l), alpha_lt, lambda_0_lt) * m_rd / 1.0e3;
        let points = curves::sample(&resistance, 0.25, upper, SAMPLES);
        let demand = check.map_or(0.0, |x| x.computed.value / 1.0e3);
        let now = resistance(l_lt);
        let mut binding = FieldBinding::new(format!("members[id={}].ltbLength", c.member.id), Affine::IDENTITY, "m");
        binding.quantise = Some(Quantise::Round { step: 0.05 });
        out.push(Diagram2d {
            id: format!("ltb-spacing:{}", c.member.id),
            title: loc(&format!("Buckling resistance over unrestrained length – {}", c.member.id), &format!("Biegedrillknickwiderstand über Gabellagerungslänge – {}", c.member.id)),
            description: loc("M_b,Rd over the unrestrained length L_LT with the design moment as limit; drag the marker to change L_LT.", "M_b,Rd über der Gabellagerungslänge L_LT mit dem Bemessungsmoment als Grenze; den Marker ziehen, um L_LT zu ändern."),
            axes: vec![axis("Unrestrained length L_LT", "Gabellagerungslänge L_LT", QuantityKind::Length, "m", AxisScale::Linear, None), axis("Resistance M_b,Rd", "Widerstand M_b,Rd", QuantityKind::Moment, "kN·m", AxisScale::Linear, None)],
            series: [Series::Polyline { id: "mb-rd".into(), label: loc("M_b,Rd", "M_b,Rd"), points: points.iter().copied().map(p).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }].into_iter().chain((demand > 0.0).then(|| Series::Limit { id: "m-ed".into(), label: loc("Design moment M_y,Ed", "Bemessungsmoment M_y,Ed"), axis: AxisRef::Y, value: demand, style: stroke(Tone::Warning, 1.5, Dash::Dashed) })).collect(),
            markers: vec![Marker { id: "marker-spacing".into(), at: Vec2::new(l_lt, now), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, utilisation_tone), label: loc(&format!("L_LT = {l_lt:.2} m, M_b,Rd = {now:.2} kN·m"), &format!("L_LT = {l_lt:.2} m, M_b,Rd = {now:.2} kN·m")), utilisation, bound_check: check.map(|x| x.id.clone()), binding: Some(binding) }],
            annotations: Vec::new(),
            focus: member_focus(&c.member.id),
            legend: true,
            table_twin: DataTable { columns: vec![column("Unrestrained length L_LT", "Gabellagerungslänge L_LT", "m", points.iter().map(|q| q[0]).collect()), column("M_b,Rd", "M_b,Rd", "kN·m", points.iter().map(|q| q[1]).collect())] },
        });
    }
    Some(out)
}

fn interaction(c: &Ctx) -> Option<Diagram2d> {
    let fo = c.alloy.f_o_pa;
    let class_a = c.alloy.buckling_class_a;
    let m_rd = exact(c.report, &format!("en1999.6.2.5.m.{}", c.member.id))?.limit.value / 1.0e3;
    let has_haz = c.section.elements.iter().any(|e| e.welded);
    let gross = part_1_1::props(c.section, &EffCtx::gross(fo, class_a));
    let eff_o = part_1_1::props(c.section, &EffCtx { haz_rho: Some(c.alloy.rho_o_haz), ..c.base() });
    let haz_o = part_1_1::props(c.section, &EffCtx { local: false, haz_rho: Some(c.alloy.rho_o_haz), ..c.base() });
    let haz_u = part_1_1::props(c.section, &EffCtx { local: false, haz_rho: Some(c.alloy.rho_u_haz), ..c.base() });
    let (g1, g2) = (c.params.gamma_m1, c.params.gamma_m2);
    let n_t = (haz_o.a * fo / g1).min((if has_haz { haz_u.a } else { 0.9 * gross.a }) * c.alloy.f_u_pa / g2) / 1.0e3;
    let n_c = eff_o.a * fo / g1 / 1.0e3;
    if n_t <= 0.0 || n_c <= 0.0 || m_rd <= 0.0 {
        return None;
    }
    let linear = c.class() >= part_1_1::SectionClass::Class3 || has_haz;
    let (psi, eta_y, _) = part_1_1::section_interaction_exponents(c.section.kind);
    let m_at = |n: f64| {
        let r = n.abs() / if n >= 0.0 { n_t } else { n_c };
        (if linear { m_rd * (1.0 - r) } else { m_rd * (1.0 - r.powf(psi)).max(0.0).powf(1.0 / eta_y) }).max(0.0)
    };
    let points: Vec<Xy> = (0..=40).flat_map(|i| { let n = -n_c + (n_t + n_c) * i as f64 / 40.0; [[n, m_at(n)], [n, -m_at(n)]] }).collect();
    let ring = interaction_polygon::interaction_polygon(&points);
    let mut series = vec![Series::Polygon { id: "capacity".into(), label: loc("M–N capacity", "M–N-Tragfähigkeit"), ring: ring.iter().copied().map(p).collect(), fill: Fill { tone: Tone::Info, opacity: 0.15 }, hatch: None }];
    let mut markers = Vec::new();
    let mut notes = Vec::new();
    let check = exact(c.report, &format!("en1999.6.2.9.nm.{}", c.member.id));
    let best = part_en1990::uls_combinations(c.member, &c.params, c.reliability_class).into_iter().filter_map(|g| {
        let demand = [g.n_ed / 1.0e3, g.m_y_ed.abs() / 1.0e3];
        interaction_polygon::utilisation_ray(&ring, [0.0, 0.0], demand).map(|hit| (hit.utilisation, hit.boundary, demand, g))
    }).max_by(|a, b| a.0.total_cmp(&b.0));
    if let Some((ratio, boundary, demand, g)) = best {
        series.push(Series::Ray { id: "ray".into(), label: loc("Utilisation ray", "Ausnutzungsstrahl"), origin: Vec2::ZERO, through: Vec2::new(demand[0], demand[1]), to_boundary: true, style: stroke(Tone::Warning, 1.5, Dash::Dotted) });
        notes.push(DiagramNote { id: "ray-ratio".into(), at: Vec2::new(boundary[0], boundary[1]), text: loc(&format!("|OA|/|OB| = {ratio:.2}"), &format!("|OA|/|OB| = {ratio:.2}")), tone: utilisation_tone(ratio) });
        markers.push(Marker { id: "demand".into(), at: Vec2::new(demand[0], demand[1]), shape: MarkerShape::Diamond, tone: utilisation_tone(ratio), label: loc(&format!("N_Ed = {:.1} kN, M_y,Ed = {:.2} kN·m ({})", demand[0], demand[1], g.combination), &format!("N_Ed = {:.1} kN, M_y,Ed = {:.2} kN·m ({})", demand[0], demand[1], g.combination)), utilisation: Some(ratio), bound_check: check.map(|x| x.id.clone()), binding: None });
    }
    Some(Diagram2d {
        id: format!("mn:{}", c.member.id),
        title: loc(&format!("M–N interaction – {}", c.member.id), &format!("M–N-Interaktion – {}", c.member.id)),
        description: loc(&format!("Bending resistance M_y,Rd over the axial force N (EN 1999-1-1, 6.2.9, {}) with the governing design effect and the utilisation ray.", if linear { "linear: class 3/4 or welded" } else { "plastic exponents" }), &format!("Biegewiderstand M_y,Rd über der Normalkraft N (EN 1999-1-1, 6.2.9, {}) mit der maßgebenden Beanspruchung und dem Ausnutzungsstrahl.", if linear { "linear: Klasse 3/4 oder geschweißt" } else { "plastische Exponenten" })),
        axes: vec![axis("Axial force N", "Normalkraft N", QuantityKind::Force, "kN", AxisScale::Linear, None), axis("Bending moment M_y", "Biegemoment M_y", QuantityKind::Moment, "kN·m", AxisScale::Linear, None)],
        series,
        markers,
        annotations: notes,
        focus: member_focus(&c.member.id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Axial force N", "Normalkraft N", "kN", ring.iter().map(|q| q[0]).collect()), column("Bending moment M_y", "Biegemoment M_y", "kN·m", ring.iter().map(|q| q[1]).collect())] },
    })
}

fn classification(c: &Ctx) -> Option<Vec<Diagram2d>> {
    if c.section.elements.is_empty() {
        return None;
    }
    let rows: Vec<(usize, facts::Classified, bool, bool)> = c.section.elements.iter().enumerate().map(|(i, e)| (i, facts::classify(e, &c.alloy, c.eta), e.outstand, e.welded)).collect();
    let tone_of_class = |class: part_1_1::SectionClass| match class {
        part_1_1::SectionClass::Class1 | part_1_1::SectionClass::Class2 => Tone::Info,
        part_1_1::SectionClass::Class3 => Tone::Warning,
        part_1_1::SectionClass::Class4 => Tone::Error,
    };
    let at = |i: usize| (i + 1) as f64;
    let limit_points = |k: usize| -> Vec<ScatterPoint> { rows.iter().map(|(i, r, ..)| ScatterPoint { at: Vec2::new(at(*i), r.limits[k]), tone: Tone::Neutral, label: None }).collect() };
    let series = vec![
        Series::Scatter { id: "limit-1".into(), label: loc("Class 1 limit β₁", "Grenze Klasse 1 β₁"), points: limit_points(0) },
        Series::Scatter { id: "limit-2".into(), label: loc("Class 2 limit β₂", "Grenze Klasse 2 β₂"), points: limit_points(1) },
        Series::Scatter { id: "limit-3".into(), label: loc("Class 3 limit β₃", "Grenze Klasse 3 β₃"), points: limit_points(2) },
    ];
    let markers: Vec<Marker> = rows.iter().map(|(i, r, ..)| {
        let element = &c.section.elements[*i];
        Marker { id: format!("element-{}", element.id), at: Vec2::new(at(*i), r.beta_over_eps), shape: MarkerShape::Square, tone: tone_of_class(r.class), label: loc(&format!("{}: β/ε = {:.2}, class {}", element.id, r.beta_over_eps, r.class as u8), &format!("{}: β/ε = {:.2}, Klasse {}", element.id, r.beta_over_eps, r.class as u8)), utilisation: Some(r.beta_over_eps / r.limits[2]), bound_check: exact(c.report, &format!("en1999.6.1.4.class.{}.{}", c.member.id, element.id)).map(|x| x.id.clone()), binding: None }
    }).collect();
    let top = rows.iter().map(|(_, r, ..)| r.beta_over_eps.max(r.limits[2])).fold(1.0_f64, f64::max) * 1.15;
    let classification = Diagram2d {
        id: format!("class:{}", c.member.id),
        title: loc(&format!("Classification β/ε per plate – {}", c.member.id), &format!("Klassifizierung β/ε je Blech – {}", c.member.id)),
        description: loc("Slenderness β/ε of every plate element of the section against the class limits of EN 1999-1-1, Table 6.2; elements above the class 3 limit are class 4 and carry the local buckling reduction ρ_c.", "Schlankheit β/ε jedes Blechelements des Querschnitts gegenüber den Klassengrenzen nach EN 1999-1-1, Tabelle 6.2; Elemente oberhalb der Grenze der Klasse 3 sind Klasse 4 und tragen die Beulabminderung ρ_c."),
        axes: vec![axis("Plate element", "Blechelement", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, rows.len() as f64 + 0.5))), axis("Slenderness β/ε", "Schlankheit β/ε", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, top)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: member_focus(&c.member.id),
        legend: true,
        table_twin: DataTable { columns: vec![
            column("Plate element", "Blechelement", "1", rows.iter().map(|(i, ..)| at(*i)).collect()),
            column("Slenderness β/ε", "Schlankheit β/ε", "1", rows.iter().map(|(_, r, ..)| r.beta_over_eps).collect()),
            column("Limit class 1", "Grenze Klasse 1", "1", rows.iter().map(|(_, r, ..)| r.limits[0]).collect()),
            column("Limit class 2", "Grenze Klasse 2", "1", rows.iter().map(|(_, r, ..)| r.limits[1]).collect()),
            column("Limit class 3", "Grenze Klasse 3", "1", rows.iter().map(|(_, r, ..)| r.limits[2]).collect()),
            column("Class", "Klasse", "1", rows.iter().map(|(_, r, ..)| facts::class_number(r.class)).collect()),
            column("Reduction ρ_c", "Abminderung ρ_c", "1", rows.iter().map(|(_, r, ..)| r.rho).collect()),
        ] },
    };
    let x_max = rows.iter().map(|(_, r, ..)| r.beta_over_eps).fold(10.0_f64, f64::max).max(40.0);
    let xs: Vec<f64> = (0..=SAMPLES).map(|i| x_max * i as f64 / SAMPLES as f64).collect();
    let types = [(false, false, "internal", "innen"), (false, true, "internal welded", "innen geschweißt"), (true, false, "outstand", "auskragend"), (true, true, "outstand welded", "auskragend geschweißt")];
    let rho_series: Vec<(Series, Vec<f64>)> = types.iter().map(|(outstand, welded, en, de)| {
        let values: Vec<f64> = xs.iter().map(|x| part_1_1::rho_c(*x, c.alloy.buckling_class_a, *outstand, *welded)).collect();
        let tone = if *welded { Tone::Warning } else { Tone::Info };
        (Series::Polyline { id: format!("rho-{}-{}", if *outstand { "outstand" } else { "internal" }, if *welded { "welded" } else { "plain" }), label: loc(&format!("ρ_c {en}"), &format!("ρ_c {de}")), points: xs.iter().zip(&values).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(tone, if *outstand { 1.5 } else { 2.5 }, if *welded { Dash::Dashed } else { Dash::Solid }) }, values)
    }).collect();
    let rho_markers: Vec<Marker> = rows.iter().map(|(i, r, ..)| {
        let element = &c.section.elements[*i];
        Marker { id: format!("rho-{}", element.id), at: Vec2::new(r.beta_over_eps.min(x_max), r.rho), shape: MarkerShape::Circle, tone: if r.rho < 1.0 { Tone::Warning } else { Tone::Info }, label: loc(&format!("{}: ρ_c = {:.3}", element.id, r.rho), &format!("{}: ρ_c = {:.3}", element.id, r.rho)), utilisation: None, bound_check: exact(c.report, &format!("en1999.6.1.4.class.{}.{}", c.member.id, element.id)).map(|x| x.id.clone()), binding: None }
    }).collect();
    let mut twin = vec![column("Slenderness β/ε", "Schlankheit β/ε", "1", xs.clone())];
    twin.extend(rho_series.iter().zip(types).map(|((_, values), (_, _, en, de))| column(&format!("ρ_c {en}"), &format!("ρ_c {de}"), "1", values.clone())));
    let effective = Diagram2d {
        id: format!("effective:{}", c.member.id),
        title: loc(&format!("Effective thickness ρ_c – {}", c.member.id), &format!("Wirksame Dicke ρ_c – {}", c.member.id)),
        description: loc("Local buckling reduction ρ_c = C₁/(β/ε) − C₂/(β/ε)² of class 4 plates (EN 1999-1-1, 6.1.5, Table 6.3); t_eff = ρ_c·t. Markers are the plates of this section.", "Beulabminderung ρ_c = C₁/(β/ε) − C₂/(β/ε)² von Blechen der Klasse 4 (EN 1999-1-1, 6.1.5, Tabelle 6.3); t_eff = ρ_c·t. Die Marker sind die Bleche dieses Querschnitts."),
        axes: vec![axis("Slenderness β/ε", "Schlankheit β/ε", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, x_max))), axis("Reduction ρ_c", "Abminderung ρ_c", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series: rho_series.into_iter().map(|(s, _)| s).collect(),
        markers: rho_markers,
        annotations: Vec::new(),
        focus: member_focus(&c.member.id),
        legend: true,
        table_twin: DataTable { columns: twin },
    };
    Some(vec![classification, effective])
}

fn haz(c: &Ctx) -> Option<Vec<Diagram2d>> {
    let welded: Vec<_> = c.section.elements.iter().filter(|e| e.welded).collect();
    if welded.is_empty() {
        return None;
    }
    let ts: Vec<f64> = (2..=60).map(|i| i as f64 * 0.5).collect();
    let bands: Vec<AlloyProps> = ts.iter().filter_map(|t| part_1_1::resolve_alloy_for_thickness(c.designation, t / 1000.0)).collect();
    if bands.len() != ts.len() {
        return None;
    }
    let rho_o: Vec<Vec2> = ts.iter().zip(&bands).map(|(t, b)| Vec2::new(*t, b.rho_o_haz)).collect();
    let rho_u: Vec<Vec2> = ts.iter().zip(&bands).map(|(t, b)| Vec2::new(*t, b.rho_u_haz)).collect();
    let b_haz: Vec<f64> = ts.iter().map(|t| facts::haz_extent(t / 1000.0) * 1000.0).collect();
    let marker = |e: &super::model::PlateElement, id_prefix: &str, value: f64, label: String| {
        let mut binding = FieldBinding::new(format!("sections[id={}].elements[id={}].thickness", c.section.id, e.id), Affine { base: 0.0, k: 1.0e-3 }, "m");
        binding.quantise = Some(Quantise::Round { step: 0.0005 });
        Marker { id: format!("reference:{id_prefix}-{}", e.id), at: Vec2::new((e.thickness * 1000.0).clamp(1.0, 30.0), value), shape: MarkerShape::Circle, tone: Tone::Warning, label: loc(&label, &label), utilisation: None, bound_check: None, binding: Some(binding) }
    };
    let rho_markers: Vec<Marker> = welded.iter().filter_map(|e| part_1_1::resolve_alloy_for_thickness(c.designation, e.thickness).map(|a| marker(e, "rho", a.rho_o_haz, format!("{}: t = {:.1} mm, ρ_o,haz = {:.2}, ρ_u,haz = {:.2}", e.id, e.thickness * 1e3, a.rho_o_haz, a.rho_u_haz)))).collect();
    let width_markers: Vec<Marker> = welded.iter().map(|e| marker(e, "bhaz", facts::haz_extent(e.thickness) * 1000.0, format!("{}: t = {:.1} mm, b_haz = {:.0} mm", e.id, e.thickness * 1e3, facts::haz_extent(e.thickness) * 1000.0))).collect();
    let factors = Diagram2d {
        id: format!("haz:{}", c.member.id),
        title: loc(&format!("HAZ strength factors – {} ({})", c.member.id, c.designation), &format!("WEZ-Festigkeitsfaktoren – {} ({})", c.member.id, c.designation)),
        description: loc("Reduction factors ρ_o,haz (0.2 % proof strength) and ρ_u,haz (ultimate strength) of the heat-affected zone over the plate thickness (EN 1999-1-1, 6.1.6, Table 3.2b); markers are the welded plates, draggable along the thickness.", "Abminderungsfaktoren ρ_o,haz (0,2-%-Dehngrenze) und ρ_u,haz (Zugfestigkeit) der Wärmeeinflusszone über der Blechdicke (EN 1999-1-1, 6.1.6, Tabelle 3.2b); die Marker sind die geschweißten Bleche, entlang der Dicke ziehbar."),
        axes: vec![axis("Plate thickness t", "Blechdicke t", QuantityKind::Length, "mm", AxisScale::Linear, Some((1.0, 30.0))), axis("HAZ factor ρ_haz", "WEZ-Faktor ρ_haz", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series: vec![
            Series::Polyline { id: "rho-o-haz".into(), label: loc("ρ_o,haz", "ρ_o,haz"), points: rho_o.clone(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "rho-u-haz".into(), label: loc("ρ_u,haz", "ρ_u,haz"), points: rho_u.clone(), style: stroke(Tone::Neutral, 2.0, Dash::Dashed) },
        ],
        markers: rho_markers,
        annotations: Vec::new(),
        focus: member_focus(&c.member.id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Plate thickness t", "Blechdicke t", "mm", ts.clone()), column("ρ_o,haz", "ρ_o,haz", "1", rho_o.iter().map(|q| q.y).collect()), column("ρ_u,haz", "ρ_u,haz", "1", rho_u.iter().map(|q| q.y).collect()), column("HAZ width b_haz", "WEZ-Breite b_haz", "mm", b_haz.clone())] },
    };
    let width = Diagram2d {
        id: format!("hazwidth:{}", c.member.id),
        title: loc(&format!("HAZ width – {}", c.member.id), &format!("WEZ-Breite – {}", c.member.id)),
        description: loc("Extent b_haz of the heat-affected zone on each side of a MIG weld over the plate thickness (EN 1999-1-1, 6.1.6.3); markers are the welded plates.", "Ausdehnung b_haz der Wärmeeinflusszone beidseits einer MIG-Schweißnaht über der Blechdicke (EN 1999-1-1, 6.1.6.3); die Marker sind die geschweißten Bleche."),
        axes: vec![axis("Plate thickness t", "Blechdicke t", QuantityKind::Length, "mm", AxisScale::Linear, Some((1.0, 30.0))), axis("HAZ width b_haz", "WEZ-Breite b_haz", QuantityKind::Length, "mm", AxisScale::Linear, Some((0.0, 50.0)))],
        series: vec![Series::Polyline { id: "b-haz".into(), label: loc("b_haz", "b_haz"), points: ts.iter().zip(&b_haz).map(|(t, w)| Vec2::new(*t, *w)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }],
        markers: width_markers,
        annotations: Vec::new(),
        focus: member_focus(&c.member.id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Plate thickness t", "Blechdicke t", "mm", ts), column("HAZ width b_haz", "WEZ-Breite b_haz", "mm", b_haz)] },
    };
    Some(vec![factors, width])
}

/// 📈️ Governing fatigue spectrum of the document: the group of fatigue details with the largest Miner damage.
fn governing_fatigue<'a>(d: &'a En1999Snapshot) -> Option<&'a str> {
    let params = na_de::AnnexParams::for_choice(d.annex);
    part_1_3::spectrum_groups(&d.fatigue_details).into_iter().filter_map(|g| {
        let (dc, m1, m2) = part_1_3::spectrum_curve(&d.fatigue_details[g[0]])?;
        let blocks: Vec<(f64, f64)> = g.iter().map(|i| (d.fatigue_details[*i].delta_sigma_ed, d.fatigue_details[*i].n_cycles)).collect();
        Some((d.fatigue_details[g[0]].id.as_str(), part_1_3::spectrum_damage(&blocks, dc, m1, m2, params.gamma_ff, params.gamma_mf)))
    }).max_by(|a, b| a.1.total_cmp(&b.1)).map(|g| g.0)
}

fn fatigue(d: &En1999Snapshot, report: &CheckReport, group: &[&FatigueDetail], governing: bool) -> Option<Diagram2d> {
    let detail = *group.first()?;
    let params = na_de::AnnexParams::for_choice(d.annex);
    let own = part_1_3::detail_category_delta_sigma_c(&detail.detail_category).unwrap_or(detail.delta_sigma_c);
    let (m1, m2) = (detail.m1, if detail.m2 > 0.0 { detail.m2 } else { detail.m1 + 2.0 });
    if own <= 0.0 || m1 <= 0.0 {
        return None;
    }
    let ns = curves::log_space(1.0e4, 1.0e8, SAMPLES);
    let strength = |dc: f64| -> Vec<Vec2> { ns.iter().map(|n| Vec2::new(*n, part_1_3::fatigue_strength_pa(dc, m1, m2, *n) / 1.0e6)).collect() };
    let mut series: Vec<Series> = FATIGUE_CATEGORIES.iter().filter(|c| (*c * 1.0e6 - own).abs() > 1.0).map(|c| Series::Polyline { id: format!("category-{c:.0}"), label: loc(&format!("Δσ_C = {c:.0} MPa"), &format!("Δσ_C = {c:.0} MPa")), points: strength(c * 1.0e6), style: stroke(Tone::Neutral, 1.0, Dash::Dotted) }).collect();
    let own_points = strength(own);
    series.push(Series::Polyline { id: "wohler".into(), label: loc(&format!("Δσ_C = {own_mpa:.0} MPa (this detail)", own_mpa = own / 1.0e6), &format!("Δσ_C = {own_mpa:.0} MPa (dieses Detail)", own_mpa = own / 1.0e6)), points: own_points.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) });
    series.push(Series::Limit { id: "delta-d".into(), label: loc("Knee Δσ_D", "Knickpunkt Δσ_D"), axis: AxisRef::Y, value: part_1_3::delta_sigma_d(own, m1) / 1.0e6, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) });
    series.push(Series::Limit { id: "delta-l".into(), label: loc("Cut-off Δσ_L", "Schwellenwert Δσ_L"), axis: AxisRef::Y, value: part_1_3::delta_sigma_l(own, m1, m2) / 1.0e6, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
    let blocks: Vec<(f64, f64)> = group.iter().map(|g| (g.delta_sigma_ed, g.n_cycles)).collect();
    let damage = part_1_3::spectrum_damage(&blocks, own, m1, m2, params.gamma_ff, params.gamma_mf);
    let check = if group.len() > 1 { exact(report, &format!("en1999.1-3.fat.miner.{}", detail.id)) } else { exact(report, &format!("en1999.1-3.fat.{}", detail.id)) };
    let ratio = check.map_or(damage, |c| c.utilization);
    let markers: Vec<Marker> = group.iter().map(|g| {
        let share = part_1_3::damage(g.delta_sigma_ed, g.n_cycles, own, m1, m2, params.gamma_ff, params.gamma_mf);
        let demand = g.delta_sigma_ed * params.gamma_ff / 1.0e6;
        let mut binding = FieldBinding::new(format!("fatigueDetails[id={}].nCycles", g.id), Affine::IDENTITY, "1");
        binding.quantise = Some(Quantise::Round { step: 1.0e4 });
        Marker { id: if group.len() > 1 { format!("block-{}", g.id) } else { "demand".into() }, at: Vec2::new(g.n_cycles.clamp(1.0e4, 1.0e8), demand.clamp(5.0, 300.0)), shape: MarkerShape::Diamond, tone: utilisation_tone(ratio), label: loc(&format!("{}: γ_Ff·Δσ_Ed = {demand:.1} MPa, n = {:.2e}, D_i = {share:.3}", g.id, g.n_cycles), &format!("{}: γ_Ff·Δσ_Ed = {demand:.1} MPa, n = {:.2e}, D_i = {share:.3}", g.id, g.n_cycles)), utilisation: Some(ratio), bound_check: check.map(|c| c.id.clone()), binding: Some(binding) }
    }).collect();
    let tag = |en: bool| match (governing, en) { (true, true) => " – governing detail", (true, false) => " – maßgebendes Detail", _ => "" };
    Some(Diagram2d {
        id: format!("sn:{}", detail.id),
        title: loc(&format!("S–N curves by detail category – {}{}", detail.id, tag(true)), &format!("Wöhlerlinien nach Kerbfall – {}{}", detail.id, tag(false))),
        description: loc(&format!("Fatigue strength Δσ_R over the cycles N in log–log scale (EN 1999-1-3, Annex J) with slopes m₁ = {m1:.1}, m₂ = {m2:.1}; one marker per spectrum block (γ_Ff·Δσ_Ed at n), Miner damage D = {damage:.2} over {} block(s).", group.len()), &format!("Ermüdungsfestigkeit Δσ_R über den Lastspielen N doppeltlogarithmisch (EN 1999-1-3, Anhang J) mit den Neigungen m₁ = {m1:.1}, m₂ = {m2:.1}; je Kollektivblock ein Marker (γ_Ff·Δσ_Ed bei n), Miner-Schädigung D = {damage:.2} über {} Block/Blöcke.", group.len())),
        axes: vec![axis("Cycles N", "Lastspiele N", QuantityKind::Dimensionless, "1", AxisScale::Log10, Some((1.0e4, 1.0e8))), axis("Stress range Δσ", "Spannungsschwingbreite Δσ", QuantityKind::Stress, "MPa", AxisScale::Log10, Some((5.0, 300.0)))],
        series,
        markers,
        annotations: vec![DiagramNote { id: "damage".into(), at: Vec2::new(2.0e4, 200.0), text: loc(&format!("D = {damage:.2}{}", tag(true)), &format!("D = {damage:.2}{}", tag(false))), tone: utilisation_tone(ratio) }],
        focus: Some(EntityRef::new(EntityKind::Group, &detail.id, format!("fatigueDetails[id={}]", detail.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Cycles N", "Lastspiele N", "1", own_points.iter().map(|q| q.x).collect()), column("Strength Δσ_R", "Festigkeit Δσ_R", "MPa", own_points.iter().map(|q| q.y).collect())] },
    })
}

#[path = "../🔥️fire-diagrams/🦀️.rs"]
mod fire_diagrams;

fn fire(d: &En1999Snapshot, report: &CheckReport, fire: &FireScenario) -> Option<Vec<Diagram2d>> {
    let member = d.members.iter().find(|m| m.id == fire.member_id)?;
    let section = d.sections.iter().find(|s| s.id == member.section_id)?;
    let a_m_v = part_1_2::section_factor(section);
    let cover = d.fire_insulations.iter().find(|i| i.fire_id == fire.id).map(part_1_2::Insulation::from);
    let end_min = (fire.duration_s / 60.0 * 1.25).max(5.0);
    let times: Vec<f64> = (0..=30).map(|i| end_min * i as f64 / 30.0).collect();
    let gas: Vec<Vec2> = times.iter().map(|t| Vec2::new(*t, part_1_2::gas_temperature_c(t * 60.0))).collect();
    let metal: Vec<Vec2> = times.iter().map(|t| Vec2::new(*t, part_1_2::member_temperature_c(fire.theta_a, a_m_v, t * 60.0, cover.as_ref()))).collect();
    let theta_end = part_1_2::member_temperature_c(fire.theta_a, a_m_v, fire.duration_s.max(0.0), cover.as_ref());
    let check = exact(report, &format!("en1999.1-2.fire.theta-cr.{}", fire.id));
    let theta_cr = check.map_or(550.0, |c| c.limit.value);
    let focus = Some(EntityRef::new(EntityKind::Load, &fire.id, format!("fireScenarios[id={}]", fire.id)));
    let tone = if theta_end > theta_cr { Tone::Error } else { Tone::Info };
    let mut binding = FieldBinding::new(format!("fireScenarios[id={}].durationS", fire.id), Affine { base: 0.0, k: 60.0 }, "s");
    binding.quantise = Some(Quantise::Round { step: 60.0 });
    let temperature = Diagram2d {
        id: format!("fire:{}", fire.id),
        title: loc(&format!("Fire temperature curves – {}", member.id), &format!("Brandtemperaturverläufe – {}", member.id)),
        description: loc("ISO 834 gas temperature and aluminium temperature over time against the critical temperature (EN 1999-1-2, 4.2.5); drag the marker to change the duration.", "ISO-834-Gastemperatur und Aluminiumtemperatur über der Zeit gegenüber der kritischen Temperatur (EN 1999-1-2, 4.2.5); den Marker ziehen, um die Branddauer zu ändern."),
        axes: vec![axis("Time t", "Zeit t", QuantityKind::Time, "min", AxisScale::Linear, Some((0.0, end_min))), axis("Temperature θ", "Temperatur θ", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((0.0, 1100.0)))],
        series: vec![
            Series::Polyline { id: "gas".into(), label: loc("ISO 834 gas", "ISO-834-Gas"), points: gas.clone(), style: stroke(Tone::Warning, 1.5, Dash::Solid) },
            Series::Polyline { id: "aluminium".into(), label: loc("Aluminium θ_a", "Aluminium θ_a"), points: metal.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "theta-cr".into(), label: loc("Critical θ_cr", "Kritisch θ_cr"), axis: AxisRef::Y, value: theta_cr, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "duration".into(), at: Vec2::new((fire.duration_s / 60.0).clamp(0.0, end_min), theta_end.clamp(0.0, 1100.0)), shape: MarkerShape::Circle, tone, label: loc(&format!("θ_a = {theta_end:.0} °C after {:.0} min", fire.duration_s / 60.0), &format!("θ_a = {theta_end:.0} °C nach {:.0} min", fire.duration_s / 60.0)), utilisation: Some(theta_end / theta_cr.max(1.0)), bound_check: check.map(|c| c.id.clone()), binding: Some(binding) }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: true,
        table_twin: DataTable { columns: vec![column("Time t", "Zeit t", "min", times.clone()), column("Gas θ_g", "Gas θ_g", "°C", gas.iter().map(|q| q.y).collect()), column("Aluminium θ_a", "Aluminium θ_a", "°C", metal.iter().map(|q| q.y).collect())] },
    };
    let thetas: Vec<f64> = (0..=SAMPLES).map(|i| 20.0 + 530.0 * i as f64 / SAMPLES as f64).collect();
    let k_now = part_1_2::k_theta(theta_end);
    let reduction = Diagram2d {
        id: format!("fire-k:{}", fire.id),
        title: loc(&format!("Strength and stiffness reduction – {}", member.id), &format!("Festigkeits- und Steifigkeitsabminderung – {}", member.id)),
        description: loc("Reduction factors k_0.2proof,θ and k_E,θ of the aluminium alloy over the member temperature (EN 1999-1-2, Table 3.1) with the critical temperature.", "Abminderungsfaktoren k_0,2proof,θ und k_E,θ der Aluminiumlegierung über der Bauteiltemperatur (EN 1999-1-2, Tabelle 3.1) mit der kritischen Temperatur."),
        axes: vec![axis("Member temperature θ_a", "Bauteiltemperatur θ_a", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((20.0, 550.0))), axis("Reduction factor k", "Abminderungsfaktor k", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.05)))],
        series: vec![
            Series::Polyline { id: "k-theta".into(), label: loc("k_0.2proof,θ", "k_0,2proof,θ"), points: thetas.iter().map(|t| Vec2::new(*t, part_1_2::k_theta(*t))).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "k-e-theta".into(), label: loc("k_E,θ", "k_E,θ"), points: thetas.iter().map(|t| Vec2::new(*t, part_1_2::k_e_theta(*t))).collect(), style: stroke(Tone::Neutral, 2.0, Dash::Dashed) },
            Series::Limit { id: "theta-cr".into(), label: loc("Critical θ_cr", "Kritisch θ_cr"), axis: AxisRef::X, value: theta_cr.clamp(20.0, 550.0), style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id("k-at-end", &check.map(|c| c.id.clone())), at: Vec2::new(theta_end.clamp(20.0, 550.0), k_now), shape: MarkerShape::Circle, tone, label: loc(&format!("k_0.2proof,θ = {k_now:.2} at {theta_end:.0} °C"), &format!("k_0,2proof,θ = {k_now:.2} bei {theta_end:.0} °C")), utilisation: None, bound_check: check.map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Member temperature θ_a", "Bauteiltemperatur θ_a", "°C", thetas.clone()), column("k_0.2proof,θ", "k_0,2proof,θ", "1", thetas.iter().map(|t| part_1_2::k_theta(*t)).collect()), column("k_E,θ", "k_E,θ", "1", thetas.iter().map(|t| part_1_2::k_e_theta(*t)).collect())] },
    };
    let mut out = vec![temperature, reduction];
    out.extend(fire_diagrams::proof_strength_and_resistance(d, fire, member, section, theta_end, theta_cr, check.map(|c| c.id.clone())));
    if let Some((_, alloy)) = facts::member_parts(d, member) {
        if let Some(field) = part_1_2::temperature_field(section, alloy.f_o_pa, alloy.buckling_class_a, fire.theta_a, fire.duration_s.max(0.0), theta_end) {
            let mut strips: Vec<(f64, f64)> = facts::gross_bands(section, &alloy).iter().zip(&field.thetas).map(|(b, t)| (b.z * 1.0e3, *t)).collect();
            strips.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
            let (z_lo, z_hi) = (strips.first().map_or(0.0, |s| s.0), strips.last().map_or(1.0, |s| s.0));
            let hottest = strips.iter().map(|s| s.1).fold(0.0, f64::max);
            out.push(Diagram2d {
                id: format!("fire-field:{}", fire.id),
                title: loc(&format!("Temperature field over the section – {}", member.id), &format!("Temperaturfeld über den Querschnitt – {}", member.id)),
                description: loc("Unprotected temperature of every plate strip (section factor 2/t) over its height in the section against the critical temperature; thin flanges heat faster than thick webs.", "Temperatur jedes Blechstreifens (Profilfaktor 2/t) über seiner Höhe im Querschnitt gegenüber der kritischen Temperatur; dünne Flansche erwärmen sich schneller als dicke Stege."),
                axes: vec![axis("Height z", "Höhe z", QuantityKind::Length, "mm", AxisScale::Linear, Some((z_lo - 10.0, z_hi + 10.0))), axis("Temperature θ", "Temperatur θ", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((0.0, 1100.0)))],
                series: vec![
                    Series::Polyline { id: "strips".into(), label: loc("Strip temperature", "Streifentemperatur"), points: strips.iter().map(|s| Vec2::new(s.0, s.1)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
                    Series::Limit { id: "theta-cr".into(), label: loc("Critical θ_cr", "Kritisch θ_cr"), axis: AxisRef::Y, value: theta_cr, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
                ],
                markers: vec![Marker { id: "hottest".into(), at: Vec2::new(strips.iter().find(|s| s.1 == hottest).map_or(z_lo, |s| s.0), hottest.clamp(0.0, 1100.0)), shape: MarkerShape::Circle, tone: if hottest > theta_cr { Tone::Error } else { Tone::Info }, label: loc(&format!("Hottest strip {hottest:.0} °C"), &format!("Heißester Streifen {hottest:.0} °C")), utilisation: Some(hottest / theta_cr.max(1.0)), bound_check: check.map(|c| c.id.clone()), binding: None }],
                annotations: Vec::new(),
                focus: Some(EntityRef::new(EntityKind::Load, &fire.id, format!("fireScenarios[id={}]", fire.id))),
                legend: true,
                table_twin: DataTable { columns: vec![column("Height z", "Höhe z", "mm", strips.iter().map(|s| s.0).collect()), column("Strip temperature θ", "Streifentemperatur θ", "°C", strips.iter().map(|s| s.1).collect())] },
            });
        }
    }
    if let Some((_, alloy)) = facts::member_parts(d, member) {
        let history = part_1_2::strip_history(section, alloy.f_o_pa, alloy.buckling_class_a, fire.theta_a, &times.iter().map(|m| m * 60.0).collect::<Vec<_>>());
        if !history.is_empty() {
            let hottest = history.iter().enumerate().max_by(|a, b| a.1.last().unwrap_or(&0.0).total_cmp(b.1.last().unwrap_or(&0.0))).map_or(0, |(i, _)| i);
            let mut series: Vec<Series> = history.iter().enumerate().map(|(k, h)| Series::Polyline {
                id: format!("strip-{}", k + 1),
                label: loc(&format!("Strip {}", k + 1), &format!("Streifen {}", k + 1)),
                points: times.iter().zip(h).map(|(t, th)| Vec2::new(*t, *th)).collect(),
                style: if k == hottest { stroke(Tone::Warning, 2.5, Dash::Solid) } else { stroke(Tone::Info, 1.5, if k % 2 == 0 { Dash::Solid } else { Dash::Dashed }) },
            }).collect();
            series.push(Series::Limit { id: "theta-cr".into(), label: loc("Critical θ_cr", "Kritisch θ_cr"), axis: AxisRef::Y, value: theta_cr, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
            let mut columns = vec![column("Time t", "Zeit t", "min", times.clone())];
            columns.extend(history.iter().enumerate().map(|(k, h)| column(&format!("Strip {} θ", k + 1), &format!("Streifen {} θ", k + 1), "°C", h.clone())));
            out.push(Diagram2d {
                id: format!("fire-strips:{}", fire.id),
                title: loc(&format!("Strip temperature over time – {}", member.id), &format!("Streifentemperatur über der Zeit – {}", member.id)),
                description: loc("Unprotected temperature of every plate strip (section factor 2/t) over the fire duration against the critical temperature; the 3D view animates the same strips with the fire time slider.", "Temperatur jedes Blechstreifens (Profilfaktor 2/t) über der Branddauer gegenüber der kritischen Temperatur; die 3D-Ansicht animiert dieselben Streifen mit dem Branddauer-Regler."),
                axes: vec![axis("Time t", "Zeit t", QuantityKind::Time, "min", AxisScale::Linear, Some((0.0, end_min))), axis("Temperature θ", "Temperatur θ", QuantityKind::Temperature, "°C", AxisScale::Linear, Some((0.0, 1100.0)))],
                series,
                markers: Vec::new(),
                annotations: Vec::new(),
                focus: Some(EntityRef::new(EntityKind::Load, &fire.id, format!("fireScenarios[id={}]", fire.id))),
                legend: true,
                table_twin: DataTable { columns },
            });
        }
    }
    Some(out)
}

fn shell(d: &En1999Snapshot, report: &CheckReport, shell: &AluminiumShell) -> Option<Diagram2d> {
    let material = d.materials.iter().find(|m| m.id == shell.material_id)?;
    let alloy = part_1_1::resolve_alloy_for_thickness(&material.designation, shell.thickness)?;
    let (r, t, l) = (shell.radius, shell.thickness, shell.length);
    let alpha = part_1_4_1_5::alpha_x(r, t, part_1_4_1_5::shell_quality_q("B"));
    let lambda = (alloy.f_o_pa / part_1_4_1_5::sigma_x_rcr(r, t, l)).sqrt();
    let chi = |x: f64| part_1_4_1_5::shell_chi(x, alpha, 0.60, 1.0, 0.20);
    let xs: Vec<f64> = (0..=SAMPLES).map(|i| LAMBDA_MAX * i as f64 / SAMPLES as f64).collect();
    let ys: Vec<f64> = xs.iter().map(|x| chi(*x)).collect();
    let check = exact(report, &format!("en1999.1-5.buckle.{}", shell.id));
    let utilisation = check.map(|c| c.utilization);
    Some(Diagram2d {
        id: format!("shell:{}", shell.id),
        title: loc(&format!("Shell buckling reduction – {}", shell.id), &format!("Schalenbeul-Abminderung – {}", shell.id)),
        description: loc(&format!("Reduction χ_x over the relative shell slenderness λ̄_x (EN 1999-1-5, 5.3) with α_x = {alpha:.3} (fabrication class B), β = 0.60, η = 1.0, λ̄₀ = 0.20."), &format!("Abminderung χ_x über dem bezogenen Schalenschlankheitsgrad λ̄_x (EN 1999-1-5, 5.3) mit α_x = {alpha:.3} (Ausführungsklasse B), β = 0,60, η = 1,0, λ̄₀ = 0,20.")),
        axes: vec![axis("Relative slenderness λ̄_x", "Bezogener Schlankheitsgrad λ̄_x", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, LAMBDA_MAX))), axis("Reduction factor χ_x", "Abminderungsfaktor χ_x", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.1)))],
        series: vec![Series::Polyline { id: "chi-x".into(), label: loc("χ_x", "χ_x"), points: xs.iter().zip(&ys).map(|(x, y)| Vec2::new(*x, *y)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }],
        markers: vec![Marker { id: "shell".into(), at: Vec2::new(lambda.min(LAMBDA_MAX), chi(lambda)), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, utilisation_tone), label: loc(&format!("λ̄_x = {lambda:.2}, χ_x = {:.3}", chi(lambda)), &format!("λ̄_x = {lambda:.2}, χ_x = {:.3}", chi(lambda))), utilisation, bound_check: check.map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Shell, &shell.id, format!("shells[id={}]", shell.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Relative slenderness λ̄_x", "Bezogener Schlankheitsgrad λ̄_x", "1", xs), column("Reduction factor χ_x", "Abminderungsfaktor χ_x", "1", ys)] },
    })
}

fn width_marker(section: &AluminiumSection, y: f64, z: f64) -> Marker {
    let mut binding = FieldBinding::new(format!("sections[id={}].width", section.id), Affine { base: 0.0, k: 2.0 }, "m");
    binding.quantise = Some(Quantise::Round { step: 0.001 });
    Marker { id: "reference:width".into(), at: Vec2::new(y, z), shape: MarkerShape::Diamond, tone: Tone::Warning, label: loc(&format!("Width b = {:.1} mm", 2e3 * y), &format!("Breite b = {:.1} mm", 2e3 * y)), utilisation: None, bound_check: None, binding: Some(binding) }
}

fn web_marker(section: &AluminiumSection, outer: f64, inner: f64) -> Marker {
    let (map, thickness) = if outer == 0.0 { (Affine { base: 0.0, k: 2.0 }, 2.0 * inner) } else { (Affine { base: outer, k: -1.0 }, outer - inner) };
    let mut binding = FieldBinding::new(format!("sections[id={}].webThickness", section.id), map, "m");
    binding.quantise = Some(Quantise::Round { step: 0.0005 });
    Marker { id: "reference:web-thickness".into(), at: Vec2::new(inner, 0.0), shape: MarkerShape::Circle, tone: Tone::Warning, label: loc(&format!("Web thickness t_w = {:.1} mm", 1e3 * thickness), &format!("Stegdicke t_w = {:.1} mm", 1e3 * thickness)), utilisation: None, bound_check: None, binding: Some(binding) }
}

fn outline_diagram(d: &En1999Snapshot, section: &AluminiumSection) -> Option<Diagram2d> {
    use super::model::SectionKind;
    let free = checks::rings(&d.outline_vertices, &section.id);
    let (rings, props, free_outline) = match checks::outline_props(&free) {
        Some(props) => (free.iter().map(|r| (r.name.clone(), r.points.clone())).collect::<Vec<_>>(), props, true),
        None => {
            let polygon = super::kit::section_shape(section).outline().ok()?.into_iter().next()?;
            let points = |ring: &Ring2| ring.points.iter().map(|q| (q.x, q.y)).collect::<Vec<_>>();
            let outer = points(&polygon.outer);
            let holes: Vec<Vec<(f64, f64)>> = polygon.holes.iter().map(points).collect();
            let props = checks::polygon_props(&outer, &holes.iter().map(Vec::as_slice).collect::<Vec<_>>());
            (std::iter::once(("outer".to_string(), outer)).chain(holes.into_iter().enumerate().map(|(i, h)| (format!("hole-{}", i + 1), h))).collect(), props, false)
        }
    };
    let mut series = Vec::new();
    let (mut ys, mut zs) = (Vec::new(), Vec::new());
    for (name, ring) in &rings {
        let mut points: Vec<Vec2> = ring.iter().map(|q| Vec2::new(q.0, q.1)).collect();
        points.push(points[0].clone());
        for q in ring {
            ys.push(q.0);
            zs.push(q.1);
        }
        series.push(Series::Polyline { id: format!("ring-{name}"), label: loc(&format!("Ring {name}"), &format!("Ring {name}")), points, style: stroke(if name == "outer" { Tone::Info } else { Tone::Warning }, 2.0, Dash::Solid) });
    }
    let extent = |v: &[f64]| (v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let (y0, y1) = extent(&ys);
    let (z0, z1) = extent(&zs);
    let pad = 0.1 * (y1 - y0).max(z1 - z0).max(1e-3);
    let mut markers = vec![Marker { id: "reference:centroid".into(), at: Vec2::new(props.yg, props.zg), shape: MarkerShape::Circle, tone: Tone::Info, label: loc(&format!("Centroid ({:.1}, {:.1}) mm", props.yg * 1e3, props.zg * 1e3), &format!("Schwerpunkt ({:.1}, {:.1}) mm", props.yg * 1e3, props.zg * 1e3)), utilisation: None, bound_check: None, binding: None }];
    if free_outline {
        markers.extend(d.outline_vertices.iter().filter(|v| v.section_id == section.id).map(|v| Marker { id: format!("reference:vertex-{}", v.id), at: Vec2::new(v.y, v.z), shape: MarkerShape::Diamond, tone: Tone::Neutral, label: loc(&format!("Vertex {}: ({:.1}, {:.1}) mm", v.id, v.y * 1e3, v.z * 1e3), &format!("Punkt {}: ({:.1}, {:.1}) mm", v.id, v.y * 1e3, v.z * 1e3)), utilisation: None, bound_check: None, binding: None }));
    } else if section.kind != SectionKind::Chs {
        markers.push(width_marker(section, y1, z0));
        match section.kind {
            SectionKind::ExtrudedI if section.web_thickness > 0.0 => markers.push(web_marker(section, 0.0, section.web_thickness / 2.0)),
            SectionKind::Rhs | SectionKind::Box if rings.len() > 1 => markers.push(web_marker(section, y1, rings[1].1.iter().map(|q| q.0).fold(f64::NEG_INFINITY, f64::max))),
            _ => {}
        }
    }
    Some(Diagram2d {
        id: format!("outline:{}", section.id),
        title: loc(&format!("Section outline – {}", section.id), &format!("Querschnittsumriss – {}", section.id)),
        description: loc(&format!("Polygon with holes of section {} ({}): A = {:.1} mm², I_y = {:.3e} mm⁴, I_z = {:.3e} mm⁴; the centroid is marked{}.", section.id, if free_outline { "outline vertices" } else { "parametric profile" }, props.a * 1e6, props.iy * 1e12, props.iz * 1e12, if free_outline { ", the diamonds are the outline vertices" } else { ", the markers drag the width and the web thickness" }), &format!("Polygon mit Löchern des Querschnitts {} ({}): A = {:.1} mm², I_y = {:.3e} mm⁴, I_z = {:.3e} mm⁴; der Schwerpunkt ist markiert{}.", section.id, if free_outline { "Umrisspunkte" } else { "parametrisches Profil" }, props.a * 1e6, props.iy * 1e12, props.iz * 1e12, if free_outline { ", die Rauten sind die Umrisspunkte" } else { ", die Marker ändern Breite und Stegdicke" })),
        axes: vec![axis("Coordinate y", "Koordinate y", QuantityKind::Length, "m", AxisScale::Linear, Some((y0 - pad, y1 + pad))), axis("Coordinate z", "Koordinate z", QuantityKind::Length, "m", AxisScale::Linear, Some((z0 - pad, z1 + pad)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &section.id, format!("sections[id={}]", section.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Coordinate y", "Koordinate y", "m", ys), column("Coordinate z", "Koordinate z", "m", zs)] },
    })
}

fn weld_run_diagram(d: &En1999Snapshot, member: &AluminiumMember, section: &AluminiumSection) -> Option<Diagram2d> {
    let runs: Vec<&WeldRun> = d.weld_runs.iter().filter(|r| r.member_id == member.id).collect();
    if runs.is_empty() {
        return None;
    }
    let mut series = Vec::new();
    let (mut starts, mut ends, mut hazs) = (Vec::new(), Vec::new(), Vec::new());
    for (k, run) in runs.iter().enumerate() {
        let t = section.elements.iter().find(|e| e.id == run.element_id).map_or(0.006, |e| e.thickness);
        let b = checks::haz_extent(run, t);
        let (from, to) = if run.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse { (run.start, run.start) } else { (run.start, (run.start + run.length).min(member.length)) };
        let level = k as f64 + 1.0;
        series.push(Series::Polyline { id: format!("haz-{}", run.id), label: loc(&format!("HAZ of {}", run.id), &format!("WEZ von {}", run.id)), points: vec![Vec2::new((from - b).max(0.0), level), Vec2::new((to + b).min(member.length), level)], style: stroke(Tone::Warning, 6.0, Dash::Solid) });
        series.push(Series::Polyline { id: format!("run-{}", run.id), label: loc(&format!("Weld {}", run.id), &format!("Naht {}", run.id)), points: vec![Vec2::new(from, level), Vec2::new(to.max(from + 1e-6), level)], style: stroke(Tone::Error, 2.5, Dash::Solid) });
        starts.push(from);
        ends.push(to);
        hazs.push(b);
    }
    Some(Diagram2d {
        id: format!("weld-runs:{}", member.id),
        title: loc(&format!("Weld runs and HAZ along {}", member.id), &format!("Schweißnähte und WEZ entlang {}", member.id)),
        description: loc("Each weld run along the member axis (red) with its heat-affected zone b_haz on both sides (EN 1999-1-1, 6.1.6.3); a transverse run is a point and drives the localized-weld factor ω_x (6.3.3.3).", "Jede Schweißnaht entlang der Bauteilachse (rot) mit ihrer Wärmeeinflusszone b_haz beidseitig (EN 1999-1-1, 6.1.6.3); eine Quernaht ist ein Punkt und bestimmt den Faktor ω_x der örtlichen Schweißnaht (6.3.3.3)."),
        axes: vec![axis("Position x along the member", "Lage x entlang des Bauteils", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, member.length))), axis("Weld run", "Schweißnaht", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, runs.len() as f64 + 1.0)))],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: member_focus(&member.id),
        legend: true,
        table_twin: DataTable { columns: vec![column("Start x", "Anfang x", "m", starts), column("End x", "Ende x", "m", ends), column("HAZ b_haz", "WEZ b_haz", "m", hazs)] },
    })
}

/// 📈️ Every diagram of the document; with `focus` set to an entity id only the diagrams of that entity (and, for a member, those of its fatigue and fire details).
pub fn diagrams(document: &En1999Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1999, document.annex))
}

fn diagrams_unbased(document: &En1999Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let matches = |ids: &[&str]| focus.is_none_or(|f| ids.contains(&f));
    let mut out = Vec::new();
    for member in &document.members {
        let related = super::kit::dependents(document, &member.id);
        if !matches(&related) {
            continue;
        }
        if focus.is_none_or(|f| f == member.id || document.transverse_stiffeners.iter().any(|s| s.girder_id == member.id && s.id == f)) {
            out.extend(web_panels::web_panels(document, report, member));
        }
        if focus.is_none_or(|f| f == member.id) {
            out.extend(document.sections.iter().find(|s| s.id == member.section_id).and_then(|s| weld_run_diagram(document, member, s)));
            if let Some((section, alloy)) = facts::member_parts(document, member).filter(|(s, _)| !checks::is_free(s)) {
                let designation = document.materials.iter().find(|m| m.id == member.material_id).map_or("", |m| m.designation.as_str());
                let c = Ctx { report, member, section, designation, alloy, params: na_de::AnnexParams::for_choice(document.annex), reliability_class: document.reliability_class, eta: facts::eta_of(report, &member.id, section, &alloy), haz: checks::haz_overrides(&checks::runs_of(&document.weld_runs, &member.id)) };
                out.extend(buckling(&c));
                out.extend(torsion::torsional(&c));
                out.extend(torsion::welded(&c));
                out.extend(ltb(&c).into_iter().flatten());
                out.extend(interaction(&c));
                out.extend(classification(&c).into_iter().flatten());
                out.extend(haz(&c).into_iter().flatten());
            }
        }
        for group in part_1_3::spectrum_groups(&document.fatigue_details) {
            let rows: Vec<&FatigueDetail> = group.iter().map(|i| &document.fatigue_details[*i]).collect();
            if rows[0].member_id == member.id && focus.is_none_or(|x| x == member.id || rows.iter().any(|r| r.id == x)) {
                out.extend(fatigue(document, report, &rows, governing_fatigue(document) == Some(rows[0].id.as_str())));
            }
        }
        for scenario in document.fire_scenarios.iter().filter(|f| f.member_id == member.id && focus.is_none_or(|x| x == f.id || x == member.id)) {
            out.extend(fire(document, report, scenario).into_iter().flatten());
        }
    }
    for section in document.sections.iter().filter(|s| focus.is_none_or(|f| f == s.id || document.members.iter().any(|m| m.section_id == s.id && m.id == f) || document.outline_vertices.iter().any(|v| v.section_id == s.id && v.id == f))) {
        out.extend(outline_diagram(document, section));
    }
    for material in document.materials.iter().filter(|m| focus.is_none_or(|f| f == m.id)) {
        out.extend(stress_strain::stress_strain(material));
    }
    for load in document.patch_loads.iter().filter(|l| focus.is_none_or(|f| f == l.id || f == l.subject_id)) {
        out.extend(patch_resistance::patch_resistance(document, report, load));
    }
    for s in document.shells.iter().filter(|s| focus.is_none_or(|f| f == s.id)) {
        out.extend(shell(document, report, s));
    }
    for conn in document.connections.iter().filter(|c| focus.is_none_or(|f| f == c.id)) {
        out.extend(connection_diagrams::bolt_group(report, conn));
        out.extend(connection_diagrams::weld_resistance(document, report, conn));
    }
    for sheet in document.cold_formed.iter().filter(|s| focus.is_none_or(|f| f == s.id)) {
        out.extend(sheet_diagrams::sheet_local(document, report, sheet));
        out.extend(sheet_diagrams::sheet_resistance(document, report, sheet));
        out.extend(sheet_diagrams::sheet_profile(document, report, sheet));
    }
    out.extend(outcome_diagrams::outcomes(document, report, focus));
    out
}
