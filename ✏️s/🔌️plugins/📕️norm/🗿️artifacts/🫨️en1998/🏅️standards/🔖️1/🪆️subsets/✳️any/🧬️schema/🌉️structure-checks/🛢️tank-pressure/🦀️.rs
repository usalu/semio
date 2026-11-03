//! 🛢️ Tank checks of EN 1998-4 Annex A: the hydrodynamic wall pressure with its impulsive and convective components against the hoop strength of the shell, and the sloshing wave height against the freeboard. See [`super`].

use super::*;
use semio_s_artifact_norm_en1998_schema::tank;
use semio_s_plugin_norm_compliance::ndp_number;

type Tank = semio_s_artifact_norm_en1998_model::En1998Tank;

const SLOSH: &str = "ndp.en1998.en4.tank.slosh-height-coefficient";

/// 🛢️ The wall pressure and the sloshing wave of one tank for the impulsive and convective accelerations `a_i`, `a_c` (m/s²) of the spectrum and the convective period `t_c`.
pub fn check_tank_hydrodynamics(report: &mut CheckReport, annex: AnnexChoice, t: &Tank, a_i: f64, a_c: f64, t_c: f64) {
    let path = id_sel("tanks", &t.id);
    let fluid_height = tank::fluid_height_m(t.height_m, t.filling_ratio);
    let fluid_mass = t.filling_ratio.clamp(0.0, 1.0) * t.content_qk_n / tank::GRAVITY;
    let rho = tank::density_kg_m3(fluid_mass, t.radius_m, fluid_height);
    let profile = tank::wall_profile(fluid_height, t.radius_m, rho, a_i, a_c, 40);
    if let Some(peak) = tank::governing(&profile) {
        let detail_en = format!("at z = {:.2} m: hydrostatic {:.0} Pa + SRSS(impulsive {:.0} Pa, convective {:.0} Pa) = {:.0} Pa (ρ = {rho:.0} kg/m³, a_i = {a_i:.3} m/s², a_c = {a_c:.3} m/s² at T_c = {t_c:.2} s)", peak.z_m, peak.hydrostatic_pa, peak.impulsive_pa, peak.convective_pa, peak.total_pa);
        let detail_de = format!("bei z = {:.2} m: hydrostatisch {:.0} Pa + SRSS(impulsiv {:.0} Pa, konvektiv {:.0} Pa) = {:.0} Pa (ρ = {rho:.0} kg/m³, a_i = {a_i:.3} m/s², a_c = {a_c:.3} m/s² bei T_c = {t_c:.2} s)", peak.z_m, peak.hydrostatic_pa, peak.impulsive_pa, peak.convective_pa, peak.total_pa);
        let base = CheckResult::assess(format!("en1998.4.{}.pressure", t.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "A.3.2.3"), SubjectRef::new(&t.id, format!("{path}.shellThicknessM"), lc(&t.id, &t.id)), lc("Hydrodynamic wall pressure and hoop stress", "Hydrodynamischer Wanddruck und Ringspannung")).annex(annex);
        if t.shell_thickness_m > 0.0 && t.shell_strength_mpa > 0.0 {
            let sigma = tank::hoop_stress_mpa(peak.total_pa, t.radius_m, t.shell_thickness_m);
            let mut c = base.utilization(q_press(sigma * 1.0e6), floor_limit(q_press(t.shell_strength_mpa * 1.0e6))).explanation(lc(&format!("{detail_en} — σ = p R / t = {sigma:.1} MPa against {:.1} MPa", t.shell_strength_mpa), &format!("{detail_de} — σ = p R / t = {sigma:.1} MPa gegen {:.1} MPa", t.shell_strength_mpa)));
            if sigma > t.shell_strength_mpa {
                let required = tank::required_thickness_m(peak.total_pa, t.radius_m, t.shell_strength_mpa);
                c = c.remedy(Remedy::at_least(SubjectRef::new(&t.id, format!("{path}.shellThicknessM"), lc("Shell thickness", "Schalendicke")), q_len(t.shell_thickness_m), q_len(required), lc(&format!("Increase the shell thickness to at least {:.1} mm.", required * 1000.0), &format!("Die Schalendicke auf mindestens {:.1} mm erhöhen.", required * 1000.0))));
            }
            report.push(c.build());
        } else {
            report.push(base.informative(q_press(peak.total_pa)).explanation(lc(&detail_en, &detail_de)).build());
        }
    }
    let slosh = tank::slosh_height_m(ndp_number(SLOSH, annex), t.radius_m, a_c);
    let freeboard = (t.height_m - fluid_height).max(0.0);
    let mut c = CheckResult::assess(format!("en1998.4.{}.slosh", t.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "A.3.3.2"), SubjectRef::new(&t.id, format!("{path}.fillingRatio"), lc(&t.id, &t.id)), lc("Sloshing wave height against the freeboard", "Schwappwellenhöhe gegen den Freibord"))
        .utilization(q_len(slosh), floor_limit(q_len(freeboard)))
        .annex(annex)
        .ndp(SLOSH)
        .explanation(lc(&format!("d_max = {:.2}·R·S_e(T_c)/g = {slosh:.3} m (T_c = {t_c:.2} s) against the freeboard {freeboard:.3} m", ndp_number(SLOSH, annex)), &format!("d_max = {:.2}·R·S_e(T_c)/g = {slosh:.3} m (T_c = {t_c:.2} s) gegen den Freibord {freeboard:.3} m", ndp_number(SLOSH, annex))));
    if slosh > freeboard {
        let limit = if t.height_m > 0.0 { ((t.height_m - slosh) / t.height_m).clamp(0.0, 1.0) } else { 0.0 };
        c = c.remedy(Remedy::at_most(SubjectRef::new(&t.id, format!("{path}.fillingRatio"), lc("Filling ratio", "Füllgrad")), q_dim(t.filling_ratio), q_dim(limit), lc(&format!("Lower the filling ratio to at most {limit:.3} so that the wave of {slosh:.2} m stays inside the tank."), &format!("Den Füllgrad auf höchstens {limit:.3} senken, damit die Welle von {slosh:.2} m im Tank bleibt."))));
    }
    report.push(c.build());
}
