//! 🧩️ Patchable rows of the layout diff: the patchable style, page and story rows, the frame field patch and the frame selection helpers. See [`super`].

use super::*;

impl Patchable<crate::ParagraphStylePatch> for ParagraphStyle {
    fn apply_patch(&mut self, patch: &crate::ParagraphStylePatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(font_family) = &patch.font_family {
            self.font_family = font_family.clone();
        }
        if let Some(font_size) = patch.font_size {
            self.font_size = font_size;
        }
        if let Some(font_weight) = patch.font_weight {
            self.font_weight = font_weight;
        }
        if let Some(leading) = patch.leading {
            self.leading = leading;
        }
        if let Some(tracking) = patch.tracking {
            self.tracking = tracking;
        }
        if let Some(alignment) = &patch.alignment {
            self.alignment = alignment.clone();
        }
    }
}

impl Patchable<crate::CharacterStylePatch> for CharacterStyle {
    fn apply_patch(&mut self, patch: &crate::CharacterStylePatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(font_family) = &patch.font_family {
            self.font_family = font_family.clone();
        }
        if let Some(font_size) = &patch.font_size {
            self.font_size = *font_size;
        }
        if let Some(font_weight) = &patch.font_weight {
            self.font_weight = *font_weight;
        }
        if let Some(italic) = &patch.italic {
            self.italic = *italic;
        }
        if let Some(color) = &patch.color {
            self.color = *color;
        }
        if let Some(tracking) = &patch.tracking {
            self.tracking = *tracking;
        }
    }
}

impl Patchable<crate::ParentPagePatch> for ParentPage {
    fn apply_patch(&mut self, patch: &crate::ParentPagePatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(width) = patch.width {
            self.width = width;
        }
        if let Some(height) = patch.height {
            self.height = height;
        }
    }
}

impl Patchable<crate::SpreadPatch> for Spread {
    fn apply_patch(&mut self, patch: &crate::SpreadPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
    }
}

/// 🩹️ Pure field-apply for a {@link FramePatch} onto a {@link Frame} — no inverse capture (every
/// semantic mutation computes its own inverse from `base` directly; see `↩️inverse` triad leaves).
pub fn apply_frame_field_patch(frame: &mut Frame, patch: &FramePatch) {
    if let Some(layer_id) = &patch.layer_id {
        set_frame_layer(frame, layer_id);
    }
    {
        let bounds = match frame {
            Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
        };
        if let Some(value) = patch.x {
            bounds.x = value;
        }
        if let Some(value) = patch.y {
            bounds.y = value;
        }
        if let Some(value) = patch.width {
            bounds.width = value;
        }
        if let Some(value) = patch.height {
            bounds.height = value;
        }
        if let Some(value) = patch.rotation {
            bounds.rotation = value;
        }
    }
    {
        let (locked, visible) = match frame {
            Frame::Rect { locked, visible, .. } | Frame::Text { locked, visible, .. } | Frame::Image { locked, visible, .. } => (locked, visible),
        };
        if let Some(value) = patch.locked {
            *locked = value.then_some(true);
        }
        if let Some(value) = patch.visible {
            *visible = (!value).then_some(false);
        }
    }
    match frame {
        Frame::Rect { fill, stroke, .. } => {
            if let Some(new) = patch.fill {
                *fill = new;
            }
            if let Some(new) = patch.stroke {
                *stroke = new;
            }
        }
        Frame::Text { wrap_mode, columns, story_id, thread_next, inset, .. } => {
            if let Some(new) = &patch.wrap_mode {
                *wrap_mode = new.clone();
            }
            if let Some(new) = patch.columns {
                *columns = new;
            }
            if let Some(new) = &patch.story_id {
                *story_id = new.clone();
            }
            if let Some(new) = &patch.thread_next {
                *thread_next = new.clone();
            }
            if let Some(new) = patch.inset_x {
                inset.x = new;
            }
            if let Some(new) = patch.inset_y {
                inset.y = new;
            }
            if let Some(new) = patch.inset_width {
                inset.width = new;
            }
            if let Some(new) = patch.inset_height {
                inset.height = new;
            }
        }
        Frame::Image { .. } => {}
    }
}

/// 📝️ Sparse patch for a {@link TextStory}'s body content.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct TextStoryPatch {
    pub content: Option<String>,
    #[value(default)]
    pub style_runs: Option<crate::TextStyleRunsDelta>,
}

impl Patchable<TextStoryPatch> for TextStory {
    fn apply_patch(&mut self, patch: &TextStoryPatch) {
        if let Some(content) = &patch.content {
            self.content = content.clone();
        }
        if let Some(style_runs) = patch.style_runs.as_ref().and_then(|delta| crate::apply_story_runs(&self.style_runs, delta).ok()) {
            self.style_runs = style_runs;
        }
    }
}

/// 🔗️ Sparse patch for an {@link ImageLink}'s file path.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct ImageLinkPatch {
    pub path: Option<String>,
    #[value(default)]
    pub width: Option<u32>,
    #[value(default)]
    pub height: Option<u32>,
    #[value(default)]
    pub dpi: Option<u32>,
    #[value(default)]
    pub color_profile: Option<String>,
}

impl Patchable<ImageLinkPatch> for ImageLink {
    fn apply_patch(&mut self, patch: &ImageLinkPatch) {
        if let Some(path) = &patch.path {
            self.path = path.clone();
        }
        if let Some(width) = patch.width {
            self.width = width;
        }
        if let Some(height) = patch.height {
            self.height = height;
        }
        if let Some(dpi) = patch.dpi {
            self.dpi = dpi;
        }
        if let Some(color_profile) = &patch.color_profile {
            self.color_profile = (!color_profile.is_empty()).then(|| color_profile.clone());
        }
    }
}

/// 🖼️ Sparse patch for a {@link Frame}: bounds for any kind, fill/stroke for rects, wrap-mode/columns
/// for text. The doubly-optional `fill`/`stroke` distinguishes "unchanged" (outer `None`) from
/// "cleared" (inner `None`). Needed both by `op`'s `PatchFrame` operation and by the DSL/spr mirror in
/// `spr` (`FramePatchDsl`), so it lives here alongside the other `*Patch` records rather than in `op`
/// itself. Frame patching is per-page nested rather than a flat collection-wide op, so unlike the
/// patches above it has no `Patchable` impl.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct FramePatch {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    #[value(default)]
    pub rotation: Option<f64>,
    pub fill: Option<Option<[f32; 4]>>,
    pub stroke: Option<Option<[f32; 4]>>,
    pub wrap_mode: Option<String>,
    pub columns: Option<u32>,
    #[value(default)]
    pub locked: Option<bool>,
    #[value(default)]
    pub visible: Option<bool>,
    #[value(default)]
    pub story_id: Option<String>,
    #[value(default)]
    pub thread_next: Option<Option<String>>,
    #[value(default)]
    pub inset_x: Option<f64>,
    #[value(default)]
    pub inset_y: Option<f64>,
    #[value(default)]
    pub inset_width: Option<f64>,
    #[value(default)]
    pub inset_height: Option<f64>,
    #[value(default)]
    pub layer_id: Option<String>,
}

/// 🖼️ The sparse diff of a frame-selection transform (`drag-frames`/`rotate-frames`/`scale-frames`): every unlocked
/// target frame of `page_id` gets `transform` applied to its BASE bounds — so the leaf replays on any base — and is
/// patched in page order with the fields `fields` keeps. A repeated or empty target set is the Fatal `mutation.invariant`;
/// a missing page, or no target that is an unlocked frame of it, is the Error `mutation.target-missing`; missing and
/// locked targets are skipped with one Warning `mutation.partial` per reason; a transform that moves nothing is the
/// Warning `mutation.no-op`.
pub fn layout_frame_selection_diff(base: &LayoutSnapshot, page_id: &str, targets: &[String], transform: impl Fn(&semio_s_artifact_layout_layout_model::LayoutBounds) -> semio_s_artifact_layout_layout_model::LayoutBounds, fields: impl Fn(&semio_s_artifact_layout_layout_model::LayoutBounds) -> crate::FramePatch) -> protocol::MutationOutcome<LayoutDiff> {
    use crate::standards::v1::subsets::any::schema::diff::LayoutPagesModification;
    if let Err(reason) = layout_frame_targets_invariant(targets) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, targets.to_vec());
    }
    let Some(page) = base.pages.iter().find(|page| page.id == page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{page_id}\" does not exist."), [page_id.to_string()]);
    };
    let missing: Vec<String> = targets.iter().filter(|id| !page.frames.iter().any(|frame| frame.id() == id.as_str())).cloned().collect();
    let locked: Vec<String> = page.frames.iter().filter(|frame| targets.iter().any(|id| id == frame.id()) && semio_s_artifact_layout_layout_model::frame_edits_blocked(base, page, frame)).map(|frame| frame.id().to_string()).collect();
    let movable = layout_frame_selection_targets(base, page_id, targets);
    if movable.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} target(s) is an unlocked frame of page \"{page_id}\".", targets.len()), targets.to_vec());
    }
    let partial: Vec<protocol::MutationMessage> = [(missing, "not on this page"), (locked, "locked")]
        .into_iter()
        .filter(|(ids, _)| !ids.is_empty())
        .map(|(ids, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", ids.len(), targets.len(), ids.join(", "))).at(ids))
        .collect();
    let patched: Vec<PageFramesModification> = movable.iter().filter_map(|frame| Some(transform(frame.bounds())).filter(|next| next != frame.bounds()).map(|next| PageFramesModification { id: frame.id().to_string(), patch: fields(&next) })).collect();
    if patched.is_empty() {
        return protocol::MutationOutcome::new(LayoutDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "The transform moves no frame.").at(targets.to_vec())]));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: page_id.to_string(), patch: PagePatch { frames: PageFramesDelta { modified: patched, ..Default::default() }, ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
    .absorb_messages(partial)
}

/// 🎯️ The frames a frame-selection transform moves: the unlocked frames of `page_id` named by `targets`, in page order.
pub fn layout_frame_selection_targets<'a>(base: &'a LayoutSnapshot, page_id: &str, targets: &[String]) -> Vec<&'a semio_s_artifact_layout_layout_model::Frame> {
    let Some(page) = base.pages.iter().find(|page| page.id == page_id) else { return Vec::new() };
    page.frames.iter().filter(|frame| targets.iter().any(|id| id == frame.id()) && !semio_s_artifact_layout_layout_model::frame_edits_blocked(base, page, frame)).collect()
}

/// 🚨️ A frame-selection target set names at least one frame and no frame twice — the schema's `minItems`/`uniqueItems`.
pub fn layout_frame_targets_invariant(targets: &[String]) -> Result<(), String> {
    if targets.is_empty() {
        return Err("targets must name at least one frame".to_string());
    }
    match targets.iter().enumerate().find(|(at, id)| targets[..*at].contains(id)) {
        Some((_, id)) => Err(format!("targets must not repeat {id:?}")),
        None => Ok(()),
    }
}

/// 📍️ The pivot a frame-selection rotate or scale records for `targets` on `page`: the centroid of their frame centres —
/// one recorded point, so the leaf replays on any base. `None` when no target is a frame of the page.
pub fn layout_frame_selection_pivot(page: &semio_s_artifact_layout_layout_model::Page, targets: &[String]) -> Option<(f64, f64)> {
    let centres: Vec<(f64, f64)> = page.frames.iter().filter(|frame| targets.iter().any(|id| id == frame.id())).map(|frame| layout_frame_centre(frame.bounds())).collect();
    (!centres.is_empty()).then(|| (centres.iter().map(|centre| centre.0).sum::<f64>() / centres.len() as f64, centres.iter().map(|centre| centre.1).sum::<f64>() / centres.len() as f64))
}

/// 🎯️ A frame's centre: the point its rotation turns about.
pub fn layout_frame_centre(bounds: &semio_s_artifact_layout_layout_model::LayoutBounds) -> (f64, f64) {
    (bounds.x + bounds.width * 0.5, bounds.y + bounds.height * 0.5)
}
