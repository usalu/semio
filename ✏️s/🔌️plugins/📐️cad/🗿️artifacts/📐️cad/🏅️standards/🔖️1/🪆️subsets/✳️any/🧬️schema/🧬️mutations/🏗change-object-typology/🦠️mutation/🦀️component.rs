//! 🏗️ CAD mutation — `ChangeObjectTypology` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏗️ Change typology of one object's `typology` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-typology")]
pub struct ChangeObjectTypology {
    pub pane: CadPaneId,
    pub object_id: String,
    pub new_typology: String,
}

impl MutationKind<CadSnapshot, CadMutation> for ChangeObjectTypology {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-typology", record: "ChangedObjectTypology" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change typology of object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
