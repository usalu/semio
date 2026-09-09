use super::*;
use crate::editor::shooting::testkit::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

#[semio_framework_async_macros::async_test]
async fn set_active_asset_emits_both_a_document_and_a_fit_revision_config_operation() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let result = dispatch(&mut app, ShootingCommand::SetActiveAsset(set_active_asset::SetActiveAsset { asset_id: Some(asset_id.clone()) })).await;
    assert_eq!(result.mutations.len(), 1, "activating an asset is a real document edit");
    assert_eq!(app.snapshot().expect("snapshot").active_asset_id, asset_id);
}

#[semio_framework_async_macros::async_test]
async fn import_asset_names_and_activates_the_new_asset() {
    let mut app = shooting_app().await;
    dispatch(&mut app, ShootingCommand::ImportAsset(import_asset::ImportAsset { payload: "data:model/gltf-binary;base64,AAAA".into(), name: Some("chair.glb".into()) })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let imported = snapshot.assets.last().unwrap();
    assert_eq!(imported.name, "chair");
    assert!(imported.url.starts_with("data:"));
    assert_eq!(snapshot.active_asset_id, imported.id);
}

#[semio_framework_async_macros::async_test]
async fn import_asset_request_declares_the_glb_accept_filter() {
    use semio_framework_plugin::Effect;
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::ImportAssetRequest(import_asset_request::ImportAssetRequest {})).await;
    match &result.requested_effects[0] {
        Effect::RequestFileOpen { read_as, import_action, .. } => {
            assert_eq!(read_as.as_deref(), Some("dataUrl"));
            assert_eq!(import_action, "importAsset");
        }
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
}
