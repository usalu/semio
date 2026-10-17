#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn mesh_child_handle_addresses_its_own_target() {
    let text = runtime_fixture_text("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::GisTerrainSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let mesh = snapshot.mesh.expect("demo fixture carries its composed mesh");
    assert_eq!(mesh.child_id, mesh.target.artifact_id);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = runtime_fixture_text("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::GisTerrainSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::infer(&crate::GisTerrainSnapshot::default()).expect("valid materialized inference fixture"), crate::standards::v1::subsets::any::schema::inferences::GisTerrainInference::default(),);
}
//#endregion 🧪️InferenceLaws

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
