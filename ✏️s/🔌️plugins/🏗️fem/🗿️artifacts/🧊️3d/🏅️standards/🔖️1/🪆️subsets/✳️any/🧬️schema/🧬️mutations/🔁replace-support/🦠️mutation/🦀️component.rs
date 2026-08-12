//! 🔁️ Fem3d mutation — `ReplaceSupport` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemSupport};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing support's payload (structured, no field-by-field editor
/// gesture exists for this entity — every real caller sets the whole record at once).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-support")]
pub struct ReplaceSupport {
    pub id: String,
    pub new_support: FemSupport,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceSupport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "support", kind: "replace-support", record: "ReplacedSupport" };

    fn diff(&self, base: &Fem3dSnapshot) -> crate::artifacts::fem3d::diff::Fem3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace support \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
