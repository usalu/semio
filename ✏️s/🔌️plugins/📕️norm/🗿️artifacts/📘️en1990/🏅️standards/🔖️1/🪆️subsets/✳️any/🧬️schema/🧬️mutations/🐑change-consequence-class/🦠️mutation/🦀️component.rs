//! 🎯 `change-consequence-class` — sets the EN 1990 document's consequence class (CC1/CC2/CC3),
//! which drives the target reliability index used by `check_reliability_index`.

use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeConsequenceClass {
    pub new_consequence_class: u8,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeConsequenceClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "consequence-class", kind: "change-consequence-class", record: "ChangedConsequenceClass" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change consequence class to CC{}", self.new_consequence_class)
    }
}
//#endregion 🔖️Payload
