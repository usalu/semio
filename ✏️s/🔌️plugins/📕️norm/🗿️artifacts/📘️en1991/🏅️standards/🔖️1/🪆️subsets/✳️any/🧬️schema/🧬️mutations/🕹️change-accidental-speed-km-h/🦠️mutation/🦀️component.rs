//! 🚗 `change-accidental-speed-km-h` — sets the En1991 accidental impact speed scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAccidentalSpeedKmH {
    pub new_accidental_speed_km_h: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeAccidentalSpeedKmH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "accidental-speed-km-h", kind: "change-accidental-speed-km-h", record: "ChangedAccidentalSpeedKmH" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change accidental impact speed to {:?}", self.new_accidental_speed_km_h)
    }
}
//#endregion 🔖️Payload
