//! 🖼️ CAD mutation — `ReplacePaneObjects` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadObject, CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ Whole-value swap of one pane's entire object list — the derived-transformation write-back
/// gesture (`apply_transformation_mutations`) that replaces every object in a target pane at once.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-pane-objects")]
pub struct ReplacePaneObjects {
    pub pane: CadPaneId,
    #[dsl(table)]
    pub objects: Vec<CadObject>,
}

impl MutationKind<CadSnapshot, CadMutation> for ReplacePaneObjects {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "pane-objects", kind: "replace-pane-objects", record: "ReplacedPaneObjects" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace {:?} pane objects", self.pane)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
