use super::*;

#[semio_framework_async_macros::async_test]
async fn full_seismic_e2e() {
    let report = check_full_seismic(&En1998Snapshot::default());
    assert_eq!(report.checks.len(), 12);
}

#[semio_framework_async_macros::async_test]
async fn full_seismic_en_annex_e2e() {
    let document = En1998Snapshot { annex: "en".into(), ..En1998Snapshot::default() };
    let report = check_full_seismic(&document);
    assert_eq!(report.checks.len(), 12);
}
