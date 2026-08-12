//! 📸 Shooting mutation payloads — the `shots` id-keyed collection's semantic verbs. Every payload
//! delegates its `diff`/`inverse` to the sibling `🔺️diff`/`↩️inverse` leaves.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingShot, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🌱️CreateShot
/// 🌱️ Brings a new [`ShootingShot`] into existence (append-only apply, see [`super::super::assets::mutation::CreateAsset`]'s
/// doc for the same `index` convention).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateShot {
    pub shot: ShootingShot,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "shot", kind: "create-shot", record: "CreatedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_create_shot(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_create_shot(self, base)
    }
    fn label(&self) -> String {
        format!("Create shot \"{}\"", self.shot.label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot.id.clone()]
    }
}
//#endregion 🌱️CreateShot

//#region 🗑️DeleteShot
/// 🗑️ Removes a shot by id; inverse recreates it via [`CreateShot`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteShot {
    pub id: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DeleteShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "shot", kind: "delete-shot", record: "DeletedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_delete_shot(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_delete_shot(self, base)
    }
    fn label(&self) -> String {
        format!("Delete shot \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteShot

//#region ✏️RenameShot
/// ✏️ Changes a shot's identity `label` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameShot {
    pub id: String,
    pub new_label: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RenameShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "shot", kind: "rename-shot", record: "RenamedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_rename_shot(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_rename_shot(self, base)
    }
    fn label(&self) -> String {
        format!("Rename shot to \"{}\"", self.new_label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ✏️RenameShot

//#region 📐️ChangeShotWidth
/// 📐️ Sets a shot's render `width`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeShotWidth {
    pub id: String,
    pub new_width: u32,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeShotWidth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "shot-width", kind: "change-shot-width", record: "ChangedShotWidth" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_shot_width(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_shot_width(self, base)
    }
    fn label(&self) -> String {
        format!("Change shot \"{}\" width", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 📐️ChangeShotWidth

//#region 📐️ChangeShotHeight
/// 📐️ Sets a shot's render `height`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeShotHeight {
    pub id: String,
    pub new_height: u32,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeShotHeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "shot-height", kind: "change-shot-height", record: "ChangedShotHeight" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_shot_height(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_shot_height(self, base)
    }
    fn label(&self) -> String {
        format!("Change shot \"{}\" height", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 📐️ChangeShotHeight

//#region 🖼️ChangeShotFormat
/// 🖼️ Sets a shot's export `format`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeShotFormat {
    pub id: String,
    pub new_format: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeShotFormat {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "shot-format", kind: "change-shot-format", record: "ChangedShotFormat" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_shot_format(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_shot_format(self, base)
    }
    fn label(&self) -> String {
        format!("Change shot \"{}\" format", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🖼️ChangeShotFormat

//#region ✂️ChangeShotShape
/// ✂️ Sets a shot's crop `shape`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeShotShape {
    pub id: String,
    pub new_shape: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeShotShape {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "shot-shape", kind: "change-shot-shape", record: "ChangedShotShape" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_change_shot_shape(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_change_shot_shape(self, base)
    }
    fn label(&self) -> String {
        format!("Change shot \"{}\" shape", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ✂️ChangeShotShape

//#region 🔀️ReorderShots
/// 🔀️ Repositions a shot within the display-ordered `shots` list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderShots {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReorderShots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "shots", kind: "reorder-shots", record: "ReorderedShots" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_reorder_shots(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_reorder_shots(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder shot \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀️ReorderShots
