
use super::*;

#[semio_framework_async_macros::async_test]
async fn check_result_passes_when_utilization_below_one() {
    let clause = ClauseId::new("EN 1990", "§6.4", "6.10");
    let result = CheckResult::from_utilization(clause, Quantity::stress_mpa(250.0), Quantity::stress_mpa(300.0), "ULS stress check", AnnexChoice::De);
    assert_eq!(result.status, CheckStatus::Pass);
    assert!(result.utilization < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn table_lookup_linear_interpolates() {
    let table = [TableEntry1D { x: 0.0, y: 1.0 }, TableEntry1D { x: 10.0, y: 2.0 }];
    assert!((table_lookup_linear(&table, 5.0) - 1.5).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn check_minimum_passes_when_above_threshold() {
    let result = CheckResult::from_minimum(ClauseId::new("DIN 4108-3", "§6", "6.1"), Quantity::new(QuantityKind::Dimensionless, 0.8), Quantity::new(QuantityKind::Dimensionless, 0.25), "f_Rsi", AnnexChoice::De);
    assert_eq!(result.status, CheckStatus::Pass);
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslArtifact, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(id = "norm.demo", extension = "demo-norm", layout = "lines")]
struct DemoDocument {
    value: f64,
}

//#region 🔖️ArtifactCodec
// 🧬️ Was a fifth hand-written copy of the same ArtifactDsl/ArtifactPack envelope glue the real
// fifteen norm families duplicated in their own `HandcraftedArtifactCodecs` regions (W5a,
// 26/08/11/SEMIO-ARTIFACT-…) — now exercises `impl_norm_artifact_record!` instead, so this test
// fixture doubles as this module's own round-trip proof for the shared codec (see the
// `demo_document_*`/`document_text_round_trips_for_a_norm_family_document` tests below).
impl_norm_artifact_record!(DemoDocument, extension = "demo-norm", envelope_id = "norm.demo");
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn demo_document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&DemoDocument { value: 4.5 });
    store::os_store::test_support::assert_dsl_pack_equivalence(&DemoDocument { value: 4.5 });
}
