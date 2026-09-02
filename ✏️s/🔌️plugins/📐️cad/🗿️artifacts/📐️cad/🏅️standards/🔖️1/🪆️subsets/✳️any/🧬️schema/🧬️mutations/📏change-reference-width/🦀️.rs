//! 📏️ CAD mutation — `ChangeReferenceWidth` payload + `MutationKind` impl.

use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// 📏️ Change width of one reference overlay's `width_world` field.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-reference-width")]
pub struct ChangeReferenceWidth {
    pub model_definition_id: String,
    pub reference_id: String,
    pub new_width_world: f64,
}

impl MutationKind<CadSnapshot, CadMutation> for ChangeReferenceWidth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "reference", kind: "change-reference-width", record: "ChangedReferenceWidth" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change width of reference \"{}\"", self.reference_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone(), self.reference_id.clone()]
    }
}
//#endregion 🔖️Mutation
