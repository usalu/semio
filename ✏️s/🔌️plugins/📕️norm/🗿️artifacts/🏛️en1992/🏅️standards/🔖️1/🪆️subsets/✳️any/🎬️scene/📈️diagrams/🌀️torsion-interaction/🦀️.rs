//! 🌀️ Torsion and shear (EN 1992-1-1 6.3.2): the interaction of the governing torsion T_Ed and shear V_Ed with the two limits of T-04 (no calculated reinforcement, strut crushing) as one diagram of the (V, T) plane, with the required link and longitudinal steel of Eq. 6.28 in its table twin. The numbers are the catalogue entries T-02 and T-04 on the case of the member check, so picture and report cannot disagree.

use super::checks::{self, part_1_1};
use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::model::RcMember;
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

/// 🌀️ The governing torsion and shear of a member with the limits of T-04, in N and N·m.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interaction {
    pub t_ed: f64,
    pub v_ed: f64,
    pub t_rd_c: f64,
    pub t_rd_max: f64,
    pub v_rd_c: f64,
    pub v_rd_max: f64,
    pub util_min: f64,
    pub util_max: f64,
    pub asw_s_required: f64,
    pub asl_required: f64,
}

fn value(outcomes: &[checks::Outcome], key: &str) -> Option<f64> {
    outcomes.iter().find(|o| o.key == key).map(|o| o.capacity)
}

/// 🧮️ The interaction of the governing combinations of `m`; `None` without torsion, grades or section.
pub fn interaction(input: &SceneInput, m: &RcMember, sec: &checks::Sec) -> Option<Interaction> {
    let doc = input.document;
    let (concrete, steel) = (doc.concrete(&m.concrete_grade_id)?, doc.reinforcement(&m.reinforcement_grade_id)?);
    let geo = checks::Geo::of(sec, m);
    let effects = checks::combine_member_actions(m, &doc.action_factors());
    let ut = checks::governing_by(&effects, "uls", |e| e.t_ed.abs()).filter(|e| e.t_ed.abs() > 0.0)?;
    let uv = checks::governing_by(&effects, "uls", |e| e.v_ed.abs())?;
    let z = 0.9 * geo.d;
    let sigma_cp = if geo.area > 0.0 { (-uv.n_ed).max(0.0) / geo.area } else { 0.0 };
    let (_, cot) = part_1_1::shear_capacity_n(geo.b_w, z, concrete.f_ck, steel.f_yk, m.asw_per_s(), sigma_cp, uv.v_ed.abs(), doc.annex);
    let case = checks::torsion_case(m, sec, doc.annex, concrete.f_ck, steel.f_yk, geo.b_w, geo.d, geo.a_s, ut.t_ed, uv.v_ed, cot);
    let combined = checks::evaluate_outcomes("T-04", &case).ok()?;
    let links = checks::evaluate_outcomes("T-02", &case).ok()?;
    let util = |key: &str| combined.iter().find(|o| o.key == key).map(|o| o.utilization());
    Some(Interaction {
        t_ed: ut.t_ed.abs(),
        v_ed: uv.v_ed.abs(),
        t_rd_c: value(&combined, "t_rd_c")?,
        t_rd_max: value(&combined, "t_rd_max")?,
        v_rd_c: value(&combined, "v_rd_c")?,
        v_rd_max: value(&combined, "v_rd_max")?,
        util_min: util("util_min")?,
        util_max: util("util_max")?,
        asw_s_required: value(&links, "asw_s")?,
        asl_required: value(&links, "asl")?,
    })
}

/// 📈️ Diagram `torsion-interaction:<member>`: the two limit lines `T/T_Rd,c + V/V_Rd,c = 1` (dashed) and `T/T_Rd,max + V/V_Rd,max = 1` (solid) in the (V, T) plane with the design point; `None` for members without torsion.
pub fn diagram(input: &SceneInput, m: &RcMember, sec: &checks::Sec, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let i = interaction(input, m, sec)?;
    let binding = binding(input.report, &m.id, &["en1992.6.3.interaction."]);
    let tone = tone_of(&binding);
    let (kn, knm) = (1e-3, 1e-3);
    let min_line = [[0.0, i.t_rd_c * knm], [i.v_rd_c * kn, 0.0]];
    let max_line = [[0.0, i.t_rd_max * knm], [i.v_rd_max * kn, 0.0]];
    let mut g = base(
        m,
        "torsion-interaction",
        titled(m, "Torsion and shear", "Torsion und Querkraft"),
        loc("Interaction of torsion and shear (EN 1992-1-1 6.3.2): the design point against the linear limits T/T_Rd,c + V/V_Rd,c = 1 (no calculated reinforcement) and T/T_Rd,max + V/V_Rd,max = 1 (strut crushing, solid sections).", "Interaktion von Torsion und Querkraft (EN 1992-1-1 6.3.2): der Bemessungspunkt gegen die linearen Grenzen T/T_Rd,c + V/V_Rd,c = 1 (keine rechnerische Bewehrung) und T/T_Rd,max + V/V_Rd,max = 1 (Druckstrebenversagen, Vollquerschnitte)."),
        vec![axis("Shear force V", "Querkraft V", QuantityKind::Force, "kN"), axis("Torsion moment T", "Torsionsmoment T", QuantityKind::Moment, "kN·m")],
        kind,
        path,
    );
    g.series.push(Series::Polyline { id: "limit-max".into(), label: loc("Strut limit T_Rd,max, V_Rd,max", "Druckstrebengrenze T_Rd,max, V_Rd,max"), points: xy(&max_line), style: stroke(2.0, Dash::Solid, Tone::Info) });
    g.series.push(Series::Polyline { id: "limit-min".into(), label: loc("No reinforcement needed below T_Rd,c, V_Rd,c", "Keine Bewehrung erforderlich unter T_Rd,c, V_Rd,c"), points: xy(&min_line), style: stroke(1.5, Dash::Dashed, Tone::Warning) });
    g.series.push(Series::Scatter { id: "design-point".into(), label: loc("Design point", "Bemessungspunkt"), points: vec![ScatterPoint { at: Vec2::new(i.v_ed * kn, i.t_ed * knm), tone, label: Some(loc("VEd, TEd", "VEd, TEd")) }] });
    g.markers.push(Marker { id: "design-marker".into(), at: Vec2::new(i.v_ed * kn, i.t_ed * knm), shape: MarkerShape::Diamond, tone, label: loc(&format!("TEd/TRd,max + VEd/VRd,max = {:.2}", i.util_max), &format!("TEd/TRd,max + VEd/VRd,max = {:.2}", i.util_max)), utilisation: Some(i.util_max), bound_check: binding.map(|b| b.check_id), binding: None });
    let one = |en: &str, de: &str, unit: &str, v: f64| column(en, de, unit, vec![v]);
    g.table_twin = DataTable { columns: vec![
        one("Torsion T_Ed", "Torsion T_Ed", "kN·m", i.t_ed * knm),
        one("Shear V_Ed", "Querkraft V_Ed", "kN", i.v_ed * kn),
        one("T_Rd,c", "T_Rd,c", "kN·m", i.t_rd_c * knm),
        one("T_Rd,max", "T_Rd,max", "kN·m", i.t_rd_max * knm),
        one("V_Rd,c", "V_Rd,c", "kN", i.v_rd_c * kn),
        one("V_Rd,max", "V_Rd,max", "kN", i.v_rd_max * kn),
        one("Interaction without reinforcement", "Interaktion ohne Bewehrung", "1", i.util_min),
        one("Interaction at the strut limit", "Interaktion an der Druckstrebengrenze", "1", i.util_max),
        one("Required A_sw/s (one leg)", "Erforderliches A_sw/s (ein Schenkel)", "mm²/mm", i.asw_s_required),
        one("Required longitudinal steel ΣA_sl", "Erforderliche Längsbewehrung ΣA_sl", "mm²", i.asl_required * 1e6),
    ] };
    Some(g)
}
