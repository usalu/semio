//! 🗑️ Fem2d mutation — `DeleteMaterial` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ Removes an existing material by id, capturing nothing itself (the removed payload is recovered
/// from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-material")]
pub struct DeleteMaterial {
    pub id: String,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for DeleteMaterial {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "material", kind: "delete-material", record: "DeletedMaterial" };

    fn diff(&self, base: &Fem2dSnapshot) -> crate::artifacts::fem2d::diff::Fem2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete material \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
