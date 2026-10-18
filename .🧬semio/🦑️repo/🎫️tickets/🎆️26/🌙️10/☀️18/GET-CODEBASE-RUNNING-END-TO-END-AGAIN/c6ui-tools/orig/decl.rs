//! 📜️ Renderer-independent UI graph declarations.

pub use semio_framework_ui_layout::{layout, utilities, role_chrome};
pub use semio_framework_icon_contract::IconName;

pub mod ui {
    // #region ui
    //! 🧩 Declarative UI graph types shared by kernel, plugins, and renderers.

    use crate::IconName;
    use semio_framework_ui_locale::Label;
    use semio_framework_value::DslValue;
    use semio_framework_value::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    //#region 🔖Action
    pub use super::layout::{build_shell_context_menu_specs, context_menu_group_label, organize_context_menu, ribbon_parent_label, ShellMenuAction, CONTEXT_MENU_GROUP_ID_PREFIX, CONTEXT_MENU_OVERFLOW_CATEGORY, RIBBON_PARENT_CATEGORIES};
    pub use super::layout::{ActionDescriptor, StyleSpec, UiPeerMark, UiPresence, UiState, UiStatus};
    pub use super::layout::{ContextMenuHit, ContextMenuItemSpec, ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, ContextMenuTextContext, UiMenuRef};
    //#endregion 🔖Action

    //#region 🔖Primitives
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiStackNode {
        pub direction: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub gap: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub padding: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub activate: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub drop_action: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub drop_overlay: Option<UiDropOverlaySpec>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        pub children: Vec<UiNode>,
    }

    /// 📥️ Hover-state copy for a `UiStackNode`'s `drop_overlay`: shown while a drag is over the stack, ahead of `drop_action` firing on release.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiDropOverlaySpec {
        pub title: Label,
        pub hint: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub accept: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiTextNode {
        pub value: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub emphasize: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub data_attributes: Option<HashMap<String, String>>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiButtonNode {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        pub icon_id: IconName,
        pub label: Label,
        pub action: ActionDescriptor,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub style: Option<StyleSpec>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiSeparatorNode {
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiImageNode {
        pub id: String,
        pub src: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub alt: Option<Label>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    /// ⌨️ A retained text, number, date, colour or file field — the contract's `InputProps`.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiInputNode {
        pub id: String,
        pub input_kind: String,
        pub value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub placeholder: Option<Label>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub accessibility_label: Option<Label>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub commit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub accept: Option<String>,
        /// 🎯️ Fraction digits a number field commits — the contract's `InputProps.precision`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub precision: Option<u16>,
        /// 📌️ A number field's detents — the contract's `InputProps.snaps`: its page keys jump between them and a
        /// committed detent is kept off the step ladder.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub snaps: Vec<f64>,
        /// 🪁️ A number field shows and reads `stored × display_factor` — the contract's `InputProps.display_factor`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub display_factor: Option<f64>,
        /// 🚧️ The hard range a typed number must keep — the contract's `InputProps.limits`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub limits: Option<ui_contract::UiNumberLimits>,
        pub on_change: ActionDescriptor,
        /// ⏎️ React's `SearchInput.onSubmit` — the line the user CONFIRMS with Enter, dispatched
        /// through [`ui_contract::Trigger::Submit`]. Distinct from `on_change`, which React fires on
        /// every keystroke of the same field (`🖥️ui/🎯️targets/⚛️react/🟦️.tsx`'s `Search`: `onChange` →
        /// `applyDraft`, `onKeyDown` Enter → `input.onSubmit(draft.trim())`), so a command line that
        /// feeds autocomplete while typing and runs the verb on Enter needs BOTH bound at once.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub on_submit: Option<ActionDescriptor>,
        /// ⎋️ React's `SearchInput.onAbort` — Escape inside the field cancels the active engagement
        /// session and hands the line back to the program ([`ui_contract::Trigger::Abort`]).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub on_abort: Option<ActionDescriptor>,
        /// 🔁️ React's `SearchInput.onRepeatLast` — Space on an EMPTY idle line restarts the last
        /// finalized engagement (`applySearchSpaceAction`, [`ui_contract::Trigger::RepeatLast`]).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub on_repeat_last: Option<ActionDescriptor>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiSelectItem {
        pub value: String,
        pub label: Label,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiSelectNode {
        pub id: String,
        pub value: String,
        pub items: Vec<UiSelectItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub placeholder: Option<Label>,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiToggleNode {
        #[serde(default, skip_serializing_if = "ui_contract::ToggleAppearance::is_button")]
        #[value(default, skip_serializing_if = "ui_contract::ToggleAppearance::is_button")]
        pub appearance: ui_contract::ToggleAppearance,
        pub id: String,
        pub icon_id: IconName,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub text: Option<Label>,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    /** 🌿️ A nestable labeled container of `UiNode` children — the declarative-tree mechanism for
     * subtrees like `Origin > X/Y/Z`: `ui_declarative_child_to_tree_item` expands a `Group` into a
     * `UiTreeItemNode` whose `items` are its recursively-converted children, so depth composes to any
     * level (`Plane > Origin > X/Y/Z`). Unlike `UiSectionNode` (top-level tree sections only, see
     * `assertNoNestedTreeSections` on the TS side), a `Group` may itself appear as another `Group`'s or
     * `UiFieldNode`'s child. */
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiGroupNode {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        pub children: Vec<UiNode>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiKeyValueEntry {
        pub label: Label,
        pub value: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiKeyValueNode {
        pub entries: Vec<UiKeyValueEntry>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    /// 🧿️ A retained slider or dial — the contract's `SliderProps`: values are stored values, the readout, the spoken value
    /// and a typed value are display values (`display_factor`, `precision`, `display_unit` else `unit`), a typed value is
    /// refused by `limits` (the travel itself when absent).
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiSliderNode {
        pub id: String,
        pub value: f64,
        pub min: f64,
        pub max: f64,
        pub step: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        /// 🧲️ Detents — the contract's `SliderProps.snaps`, painted as ticks and applied by the shared law.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub snaps: Vec<f64>,
        /// 🎡️ A straight track or a rotary dial — the contract's `SliderProps.appearance`.
        #[serde(default, skip_serializing_if = "ui_contract::SliderAppearance::is_track")]
        #[value(default, skip_serializing_if = "ui_contract::SliderAppearance::is_track")]
        pub appearance: ui_contract::SliderAppearance,
        /// 📈️ A linear or log axis — the contract's `SliderProps.scale`.
        #[serde(default, skip_serializing_if = "ui_contract::UiNumberScale::is_linear")]
        #[value(default, skip_serializing_if = "ui_contract::UiNumberScale::is_linear")]
        pub scale: ui_contract::UiNumberScale,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub precision: Option<u16>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub display_unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub display_factor: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub limits: Option<ui_contract::UiNumberLimits>,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    impl UiSliderNode {
        /// 🔤️ The readout of a stored `value`: its display text (`stored × display_factor` at `precision`, else twelve digits).
        pub fn readout(&self, value: f64) -> String {
            ui_contract::ui_number_display_text(value, self.display_factor, self.precision)
        }

        /// 🏷️ The unit the slider shows — `display_unit`, else `unit`.
        pub fn shown_unit(&self) -> Option<&str> {
            self.display_unit.as_deref().or(self.unit.as_deref()).filter(|unit| !unit.is_empty())
        }

        /// 🪧️ React's external readout beside a slider with a unit: the declared value's display text and the shown unit.
        pub fn unit_label(&self) -> Option<String> {
            self.shown_unit().map(|unit| format!("{} {unit}", self.readout(self.value)))
        }

        /// 🪐️ The travel's axis — linear or log — as the contract's axis law reads it.
        pub fn axis_position(&self, value: f64) -> f64 {
            ui_contract::slider_axis_position(value, self.min, self.max, self.scale)
        }
    }

    /// 🪜️ A retained number stepper — the contract's `NumberStepperProps`, its detents, shown unit (`display_unit` else
    /// `unit`), display factor and limits included.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiNumberStepperNode {
        pub id: String,
        pub value: f64,
        pub step: f64,
        pub uniform: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        /// 🎯️ Fraction digits it shows and commits — the contract's `NumberStepperProps.precision`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub precision: Option<u16>,
        /// 📍️ Detents its page keys stop on — the contract's `NumberStepperProps.snaps`.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub snaps: Vec<f64>,
        /// 🫧️ The unit shown beside the value (the contract's `display_unit`, else `unit`).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub display_factor: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub limits: Option<ui_contract::UiNumberLimits>,
        pub on_absolute: ActionDescriptor,
        pub on_delta: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    impl UiNumberStepperNode {
        /// 🛰️ The text a stored `value` shows: its display text, and the shown unit when the stepper has one.
        pub fn value_text(&self, value: f64) -> String {
            let text = ui_contract::ui_number_display_text(value, self.display_factor, self.precision);
            self.unit.as_deref().filter(|unit| !unit.is_empty()).map_or_else(|| text.clone(), |unit| format!("{text} {unit}"))
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiRingNode {
        pub id: String,
        pub orb_id: String,
        pub t: f64,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiIconSelectNode {
        pub id: String,
        pub value: String,
        pub uniform: bool,
        pub classifier_kind: String,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    /// 📶️ The retained twin of `ui_contract::Component::Progress`: a read-only bar, determinate on
    /// `0..=total` or an indeterminate sweep while `total` is `None`. `value_text` is the localized spoken
    /// form the accessibility projection announces; paint never invents a percentage string.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiProgressNode {
        pub id: String,
        pub completed: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub total: Option<f64>,
        pub value_text: Label,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(tag = "type", rename_all = "camelCase")]
    #[value(tag = "type", rename_all = "camelCase")]
    pub enum UiControlNode {
        Input(UiInputNode),
        Select(UiSelectNode),
        Toggle(UiToggleNode),
        Button(UiButtonNode),
        KeyValue(UiKeyValueNode),
        Slider(UiSliderNode),
        NumberStepper(UiNumberStepperNode),
        Ring(UiRingNode),
        IconSelect(UiIconSelectNode),
    }

    impl UiControlNode {
        /// 🧭️ Exhaustive accessor — a new control variant fails to compile here until wired.
        /// `&UiPresence` (ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-
        /// CREATION C7.6): `UiPresence` gained `peers: Vec<UiPeerMark>` and lost `Copy`.
        pub fn presence(&self) -> &UiPresence {
            match self {
                UiControlNode::Input(n) => &n.presence,
                UiControlNode::Select(n) => &n.presence,
                UiControlNode::Toggle(n) => &n.presence,
                UiControlNode::Button(n) => &n.presence,
                UiControlNode::KeyValue(n) => &n.presence,
                UiControlNode::Slider(n) => &n.presence,
                UiControlNode::NumberStepper(n) => &n.presence,
                UiControlNode::Ring(n) => &n.presence,
                UiControlNode::IconSelect(n) => &n.presence,
            }
        }
        pub fn presence_mut(&mut self) -> &mut UiPresence {
            match self {
                UiControlNode::Input(n) => &mut n.presence,
                UiControlNode::Select(n) => &mut n.presence,
                UiControlNode::Toggle(n) => &mut n.presence,
                UiControlNode::Button(n) => &mut n.presence,
                UiControlNode::KeyValue(n) => &mut n.presence,
                UiControlNode::Slider(n) => &mut n.presence,
                UiControlNode::NumberStepper(n) => &mut n.presence,
                UiControlNode::Ring(n) => &mut n.presence,
                UiControlNode::IconSelect(n) => &mut n.presence,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiFieldNode {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        pub child: Box<UiNode>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiSectionNode {
        pub id: String,
        // ⚠️ `alias = "title"` is deliberately NOT mirrored into `#[value(...)]` — the first-party
        // derive (`🌱️value/✨️derive`) has no `alias` attribute and an unrecognized key is a hard
        // compile error (see `WindowLayoutWindowNode::active_window_kind_id` and
        // `UiTreeItemNode::icon_id`/`default_open` for the same pattern). `ToValue`/`FromValue`
        // round-trips on the canonical `label` key only; the serde path keeps accepting the legacy
        // `title` alias unchanged.
        #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub label: Option<Label>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        pub children: Vec<UiNode>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub enum UiTreeActionPlacement {
        #[default]
        Row,
        Menu,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiTreeItemAction {
        pub icon_id: IconName,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub label: Option<Label>,
        pub action: ActionDescriptor,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub placement: Option<UiTreeActionPlacement>,
        /// 🚫️ Painted and announced disabled; its gesture dispatches nothing (the contract refuses the verb typed).
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        #[value(default, skip_serializing_if = "std::ops::Not::not")]
        pub disabled: bool,
        /// 💬️ Why a disabled action cannot run, producer-localized — the contract's `RowAction.reason`, which every renderer
        /// announces as the disabled action's description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub reason: Option<Label>,
    }

    impl UiTreeItemAction {
        /** 📍️ Row actions paint on the tree header; menu actions belong in the row context menu. */
        pub fn placement(&self) -> UiTreeActionPlacement {
            self.placement.clone().unwrap_or_default()
        }
    }

    /// 🪟️ The materialised slice of a logically `total`-long tree child list: the node's `items` are
    /// the entries `[offset, offset + items.len())` of that list. `total > 0` with no materialised
    /// items means expandable-but-not-yet-loaded, never "empty" — a renderer pitches the
    /// unmaterialised rows as empty spacer bands so the scrollbar spans the whole document. The
    /// legacy-node mirror of `ui_contract::TreeWindow`; the two are bridged in
    /// `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`. The retained layout pitches the unmaterialised rows as spacers
    /// (`layout::tree_window_spacer_px`), `Ui::tree_window_measures` reads every windowed container against its scroll
    /// viewport, and the one request rule (`🧱️elements/🌳️Tree/🪟️window/🦀️.rs`, React's `treeWindowServedRequestsV1`) asks
    /// back the rows the viewport needs — the same `ViewModel.tree_windows` React's host reports. There is deliberately no
    /// `+N` continuation-row fallback.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub enum UiTreeWindowRowExtent {
        #[default]
        Standard,
        CompactText,
        CompactSmallControl,
        CompactControl,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiTreeWindow {
        pub total: u32,
        pub offset: u32,
        pub row_extent: UiTreeWindowRowExtent,
    }

    impl UiTreeWindow {
        /// 🪟️ Rows of empty pitch a renderer paints BEFORE the materialised children.
        pub fn leading_rows(&self) -> u32 {
            self.offset.min(self.total)
        }

        /// 🪟️ Rows of empty pitch a renderer paints AFTER `materialised` children.
        pub fn trailing_rows(&self, materialised: usize) -> u32 {
            self.total.saturating_sub(self.leading_rows()).saturating_sub(u32::try_from(materialised).unwrap_or(u32::MAX))
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiTreeItemNode {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", alias = "icon")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub icon_id: Option<IconName>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(skip_serializing_if = "Option::is_none", alias = "expanded")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub action: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub actions: Option<Vec<UiTreeItemAction>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub draggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub drag_data: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub items: Option<Vec<UiTreeItemNode>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub control: Option<UiControlNode>,
        /// 🧱️ The lines this row's content fills in its value column when that content is more than one inline
        /// `control` — a recipe group (colour, vector, reference list), text, progress or several nodes — stacked one
        /// line per leaf and label (`reconcile::tree_content_lines`); the content's own document nodes paint there.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub content_lines: Option<u16>,
        /// 🎛️ The explicitly related horizontal Toolbar and its real Button nodes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub inline_toolbar: Option<UiStackNode>,
        /// 🎞️ One semantic engine Surface placed below the row chrome.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub detail: Option<UiComponentSceneNode>,
        /// 👁️ Domain "eye toggle" flag: the row stays visible, dimmed, and clickable (to un-hide) —
        /// this is NOT `presence.state == Hidden`, which means not rendered at all.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub dimmed: Option<bool>,
        /// 🖱️ Row-level context-menu address — most rows share one `menu.id` across a tree with the row
        /// id carried in `args` (e.g. `{"id": row.id}`), rather than minting a unique menu id per row.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        /// 🪟️ The materialised slice of this row's logical child list — see [`UiTreeWindow`]. A row
        /// with `window.total > 0` is expandable even while `items` is empty or absent.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub window: Option<UiTreeWindow>,
        /// 🎯️ The interaction granularity this row picks when the owning tree carries the domain
        /// binding (`UiTreeNode::interaction_domain`), keyed by this row's own `id`. Carried here so a
        /// later wgpu pick path can synthesise the same domain intent React's interpreter already
        /// does; nothing in the wgpu target reads it yet.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub granularity: Option<String>,
        /// 🚦️ The semantic tone this row's label and icon paint (`Theme::tone_ink`) — a history row's outcome, a refused
        /// input: one of the four outcome roles, never the only cue of the state it colours.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub tone: Option<ui_contract::Tone>,
    }

    impl UiTreeItemNode {
        /** 🌳️ Builds a tree item with optional extensions unset. */
        pub fn base(id: impl Into<String>, label: impl Into<Label>) -> Self {
            Self {
                id: id.into(),
                label: label.into(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                content_lines: None,
                inline_toolbar: None,
                detail: None,
                dimmed: None,
                menu: None,
                window: None,
                granularity: None,
                tone: None,
            }
        }

        /// 🗂️ Whether the item discloses rows: materialised `items`, or a tree window over rows not streamed yet
        /// (`window.total > 0` with no children — React's `TreeDataWindow` reads it the same way).
        pub fn has_rows(&self) -> bool {
            self.items.as_deref().is_some_and(|items| !items.is_empty()) || self.window.is_some_and(|window| window.total > 0)
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiTreeSectionNode {
        pub id: String,
        /// 🎛️ Horizontal toolbar rendered inside this section's header.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub header_toolbar: Option<UiStackNode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub label: Option<Label>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        pub items: Vec<UiTreeItemNode>,
        /// 🪟️ The materialised slice of this section's logical child list — see [`UiTreeWindow`].
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub window: Option<UiTreeWindow>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiTreeNode {
        #[serde(default, skip_serializing_if = "ui_contract::TreePresentation::is_standard")]
        #[value(default, skip_serializing_if = "ui_contract::TreePresentation::is_standard")]
        pub presentation: ui_contract::TreePresentation,
        pub sections: Vec<UiTreeSectionNode>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub drop_action: Option<ActionDescriptor>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        /// 🕹️ Binds this rendered tree to an app-declared `InteractionDefinition` domain — the framework
        /// (not the app) then owns the domain's selection/hover via `interactionSelect`/`interactionHover`,
        /// stamped back onto item `presence` by `ui_tree_stamp_presence`. Replaces the deleted per-app
        /// `selected_ids`/`highlighted_ids`/`selection_change` wire surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub interaction_domain: Option<String>,
    }

    /// 🖌️ Stamps `selected`/`previewed`/`color`/`peers` per-item presence across every item in every
    /// section of a tree — the framework-side counterpart of a `UiTreeNode.interaction_domain`
    /// binding. `previewed` wins visually over a plain `selected` item only insofar as both are
    /// representable simultaneously (an item can be selected AND previewed). `own_color` (ticket
    /// 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.6) is stamped onto
    /// every item unconditionally — it names the color THIS session renders its own hover/selection
    /// as, whether or not this particular item is currently marked; `peer_marks_for` resolves an
    /// item's own peer roster by id (called once per item, not pre-collected, since the caller's
    /// `InteractionView::peers_selecting`/`peers_hovering` are themselves per-id lookups).
    pub fn ui_tree_stamp_presence(sections: &mut [UiTreeSectionNode], selected: &std::collections::HashSet<String>, previewed: &std::collections::HashSet<String>, own_color: Option<u8>, peer_marks_for: &dyn Fn(&str) -> Vec<UiPeerMark>) {
        fn stamp_items(items: &mut [UiTreeItemNode], selected: &std::collections::HashSet<String>, previewed: &std::collections::HashSet<String>, own_color: Option<u8>, peer_marks_for: &dyn Fn(&str) -> Vec<UiPeerMark>) {
            for item in items {
                item.presence.selected = selected.contains(&item.id);
                if previewed.contains(&item.id) {
                    item.presence.state = UiState::Previewed;
                }
                item.presence.color = own_color;
                item.presence.peers = peer_marks_for(&item.id);
                if let Some(children) = &mut item.items {
                    stamp_items(children, selected, previewed, own_color, peer_marks_for);
                }
            }
        }
        for section in sections {
            stamp_items(&mut section.items, selected, previewed, own_color, peer_marks_for);
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiInspectorFieldGroup {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        pub fields: Vec<UiNode>,
    }

    pub const UI_INSPECTOR_MIXED_PLACEHOLDER: &str = "Mixed";
    //#endregion 🔖️Primitives

    //#region 🔖️InspectorHelpers
    pub fn ui_inspector_all_equal<T: PartialEq>(values: &[T]) -> bool {
        if values.len() <= 1 {
            return true;
        }
        values.windows(2).all(|pair| pair[0] == pair[1])
    }

    pub struct UiInspectorMixedText {
        pub value: String,
        pub placeholder: Option<String>,
    }

    pub fn ui_inspector_mixed_text(values: &[String]) -> UiInspectorMixedText {
        let uniform = ui_inspector_all_equal(values);
        UiInspectorMixedText { value: if uniform { values.first().cloned().unwrap_or_default() } else { String::new() }, placeholder: if uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) } }
    }

    pub struct UiInspectorMixedNumber {
        pub value: f64,
        pub uniform: bool,
    }

    pub fn ui_inspector_mixed_number(values: &[f64]) -> UiInspectorMixedNumber {
        let uniform = ui_inspector_all_equal(values);
        UiInspectorMixedNumber { value: if uniform { *values.first().unwrap_or(&0.0) } else { f64::NAN }, uniform }
    }

    pub fn ui_inspector_mixed_select(values: &[String]) -> UiInspectorMixedText {
        ui_inspector_mixed_text(values)
    }

    pub struct UiInspectorMixedToggle {
        pub pressed: bool,
        pub uniform: bool,
    }

    pub fn ui_inspector_mixed_toggle(values: &[bool]) -> UiInspectorMixedToggle {
        let uniform = ui_inspector_all_equal(values);
        UiInspectorMixedToggle { pressed: uniform && values.first().copied().unwrap_or(false), uniform }
    }

    pub fn ui_inspector_mixed_slider(values: &[f64]) -> UiInspectorMixedNumber {
        ui_inspector_mixed_number(values)
    }

    pub fn ui_inspector_readonly_field(id: impl Into<String>, label: impl Into<Label>, value: impl Into<String>) -> UiNode {
        let id = id.into();
        UiNode::Field(UiFieldNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::Input(UiInputNode {
                menu: None,
                id,
                input_kind: "text".into(),
                value: value.into(),
                placeholder: None,
                accessibility_label: None,
                commit: None,
                on_change: ActionDescriptor { controller_id: String::new(), action: String::new(), args: None },
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_submit: None,
                on_abort: None,
                on_repeat_last: None,
                presence: UiPresence::default(),
                ..Default::default()
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
        })
    }

    /** 🔢️ Builds an editable number-stepper field row, computing the mixed/uniform display from
     * `values` via {@link ui_inspector_mixed_number}. `action` is cloned into both `onAbsolute` (typed
     * entry, dispatched with `{value}` merged into `args`) and `onDelta` (nudge buttons, `{delta}`) —
     * callers' patch handlers branch on whichever key the dispatched action actually carries. */
    pub fn ui_inspector_stepper_field(id: impl Into<String>, label: impl Into<Label>, values: &[f64], step: f64, action: ActionDescriptor) -> UiNode {
        let id = id.into();
        let mixed = ui_inspector_mixed_number(values);
        UiNode::Field(UiFieldNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::NumberStepper(UiNumberStepperNode { menu: None, id, value: mixed.value, step, uniform: mixed.uniform, min: None, max: None, precision: None, on_absolute: action.clone(), on_delta: action, presence: UiPresence::default(), ..Default::default() })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
        })
    }

    /** 🔘️ Builds an editable boolean toggle field row, computing the mixed/uniform display from
     * `values` via {@link ui_inspector_mixed_toggle}. */
    pub fn ui_inspector_toggle_field(id: impl Into<String>, label: impl Into<Label>, icon_id: impl Into<IconName>, values: &[bool], action: ActionDescriptor) -> UiNode {
        let id = id.into();
        let mixed = ui_inspector_mixed_toggle(values);
        UiNode::Field(UiFieldNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, menu: None, id, icon_id: icon_id.into(), text: None, on_change: action, presence: UiPresence::selected(mixed.pressed) })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
        })
    }

    /** 📐️ Builds a nested `Origin`-style group: a parent tree item labeled `label` containing
     * three {@link ui_inspector_stepper_field} children (`X`/`Y`/`Z`), each computing its own per-axis
     * mixed state independently — a multi-selection that agrees on X but not Y shows only Y as "Mixed".
     * `axis_action(axis)` builds the per-axis `ActionDescriptor`; callers typically merge
     * `{"field": "<id>.x"}` etc. into its `args` so the patch handler can dot-path into the right
     * component with `value` (absolute) or `delta` (relative, offset-preserving across multi-select). */
    pub fn ui_inspector_vec3_group(id: impl Into<String>, label: impl Into<Label>, values: &[[f64; 3]], step: f64, axis_action: impl Fn(&str) -> ActionDescriptor) -> UiNode {
        let id = id.into();
        let xs: Vec<f64> = values.iter().map(|v| v[0]).collect();
        let ys: Vec<f64> = values.iter().map(|v| v[1]).collect();
        let zs: Vec<f64> = values.iter().map(|v| v[2]).collect();
        UiNode::Group(UiGroupNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![
                // 🔤️ Axis symbols (X/Y/Z) are mathematical notation, not translatable UI chrome.
                ui_inspector_stepper_field(format!("{id}.x"), Label::data("X"), &xs, step, axis_action("x")),
                ui_inspector_stepper_field(format!("{id}.y"), Label::data("Y"), &ys, step, axis_action("y")),
                ui_inspector_stepper_field(format!("{id}.z"), Label::data("Z"), &zs, step, axis_action("z")),
            ],
        })
    }

    pub fn ui_inspector_groups_to_tree(groups: &[UiInspectorFieldGroup]) -> UiNode {
        let sections: Vec<UiSectionNode> = groups
            .iter()
            .filter(|group| !group.fields.is_empty())
            .map(|group| UiSectionNode { menu: None, id: group.id.clone(), label: Some(group.label.clone()), default_open: Some(group.default_open.unwrap_or(true)), presence: UiPresence::default(), children: group.fields.clone() })
            .collect();
        ui_declarative_sections_to_tree(&sections)
    }

    pub fn ui_declarative_sections_to_tree(sections: &[UiSectionNode]) -> UiNode {
        let mut tree_sections: Vec<UiTreeSectionNode> = Vec::with_capacity(sections.len());
        for section in sections {
            let mut items = Vec::with_capacity(section.children.len());
            for (index, child) in section.children.iter().enumerate() {
                items.push(ui_declarative_child_to_tree_item(child, format!("{}.{}", section.id, index)));
            }
            tree_sections.push(UiTreeSectionNode { header_toolbar: None, window: None, id: section.id.clone(), label: section.label.clone(), default_open: Some(section.default_open.unwrap_or(true)), presence: section.presence.clone(), items });
        }
        UiNode::Tree(if tree_sections.is_empty() {
            UiTreeNode {
                presentation: Default::default(),
                menu: None,
                sections: vec![UiTreeSectionNode {
                    header_toolbar: None,
                    window: None,
                    id: "empty".into(),
                    label: None,
                    default_open: None,
                    presence: UiPresence::default(),
                    items: vec![UiTreeItemNode {
                        tone: None,
                        window: None,
                        granularity: None,
                        id: "empty".into(),
                        label: Label::data("—"),
                        description: None,
                        icon_id: None,
                        presence: UiPresence::default(),
                        default_open: None,
                        action: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        content_lines: None,
                        inline_toolbar: None,
                        detail: None,
                        dimmed: None,
                        menu: None,
                    }],
                }],
                presence: UiPresence::default(),
                interaction_domain: None,
                drop_action: None,
            }
        } else {
            UiTreeNode { presentation: Default::default(), menu: None, sections: tree_sections, presence: UiPresence::default(), drop_action: None, interaction_domain: None }
        })
    }

    fn ui_declarative_child_to_tree_item(node: &UiNode, fallback_id: String) -> UiTreeItemNode {
        match node {
            UiNode::Text(text) => UiTreeItemNode {
                tone: None,
                window: None,
                granularity: None,
                menu: None,
                id: format!("{}.text", fallback_id),
                label: text.value.clone(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                content_lines: None,
                inline_toolbar: None,
                detail: None,
                dimmed: None,
            },
            UiNode::Field(field) => {
                let description = if let UiNode::Input(input) = field.child.as_ref() { input.placeholder.clone().map(Label::into_string).or_else(|| if input.value.is_empty() { None } else { Some(input.value.clone()) }) } else { None };
                UiTreeItemNode {
                    tone: None,
                    window: None,
                    granularity: None,
                    menu: None,
                    id: field.id.clone(),
                    label: field.label.clone(),
                    description,
                    icon_id: None,
                    presence: UiPresence::default(),
                    default_open: None,
                    action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: ui_node_to_control(&field.child),
                    content_lines: None,
                    inline_toolbar: None,
                    detail: None,
                    dimmed: None,
                }
            }
            UiNode::Button(button) => UiTreeItemNode {
                tone: None,
                window: None,
                granularity: None,
                menu: None,
                id: button.id.clone().unwrap_or(fallback_id),
                label: button.label.clone(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: Some(UiControlNode::Button(button.clone())),
                content_lines: None,
                inline_toolbar: None,
                detail: None,
                dimmed: None,
            },
            UiNode::Input(input) => tree_control_item(input.id.clone(), UiControlNode::Input(input.clone())),
            UiNode::Select(select) => tree_control_item(select.id.clone(), UiControlNode::Select(select.clone())),
            UiNode::Toggle(toggle) => tree_control_item(toggle.id.clone(), UiControlNode::Toggle(toggle.clone())),
            UiNode::Group(group) => {
                let mut items = Vec::with_capacity(group.children.len());
                for (index, child) in group.children.iter().enumerate() {
                    items.push(ui_declarative_child_to_tree_item(child, format!("{}.{}", group.id, index)));
                }
                UiTreeItemNode {
                    tone: None,
                    window: None,
                    granularity: None,
                    menu: None,
                    id: group.id.clone(),
                    label: group.label.clone(),
                    description: None,
                    icon_id: None,
                    presence: UiPresence::default(),
                    default_open: group.default_open,
                    action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: Some(items),
                    control: None,
                    content_lines: None,
                    inline_toolbar: None,
                    detail: None,
                    dimmed: None,
                }
            }
            UiNode::KeyValue(key_value) => tree_control_item(fallback_id, UiControlNode::KeyValue(key_value.clone())),
            UiNode::Slider(slider) => tree_control_item(slider.id.clone(), UiControlNode::Slider(slider.clone())),
            UiNode::NumberStepper(stepper) => tree_control_item(stepper.id.clone(), UiControlNode::NumberStepper(stepper.clone())),
            UiNode::Ring(ring) => tree_control_item(ring.id.clone(), UiControlNode::Ring(ring.clone())),
            UiNode::IconSelect(icon_select) => tree_control_item(icon_select.id.clone(), UiControlNode::IconSelect(icon_select.clone())),
            UiNode::Separator(_) => UiTreeItemNode {
                tone: None,
                window: None,
                granularity: None,
                menu: None,
                id: format!("{}.sep", fallback_id),
                label: Label::data("—"),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                content_lines: None,
                inline_toolbar: None,
                detail: None,
                dimmed: None,
            },
            other => UiTreeItemNode {
                tone: None,
                window: None,
                granularity: None,
                menu: None,
                id: fallback_id,
                label: Label::data(format!("{other:?}")),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                content_lines: None,
                inline_toolbar: None,
                detail: None,
                dimmed: None,
            },
        }
    }

    fn tree_control_item(id: String, control: UiControlNode) -> UiTreeItemNode {
        UiTreeItemNode {
            tone: None,
            window: None,
            granularity: None,
            menu: None,
            id,
            label: Label::data(String::new()),
            description: None,
            icon_id: None,
            presence: UiPresence::default(),
            default_open: None,
            action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: Some(control),
            content_lines: None,
            inline_toolbar: None,
            detail: None,
            dimmed: None,
        }
    }
    //#endregion 🔖️InspectorHelpers

    //#region 🔖️ComponentScenes
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
    pub enum SurfaceKind {
        #[serde(rename = "canvas-2d")]
        #[value(rename = "canvas-2d")]
        Canvas2d,
        #[serde(rename = "world-3d")]
        #[value(rename = "world-3d")]
        World3d,
        #[serde(rename = "node-graph")]
        #[value(rename = "node-graph")]
        NodeGraph,
        #[serde(rename = "text-editor")]
        #[value(rename = "text-editor")]
        TextEditor,
        #[serde(rename = "table")]
        #[value(rename = "table")]
        Table,
        #[serde(rename = "paint-2d")]
        #[value(rename = "paint-2d")]
        Paint2d,
        #[serde(rename = "virtual-file-system")]
        #[value(rename = "virtual-file-system")]
        VirtualFileSystem,
        #[serde(rename = "tiled-map")]
        #[value(rename = "tiled-map")]
        TiledMap,
        #[serde(rename = "board-2d")]
        #[value(rename = "board-2d")]
        Board2d,
        #[serde(rename = "icon-render")]
        #[value(rename = "icon-render")]
        IconRender,
        #[serde(rename = "ink-canvas")]
        #[value(rename = "ink-canvas")]
        InkCanvas,
        #[serde(rename = "graph-timeline")]
        #[value(rename = "graph-timeline")]
        GraphTimeline,
        #[serde(rename = "block-list")]
        #[value(rename = "block-list")]
        BlockList,
        #[serde(rename = "diff-view")]
        #[value(rename = "diff-view")]
        DiffView,
        #[serde(rename = "event-feed")]
        #[value(rename = "event-feed")]
        EventFeed,
    }

    impl SurfaceKind {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::Canvas2d => "canvas-2d",
                Self::World3d => "world-3d",
                Self::NodeGraph => "node-graph",
                Self::TextEditor => "text-editor",
                Self::Table => "table",
                Self::Paint2d => "paint-2d",
                Self::VirtualFileSystem => "virtual-file-system",
                Self::TiledMap => "tiled-map",
                Self::Board2d => "board-2d",
                Self::IconRender => "icon-render",
                Self::InkCanvas => "ink-canvas",
                Self::GraphTimeline => "graph-timeline",
                Self::BlockList => "block-list",
                Self::DiffView => "diff-view",
                Self::EventFeed => "event-feed",
            }
        }

        pub fn is_viewport(self) -> bool {
            matches!(self, Self::World3d | Self::NodeGraph | Self::Canvas2d | Self::Board2d | Self::InkCanvas)
        }
    }

    // 🎬️ The 15 product scene structs (`Canvas2dScene`, `World3dScene`, `NodeGraphScene` + its
    // nested port/node/edge/viewport/operator records, `TextEditorScene`, `TableScene`,
    // `Paint2dScene`, `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene`, `Board2dScene`,
    // `InkCanvasScene`, `GraphTimelineScene`, `DiffViewScene`, `EventFeedScene`, `BlockListScene`)
    // relocated to `semio-framework-ui-scene` — ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
    // RUNTIME packet `scene-surface`. Re-exported here (not a compatibility shim: this crate
    // genuinely still builds `UiNode`s from them, see `build_table_scene` etc. below) so every
    // Scene payload ownership belongs to the independent scene contract.
    // `TableScene::drop_action_json` and `NodeGraphOperatorChannelRecord::default_json` are now
    // opaque JSON strings, not live `ActionDescriptor`/`serde_json::Value` — see the scene crate's
    // own `🦀️scenes.rs` header for why (that crate is wasm-safe and depends on nothing beyond
    // `ui_contract`/`serde`, so it cannot carry either type).
    pub use ui_scene::{
        canvas2d_snapshot_abort_write, canvas2d_snapshot_abort_write_step, canvas2d_snapshot_admit_page, canvas2d_snapshot_begin, canvas2d_snapshot_begin_close, canvas2d_snapshot_close_step, canvas2d_snapshot_seal,
        canvas2d_snapshot_terminal_is_empty, canvas2d_snapshot_with_page, canvas2d_snapshot_write_terminal_is_empty, decode as decode_surface_doc, encode as encode_surface_doc, world3d_snapshot_abort_write, world3d_snapshot_abort_write_step,
        world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_claim_draw_permit, world3d_snapshot_close_step, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty, world3d_snapshot_with_page,
        world3d_snapshot_write_terminal_is_empty, BlockListSelectionTarget, BlockListBlock, BlockListStep, BlockListPaletteEntry, BlockListScene, Board2dScene, Canvas2dFraming, Canvas2dRejectedSnapshotPage, Canvas2dScene, Canvas2dSnapshotDescriptor, Canvas2dSnapshotFault, Canvas2dSnapshotLease, Canvas2dSnapshotPage,
        Canvas2dSnapshotWriteToken, DiffViewScene, EventFeedScene, GraphTimelineScene, IconRenderScene, InkCanvasInteractionDomain, InkCanvasScene, NodeGraphEdgeRecord, NodeGraphFindItem, NodeGraphHover, NodeGraphInteractionDomain,
        NodeGraphNodeRecord, NodeGraphOperatorChannelRecord, NodeGraphOperatorRecord, NodeGraphOperatorVariadicRecord, NodeGraphPortRecord, NodeGraphScene, Paint2dScene, SceneDoc, TableScene, TextEditorScene, TextSplice, TextSpliceComposition, LocatedTextSplice, AppliedTextSplice, DraftChange, DRAFT_SPLICES_ARGUMENT, compose_draft_step, write_draft_changes_json_into, DraftChangesJsonCursor, TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_IDLE_MS, TEXT_SPLICE_CONTEXT_SCALARS, TiledMapScene, VirtualFileSystemScene,
        scene_lane_hash, world3d_modelling_lane_text, world3d_modelling_strings, world3d_sub_element_paint, project_world3d_annotations, format_world3d_legend_value, World3dAnnotation, World3dAnnotationLayer, World3dColorRamp, World3dHighlight, World3dLabelAlign,
        World3dLegendLines, World3dMarkerShape, World3dModellingOptions, World3dModellingStrings, World3dPickGranularity, World3dProjectedAnnotation, World3dScalarDomain, World3dScalarField, World3dSection, World3dSubElementDefaults, World3dText, World3dTone,
        WORLD3D_ANNOTATION_ARROW_PX, WORLD3D_ANNOTATION_LABEL_GAP_PX, WORLD3D_ANNOTATION_MARKER_PX, WORLD3D_ANNOTATION_STROKE_PX, WORLD3D_SCALAR_NO_DATA_RGB, World3dProjectedArc,
        World3dPresentation, World3dPresentationClear, World3dRejectedSnapshotPage, World3dScene, World3dSnapshotDescriptor, World3dSnapshotDrawPermit, World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage,
        World3dSnapshotPageKind, World3dSnapshotSpan, World3dSnapshotWriteToken, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY,
    };

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WorldMeshLodEntry {
        pub lod: f64,
        pub url: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WorldLodRecord {
        #[serde(default = "default_true")]
        #[value(default = "default_true")]
        pub automatic: bool,
        #[serde(default = "default_manual_lod")]
        #[value(default = "default_manual_lod")]
        pub manual: f64,
        #[serde(default = "default_distance_reference")]
        #[value(default = "default_distance_reference")]
        pub distance_reference: f64,
        #[serde(default)]
        #[value(default)]
        pub depth_variable: bool,
        #[serde(default = "default_grid_factor")]
        #[value(default = "default_grid_factor")]
        pub grid_factor: f64,
        #[serde(default)]
        #[value(default)]
        pub grid_snap_enabled: bool,
        #[serde(default = "default_true")]
        #[value(default = "default_true")]
        pub show_grid: bool,
        #[serde(default)]
        #[value(default)]
        pub grid_datum: Option<[f64; 3]>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WorldChunkingRecord {
        pub chunk_size: f64,
        pub max_distance: f64,
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_manual_lod() -> f64 {
        100.0
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_distance_reference() -> f64 {
        100.0
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_grid_factor() -> f64 {
        10.0
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_true() -> bool {
        true
    }

    pub fn world3d_default_lod_json() -> String {
        serde_json::json!({
            "automatic": true,
            "manual": 100.0,
            "distanceReference": 100.0,
            "depthVariable": false,
            "gridFactor": 10.0,
            "gridSnapEnabled": false,
            "showGrid": true,
            "gridDatum": [0.0, 0.0, 0.0],
        })
        .to_string()
    }

    pub fn world3d_chunking_json(chunk_size: f64, max_distance: f64) -> String {
        serde_json::json!({
            "chunkSize": chunk_size,
            "maxDistance": max_distance,
        })
        .to_string()
    }

    /// 🎯️ The world-3d `fit` lane: ask the render host to frame the scene's own bounds ONCE per
    /// `revision`, preserving the current view direction.
    ///
    /// `revision` is a producer-owned document identity, not a frame counter: `WorldAutoFit`
    /// (`📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost`) keys its one-shot on
    /// `${revision}:${meshes}` and never refits again while that key holds, so a producer that
    /// derives the revision from what the DOCUMENT is (its catalogs, its domain) refits exactly on a
    /// document swap, while a producer that derived it from the geometry would yank the camera on
    /// every edit. Without a fit lane the camera after a document swap is whatever the previous
    /// document left behind — a fixture centred elsewhere is simply off-screen (ticket 26/09/02 W-P5
    /// §7, W-S2).
    /// 📦️ `bounds` is the producer's OWN delivered extent, published rather than rediscovered: a
    /// render host that measures its scene graph instead can only frame what it has already
    /// uploaded, and the wgpu surface has no scene graph to measure at all — so without this lane
    /// the two renderers frame differently by construction
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
    pub fn world3d_fit_json(revision: u32, padding: f64, bounds: Option<([f64; 3], [f64; 3])>) -> String {
        match bounds {
            Some((minimum, maximum)) => serde_json::json!({ "enabled": true, "revision": revision, "padding": padding, "boundsMin": minimum, "boundsMax": maximum }).to_string(),
            None => serde_json::json!({ "enabled": true, "revision": revision, "padding": padding }).to_string(),
        }
    }

    // 🎬️ `world3d_default_selection_json`/`world3d_default_meshes_json` moved with `World3dScene`
    // itself into `semio-framework-ui-scene` (they back its `#[serde(default = ...)]` fields) — no
    // longer defined here; nothing else in this file called them directly.

    pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
        serde_json::json!({
            "position": position,
            "target": target,
            "up": [0.0, 0.0, 1.0],
            "fov": fov,
        })
        .to_string()
    }

    // 🎬️ `NodeGraphRecords` (`NodeGraphPortRecord`/`NodeGraphNodeRecord`/`NodeGraphEdgeRecord`/
    // `NodeGraphFindItem`/`NodeGraphHover`/`NodeGraphOperator*Record`) and
    // `NodeGraphScene` itself moved to `semio-framework-ui-scene` together (see the re-export above)
    // — the latter's fields are typed directly against the former, so they moved as one unit.

    // 🎬️ `TextEditorScene` and `TableScene` (+ its `base()`) moved to `semio-framework-ui-scene`
    // (see the re-export above). `TableScene.drop_action` is `drop_action_json: Option<String>` on
    // the moved type — it named `ActionDescriptor` (this crate's own type, defined just below via
    // `UiTreeItemAction`'s sibling), which the wasm-safe scene crate cannot depend on.
    //
    // `TableCell` stays here; `TableScene` receives its rows through the scene wire string.

    //#region 🔖️TableCells
    /// 🧾️ A typed table cell value: plain text/number, or an interactive stepper/button group.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum TableCell {
        Text { value: String },
        Number { value: f64 },
        EditableText { value: String, action: ActionDescriptor },
        Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
        Buttons { buttons: Vec<UiTreeItemAction> },
    }

    //#endregion 🔖️TableCells

    /* 🖼️ Paint-2d scene: WASM `RasterSession` sync channels for the composite/navigator windows, see framework/surface/paint/rs/lib.rs. */
    // 🎬️ `Paint2dScene`, `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene` (+ its default
    // fns and `base()`), `Board2dScene` (+ its default fns and `base()`), `InkCanvasScene` (+ its
    // default fn and `base()`), `GraphTimelineScene`, `DiffViewScene`, and `EventFeedScene` all moved
    // to `semio-framework-ui-scene` (see the re-export above).

    /** 🧩️ A palette entry for a block kind insertable into a [`BlockListScene`], contributed
     * either by the host app's own built-ins or by a `"playbook.blockKind"` topic contribution. */
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct BlockPaletteEntry {
        pub block_kind: String,
        pub label: String,
        pub icon_id: IconName,
    }

    // 🎬️ `BlockListScene` itself moved to `semio-framework-ui-scene` (see the re-export above);
    // `BlockPaletteEntry` above is the host palette contribution projected into scene entries.

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiExternalSlotNode {
        pub plugin_id: String,
        pub app_id: String,
        pub body_key: String,
        pub params_json: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub host_status: Option<String>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiComponentSceneNode {
        /// 🪪️ Mounted component identity for native hosts; the document remains the action address.
        #[serde(skip)]
        #[value(skip)]
        pub host_id: String,
        pub surface_id: String,
        pub controller_id: String,
        pub component_kind: SurfaceKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub pane_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub binding_id: Option<String>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        #[value(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        /// 🖱️ Optional override of the implicit per-`component_kind` convention id (`"world3d"`,
        /// `"nodeGraph"`, `"tiledMap"`, ...) the host uses when resolving which surface answers a
        /// right-click — set only when a plugin needs a menu id other than the surface-kind default.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub canvas_2d: Option<Canvas2dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub world_3d: Option<World3dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub node_graph: Option<NodeGraphScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub text_editor: Option<TextEditorScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub table: Option<TableScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub paint_2d: Option<Paint2dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub virtual_file_system: Option<VirtualFileSystemScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub tiled_map: Option<TiledMapScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub board2d: Option<Board2dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub icon_render: Option<IconRenderScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub ink_canvas: Option<InkCanvasScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub graph_timeline: Option<GraphTimelineScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub block_list: Option<BlockListScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub diff_view: Option<DiffViewScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub event_feed: Option<EventFeedScene>,
    }
    //#endregion 🔖️ComponentScenes

    //#region 🔖️UiNode
    /// ⚠️ `ComponentScene`'s payload dwarfs the other variants (nested scene JSON blobs) — boxing it
    /// would be a breaking public-API change (every construction/match site across ~30 plugins would
    /// need `Box::new`/deref updates), out of scope for a mechanical pass.
    #[allow(clippy::large_enum_variant, reason = "boxing is a breaking public API change, out of T1 scope")]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(tag = "type", rename_all = "camelCase")]
    #[value(tag = "type", rename_all = "camelCase")]
    pub enum UiNode {
        Stack(UiStackNode),
        Text(UiTextNode),
        Button(UiButtonNode),
        Separator(UiSeparatorNode),
        Input(UiInputNode),
        Select(UiSelectNode),
        Toggle(UiToggleNode),
        KeyValue(UiKeyValueNode),
        Slider(UiSliderNode),
        NumberStepper(UiNumberStepperNode),
        Ring(UiRingNode),
        IconSelect(UiIconSelectNode),
        Progress(UiProgressNode),
        Field(UiFieldNode),
        Section(UiSectionNode),
        Group(UiGroupNode),
        Tree(UiTreeNode),
        Image(UiImageNode),
        ComponentScene(UiComponentSceneNode),
        ExternalSlot(UiExternalSlotNode),
    }

    impl UiNode {
        /// 🧭️ Exhaustive presence accessor — adding a `UiNode` variant fails to compile here (and in
        /// `presence_mut` and `paint_node`'s match) until the new element's `presence` field is wired up.
        pub fn presence(&self) -> &UiPresence {
            match self {
                UiNode::Stack(n) => &n.presence,
                UiNode::Text(n) => &n.presence,
                UiNode::Button(n) => &n.presence,
                UiNode::Separator(n) => &n.presence,
                UiNode::Input(n) => &n.presence,
                UiNode::Select(n) => &n.presence,
                UiNode::Toggle(n) => &n.presence,
                UiNode::KeyValue(n) => &n.presence,
                UiNode::Slider(n) => &n.presence,
                UiNode::NumberStepper(n) => &n.presence,
                UiNode::Ring(n) => &n.presence,
                UiNode::IconSelect(n) => &n.presence,
                UiNode::Progress(n) => &n.presence,
                UiNode::Field(n) => &n.presence,
                UiNode::Section(n) => &n.presence,
                UiNode::Group(n) => &n.presence,
                UiNode::Tree(n) => &n.presence,
                UiNode::Image(n) => &n.presence,
                UiNode::ComponentScene(n) => &n.presence,
                UiNode::ExternalSlot(n) => &n.presence,
            }
        }
        pub fn presence_mut(&mut self) -> &mut UiPresence {
            match self {
                UiNode::Stack(n) => &mut n.presence,
                UiNode::Text(n) => &mut n.presence,
                UiNode::Button(n) => &mut n.presence,
                UiNode::Separator(n) => &mut n.presence,
                UiNode::Input(n) => &mut n.presence,
                UiNode::Select(n) => &mut n.presence,
                UiNode::Toggle(n) => &mut n.presence,
                UiNode::KeyValue(n) => &mut n.presence,
                UiNode::Slider(n) => &mut n.presence,
                UiNode::NumberStepper(n) => &mut n.presence,
                UiNode::Ring(n) => &mut n.presence,
                UiNode::IconSelect(n) => &mut n.presence,
                UiNode::Progress(n) => &mut n.presence,
                UiNode::Field(n) => &mut n.presence,
                UiNode::Section(n) => &mut n.presence,
                UiNode::Group(n) => &mut n.presence,
                UiNode::Tree(n) => &mut n.presence,
                UiNode::Image(n) => &mut n.presence,
                UiNode::ComponentScene(n) => &mut n.presence,
                UiNode::ExternalSlot(n) => &mut n.presence,
            }
        }
        /// 🖱️ Exhaustive context-menu-ref accessor — adding a `UiNode` variant fails to compile here
        /// (and in `menu_mut`) until the new element's `menu` field is wired up. `None` means the
        /// element bubbles right-clicks to its nearest menu-bearing ancestor.
        pub fn menu(&self) -> Option<&UiMenuRef> {
            match self {
                UiNode::Stack(n) => n.menu.as_ref(),
                UiNode::Text(n) => n.menu.as_ref(),
                UiNode::Button(n) => n.menu.as_ref(),
                UiNode::Separator(n) => n.menu.as_ref(),
                UiNode::Input(n) => n.menu.as_ref(),
                UiNode::Select(n) => n.menu.as_ref(),
                UiNode::Toggle(n) => n.menu.as_ref(),
                UiNode::KeyValue(n) => n.menu.as_ref(),
                UiNode::Slider(n) => n.menu.as_ref(),
                UiNode::NumberStepper(n) => n.menu.as_ref(),
                UiNode::Ring(n) => n.menu.as_ref(),
                UiNode::IconSelect(n) => n.menu.as_ref(),
                UiNode::Progress(n) => n.menu.as_ref(),
                UiNode::Field(n) => n.menu.as_ref(),
                UiNode::Section(n) => n.menu.as_ref(),
                UiNode::Group(n) => n.menu.as_ref(),
                UiNode::Tree(n) => n.menu.as_ref(),
                UiNode::Image(n) => n.menu.as_ref(),
                UiNode::ComponentScene(n) => n.menu.as_ref(),
                UiNode::ExternalSlot(n) => n.menu.as_ref(),
            }
        }
        pub fn menu_mut(&mut self) -> &mut Option<UiMenuRef> {
            match self {
                UiNode::Stack(n) => &mut n.menu,
                UiNode::Text(n) => &mut n.menu,
                UiNode::Button(n) => &mut n.menu,
                UiNode::Separator(n) => &mut n.menu,
                UiNode::Input(n) => &mut n.menu,
                UiNode::Select(n) => &mut n.menu,
                UiNode::Toggle(n) => &mut n.menu,
                UiNode::KeyValue(n) => &mut n.menu,
                UiNode::Slider(n) => &mut n.menu,
                UiNode::NumberStepper(n) => &mut n.menu,
                UiNode::Ring(n) => &mut n.menu,
                UiNode::IconSelect(n) => &mut n.menu,
                UiNode::Progress(n) => &mut n.menu,
                UiNode::Field(n) => &mut n.menu,
                UiNode::Section(n) => &mut n.menu,
                UiNode::Group(n) => &mut n.menu,
                UiNode::Tree(n) => &mut n.menu,
                UiNode::Image(n) => &mut n.menu,
                UiNode::ComponentScene(n) => &mut n.menu,
                UiNode::ExternalSlot(n) => &mut n.menu,
            }
        }
    }

    // 🎬️ `NodeGraphScene::base` and `TextEditorScene::base` moved into `semio-framework-ui-scene`
    // itself (inherent impls on a foreign type are an orphan-rule error, not just a style choice —
    // see R9/E116). `json_view`/`code_input` stay here as free functions: both embed a `ui_wgpu`-only
    // `ActionDescriptor` into `settings_json` via `serde_json`, so they legitimately belong on this
    // side of the boundary the same way `TableCell`/`table_row_json` do.

    /** 📖️ Builds a read-only JSON viewer text-editor scene: a pretty-printed JSON buffer,
     * `"json"` language, and `settingsJson` set to `{"readOnly":true}`. */
    pub fn text_editor_json_view(json_pretty: String) -> TextEditorScene {
        let mut scene = TextEditorScene::base(json_pretty, Some("json".into()), None);
        scene.settings_json = Some(serde_json::json!({ "readOnly": true }).to_string());
        scene
    }

    /** ⌨️ Builds an editable code-input text-editor scene wired to a host settings-change
     * action: `settingsJson` carries `{"readOnly":false,"onEditSettings":<ActionDescriptor>}`, fired
     * by the renderer when the user edits editor settings (font size, tab width, ...) via its own
     * chrome. */
    pub fn text_editor_code_input(buffer: String, language: &str, on_edit_settings: &ActionDescriptor) -> TextEditorScene {
        let mut scene = TextEditorScene::base(buffer, Some(language.into()), None);
        scene.settings_json = Some(serde_json::json!({ "readOnly": false, "onEditSettings": on_edit_settings }).to_string());
        scene
    }

    //#region 🔖️SceneActions
    /** 🎮️ Renderer-to-plugin action names for node-graph surfaces. */
    pub mod node_graph_actions {
        pub const SELECT: &str = "nodeGraphSelect";
        pub const HOVER: &str = "nodeGraphHover";
        pub const EDIT: &str = "nodeGraphEdit";
        pub const VIEWPORT: &str = "nodeGraphViewport";
        pub const SPOTLIGHT_COMMIT: &str = "spotlightCommit";
    }

    /** ✍️ Renderer-to-plugin action names for text-editor surfaces. */
    pub mod text_editor_actions {
        pub const EDIT: &str = "textEdit";
        pub const SELECT: &str = "textSelect";
        pub const HOVER: &str = "textHover";
        pub const REQUEST_COMPLETIONS: &str = "requestCompletions";
        pub const COMMIT_RENAME: &str = "commitRename";
        pub const FORMAT_DOCUMENT: &str = "formatDocument";
    }

    /** 🧩️ Renderer-to-plugin action names for 2D board surfaces. */
    pub mod board2d_actions {
        pub const APPLY_BOARD_EVENTS: &str = "applyBoardEvents";
    }

    /** 🖊️ Renderer-to-plugin action names for ink canvas surfaces. */
    pub mod ink_canvas_actions {
        pub const APPLY_EVENTS: &str = "inkApplyEvents";
    }

    /** 🗺️ Renderer-to-plugin action names for tiled map surfaces. */
    pub mod tiled_map_actions {
        pub const SET_CAMERA: &str = "setCamera";
        pub const SET_FEATURE_SELECTION: &str = "setFeatureSelection";
        pub const SET_HOVER: &str = "setHover";
        pub const SET_SELECTION_METHOD: &str = "setSelectionMethod";
        pub const SET_SELECTION_MODE: &str = "setSelectionMode";
        pub const CLEAR_SELECTION: &str = "clearSelection";
        pub const SELECT_ALL: &str = "selectAll";
        pub const DESELECT: &str = "deselect";
        pub const FOCUS_FEATURE: &str = "focusFeature";
        pub const OPEN_SOURCE: &str = "openSource";
        pub const SET_LAYER_STROKE_SCALE: &str = "setLayerStrokeScale";
        pub const FIT_WORLD: &str = "fitWorld";
    }
    //#endregion 🔖️SceneActions

    pub fn ui_stack_vertical(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: Some("standard".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, children, drop_action: None, drop_overlay: None })
    }

    /** 🖼️ Builds an image node rendering a source URL or path. */
    pub fn ui_image(id: impl Into<String>, src: impl Into<String>, alt: Option<Label>) -> UiNode {
        UiNode::Image(UiImageNode { menu: None, id: id.into(), src: src.into(), alt, presence: UiPresence::default() })
    }

    /** 🎛️ Extracts the control payload of a {@link UiNode} when it is a control variant. */
    pub fn ui_node_to_control(node: &UiNode) -> Option<UiControlNode> {
        match node {
            UiNode::Input(input) => Some(UiControlNode::Input(input.clone())),
            UiNode::Select(select) => Some(UiControlNode::Select(select.clone())),
            UiNode::Toggle(toggle) => Some(UiControlNode::Toggle(toggle.clone())),
            UiNode::Button(button) => Some(UiControlNode::Button(button.clone())),
            UiNode::KeyValue(key_value) => Some(UiControlNode::KeyValue(key_value.clone())),
            UiNode::Slider(slider) => Some(UiControlNode::Slider(slider.clone())),
            UiNode::NumberStepper(stepper) => Some(UiControlNode::NumberStepper(stepper.clone())),
            UiNode::Ring(ring) => Some(UiControlNode::Ring(ring.clone())),
            UiNode::IconSelect(icon) => Some(UiControlNode::IconSelect(icon.clone())),
            _ => None,
        }
    }

    /** 🎛️ Wraps a {@link UiControlNode} back into its matching {@link UiNode} control variant (inverse of {@link ui_node_to_control}). */
    pub fn ui_control_to_node(control: UiControlNode) -> UiNode {
        match control {
            UiControlNode::Input(input) => UiNode::Input(input),
            UiControlNode::Select(select) => UiNode::Select(select),
            UiControlNode::Toggle(toggle) => UiNode::Toggle(toggle),
            UiControlNode::Button(button) => UiNode::Button(button),
            UiControlNode::KeyValue(key_value) => UiNode::KeyValue(key_value),
            UiControlNode::Slider(slider) => UiNode::Slider(slider),
            UiControlNode::NumberStepper(stepper) => UiNode::NumberStepper(stepper),
            UiControlNode::Ring(ring) => UiNode::Ring(ring),
            UiControlNode::IconSelect(icon) => UiNode::IconSelect(icon),
        }
    }

    impl Default for UiNode {
        // 🚫️async: E1 impl of external trait (Default) — inlines `ui_stack_vertical`'s pure body rather
        // than asyncifying that widely-async-consumed builder just for this one sync call site — see R9
        fn default() -> Self {
            UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: Some("standard".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, children: vec![], drop_action: None, drop_overlay: None })
        }
    }

    pub fn ui_text(value: impl Into<Label>) -> UiNode {
        UiNode::Text(UiTextNode { menu: None, value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default() })
    }

    /** 🔌️ Renders a contributing program body inline at this tree position. */
    pub fn ui_external_slot(plugin_id: impl Into<String>, app_id: impl Into<String>, body_key: impl Into<String>, params_json: impl Into<String>) -> UiNode {
        UiNode::ExternalSlot(UiExternalSlotNode { menu: None, plugin_id: plugin_id.into(), app_id: app_id.into(), body_key: body_key.into(), params_json: params_json.into(), host_status: None, presence: UiPresence::default() })
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per scene-kind payload; grouping into a struct is a T2 restructure, out of scope")]
    fn component_scene(
        surface_id: impl Into<String>,
        controller_id: impl Into<String>,
        component_kind: SurfaceKind,
        pane_id: Option<String>,
        binding_id: Option<String>,
        canvas_2d: Option<Canvas2dScene>,
        world_3d: Option<World3dScene>,
        node_graph: Option<NodeGraphScene>,
        text_editor: Option<TextEditorScene>,
        table: Option<TableScene>,
        paint_2d: Option<Paint2dScene>,
        virtual_file_system: Option<VirtualFileSystemScene>,
        tiled_map: Option<TiledMapScene>,
        board2d: Option<Board2dScene>,
    ) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            menu: None,
            host_id: String::new(),
            surface_id: surface_id.into(),
            controller_id: controller_id.into(),
            component_kind,
            pane_id,
            binding_id,
            presence: UiPresence::default(),
            canvas_2d,
            world_3d,
            node_graph,
            text_editor,
            table,
            paint_2d,
            virtual_file_system,
            tiled_map,
            board2d,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
        })
    }

    pub fn build_canvas_2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Canvas2dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Canvas2d, None, None, Some(scene), None, None, None, None, None, None, None, None)
    }

    pub fn build_world_3d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: World3dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::World3d, None, None, None, Some(scene), None, None, None, None, None, None, None)
    }

    pub fn build_node_graph_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: NodeGraphScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::NodeGraph, None, None, None, None, Some(scene), None, None, None, None, None, None)
    }

    pub fn build_text_editor_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TextEditorScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::TextEditor, None, None, None, None, None, Some(scene), None, None, None, None, None)
    }

    //#region 🔖️TextIdentifierOccurrences
    /// 🔎️ Expands an offset in `text` to the bounds of the identifier (`[A-Za-z0-9_]+`) it falls in, if any.
    pub fn text_identifier_bounds_at(text: &str, offset: usize) -> Option<(usize, usize)> {
        let bytes = text.as_bytes();
        let is_ident = |byte: u8| (byte as char).is_ascii_alphanumeric() || byte == b'_';
        let mut index = offset.min(bytes.len());
        while index > 0 && is_ident(bytes[index - 1]) {
            index -= 1;
        }
        let start = index;
        while index < bytes.len() && is_ident(bytes[index]) {
            index += 1;
        }
        if start == index {
            None
        } else {
            Some((start, index))
        }
    }

    /// 🔎️ JSON `{selection, hover}` occurrence ranges for the identifier under `cursor`, for editor cross-highlighting.
    pub fn text_identifier_occurrences_json(text: &str, cursor: usize) -> Option<String> {
        let (start, end) = text_identifier_bounds_at(text, cursor)?;
        let needle = &text[start..end];
        if needle.is_empty() {
            return None;
        }
        let mut ranges = Vec::new();
        let mut scan = 0usize;
        while let Some(found) = text[scan..].find(needle) {
            let at = scan + found;
            let next_end = at + needle.len();
            if text_identifier_bounds_at(text, at) == Some((at, next_end)) {
                ranges.push(serde_json::json!({ "start": at, "end": next_end }));
            }
            scan = at + needle.len();
        }
        let ranges_json = serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into());
        Some(serde_json::json!({ "selection": ranges_json, "hover": ranges_json }).to_string())
    }
    //#endregion 🔖️TextIdentifierOccurrences

    pub fn build_table_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TableScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Table, None, None, None, None, None, None, Some(scene), None, None, None, None)
    }

    pub fn build_paint_2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Paint2dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Paint2d, None, None, None, None, None, None, None, Some(scene), None, None, None)
    }

    pub fn build_virtual_file_system_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: VirtualFileSystemScene, pane_id: Option<String>, binding_id: Option<String>) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::VirtualFileSystem, pane_id, binding_id, None, None, None, None, None, None, Some(scene), None, None)
    }

    pub fn build_tiled_map_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TiledMapScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::TiledMap, None, None, None, None, None, None, None, None, None, Some(scene), None)
    }

    pub fn build_board2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Board2dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Board2d, None, None, None, None, None, None, None, None, None, None, Some(scene))
    }

    pub fn build_icon_render_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: IconRenderScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::IconRender, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { icon_render: Some(scene), ..node })
    }

    pub fn build_ink_canvas_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: InkCanvasScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::InkCanvas, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { ink_canvas: Some(scene), ..node })
    }

    pub fn build_graph_timeline_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: GraphTimelineScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::GraphTimeline, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { graph_timeline: Some(scene), ..node })
    }

    pub fn build_block_list_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: BlockListScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::BlockList, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { block_list: Some(scene), ..node })
    }

    pub fn build_diff_view_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: DiffViewScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::DiffView, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { diff_view: Some(scene), ..node })
    }

    pub fn build_event_feed_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: EventFeedScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::EventFeed, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { event_feed: Some(scene), ..node })
    }

    //#region 🔖️StatusBuilders
    /** 🗂️ Builds an empty-state placeholder: a centered title, optional description text, and an
     * optional call-to-action button. */
    pub fn ui_empty_state(id: &str, title: Label, description: Option<Label>, action: Option<UiButtonNode>) -> UiNode {
        let mut children = vec![UiNode::Text(UiTextNode { menu: None, value: title, emphasize: Some(true), data_attributes: None, presence: UiPresence::default() })];
        if let Some(description) = description {
            children.push(ui_text(description));
        }
        if let Some(action) = action {
            children.push(UiNode::Button(action));
        }
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some(id.into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children,
        })
    }

    /** ⚠️ Builds an error-state placeholder: an emphasized message and an optional retry button. */
    pub fn ui_error_state(id: &str, message: Label, retry: Option<ActionDescriptor>) -> UiNode {
        let mut children = vec![UiNode::Text(UiTextNode { menu: None, value: message, emphasize: Some(true), data_attributes: None, presence: UiPresence::default() })];
        if let Some(retry) = retry {
            children.push(UiNode::Button(UiButtonNode {
                menu: None,
                id: Some(format!("{id}.retry")),
                icon_id: IconName::RotateCw,
                // 🚧️ Framework-level fallback copy (not app content); not yet routed through app_labels!
                // since this SDK-level builder predates the two-axis macro — flagged for a follow-up pass.
                label: Label::data("Retry"),
                action: retry,
                style: None,
                presence: UiPresence::default(),
            }));
        }
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some(id.into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children,
        })
    }

    /** 🩺️ Builds a plugin-recovery panel: bilingual (en/de) crash copy plus three fixed actions —
     * restart the app (`recovery.restartApp`), disable the offending program (`recovery.disablePlugin`), or
     * open diagnostics (`recovery.showDiagnostics`). `quarantined` swaps in the host-auto-disabled copy. */
    pub fn ui_recovery_panel(plugin_id: &str, quarantined: bool, is_de: bool) -> UiNode {
        let title = if is_de { "Plugin-Wiederherstellung" } else { "Plugin Recovery" };
        let message = match (quarantined, is_de) {
            (true, true) => "Dieses Plugin wurde nach wiederholten Abstürzen unter Quarantäne gestellt.",
            (true, false) => "This program was quarantined after repeated crashes.",
            (false, true) => "Dieses Plugin ist abgestürzt.",
            (false, false) => "This program crashed.",
        };
        let restart_label = if is_de { "App neu starten" } else { "Restart App" };
        let disable_label = if is_de { "Plugin deaktivieren" } else { "Disable Plugin" };
        let diagnostics_label = if is_de { "Diagnose anzeigen" } else { "Show Diagnostics" };
        let args = Some(DslValue::Object(vec![("pluginId".into(), DslValue::String(plugin_id.to_string()))]));
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some("recovery.panel".into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![
                UiNode::Text(UiTextNode { menu: None, value: Label::data(title), emphasize: Some(true), data_attributes: None, presence: UiPresence::default() }),
                ui_text(Label::data(message)),
                UiNode::Button(UiButtonNode {
                    menu: None,
                    id: Some("recovery.restartApp".into()),
                    icon_id: IconName::RotateCcw,
                    label: Label::data(restart_label),
                    action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.restartApp".into(), args: args.clone() },
                    style: None,
                    presence: UiPresence::default(),
                }),
                UiNode::Button(UiButtonNode {
                    menu: None,
                    id: Some("recovery.disablePlugin".into()),
                    icon_id: IconName::Link2Off,
                    label: Label::data(disable_label),
                    action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.disablePlugin".into(), args: args.clone() },
                    style: None,
                    presence: UiPresence::default(),
                }),
                UiNode::Button(UiButtonNode {
                    menu: None,
                    id: Some("recovery.showDiagnostics".into()),
                    icon_id: IconName::Info,
                    label: Label::data(diagnostics_label),
                    action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.showDiagnostics".into(), args },
                    style: None,
                    presence: UiPresence::default(),
                }),
            ],
        })
    }

    /** 📥️ Builds a drop-zone `Stack` for importing files: `drop_overlay` supplies the hover-state
     * title/hint/accept copy, `drop_action` fires once the drop completes. */
    pub fn ui_import_drop_zone(id: &str, title: Label, hint: Label, accept: Option<&str>, drop_action: ActionDescriptor) -> UiNode {
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some(id.into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: Some(drop_action),
            drop_overlay: Some(UiDropOverlaySpec { title: title.clone(), hint: hint.clone(), accept: accept.map(Into::into) }),
            children: vec![ui_text(title), ui_text(hint)],
        })
    }
    //#endregion 🔖️StatusBuilders
    //#endregion 🔖️UiNode

    //#region 🔖️WireFormatGoldenTests
    /** 🧊️ Golden wire-format tests: freeze exact JSON for every UiNode/scene/SurfaceKind
    before these types move into ui_wgpu, so the move can be proven byte-identical. */
    #[cfg(test)]
    include!("../🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs");
    //#endregion 🔖️WireFormatGoldenTests

    //#region 🔖️ValueRoundTripTests
    /// 🌱️ `ToValue`/`FromValue` round-trip tests for ticket
    /// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS's additive pass over
    /// `pub mod ui`'s standalone node types, plus (since `semio-framework-ui-scene` landed hand-
    /// written `ToValue`/`FromValue` for all 15 embedded scene payload types) the previously-
    /// blocked recursive-through-`UiNode` family: `UiStackNode`/`UiGroupNode`/`UiFieldNode`/
    /// `UiSectionNode`/`UiInspectorFieldGroup`/`UiComponentSceneNode`/`UiNode` itself.
    #[cfg(test)]
    include!("../🧪️tests/🔬️targets-wgpu-component-ui-value-round-trip/🦀️.rs");
    //#endregion 🔖️ValueRoundTripTests
    // #endregion ui
}

