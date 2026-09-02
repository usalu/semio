//! 🔥 `update-fire-inputs` — atomically updates the fire-inputs facet (fire_thickness_mm, fire_rating, fire_massivity, fire_mu_0, fire_design_temperature_c are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateFireInputs {
    pub new_fire_thickness_mm: f64,
    pub new_fire_rating: String,
    pub new_fire_massivity: f64,
    pub new_fire_mu_0: f64,
    pub new_fire_design_temperature_c: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateFireInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "fire-inputs", kind: "update-fire-inputs", record: "UpdatedFireInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-1-2 fire resistance inputs".to_string()
    }
}
//#endregion 🔖️Payload
