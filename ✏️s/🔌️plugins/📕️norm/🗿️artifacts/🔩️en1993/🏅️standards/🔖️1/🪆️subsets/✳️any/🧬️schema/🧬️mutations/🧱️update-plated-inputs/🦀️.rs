//! 🧱 `update-plated-inputs` — atomically updates the plated-inputs facet (plated_lambda_p, plated_sigma_ed_mpa are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdatePlatedInputs {
    pub new_plated_lambda_p: f64,
    pub new_plated_sigma_ed_mpa: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdatePlatedInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "plated-inputs", kind: "update-plated-inputs", record: "UpdatedPlatedInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-5 plated element buckling inputs".to_string()
    }
}
//#endregion 🔖️Payload
