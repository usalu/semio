//! 📦 Shooting mutation payloads — the `assets` id-keyed collection's semantic verbs. Every payload
//! delegates its `diff`/`inverse` to the sibling `🔺️diff`/`↩️inverse` leaves (never inline logic here).

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingAsset, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🌱️CreateAsset
/// 🌱️ Brings a new [`ShootingAsset`] into existence. `index` is descriptive of authoring intent
/// (the append-only [`crate::artifacts::shooting::diff::ShootingAssetsDelta`] apply always pushes at
/// the end, matching the pre-migration `CollectionMutation::Add` behavior).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAsset {
    pub asset: ShootingAsset,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "asset", kind: "create-asset", record: "CreatedAsset" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_create_asset(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_create_asset(self, base)
    }
    fn label(&self) -> String {
        format!("Create asset \"{}\"", self.asset.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.asset.id.clone()]
    }
}
//#endregion 🌱️CreateAsset

//#region 🗑️DeleteAsset
/// 🗑️ Removes an asset by id; inverse recreates it (with its captured base position) via
/// [`CreateAsset`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteAsset {
    pub id: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DeleteAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "asset", kind: "delete-asset", record: "DeletedAsset" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_delete_asset(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_delete_asset(self, base)
    }
    fn label(&self) -> String {
        format!("Delete asset \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteAsset

//#region ✏️RenameAsset
/// ✏️ Changes an asset's identity `name` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameAsset {
    pub id: String,
    pub new_name: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RenameAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "asset", kind: "rename-asset", record: "RenamedAsset" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_rename_asset(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_rename_asset(self, base)
    }
    fn label(&self) -> String {
        format!("Rename asset to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ✏️RenameAsset

//#region 🔗️ChangeAssetUrl
/// 🔗️ Sets an asset's mesh `url`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAssetUrl {
    pub id: String,
    pub new_url: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeAssetUrl {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "asset-url", kind: "change-asset-url", record: "ChangedAssetUrl" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_asset_url(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_asset_url(self, base)
    }
    fn label(&self) -> String {
        format!("Change asset \"{}\" url", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔗️ChangeAssetUrl

//#region 🔀️ReorderAssets
/// 🔀️ Repositions an asset within the display-ordered `assets` list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderAssets {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReorderAssets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "assets", kind: "reorder-assets", record: "ReorderedAssets" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_reorder_assets(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_reorder_assets(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder asset \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀️ReorderAssets
