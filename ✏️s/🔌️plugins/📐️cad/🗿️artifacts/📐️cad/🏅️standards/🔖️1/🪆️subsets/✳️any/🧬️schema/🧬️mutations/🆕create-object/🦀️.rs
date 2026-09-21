//! 🆕️ CAD mutation — `CreateObject` payload + `MutationKind` impl.
//!
//! 🪆️ Brings one object into a pane's composed `s.stdio.semio.model` CHILD. Like `move-objects` this
//! is not a child diff: the op names the pane, the insertion index and the object's own authored
//! fields, and its diff re-mints the pane's content-addressed child HANDLE with the extended
//! `CadWorkingScene` attached as the handle's local materialization. `index` is carried (rather than
//! always appending) so `delete-object`'s inverse restores the removed object at its original slot —
//! the child id is content-addressed over the ORDERED element list, so order is identity.

use crate::mutations::{CadMutation, CadObjectPrimitive, CadObjectSpec};
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-object")]
pub struct CreateObject {
    pub pane: CadPaneId,
    pub index: u32,
    #[dsl(block)]
    pub object: CadObjectSpec,
    #[dsl(table)]
    pub primitives: Vec<CadObjectPrimitive>,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "object", kind: "create-object", record: "CreatedObject" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create object \"{}\"", self.object.label), &format!("Objekt \"{}\" erstellen", self.object.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object.id.clone()]
    }
}
//#endregion 🔖️Mutation
