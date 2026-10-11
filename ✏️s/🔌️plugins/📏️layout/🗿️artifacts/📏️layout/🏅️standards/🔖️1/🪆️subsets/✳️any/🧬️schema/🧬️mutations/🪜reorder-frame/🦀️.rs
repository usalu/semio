//! 🪜 `reorder-frame` — moves one page frame one step forward or backward in the paint order.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPagesModification, PageFramesDelta, PagePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReorderFrame {
    pub page_id: String,
    pub frame_id: String,
    pub forward: bool,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ReorderFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "frame", kind: "reorder-frame", record: "ReorderFrame" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_reorder_frame(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({ inverse_reorder_frame(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        if self.forward {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Bring frame \"{}\" forward", self.frame_id), &format!("Rahmen \"{}\" nach vorn holen", self.frame_id))
        } else {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Send frame \"{}\" backward", self.frame_id), &format!("Rahmen \"{}\" nach hinten stellen", self.frame_id))
        }
    }
    fn target(&self) -> Vec<String> { vec![self.frame_id.clone()] }
}

pub fn diff_reorder_frame(payload: &ReorderFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(index) = page.frames.iter().position(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist.", payload.frame_id), [payload.frame_id.clone()]);
    };
    let frame = &page.frames[index];
    if frame.locked() || crate::layer_locked(base, page, frame.layer_id()) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "A locked frame stays in its stack position.", [payload.frame_id.clone()]);
    }
    let Some(target) = neighbor(index, page.frames.len(), payload.forward) else {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame is already at that end of the stack.");
    };
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.page_id.clone(), patch: PagePatch { frames: PageFramesDelta::relocation_by_id(payload.frame_id.clone(), index, target), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}

pub fn inverse_reorder_frame(payload: &ReorderFrame, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else { return Vec::new() };
    let Some(index) = page.frames.iter().position(|frame| frame.id() == payload.frame_id) else { return Vec::new() };
    if neighbor(index, page.frames.len(), payload.forward).is_none() {
        return Vec::new();
    }
    vec![LayoutMutation::ReorderFrame(ReorderFrame { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), forward: !payload.forward })]

    })())
}

fn neighbor(index: usize, len: usize, forward: bool) -> Option<usize> {
    if forward {
        let next = index + 1;
        (next < len).then_some(next)
    } else {
        index.checked_sub(1)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
