//! 📏 `update-through-thickness-inputs` — atomically updates the through-thickness-inputs facet (t10_steel_subgrade, t10_actual_thickness_mm, t10_t_ed_c are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateThroughThicknessInputs {
    pub new_t10_steel_subgrade: String,
    pub new_t10_actual_thickness_mm: f64,
    pub new_t10_t_ed_c: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateThroughThicknessInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "through-thickness-inputs", kind: "update-through-thickness-inputs", record: "UpdatedThroughThicknessInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-10 through-thickness inputs".to_string()
    }
}
//#endregion 🔖️Payload
