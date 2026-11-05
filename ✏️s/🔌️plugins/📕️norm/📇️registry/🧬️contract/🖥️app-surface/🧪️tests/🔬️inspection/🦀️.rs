//! 🔎️ Unit test of the inspection panel, which shares the detail rows of the checks window; the fixtures repeat those of the checks window tests.

use crate::*;
use semio_s_plugin_norm_compliance::{AnnexChoice, LocalizedCopy, SubjectRef};

fn check(id: &str, part: &str, section: &str, entity: &str, utilization: f64, status: CheckStatus) -> CheckResult {
    let mut builder = CheckResult::assess(id, part, ClauseId::new("EN 1993", "1-1", section), SubjectRef::new(entity, format!("members[{entity}].tM"), LocalizedCopy::new(format!("Member {entity}"), format!("Stab {entity}"))), LocalizedCopy::new(format!("check {id}"), format!("Nachweis {id}")))
        .utilization(Quantity::new(QuantityKind::Length, utilization * 0.0125), Quantity::new(QuantityKind::Length, 0.0125))
        .explanation(LocalizedCopy::new("explained", "erläutert"))
        .annex(AnnexChoice::De);
    if status == CheckStatus::Fail {
        builder = builder.remedy(Remedy::at_least(SubjectRef::new(entity, format!("members[{entity}].tM"), LocalizedCopy::new("t", "t")), Quantity::length_m(0.012), Quantity::length_m(0.022), LocalizedCopy::new("Increase thickness", "Dicke erhöhen")));
    }
    builder.status(status).build()
}

fn report() -> CheckReport {
    let mut report = CheckReport::default();
    report.push(check("a", "ULS", "6.2.9", "B1", 0.64, CheckStatus::Pass));
    report.push(check("b", "ULS", "6.2.10", "B2", 1.12, CheckStatus::Fail));
    report.push(check("c", "SLS", "7.2", "B1", 0.95, CheckStatus::Warning));
    report.push(check("d", "SLS", "7.3", "B3", 0.0, CheckStatus::NotApplicable));
    report
}

fn project(node: BuiltNode) -> String {
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("project the checks tree")
}

#[test]
fn inspection_shares_the_detail_rows_of_the_window() {
    let report = report();
    let json = project(render_inspection(&report, Some(1), Locale::De, None).expect("inspection"));
    for row in ["norm-inspection.check.limit", "norm-inspection.check.clause", "norm-inspection.check.remedy-0", "norm-inspection.check.status"] {
        assert!(json.contains(row), "{row}: {json}");
    }
    assert!(json.contains("Dicke erhöhen"));
}
