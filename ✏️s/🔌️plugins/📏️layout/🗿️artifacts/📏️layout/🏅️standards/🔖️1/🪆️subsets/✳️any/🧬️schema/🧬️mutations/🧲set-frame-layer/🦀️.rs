//! 🧲 `set-frame-layer` — moves one page frame onto another layer of the same page.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPagesModification, PageFramesDelta, PagePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetFrameLayer {
    pub page_id: String,
    pub frame_id: String,
    pub layer_id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for SetFrameLayer {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "frame-layer", kind: "set-frame-layer", record: "SetFrameLayer" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_frame_layer(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({ inverse_set_frame_layer(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Move frame \"{}\" to layer \"{}\"", self.frame_id, self.layer_id), &format!("Rahmen \"{}\" auf Ebene \"{}\" legen", self.frame_id, self.layer_id)) }
    fn target(&self) -> Vec<String> { vec![self.frame_id.clone()] }
}

pub fn diff_set_frame_layer(payload: &SetFrameLayer, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist.", payload.frame_id), [payload.frame_id.clone()]);
    };
    if page.layers.iter().all(|layer| layer.id != payload.layer_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A frame moves only onto a layer of its own page.", std::iter::empty::<String>());
    }
    if frame.layer_id() == payload.layer_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame is already on that layer.");
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.page_id.clone(), patch: PagePatch { frames: PageFramesDelta::modification(payload.frame_id.clone(), FramePatch { layer_id: Some(payload.layer_id.clone()), ..Default::default() }), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_set_frame_layer(payload: &SetFrameLayer, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else { return Vec::new() };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else { return Vec::new() };
    vec![LayoutMutation::SetFrameLayer(SetFrameLayer { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), layer_id: frame.layer_id().to_string() })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
