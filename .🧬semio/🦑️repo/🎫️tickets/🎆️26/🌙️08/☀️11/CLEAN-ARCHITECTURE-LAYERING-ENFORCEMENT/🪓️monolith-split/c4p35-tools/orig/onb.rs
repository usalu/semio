//! 🎓️ App onboarding: the step-gated first-run introduction, the recorded tutorial timeline with its playback engine and the reserved actions that start or record them.

use replication::DomainSelection;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use ui_wgpu::wgpu::WindowLayout;
use semio_framework_manifest_actions::{ActionDefinition, ActionKind};
use semio_framework_manifest_address::{ActionRef, ToolRef, UtilityRef};
use semio_framework_manifest_arg_schema::{ActionArgDef, ActionArgOption};

//#region 🔖️IntroductionAction
/// 🎓️ The reserved action id apps dispatch to (re)start an app's introduction — injected by the app builder as a
/// shell-intercepted View action into any `AppDefinition` that declares one (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const START_INTRODUCTION_ACTION_ID: &str = "startIntroduction";

/// 🎓️ The reserved `startIntroduction` View action: fully shell-intercepted (never
/// forwarded to the program), it resets playback to the first step of `AppDefinition.introduction`.
/// Unlike ordinary app actions this stays out of the action palette because the shell exposes the
/// dedicated `Introduce App` command.
pub fn start_introduction_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(START_INTRODUCTION_ACTION_ID, LocalizedLabel::native("Introduce App", "App vorstellen"), ActionKind::View, "graduation-cap") }
}

//#endregion 🔖️IntroductionAction
//#region 🔖️Introduction

/// 🎓️ A first-run walkthrough an app declares to introduce its UI, utilities, and actions to a
/// first-time user. Rendered as an ordered sequence of `IntroductionStepDefinition`s over a full-screen
/// glass veil; the shell owns playback (start/advance/skip) as ephemeral chrome state, never the
/// document.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionDefinition {
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    pub steps: Vec<IntroductionStepDefinition>,
}

/// 🪜️ One step of an `IntroductionDefinition`: an info box pointing at `introduce`, with `show`
/// raising extra elements above the glass veil and `interactions` completing the step.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionStepDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    pub body: LocalizedLabel,
    /// 🎯️ The single element id raised above the glass, pulsing `data-introduced`, that the info box
    /// anchors to. `None` = a screen-style step: full veil, centered info box.
    #[serde(default)]
    #[value(default)]
    pub introduce: Option<String>,
    /// 🕳️ Additional element ids raised above the glass — interactive, no pulse — e.g. every 3D window
    /// that accepts a catalogue drop while `introduce` teaches the drag source.
    #[serde(default)]
    #[value(default)]
    pub show: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub placement: IntroductionPlacement,
    /// ✅️ Interactions completing this step; empty means purely informational (Next-button-only).
    #[serde(default)]
    #[value(default)]
    pub interactions: Vec<IntroductionInteraction>,
    /// 🔢️ Whether `interactions` must complete in declaration order — out-of-order completions are
    /// ignored. Unordered: the first incomplete matching interaction completes.
    #[serde(default)]
    #[value(default)]
    pub ordered: bool,
    /// 🏛️ Institution/partner logos shown in the info box below the body — e.g. funding acknowledgements.
    #[serde(default)]
    #[value(default)]
    pub logos: Vec<IntroductionLogo>,
    /// 🎬️ Ghost-cursor demonstrations played in order, one after another, then looping back to the first —
    /// e.g. a viewport step showing zoom, then pan, then orbit. When the step also declares `interactions`,
    /// `demonstrations[i]` previews `interactions[i]` and completed interactions are omitted from replay.
    /// Empty means no demonstration.
    #[serde(default)]
    #[value(default)]
    pub demonstrations: Vec<IntroductionDemonstration>,
}

// 🚫️async: E1 — pure builder methods (self-mutation only, zero suspension points), reverted
// per R9: multiple test consumers are language-barred from async (plain `#[test] fn`, sync
// `catch_unwind` closures, `Vec<IntroductionStepDefinition>` literals).
impl IntroductionStepDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<LocalizedLabel>, body: impl Into<LocalizedLabel>) -> Self {
        Self { id: id.into(), title: title.into(), body: body.into(), introduce: None, show: Vec::new(), placement: IntroductionPlacement::default(), interactions: Vec::new(), ordered: false, logos: Vec::new(), demonstrations: Vec::new() }
    }

    /// 🎯️ Sets the single element id raised above the glass and anchoring the info box.
    pub fn introduce(mut self, element_id: impl Into<String>) -> Self {
        self.introduce = Some(element_id.into());
        self
    }

    /// 🕳️ Additional element ids raised above the glass alongside `introduce` (no pulse).
    pub fn show(mut self, element_ids: Vec<String>) -> Self {
        self.show = element_ids;
        self
    }

    /// 📍️ Overrides where the info box is placed relative to `introduce`.
    pub fn placement(mut self, placement: IntroductionPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// ✅️ Makes the step complete when the user performs all `interactions` (any order) instead of
    /// pressing Next.
    pub fn interact(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self
    }

    /// 🔢️ Like `interact`, but `interactions` must complete in declaration order.
    pub fn interact_ordered(mut self, interactions: Vec<IntroductionInteraction>) -> Self {
        self.interactions = interactions;
        self.ordered = true;
        self
    }

    /// 🏛️ Attaches institution/partner logos to the step's info box.
    pub fn logos(mut self, logos: Vec<IntroductionLogo>) -> Self {
        self.logos = logos;
        self
    }

    /// 🎬️ Attaches ghost-cursor demonstrations played in order, then looping back to the first.
    pub fn demonstrate(mut self, demonstrations: Vec<IntroductionDemonstration>) -> Self {
        self.demonstrations = demonstrations;
        self
    }
}

/// 🏛️ One institution/partner logo shown in an `IntroductionStepDefinition`'s info box — a plain
/// URL pair (no DOM/CSS types), optionally linking out when clicked.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionLogo {
    pub src: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dark_src: Option<String>,
    pub alt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

/// 📍️ Where the info box is placed relative to its anchor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionPlacement {
    #[default]
    Auto,
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

/// 👉️ What one `IntroductionInteraction` requires: `Action`/`Utility`/`Tool`/`Panel`/`Expand`
/// complete as soon as the user activates that utility/tool, opens that panel tab, or expands that tree
/// section — teaching by doing. `Pan`/`Zoom`/`Orbit` complete on that camera-navigation gesture over the
/// 3D window named by the payload (a window-kind id) — classified from camera-state deltas by the shell
/// that renders the window, so only shells that render a 3D world (the React shell) can complete them.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
#[value(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum IntroductionInteractionKind {
    /// 📇️ References an action owned by the active window kind.
    Action(ActionRef),
    /// 🧰️ References `AppDefinition.utilities`.
    Utility(UtilityRef),
    /// 🛠️ References `AppDefinition.tools` (mode-level tools such as fill).
    Tool(ToolRef),
    /// 📑️ Shell panel tab id (e.g. `framework.panel.catalogue`) — completes when that panel opens.
    Panel(String),
    /// 🌲️ Tree section/item id (e.g. `puzzle3d-play-kinds.objects`) — completes when the user expands it.
    Expand(String),
    /// 🖐️ Completes when the user pans the named 3D window.
    Pan(String),
    /// 🔍️ Completes when the user zooms (scroll or dolly) the named 3D window.
    Zoom(String),
    /// 🌐️ Completes when the user orbits the named 3D window.
    Orbit(String),
}

/// ✅️ One thing the user must do to complete an interaction-gated `IntroductionStepDefinition` —
/// rendered as a checklist row in the info box and celebrated individually on completion.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionInteraction {
    pub on: IntroductionInteractionKind,
    /// 🏷️ Short checklist label shown in the step's info box.
    pub label: String,
    /// 🎉️ Element id stamped `data-celebrated` on completion; `None` falls back to the step's `introduce`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub celebrate: Option<String>,
}

impl IntroductionInteraction {
    async fn new(on: IntroductionInteractionKind, label: impl Into<String>) -> Self {
        Self { on, label: label.into(), celebrate: None }
    }

    /// 📇️ An interaction completing when the user activates action `id`.
    pub async fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Action(ActionRef::new(id.into())), label).await
    }

    /// 🧰️ An interaction completing when the user activates utility `id`.
    pub async fn utility(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Utility(UtilityRef::new(id.into())), label).await
    }

    /// 🛠️ An interaction completing when the user activates tool `id`.
    pub async fn tool(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Tool(ToolRef::new(id.into()).await), label).await
    }

    /// 📑️ An interaction completing when panel tab `id` opens.
    pub async fn panel(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Panel(id.into()), label).await
    }

    /// 🌲️ An interaction completing when tree section/item `id` expands.
    pub async fn expand(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Expand(id.into()), label).await
    }

    /// 🖐️ An interaction completing when the user pans 3D window `window_kind_id`.
    pub async fn pan(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Pan(window_kind_id.into()), label).await
    }

    /// 🔍️ An interaction completing when the user zooms 3D window `window_kind_id`.
    pub async fn zoom(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Zoom(window_kind_id.into()), label).await
    }

    /// 🌐️ An interaction completing when the user orbits 3D window `window_kind_id`.
    pub async fn orbit(window_kind_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IntroductionInteractionKind::Orbit(window_kind_id.into()), label).await
    }

    /// 🎉️ Overrides which element id is stamped `data-celebrated` on completion.
    pub async fn celebrate(mut self, element_id: impl Into<String>) -> Self {
        self.celebrate = Some(element_id.into());
        self
    }
}

/// 📌️ Where a demonstration gesture points, resolvable to a viewport pixel at play time. One
/// point type covers click targets and drag endpoints across every addressing scheme the shell needs:
/// element-relative, absolute/normalized screen space, absolute/normalized window(pane)-local space, and
/// a 3D scene world position projected through that window's live camera.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
// 🐢️ `rename_all_fields` is required alongside `rename_all` — the latter only renames the *variant* tag
// values; without the former, a future multi-word field inside a variant would silently serialize
// snake_case and desync from the generated TS type (see `UiDirtyScope`'s comment for the full story).
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum IntroductionPoint {
    /// 🎯️ Center (or `offset`, normalized 0–1 within the element's rect) of the element `id` resolves to.
    Element {
        id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        offset: Option<[f64; 2]>,
    },
    /// 🖥️ Absolute viewport pixel.
    Screen { x: f64, y: f64 },
    /// 🖥️ Normalized 0–1 of the viewport.
    ScreenNormalized { x: f64, y: f64 },
    /// 🪟️ Pixel local to window/pane element `id`'s rect (top-left origin).
    Window { id: String, x: f64, y: f64 },
    /// 🪟️ Normalized 0–1 within window/pane element `id`'s rect.
    WindowNormalized { id: String, x: f64, y: f64 },
    /// 🧊️ 3D world-space position in the scene shown by window `id`, projected through its live camera.
    Scene { id: String, position: [f64; 3] },
    /// 🗺️ 2D world-space coordinates (camera x/y/zoom) on the infinite-canvas surface shown by window
    /// `id` — the 2D sibling of `Scene`. On a 3D window this resolves via the ground plane (z = 0).
    Canvas { id: String, x: f64, y: f64 },
    /// 🏷️ A live entity addressed semantically in the shell's established pick-target grammar (see
    /// `CanvasPickTarget`): `domain` is the surface's target domain ("vortex", "object", "attraction",
    /// "node", "edge", "handle", "position", "route", "block", "layer", …), `entity` its id verbatim
    /// (compound forms like `"objectId:vortexId"` or `"widgetId:port"` included; `"*"` = any — the
    /// surface picks a representative, nearest the viewport center). `offset` is normalized 0–1 within
    /// the entity's bounds, default center.
    Entity {
        id: String,
        domain: String,
        entity: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        offset: Option<[f64; 2]>,
    },
    /// 🪡️ A parametric point along an entity's curve geometry (an attraction segment, graph edge, ink
    /// stroke, or canvas path layer) — `t` in 0–1 by arc length.
    Curve { id: String, domain: String, entity: String, t: f64 },
    /// 🎚️ A value mapped through an entity's live value domain (e.g. a graph slider's min..max onto its
    /// track), resolved to the corresponding point along the entity's geometry.
    Domain { id: String, domain: String, entity: String, value: f64 },
}

impl IntroductionPoint {
    /// 🗺️ 2D world-space coordinates on the infinite-canvas surface shown by window `window_id`.
    pub async fn canvas(window_id: impl Into<String>, x: f64, y: f64) -> Self {
        Self::Canvas { id: window_id.into(), x, y }
    }

    /// 🏷️ A specific entity by domain + id, centered (no `offset`).
    pub async fn entity(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>) -> Self {
        Self::Entity { id: window_id.into(), domain: domain.into(), entity: entity.into(), offset: None }
    }

    /// 🏷️ Any entity in `domain` — the surface picks a representative, nearest the viewport center.
    pub async fn any_entity(window_id: impl Into<String>, domain: impl Into<String>) -> Self {
        Self::entity(window_id, domain, "*").await
    }

    /// 🪡️ A parametric point at `t` (0–1 by arc length) along an entity's curve geometry.
    pub async fn curve(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, t: f64) -> Self {
        Self::Curve { id: window_id.into(), domain: domain.into(), entity: entity.into(), t }
    }

    /// 🎚️ A value mapped through an entity's live value domain (e.g. a slider's min..max).
    pub async fn domain_value(window_id: impl Into<String>, domain: impl Into<String>, entity: impl Into<String>, value: f64) -> Self {
        Self::Domain { id: window_id.into(), domain: domain.into(), entity: entity.into(), value }
    }
}

/// 🖱️ Which mouse button a drag-like demonstration presses.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionPointerButton {
    #[default]
    Left,
    Middle,
    Right,
}

/// ⌨️ Keyboard modifier held during a drag-like demonstration.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionKeyModifier {
    Alt,
    Shift,
    Control,
    Meta,
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn introduction_pointer_button_left() -> IntroductionPointerButton {
    IntroductionPointerButton::Left
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn introduction_pointer_button_right() -> IntroductionPointerButton {
    IntroductionPointerButton::Right
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn introduction_orbit_default_modifiers() -> Vec<IntroductionKeyModifier> {
    vec![IntroductionKeyModifier::Alt]
}

/// 👆️ A gesture a demonstration plays: the ghost cursor travels to (or between) `IntroductionPoint`s
/// and performs the visual press/release affordance for the gesture kind.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
// 🐢️ `rename_all_fields` required alongside `rename_all` so `Scroll`'s `delta_y` field actually
// serializes/types as `deltaY` — see `IntroductionPoint`'s comment / `UiDirtyScope`'s for the full story.
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum IntroductionGesture {
    LeftClick {
        at: IntroductionPoint,
    },
    RightClick {
        at: IntroductionPoint,
    },
    DoubleClick {
        at: IntroductionPoint,
    },
    Drag {
        from: IntroductionPoint,
        to: IntroductionPoint,
        #[serde(default = "introduction_pointer_button_left")]
        #[value(default = "introduction_pointer_button_left")]
        button: IntroductionPointerButton,
        #[serde(default)]
        #[value(default)]
        modifiers: Vec<IntroductionKeyModifier>,
    },
    Scroll {
        at: IntroductionPoint,
        delta_y: f64,
    },
    /// 🌐️ A curved (not straight-line) drag around a pivot — camera orbit, distinct from `Drag`'s
    /// straight-line pan/reposition motion.
    Orbit {
        from: IntroductionPoint,
        to: IntroductionPoint,
        #[serde(default = "introduction_pointer_button_right")]
        #[value(default = "introduction_pointer_button_right")]
        button: IntroductionPointerButton,
        #[serde(default = "introduction_orbit_default_modifiers")]
        #[value(default = "introduction_orbit_default_modifiers")]
        modifiers: Vec<IntroductionKeyModifier>,
    },
}

/// 🖱️ Ghost-cursor glyph, mirroring `🎨️ui.css`'s `--cursor-*` custom cursors.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IntroductionCursor {
    #[default]
    Default,
    Pointer,
    Grab,
    Grabbing,
    Crosshair,
    Move,
}

/// 🎬️ A looping ghost-cursor demonstration attached to an interaction-gated
/// `IntroductionStepDefinition`. Plays only while the user's own pointer is idle — any real pointer
/// movement mutes it and restores the real cursor instantly; going idle again while the step is still
/// active replays it from the beginning. `cursor` overrides the glyph shown over the target; omitted, it
/// derives from `gesture` (clicks → pointer, drag → grab/grabbing).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IntroductionDemonstration {
    pub gesture: IntroductionGesture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<IntroductionCursor>,
}

impl IntroductionDemonstration {
    /// 👆️ A left-click demonstration at `at`.
    pub async fn left_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::LeftClick { at }, cursor: None }
    }

    /// 👆️ A right-click demonstration at `at`.
    pub async fn right_click(at: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::RightClick { at }, cursor: None }
    }

    /// ✋️ A click-and-drag demonstration from `from` to `to`.
    pub async fn drag(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::Drag { from, to, button: IntroductionPointerButton::Left, modifiers: vec![] }, cursor: None }
    }

    /// 🖲️ A scroll-wheel demonstration at `at`; `delta_y` sign conveys direction.
    pub async fn scroll(at: IntroductionPoint, delta_y: f64) -> Self {
        Self { gesture: IntroductionGesture::Scroll { at, delta_y }, cursor: None }
    }

    /// 🌐️ A camera-orbit demonstration curving from `from` to `to`.
    pub async fn orbit(from: IntroductionPoint, to: IntroductionPoint) -> Self {
        Self { gesture: IntroductionGesture::Orbit { from, to, button: IntroductionPointerButton::Right, modifiers: vec![IntroductionKeyModifier::Alt] }, cursor: None }
    }
}

//#endregion 🔖️Introduction
//#region 🔖️Tutorial

/// 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of the step-gated
/// `IntroductionDefinition`. Where an introduction gates progression on the user performing an
/// interaction, a tutorial plays a multi-track recording (narration, video overlay, UI state, document
/// edits, camera, ghost-cursor gestures) against a sandboxed copy of the document while the user watches,
/// scrubs, or deviates and converges back. A *recording* IS a `TutorialDefinition` — the recorder simply
/// produces a densely-sampled one; nothing distinguishes a hand-authored tutorial from a captured one.
/// Distinct from the docs-tooltip `tutorial` link field in `ui/js/react`'s `UiLabelLeaf` (a URL into the
/// manual) — this is the interactive playback mechanism.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    /// ⏱️ Total timeline length in milliseconds; every track entry's `at` (+ duration) must fit within.
    pub duration_ms: u64,
    /// 📖️ Scrub-bar markers, sorted ascending by `at`.
    #[serde(default)]
    #[value(default)]
    pub chapters: Vec<TutorialChapter>,
    /// 🎬️ Starting conditions the player restores into its sandbox before t=0.
    pub base: TutorialBase,
    pub tracks: TutorialTracks,
    /// 🧾️ Recorder provenance (ISO 8601 timestamp); `None` means hand-authored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub recorded_at: Option<String>,
}

impl TutorialDefinition {
    /// 📂️ Deserializes a `TutorialDefinition` from its JSON wire format — the constructor apps use
    /// to load a hand-authored or recorded tutorial (e.g. via `include_str!`) into `.tutorial(...)`.
    // 🚧️ BLOCKED: `TutorialDefinition` itself is serde-only (see its docstring) — this stays on
    // `serde_json` until that lands.
    pub async fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// 📖️ One scrub-bar marker in a `TutorialDefinition`'s timeline.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialChapter {
    pub id: String,
    pub at: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<LocalizedLabel>,
}

/// 🎬️ What must be true at t=0: the document the tutorial sandboxes and the initial UI/camera
/// state. The player snapshots the user's live document, loads this in its place, and restores the
/// snapshot on exit — a tutorial can never touch real work.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialBase {
    /// 📂️ Full document DSL text (`ArtifactTextFiles.dsl`) to sandbox-load; `None` falls back to `example_id`, and both
    /// `None` falls back to the app's default/empty document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document_dsl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub example_id: Option<String>,
    pub ui: TutorialUiSnapshot,
    /// 🎥️ Initial camera per window instance (every entry's `at` is `0`).
    #[serde(default)]
    #[value(default)]
    pub cameras: Vec<TutorialCameraKeyframe>,
}

/// 🎞️ The seven parallel tracks of a `TutorialDefinition`'s timeline; every entry's `at` is a
/// millisecond offset from tutorial start, and each `Vec` is sorted ascending by `at`
/// (`validate_tutorial` enforces this).
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialTracks {
    #[serde(default)]
    #[value(default)]
    pub narration: Vec<TutorialNarrationCue>,
    #[serde(default)]
    #[value(default)]
    pub video: Vec<TutorialVideoCue>,
    /// 🏷️ Annotational only — drives affordance pulses and scrub-bar tick marks; playback never
    /// re-dispatches these into a plugin (see `TutorialEventKind`).
    #[serde(default)]
    #[value(default)]
    pub events: Vec<TutorialEvent>,
    #[serde(default)]
    #[value(default)]
    pub ui: Vec<TutorialUiKeyframe>,
    /// 🖋️ The sole source of document mutation during playback — see `TutorialDocumentEventKind`.
    #[serde(default)]
    #[value(default)]
    pub document: Vec<TutorialDocumentEvent>,
    #[serde(default)]
    #[value(default)]
    pub camera: Vec<TutorialCameraKeyframe>,
    #[serde(default)]
    #[value(default)]
    pub gestures: Vec<TutorialGestureCue>,
}

/// 📦️ Where a tutorial media asset's bytes live. `Blob` is wire-identical to `store::BlobRef`
/// (content-addressed Blake3 hash + size + media type) — `framework/core` does not depend on
/// `semio-vcs`, so the shape is mirrored rather than reused; conversion between the two is
/// field-for-field.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialAssetSrc {
    /// 🌐️ Static asset route (a brand's `assetsDir` or the shared `ui/asset` mount).
    Url { url: String },
    /// 🗄️ Content-addressed blob in the studio's `BlobStore`.
    Blob { hash: String, size: u64, media_type: String },
    /// 🧵️ Inline data URL — the recorder's default before a save destination is chosen.
    DataUrl { data: String },
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn tutorial_narration_default_rate() -> f64 {
    1.0
}

// 🚫️async: E4 fn-pointer slot (serde skip_serializing_if)
fn tutorial_rate_is_default(rate: &f64) -> bool {
    (*rate - 1.0).abs() < f64::EPSILON
}

/// 🎙️ One voiceover cue: `text` is both the TTS script and the caption fallback; `audio`
/// overrides TTS with a recorded take. The timeline is always the master clock — a still-speaking TTS
/// utterance is cancelled at the next cue's `at`; audio assets are seeked and rate-matched to the
/// playhead instead of played independently.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialNarrationCue {
    pub id: String,
    pub at: u64,
    /// ⏱️ Audio duration when `audio` is set (recorder-measured); a rough TTS estimate otherwise — used
    /// for scrub-bar layout only, never to gate playback.
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub text: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<TutorialAssetSrc>,
    /// 🗣️ Web Speech API voice-name hint; ignored once `audio` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    /// 🎚️ TTS/audio rate multiplier layered under the player's own playback-rate control.
    #[serde(default = "tutorial_narration_default_rate", skip_serializing_if = "tutorial_rate_is_default")]
    #[value(default = "tutorial_narration_default_rate", skip_serializing_if = "tutorial_rate_is_default")]
    pub rate: f64,
    /// 💬️ Timed caption sub-segments (offsets relative to this cue's `at`); empty means `text` is shown
    /// whole for the cue's `duration_ms`.
    #[serde(default)]
    #[value(default)]
    pub captions: Vec<TutorialCaption>,
}

/// 💬️ One timed caption sub-segment of a `TutorialNarrationCue`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialCaption {
    pub at: u64,
    pub duration_ms: u64,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub text: LocalizedLabel,
}

/// 🖼️ Normalized 0–1 viewport rect for a `TutorialVideoCue` overlay.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialOverlayRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for TutorialOverlayRect {
    /// 📌️ Bottom-right picture-in-picture, ~16:9.
    fn default() -> Self {
        Self { x: 0.72, y: 0.70, width: 0.24, height: 0.24 }
    }
}

/// 📹️ A timed video overlay — e.g. a presenter webcam picture-in-picture, or an authored clip.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialVideoCue {
    pub at: u64,
    pub duration_ms: u64,
    pub src: TutorialAssetSrc,
    #[serde(default)]
    #[value(default)]
    pub rect: TutorialOverlayRect,
    /// 🔇️ True when narration carries the audio (a webcam take recorded muted).
    #[serde(default)]
    #[value(default)]
    pub muted: bool,
    /// ⏩️ Seek offset into the source at cue start.
    #[serde(default)]
    #[value(default)]
    pub source_offset_ms: u64,
}

/// 🏷️ One recorded action/command/keypress, annotational only — see `TutorialTracks::events`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialEvent {
    pub at: u64,
    pub kind: TutorialEventKind,
}

/// 🏷️ What one `TutorialEvent` annotates.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialEventKind {
    /// 📇️ A relative dispatch to an action owned by the active window kind, with its effective args.
    Action {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// 🎛️ A `CommandDefinition` dispatch.
    Command {
        command: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// ⌨️ A keybinding press, display-only over the action it triggered.
    Key { keys: String },
}

/// 🧮️ One UI-state track entry: either a full restore-point snapshot (a valid seek anchor) or a
/// sparse list of changes since the previous sample.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialUiKeyframe {
    pub at: u64,
    pub sample: TutorialUiSample,
}

/// 🧮️ See `TutorialUiKeyframe`.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialUiSample {
    Snapshot { state: Box<TutorialUiSnapshot> },
    Delta { changes: Vec<TutorialUiChange> },
}

/// 🧮️ Renderer-neutral restore point for chrome/UI state — a superset of `ViewModel` plus the
/// dock/panel/dialog state neither shell serializes today. Deliberately NOT a serialization of either
/// shell's internal store: each shell implements its own `captureUiSnapshot`/`applyUiSnapshot` against
/// this shape. Locale/terminology are excluded on purpose — a tutorial plays in the viewer's own locale.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialUiSnapshot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub active_mode_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub focused_window_id: Option<String>,
    /// 🧰️ Mirrors `ViewModel.active_utility_by_window_id`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_utility_by_window_id: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub active_tool_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<WindowLayout>,
    /// 📑️ Active tab id per concrete panel anchor; anchors absent from the map are collapsed/closed.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_panel_tab_by_group: std::collections::HashMap<String, String>,
    /// 🗂️ Opaque program vocabulary, verbatim `ViewModel.panel_json`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
    /// 🕹️ Per-domain selection state, keyed by `InteractionDefinition.id` — the framework-owned
    /// replacement for the deleted opaque `selection_json`; see `TutorialUiChange::Selection`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub interaction_selection: std::collections::HashMap<String, DomainSelection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub open_dialog_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub expanded_tree_ids: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub command_panel_open: bool,
}

/// 🩹️ One typed, sparse UI-state change — the alphabet `compose_tutorial_ui` replays over a prior
/// `TutorialUiSnapshot` to reconstruct state at any timeline offset without shipping a full snapshot at
/// every sample.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialUiChange {
    ActiveMode {
        id: String,
    },
    FocusedWindow {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    /// 🧰️ `utility_id: None` deactivates — mirrors `SetActiveUtility` semantics.
    ActiveUtility {
        window_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        utility_id: Option<String>,
    },
    ActiveTool {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
    },
    Layout {
        layout: WindowLayout,
    },
    /// 📑️ `group` carries the concrete anchor id; `tab_id: None` closes that anchor.
    PanelTab {
        group: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        tab_id: Option<String>,
    },
    PanelState {
        panel_json: String,
    },
    /// 🕹️ Drives one interaction domain's selection during replay — carries the resolved
    /// `DomainSelection` directly rather than re-dispatching `interactionSelect` (a raw pointer/keyboard
    /// event would be non-deterministic on replay). `ids: []` clears the domain's selection.
    Selection {
        domain_id: String,
        granularity: String,
        #[serde(default)]
        #[value(default)]
        ids: Vec<String>,
    },
    Dialog {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    TreeExpansion {
        id: String,
        expanded: bool,
    },
    CommandPanel {
        open: bool,
    },
}

/// 🖋️ One document-track entry — mirrors `store::os_store::store_schema::ArtifactCommand` with `Mutation =
/// serde_json::Value` (opaque per-app mutation JSON, already the wire shape of every `KernelMutation`
/// diff). This is the SOLE source of document mutation during playback: recorded `TutorialEvent`s are
/// annotational only, never re-dispatched, because re-dispatching a plugin action is non-deterministic
/// (fresh ids/timestamps) and would double-apply against this track.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialDocumentEvent {
    pub at: u64,
    pub kind: TutorialDocumentEventKind,
}

/// 🖋️ See `TutorialDocumentEvent`. `Edit` carries both `forwards` and `backwards` operations
/// verbatim from the vcs edit that produced it — the source of exact bidirectional scrubbing.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialDocumentEventKind {
    Edit {
        forwards: Vec<DslValue>,
        backwards: Vec<DslValue>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Undo,
    Redo,
    Checkpoint {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
    CheckoutCheckpoint {
        checkpoint_id: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    /// 📂️ Wholesale document replacement (e.g. a mid-tutorial example switch) — full
    /// `ArtifactEnvelope` JSON in both directions.
    Load {
        document_dsl: String,
        previous_dsl: String,
    },
}

// 🚫️async: E4 fn-pointer slot (serde default)
fn tutorial_camera_up_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

/// 🎥️ One camera track keyframe for a specific window instance.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialCameraKeyframe {
    pub at: u64,
    /// 🪟️ Window *instance* id (matches `ViewWindowInstance.id`).
    pub window_id: String,
    pub camera: TutorialCameraState,
    /// 🪄️ Easing INTO this keyframe from the previous one on the same window.
    #[serde(default)]
    #[value(default)]
    pub easing: TutorialEasing,
}

/// 🎥️ A camera pose — `Orbit` mirrors `World3dScene.camera_json`/`OrbitController`, `Canvas`
/// mirrors `Canvas2dScene`'s `cameraX`/`cameraY`/`zoom`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum TutorialCameraState {
    Orbit {
        position: [f64; 3],
        target: [f64; 3],
        #[serde(default = "tutorial_camera_up_z")]
        #[value(default = "tutorial_camera_up_z")]
        up: [f64; 3],
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        fov: Option<f64>,
    },
    Canvas {
        x: f64,
        y: f64,
        zoom: f64,
    },
}

/// 🪄️ Interpolation curve into a `TutorialCameraKeyframe` from its predecessor on the same window.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum TutorialEasing {
    Linear,
    #[default]
    EaseInOut,
    /// 📌️ No interpolation — hold the previous pose until this keyframe, then snap.
    Hold,
}

/// 👻️ One ghost-cursor gesture cue, reusing the introduction demonstration vocabulary verbatim —
/// both shells already resolve/render `IntroductionGesture`/`IntroductionPoint`/`IntroductionCursor`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TutorialGestureCue {
    pub at: u64,
    pub duration_ms: u64,
    pub gesture: IntroductionGesture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<IntroductionCursor>,
}

/// 🎬️ The reserved action id apps dispatch to (re)start a tutorial — injected by the app builder as a
/// fully shell-intercepted View action into any `AppDefinition` that declares one (mirrors
/// `START_INTRODUCTION_ACTION_ID`). Distinct from an introduction: a tutorial takes a required
/// `tutorialId` argument since an app may declare more than one.
pub const START_TUTORIAL_ACTION_ID: &str = "startTutorial";

/// 🎬️ The reserved `startTutorial` View action: fully shell-intercepted, it sandboxes
/// the live document, loads the selected tutorial's `base`, and starts playback from t=0.
pub fn start_tutorial_action_definition(tutorials: &[TutorialDefinition]) -> ActionDefinition {
    let mut options = Vec::with_capacity(tutorials.len());
    for t in tutorials {
        options.push(ActionArgOption::new(t.id.clone(), t.title.clone()));
    }
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(START_TUTORIAL_ACTION_ID, LocalizedLabel::native("Play Tutorial", "Tutorial abspielen"), ActionKind::View, "eye") }.with_args([ActionArgDef::select(
        "tutorialId",
        LocalizedLabel::native("Tutorial", "Tutorial"),
        options,
    )
    .required()])
}

/// ⏺️ The reserved action id that opens the tutorial recorder chrome — injected by the app builder into
/// EVERY `AppDefinition` (recording needs no app-side declaration at all).
pub const RECORD_TUTORIAL_ACTION_ID: &str = "recordTutorial";

/// ⏺️ The reserved `recordTutorial` View action: fully shell-intercepted, arms the
/// recorder against the live document (never a sandboxed copy — a recording IS the user's work).
pub fn record_tutorial_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(RECORD_TUTORIAL_ACTION_ID, LocalizedLabel::native("Record Tutorial", "Tutorial aufzeichnen"), ActionKind::View, "eye") }
        .describe(LocalizedLabel::native("Opens the tutorial recorder on the open document so the next interactions are recorded as a tutorial.", "Öffnet den Tutorial-Rekorder für das geöffnete Dokument, damit die nächsten Interaktionen als Tutorial aufgezeichnet werden."))
}

/// ⏱️ Real-time (not timeline-time, not rate-scaled) duration of the camera glide the player performs
/// when the user presses Play after deviating from an active tutorial's recorded state.
pub const TUTORIAL_CONVERGE_MS: u64 = 600;

//#region 🔖️TutorialEngine
/// ✅️ Structural validation shared by the plugin builder and both recorders before save: every
/// track sorted ascending by `at`, every entry within `[0, durationMs]`, chapter/narration-cue ids
/// unique, `base.cameras` all at `at == 0`. Does NOT check that referenced action/command/element ids
/// exist — the plugin builder's validation (which has the full `AppDefinition` in scope) does that.
pub fn validate_tutorial(def: &TutorialDefinition) -> Result<(), String> {
    fn sorted_by_at<T>(label: &str, items: &[T], at: impl Fn(&T) -> u64, duration_ms: u64) -> Result<(), String> {
        let mut last: Option<u64> = None;
        for item in items {
            let at = at(item);
            if at > duration_ms {
                return Err(format!("tutorial track `{label}` has an entry at {at}ms beyond durationMs {duration_ms}"));
            }
            if let Some(last) = last {
                if at < last {
                    return Err(format!("tutorial track `{label}` is not sorted ascending by `at` ({last}ms then {at}ms)"));
                }
            }
            last = Some(at);
        }
        Ok(())
    }

    sorted_by_at("chapters", &def.chapters, |c| c.at, def.duration_ms)?;
    sorted_by_at("narration", &def.tracks.narration, |c| c.at, def.duration_ms)?;
    sorted_by_at("video", &def.tracks.video, |c| c.at, def.duration_ms)?;
    sorted_by_at("events", &def.tracks.events, |e| e.at, def.duration_ms)?;
    sorted_by_at("ui", &def.tracks.ui, |k| k.at, def.duration_ms)?;
    sorted_by_at("document", &def.tracks.document, |e| e.at, def.duration_ms)?;
    sorted_by_at("camera", &def.tracks.camera, |k| k.at, def.duration_ms)?;
    sorted_by_at("gestures", &def.tracks.gestures, |c| c.at, def.duration_ms)?;

    let mut chapter_ids = std::collections::HashSet::new();
    for chapter in &def.chapters {
        if !chapter_ids.insert(chapter.id.as_str()) {
            return Err(format!("duplicate tutorial chapter id `{}`", chapter.id));
        }
    }
    let mut cue_ids = std::collections::HashSet::new();
    for cue in &def.tracks.narration {
        if !cue_ids.insert(cue.id.as_str()) {
            return Err(format!("duplicate tutorial narration cue id `{}`", cue.id));
        }
    }
    for camera in &def.base.cameras {
        if camera.at != 0 {
            return Err(format!("tutorial base camera keyframe for window `{}` must have at == 0", camera.window_id));
        }
    }
    Ok(())
}

fn tutorial_ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

fn tutorial_lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// 🎥️ Interpolates between two camera keyframes at timeline offset `at_ms` (clamped into
/// `[prev.at, next.at]`). Position/target/up/fov lerp componentwise; `Canvas.zoom` interpolates in log
/// space so zooming reads as constant visual speed. `next.easing` governs the curve; `Hold` snaps to
/// `prev` until `next.at`, then jumps. Mismatched camera kinds between the two keyframes (`Orbit` vs
/// `Canvas` on the same window) never interpolate — the result snaps to whichever side `t` is closer to.
pub fn interpolate_tutorial_camera(prev: &TutorialCameraKeyframe, next: &TutorialCameraKeyframe, at_ms: f64) -> TutorialCameraState {
    let span = (next.at as f64 - prev.at as f64).max(1.0);
    let raw = ((at_ms - prev.at as f64) / span).clamp(0.0, 1.0);
    let t = match next.easing {
        TutorialEasing::Linear => raw,
        TutorialEasing::EaseInOut => tutorial_ease_in_out(raw),
        TutorialEasing::Hold => {
            if raw >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
    };
    match (&prev.camera, &next.camera) {
        (TutorialCameraState::Orbit { position: p0, target: t0, up: u0, fov: f0 }, TutorialCameraState::Orbit { position: p1, target: t1, up: u1, fov: f1 }) => TutorialCameraState::Orbit {
            position: tutorial_lerp3(*p0, *p1, t),
            target: tutorial_lerp3(*t0, *t1, t),
            up: tutorial_lerp3(*u0, *u1, t),
            fov: match (f0, f1) {
                (Some(a), Some(b)) => Some(a + (b - a) * t),
                (Some(a), None) => Some(*a),
                (None, Some(b)) => Some(*b),
                (None, None) => None,
            },
        },
        (TutorialCameraState::Canvas { x: x0, y: y0, zoom: z0 }, TutorialCameraState::Canvas { x: x1, y: y1, zoom: z1 }) => TutorialCameraState::Canvas { x: x0 + (x1 - x0) * t, y: y0 + (y1 - y0) * t, zoom: (z0.ln() + (z1.ln() - z0.ln()) * t).exp() },
        _ => {
            if t < 0.5 {
                prev.camera.clone()
            } else {
                next.camera.clone()
            }
        }
    }
}

/// 🎥️ Finds the camera pose for `window_id` at `at_ms`: exact if `at_ms` lands on or before the
/// first keyframe (falling back to `base.cameras`), interpolated between the bracketing pair otherwise,
/// held at the last pose past the final keyframe. `None` when the window has no camera keyframes at all.
pub fn tutorial_camera_at(def: &TutorialDefinition, window_id: &str, at_ms: f64) -> Option<TutorialCameraState> {
    let keyframes: Vec<&TutorialCameraKeyframe> = def.base.cameras.iter().chain(def.tracks.camera.iter()).filter(|k| k.window_id == window_id).collect();
    let first = keyframes.first()?;
    if at_ms <= first.at as f64 {
        return Some(first.camera.clone());
    }
    for pair in keyframes.windows(2) {
        let (prev, next) = (pair[0], pair[1]);
        if at_ms <= next.at as f64 {
            return Some(interpolate_tutorial_camera(prev, next, at_ms));
        }
    }
    Some(keyframes.last().unwrap().camera.clone())
}

/// 🩹️ Applies one `TutorialUiChange` onto a `TutorialUiSnapshot` in place — the pure core both
/// `compose_tutorial_ui` and each shell's live director share.
pub fn apply_tutorial_ui_change(state: &mut TutorialUiSnapshot, change: &TutorialUiChange) {
    match change {
        TutorialUiChange::ActiveMode { id } => state.active_mode_id = Some(id.clone()),
        TutorialUiChange::FocusedWindow { id } => state.focused_window_id = id.clone(),
        TutorialUiChange::ActiveUtility { window_id, utility_id } => match utility_id {
            Some(id) => {
                state.active_utility_by_window_id.insert(window_id.clone(), id.clone());
            }
            None => {
                state.active_utility_by_window_id.remove(window_id);
            }
        },
        TutorialUiChange::ActiveTool { id } => state.active_tool_id = id.clone(),
        TutorialUiChange::Layout { layout } => state.layout = Some(layout.clone()),
        TutorialUiChange::PanelTab { group, tab_id } => match tab_id {
            Some(id) => {
                state.active_panel_tab_by_group.insert(group.clone(), id.clone());
            }
            None => {
                state.active_panel_tab_by_group.remove(group);
            }
        },
        TutorialUiChange::PanelState { panel_json } => state.panel_json = Some(panel_json.clone()),
        TutorialUiChange::Selection { domain_id, granularity, ids } => {
            if ids.is_empty() {
                state.interaction_selection.remove(domain_id);
            } else {
                state.interaction_selection.insert(domain_id.clone(), DomainSelection { granularity: granularity.clone(), ids: ids.clone(), anchor_id: None });
            }
        }
        TutorialUiChange::Dialog { id, .. } => state.open_dialog_id = id.clone(),
        TutorialUiChange::TreeExpansion { id, expanded } => {
            if *expanded {
                if !state.expanded_tree_ids.iter().any(|existing| existing == id) {
                    state.expanded_tree_ids.push(id.clone());
                }
            } else {
                state.expanded_tree_ids.retain(|existing| existing != id);
            }
        }
        TutorialUiChange::CommandPanel { open } => state.command_panel_open = *open,
    }
}

/// 🧮️ Reconstructs the full `TutorialUiSnapshot` at `at_ms`: starts from `base.ui`, then the
/// latest `Snapshot` sample with `at <= at_ms` (if any, replacing the base), then replays every `Delta`
/// sample after that snapshot up to and including `at_ms`, in order. This is the one place seeking (and
/// the deviation-then-play converge step) source their target UI state.
pub fn compose_tutorial_ui(def: &TutorialDefinition, at_ms: f64) -> TutorialUiSnapshot {
    let mut state = def.base.ui.clone();
    let mut deltas: Vec<&TutorialUiChange> = Vec::new();
    for keyframe in &def.tracks.ui {
        if keyframe.at as f64 > at_ms {
            break;
        }
        match &keyframe.sample {
            TutorialUiSample::Snapshot { state: snapshot } => {
                state = snapshot.as_ref().clone();
                deltas.clear();
            }
            TutorialUiSample::Delta { changes } => {
                deltas.extend(changes.iter());
            }
        }
    }
    for change in deltas {
        apply_tutorial_ui_change(&mut state, change);
    }
    state
}

/// ✂️ Everything a live director's tick from `from_ms` to `to_ms` must apply: annotational
/// events, document edits, and UI deltas within the half-open interval on the crossing direction (empty
/// when `from_ms == to_ms`). Backward direction (scrubbing left) reverses entry order so callers apply
/// each `TutorialDocumentEventKind::Edit`'s `backwards` ops from most-recent to least-recent. Plain Rust
/// struct (not owned schema mirrored) — the TS port lives in `framework/renderer/react/index.tsx` and is pinned
/// to this one via shared golden fixtures, not a wasm call per frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TutorialSlice {
    pub forward: bool,
    pub events: Vec<TutorialEvent>,
    pub document: Vec<TutorialDocumentEvent>,
    pub ui_changes: Vec<TutorialUiChange>,
}

/// ✂️ Computes the `TutorialSlice` for advancing the playhead from `from_ms` to `to_ms` (`to_ms`
/// may be less than `from_ms` when scrubbing backward).
///
/// 🐢️ A `TutorialUiSample::Snapshot` crossed mid-slice is intentionally NOT flattened into deltas here:
/// recomposing state across a snapshot boundary is exactly what `compose_tutorial_ui` already does
/// correctly and cheaply. This function is for the live per-tick advance, which never spans a snapshot
/// in practice (ticks run far more often than the multi-second snapshot cadence); any caller that jumps
/// across a snapshot boundary (a seek/scrub) should call `compose_tutorial_ui` wholesale instead of
/// accumulating through this slice.
pub fn tutorial_slice(def: &TutorialDefinition, from_ms: f64, to_ms: f64) -> TutorialSlice {
    let forward = to_ms >= from_ms;
    let (lo, hi) = if forward { (from_ms, to_ms) } else { (to_ms, from_ms) };
    let in_range = |at: u64| (at as f64) > lo && (at as f64) <= hi;

    let mut events: Vec<TutorialEvent> = def.tracks.events.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut document: Vec<TutorialDocumentEvent> = def.tracks.document.iter().filter(|e| in_range(e.at)).cloned().collect();
    let mut ui_changes: Vec<TutorialUiChange> = Vec::new();
    for keyframe in def.tracks.ui.iter().filter(|k| in_range(k.at)) {
        if let TutorialUiSample::Delta { changes } = &keyframe.sample {
            ui_changes.extend(changes.iter().cloned());
        }
    }
    if !forward {
        events.reverse();
        document.reverse();
        ui_changes.reverse();
    }
    TutorialSlice { forward, events, document, ui_changes }
}

//#endregion 🔖️TutorialEngine
//#endregion 🔖️Tutorial

#[cfg(test)]
use semio_framework_manifest_address::window_element_id;
#[cfg(test)]
use semio_framework_manifest_arg_schema::ActionArgControl;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
