//! ➕ `create-widget` payload — brings a new id-keyed [`Widget`] into existence at an insertion
//! index (FINAL-state, per `📓️derivation-rules.md` rule 3's addressing convention).

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::Widget;
use serde::{Deserialize, Serialize};

//#region 🔖️CreateWidget
/// ➕ Full initial payload for a new widget, placed at `index` if no widget with the same id
/// already exists (upsert-by-id, matching `apply_widgets_diff`'s own dedupe rule).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for CreateWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::create_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::create_widget::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create widget \"{}\"", crate::artifacts::procedural3d::widget_id(&self.widget))
    }

    fn target(&self) -> Vec<String> {
        vec![crate::artifacts::procedural3d::widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️CreateWidget
