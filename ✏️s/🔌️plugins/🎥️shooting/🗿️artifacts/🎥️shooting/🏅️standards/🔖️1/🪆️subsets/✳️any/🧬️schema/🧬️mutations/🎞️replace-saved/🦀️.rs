//! 🎞️ Shooting mutation payload — `ReplaceSavedCameraView`. Whole-value swap of a saved camera's `camera` pose — overwrites rather than merges, so this is a `replace`, not a `change`.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::{ShootingCamera, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReplaceSavedCameraView {
    pub id: String,
    pub new_camera: ShootingCamera,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReplaceSavedCameraView {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "saved-camera-view", kind: "replace-saved-camera-view", record: "ReplacedSavedCameraView" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace saved camera \"{}\" view", self.id), &format!("Ansicht von gespeicherter Kamera \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
