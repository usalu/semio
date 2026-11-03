//! 🏚️ Assessment checks of EN 1998-3 beyond the chord rotation: the cyclic shear verification of the brittle mechanism (AS-06, AS-07) and the knowledge level gating of the analysis method (AS-01, AS-08). See [`super::part_details`] and [`part_3_state`].

use super::*;
use semio_s_artifact_norm_en1998_schema::part_3_state::{self, AnalysisMethod};
use semio_s_plugin_norm_compliance::CheckStatus;

type Assessment = semio_s_artifact_norm_en1998_model::En1998Assessment;

fn known_methods(kl: semio_s_artifact_norm_en1998_model::KnowledgeLevel) -> String {
    AnalysisMethod::permitted_methods(kl).iter().map(|m| m.token()).collect::<Vec<_>>().join(", ")
}

/// 📅 Note of the limit state check on the NA-DE return period: under the German annex the registry row `ndp.en1998.en3.na.return-period` fixes T_R = 475 yr for every limit state (AS-10) and the capacity checks apply it; the row is unverified and names the NDP of clause 2.1 of the National Annex to EN 1998-3 that has to be read; empty under the EN annex.
pub fn na_return_period_note(annex: AnnexChoice, german: bool) -> String {
    let Some(period) = semio_s_plugin_norm_compliance::ndp_number_opt("ndp.en1998.en3.na.return-period", annex) else { return String::new() };
    if german {
        format!("; NA-DE legt T_R = {period:.0} a fest (a_gR nach Tabelle NA.1) und die Grenzzustandsnachweise wenden diese Periode an (UNGEPRÜFT: Registerzeile ohne gelesenen Text, zu lesen ist der NDP zu Abschnitt 2.1 des Nationalen Anhangs zu EN 1998-3)")
    } else {
        format!("; NA-DE fixes T_R = {period:.0} yr (a_gR of Table NA.1) and the limit state checks apply this period (UNVERIFIED: registry row without a read text, the NDP to read is clause 2.1 of the National Annex to EN 1998-3)")
    }
}

/// 🔧 Verifies the brittle shear mechanism of an assessed member, E_d = V_Ed ≤ R_d = V_R of the cyclic shear capacity with the strengths f_m/(CF γ_m) degraded by the plastic ductility demand (EN 1998-3 4.4 and A.3.3); silent while no shear demand or no section is given.
pub fn check_cyclic_shear(report: &mut CheckReport, annex: AnnexChoice, a: &Assessment) {
    if a.shear_force_demand_n <= 0.0 {
        return;
    }
    let Some(shear) = part_3_state::assessment_shear_capacity(a, annex) else { return };
    let basis = part_3_state::basis(a, annex);
    let path = id_sel("assessments", &a.id);
    let mut check = CheckResult::assess(
        format!("en1998.3.{}.cyclicShear", a.id),
        "DIN EN 1998-3",
        ClauseId::new("EN 1998-3", "3", "A.3.3"),
        SubjectRef::new(&a.id, format!("{path}.shearForceDemandN"), lc(&a.id, &a.id)),
        lc("Cyclic shear capacity of the brittle mechanism", "Zyklische Querkrafttragfähigkeit des spröden Mechanismus"),
    )
    .utilization(q_force(a.shear_force_demand_n), floor_limit(q_force(shear.resistance_n)))
    .annex(annex)
    .explanation(lc(
        &format!("E_d = {:.0} N against R_d = V_R = {:.0} N (axial {:.0} N, concrete {:.0} N, stirrups {:.0} N, degradation {:.3} at μ_pl {:.2}, x {:.3} m, CF {:.2}, γ_el {:.2}, brittle strengths f_m/(CF γ_m))", a.shear_force_demand_n, shear.resistance_n, shear.axial_n, shear.concrete_n, shear.stirrup_n, shear.degradation, shear.plastic_ductility, shear.neutral_axis_m, basis.cf, a.gamma_el),
        &format!("E_d = {:.0} N gegen R_d = V_R = {:.0} N (Normalkraft {:.0} N, Beton {:.0} N, Bügel {:.0} N, Abminderung {:.3} bei μ_pl {:.2}, x {:.3} m, CF {:.2}, γ_el {:.2}, spröde Festigkeiten f_m/(CF γ_m))", a.shear_force_demand_n, shear.resistance_n, shear.axial_n, shear.concrete_n, shear.stirrup_n, shear.degradation, shear.plastic_ductility, shear.neutral_axis_m, basis.cf, a.gamma_el),
    ));
    if a.shear_force_demand_n > shear.resistance_n {
        check = cyclic_shear_remedy(check, annex, a, &path);
    }
    let ndps = ["ndp.en1998.en3.shear.axial-limit", "ndp.en1998.en3.shear.ductility-cap", "ndp.en1998.en3.shear.ductility-slope", "ndp.en1998.en3.shear.base", "ndp.en1998.en3.shear.rho-min", "ndp.en1998.en3.shear.slenderness-slope", "ndp.en1998.en3.shear.slenderness-cap", "ndp.en1998.en3.chord-rotation.sd-factor", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"];
    report.push(check.ndps(&ndps).build());
}

fn cyclic_shear_remedy(check: semio_s_plugin_norm_compliance::CheckBuilder, annex: AnnexChoice, a: &Assessment, path: &str) -> semio_s_plugin_norm_compliance::CheckBuilder {
    let reach = |spacing: f64| part_3_state::assessment_shear_capacity(&Assessment { stirrup_spacing_m: spacing, ..a.clone() }, annex).map_or(0.0, |s| s.resistance_n);
    if a.stirrup_leg_area_m2 > 0.0 && a.stirrup_spacing_m > 0.02 && reach(0.02) >= a.shear_force_demand_n {
        let (mut high, mut low) = (a.stirrup_spacing_m, 0.02);
        for _ in 0..48 {
            let mid = 0.5 * (high + low);
            if reach(mid) >= a.shear_force_demand_n { low = mid } else { high = mid }
        }
        return check.remedy(Remedy::at_most(
            SubjectRef::new(&a.id, format!("{path}.stirrupSpacingM"), lc("Stirrup spacing s_h", "Bügelabstand s_h")),
            q_len(a.stirrup_spacing_m),
            q_len(low),
            lc("Reduce the stirrup spacing to raise the stirrup contribution V_w of the cyclic shear capacity.", "Bügelabstand verringern, um den Bügelanteil V_w der zyklischen Querkrafttragfähigkeit zu erhöhen."),
        ));
    }
    let capacity = reach(a.stirrup_spacing_m);
    check.remedy(Remedy::at_most(
        SubjectRef::new(&a.id, format!("{path}.shearForceDemandN"), lc("Shear force demand V_Ed", "Querkraftbeanspruchung V_Ed")),
        q_force(a.shear_force_demand_n),
        q_force(capacity),
        lc("Strengthen the member in shear (jacket, FRP) or reduce the shear demand, e.g. by limiting the flexural hinge capacity.", "Bauteil auf Querkraft verstärken (Mantel, FRP) oder die Querkraftbeanspruchung verringern, z. B. durch Begrenzung der Biegegelenk-Kapazität."),
    ))
}

/// 🧭 Whether the assessment of the supported building runs the pushover (N2) analysis: the building is found and every storey has a shear resistance.
pub fn pushover_used(document: &semio_s_artifact_norm_en1998_model::En1998Snapshot, a: &Assessment) -> bool {
    document.buildings.iter().find(|b| b.id == a.supported_building_id).is_some_and(|b| !b.storeys.is_empty() && b.storeys.iter().all(|s| s.shear_resistance_n > 0.0))
}

/// 🧭 The analysis method of an assessment: the typed input `analysisMethod`, or for `derived` the pushover (N2) when the supported building has a shear resistance in every storey and the lateral force method otherwise (EN 1998-3 4.3).
pub fn analysis_method(document: &semio_s_artifact_norm_en1998_model::En1998Snapshot, a: &Assessment) -> AnalysisMethod {
    use semio_s_artifact_norm_en1998_model::AnalysisMethodChoice as Choice;
    match a.analysis_method {
        Choice::Derived if pushover_used(document, a) => AnalysisMethod::NonlinearStatic,
        Choice::Derived | Choice::Lf => AnalysisMethod::LateralForce,
        Choice::Mrs => AnalysisMethod::ModalResponseSpectrum,
        Choice::Pushover => AnalysisMethod::NonlinearStatic,
        Choice::Nlth => AnalysisMethod::NonlinearTimeHistory,
    }
}

/// 🧭 Reports the analysis methods the assessment uses against those its knowledge level permits (EN 1998-3 Table 3.1): the lateral force capacity check is always permitted, the pushover (N2) check needs KL2 or KL3; a violation is a warning with the remedy to raise the knowledge level.
pub fn check_analysis_method(report: &mut CheckReport, annex: AnnexChoice, document: &semio_s_artifact_norm_en1998_model::En1998Snapshot, a: &Assessment) {
    let basis = part_3_state::basis(a, annex);
    let used = analysis_method(document, a);
    let ok = used.permitted_at(a.knowledge_level);
    let path = id_sel("assessments", &a.id);
    let kl = part_3_state::knowledge_level_token(a.knowledge_level);
    let mut check = CheckResult::assess(
        format!("en1998.3.{}.analysisMethod", a.id),
        "DIN EN 1998-3",
        ClauseId::new("EN 1998-3", "3", "4.3"),
        SubjectRef::new(&a.id, format!("{path}.analysisMethod"), lc(&a.id, &a.id)),
        lc("Analysis method permitted by the knowledge level", "Vom Kenntnisstand erlaubtes Berechnungsverfahren"),
    )
    .status(if ok { CheckStatus::Pass } else { CheckStatus::Warning })
    .annex(annex)
    .explanation(lc(
        &format!("{} analysis with knowledge level {kl} (CF {:.2}, applied {:.2}); permitted: {}", used.token(), basis.cf_table, basis.cf, known_methods(a.knowledge_level)),
        &format!("Berechnung {} mit Kenntnisstand {kl} (CF {:.2}, angesetzt {:.2}); erlaubt: {}", used.token(), basis.cf_table, basis.cf, known_methods(a.knowledge_level)),
    ));
    if !ok {
        check = check.remedy(Remedy::one_of(
            SubjectRef::new(&a.id, format!("{path}.knowledgeLevel"), lc("Knowledge level", "Kenntnisstand")),
            vec!["kl2".into(), "kl3".into()],
            lc("Raise the knowledge level to KL2 or KL3 (extended or comprehensive investigation) or restrict the assessment to the lateral force and modal response spectrum methods.", "Kenntnisstand auf KL2 oder KL3 anheben (erweiterte oder umfassende Untersuchung) oder die Bewertung auf Ersatzkraft- und Antwortspektrenverfahren beschränken."),
        ));
        check = check.remedy(Remedy::one_of(
            SubjectRef::new(&a.id, format!("{path}.analysisMethod"), lc("Analysis method", "Berechnungsverfahren")),
            vec!["lf".into(), "mrs".into()],
            lc("Choose the lateral force or the modal response spectrum method, the methods KL1 permits.", "Ersatzkraft- oder modales Antwortspektrenverfahren wählen, die Verfahren, die KL1 erlaubt."),
        ));
    }
    report.push(check.build());
}
