//! 🏗️ `change-m-pl-rd` — sets the En 1994 full-interaction composite plastic moment resistance M_pl,Rd [kNm] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMPlRd {
    pub new_m_pl_rd: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeMPlRd {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "m-pl-rd", kind: "change-m-pl-rd", record: "ChangedMPlRd" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change plastic moment resistance M_pl,Rd to {}", self.new_m_pl_rd)
    }
}
//#endregion 🔖️Payload
