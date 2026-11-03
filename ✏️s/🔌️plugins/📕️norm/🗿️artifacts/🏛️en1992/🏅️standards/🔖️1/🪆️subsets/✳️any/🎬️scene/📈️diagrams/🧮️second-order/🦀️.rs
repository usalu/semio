//! 🧮️ Second-order effects of slender columns (EN 1992-1-1 5.8.3.2 and 5.8.8, nominal curvature): the M-N capacity polygon of the general section with the first-order point (M_0Ed, N_Ed) and the amplified point (M_Ed + M_2 + imperfection, N_Ed) that the slenderness check compares with M_Rd, joined by the horizontal growth of the moment. The amplified moment and M_Rd are read from the check, the first-order moment and the slenderness from the same formulas as the check, so picture and report cannot disagree.

use super::checks::{self, part_1_1};
use super::{axis, base, capacity_hull, stroke, titled, xy};
use super::kit::*;
use super::model::{MemberKind, RcMember};
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

/// 🧮️ The slenderness data of a column in N, N·m and mm, as the check evaluates them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecondOrder {
    pub n_ed: f64,
    pub m_first: f64,
    pub m_total: f64,
    pub m_rd: f64,
    pub lambda: f64,
    pub lambda_limit: f64,
    pub n_ratio: f64,
    pub omega: f64,
}

/// 🧮️ The first-order moment of a column: the larger of the design moment and the minimum eccentricity moment `N max(h/30, 20 mm)` of 6.1(4) (the check uses 0.02 m as the floor).
pub fn first_order_moment(m_ed: f64, n_ed: f64, h: f64) -> f64 {
    m_ed.abs().max(n_ed.abs() * (h / 30.0).max(0.02))
}

/// 🧮️ The second-order data of `m` or `None` for members that are not slender columns or walls, without the slenderness check or without a design axial force.
pub fn second_order(input: &SceneInput, m: &RcMember, sec: &checks::Sec) -> Option<SecondOrder> {
    if !matches!(m.kind, MemberKind::Column | MemberKind::Wall) || m.buckling_length <= 0.0 {
        return None;
    }
    let doc = input.document;
    let check = input.report.checks.iter().find(|c| c.id == format!("en1992.5.8.slender.{}", m.id) && c.utilization.is_finite())?;
    let (concrete, steel) = (doc.concrete(&m.concrete_grade_id)?, doc.reinforcement(&m.reinforcement_grade_id)?);
    let geo = checks::Geo::of(sec, m);
    let effects = checks::combine_member_actions(m, &doc.action_factors());
    let uls = checks::governing(&effects, "uls")?;
    let n_ed = uls.n_ed.abs();
    let (fcd, fyd) = (checks::f_cd(doc.annex, concrete.f_ck / 1e6, false) * 1e6, checks::f_yd(doc.annex, steel.f_yk / 1e6, false) * 1e6);
    let n_ratio = (n_ed / (geo.area * fcd).max(1.0)).min(1.0);
    let omega = geo.a_s * fyd / (geo.area * fcd).max(1e-9);
    Some(SecondOrder {
        n_ed,
        m_first: first_order_moment(uls.m_ed, n_ed, geo.h),
        m_total: check.computed.value.abs(),
        m_rd: check.limit.value.abs(),
        lambda: part_1_1::slenderness(m.buckling_length, geo.radius()),
        lambda_limit: part_1_1::lambda_limit(n_ratio, omega, doc.annex),
        n_ratio,
        omega,
    })
}

/// 📈️ Diagram `second-order:<member>`: capacity polygon, first-order point, amplified point and the growth between them; `None` for members that are not slender columns.
pub fn diagram(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let s = second_order(input, m, sec)?;
    let hull = capacity_hull(sec);
    if hull.len() < 3 {
        return None;
    }
    let binding = binding(input.report, &m.id, &["en1992.5.8.slender."]);
    let tone = tone_of(&binding);
    let (n, first, total) = (s.n_ed * 1e-3, s.m_first * 1e-3, s.m_total * 1e-3);
    let slender = s.lambda > s.lambda_limit;
    let mut g = base(
        m,
        "second-order",
        titled(m, "Second-order effects", "Effekte zweiter Ordnung"),
        loc("Slender column (EN 1992-1-1 5.8.3 and 5.8.8): the M-N capacity with the first-order moment M0Ed and the amplified moment M0Ed + M2 + imperfection at the design axial force; the column is verified when the amplified point lies inside the polygon.", "Schlanke Stütze (EN 1992-1-1 5.8.3 und 5.8.8): die M-N-Tragfähigkeit mit dem Moment erster Ordnung M0Ed und dem vergrößerten Moment M0Ed + M2 + Imperfektion bei der Bemessungsnormalkraft; die Stütze ist nachgewiesen, wenn der vergrößerte Punkt im Polygon liegt."),
        vec![axis("Bending moment M", "Biegemoment M", QuantityKind::Moment, "kN·m"), axis("Axial force N (compression positive)", "Normalkraft N (Druck positiv)", QuantityKind::Force, "kN")],
        kind,
        path,
    );
    g.series.push(Series::Polygon { id: "capacity".into(), label: loc("Capacity MRd-NRd", "Tragfähigkeit MRd-NRd"), ring: xy(&hull), fill: Fill { tone: Tone::Info, opacity: 0.2 }, hatch: None });
    g.series.push(Series::Polyline { id: "growth".into(), label: loc("Moment growth M2 and imperfection", "Momentenzuwachs M2 und Imperfektion"), points: xy(&[[first, n], [total, n]]), style: stroke(2.0, Dash::Dashed, tone) });
    g.series.push(Series::Scatter { id: "first-order".into(), label: loc("First order", "Erste Ordnung"), points: vec![ScatterPoint { at: Vec2::new(first, n), tone: Tone::Neutral, label: Some(loc("M0Ed, NEd", "M0Ed, NEd")) }] });
    g.markers.push(Marker { id: "amplified".into(), at: Vec2::new(total, n), shape: MarkerShape::Diamond, tone, label: loc(&format!("Amplified moment {:.1} kNm, λ = {:.1} {} λlim = {:.1}", total, s.lambda, if slender { ">" } else { "≤" }, s.lambda_limit), &format!("Vergrößertes Moment {:.1} kNm, λ = {:.1} {} λlim = {:.1}", total, s.lambda, if slender { ">" } else { "≤" }, s.lambda_limit)), utilisation: Some(s.m_total / s.m_rd.max(1.0)), bound_check: binding.map(|b| b.check_id), binding: None });
    let one = |en: &str, de: &str, unit: &str, v: f64| column(en, de, unit, vec![v]);
    g.table_twin = DataTable { columns: vec![
        one("Axial force N_Ed", "Normalkraft N_Ed", "kN", n),
        one("First-order moment M_0Ed", "Moment erster Ordnung M_0Ed", "kN·m", first),
        one("Amplified moment M_Ed", "Vergrößertes Moment M_Ed", "kN·m", total),
        one("Resistance M_Rd at N_Ed", "Tragmoment M_Rd bei N_Ed", "kN·m", s.m_rd * 1e-3),
        one("Slenderness λ", "Schlankheit λ", "1", s.lambda),
        one("Limit slenderness λ_lim", "Grenzschlankheit λ_lim", "1", s.lambda_limit),
        one("Relative normal force n", "Bezogene Normalkraft n", "1", s.n_ratio),
        one("Mechanical reinforcement ratio ω", "Mechanischer Bewehrungsgrad ω", "1", s.omega),
    ] };
    Some(g)
}
