//! 🕹️ Layout interaction domain: the `"elements"` domain, its snapshot and the select, hover and clear effects.

use semio_framework::kernel::Effect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::CLEAR_SELECTION_ACTION_ID;
use semio_framework_plugin::INTERACTION_HOVER_ACTION_ID;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;

/// 🕹️ The one framework-owned interaction domain layout declares — pages/frames on the Blueprint
/// canvas plus the document tree's frame rows (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
/// Pages are never targets: canvas hit-testing only ever resolves frame ids (`DisplayList::hit_test`
/// only walks `rects`/`images`), so `"elements"` is frames-only despite the document tree also
/// listing pages/spreads/layers/etc.
pub const LAYOUT_INTERACTION_ELEMENTS: &str = "elements";

pub const LAYOUT_GRANULARITY_ELEMENT: &str = "element";

/// 🕹️ Owned snapshot of the framework `"elements"` domain — selection plus `"pointer"` hover — read
/// once per render by `LayoutPlayApp::render_with_request_context` and threaded into the Blueprint
/// canvas chrome and the inspection panel (mirrors `Gis2dInteractionSnapshot`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutInteractionSnapshot {
    pub ids: Vec<String>,
    pub hovered_ids: Vec<String>,
}

impl LayoutInteractionSnapshot {
    pub const POINTER_CHANNEL: &'static str = "pointer";

    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self {
            ids: interaction.selection(LAYOUT_INTERACTION_ELEMENTS).ids.clone(),
            hovered_ids: interaction.hover(LAYOUT_INTERACTION_ELEMENTS, Self::POINTER_CHANNEL).ids.clone(),
        }
    }

    pub fn hovered_id(&self) -> Option<&str> {
        self.hovered_ids.first().map(String::as_str)
    }
}

/// 🕹️ Builds `interactionSelect`'s JSON args for one merge over `ids` (all granularity `"element"`) —
/// shared by the canvas pointer commands (wrapped into a `Effect::DispatchAction`) and any
/// document-tree row whose click should select a real canvas element (wrapped into an `ActionDescriptor`).
pub fn layout_select_action_args(ids: &[String], merge: &str) -> DslValue {
    let targets: Vec<DslValue> = ids.iter().map(|id| semio_framework::dsl_value!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })).collect();
    semio_framework::dsl_value!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "targets": semio_framework_pack_json::to_json_string(&targets), "merge": merge, "method": "pick" })
}

/// 🐁️ Builds `interactionHover`'s JSON args for the `"pointer"` channel — `id: None` clears hover.
pub fn layout_hover_action_args(id: Option<&str>) -> DslValue {
    let targets: Vec<DslValue> = id.map(|id| vec![semio_framework::dsl_value!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })]).unwrap_or_default();
    semio_framework::dsl_value!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "channel": "pointer", "targets": semio_framework_pack_json::to_json_string(&targets) })
}

/// 🕹️ Wraps [`layout_select_action_args`] into the redispatch effect a canvas gesture's own `handle`
/// returns — `dispatch_action` intercepts the six framework interaction verbs BEFORE routing to
/// `ArtifactApp::handle`, so a plain config mutation can no longer express a selection change; the
/// app asks the host to redispatch `interactionSelect` instead (master doc: "surfaces do geometric
/// hit-testing and emit one batched `interactionSelect`").
pub fn layout_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(115), action: INTERACTION_SELECT_ACTION_ID.into(), args: Some(layout_select_action_args(ids, merge)), delay_ms: 0 }
}

/// 🐁️ Wraps [`layout_hover_action_args`] the same way, for `interactionHover`.
pub fn layout_hover_effect(id: Option<&str>) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(114), action: INTERACTION_HOVER_ACTION_ID.into(), args: Some(layout_hover_action_args(id)), delay_ms: 0 }
}

/// 🕹️ Clicking empty canvas clears every domain's selection — `clearSelection` takes no `domainId`.
pub fn layout_clear_selection_effect() -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(113), action: CLEAR_SELECTION_ACTION_ID.into(), args: None, delay_ms: 0 }
}
