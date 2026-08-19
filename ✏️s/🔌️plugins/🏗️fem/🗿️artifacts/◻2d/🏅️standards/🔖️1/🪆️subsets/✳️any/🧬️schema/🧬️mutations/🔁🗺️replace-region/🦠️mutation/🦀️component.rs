//! 🔁️ Fem2d mutation — `ReplaceRegion` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemRegion};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing meshed region's payload (structured geometry + properties, no
/// field-by-field editor gesture exists for this entity — every real caller sets the whole record at
/// once).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-region")]
pub struct ReplaceRegion {
    pub id: String,
    pub new_region: FemRegion,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ReplaceRegion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "region", kind: "replace-region", record: "ReplacedRegion" };

    async fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace region \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
