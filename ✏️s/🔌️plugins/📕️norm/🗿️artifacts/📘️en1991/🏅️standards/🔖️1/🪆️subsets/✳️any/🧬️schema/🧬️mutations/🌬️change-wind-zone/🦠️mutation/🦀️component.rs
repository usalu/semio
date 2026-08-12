//! 🌬️ `change-wind-zone` — sets the En1991 wind zone scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeWindZone {
    pub new_wind_zone: u8,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeWindZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "wind-zone", kind: "change-wind-zone", record: "ChangedWindZone" };

    fn diff(&self, base: &En1991Snapshot) -> <En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change wind zone to {:?}", self.new_wind_zone)
    }
}
//#endregion 🔖️Payload
