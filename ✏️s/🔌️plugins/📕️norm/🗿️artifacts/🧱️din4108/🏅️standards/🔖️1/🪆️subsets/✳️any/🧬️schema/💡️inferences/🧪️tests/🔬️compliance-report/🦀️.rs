use crate::document::CheckStatus;
use crate::standards::v1::subsets::any::schema::inferences::evaluate;
use crate::Din4108Snapshot;
use dsl::ToValue;

fn apply_numeric(snap: &mut Din4108Snapshot, path: &str, value: f64) {
    let mut root = ToValue::to_value(&*snap);
    crate::app_surface::set_value_at_path(&mut root, path, dsl::DslValue::float(value)).expect("set");
    *snap = dsl::FromValue::from_value(root).expect("decode");
}

#[semio_framework_async_macros::async_test]
async fn compliant_example_complies() {
    let report = evaluate(&Din4108Snapshot::compliant_etics_dwelling());
    assert!(report.complies(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    assert!(report.checks.len() >= 10);
}

#[semio_framework_async_macros::async_test]
async fn failing_example_has_failures_and_remedies() {
    let report = evaluate(&Din4108Snapshot::failing_thin_insulation());
    assert!(!report.complies());
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 2);
    for f in fails {
        assert!(!f.remedies.is_empty(), "no remedy on {}", f.id);
        assert!(f.remedies.iter().any(|r| r.applicable), "no applicable remedy on {}", f.id);
    }
    let frsi = report.checks.iter().find(|c| c.id == "din4108-2.frsi.wall-north").unwrap();
    assert_eq!(frsi.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_insulation_thickness_fixes_min_r() {
    let mut snap = Din4108Snapshot::failing_thin_insulation();
    let report = evaluate(&snap);
    let fail = report.failing().find(|c| c.id.contains("table3") && c.id.contains("wall-north")).expect("wall table3 fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable && r.target.path.contains("thicknessM")).expect("thickness remedy");
    assert!(remedy.target.path.contains("elements[id=wall-north]"), "{}", remedy.target.path);
    apply_numeric(&mut snap, &remedy.target.path, remedy.required.value.max(remedy.current.value));
    let after = evaluate(&snap);
    let again = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert_ne!(again.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_n50_fixes_airtightness() {
    let mut snap = Din4108Snapshot::failing_thin_insulation();
    let report = evaluate(&snap);
    let fail = report.failing().find(|c| c.id == "din4108-7.n50").expect("n50");
    let remedy = fail.remedies.iter().find(|r| r.applicable).expect("remedy");
    apply_numeric(&mut snap, &remedy.target.path, remedy.required.value);
    let after = evaluate(&snap);
    assert_eq!(after.checks.iter().find(|c| c.id == fail.id).unwrap().status, CheckStatus::Pass);
}
