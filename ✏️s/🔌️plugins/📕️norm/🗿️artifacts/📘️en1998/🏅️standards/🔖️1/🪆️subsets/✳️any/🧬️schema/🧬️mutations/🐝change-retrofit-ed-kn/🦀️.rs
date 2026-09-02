//! 🐝 `change-retrofit-ed-kn` payload — changes the En1998 document's `retrofit_e_d_kn` (retrofit demand E_d [kN]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_retrofit_e_d_kn::ChangeRetrofitEDKn;

//#region 🔖️ChangeRetrofitEDKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeRetrofitEDKn {
    pub new_retrofit_e_d_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeRetrofitEDKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "retrofit-ed-kn", kind: "change-retrofit-ed-kn", record: "ChangedRetrofitEDKn" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change retrofit demand E_d [kN] to {}", self.new_retrofit_e_d_kn)
    }
}
//#endregion 🔖️ChangeRetrofitEDKn
