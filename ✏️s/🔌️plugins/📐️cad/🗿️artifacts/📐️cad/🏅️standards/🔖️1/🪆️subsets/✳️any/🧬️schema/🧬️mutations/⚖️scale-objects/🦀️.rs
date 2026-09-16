//! ⚖️ CAD mutation — `ScaleObjects` payload + `MutationKind` impl.
//!
//! 🪆️ Same re-materialization seam as `move-objects` (see that leaf's module doc).

use crate::mutations::{CadMutation, CadObjectScale};
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "scale-objects")]
pub struct ScaleObjects {
    pub pane: CadPaneId,
    #[dsl(table)]
    pub placements: Vec<CadObjectScale>,
}

impl MutationKind<CadSnapshot, CadMutation> for ScaleObjects {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "scale", entity: "objects", kind: "scale-objects", record: "ScaledObjects" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale {} object(s)", self.placements.len())
    }
    fn target(&self) -> Vec<String> {
        self.placements.iter().map(|placement| placement.object_id.clone()).collect()
    }
}
//#endregion 🔖️Mutation
