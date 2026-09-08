//! 📦️ Shooting play app commands — asset activation, bulk field patches, creation and GLB import.

use crate::mutations::change_asset_url::ChangeAssetUrl;
use crate::mutations::create_asset::CreateAsset;
use crate::mutations::rename_asset::RenameAsset;
use crate::mutations::set_active_asset::SetActiveAsset as SetActiveAssetMutation;
use crate::op::ShootingMutation;
use crate::standards::v1::subsets::any::schema::next_shooting_id;
use crate::{ShootingAsset, ShootingSnapshot};
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use dsl::json;
use dsl::os_pack::json::Value;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🩹️ Builds the single-field `ShootingMutation` for a `patchAsset`/`patchAssets` field write,
/// addressed at `id`.
fn asset_mutation_for_field(id: String, field: &str, value: &Value) -> Option<ShootingMutation> {
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

    pub fn handle(payload: &SetActiveAsset, _doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match payload.asset_id.as_deref().filter(|id| !id.is_empty()) {
            Some(id) => Ok(Emit {
                artifact_mutations: vec![ShootingMutation::SetActiveAsset(SetActiveAssetMutation { asset_id: Some(id.into()) })],
                config_mutations: vec![ShootingConfigMutation::SetFitRevision(crate::editor::shooting::config::SetFitRevision { value: cfg.snapshot.fit_revision + 1 })],
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

    pub fn handle(payload: &PatchAssets, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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
    pub fn handle(payload: &AddAsset, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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
    pub fn handle(payload: &ImportAsset, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("asset");
        let resolved_name = payload.name.as_deref().map(|name| name.trim_end_matches(".glb").to_string()).filter(|name| !name.is_empty()).unwrap_or_else(|| format!("Asset {}", snapshot.assets.len() + 1));
        let asset = ShootingAsset { id: id.clone(), name: resolved_name, url: payload.payload.clone(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::CreateAsset(CreateAsset { asset, index: Some(snapshot.assets.len()) }), ShootingMutation::SetActiveAsset(SetActiveAssetMutation { asset_id: Some(id) })],
            config_mutations: vec![ShootingConfigMutation::SetFitRevision(crate::editor::shooting::config::SetFitRevision { value: cfg.snapshot.fit_revision + 1 })],
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

    pub fn handle(_payload: &ImportAssetRequest, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(108), accept: ".glb,model/gltf-binary".into(), read_as: Some("dataUrl".into()), import_action: "importAsset".into(), multiple: false }))
    }
}
//#endregion 🔖️ImportAssetRequest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
