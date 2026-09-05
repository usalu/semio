//! 🔗 `update-tension-component-inputs` — atomically updates the tension-component-inputs facet (tension_component_f_uk_kn, tension_component_f_k_kn, tension_component_n_ed_kn are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateTensionComponentInputs {
    pub new_tension_component_f_uk_kn: f64,
    pub new_tension_component_f_k_kn: f64,
    pub new_tension_component_n_ed_kn: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateTensionComponentInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "tension-component-inputs", kind: "update-tension-component-inputs", record: "UpdatedTensionComponentInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-11 tension component inputs".to_string()
    }
}
//#endregion 🔖️Payload
