//! 🚿 `change-settlement-limit-mm` payload — changes the En1997 document's `settlement_limit_mm` (settlement limit [mm]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_settlement_limit_mm::ChangeSettlementLimitMm;

//#region 🔖️ChangeSettlementLimitMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSettlementLimitMm {
    pub new_settlement_limit_mm: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeSettlementLimitMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "settlement-limit-mm", kind: "change-settlement-limit-mm", record: "ChangedSettlementLimitMm" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change settlement limit [mm] to {}", self.new_settlement_limit_mm)
    }
}
//#endregion 🔖️ChangeSettlementLimitMm
