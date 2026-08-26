//! 💥 `change-accidental-mass-t` — sets the En1991 accidental impact mass scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAccidentalMassT {
    pub new_accidental_mass_t: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeAccidentalMassT {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "accidental-mass-t", kind: "change-accidental-mass-t", record: "ChangedAccidentalMassT" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change accidental impact mass to {:?}", self.new_accidental_mass_t)
    }
}
//#endregion 🔖️Payload
