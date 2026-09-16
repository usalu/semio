//! ❌️ CAD mutation — `DeleteObject` payload + `MutationKind` impl.
//!
//! 🪆️ Removes one object from a pane's composed `s.stdio.semio.model` CHILD, re-minting that pane's
//! content-addressed child HANDLE from the shortened list (see `create-object`'s module doc).

use crate::mutations::CadMutation;
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-object")]
pub struct DeleteObject {
    pub pane: CadPaneId,
    pub object_id: String,
}

impl MutationKind<CadSnapshot, CadMutation> for DeleteObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "object", kind: "delete-object", record: "DeletedObject" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
