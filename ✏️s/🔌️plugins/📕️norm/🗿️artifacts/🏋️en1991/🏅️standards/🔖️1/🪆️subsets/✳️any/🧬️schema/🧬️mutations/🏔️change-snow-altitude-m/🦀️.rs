//! ⛰️ `change-snow-altitude-m` — sets the En1991 snow altitude scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSnowAltitudeM {
    pub new_snow_altitude_m: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSnowAltitudeM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "snow-altitude-m", kind: "change-snow-altitude-m", record: "ChangedSnowAltitudeM" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change snow altitude to {:?}", self.new_snow_altitude_m)
    }
}
//#endregion 🔖️Payload
