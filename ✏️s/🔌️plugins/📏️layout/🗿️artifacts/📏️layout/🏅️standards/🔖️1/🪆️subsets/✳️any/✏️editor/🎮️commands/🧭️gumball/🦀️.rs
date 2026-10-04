//! 🧭️ Blueprint gumball — the `translateSelection`/`rotateSelection`/`scaleSelection` verbs. Each is one dispatch of the
//! Transform utility's tool machine: a one-shot (no `phase`) or one `stream`/`commit`/`abort` step of a gesture the canvas
//! overlay streams, yielding the parametric `drag-frames`/`rotate-frames`/`scale-frames` leaf in ONE tool transaction.

use crate::editor::layout::canvas::active_page;
use crate::editor::layout::modes::edit::windows::blueprint::config::current;
use crate::editor::layout::modes::edit::windows::blueprint::transform::{layout_transform_dispatch, layout_unique_targets, LayoutFrameMotion, LayoutFrameRecord, LayoutTransformDispatch, LayoutTransformPhase, LayoutTransformToolState};
use crate::mutations::{layout_frame_selection_pivot, LayoutMutation};
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

/// ✋️ `translateSelection` — moves the frames by `{dx, dy}` page units.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "translate-selection")]
pub struct TranslateSelection {
    #[value(default)]
    pub ids: Vec<String>,
    #[value(default)]
    pub dx: f64,
    #[value(default)]
    pub dy: f64,
    #[value(default)]
    pub phase: Option<String>,
    #[value(default)]
    pub reason: Option<String>,
}

/// 🔃️ `rotateSelection` — turns the frames by `angle` radians (counter-clockwise) about the centroid of their centres.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    #[value(default)]
    pub ids: Vec<String>,
    #[value(default)]
    pub angle: f64,
    #[value(default)]
    pub phase: Option<String>,
    #[value(default)]
    pub reason: Option<String>,
}

/// 🗜️ `scaleSelection` — scales the frames by `{sx, sy}` about the centroid of their centres.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    #[value(default)]
    pub ids: Vec<String>,
    #[value(default)]
    pub sx: f64,
    #[value(default)]
    pub sy: f64,
    #[value(default)]
    pub phase: Option<String>,
    #[value(default)]
    pub reason: Option<String>,
}

/// 🎯️ What a gumball verb asks of its frames before the pivot is known.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutGumballIntent {
    Drag { dx: f64, dy: f64 },
    Rotate { angle: f64 },
    Scale { sx: f64, sy: f64 },
}

impl LayoutGumballIntent {
    /// 📍️ The motion about `pivot`; a turn or scaling without a pivot (no target frame) moves nothing.
    fn motion(self, pivot: Option<(f64, f64)>) -> Option<LayoutFrameMotion> {
        match (self, pivot) {
            (Self::Drag { dx, dy }, _) => Some(LayoutFrameMotion::Drag { dx, dy }),
            (Self::Rotate { angle }, Some((pivot_x, pivot_y))) => Some(LayoutFrameMotion::Rotate { pivot_x, pivot_y, angle }),
            (Self::Scale { sx, sy }, Some((pivot_x, pivot_y))) => Some(LayoutFrameMotion::Scale { pivot_x, pivot_y, sx, sy }),
            (Self::Rotate { .. } | Self::Scale { .. }, None) => None,
        }
    }
}

/// 🎛️ One gumball verb, decoded: its manifest id, its gesture phase, the frames it names and what it asks of them.
pub struct LayoutGumballVerb<'a> {
    pub verb: &'static str,
    pub phase: Option<&'a str>,
    pub reason: Option<&'a str>,
    pub ids: &'a [String],
    pub intent: LayoutGumballIntent,
}

impl TranslateSelection {
    /// 🎛️ This payload as a gumball verb.
    pub fn verb(&self) -> LayoutGumballVerb<'_> {
        LayoutGumballVerb { verb: "translateSelection", phase: self.phase.as_deref(), reason: self.reason.as_deref(), ids: &self.ids, intent: LayoutGumballIntent::Drag { dx: self.dx, dy: self.dy } }
    }
}

impl RotateSelection {
    /// 🎛️ This payload as a gumball verb.
    pub fn verb(&self) -> LayoutGumballVerb<'_> {
        LayoutGumballVerb { verb: "rotateSelection", phase: self.phase.as_deref(), reason: self.reason.as_deref(), ids: &self.ids, intent: LayoutGumballIntent::Rotate { angle: self.angle } }
    }
}

impl ScaleSelection {
    /// 🎛️ This payload as a gumball verb.
    pub fn verb(&self) -> LayoutGumballVerb<'_> {
        LayoutGumballVerb { verb: "scaleSelection", phase: self.phase.as_deref(), reason: self.reason.as_deref(), ids: &self.ids, intent: LayoutGumballIntent::Scale { sx: self.sx, sy: self.sy } }
    }
}

/// 🧭️ Runs one gumball verb through the window's transform tool on the active page of `cfg`: the frames are the verb's
/// `ids`, else `selected`; a turn or scaling records the centroid of the frame centres as its pivot. An unknown phase is
/// refused without touching the gesture. `notice` is the localized sentence a fully locked request raises.
pub fn transform(verb: LayoutGumballVerb<'_>, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>, selected: &[String], open: Option<&LayoutTransformToolState>, notice: Option<&str>) -> LayoutTransformDispatch {
    let Some(phase) = LayoutTransformPhase::parse(verb.phase, verb.reason) else {
        return LayoutTransformDispatch { emit: Emit { ui_scope: semio_framework::kernel::UiDirtyScope::None, ..Emit::default() }, tool: None };
    };
    let record = active_page(doc.snapshot, &current(cfg)).and_then(|page| {
        let targets = layout_unique_targets(if verb.ids.is_empty() { selected } else { verb.ids }.iter().cloned());
        let motion = verb.intent.motion(layout_frame_selection_pivot(page, &targets))?;
        Some(LayoutFrameRecord { page_id: page.id.clone(), targets, motion })
    });
    let operation = doc.operation_optional();
    let seed = operation.map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
    let revision = operation.map(|operation| operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect::<String>()).unwrap_or_default();
    layout_transform_dispatch(verb.verb, phase, record, doc.snapshot, open, seed, &revision, notice)
}

/// ✋️ `translateSelection` without a window: a one-shot transaction (a streamed phase has no window to persist in).
pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Ok(transform(payload.verb(), doc, cfg, &[], None, None).emit)
}

/// 🔃️ `rotateSelection` without a window: a one-shot transaction.
pub fn rotate(payload: &RotateSelection, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Ok(transform(payload.verb(), doc, cfg, &[], None, None).emit)
}

/// 🗜️ `scaleSelection` without a window: a one-shot transaction.
pub fn scale(payload: &ScaleSelection, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Ok(transform(payload.verb(), doc, cfg, &[], None, None).emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
