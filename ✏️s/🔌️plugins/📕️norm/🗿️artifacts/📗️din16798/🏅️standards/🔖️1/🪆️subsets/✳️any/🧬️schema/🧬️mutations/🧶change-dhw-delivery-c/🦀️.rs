//! 🔧 `change-dhw-delivery-c` payload — changes the Din16798 document's `dhw_delivery_c` (DHW delivery temperature).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_dhw_delivery_c::ChangeDhwDeliveryC;

//#region 🔖️ChangeDhwDeliveryC
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeDhwDeliveryC {
    pub new_dhw_delivery_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDhwDeliveryC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "dhw-delivery-c", kind: "change-dhw-delivery-c", record: "ChangedDhwDeliveryC" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change DHW delivery temperature to {}", self.new_dhw_delivery_c)
    }
}
//#endregion 🔖️ChangeDhwDeliveryC
