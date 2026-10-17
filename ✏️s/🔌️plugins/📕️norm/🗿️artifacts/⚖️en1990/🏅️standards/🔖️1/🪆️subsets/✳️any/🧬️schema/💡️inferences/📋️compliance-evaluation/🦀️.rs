//! 📋️ Evaluation entry point of the EN 1990 compliance report: the member loop that gathers every check into one report. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub fn evaluate(document: &En1990Snapshot) -> CheckReport {
    let annex: NationalAnnexes = if document.annex == AnnexChoice::De { NaDe.into() } else { NaEn.into() };
    let mut report = CheckReport::default();
    if document.members.is_empty() {
        report.push(
            CheckResult::assess("en1990.members", "EN 1990", ClauseId::new("EN 1990", "§2", "members"), project_subject(document), copy("Members", "Bauteile"))
                .not_applicable(copy("No members / verification points defined.", "Keine Bauteile / Nachweisstellen definiert."))
                .annex(document.annex)
                .build(),
        );
        return report;
    }
    let broken_members = push_referential_integrity(&mut report, document);
    push_k_fi(&mut report, document);
    push_design_working_life(&mut report, document);
    push_annex_b_dsl_il(&mut report, document);
    report.push(check_reliability_index_for(
        document.beta_computed,
        document.consequence_class,
        document.reliability_class,
        document.reference_period_years,
        document.annex,
    ));
    for member in &document.members {
        if broken_members.contains(&member.id) {
            push_member_unassessable(&mut report, document, member);
            continue;
        }
        let actions = action_set_for_member(document, &member.id);
        let n_leading = actions.q_k.len().max(1);
        for leading in 0..n_leading {
            if actions.q_k.is_empty() && leading > 0 {
                break;
            }
            push_uls_str_geo(&mut report, &annex, document, member, &actions, leading);
            push_uls_geo(&mut report, &annex, document, member, &actions, leading);
            push_geo_set_c(&mut report, &annex, document, member, &actions, leading);
            push_uls_equ(&mut report, &annex, document, member, &actions, leading);
            push_uls_equ_stab(&mut report, &annex, document, member, &actions, leading);
            push_sls_action_effects(&mut report, &annex, document, member, &actions, leading);
            push_accidental(&mut report, &annex, document, member, &actions, leading);
        }
        push_seismic(&mut report, &annex, document, member, &actions);
        push_sls_deflection(&mut report, document, member);
        push_sls_vibration(&mut report, document, member);
        push_fat(&mut report, &annex, document, member, &actions);
        push_a2_bridge_sls(&mut report, document, member);
    }
    report
}
//#endregion 🔖️ComplianceReport
