use semio_framework_plugin::Effect;
#[allow(unused_imports)]
use crate::{editor::shooting::config::ShootingConfig, editor::shooting::config::ShootingConfigMutation};
#[allow(unused_imports)]
use crate::editor::shooting::ui_helpers_and_io::ShootingDispatchCtx;
#[allow(unused_imports)]
use crate::mutations::change_asset_url::ChangeAssetUrl;
#[allow(unused_imports)]
use crate::mutations::create_asset::CreateAsset;
#[allow(unused_imports)]
use crate::mutations::rename_asset::RenameAsset;
#[allow(unused_imports)]
use crate::mutations::set_active_asset::SetActiveAsset as SetActiveAssetMutation;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
#[allow(unused_imports)]
use crate::next_shooting_id;
#[allow(unused_imports)]
use crate::{ShootingAsset, ShootingSnapshot};
#[allow(unused_imports)]
use semio_framework_pack_json::json;
#[allow(unused_imports)]
use semio_framework_pack_json::Value;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::shooting::commands::asset::*;
use crate::editor::shooting::unit_tests::context::{dispatch, shooting_app};
use crate::editor::shooting::ShootingCommand;

#[semio_framework_async_macros::async_test]
async fn set_active_asset_emits_both_a_document_and_a_fit_revision_config_operation() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let result = dispatch(&mut app, ShootingCommand::SetActiveAsset(set_active_asset::SetActiveAsset { asset_id: Some(asset_id.clone()) })).await;
    assert!(result.edited_document(), "activating an asset is a real document edit");
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
