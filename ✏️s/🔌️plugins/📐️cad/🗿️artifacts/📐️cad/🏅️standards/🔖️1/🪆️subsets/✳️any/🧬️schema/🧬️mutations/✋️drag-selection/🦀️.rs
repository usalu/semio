//! ✋️ CAD mutation — `DragSelection`: a relative, parametric drag of one pane's objects by one common world offset.
//! The gesture's own inputs (which objects, which offset) are the payload, so editing the drag in history re-derives
//! every origin from whatever base it replays on. The gumball translate and the `transform.move` interaction yield it.

use crate::diff::CadDiff;
use crate::mutations::{cad_selection_items, cad_selection_vector, CadMutation};
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ✋️ `drag-selection` payload — the pane, the objects it moves and the world offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "drag-selection")]
pub struct DragSelection {
    pub pane: CadPaneId,
    pub targets: Vec<String>,
    pub offset: [f64; 3],
}

impl MutationKind<CadSnapshot, CadMutation> for DragSelection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection", record: "DraggedSelection" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (items_en, items_de) = cad_selection_items(self.targets.len());
        let (offset_en, offset_de) = cad_selection_vector(self.offset);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {items_en} by {offset_en}"), &format!("{items_de} um {offset_de} ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
