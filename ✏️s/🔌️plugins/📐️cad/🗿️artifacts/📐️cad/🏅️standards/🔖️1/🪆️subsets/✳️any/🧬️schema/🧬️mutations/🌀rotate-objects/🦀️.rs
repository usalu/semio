//! 🌀️ CAD mutation — `RotateObjects` payload + `MutationKind` impl: the gumball-rotate op.
//!
//! 🪆️ Same re-materialization seam as `move-objects` (see that leaf's module doc): the pane's
//! composed `s.stdio.semio.model` child HANDLE is re-minted from the rotated object list.

use crate::mutations::{CadMutation, CadObjectOrientation};
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rotate-objects")]
pub struct RotateObjects {
    pub pane: CadPaneId,
    #[dsl(table)]
    pub placements: Vec<CadObjectOrientation>,
}

impl MutationKind<CadSnapshot, CadMutation> for RotateObjects {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "objects", kind: "rotate-objects", record: "RotatedObjects" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Rotate {} object(s)", self.placements.len()), &format!("{} Objekt(s) drehen", self.placements.len()))
    }
    fn target(&self) -> Vec<String> {
        self.placements.iter().map(|placement| placement.object_id.clone()).collect()
    }
}
//#endregion 🔖️Mutation
