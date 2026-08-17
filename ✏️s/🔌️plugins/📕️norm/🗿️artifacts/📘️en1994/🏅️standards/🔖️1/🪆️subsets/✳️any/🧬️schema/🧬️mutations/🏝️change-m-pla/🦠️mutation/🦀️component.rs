//! 🔩 `change-m-pla` — sets the En 1994 steel-section-only plastic moment M_pl,a [kNm] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMPla {
    pub new_m_pla: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeMPla {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "m-pla", kind: "change-m-pla", record: "ChangedMPla" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change steel plastic moment M_pl,a to {}", self.new_m_pla)
    }
}
//#endregion 🔖️Payload
