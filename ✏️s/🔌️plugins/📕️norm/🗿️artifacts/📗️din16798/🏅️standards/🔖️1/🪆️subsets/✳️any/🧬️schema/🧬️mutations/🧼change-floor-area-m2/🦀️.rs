//! 🔧 `change-floor-area-m2` payload — changes the Din16798 document's `floor_area_m2` (floor area).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_floor_area_m2::ChangeFloorAreaM2;

//#region 🔖️ChangeFloorAreaM2
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFloorAreaM2 {
    pub new_floor_area_m2: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeFloorAreaM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "floor-area-m2", kind: "change-floor-area-m2", record: "ChangedFloorAreaM2" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change floor area to {}", self.new_floor_area_m2)
    }
}
//#endregion 🔖️ChangeFloorAreaM2
