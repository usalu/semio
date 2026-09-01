//! 🦋 `change-retrofit-gamma-el` payload — changes the En1998 document's `retrofit_gamma_el` (retrofit confidence factor gamma_el).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_retrofit_gamma_el::ChangeRetrofitGammaEl;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeRetrofitGammaEl
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRetrofitGammaEl {
    pub new_retrofit_gamma_el: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeRetrofitGammaEl {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "retrofit-gamma-el", kind: "change-retrofit-gamma-el", record: "ChangedRetrofitGammaEl" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change retrofit confidence factor gamma_el to {}", self.new_retrofit_gamma_el)
    }
}
//#endregion 🔖️ChangeRetrofitGammaEl
