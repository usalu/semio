//! 🧲️ Prestress in the analysis (EN 1992-1-1 5.10, catalogue entry A-16): the initial prestress `σ_pm0 = P_m0/A_p` against its limit `min(k7 f_pk, k8 f_p0.1k)` over the force `P_m0` as one diagram per prestressed member, the design point bound to the check `en1992.a-16.util.<member>` and dragging the stored force. The limit is the A-16 outcome on the case the entity check builds (NDPs from the registry), so picture and report cannot disagree; the tendon is toned by the same check in 3D. See [`super`].

use super::checks::{self, Case};
use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::model::RcMember;
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

const OVERSHOOT: f64 = 1.5;

/// 🧮️ `(σ_pm0,max in MPa, P_m0,max in N)` of A-16 for the member's tendon.
pub fn limits(input: &SceneInput, m: &RcMember) -> Option<(f64, f64)> {
    let ps = m.prestress.as_ref().filter(|p| p.area > 0.0)?;
    let steel = input.document.prestress_steels.iter().find(|s| s.id == m.prestress_steel_id)?;
    let case = Case::new(&m.id, input.document.annex).with_n("ap", ps.area * 1.0e6).with_n("fpk", steel.f_pk * 1.0e-6).with_n("fp01k", steel.f_p0_1k * 1.0e-6);
    let o = checks::evaluate_outcomes("A-16", &case).ok()?;
    let get = |key: &str| o.iter().find(|x| x.key == key).map(|x| x.capacity);
    Some((get("sigma_pm0_max")? * 1.0e-6, get("pm0_max")?))
}

/// 📈️ Diagram `prestress-limit:<member>`: `σ_pm0` over `P_m0` with the limit line and the design point; `None` for members without prestress.
pub fn diagram(input: &SceneInput, m: &RcMember, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let ps = m.prestress.as_ref().filter(|p| p.area > 0.0 && p.force > 0.0)?;
    let (limit, p_max) = limits(input, m)?;
    let ap = ps.area * 1.0e6;
    let reach = (OVERSHOOT * p_max).max(ps.force * 1.1);
    let check = input.report.checks.iter().find(|c| c.id == format!("en1992.a-16.util.{}", m.id));
    let tone = check.map_or(Tone::Info, |c| tone_of(&Some(projection_kit::binding_of_check(c))));
    let sigma = ps.force / ap;
    let mut g = base(
        m,
        "prestress-limit",
        titled(m, "Prestress limit", "Vorspanngrenze"),
        loc("Initial prestress σ_pm0 = P_m0/A_p against its limit min(k7 f_pk, k8 f_p0.1k) of EN 1992-1-1 5.10 (catalogue entry A-16); the diamond is the stored prestressing force and drags it.", "Anfangsvorspannung σ_pm0 = P_m0/A_p gegen die Grenze min(k7 f_pk, k8 f_p0,1k) nach EN 1992-1-1 5.10 (Katalogeintrag A-16); die Raute ist die gespeicherte Vorspannkraft und verschiebt sie."),
        vec![axis("Prestressing force P_m0", "Vorspannkraft P_m0", QuantityKind::Force, "kN"), axis("Steel stress σ_pm0", "Spannstahlspannung σ_pm0", QuantityKind::Stress, "MPa")],
        kind,
        path,
    );
    g.series.push(Series::Polyline { id: "sigma".into(), label: loc("σ_pm0 = P_m0/A_p", "σ_pm0 = P_m0/A_p"), points: xy(&[[0.0, 0.0], [reach * 1.0e-3, reach / ap]]), style: stroke(2.0, Dash::Solid, Tone::Info) });
    g.series.push(Series::Limit { id: "limit".into(), label: loc(&format!("Limit {limit:.0} MPa"), &format!("Grenze {limit:.0} MPa")), axis: AxisRef::Y, value: limit, style: stroke(1.5, Dash::Dashed, Tone::Warning) });
    let grip = FieldBinding { path: format!("{path}.prestress.force"), map: Affine { base: 0.0, k: 1.0e3 }, unit: "N".into(), quantise: Some(Quantise::Round { step: 1.0e3 }) };
    g.markers.push(Marker { id: "design".into(), at: Vec2::new(ps.force * 1.0e-3, sigma), shape: MarkerShape::Diamond, tone, label: loc(&format!("P_m0 = {:.0} kN, σ_pm0 = {sigma:.0} MPa", ps.force * 1.0e-3), &format!("P_m0 = {:.0} kN, σ_pm0 = {sigma:.0} MPa", ps.force * 1.0e-3)), utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite() && *u > 0.0), bound_check: check.map(|c| c.id.clone()), binding: Some(grip) });
    g.table_twin = DataTable { columns: vec![
        column("Prestressing force P_m0", "Vorspannkraft P_m0", "kN", vec![0.0, ps.force * 1.0e-3, p_max * 1.0e-3]),
        column("Steel stress σ_pm0", "Spannstahlspannung σ_pm0", "MPa", vec![0.0, sigma, limit]),
    ] };
    Some(g)
}
