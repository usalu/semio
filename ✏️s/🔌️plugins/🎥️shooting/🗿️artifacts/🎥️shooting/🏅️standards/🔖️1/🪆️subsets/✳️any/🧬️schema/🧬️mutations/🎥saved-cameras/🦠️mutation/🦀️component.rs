//! 🎥 Shooting mutation payloads — the `savedCameras` id-keyed collection's semantic verbs. Every
//! payload delegates its `diff`/`inverse` to the sibling `🔺️diff`/`↩️inverse` leaves.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingCamera, ShootingSavedCamera, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🌱️CreateSavedCamera
/// 🌱️ Brings a new [`ShootingSavedCamera`] into existence (append-only apply).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSavedCamera {
    pub saved_camera: ShootingSavedCamera,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "saved-camera", kind: "create-saved-camera", record: "CreatedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_create_saved_camera(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_create_saved_camera(self, base)
    }
    fn label(&self) -> String {
        format!("Create saved camera \"{}\"", self.saved_camera.label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.saved_camera.id.clone()]
    }
}
//#endregion 🌱️CreateSavedCamera

//#region 🗑️DeleteSavedCamera
/// 🗑️ Removes a saved camera by id; inverse recreates it via [`CreateSavedCamera`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSavedCamera {
    pub id: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DeleteSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "saved-camera", kind: "delete-saved-camera", record: "DeletedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_delete_saved_camera(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_delete_saved_camera(self, base)
    }
    fn label(&self) -> String {
        format!("Delete saved camera \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteSavedCamera

//#region ✏️RenameSavedCamera
/// ✏️ Changes a saved camera's identity `label` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameSavedCamera {
    pub id: String,
    pub new_label: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RenameSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "saved-camera", kind: "rename-saved-camera", record: "RenamedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_rename_saved_camera(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_rename_saved_camera(self, base)
    }
    fn label(&self) -> String {
        format!("Rename saved camera to \"{}\"", self.new_label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ✏️RenameSavedCamera

//#region 🎥️ReplaceSavedCameraView
/// 🎥️ Whole-value swap of a saved camera's `camera` pose — [`ShootingSavedCameraPatch::camera`](crate::artifacts::shooting::ShootingSavedCameraPatch)
/// overwrites rather than merges, so this is a `replace`, not a `change`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceSavedCameraView {
    pub id: String,
    pub new_camera: ShootingCamera,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReplaceSavedCameraView {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "saved-camera-view", kind: "replace-saved-camera-view", record: "ReplacedSavedCameraView" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_replace_saved_camera_view(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_replace_saved_camera_view(self, base)
    }
    fn label(&self) -> String {
        format!("Replace saved camera \"{}\" view", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🎥️ReplaceSavedCameraView

//#region 🔀️ReorderSavedCameras
/// 🔀️ Repositions a saved camera within the display-ordered `savedCameras` list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderSavedCameras {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReorderSavedCameras {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "saved-cameras", kind: "reorder-saved-cameras", record: "ReorderedSavedCameras" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_reorder_saved_cameras(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_reorder_saved_cameras(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder saved camera \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀️ReorderSavedCameras
