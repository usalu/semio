//! ⚖️ EN 1997 compliance numeric tests + Python oracle parity + remedy laws.

use semio_s_plugin_norm_app_surface::{get_value_at_path, parse_path};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};
use semio_framework_value::ToValue;
use semio_s_artifact_norm_en1997_checks::{check_project, part_1, part_2, parse_design_approach, resolve_params, DesignApproach};
use semio_s_artifact_norm_en1997_inferences::outline::En1997Outline;
use semio_s_artifact_norm_en1997_model::{compliant_demo, noncompliant_demo};
use semio_s_artifact_norm_en1997_io::text::snapshot::decode_en1997_dsl;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use std::path::PathBuf;
use std::process::Command;

/// 🧾️ A verdict up to the registry basis: the check holds `expected` or, when it rests on an NDP row the registry does not mark verified (every EN 1997 row is still unverified), `UnverifiedBasis` with the same side of the limit and that unverified row cited.
fn assert_verdict_up_to_the_basis(check: &semio_s_plugin_norm_compliance::CheckResult, expected: CheckStatus) {
    if check.status == expected {
        return;
    }
    assert!(check.status == CheckStatus::UnverifiedBasis && matches!(expected, CheckStatus::Pass | CheckStatus::Warning), "{}: {:?} instead of {expected:?}", check.id, check.status);
    assert!(check.unverified_basis(), "{} is unverified without an unverified NDP: {:?}", check.id, check.ndps);
    assert_eq!(check.utilization <= 1.0 + 1e-9, expected == CheckStatus::Pass, "{}: utilisation {} contradicts {expected:?}", check.id, check.utilization);
}

/// 🧾️ The verdict law of a compliant document: no failing check, every utilisation within 1, and the report complies exactly when no check rests on an unverified NDP row.
fn assert_compliant_up_to_the_basis(report: &semio_s_plugin_norm_compliance::CheckReport) {
    let failing: Vec<_> = report.failing().map(|c| c.id.clone()).collect();
    assert!(failing.is_empty(), "failing checks: {failing:?}");
    let worst = report.checks.iter().filter(|c| c.status != CheckStatus::NotApplicable).max_by(|a, b| a.utilization.total_cmp(&b.utilization)).map(|c| (c.id.clone(), c.status, c.utilization));
    assert!(report.worst_utilization() <= 1.0 + 1e-9, "worst utilisation {} (highest check {worst:?})", report.worst_utilization());
    assert_eq!(report.complies(), report.unverified().next().is_none(), "complies exactly without an unverified basis");
    for check in report.unverified() {
        assert!(check.unverified_basis(), "{} is unverified without an unverified NDP: {:?}", check.id, check.ndps);
    }
}

/// 🧾️ True when the check is cleared up to the registry basis: `Pass`, or `UnverifiedBasis` within the limit.
fn clears_up_to_the_basis(check: &semio_s_plugin_norm_compliance::CheckResult) -> bool {
    check.status == CheckStatus::Pass || (check.status == CheckStatus::UnverifiedBasis && check.utilization <= 1.0 + 1e-9)
}

include!("🧮️design-approach-and-resistance-laws/🦀️.rs");

include!("🔮️oracle-and-remedy-laws/🦀️.rs");

include!("🌐️entity-check-laws/🦀️.rs");

#[test]
fn compliant_dsl_asset_complies() {
    let text = include_str!("../../../🖼️assets/🏗️compliant/🗣️.dsl.semio");
    let doc = decode_en1997_dsl(text).expect("decode compliant DSL");
    assert_compliant_up_to_the_basis(&check_project(&doc));
}

#[test]
fn noncompliant_dsl_asset_has_multiple_fails() {
    let text = include_str!("../../../🖼️assets/🚨noncompliant/🗣️.dsl.semio");
    let doc = decode_en1997_dsl(text).expect("decode noncompliant DSL");
    let n = check_project(&doc).failing().count();
    assert!(n >= 2, "noncompliant DSL fail_count={n}");
}

include!("🏷️field-meta-and-perturbation-laws/🦀️.rs");

include!("📇️catalogue-laws/🦀️.rs");

#[test]
fn annex_switch_reads_the_registry_and_changes_only_the_expected_values() {
    use semio_s_artifact_norm_en1997_checks::resolve_upl_params;
    let (en, de) = (resolve_upl_params(AnnexChoice::En, "persistent"), resolve_upl_params(AnnexChoice::De, "persistent"));
    assert_eq!((en.gamma_hyd, de.gamma_hyd), (1.35, 1.8));
    assert_eq!((en.gamma_g_stb, en.gamma_g_dst), (de.gamma_g_stb, de.gamma_g_dst));
    let (g_en, g_de) = (resolve_params(DesignApproach::Da2, AnnexChoice::En, "persistent"), resolve_params(DesignApproach::Da2, AnnexChoice::De, "persistent"));
    assert_eq!((g_en.gamma_g, g_en.gamma_q), (g_de.gamma_g, g_de.gamma_q));
    assert_eq!(resolve_params(DesignApproach::Da2, AnnexChoice::De, "transient").gamma_g, 1.2);
}
