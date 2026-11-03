//! 📋️ Evaluation entry point of the EN 1990 compliance report: the member loop that gathers every check into one report. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::incremental_evaluation::Gate;

//#region 🔖️ComplianceReport
pub fn evaluate(document: &En1990Snapshot) -> CheckReport {
    evaluate_scoped(document, Scope::All)
}

/// 🛂️ What every member and document-analysis row needs from the document-wide gates: whether the inputs are valid and which members have broken or missing wiring. One gate belongs to the unit list of one document revision ([`En1990Units`] creates it), so a cold pass over `n` members evaluates the gates once, not `n` times.
pub struct RowGate {
    admitted: bool,
    broken: std::collections::BTreeSet<String>,
}

impl RowGate {
    pub fn of(document: &En1990Snapshot) -> Self {
        let mut sink = CheckReport::default();
        let admitted = push_input_validation(&mut sink, document);
        let broken = if admitted && !document.members.is_empty() { push_referential_integrity(&mut sink, document) } else { std::collections::BTreeSet::new() };
        Self { admitted, broken }
    }

    pub fn admitted(&self) -> bool {
        self.admitted
    }

    pub fn broken(&self) -> &std::collections::BTreeSet<String> {
        &self.broken
    }
}

/// 🎯️ The evaluation restricted to `scope` (every check outside the member loop, or one member row); the gates (input validation, empty members, broken wiring) are evaluated per call, so the ordered concatenation of the scopes is [`evaluate`]; a unit list shares them through [`evaluate_gated`].
pub fn evaluate_scoped(document: &En1990Snapshot, scope: Scope<'_>) -> CheckReport {
    evaluate_gated(document, scope, &Gate::new())
}

/// 🎯️ [`evaluate_scoped`] with the row gates taken from `gate` (computed by the first row that asks); the global scopes emit the gate checks themselves and never read it.
pub fn evaluate_gated(document: &En1990Snapshot, scope: Scope<'_>, gate: &Gate<RowGate>) -> CheckReport {
    let mut report = CheckReport::default();
    let globals = scope.globals();
    let admitted = if globals { push_input_validation(&mut report, document) } else { gate.get(|| RowGate::of(document)).admitted };
    if !admitted {
        return report;
    }
    if document.members.is_empty() {
        if globals {
            report.push(
                CheckResult::assess("en1990.members", "EN 1990", ClauseId::new("EN 1990", "§2", "members"), project_subject(document), copy("Members", "Bauteile"))
                .not_applicable(copy("No members / verification points defined.", "Keine Bauteile / Nachweisstellen definiert."))
                .annex(document.annex)
                .build(),
            );
        }
        push_document_analyses(&mut report, document, scope);
        return report;
    }
    let owned;
    let broken_members = if globals {
        owned = push_referential_integrity(&mut report, document);
        &owned
    } else {
        &gate.get(|| RowGate::of(document)).broken
    };
    if globals {
        push_rc_vs_cc(&mut report, document);
        push_design_working_life(&mut report, document);
        push_annex_b_dsl_il(&mut report, document);
        push_k_fi_applied(&mut report, document);
        for index in 0..document.factor_overrides.len() {
            push_factor_overrides(&mut report, document, index);
        }
        push_traffic_groups(&mut report, document);
        push_traffic_group_rows(&mut report, document);
        report.push(check_reliability_index_for(document.beta_computed, document.consequence_class, document.reliability_class.number(), document.reference_period_years, document.annex));
    }
    for (_, member) in document.members.iter().enumerate().filter(|(index, _)| scope.admits("members", *index)) {
        if broken_members.contains(&member.id) {
            push_member_unassessable(&mut report, document, member);
            continue;
        }
        let actions = member_actions(document, &member.id);
        if actions.iter().all(|action| action.value == 0.0) {
            push_member_without_actions(&mut report, document, member);
        } else {
            push_uls_str_geo(&mut report, document, member, &actions);
            push_geo_set_c(&mut report, document, member, &actions);
            push_traffic_group_combinations(&mut report, document, member, &actions);
            push_uls_equ(&mut report, document, member, &actions);
            push_accidental(&mut report, document, member, &actions);
            push_seismic(&mut report, document, member, &actions);
            push_sls_action_effects(&mut report, document, member, &actions);
            push_fat(&mut report, document, member, &actions);
            push_execution_situation(&mut report, document, member, &actions);
        }
        push_sls_deflection(&mut report, document, member);
        push_sls_vibration(&mut report, document, member);
        push_limit_basis(&mut report, document, member);
        push_a2_bridge_sls(&mut report, document, member);
        push_rail_sls(&mut report, document, member);
        push_frame_drift(&mut report, document, member);
        push_material_design(&mut report, document, member);
        push_geometry_design(&mut report, document, member);
        push_analysis_model(&mut report, document, member);
        push_fatigue_damage(&mut report, document, member);
    }
    push_document_analyses(&mut report, document, scope);
    report
}

/// 📊️ The reliability analyses, test series and design cases of the document, independent of the member loop.
fn push_document_analyses(report: &mut CheckReport, document: &En1990Snapshot, scope: Scope<'_>) {
    for (_, analysis) in document.reliability_analyses.iter().enumerate().filter(|(index, _)| scope.admits("reliabilityAnalyses", *index)) {
        push_reliability_analysis(report, document, analysis);
    }
    for (_, series) in document.test_series.iter().enumerate().filter(|(index, _)| scope.admits("testSeries", *index)) {
        push_test_series(report, document, series);
        push_test_standard(report, document, series);
    }
    for (_, case) in document.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(catalogue::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), document.annex));
    }
}
//#endregion 🔖️ComplianceReport
