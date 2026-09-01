//! 🌱️ Fem2d mutation — `CreateRegion` payload + `MutationKind` impl.

use crate::artifacts::fem2d::{Fem2dSnapshot, FemRegion};
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta};
use crate::artifacts::fem2d::mutations::{Fem2dMutation, delete_region};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemRegion`] meshed continuum region into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-region")]
pub struct CreateRegion {
    pub region: FemRegion,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateRegion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "region", kind: "create-region", record: "CreatedRegion" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create region \"{}\"", self.region.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.region.id.clone()]
    }
}
//#endregion 🔖️Mutation
