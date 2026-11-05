//! ➕️ `insert-panel-layer` — inserts one layer into the layers of one panel at an index (clamped to the list end).
use semio_s_artifact_norm_en1995_model::{En1995Snapshot, PanelLayer};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertPanelLayer { pub panel_id: String, pub index: usize, pub layer: PanelLayer }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for InsertPanelLayer where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_panel_layer::RemovePanelLayer> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "panel-layer", kind: "insert-panel-layer", record: "InsertedPanelLayer" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Insert layer {} into panel {} at #{}", self.layer.id, self.panel_id, self.index), &format!("Lage {} in Platte {} an #{} einfügen", self.layer.id, self.panel_id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.panel_id.clone(), self.layer.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for InsertPanelLayer where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_panel_layer::RemovePanelLayer> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::insert_panel_layer::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_panel_layer::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
