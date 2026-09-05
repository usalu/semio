//! 🌀️ `replace-points` — whole-value swap of the geometry playground's point cloud — the semantic
//! replacement for the old generic `SetGeometry`, used by gestures that load/paste an entire point
//! set (the app's `SetPoints` command) rather than editing one point.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationGeometry, EquationMutation, EquationPoint, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePoints {
    pub points: Vec<EquationPoint>,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for ReplacePoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "points", kind: "replace-points", record: "ReplacedPoints" };

    async fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &EquationSnapshot) -> Vec<EquationMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        "Replace points".into()
    }
    async fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into()]
    }
}
//#endregion 🔖️Payload
