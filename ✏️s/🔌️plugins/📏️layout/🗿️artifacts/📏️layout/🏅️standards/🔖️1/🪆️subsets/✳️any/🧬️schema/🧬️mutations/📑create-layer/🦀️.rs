//! 📑 `create-layer` — appends an empty layer on one page. The inverse removes that layer.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::{Layer, LayoutDiff, LayoutSnapshot, PagePatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLayer {
    pub page_id: String,
    pub id: String,
    pub name: String,
    #[value(default)]
    pub remove: bool,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateLayer {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_create_layer(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> { inverse_create_layer(self, base) }
    fn label(&self) -> protocol::LocalizedLabel {
        if self.remove {
            protocol::LocalizedLabel::native(&format!("Delete layer \"{}\"", self.name), &format!("Ebene \"{}\" löschen", self.name))
        } else {
            protocol::LocalizedLabel::native(&format!("Add layer \"{}\"", self.name), &format!("Ebene \"{}\" hinzufügen", self.name))
        }
    }
    fn target(&self) -> Vec<String> { vec![self.page_id.clone()] }
}

pub fn diff_create_layer(payload: &CreateLayer, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if payload.remove {
        let Some(layer) = page.layers.iter().find(|layer| layer.id == payload.id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.id), [payload.id.clone()]);
        };
        if page.frames.iter().any(|frame| frame.layer_id() == payload.id) || !layer.object_ids.is_empty() {
            return protocol::MutationOutcome::fatal("mutation.invariant", "A layer can be removed only while it holds no frames.", std::iter::empty::<String>());
        }
        return protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.page_id.clone(), patch: PagePatch { layer_removed: Some(payload.id.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() });
    }
    if payload.id.is_empty() || payload.name.is_empty() || payload.name.len() > 256 || page.layers.len() >= 16 || page.layers.iter().any(|layer| layer.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A page holds at most 16 layers, and each new layer needs a fresh id and a short name.", std::iter::empty::<String>());
    }
    let layer = Layer { id: payload.id.clone(), name: payload.name.clone(), visible: true, locked: false, object_ids: Vec::new() };
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.page_id.clone(), patch: PagePatch { layer_added: Some(layer), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_create_layer(payload: &CreateLayer, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    if payload.remove {
        let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else { return Vec::new() };
        let Some(layer) = page.layers.iter().find(|layer| layer.id == payload.id) else { return Vec::new() };
        return vec![LayoutMutation::CreateLayer(CreateLayer { page_id: payload.page_id.clone(), id: layer.id.clone(), name: layer.name.clone(), remove: false })];
    }
    vec![LayoutMutation::CreateLayer(CreateLayer { page_id: payload.page_id.clone(), id: payload.id.clone(), name: payload.name.clone(), remove: true })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
