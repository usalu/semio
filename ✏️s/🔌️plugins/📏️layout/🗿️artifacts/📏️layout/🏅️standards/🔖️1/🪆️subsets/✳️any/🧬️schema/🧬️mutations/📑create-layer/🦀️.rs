//! 📑 `create-layer` — appends an empty layer on one page. The inverse removes that layer.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPagesModification, PageLayersDelta, PagePatch};
use crate::{Layer, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateLayer {
    pub page_id: String,
    pub id: String,
    pub name: String,
    pub index: Option<usize>,
    #[value(default)]
    pub remove: bool,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateLayer {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_create_layer(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({ inverse_create_layer(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        if self.remove {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete layer \"{}\"", self.name), &format!("Ebene \"{}\" löschen", self.name))
        } else {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Add layer \"{}\"", self.name), &format!("Ebene \"{}\" hinzufügen", self.name))
        }
    }
    fn target(&self) -> Vec<String> { vec![self.page_id.clone()] }
}

pub fn diff_create_layer(payload: &CreateLayer, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if payload.remove {
        let Some(at) = page.layers.iter().position(|layer| layer.id == payload.id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.id), [payload.id.clone()]);
        };
        let layer = &page.layers[at];
        if page.frames.iter().any(|frame| frame.layer_id() == payload.id) || !layer.object_ids.is_empty() {
            return protocol::MutationOutcome::fatal("mutation.invariant", "A layer can be removed only while it holds no frames.", std::iter::empty::<String>());
        }
        return protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.page_id.clone(), patch: PagePatch { layers: PageLayersDelta::removal_by_id(payload.id.clone(), at), ..Default::default() } }], ..Default::default() }), ..Default::default() });
    }
    if payload.id.is_empty() || payload.name.is_empty() || payload.name.len() > 256 || page.layers.len() >= 16 || page.layers.iter().any(|layer| layer.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A page holds at most 16 layers, and each new layer needs a fresh id and a short name.", std::iter::empty::<String>());
    }
    if payload.index.is_some_and(|at| at > page.layers.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), page.layers.len()), [payload.id.clone()]);
    }
    let layer = Layer { id: payload.id.clone(), name: payload.name.clone(), visible: true, locked: false, object_ids: Vec::new() };
    let at = payload.index.unwrap_or(page.layers.len());
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.page_id.clone(), patch: PagePatch { layers: PageLayersDelta::insertion(at, layer), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_create_layer(payload: &CreateLayer, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.remove {
        let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else { return Vec::new() };
        let Some(at) = page.layers.iter().position(|layer| layer.id == payload.id) else { return Vec::new() };
        let layer = &page.layers[at];
        let mut steps = Vec::new();
        if !layer.visible || layer.locked {
            steps.push(LayoutMutation::UpdateLayer(crate::mutations::update_layer::UpdateLayer { page_id: payload.page_id.clone(), layer_id: layer.id.clone(), name: layer.name.clone(), visible: layer.visible, locked: layer.locked }));
        }
        steps.push(LayoutMutation::CreateLayer(CreateLayer { page_id: payload.page_id.clone(), id: layer.id.clone(), name: layer.name.clone(), index: Some(at), remove: false }));
        return steps;
    }
    vec![LayoutMutation::CreateLayer(CreateLayer { page_id: payload.page_id.clone(), id: payload.id.clone(), name: payload.name.clone(), index: None, remove: true })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
