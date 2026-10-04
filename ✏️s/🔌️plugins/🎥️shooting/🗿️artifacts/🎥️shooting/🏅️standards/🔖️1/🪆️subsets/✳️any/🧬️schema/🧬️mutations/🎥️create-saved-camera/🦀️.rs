//! 🎥 Shooting mutation payload — `CreateSavedCamera`. Brings a new saved camera into existence (append-only apply).

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::{ShootingSavedCamera, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CreateSavedCamera {
    pub saved_camera: ShootingSavedCamera,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "saved-camera", kind: "create-saved-camera", record: "CreatedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create saved camera \"{}\"", self.saved_camera.label), &format!("Gespeicherte Kamera \"{}\" erstellen", self.saved_camera.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.saved_camera.id.clone()]
    }
}
