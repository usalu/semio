//! 📦️ Shooting play app commands — asset activation, bulk field patches, creation and GLB import.

use crate::artifacts::shooting::mutations::change_asset_url::mutation::ChangeAssetUrl;
use crate::artifacts::shooting::mutations::create_asset::mutation::CreateAsset;
use crate::artifacts::shooting::mutations::rename_asset::mutation::RenameAsset;
use crate::artifacts::shooting::mutations::set_active_asset::mutation::SetActiveAsset as SetActiveAssetMutation;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::schema::next_shooting_id;
use crate::artifacts::shooting::{ShootingAsset, ShootingSnapshot};
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use serde_json::{json, Value};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🩹️ Builds the single-field `ShootingMutation` for a `patchAsset`/`patchAssets` field write,
/// addressed at `id`.
async fn asset_mutation_for_field(id: String, field: &str, value: &Value) -> Option<ShootingMutation> {
    match field {
        "name" => value.as_str().map(|v| ShootingMutation::RenameAsset(RenameAsset { id, new_name: v.into() })),
        "url" => value.as_str().map(|v| ShootingMutation::ChangeAssetUrl(ChangeAssetUrl { id, new_url: v.into() })),
        _ => None,
    }
}

//#region 🔖️SetActiveAsset
pub mod set_active_asset {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-asset")]
    pub struct SetActiveAsset {
        pub asset_id: Option<String>,
    }

    pub async fn handle(payload: &SetActiveAsset, _doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match payload.asset_id.as_deref().filter(|id| !id.is_empty()) {
            Some(id) => Ok(Emit {
                artifact_mutations: vec![ShootingMutation::SetActiveAsset(SetActiveAssetMutation { asset_id: Some(id.into()) })],
                config_mutations: vec![ShootingConfigMutation::SetFitRevision { value: cfg.snapshot.fit_revision + 1 }],
                ..Default::default()
            }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveAsset

//#region 🔖️PatchAssets
pub mod patch_assets {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "patch-assets")]
    pub struct PatchAssets {
        pub asset_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub async fn handle(payload: &PatchAssets, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        if payload.asset_ids.is_empty() {
            return Ok(Emit::default());
        }
        let value = json!(payload.value);
        let mutations: Vec<ShootingMutation> = payload.asset_ids.iter().cloned().filter_map(|id| asset_mutation_for_field(id, &payload.field, &value)).collect();
        if mutations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::mutations(mutations))
        }
    }
}
//#endregion 🔖️PatchAssets

//#region 🔖️AddAsset
pub mod add_asset {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "add-asset")]
    pub struct AddAsset {
        pub format: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new asset used to also select
    /// itself here — the `"assets"` domain's selection is framework-owned `InteractionState` now, only
    /// ever mutated by the framework's own injected `interactionSelect` handling, never by an app
    /// command's `Emit::config_mutations` (matches `raster`'s `add-layer` precedent).
    pub async fn handle(payload: &AddAsset, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("asset");
        let format = &payload.format;
        let asset =
            ShootingAsset { id: id.clone(), name: format!("Asset {}", snapshot.assets.len() + 1), url: format!("/mesh/placeholder.{format}"), format: format.clone(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
        Ok(Emit { artifact_mutations: vec![ShootingMutation::CreateAsset(CreateAsset { asset, index: Some(snapshot.assets.len()) }), ShootingMutation::SetActiveAsset(SetActiveAssetMutation { asset_id: Some(id) })], ..Default::default() })
    }
}
//#endregion 🔖️AddAsset

//#region 🔖️ImportAsset
pub mod import_asset {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "import-asset")]
    pub struct ImportAsset {
        pub payload: String,
        pub name: Option<String>,
    }

    /// 🕹️ Same dropped auto-select as `add_asset::handle` above (see its doc comment) — `fit_revision`
    /// still bumps here, that stays a genuinely app-owned config field.
    pub async fn handle(payload: &ImportAsset, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("asset");
        let resolved_name = payload.name.as_deref().map(|name| name.trim_end_matches(".glb").to_string()).filter(|name| !name.is_empty()).unwrap_or_else(|| format!("Asset {}", snapshot.assets.len() + 1));
        let asset = ShootingAsset { id: id.clone(), name: resolved_name, url: payload.payload.clone(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::CreateAsset(CreateAsset { asset, index: Some(snapshot.assets.len()) }), ShootingMutation::SetActiveAsset(SetActiveAssetMutation { asset_id: Some(id) })],
            config_mutations: vec![ShootingConfigMutation::SetFitRevision { value: cfg.snapshot.fit_revision + 1 }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️ImportAsset

//#region 🔖️ImportAssetRequest
pub mod import_asset_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "import-asset-request")]
    pub struct ImportAssetRequest {}

    pub async fn handle(_payload: &ImportAssetRequest, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(108), accept: ".glb,model/gltf-binary".into(), read_as: Some("dataUrl".into()), import_action: "importAsset".into(), multiple: false }))
    }
}
//#endregion 🔖️ImportAssetRequest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_active_asset_emits_both_a_document_and_a_fit_revision_config_operation() {
        let mut app = shooting_app();
        let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
        let result = dispatch(&mut app, ShootingCommand::SetActiveAsset(set_active_asset::SetActiveAsset { asset_id: Some(asset_id.clone()) }));
        assert_eq!(result.mutations.len(), 1, "activating an asset is a real document edit");
        assert_eq!(app.snapshot().expect("snapshot").active_asset_id, asset_id);
    }

    #[semio_framework_async_macros::async_test]
    async fn import_asset_names_and_activates_the_new_asset() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::ImportAsset(import_asset::ImportAsset { payload: "data:model/gltf-binary;base64,AAAA".into(), name: Some("chair.glb".into()) }));
        let snapshot = app.snapshot().expect("snapshot");
        let imported = snapshot.assets.last().unwrap();
        assert_eq!(imported.name, "chair");
        assert!(imported.url.starts_with("data:"));
        assert_eq!(snapshot.active_asset_id, imported.id);
    }

    #[semio_framework_async_macros::async_test]
    async fn import_asset_request_declares_the_glb_accept_filter() {
        use semio_framework_plugin::Effect;
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::ImportAssetRequest(import_asset_request::ImportAssetRequest {}));
        match &result.requested_effects[0] {
            Effect::RequestFileOpen { read_as, import_action, .. } => {
                assert_eq!(read_as.as_deref(), Some("dataUrl"));
                assert_eq!(import_action, "importAsset");
            }
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
