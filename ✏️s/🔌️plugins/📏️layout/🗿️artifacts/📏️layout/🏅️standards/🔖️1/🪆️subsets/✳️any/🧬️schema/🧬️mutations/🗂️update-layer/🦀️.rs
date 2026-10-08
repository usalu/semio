//! 🗂 `update-layer` — sets a page layer's name, visibility, and lock.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayerPatch, LayoutPagesDelta, LayoutPagesModification, PageLayersDelta, PagePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateLayer {
    pub page_id: String,
    pub layer_id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdateLayer {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "layer", kind: "update-layer", record: "UpdatedLayer" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_layer(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_update_layer(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update layer \"{}\"", self.name), &format!("Ebene \"{}\" aktualisieren", self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.layer_id.clone()]
    }
}

pub fn diff_update_layer(payload: &UpdateLayer, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(layer) = page.layers.iter().find(|layer| layer.id == payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist on page \"{}\".", payload.layer_id, payload.page_id), [payload.layer_id.clone()]);
    };
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A layer needs a name.", std::iter::empty::<String>());
    }
    if layer.name == payload.name && layer.visible == payload.visible && layer.locked == payload.locked {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Layer is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            modified: vec![LayoutPagesModification {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    layers: PageLayersDelta::modification(payload.layer_id.clone(), LayerPatch { name: Some(payload.name.clone()), visible: Some(payload.visible), locked: Some(payload.locked) }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn inverse_update_layer(payload: &UpdateLayer, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else { return Vec::new() };
    let Some(layer) = page.layers.iter().find(|layer| layer.id == payload.layer_id) else { return Vec::new() };
    vec![LayoutMutation::UpdateLayer(UpdateLayer { page_id: payload.page_id.clone(), layer_id: payload.layer_id.clone(), name: layer.name.clone(), visible: layer.visible, locked: layer.locked })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
