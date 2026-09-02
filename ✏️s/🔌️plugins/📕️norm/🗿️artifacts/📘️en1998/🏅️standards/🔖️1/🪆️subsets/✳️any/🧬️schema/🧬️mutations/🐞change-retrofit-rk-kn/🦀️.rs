//! 🐞 `change-retrofit-rk-kn` payload — changes the En1998 document's `retrofit_r_k_kn` (retrofit capacity R_k [kN]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_retrofit_r_k_kn::ChangeRetrofitRKKn;

//#region 🔖️ChangeRetrofitRKKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeRetrofitRKKn {
    pub new_retrofit_r_k_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeRetrofitRKKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "retrofit-rk-kn", kind: "change-retrofit-rk-kn", record: "ChangedRetrofitRKKn" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change retrofit capacity R_k [kN] to {}", self.new_retrofit_r_k_kn)
    }
}
//#endregion 🔖️ChangeRetrofitRKKn
