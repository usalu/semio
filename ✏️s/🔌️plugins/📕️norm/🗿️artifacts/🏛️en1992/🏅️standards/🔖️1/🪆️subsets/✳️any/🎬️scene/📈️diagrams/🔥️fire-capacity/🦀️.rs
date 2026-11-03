//! 🔥️ Fire capacity over time: the reduced bending resistance M_Rd,fi(t) of the 500 °C isotherm section (EN 1992-1-2 Annex B.1) at 15-minute steps with the fire design moment of the fire check as a level.

use super::checks::fire_field;
use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::mechanics;
use super::model::RcMember;
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

const STEP_MINUTES: f64 = 15.0;
const STEPS: usize = 8;

fn capacity(m: &RcMember, sec: &super::checks::Sec, f_yk_mpa: f64, n_comp: f64, minutes: f64) -> f64 {
    let (outline, faces) = (m.outline(), fire_field::exposed_faces(m.kind));
    let field = fire_field::solve_outline(&outline, faces, minutes, fire_field::MOISTURE_DEFAULT);
    let ring: Vec<(f64, f64)> = field.isotherm_ring(fire_field::LEVEL_500, 72).iter().map(|p| (p.0 * 1000.0, p.1 * 1000.0)).collect();
    if ring.len() < 3 {
        return 0.0;
    }
    fire_field::reduced_section(sec, ring, &field, f_yk_mpa).m_rd_y(n_comp).map_or(0.0, |r| r.0 / 1.0e6)
}

/// 📈️ Diagram `fire-capacity:<member>`: M_Rd,fi in kNm over the fire time in minutes, with the fire design moment M_Ed,fi recovered from the utilisation of the fire check `en1992.1-2.nm.<member>`; `None` without a fire spec, bars or a steel grade.
pub fn diagram(input: &SceneInput, m: &RcMember, sec: &super::checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let fire = m.fire.as_ref()?;
    let f_yk_mpa = input.document.reinforcement(&m.reinforcement_grade_id)?.f_yk / 1.0e6;
    if sec.bars.is_empty() {
        return None;
    }
    let n_comp = mechanics::reported_effects(input.report, m).unwrap_or_else(|| mechanics::design_effects(m, &input.document.action_factors())).n;
    let curve: Vec<[f64; 2]> = (0..=STEPS).map(|i| i as f64 * STEP_MINUTES).map(|t| [t, capacity(m, sec, f_yk_mpa, n_comp, t.max(1.0))]).collect();
    let rated = fire_field::rating_minutes(fire.rating);
    let check = input.report.checks.iter().find(|c| c.id == format!("en1992.1-2.nm.{}", m.id));
    let m_rd_rated = capacity(m, sec, f_yk_mpa, n_comp, rated);
    let demand = check.map(|c| c.utilization * m_rd_rated.max(1.0e-3));
    let tone = check.map_or(Tone::Info, |c| super::scene::utilisation_tone(c.utilization));
    let mut g = base(
        m,
        "fire-capacity",
        titled(m, "Fire capacity over time", "Brandtragfähigkeit über die Zeit"),
        loc("Reduced bending resistance M_Rd,fi(t) of the 500 degC isotherm section (EN 1992-1-2 Annex B.1) at the axial force of the governing flexure check; the level is the fire design moment M_Ed,fi of the fire check at the rated duration. The k_s(theta) table is unverified against the National Annex.", "Reduzierte Biegetragfähigkeit M_Rd,fi(t) des 500-°C-Isothermenquerschnitts (EN 1992-1-2 Anhang B.1) bei der Normalkraft des maßgebenden Biegenachweises; die Linie ist das Brandbemessungsmoment M_Ed,fi des Brandnachweises zur Feuerwiderstandsdauer. Die k_s(theta)-Tabelle ist gegen den Nationalen Anhang ungeprüft."),
        vec![axis("Fire time", "Branddauer", QuantityKind::Time, "min"), axis("Bending resistance M_Rd,fi", "Biegetragfähigkeit M_Rd,fi", QuantityKind::Moment, "kN·m")],
        kind,
        path,
    );
    g.series.push(Series::Polyline { id: "m-rd-fi".into(), label: loc("M_Rd,fi(t)", "M_Rd,fi(t)"), points: xy(&curve), style: stroke(2.0, Dash::Solid, tone) });
    if let Some(d) = demand {
        g.series.push(Series::Polyline { id: "m-ed-fi".into(), label: loc("M_Ed,fi", "M_Ed,fi"), points: xy(&[[0.0, d], [STEPS as f64 * STEP_MINUTES, d]]), style: stroke(1.5, Dash::Dashed, Tone::Warning) });
    }
    let rated_check = check.map(|c| c.id.clone());
    g.markers.push(Marker { id: super::marker_id("rated", &rated_check), at: Vec2::new(rated, m_rd_rated), shape: MarkerShape::Diamond, tone, label: loc(&format!("Rated duration {rated:.0} min, M_Rd,fi = {m_rd_rated:.1} kNm"), &format!("Feuerwiderstandsdauer {rated:.0} min, M_Rd,fi = {m_rd_rated:.1} kNm")), utilisation: check.map(|c| c.utilization), bound_check: rated_check, binding: None });
    g.table_twin = DataTable { columns: vec![column("Fire time", "Branddauer", "min", curve.iter().map(|p| p[0]).collect()), column("M_Rd,fi", "M_Rd,fi", "kN·m", curve.iter().map(|p| p[1]).collect())] };
    Some(g)
}
