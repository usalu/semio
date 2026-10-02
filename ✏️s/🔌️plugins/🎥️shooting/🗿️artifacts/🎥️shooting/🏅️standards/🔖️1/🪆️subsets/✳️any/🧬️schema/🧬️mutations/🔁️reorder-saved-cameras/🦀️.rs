//! 🔁 Shooting mutation payload — `ReorderSavedCameras`. Repositions a saved camera within the display-ordered `savedCameras` list.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReorderSavedCameras {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReorderSavedCameras {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "saved-cameras", kind: "reorder-saved-cameras", record: "ReorderedSavedCameras" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder saved camera \"{}\"", self.id), &format!("Reihenfolge von gespeicherter Kamera \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
