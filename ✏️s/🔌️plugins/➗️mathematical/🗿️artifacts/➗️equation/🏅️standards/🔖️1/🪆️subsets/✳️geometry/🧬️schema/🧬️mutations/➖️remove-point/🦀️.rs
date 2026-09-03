//! ➖️ `remove-point` — takes a point out of the geometry playground's point cloud. `index` is
//! BASE-state, per the addressing convention for index-keyed collections.

use crate::artifacts::equation::standards::v1::subsets::geometry::schema::mutations::insert_point;
use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemovePoint {
    pub index: usize,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for RemovePoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "point", kind: "remove-point", record: "RemovedPoint" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Remove point at {}", self.index)
    }
    async fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
