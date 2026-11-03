//! 📏️ Crack width against the quasi-permanent moment (EN 1992-1-1 7.3.4): w_k = s_r,max (ε_sm − ε_cm) of the crack-width check swept over the moment, from the cracking moment to above the design moment, against the limit w_max with the design point. The sweep calls the same case and the same entry as the check, so the point on the curve is the value of the report.

use super::checks::{self, part_1_1};
use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::model::RcMember;
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

const SAMPLES: usize = 48;
const OVERSHOOT: f64 = 1.6;

/// 📏️ One point of the sweep: moment in N·m, crack width in m, steel stress in Pa and maximum crack spacing in m; the width is 0 while the section is uncracked.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Row {
    pub moment: f64,
    pub width: f64,
    pub sigma_s: f64,
    pub sr_max: f64,
}

/// 🧮️ The crack width at the moment `m_qp` (N·m) and normal force `n_ed` (N, tension positive) of the member; an uncracked section gives the row of width 0, a case the entry refuses gives `None`.
pub fn row(m: &RcMember, sec: &checks::Sec, annex: super::norm::AnnexChoice, f_ck: f64, e_s: f64, m_qp: f64, n_ed: f64) -> Option<Row> {
    let case = checks::crack_case(m, sec, annex, f_ck, e_s, m_qp, n_ed);
    match checks::crack_width(&case) {
        Ok(k) => Some(Row { moment: m_qp, width: k.wk * 1e-3, sigma_s: k.sigma_s * 1e6, sr_max: k.sr_max * 1e-3 }),
        Err(e) if e.starts_with("section is uncracked") => Some(Row { moment: m_qp, width: 0.0, sigma_s: 0.0, sr_max: 0.0 }),
        Err(_) => None,
    }
}

/// 📈️ Diagram `crack-width-moment:<member>`: w_k(M_qp) with the limit w_max and the design point; `None` for members without a quasi-permanent combination or bending.
pub fn diagram(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let doc = input.document;
    let (concrete, steel) = (doc.concrete(&m.concrete_grade_id)?, doc.reinforcement(&m.reinforcement_grade_id)?);
    let effects = checks::combine_member_actions(m, &doc.action_factors());
    let qp = checks::governing(&effects, "sls_qp")?;
    let m_design = qp.m_ed.abs();
    if m_design <= 1e-6 || sec.bars.is_empty() {
        return None;
    }
    let rows: Vec<Row> = (0..=SAMPLES).filter_map(|i| row(m, sec, doc.annex, concrete.f_ck, steel.e_s, m_design * OVERSHOOT * i as f64 / SAMPLES as f64, qp.n_ed)).collect();
    let at_design = row(m, sec, doc.annex, concrete.f_ck, steel.e_s, m_design, qp.n_ed)?;
    if rows.len() < 3 {
        return None;
    }
    let w_max = part_1_1::w_max_m(doc.annex, m.exposure) * 1e3;
    let binding = binding(input.report, &m.id, &["en1992.7.3.crack."]);
    let tone = tone_of(&binding);
    let curve: Vec<[f64; 2]> = rows.iter().map(|r| [r.moment * 1e-3, r.width * 1e3]).collect();
    let cracking = rows.iter().find(|r| r.width > 0.0).map(|r| r.moment * 1e-3);
    let mut g = base(
        m,
        "crack-width-moment",
        titled(m, "Crack width against moment", "Rissbreite über dem Moment"),
        loc("Calculated crack width w_k = s_r,max (ε_sm − ε_cm) of EN 1992-1-1 7.3.4 over the quasi-permanent moment, zero up to the cracking moment, against the limit w_max of Table 7.1N; the marker is the design moment of the crack-width check.", "Berechnete Rissbreite w_k = s_r,max (ε_sm − ε_cm) nach EN 1992-1-1 7.3.4 über dem quasi-ständigen Moment, null bis zum Rissmoment, gegen den Grenzwert w_max der Tabelle 7.1N; die Markierung ist das Bemessungsmoment des Rissbreitennachweises."),
        vec![axis("Quasi-permanent moment M", "Quasi-ständiges Moment M", QuantityKind::Moment, "kN·m"), axis("Crack width w_k", "Rissbreite w_k", QuantityKind::Length, "mm")],
        kind,
        path,
    );
    g.series.push(Series::Polyline { id: "wk".into(), label: loc("Crack width w_k(M)", "Rissbreite w_k(M)"), points: xy(&curve), style: stroke(2.0, Dash::Solid, Tone::Info) });
    g.series.push(Series::Limit { id: "w-max".into(), label: loc(&format!("w_max = {w_max:.2} mm"), &format!("w_max = {w_max:.2} mm")), axis: AxisRef::Y, value: w_max, style: stroke(1.0, Dash::Dashed, Tone::Warning) });
    g.markers.push(Marker { id: "design".into(), at: Vec2::new(at_design.moment * 1e-3, at_design.width * 1e3), shape: MarkerShape::Diamond, tone, label: loc(&format!("w_k = {:.3} mm at M = {:.1} kNm", at_design.width * 1e3, at_design.moment * 1e-3), &format!("w_k = {:.3} mm bei M = {:.1} kNm", at_design.width * 1e3, at_design.moment * 1e-3)), utilisation: Some(at_design.width * 1e3 / w_max), bound_check: binding.map(|b| b.check_id), binding: None });
    if let Some(cr) = cracking {
        g.markers.push(Marker { id: "reference:cracking".into(), at: Vec2::new(cr, 0.0), shape: MarkerShape::Triangle, tone: Tone::Neutral, label: loc(&format!("Cracking from M = {cr:.1} kNm"), &format!("Rissbildung ab M = {cr:.1} kNm")), utilisation: None, bound_check: None, binding: None });
    }
    g.table_twin = DataTable { columns: vec![
        column("Moment M", "Moment M", "kN·m", rows.iter().map(|r| r.moment * 1e-3).collect()),
        column("Crack width w_k", "Rissbreite w_k", "mm", rows.iter().map(|r| r.width * 1e3).collect()),
        column("Steel stress σ_s", "Stahlspannung σ_s", "MPa", rows.iter().map(|r| r.sigma_s * 1e-6).collect()),
        column("Crack spacing s_r,max", "Rissabstand s_r,max", "mm", rows.iter().map(|r| r.sr_max * 1e3).collect()),
    ] };
    Some(g)
}
