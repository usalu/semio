use crate::artifact_schema::evaluate_structure;
use crate::artifact_schema::snapshot::decode_en1995_dsl;
use crate::En1995Snapshot;

fn decoded() -> En1995Snapshot {
    decode_en1995_dsl(crate::glulam_footbridge::PRIMARY_TEXT).expect("glulam-footbridge DSL decodes")
}

#[semio_framework_async_macros::async_test]
async fn primary_asset_decodes_to_the_compliant_bridge() {
    assert_eq!(decoded(), En1995Snapshot::compliant_bridge());
}

#[semio_framework_async_macros::async_test]
async fn decoded_example_complies_with_every_bridge_check() {
    let snap = decoded();
    let report = evaluate_structure(snap.annex, &snap.members, &snap.connections);
    assert!(report.complies(), "fails: {:?}", report.failing().map(|c| c.id.clone()).collect::<Vec<_>>());
    for prefix in ["en1995.2.a.fatigue.bridge-G1", "en1995.2.b.avert.bridge-G1", "en1995.2.b.ahor.bridge-G1", "en1995.2.uls.bending.bridge-G1", "en1995.2.sls.deflection.bridge-G1"] {
        assert!(report.checks.iter().any(|c| c.id.starts_with(prefix)), "missing {prefix}");
    }
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifact_schema::inferences::En1995Inference;
    use protocol::Inference;
    let snapshot = decoded();
    assert_eq!(En1995Inference::infer(&snapshot), En1995Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifact_schema::inferences::En1995Inference;
    use protocol::Inference;
    assert_eq!(En1995Inference::infer(&En1995Snapshot::default()), En1995Inference::default());
}
