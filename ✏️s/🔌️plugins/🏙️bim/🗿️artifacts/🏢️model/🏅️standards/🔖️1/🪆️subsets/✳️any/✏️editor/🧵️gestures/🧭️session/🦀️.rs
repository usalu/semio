//! 🧭️ The vocabulary every authoring tool speaks: the pointer events a window delivers (already in model metres), the context a tool reads (the model, its inference, the surface
//! the pointer is on, the selections of both domains), what a tool answers (`Step`: the mutations of one history row, a selection request, a utility to arm) and what it shows
//! meanwhile (`Preview`: marks painted over the window, never part of the document). A tool is a small statechart: `event` advances it, `preview` projects it.

use super::plane::P;
use super::snap::{snap, SnapHit, SnapKind, SnapRequest};
use crate::editor::bim::entities::id_taken;
use crate::editor::bim::kit::IdMint;
use crate::editor::bim::terminology::BimLabels;
use crate::{ModelInference, ModelMutation, ModelSnapshot};
use semio_framework_ui_locale::LabelText;
use std::collections::BTreeMap;
use value_derive::{FromValue, ToValue};

//#region 🔖️Codes
/// 🚫️ The model has no storey the tool could draw on.
pub const STOREY_MISSING: &str = "bim.tool.storey-missing";
/// 🚫️ The library holds no type of the kind the tool places.
pub const TYPE_MISSING: &str = "bim.tool.type-missing";
/// 🚫️ The model refused what the tool proposed; nothing was written.
pub const REJECTED: &str = "bim.tool.rejected";
//#endregion 🔖️Codes

//#region 🔖️Events
/// ⌨️ The modifier keys held during a pointer event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
}

impl Modifiers {
    pub fn subtractive(&self) -> bool {
        self.ctrl || self.meta
    }

    /// 🔀️ The framework merge mode of the held modifiers.
    pub fn merge(&self) -> &'static str {
        match (self.shift, self.subtractive()) {
            (true, true) => "invertive",
            (true, false) => "additive",
            (false, true) => "subtractive",
            (false, false) => "replace",
        }
    }
}

/// 🖱️ One pointer sample: where it is on the surface in model metres, the modifiers and how far one pixel reaches in metres (the pick and snap tolerance).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pointer {
    pub at: P,
    pub modifiers: Modifiers,
    pub tolerance: f64,
}

/// 🖱️ What a window tells a tool.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolEvent {
    Down(Pointer),
    Move(Pointer),
    Up(Pointer),
    Double(Pointer),
    Finish,
    Escape,
    Lost,
}
//#endregion 🔖️Events

//#region 🔖️Context
/// 🪟️ The surface a pointer is on: the plan or the world of a storey (coordinates are plan metres), or the section (coordinates are `(u, v)` along the line and up).
#[derive(Clone, Debug, PartialEq)]
pub enum Surface {
    Plan { storey: String },
    World { storey: String },
    Section { start: P, end: P },
}

/// 👁️ What a tool reads while it advances.
pub struct ToolContext<'a> {
    pub snapshot: &'a ModelSnapshot,
    pub inference: &'a ModelInference,
    pub surface: Surface,
    pub selected: &'a [String],
    pub library: &'a [String],
    pub labels: Option<&'static BimLabels>,
    pub mint: IdMint,
}

impl<'a> ToolContext<'a> {
    pub fn new(snapshot: &'a ModelSnapshot, inference: &'a ModelInference, surface: Surface, authoring_seed: &str) -> Self {
        Self { snapshot, inference, surface, selected: &[], library: &[], labels: None, mint: IdMint::seeded(authoring_seed) }
    }

    /// 🪜️ The storey a plan or world tool draws on.
    pub fn storey(&self) -> Option<&str> {
        match &self.surface {
            Surface::Plan { storey } | Surface::World { storey } => Some(storey.as_str()).filter(|storey| self.snapshot.storeys.contains_key(*storey)),
            Surface::Section { .. } => None,
        }
    }

    pub fn building(&self) -> Option<&str> {
        self.storey().and_then(|storey| self.snapshot.storeys.get(storey)).map(|row| row.building.as_str())
    }

    pub fn mint(&mut self, prefix: &str) -> String {
        let snapshot = self.snapshot;
        self.mint.mint(prefix, |id| id_taken(snapshot, id))
    }

    /// 📚️ The type a placement uses: the selected library entry of the family when there is one, else the first of the family.
    pub fn library_type<T>(&self, family: &BTreeMap<String, T>) -> Option<String> {
        self.library.iter().find(|id| family.contains_key(*id)).or_else(|| family.keys().next()).cloned()
    }

    /// 🛡️ Whether the model accepts `mutation` as it stands: the tools propose only what applies.
    pub fn accepts(&self, mutation: &ModelMutation) -> bool {
        use protocol::Mutation;
        let (_diff, messages) = mutation.diff(self.snapshot).into_parts();
        !messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal))
    }

    /// 🏷️ The default name of the next element of a kind: the kind's label and the next ordinal.
    pub fn name_of(&self, label: fn(&BimLabels) -> LabelText, fallback: &str, existing: usize) -> String {
        format!("{} {}", self.labels.map_or_else(|| fallback.to_string(), |labels| label(labels).as_str().to_string()), existing + 1)
    }

    /// 🧲️ The snapped model point of a pointer: the strongest candidate of the storey within reach, else the orthogonal direction from `anchor`.
    pub fn snapped(&self, pointer: &Pointer, anchor: Option<P>, exclude: &[String], extra: &[P]) -> SnapHit {
        if matches!(self.surface, Surface::Section { .. }) {
            return SnapHit { point: pointer.at, kind: SnapKind::Free, source: String::new() };
        }
        snap(self.snapshot, pointer.at, &SnapRequest { storey: self.storey(), tolerance: pointer.tolerance * SNAP_PIXELS, anchor, lock_orthogonal: pointer.modifiers.shift && anchor.is_some(), exclude, extra })
    }
}

/// 🧲️ The reach of a snap in pixels.
pub const SNAP_PIXELS: f64 = 10.0;
/// 🎯️ The reach of a pick or a handle in pixels.
pub const PICK_PIXELS: f64 = 8.0;
//#endregion 🔖️Context

//#region 🔖️Step
/// 🎯️ A selection request: the `(granularity, id)` targets and the framework merge mode.
#[derive(Clone, Debug, PartialEq)]
pub struct Pick {
    pub targets: Vec<(String, String)>,
    pub merge: &'static str,
}

/// 🧱️ What a tool answers to one event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Step {
    pub mutations: Vec<ModelMutation>,
    pub pick: Option<Pick>,
    pub arm: Option<String>,
    pub refused: Option<&'static str>,
}

impl Step {
    pub fn mutate(mutations: Vec<ModelMutation>) -> Self {
        Self { mutations, ..Self::default() }
    }

    pub fn refuse(code: &'static str) -> Self {
        Self { refused: Some(code), ..Self::default() }
    }

    pub fn select(targets: Vec<(String, String)>, merge: &'static str) -> Self {
        Self { pick: Some(Pick { targets, merge }), ..Self::default() }
    }

    /// 🛡️ The step writing `mutation` when the model accepts it, else the refusal.
    pub fn write(ctx: &ToolContext<'_>, mutation: ModelMutation) -> Self {
        if ctx.accepts(&mutation) { Self::mutate(vec![mutation]) } else { Self::refuse(REJECTED) }
    }
}
//#endregion 🔖️Step

//#region 🔖️Preview
/// 🖌️ How a mark is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum Style {
    Ghost,
    Guide,
    Snap,
    Handle,
    Warning,
    Selection,
}

/// 🔷️ What a mark is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum Shape {
    Path,
    Dot,
    Label,
    Snap,
}

/// 🫧️ One mark over a window: a path (flat `x, y` pairs), a dot, a text label at the first point or a snap marker named by its kind in `text`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Mark {
    pub shape: Shape,
    pub style: Style,
    pub closed: bool,
    pub points: Vec<f64>,
    pub text: String,
}

impl Mark {
    pub fn path(points: &[P], closed: bool, style: Style) -> Self {
        Self { shape: Shape::Path, style, closed, points: points.iter().flatten().copied().collect(), text: String::new() }
    }

    pub fn dot(at: P, style: Style) -> Self {
        Self { shape: Shape::Dot, style, closed: false, points: at.to_vec(), text: String::new() }
    }

    pub fn label(at: P, text: impl Into<String>) -> Self {
        Self { shape: Shape::Label, style: Style::Guide, closed: false, points: at.to_vec(), text: text.into() }
    }

    pub fn snap(hit: &SnapHit) -> Self {
        Self { shape: Shape::Snap, style: Style::Snap, closed: false, points: hit.point.to_vec(), text: format!("{:?}", hit.kind).to_lowercase() }
    }

    pub fn corners(&self) -> Vec<P> {
        self.points.chunks_exact(2).map(|pair| [pair[0], pair[1]]).collect()
    }
}

/// 🫧️ The marks a tool shows over its window.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Preview {
    pub marks: Vec<Mark>,
}

impl Preview {
    pub fn of(marks: Vec<Mark>) -> Self {
        Self { marks }
    }

    pub fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }

    /// 🔤️ The preview as the text a window transient carries (empty for no marks).
    pub fn to_text(&self) -> String {
        if self.marks.is_empty() { String::new() } else { semio_framework_pack_json::to_json_string(self) }
    }

    /// 🔤️ The preview a window transient carries; text that does not decode is no preview.
    pub fn from_text(text: &str) -> Self {
        if text.is_empty() {
            return Self::default();
        }
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()
    }
}

/// 📏️ A length label at the middle of a segment, in metres with two decimals.
pub fn length_label(a: P, b: P) -> Mark {
    Mark::label(super::plane::mid(a, b), format!("{:.2} m", super::plane::dist(a, b)))
}
//#endregion 🔖️Preview

//#region 🔖️Tool
/// 🧭️ One authoring tool: a statechart advanced by events and projected as marks.
pub trait Tool: Send {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step;

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview;
}

/// 💤️ The tool of a utility that owns no gesture.
#[derive(Default)]
pub struct Idle;

impl Tool for Idle {
    fn event(&mut self, _ctx: &mut ToolContext<'_>, _event: &ToolEvent) -> Step {
        Step::default()
    }

    fn preview(&self, _ctx: &ToolContext<'_>) -> Preview {
        Preview::default()
    }
}
//#endregion 🔖️Tool

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
