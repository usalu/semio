//! 📑 `update-text-frame` — sets a text frame's story, thread, and inset.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::{Frame, FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateTextFrame {
    pub page_id: String,
    pub frame_id: String,
    pub story_id: String,
    pub thread_next: Option<String>,
    pub inset_x: f64,
    pub inset_y: f64,
    pub inset_width: f64,
    pub inset_height: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdateTextFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "text-frame", kind: "update-text-frame", record: "UpdatedTextFrame" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_text_frame(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_update_text_frame(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Update text frame \"{}\"", self.frame_id), &format!("Textrahmen \"{}\" aktualisieren", self.frame_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}

pub fn diff_update_text_frame(payload: &UpdateTextFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    let Frame::Text { story_id, thread_next, inset, .. } = frame else {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("Frame \"{}\" is not a text frame.", payload.frame_id), [payload.frame_id.clone()]);
    };
    if !base.stories.iter().any(|story| story.id == payload.story_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Story \"{}\" does not exist.", payload.story_id), [payload.story_id.clone()]);
    }
    if let Some(next) = &payload.thread_next {
        if next == &payload.frame_id || !base.pages.iter().any(|page| page.frames.iter().any(|frame| frame.id() == next && matches!(frame, Frame::Text { .. }))) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Thread target \"{}\" is not another text frame.", next), [next.clone()]);
        }
    }
    if !payload.inset_x.is_finite() || !payload.inset_y.is_finite() || !payload.inset_width.is_finite() || !payload.inset_height.is_finite() || payload.inset_width < 0.0 || payload.inset_height < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Text inset origin must be finite and the inset size must be finite and non-negative.", std::iter::empty::<String>());
    }
    if story_id == &payload.story_id && thread_next == &payload.thread_next && inset.x == payload.inset_x && inset.y == payload.inset_y && inset.width == payload.inset_width && inset.height == payload.inset_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Text frame is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch {
                    frame_patched: Some(PageFramePatched {
                        frame_id: payload.frame_id.clone(),
                        patch: FramePatch {
                            story_id: Some(payload.story_id.clone()),
                            thread_next: Some(payload.thread_next.clone()),
                            inset_x: Some(payload.inset_x),
                            inset_y: Some(payload.inset_y),
                            inset_width: Some(payload.inset_width),
                            inset_height: Some(payload.inset_height),
                            ..Default::default()
                        },
                    }),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn inverse_update_text_frame(payload: &UpdateTextFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else { return Vec::new() };
    let Some(Frame::Text { story_id, thread_next, inset, .. }) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else { return Vec::new() };
    vec![LayoutMutation::UpdateTextFrame(UpdateTextFrame {
        page_id: payload.page_id.clone(),
        frame_id: payload.frame_id.clone(),
        story_id: story_id.clone(),
        thread_next: thread_next.clone(),
        inset_x: inset.x,
        inset_y: inset.y,
        inset_width: inset.width,
        inset_height: inset.height,
    })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
