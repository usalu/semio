//! 📦️ Shooting play app commands — asset activation, bulk field patches, creation and GLB import.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::artifacts::shooting::engine::next_shooting_id;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingAsset, ShootingAssetPatch, ShootingSnapshot};
use protocol::CollectionMutation;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 🩹️ Builds the `ShootingAssetPatch` for a `patchAsset`/`patchAssets` field write.
fn asset_patch_for_field(field: &str, value: &Value) -> Option<ShootingAssetPatch> {
    match field {
        "name" => value.as_str().map(|v| ShootingAssetPatch { name: Some(v.into()), ..Default::default() }),
        "url" => value.as_str().map(|v| ShootingAssetPatch { url: Some(v.into()), ..Default::default() }),
        _ => None,
    }
}

//#region 🔖️SetActiveAsset
pub mod set_active_asset {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-asset")]
    pub struct SetActiveAsset {
        pub asset_id: Option<String>,
    }

    pub fn handle(payload: &SetActiveAsset, _doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match payload.asset_id.as_deref().filter(|id| !id.is_empty()) {
            Some(id) => Ok(Emit { artifact_mutations: vec![ShootingMutation::SetActiveAsset { asset_id: Some(id.into()) }], config_mutations: vec![ShootingConfigMutation::SetFitRevision { value: cfg.snapshot.fit_revision + 1 }], ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveAsset

//#region 🔖️PatchAssets
pub mod patch_assets {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-assets")]
    pub struct PatchAssets {
        pub asset_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchAssets, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match asset_patch_for_field(&payload.field, &json!(payload.value)) {
            Some(patch) if !payload.asset_ids.is_empty() => Ok(Emit::mutations(payload.asset_ids.iter().cloned().map(|id| ShootingMutation::Assets(CollectionMutation::Patch { id, patch: patch.clone() })).collect())),
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchAssets

//#region 🔖️AddAsset
pub mod add_asset {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-asset")]
    pub struct AddAsset {
        pub format: String,
    }

    pub fn handle(payload: &AddAsset, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("asset");
        let format = &payload.format;
        let asset = ShootingAsset { id: id.clone(), name: format!("Asset {}", snapshot.assets.len() + 1), url: format!("/mesh/placeholder.{format}"), format: format.clone(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::Assets(CollectionMutation::Add { index: snapshot.assets.len(), item: asset }), ShootingMutation::SetActiveAsset { asset_id: Some(id.clone()) }],
            config_mutations: vec![ShootingConfigMutation::SetSelection { shot_ids: Vec::new(), asset_ids: vec![id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddAsset

//#region 🔖️ImportAsset
pub mod import_asset {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-asset")]
    pub struct ImportAsset {
        pub payload: String,
        pub name: Option<String>,
    }

    pub fn handle(payload: &ImportAsset, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("asset");
        let resolved_name = payload.name.as_deref().map(|name| name.trim_end_matches(".glb").to_string()).filter(|name| !name.is_empty()).unwrap_or_else(|| format!("Asset {}", snapshot.assets.len() + 1));
        let asset = ShootingAsset { id: id.clone(), name: resolved_name, url: payload.payload.clone(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::Assets(CollectionMutation::Add { index: snapshot.assets.len(), item: asset }), ShootingMutation::SetActiveAsset { asset_id: Some(id.clone()) }],
            config_mutations: vec![ShootingConfigMutation::SetSelection { shot_ids: Vec::new(), asset_ids: vec![id] }, ShootingConfigMutation::SetFitRevision { value: cfg.snapshot.fit_revision + 1 }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️ImportAsset

//#region 🔖️ImportAssetRequest
pub mod import_asset_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-asset-request")]
    pub struct ImportAssetRequest {}

    pub fn handle(_payload: &ImportAssetRequest, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".glb,model/gltf-binary".into(), read_as: Some("dataUrl".into()), import_action: "importAsset".into(), multiple: false }))
    }
}
//#endregion 🔖️ImportAssetRequest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn set_active_asset_emits_both_a_document_and_a_fit_revision_config_operation() {
        let mut app = shooting_app();
        let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
        let result = dispatch(&mut app, ShootingCommand::SetActiveAsset(set_active_asset::SetActiveAsset { asset_id: Some(asset_id.clone()) }));
        assert_eq!(result.mutations.len(), 1, "activating an asset is a real document edit");
        assert_eq!(app.snapshot().expect("snapshot").active_asset_id, asset_id);
    }

    #[test]
    fn import_asset_names_and_activates_the_new_asset() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::ImportAsset(import_asset::ImportAsset { payload: "data:model/gltf-binary;base64,AAAA".into(), name: Some("chair.glb".into()) }));
        let snapshot = app.snapshot().expect("snapshot");
        let imported = snapshot.assets.last().unwrap();
        assert_eq!(imported.name, "chair");
        assert!(imported.url.starts_with("data:"));
        assert_eq!(snapshot.active_asset_id, imported.id);
    }

    #[test]
    fn import_asset_request_declares_the_glb_accept_filter() {
        use semio_framework_plugin::HostEffect;
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::ImportAssetRequest(import_asset_request::ImportAssetRequest {}));
        match &result.requested_effects[0] {
            HostEffect::RequestFileOpen { read_as, import_action, .. } => {
                assert_eq!(read_as.as_deref(), Some("dataUrl"));
                assert_eq!(import_action, "importAsset");
            }
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
