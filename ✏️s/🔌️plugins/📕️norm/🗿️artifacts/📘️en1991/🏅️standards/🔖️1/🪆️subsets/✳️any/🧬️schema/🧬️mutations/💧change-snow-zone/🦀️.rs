//! ❄️ `change-snow-zone` — sets the En1991 snow zone scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSnowZone {
    pub new_snow_zone: u8,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSnowZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "snow-zone", kind: "change-snow-zone", record: "ChangedSnowZone" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change snow zone to {:?}", self.new_snow_zone)
    }
}
//#endregion 🔖️Payload
