//! 💧 `change-exposure` payload — changes the En1996 document's `exposure` (durability exposure class).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_exposure::ChangeExposure;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeExposure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeExposure {
    pub new_exposure: crate::artifacts::en1996::part_2::ExposureClass,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeExposure {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "exposure", kind: "change-exposure", record: "ChangedExposure" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change durability exposure class to {:?}", self.new_exposure)
    }
}
//#endregion 🔖️ChangeExposure
