//! 🏗️ reorder-layer produces a structural diff and retains its semantic inverse.

use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReorderLayer {
    pub page_id: String,
    pub layer_id: String,
    pub to_index: usize,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ReorderLayer
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::reorder_layer::ReorderLayer> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "layer", kind: "reorder-layer", record: "ReorderedLayer" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Reorder layer", "Ebene umordnen") }
    fn target(&self) -> Vec<String> { vec![self.page_id.clone(), self.layer_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for ReorderLayer
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::reorder_layer::ReorderLayer> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_reorder_layer(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_reorder_layer(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_reorder_layer(payload: &ReorderLayer, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The page does not exist.", [payload.page_id.clone()]);
    };
    let Some(from) = page.layers.iter().position(|layer| layer.id == payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The layer does not exist.", [payload.layer_id.clone()]);
    };
    if payload.to_index >= page.layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The layer position is beyond the list.", [payload.layer_id.clone()]);
    }
    if from == payload.to_index {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The layer is already at that position.");
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.page_id.clone(), patch: PagePatch { layers: PageLayersDelta::relocation_by_id(payload.layer_id.clone(), from, payload.to_index), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_reorder_layer<Op: From<crate::reorder_layer::ReorderLayer>>(payload: &ReorderLayer, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_reorder_layer(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    let page = base.pages.iter().find(|page| page.id == payload.page_id).expect("validated target");
    let to_index = page.layers.iter().position(|layer| layer.id == payload.layer_id).expect("validated target");
    Ok(vec![Op::from(ReorderLayer { page_id: payload.page_id.clone(), layer_id: payload.layer_id.clone(), to_index })])
}
