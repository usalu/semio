//! 🗼️ Tower checks of EN 1998-6 beyond the fundamental mode: the modal mass participation, the overturning moment of the CQC-combined modal response on the flexible foundation, and the period lengthening of the foundation flexibility. See [`super`].

use super::*;
use semio_s_artifact_norm_en1998_schema::tower;
use semio_s_plugin_norm_compliance::ndp_number;

type Tower = semio_s_artifact_norm_en1998_model::En1998Tower;

const PARTICIPATION: &str = "ndp.en1998.en1.modal.min-mass-participation";
const LENGTHENING: &str = "ndp.en1998.en6.interaction.max-period-lengthening";

/// 🗼️ The modal response of a tower with a given stiffness: participation, modal overturning moment against `M_Rd` and the foundation interaction.
pub fn check_tower_modal(report: &mut CheckReport, annex: AnnexChoice, t: &Tower, mass_kg: f64, ground: &Ground, q: f64) {
    if t.ei_nm2 <= 0.0 || t.height_m <= 0.0 {
        return;
    }
    let Some(modes) = tower::modes(mass_kg, t.height_m, t.ei_nm2, t.base_rotational_stiffness_nm) else { return };
    let response = tower::respond(&modes, &|period| ground.design(q, period));
    let path = id_sel("towers", &t.id);
    let periods = modes.modes.iter().map(|m| format!("{:.3}", m.period_s)).collect::<Vec<_>>().join(", ");
    let min_participation = ndp_number(PARTICIPATION, annex);
    let share = modes.participation().min(1.0);
    report.push(
        CheckResult::assess(format!("en1998.6.{}.modal.massParticipation", t.id), "DIN EN 1998-6", ClauseId::new("EN 1998-1", "1", "4.3.3.1"), SubjectRef::new(&t.id, format!("{path}.eiNm2"), lc(&t.id, &t.id)), lc("Effective modal mass of the tower modes", "Effektive Modalmasse der Turmeigenformen"))
            .minimum(q_dim(share), q_dim(min_participation))
            .annex(annex)
            .ndp(PARTICIPATION)
            .explanation(lc(&format!("{} modes with T = {periods} s carry {:.1} % of the mass, at least {:.0} % are required", modes.modes.len(), share * 100.0, min_participation * 100.0), &format!("{} Eigenformen mit T = {periods} s tragen {:.1} % der Masse, mindestens {:.0} % sind erforderlich", modes.modes.len(), share * 100.0, min_participation * 100.0)))
            .remedy(Remedy { applicable: false, ..Remedy::one_of(SubjectRef::new(&t.id, format!("{path}.eiNm2"), lc(&t.id, &t.id)), Vec::new(), lc("Change the flexural stiffness EI or the base rotational stiffness of the tower so that the included modes carry the required share of the mass, or refine the model with more modes.", "Biegesteifigkeit EI oder Fußdrehsteifigkeit des Turms so ändern, dass die berücksichtigten Eigenformen den geforderten Massenanteil tragen, oder das Modell mit mehr Eigenformen verfeinern.")) })
            .build(),
    );
    let mut overturning = CheckResult::assess(format!("en1998.6.{}.modal.overturning", t.id), "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.3.2"), SubjectRef::new(&t.id, format!("{path}.mRdNm"), lc(&t.id, &t.id)), lc("Overturning moment of the modal response (CQC)", "Kippmoment der modalen Antwort (CQC)"))
        .utilization(q_mom(response.overturning_nm), floor_limit(q_mom(t.m_rd_nm)))
        .annex(annex)
        .explanation(lc(&format!("V = {:.0} N, M_Ed = {:.0} N·m from {} modes (T = {periods} s, foundation stiffness {:.3e} N·m/rad), M_Rd = {:.0} N·m", response.base_shear_n, response.overturning_nm, modes.modes.len(), t.base_rotational_stiffness_nm, t.m_rd_nm), &format!("V = {:.0} N, M_Ed = {:.0} N·m aus {} Eigenformen (T = {periods} s, Gründungssteifigkeit {:.3e} N·m/rad), M_Rd = {:.0} N·m", response.base_shear_n, response.overturning_nm, modes.modes.len(), t.base_rotational_stiffness_nm, t.m_rd_nm)));
    if response.overturning_nm > t.m_rd_nm {
        overturning = overturning.remedy(Remedy::at_least(SubjectRef::new(&t.id, format!("{path}.mRdNm"), lc("M_Rd", "M_Rd")), q_mom(t.m_rd_nm), q_mom(response.overturning_nm), lc("Increase the overturning resistance to the modal moment.", "Den Kippwiderstand auf das modale Moment erhöhen.")));
    }
    report.push(overturning.build());
    if t.base_rotational_stiffness_nm > 0.0 {
        let limit = ndp_number(LENGTHENING, annex);
        let ratio = tower::period_lengthening(mass_kg, t.height_m, t.ei_nm2, t.base_rotational_stiffness_nm).unwrap_or(1.0);
        let mut interaction = CheckResult::assess(format!("en1998.6.{}.interaction", t.id), "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.2"), SubjectRef::new(&t.id, format!("{path}.baseRotationalStiffnessNm"), lc(&t.id, &t.id)), lc("Foundation flexibility: period lengthening T₁/T₁,fixed", "Gründungsnachgiebigkeit: Periodenverlängerung T₁/T₁,fest"))
            .utilization(q_dim(ratio), q_dim(limit))
            .annex(annex)
            .ndp(LENGTHENING)
            .explanation(lc(&format!("k_θ = {:.3e} N·m/rad lengthens the first period to {ratio:.3} times the fixed-base period, at most {limit:.2} is accepted", t.base_rotational_stiffness_nm), &format!("k_θ = {:.3e} N·m/rad verlängert die erste Periode auf das {ratio:.3}-fache der Einspannperiode, höchstens {limit:.2} wird akzeptiert", t.base_rotational_stiffness_nm)));
        if ratio > limit {
            if let Some(required) = tower::stiffness_for_lengthening(mass_kg, t.height_m, t.ei_nm2, limit) {
                interaction = interaction.remedy(Remedy::at_least(SubjectRef::new(&t.id, format!("{path}.baseRotationalStiffnessNm"), lc("Foundation rotational stiffness k_θ", "Drehfedersteifigkeit der Gründung k_θ")), q_mom(t.base_rotational_stiffness_nm), q_mom(required), lc(&format!("Stiffen the foundation to k_θ of at least {required:.3e} N·m/rad."), &format!("Die Gründung auf k_θ von mindestens {required:.3e} N·m/rad versteifen."))));
            }
        }
        report.push(interaction.build());
    }
}
