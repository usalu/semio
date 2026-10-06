//! ⚡️ Wires parent mutation vocabulary — empty by design (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12,
//! §20.15): the board lives in the composed `content` child (`s.stdio.semio@v1/graph`), so every board edit is a child-lane
//! graph leaf in that child's store (`create-node`, `drag-nodes`, `create-edge`, `set-node-property`, …) and the parent owns no
//! leaf that could read the child. Editors publish those leaves through `crate::wires_child_emit` and the tool machines.

use crate::schema::diff::WiresDiff;
use crate::WiresSnapshot;

//#region 🔖️Aggregate
/// 🕳️ The uninhabited parent vocabulary of a document whose whole board is its composed child.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum WiresMutation {}

impl protocol::Mutation<WiresSnapshot> for WiresMutation {
    type Diff = WiresDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match *self {}
    }
    fn diff(&self, _base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
        match *self {}
    }
    fn inverse(&self, _base: &WiresSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        match *self {}
    }
}

/// 🏷️ No parent kind exists, so no parent operation is ever labelled; graph leaves label their own rows.
impl protocol::SemanticMutation<WiresSnapshot> for WiresMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        &[]
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        match *self {}
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match *self {}
    }
    fn target(&self) -> Vec<String> {
        match *self {}
    }
}




//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
