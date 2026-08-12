//! 📎️ CAD mutation — `ReplaceReferences` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadReference, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📎️ Whole-value swap of one model definition's entire reference-overlay list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-references")]
pub struct ReplaceReferences {
    pub model_definition_id: String,
    #[dsl(table)]
    pub references: Vec<CadReference>,
}

impl MutationKind<CadSnapshot, CadMutation> for ReplaceReferences {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "references", kind: "replace-references", record: "ReplacedReferences" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace references for \"{}\"", self.model_definition_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone()]
    }
}
//#endregion 🔖️Mutation
