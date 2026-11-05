use super::*;

#[semio_framework_async_macros::async_test]
async fn create_lowpoly_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_lowpoly_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, LOWPOLY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<LowpolyViewer as ArtifactViewer>::DIALECT, LOWPOLY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_command_default_is_noop() {
    assert_eq!(LowpolyViewCommand::default(), LowpolyViewCommand::Noop);
}

#[semio_framework_async_macros::async_test]
async fn viewer_window_title_uses_the_bilingual_kind_label() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("locales.json")).expect("viewer locale contract");
    let layout = serde_json::to_value(view::layout()).expect("independent layout serializer");
    let definition = serde_json::to_value(model::definition()).expect("independent definition serializer");
    let observed = serde_json::json!({
        "windowTitleOverride": layout["root"]["children"][0].get("title").is_some(),
        "windowLabels": definition["label"]["native"]
    });
    assert_eq!(observed, contract["expect"]);
}
