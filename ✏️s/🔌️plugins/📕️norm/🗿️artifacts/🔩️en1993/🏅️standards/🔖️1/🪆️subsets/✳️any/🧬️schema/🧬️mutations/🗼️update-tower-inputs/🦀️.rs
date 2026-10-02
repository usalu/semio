//! `upsert-tower-leg` — upsert a `TowerLeg` by id into `tower_legs`.

use crate::{TowerLeg, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateTowerInputs {
    pub tower_leg: TowerLeg,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateTowerInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "towerLeg", kind: "update-tower-inputs", record: "UpdatedTowerLeg" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update tower leg {}", self.tower_leg.id),
            &format!("Turmstiel {} aktualisieren", self.tower_leg.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.tower_leg.id.clone()]
    }
}
//#endregion 🔖️Payload
