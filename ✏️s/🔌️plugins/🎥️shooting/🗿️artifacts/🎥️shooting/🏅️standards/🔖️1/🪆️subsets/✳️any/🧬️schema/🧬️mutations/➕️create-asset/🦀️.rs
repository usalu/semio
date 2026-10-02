//! 🌱 Shooting mutation payload — `CreateAsset`. Brings a new asset into existence. `index` is descriptive of authoring intent (the append-only apply always pushes onto the end of the list).

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::{ShootingAsset, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CreateAsset {
    pub asset: ShootingAsset,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "asset", kind: "create-asset", record: "CreatedAsset" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create asset \"{}\"", self.asset.name), &format!("Asset \"{}\" erstellen", self.asset.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.asset.id.clone()]
    }
}
