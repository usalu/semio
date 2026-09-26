use crate::document::CheckStatus;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use crate::standards::v1::subsets::any::schema::snapshot::decode_din4108_dsl;

#[semio_framework_async_macros::async_test]
async fn failing_dsl_decodes_with_named_failures() {
    let text = include_str!("../../../../🖼️assets/🎬️failing-thin-insulation/🗣️.dsl.semio");
    let snap = decode_din4108_dsl(text).expect("failing dsl");
    let report = evaluate(&snap);
    assert!(!report.complies());
    let fails: Vec<_> = report.failing().map(|c| c.id.as_str()).collect();
    assert!(fails.len() >= 2, "fails={fails:?}");
    assert!(fails.iter().any(|id| id.contains("table3") && id.contains("wall-north")), "{fails:?}");
    assert!(fails.iter().any(|id| *id == "din4108-7.n50" || id.contains("summer") || id.contains("frsi")), "{fails:?}");
    let frsi = report.checks.iter().find(|c| c.id == "din4108-2.frsi.wall-north").expect("frsi");
    assert_eq!(frsi.status, CheckStatus::Fail);
}
