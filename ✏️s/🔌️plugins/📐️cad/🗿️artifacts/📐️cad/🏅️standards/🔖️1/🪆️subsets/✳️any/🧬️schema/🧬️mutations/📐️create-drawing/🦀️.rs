//! 📐️ `create-drawing` — appends a new owned `s.stdio.semio.drawing` CHILD handle to the cad
//! document's `drawings` collection (the "engineering assembly" `model, drawing` composition row —
//! design-full-plan.md §4). Empty today; this triad is the real, conforming lifecycle for the
//! forward-declared slot rather than leaving the facet's collection un-authorable.

use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-drawing")]
pub struct CreateDrawing {
    pub child_id: String,
    pub target: String,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateDrawing {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "drawing", kind: "create-drawing", record: "CreatedDrawing" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create drawing child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}
//#endregion 🔖️Mutation
