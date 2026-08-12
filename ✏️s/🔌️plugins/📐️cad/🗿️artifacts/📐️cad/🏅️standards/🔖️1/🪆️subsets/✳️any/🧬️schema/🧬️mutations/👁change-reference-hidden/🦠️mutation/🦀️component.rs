//! 👁️ CAD mutation — `ChangeReferenceHidden` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👁️ Change visibility of one reference overlay's `hidden` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-reference-hidden")]
pub struct ChangeReferenceHidden {
    pub model_definition_id: String,
    pub reference_id: String,
    pub new_hidden: bool,
}

impl MutationKind<CadSnapshot, CadMutation> for ChangeReferenceHidden {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "reference", kind: "change-reference-hidden", record: "ChangedReferenceHidden" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change visibility of reference \"{}\"", self.reference_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone(), self.reference_id.clone()]
    }
}
//#endregion 🔖️Mutation
