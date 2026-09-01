//! 🔧️ `replace-geometry-parameters` — whole-value swap of a geometry's tuning parameter map,
//! addressed by id.


use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceGeometryParameters {
    pub id: String,
    pub new_parameters: BTreeMap<String, f64>,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ReplaceGeometryParameters {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "geometry-parameters", kind: "replace-geometry-parameters", record: "ReplacedGeometryParameters" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace parameters for geometry \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
