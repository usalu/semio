//! 📸 Shooting mutation payload — `CreateShot`. Brings a new shot into existence (append-only apply).

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::{ShootingShot, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CreateShot {
    pub shot: ShootingShot,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "shot", kind: "create-shot", record: "CreatedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create shot \"{}\"", self.shot.label), &format!("Aufnahme \"{}\" erstellen", self.shot.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot.id.clone()]
    }
}
