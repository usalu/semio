//! 🎯️ `move-point` — absolute spatial reposition of a point in the geometry playground's point
//! cloud, addressed by its BASE-state index.

use crate::artifacts::equation::{EquationMutation, EquationSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MovePoint {
    pub index: usize,
    pub x: f64,
    pub y: f64,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for MovePoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "point", kind: "move-point", record: "MovedPoint" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move point at {}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
