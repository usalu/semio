//! 🔒 `set-frame-flags` — sets a frame's locked and visible flags.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFrameFlags {
    pub page_id: String,
    pub frame_id: String,
    pub locked: Option<bool>,
    pub visible: Option<bool>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for SetFrameFlags {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "frame-flags", kind: "set-frame-flags", record: "SetFrameFlags" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_set_frame_flags(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_set_frame_flags(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set flags on frame \"{}\"", self.frame_id), &format!("Markierungen von Rahmen \"{}\" setzen", self.frame_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}

pub fn diff_set_frame_flags(payload: &SetFrameFlags, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if payload.locked.is_none() && payload.visible.is_none() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Frame flags were not changed.");
    }
    if payload.locked.is_some_and(|locked| locked == frame.locked()) && payload.visible.is_some_and(|visible| visible == frame.visible()) && payload.locked.is_some() && payload.visible.is_some() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Frame flags are already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { locked: payload.locked, visible: payload.visible, ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn inverse_set_frame_flags(payload: &SetFrameFlags, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else { return Vec::new() };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else { return Vec::new() };
    vec![LayoutMutation::SetFrameFlags(SetFrameFlags {
        page_id: payload.page_id.clone(),
        frame_id: payload.frame_id.clone(),
        locked: payload.locked.map(|_| frame.locked()),
        visible: payload.visible.map(|_| frame.visible()),
    })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
