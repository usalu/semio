//! 🚚️ CAD mutation — `MoveObjects` payload + `MutationKind` impl: the gumball-drag op.
//!
//! 🪆️ Object placement lives inside the pane's composed `s.stdio.semio.model` CHILD. This op does not
//! embed a child diff (`🔖️Composition`'s rule): it names the pane plus the exact next origin of each
//! touched object, and its diff re-mints that pane's content-addressed child HANDLE with the updated
//! `CadWorkingScene` attached as the handle's local materialization — the same shape `🌊️flow`'s
//! `move-widgets` uses for its own content child.

use crate::mutations::{CadMutation, CadObjectOrigin};
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "move-objects")]
pub struct MoveObjects {
    pub pane: CadPaneId,
    #[dsl(table)]
    pub placements: Vec<CadObjectOrigin>,
}

impl MutationKind<CadSnapshot, CadMutation> for MoveObjects {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "objects", kind: "move-objects", record: "MovedObjects" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        match self.placements.len() {
            1 => protocol::LocalizedLabel::native("Move 1 object", "1 Objekt verschieben"),
            count => protocol::LocalizedLabel::native(&format!("Move {count} objects"), &format!("{count} Objekte verschieben")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.placements.iter().map(|placement| placement.object_id.clone()).collect()
    }
}
//#endregion 🔖️Mutation
