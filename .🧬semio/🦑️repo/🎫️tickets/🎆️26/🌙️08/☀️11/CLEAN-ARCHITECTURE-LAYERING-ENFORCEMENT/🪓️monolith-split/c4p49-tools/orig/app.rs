//! 🧱️ App surface: dialogs, modes, window kinds, panels, fault notice wiring and the app definition.

use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::any::Any;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use ui_wgpu::wgpu::IconName;
use ui_wgpu::wgpu::NamedLayout;
use ui_wgpu::wgpu::SurfaceKind;
use ui_wgpu::wgpu::WindowLayout;
use ui_wgpu::wgpu::WindowOptions;
use semio_framework_interaction::{InteractionDefinition, InteractionRef};
use semio_framework_kernel_history::FaultNoticeDefinition;
use semio_framework_kernel_identity::{ArtifactKind, CapabilityRequirement};
use semio_framework_manifest_actions::{ActionDefinition, ActionKind, CLEAR_SELECTION_ACTION_ID, CapabilityAudience, CommandDefinition, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID, SET_SELECTION_MODE_ACTION_ID, ToolDefinition, UtilityDefinition, resolve_audience};
use semio_framework_manifest_address::{ActionRef, Keybinding, ToolRef, UtilityRef};
use semio_framework_manifest_arg_schema::{ActionArgDef, ActionArgOption, AppRole, ArgFormat, ArgSchema};
use semio_framework_manifest_media::{AppIo, ArtifactKindSpec, MediaPortSpec};
use semio_framework_onboarding::{IntroductionDefinition, TutorialDefinition};

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Dialog
/// 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
/// identically to every glass overlay box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)
/// presents `args` as a staged form. Submit dispatches `submit_action` with the merged effective
/// args; empty `args` degenerates to a message/confirm dialog. Opened only via
/// `Effect::OpenDialog`; the shell owns open/close as ephemeral chrome state, never the document.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DialogDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub body: Option<LocalizedLabel>,
    pub args: Vec<ActionArgDef>,
    /// 📇️ References an action owned by the active window kind, dispatched with merged args.
    pub submit_action: ActionRef,
    pub submit_label: LocalizedLabel,
    /// 📇️ Optional active-window action reference dispatched on any dismissal (Escape, veil
    /// click, or the Cancel button).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cancel_action: Option<ActionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cancel_label: Option<LocalizedLabel>,
    /// 🔀️ Further decisions offered beside the submit, in focus order before it. Each is gated on and
    /// dispatches only the args it [`requires`](DialogChoice::requires) (plus the seed context), with
    /// [`DIALOG_CHOICE_ARG`] naming the choice, so one action can serve several choices
    /// (`historyEditCommit{choice}`); the submit alone is gated on the dialog's required args.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<DialogChoice>,
}

/// 🔀️ The arg key a [`DialogChoice`] dispatch carries its own `id` under; reserved in a dialog with choices.
pub const DIALOG_CHOICE_ARG: &str = "choice";

/// 🎛️ One decision button of a [`DialogDefinition`]: a localized `label`, an optional `description` a
/// reader hears with it (`aria-describedby`) and a sighted user reads beside it, the `action` it
/// dispatches, the staged args it `requires` (enabled once they resolve, and the only form args it
/// sends), its visual `tone`, and whether it is `destructive` (danger styling, and an agent lane must
/// ask before taking it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DialogChoice {
    pub id: String,
    pub label: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    pub action: ActionRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<String>,
    #[serde(default, skip_serializing_if = "is_neutral_tone")]
    #[value(default, skip_serializing_if = "is_neutral_tone")]
    pub tone: semio_framework_ui_contract::Tone,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[value(default, skip_serializing_if = "std::ops::Not::not")]
    pub destructive: bool,
}

fn is_neutral_tone(tone: &semio_framework_ui_contract::Tone) -> bool {
    *tone == semio_framework_ui_contract::Tone::Neutral
}

impl DialogChoice {
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, action: ActionRef) -> Self {
        Self { id: id.into(), label: label.into(), description: None, action, requires: Vec::new(), tone: semio_framework_ui_contract::Tone::Neutral, destructive: false }
    }

    /// 🔗️ The staged args this choice consumes: it is enabled once they resolve and sends only them.
    pub fn requires(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.requires = args.into_iter().map(Into::into).collect();
        self
    }

    /// 🚧️ The args of [`requires`](Self::requires) still unresolved in `effective` (absent, blank, or off
    /// their current choice set) — the choice is enabled exactly when this is empty.
    pub fn unresolved_args(&self, defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
        let required: Vec<ActionArgDef> = defs.iter().filter(|def| self.requires.contains(&def.id)).map(|def| ActionArgDef { required: true, ..def.clone() }).collect();
        unresolved_action_args(&required, effective)
    }

    /// 📤️ The dispatch args: the seed context of `effective` (keys no arg of `defs` declares), the args this
    /// choice requires, and [`DIALOG_CHOICE_ARG`] naming it.
    pub fn dispatch_args(&self, defs: &[ActionArgDef], effective: &DslValue) -> DslValue {
        let mut entries = effective.as_object().map(<[_]>::to_vec).unwrap_or_default();
        entries.retain(|(key, _)| key != DIALOG_CHOICE_ARG && (self.requires.contains(key) || !defs.iter().any(|def| def.id == *key)));
        entries.push((DIALOG_CHOICE_ARG.to_string(), DslValue::String(self.id.clone())));
        DslValue::Object(entries)
    }

    /// 📝️ The consequence a reader hears with the button and a sighted user reads beside it.
    pub fn description(mut self, description: impl Into<LocalizedLabel>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 🎨️ The button's colour role.
    pub fn tone(mut self, tone: semio_framework_ui_contract::Tone) -> Self {
        self.tone = tone;
        self
    }

    /// ⚠️ Marks the choice destructive: danger styling, and an agent must ask before taking it.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

// 🚫️async: E1 — pure builder methods (self-mutation only, zero suspension points), reverted
// per R9: `catch_unwind` sync-closure test consumers are language-barred from async.
impl DialogDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<LocalizedLabel>, submit_action: ActionRef) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: None,
            args: Vec::new(),
            submit_action,
            // 🌐️ "OK" is identical in both locales — a real (not placeholder) translation choice.
            submit_label: LocalizedLabel::native("OK", "OK"),
            cancel_action: None,
            cancel_label: None,
            choices: Vec::new(),
        }
    }

    /// 🔀️ Appends one decision offered beside the submit.
    pub fn choice(mut self, choice: DialogChoice) -> Self {
        self.choices.push(choice);
        self
    }

    /// 🧐️ Choice ids are non-empty and unique, a choice requires only declared args, and no staged arg
    /// shadows [`DIALOG_CHOICE_ARG`].
    pub fn validate_choices(&self) -> Result<(), String> {
        let mut ids = std::collections::BTreeSet::new();
        for choice in &self.choices {
            if choice.id.trim().is_empty() {
                return Err("choice id must be non-empty".to_string());
            }
            if !ids.insert(choice.id.as_str()) {
                return Err(format!("duplicate choice id {}", choice.id));
            }
            if let Some(arg) = choice.requires.iter().find(|arg| !self.args.iter().any(|def| def.id == **arg)) {
                return Err(format!("choice {} requires undeclared arg {arg}", choice.id));
            }
        }
        if !self.choices.is_empty() && self.args.iter().any(|arg| arg.id == DIALOG_CHOICE_ARG) {
            return Err(format!("arg {DIALOG_CHOICE_ARG} is reserved in a dialog with choices"));
        }
        Ok(())
    }

    /// 📝️ Attaches explanatory body text shown below the title.
    pub fn body(mut self, body: impl Into<LocalizedLabel>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 🧾️ Attaches the staged-form field declarations.
    pub fn args(mut self, args: Vec<ActionArgDef>) -> Self {
        self.args = args;
        self
    }

    /// ✅️ Overrides the submit button label (default "OK").
    pub fn submit_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// ❌️ Overrides the cancel button label (default "Cancel", applied by the renderer).
    pub fn cancel_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// 🚪️ Declares an action dispatched on any dismissal (Escape, veil click, Cancel button).
    pub fn on_cancel(mut self, action: ActionRef) -> Self {
        self.cancel_action = Some(action);
        self
    }
}

//#endregion 🔖️Dialog

// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    /// 🛠️ Tools available while this mode is active — references `AppDefinition.tools` ids.
    #[serde(default)]
    #[value(default)]
    pub tools: Vec<ToolRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<String>,
    /// 🎛️ Commands owned by this mode and active only while it is active.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
}

/// 🚫️ A non-empty, order-preserving list — construction-time enforcement replaces what used to be a
/// runtime `assert!` deep inside `AppBuilder::build_definition`. The first entry is the implicit
/// fallback default when nothing else specifies one.
#[derive(Clone, Debug, PartialEq)]
pub struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

// 🚫️async: E1 transitive block — every method here is a pure accessor/iterator with no I/O, and
// `iter`/`iter_mut` must feed directly into std `Iterator` combinators (`find`/`filter`/`map`, an
// external trait) at call sites, so the whole inherent impl stays sync rather than forcing an
// `.await` between every accessor and the combinator chain that consumes it (R9).
impl<T> NonEmptyVec<T> {
    pub fn one(first: T) -> Self {
        Self { first, rest: Vec::new() }
    }

    pub fn new(first: T, rest: Vec<T>) -> Self {
        Self { first, rest }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.first
    }

    pub fn push(&mut self, value: T) {
        self.rest.push(value);
    }
}

impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index == 0 {
            &self.first
        } else {
            &self.rest[index - 1]
        }
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Chain<std::iter::Once<&'a T>, std::slice::Iter<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(&self.first).chain(self.rest.iter())
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = String;
    // 🚫️async: E1 impl of external `TryFrom`; pure list-shape check, no I/O.
    fn try_from(mut values: Vec<T>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Err("expected a non-empty list, got zero entries".to_string());
        }
        let first = values.remove(0);
        Ok(Self { first, rest: values })
    }
}

impl<T: Clone> From<NonEmptyVec<T>> for Vec<T> {
    // 🚫️async: E1 impl of external `From`; pure collection reshape, no I/O.
    fn from(value: NonEmptyVec<T>) -> Self {
        std::iter::once(value.first).chain(value.rest).collect()
    }
}

/// 🌉️ Hand-written, not derived: `#[value(…)]` has no `into`/`try_from` equivalent — this mirrors
/// the retired `#[serde(try_from = "Vec<T>", into = "Vec<T>")]` bridge, wire-shaped as a plain
/// array exactly like `Vec<T>` (see `ToValue`/`FromValue for Vec<T>` in `🌱️value/🔁️codec/🦀️.rs`).
impl<T: ToValue> ToValue for NonEmptyVec<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}

impl<T: FromValue> FromValue for NonEmptyVec<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Array(items) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array, found {value:?}")));
        };
        let values = items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect::<Result<Vec<T>, ValueError>>()?;
        NonEmptyVec::try_from(values).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error))
    }
}

// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `AppDefinition`
// (`modes: Modes`/`window_kinds: WindowKinds`, both `NonEmptyVec<_>`) is itself serde-only — blocked
// on `ui_wgpu`/`interaction`/`🎠️kernel` field types elsewhere in this file — so `NonEmptyVec` must
// keep pace with it. Hand-written (not `#[serde(try_from/into = "Vec<T>")]`, since the derive is
// gone): mirrors the retired attribute via the existing `TryFrom<Vec<T>>`/`From<NonEmptyVec<T>> for
// Vec<T>` bridge above.
impl<T: Clone + Serialize> Serialize for NonEmptyVec<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let values: Vec<T> = self.clone().into();
        values.serialize(serializer)
    }
}

impl<'de, T: serde::de::DeserializeOwned> Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let values = Vec::<T>::deserialize(deserializer)?;
        NonEmptyVec::try_from(values).map_err(serde::de::Error::custom)
    }
}

/// 🚫️ Every app has at least one mode — `playbook/module/procedural` and any other single-purpose app
/// must declare an explicit mode (e.g. `"default"`) instead of the zero-mode state the type system
/// now makes unrepresentable.
pub type Modes = NonEmptyVec<ModeDefinition>;

/// 🚫️ Every app has at least one window kind — mirrors `Modes`, formerly a runtime `assert!` in
/// `AppBuilder::build_definition`.
pub type WindowKinds = NonEmptyVec<WindowKindDefinition>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WindowKindDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    pub icon_id: IconName,
    /// 🎛️ Always-present chrome facets (was: separately-optional `measures`/`engagement`).
    #[serde(default)]
    #[value(default)]
    pub options: WindowOptions,
    /// 📇️ Actions owned by this window kind. Mandatory, may be empty, never absent.
    #[serde(default)]
    #[value(default)]
    pub actions: Vec<ActionDefinition>,
    /// 🧰️ Utilities this window kind accepts — references `AppDefinition.utilities` ids. Empty = no utilities.
    #[serde(default)]
    #[value(default)]
    pub utilities: Vec<UtilityRef>,
    /// 🌅️ The first arm of each new window instance, accepted by this window's utility roster.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub initial_utility_id: Option<UtilityRef>,
    /// 🕹️ Interaction domains this window kind accepts — references `AppDefinition.interactions` ids.
    /// Empty = no interactions.
    #[serde(default)]
    #[value(default)]
    pub interactions: Vec<InteractionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub params_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub artifact_snapshot_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub input_event_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub capabilities: Vec<CapabilityRequirement>,
}

// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PanelGroup {
    Workbench,
    Details,
    Display,
    Settings,
}

impl PanelGroup {
    /// 🧭️ The dock anchor this group defaults to. Groups only ever map to the four corner anchors —
    /// the four edge-middle anchors (`top-middle`/`right-middle`/`bottom-middle`/`left-middle`) start
    /// empty and are user-populated via drag-and-drop or a dock skeleton override, never via a `PanelGroup`.
    pub fn anchor(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "top-left",
            PanelGroup::Details => "top-right",
            PanelGroup::Display => "bottom-left",
            PanelGroup::Settings => "bottom-right",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "workbench",
            PanelGroup::Details => "details",
            PanelGroup::Display => "display",
            PanelGroup::Settings => "settings",
        }
    }
}

/// 🌳️ Closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention that used to
/// live in the renderer: every panel tab is either a framework-predefined kind (compile-time
/// exhaustive) or an app-declared custom tab (open id, still required to be unique/non-empty,
/// validated at construction by `AppBuilder`).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
#[value(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum PanelTabKind {
    WorkbenchCategory,
    DisplayCategory,
    DetailsCategory,
    SettingsCategory,
    DisplayWindows,
    DisplayLayout,
    SettingsGeneral,
    SettingsTheme,
    /// 🎯️ ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET C1/C5: the "Default apps" settings
    /// sub-tab — table of dialect × {viewer, editor} with selects, writes only through the `os.*`
    /// default-app commands (see 🔖️Surface below).
    SettingsDefaultApps,
    /// 🧩️ App-declared tab — id is app-namespaced (e.g. `"puzzle.catalogue"`).
    App(String),
}

impl PanelTabKind {
    /// 🔤️ Flat string key for code that needs one, e.g. React `key=` props.
    pub fn id_str(&self) -> &str {
        match self {
            PanelTabKind::WorkbenchCategory => "framework.category.workbench",
            PanelTabKind::DisplayCategory => "framework.category.display",
            PanelTabKind::DetailsCategory => "framework.category.details",
            PanelTabKind::SettingsCategory => "framework.category.settings",
            PanelTabKind::DisplayWindows => "framework.display.windows",
            PanelTabKind::DisplayLayout => "framework.display.layout",
            PanelTabKind::SettingsGeneral => "framework.settings.general",
            PanelTabKind::SettingsTheme => "framework.settings.theme",
            PanelTabKind::SettingsDefaultApps => "framework.settings.default-apps",
            PanelTabKind::App(id) => id.as_str(),
        }
    }
}

/// 🌳️ A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it). Exactly one of the two is set; `group` is only meaningful on root (non-nested) entries.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PanelTabDefinition {
    pub kind: PanelTabKind,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub group: PanelGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub body_key: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub children: Vec<PanelTabDefinition>,
}

impl PanelTabDefinition {
    pub fn id(&self) -> &str {
        self.kind.id_str()
    }
}

//#region 🔖️Surface
/// 🎯️ A surface addressed across plugin boundaries.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AppRef {
    pub plugin_id: String,
    pub app_id: String,
}

/// 🪪️ The one canonical spelling of a surface id: `<artifact_kind>@<standard>/<subset>#<role>`.
pub fn surface_app_id(dialect: &ArtifactDialect, role: AppRole) -> String {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    format!("{}#{}", dialect.to_coordinate(), role.as_str())
}

/// 🪪️ Inverse of `surface_app_id`; rejects anything not matching the grammar.
pub fn parse_surface_app_id(id: &str) -> Result<(ArtifactDialect, AppRole), String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let (coordinate, role_str) = id.rsplit_once('#').ok_or_else(|| format!("surface id {id:?} missing '#'"))?;
    let dialect = ArtifactDialect::parse_coordinate(coordinate)?;
    let role: AppRole = role_str.parse().map_err(|err| format!("surface id {id:?}: {err}"))?;
    Ok((dialect, role))
}

//#endregion 🔖️Surface

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppDefinition {
    pub id: String,
    /// 👁️✏️ Whether this surface may mutate the artifact it is bound to — see `AppRole`.
    pub role: AppRole,
    /// 🎯️ The dialect coordinate (artifact kind, standard, subset) this surface is bound to — see
    /// `ArtifactDialect`. Together with `role` this derives the canonical `id` via `surface_app_id`.
    pub dialect: ArtifactDialect,
    /// 🗣️ The app's own display name (e.g. "Puzzle 3D") — manifest-level, locale×terminology-checked,
    /// see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub breadcrumb: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<IconName>,
    pub controller_id: String,
    /// 🚧️ `Modes` is `NonEmptyVec<ModeDefinition>`, whose `serde(try_from/into = "Vec<T>")` wire
    /// format is a flat array — not the `{ first, rest }` shape owned schema exporter would infer from the struct
    /// fields, so the wire-accurate array shape is supplied directly instead of deriving `TS` on
    /// `NonEmptyVec` itself.
    pub modes: Modes,
    pub default_mode_id: String,
    /// 🚧️ See `modes` above — `WindowKinds` is `NonEmptyVec<WindowKindDefinition>`.
    pub window_kinds: WindowKinds,
    /// 🕹️ The app-wide action roster: every action dispatchable in ANY window of this app that no
    /// window kind claims as its own, including the framework-injected History/Clipboard verbs and
    /// every reserved contribution. A window kind's dispatchable set is `WindowKindDefinition.actions` followed by
    /// this roster minus the ids some window claims — [`resolve_window_actions`], whose TypeScript
    /// twin is `resolveWindowActions` (`🎯️action-bus/🟦️.ts`). Kept here rather than copied into
    /// every window kind because a copy made the package descriptor grow as
    /// `apps × window kinds × actions`: 21 distinct rows were stored 675 times in one plugin, and
    /// the duplicates were 31.9 % of every shipped descriptor's bytes (ticket
    /// 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, slice DS1).
    #[serde(default)]
    #[value(default)]
    pub actions: Vec<ActionDefinition>,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
    /// 🧰️ The interactive utilities this app exposes (referenced by `WindowKindDefinition.utilities`).
    #[serde(default)]
    #[value(default)]
    pub utilities: Vec<UtilityDefinition>,
    /// 🛠️ The mode-level tools this app exposes (referenced by `ModeDefinition.tools`).
    #[serde(default)]
    #[value(default)]
    pub tools: Vec<ToolDefinition>,
    /// 🎛️ Commands owned by this app and active whenever it is focused.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
    /// 🕹️ The interaction domains (hover + selection) this app exposes (referenced by
    /// `WindowKindDefinition.interactions`) — see `crate::InteractionDefinition`.
    #[serde(default)]
    #[value(default)]
    pub interactions: Vec<InteractionDefinition>,
    #[serde(default)]
    #[value(default)]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_layout: Option<WindowLayout>,
    /// 🗣️ Terminology ids this app declares beyond the implicit "native" default.
    #[serde(default)]
    #[value(default)]
    pub terminologies: Vec<String>,
    /// 🗺️ Terminology id -> full replacement breadcrumb (product + app segments), e.g. "reuse" ->
    /// ["Entwerfen mit Bestand", "Aggregator"]; ids absent here keep the canonical breadcrumb under that terminology.
    #[serde(default)]
    #[value(default)]
    pub terminology_breadcrumbs: std::collections::HashMap<String, Vec<String>>,
    /// 🎓️ This app's first-run walkthrough, if it declares one — see `IntroductionDefinition`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<IntroductionDefinition>,
    /// 🎬️ Recorded, timed walkthroughs this app declares — see `TutorialDefinition`. A brand's own
    /// `tutorials` (if any) are shown alongside these, never replacing them (unlike `introduction`).
    #[serde(default)]
    #[value(default)]
    pub tutorials: Vec<TutorialDefinition>,
    /// 🗨️ The modal form dialogs this app can open via `Effect::OpenDialog`.
    #[serde(default)]
    #[value(default)]
    pub dialogs: Vec<DialogDefinition>,
    /// 🔌️ This app's workflow input ports — see `crate::MediaPortSpec`.
    #[serde(default)]
    #[value(default)]
    pub media_inputs: Vec<MediaPortSpec>,
    /// 🔌️ This app's workflow output ports — see `crate::MediaPortSpec`.
    #[serde(default)]
    #[value(default)]
    pub media_outputs: Vec<MediaPortSpec>,
    /// 🗂️ OS resource kinds this app produces/consumes — see `crate::ArtifactKindSpec`. Drives
    /// `framework/product/os/core`'s artifact catalog registry instead of a hardcoded per-app match.
    #[serde(default)]
    #[value(default)]
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    /// 🧮️ This app's typed configuration record — see `crate::ConfigSpec`. Empty until per-app waves
    /// populate it.
    #[serde(default)]
    #[value(default)]
    pub config: ConfigSpec,
    /// 🎛️ This app's typed binary command grammar — see `crate::CommandGrammar`. Empty until per-app
    /// waves populate it.
    #[serde(default)]
    #[value(default)]
    pub command_grammar: CommandGrammar,
    /// 🔌️ This app's typed media I/O surface — see `crate::AppIo`. Not yet populated; `media_inputs`/
    /// `media_outputs`/`artifact_kinds` above remain the live source of truth until later waves migrate
    /// onto this.
    #[serde(default)]
    #[value(default)]
    pub io: AppIo,
    /// 📢️ The localized notices of the fault codes this app's guest refuses with (`ArtifactApp::fault_notices`, design
    /// §20.12), stamped at registration — what both shells show for a refused dispatch after the framework's own table.
    #[serde(default)]
    #[value(default)]
    pub fault_notices: Vec<FaultNoticeDefinition>,
}

/// 🧭️ Resolves the dock layout a mode should present.
pub fn resolve_layout_for_mode(app: &AppDefinition, mode_id: &str) -> Option<WindowLayout> {
    let mode = app.modes.iter().find(|mode| mode.id == mode_id)?;
    if let Some(layout_id) = &mode.layout_id {
        if let Some(named) = app.named_layouts.iter().find(|entry| entry.id == *layout_id) {
            return Some(named.layout.clone());
        }
    }
    app.default_layout.clone()
}

//#region 🔖️action-args
/// 🧮️ Computes the effective argument map for an action: for each declared arg, the staged value
/// if present, else its declared `default`, else omitted. Renderers stage edits locally and pass them
/// here; the contract enforcer ({@link VcsArtifactApp}) materializes defaults before dispatch so plugins
/// never re-implement default-filling.
///
/// 🌱️ `seed` carries a dialog's pre-seeded context args (e.g. a row-scoped `spaceId` that is never a
/// declared, editable form field, per `Effect::OpenDialog { args }`) through untouched: any `seed`
/// key that is not a declared arg id survives into the result unmodified, and a `seed` value for a
/// declared id that hasn't been staged yet acts as that field's initial value. A dialog with zero
/// declared `defs` (a plain confirm/cancel, e.g. `deleteSpace`) passes `seed`+`staged` through
/// wholesale — TS twin: {@link effectiveActionArgs} (`🧩️action-argument-resolution/🟦️.ts`).
pub fn effective_action_args(defs: &[ActionArgDef], staged: &DslValue, seed: Option<&DslValue>) -> DslValue {
    let seed_pairs: Vec<(String, DslValue)> = seed.and_then(DslValue::as_object).map(<[_]>::to_vec).unwrap_or_default();
    if defs.is_empty() {
        let mut effective = seed_pairs;
        if let Some(staged_pairs) = staged.as_object() {
            for (key, value) in staged_pairs {
                if let Some(existing) = effective.iter_mut().find(|(k, _)| k == key) {
                    existing.1 = value.clone();
                } else {
                    effective.push((key.clone(), value.clone()));
                }
            }
        }
        return DslValue::Object(effective);
    }
    let mut effective = seed_pairs;
    for def in defs {
        if let Some(value) = staged.get(&def.id) {
            if let Some(existing) = effective.iter_mut().find(|(k, _)| *k == def.id) {
                existing.1 = value.clone();
            } else {
                effective.push((def.id.clone(), value.clone()));
            }
        } else if effective.iter().any(|(k, _)| *k == def.id) {
            // 🌱️ seeded value already present for this declared field — keep it as the pre-fill.
        } else if let Some(default) = &def.default {
            effective.push((def.id.clone(), default.clone()));
        }
    }
    DslValue::Object(effective)
}

/// ❗️ Returns the ids of required args that are still unset in `effective`. "Unset" means absent,
/// `Null`, or an empty string (covers a blank Text/Select/IconSelect/ArtifactKind/SurfaceApp — the
/// latter two resolve to a `String` effective value exactly like `Select`, contract §C8.1); `false`,
/// `0`, and `[]` are valid values for Toggle/Number/Slider/Vec3 and never count as unset.
/// `Any` requires presence, allowing null and empty strings; a string with `min_len: Some(0)`
/// explicitly permits an empty value, such as document text or a root JSON pointer.
pub fn missing_required_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    defs.iter()
        .filter(|def| def.required)
        .filter(|def| match effective.get(&def.id) {
            None => true,
            Some(DslValue::Null) => !matches!(def.schema, ArgSchema::Any),
            Some(DslValue::String(text)) => text.is_empty() && !matches!(def.schema, ArgSchema::Any | ArgSchema::String { min_len: Some(0), .. }),
            Some(_) => false,
        })
        .map(|def| def.id.clone())
        .collect()
}

/// 🔽️ Closed choices must belong to their current host-resolved option set.
pub fn action_arg_requires_choice(def: &ActionArgDef) -> bool {
    matches!(&def.schema, ArgSchema::String { options, format, .. } if !options.is_empty() || matches!(format, Some(ArgFormat::ArtifactKind { .. } | ArgFormat::SurfaceApp { .. })))
}

/// 🚫️ Returns supplied choices absent from the current catalog, including an unavailable catalog.
pub fn invalid_action_choice_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    defs.iter().filter(|def| {
        let Some(value) = effective.get(&def.id) else { return false; };
        if matches!(value, DslValue::Null) || value.as_str() == Some("") || !action_arg_requires_choice(def) { return false; }
        matches!(&def.schema, ArgSchema::String { options, .. } if !options.iter().any(|option| value.as_str() == Some(option.value.as_str())))
    }).map(|def| def.id.clone()).collect()
}

/// 🛑️ Gates staged submission on required presence and exact current choice membership.
pub fn unresolved_action_args(defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
    let unresolved: std::collections::HashSet<String> = missing_required_args(defs, effective).into_iter().chain(invalid_action_choice_args(defs, effective)).collect();
    defs.iter().filter(|def| unresolved.contains(&def.id)).map(|def| def.id.clone()).collect()
}

/// 🚦️ Whether an action is eligible to appear in a window's Actions panel — excludes the six
/// framework History actions (rendered by the History rail) and the injected `setActiveUtility`/
/// `setActiveTool` (internal View actions wired to the utility bar/tool panel, never the panel).
// 🚫️async: E1 transitive — the only consumer is `Iterator::filter` (external trait), which takes a
// sync closure; pure comparison, no I/O (R9).
fn action_is_panel_eligible(action: &ActionDefinition) -> bool {
    action.kind != ActionKind::History && action.id != SET_ACTIVE_UTILITY_ACTION_ID && action.id != SET_ACTIVE_TOOL_ACTION_ID
}

/// 🕹️ THE window action predicate — every [`ActionDefinition`] dispatchable in `window`, in
/// the order a shell must offer them: the window kind's OWN roster first, then every
/// [`AppDefinition::actions`] row that no window kind of this app claims for itself. An id declared
/// by ANY window kind is that window's authored declaration and is never re-offered from the app
/// roster. The plugin builder used to bake this union into every window kind by cloning, which made
/// a package descriptor grow as `apps × window kinds × actions` — see `AppDefinition::actions`.
/// TypeScript twin: `resolveWindowActions` (`🎯️action-bus/🟦️.ts`).
pub fn window_kind_actions<'a>(app: &'a AppDefinition, window: &'a WindowKindDefinition) -> Vec<&'a ActionDefinition> {
    let claimed: std::collections::BTreeSet<&str> = app.window_kinds.iter().flat_map(|kind| kind.actions.iter().map(|action| action.id.as_str())).collect();
    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut resolved: Vec<&'a ActionDefinition> = Vec::with_capacity(window.actions.len() + app.actions.len());
    for action in window.actions.iter().chain(app.actions.iter().filter(|action| !claimed.contains(action.id.as_str()))) {
        if seen.insert(action.id.as_str()) {
            resolved.push(action);
        }
    }
    resolved
}

/// 📇️ Resolves the actions a window kind presents in its panel — [`window_kind_actions`]
/// minus the framework-only rail actions (the six History verbs and the injected
/// `setActiveUtility`/`setActiveTool`), preserving declaration order.
pub fn resolve_window_actions<'a>(app: &'a AppDefinition, window_kind: &'a WindowKindDefinition) -> Vec<&'a ActionDefinition> {
    window_kind_actions(app, window_kind).into_iter().filter(|action| action_is_panel_eligible(action)).collect()
}

/// 🛠️ Resolves the tools the active mode presents, in declared order — references into
/// `AppDefinition.tools` via `ModeDefinition.tools`. Unlike `resolve_window_actions`, unresolvable or
/// unreferenced tools have no orphan fallback: tools are opt-in per mode, not automatically shown
/// everywhere. Unresolvable refs are skipped (the builder validates them at construction time).
pub fn resolve_mode_tools<'a>(app: &'a AppDefinition, mode_id: &str) -> Vec<&'a ToolDefinition> {
    let Some(mode) = app.modes.iter().find(|mode| mode.id == mode_id) else {
        return Vec::new();
    };
    let mut resolved: Vec<&'a ToolDefinition> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for tool_ref in &mode.tools {
        if let Some(tool) = app.tools.iter().find(|tool| tool.id == tool_ref.as_str()) {
            if seen.insert(tool.id.as_str()) {
                resolved.push(tool);
            }
        }
    }
    resolved
}

//#endregion 🔖️action-args

/// 🪜️ Formats a canonical app breadcrumb for chrome.
pub fn app_breadcrumb(breadcrumb: &[String]) -> String {
    breadcrumb.join(" · ")
}

/// 🗺️ Resolves the breadcrumb effective under the active terminology; unknown/native ids fall back to the canonical breadcrumb.
pub fn resolve_app_breadcrumb<'a>(app: &'a AppDefinition, terminology: &str) -> &'a [String] {
    app.terminology_breadcrumbs.get(terminology).map_or(&app.breadcrumb, Vec::as_slice)
}

/// 🗂️ Formats a window tab within its canonical app breadcrumb, resolved under the active terminology
/// and `locale` (needed to resolve the now-`LocalizedLabel` `app.label` for the dedup comparison below).
pub fn app_window_label(app: &AppDefinition, terminology: &str, locale: Locale, window_label: &str) -> String {
    let mut breadcrumb = resolve_app_breadcrumb(app, terminology).to_vec();
    let normalized_window = window_label.trim().to_lowercase();
    let normalized_app = app.label.resolve(Terminology::parse(terminology).unwrap_or(Terminology::Native), locale).trim().to_lowercase();
    if !normalized_window.is_empty() && normalized_window != normalized_app && breadcrumb.last().is_none_or(|segment| segment.to_lowercase() != normalized_window) {
        breadcrumb.push(normalized_window);
    }
    app_breadcrumb(&breadcrumb)
}

// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    pub artifact_json: String,
    /// 🎯️ The DIALECT this example is authored for, never one app of it. An example is a document of
    /// an artifact's subset, and every surface bound to that subset — the editor and the viewer
    /// alike — opens the same eight fixtures; `SubsetDeclaration.examples` already models them that
    /// way. Stamped at registration from the registering app's own `AppDefinition.dialect`, and
    /// resolved by [`examples_for_app`] (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub dialect: ArtifactDialect,
}

/// 🏷️ One asset-name component: everything outside `[0-9A-Za-z._-]` folds to `-` so a dialect
/// coordinate (`s.puzzle.5d@1/*`) and an example id yield a single stable path segment.
pub fn asset_name_segment(raw: &str) -> String {
    raw.chars().map(|character| if character.is_ascii_alphanumeric() || character == '.' || character == '_' || character == '-' { character } else { '-' }).collect()
}

/// 📦️ The directory-and-stem prefix every externalized example body of `example_id` travels under.
/// The suffix after it names the BYTES that were declared: `.json` for a body the descriptor
/// emitter moved out of the manifest, and the authored fixture's own extension for a body the
/// plugin never materialised at all (see `ExampleSource::deferred`). Consumers match the prefix,
/// never one fixed extension, because those two declarations hash different bytes on purpose.
pub fn example_body_asset_prefix(dialect: &ArtifactDialect, example_id: &str) -> String {
    format!(
        "📚️examples/{}.{}.{}/{}",
        asset_name_segment(&dialect.artifact_kind),
        asset_name_segment(&dialect.standard),
        asset_name_segment(&dialect.subset),
        asset_name_segment(example_id)
    )
}

/// 📦️ The full `AssetDeclaration.name` for one example body — [`example_body_asset_prefix`] plus the
/// suffix of the bytes actually declared (`.json`, `.dsl.semio`, …).
pub fn example_body_asset_name(dialect: &ArtifactDialect, example_id: &str, suffix: &str) -> String {
    format!("{}{}", example_body_asset_prefix(dialect, example_id), suffix)
}

/// 📚️ THE example-picker predicate — every example authored for `dialect`, in manifest order,
/// deduplicated by id. The react shell's navbar select (`ShellHost`'s `exampleOptions`) answers it,
/// and so does the TypeScript twin `examplesForDialect` (`🛂️manifest/🟦️.ts`); both are pinned against the shared
/// `🧫️fixtures/📚️example-picker.json` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn examples_for_dialect<'a>(examples: &'a [ExampleDefinition], dialect: &ArtifactDialect) -> Vec<&'a ExampleDefinition> {
    let mut seen = std::collections::BTreeSet::new();
    examples.iter().filter(|example| &example.dialect == dialect).filter(|example| seen.insert(example.id.clone())).collect()
}

/// 📚️ The examples one surface may offer — [`examples_for_dialect`] against that surface's own
/// dialect. Role plays no part: an editor and its viewer are two surfaces of ONE dialect and offer
/// exactly the same picker.
pub fn examples_for_app<'a>(examples: &'a [ExampleDefinition], app: &AppDefinition) -> Vec<&'a ExampleDefinition> {
    examples_for_dialect(examples, &app.dialect)
}

//#region 🔖️MediaVocabulary
//#region 🔖️ConfigSpec

/// 🧮️ An app's full typed configuration record — the manifest-level declaration
/// `AppDefinition.config` carries: one [`ActionArgDef`] per field (its `id` is the config key), so a config field,
/// an action argument and a mutation input share one input vocabulary and one renderer.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ConfigSpec {
    #[serde(default)]
    #[value(default)]
    pub fields: Vec<ActionArgDef>,
}

impl ConfigSpec {
    pub async fn empty() -> Self {
        Self::default()
    }
}

//#endregion 🔖️ConfigSpec
//#region 🔖️CommandGrammar

/// 🎛️ One keyword-dispatched command variant (e.g. `move x=1 y=2`) and its field grammar — one
/// [`ActionArgDef`] per field, `required` where the grammar demands the field.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandVariantSpec {
    pub keyword: String,
    pub fields: Vec<ActionArgDef>,
}

/// 🎛️ An app's full typed binary command grammar — the manifest-level declaration
/// `AppDefinition.command_grammar` carries. Empty until per-app waves populate it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandGrammar {
    #[serde(default)]
    #[value(default)]
    pub variants: Vec<CommandVariantSpec>,
}

impl CommandGrammar {
    pub async fn empty() -> Self {
        Self::default()
    }
}

//#endregion 🔖️CommandGrammar
//#endregion 🔖️MediaVocabulary
//#region 🧩️ReservedContribution
/// 🧩️ What one feature reserves on the apps that host it: actions joined to the app roster, actions joined to every
/// window kind of a built app, dialogs joined to an app that may edit, and the audience it fixes for its own verbs. A
/// host (the app builder, the capability catalog) passes the contributions it hosts to [`reserved_app_actions`],
/// [`reserved_window_actions`], [`reserved_editor_dialogs`] and [`reserved_audience_violation`], so a feature reaches
/// every app without the manifest naming it.
#[derive(Clone, Copy)]
pub struct ReservedContribution {
    pub id: &'static str,
    pub app_actions: fn() -> Vec<ActionDefinition>,
    pub window_actions: fn(&AppDefinition) -> Vec<ActionDefinition>,
    pub editor_dialogs: fn() -> Vec<DialogDefinition>,
    pub fixed_audience: fn(&str) -> Option<CapabilityAudience>,
}

impl ReservedContribution {
    /// 🌱️ A contribution named `id` that reserves nothing yet.
    pub const fn new(id: &'static str) -> Self {
        Self { id, app_actions: no_reserved_actions, window_actions: no_reserved_window_actions, editor_dialogs: no_reserved_dialogs, fixed_audience: no_fixed_audience }
    }

    /// 🕹️ Reserves `app_actions` on the app roster of every hosting app.
    pub const fn with_app_actions(self, app_actions: fn() -> Vec<ActionDefinition>) -> Self {
        Self { app_actions, ..self }
    }

    /// 🪟️ Reserves `window_actions` on every window kind of a built app.
    pub const fn with_window_actions(self, window_actions: fn(&AppDefinition) -> Vec<ActionDefinition>) -> Self {
        Self { window_actions, ..self }
    }

    /// 🗳️ Reserves `editor_dialogs` on every hosting app that may edit.
    pub const fn with_editor_dialogs(self, editor_dialogs: fn() -> Vec<DialogDefinition>) -> Self {
        Self { editor_dialogs, ..self }
    }

    /// 🧭️ Fixes the audience of this contribution's own verbs.
    pub const fn with_fixed_audience(self, fixed_audience: fn(&str) -> Option<CapabilityAudience>) -> Self {
        Self { fixed_audience, ..self }
    }
}

fn no_reserved_actions() -> Vec<ActionDefinition> {
    Vec::new()
}

fn no_reserved_window_actions(_app: &AppDefinition) -> Vec<ActionDefinition> {
    Vec::new()
}

fn no_reserved_dialogs() -> Vec<DialogDefinition> {
    Vec::new()
}

fn no_fixed_audience(_action_id: &str) -> Option<CapabilityAudience> {
    None
}

/// 🕹️ The app-roster actions of every contribution, in contribution order.
pub fn reserved_app_actions(contributions: &[ReservedContribution]) -> Vec<ActionDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.app_actions)()).collect()
}

/// 🪟️ The window-kind actions every contribution reserves on the built `app`, in contribution order.
pub fn reserved_window_actions(contributions: &[ReservedContribution], app: &AppDefinition) -> Vec<ActionDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.window_actions)(app)).collect()
}

/// 🗳️ The dialogs every contribution reserves on an app that may edit, in contribution order.
pub fn reserved_editor_dialogs(contributions: &[ReservedContribution]) -> Vec<DialogDefinition> {
    contributions.iter().flat_map(|contribution| (contribution.editor_dialogs)()).collect()
}

/// 🧭️ The audience the first contribution that owns `action_id` fixes for it.
pub fn reserved_fixed_audience(contributions: &[ReservedContribution], action_id: &str) -> Option<CapabilityAudience> {
    contributions.iter().find_map(|contribution| (contribution.fixed_audience)(action_id))
}

/// ⚖️ The audience a contribution fixes for `action` when `action` resolves to a different one.
pub fn reserved_audience_violation(contributions: &[ReservedContribution], action: &ActionDefinition) -> Option<CapabilityAudience> {
    reserved_fixed_audience(contributions, &action.id).filter(|expected| *expected != resolve_audience(action))
}

//#endregion 🧩️ReservedContribution
//#region 🔖️Interaction
//#region 🔖️InteractionRefreshScope
/// 🕹️ Every declared window body key indexed by the interaction domain that window kind declares —
/// the schema-first half of [`interaction_verb_surface_scope`], read once when an app's action
/// registry is built rather than per dispatched hover.
pub fn interaction_window_bodies_by_domain(app: &AppDefinition) -> BTreeMap<String, Vec<String>> {
    let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for window in app.window_kinds.iter() {
        for interaction in &window.interactions {
            let bodies = index.entry(interaction.as_str().to_string()).or_default();
            if !bodies.iter().any(|body| body == &window.body_key) {
                bodies.push(window.body_key.clone());
            }
        }
    }
    index
}

/// 🌳️ Every declared panel LEAF body key, in declaration order, branches walked — the panel half of
/// [`interaction_verb_surface_scope`].
pub fn panel_leaf_body_keys(app: &AppDefinition) -> Vec<String> {
    fn walk(tab: &PanelTabDefinition, out: &mut Vec<String>) {
        if let Some(body_key) = &tab.body_key {
            if !out.iter().any(|entry| entry == body_key) {
                out.push(body_key.clone());
            }
        }
        for child in &tab.children {
            walk(child, out);
        }
    }
    let mut out = Vec::new();
    for tab in &app.panel_tabs {
        walk(tab, &mut out);
    }
    out
}

//#endregion 🔖️InteractionRefreshScope

/// 🕹️ The six framework-owned Interaction actions, auto-injected into any `AppDefinition` that
/// declares at least one `InteractionDefinition` — mirrors `history_action_definitions`/
/// `clipboard_action_definitions`, except conditional (like `set_active_utility_action_definition`)
/// rather than unconditional: returns `[]` when `app.interactions` is empty. `interactionSelect`/
/// `interactionHover` are the raw dispatch verbs renderers translate clicks/marquee/hover into
/// (never in the palette); `clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity`
/// are user-facing and drive the per-domain Select controls.
pub fn interaction_action_definitions(app: &AppDefinition) -> Vec<ActionDefinition> {
    if app.interactions.is_empty() {
        return Vec::new();
    }
    let merge_options = vec![
        ActionArgOption::new("replace", LocalizedLabel::native("Replace", "Ersetzen")),
        ActionArgOption::new("additive", LocalizedLabel::native("Additive", "Additiv")),
        ActionArgOption::new("subtractive", LocalizedLabel::native("Subtractive", "Subtraktiv")),
        ActionArgOption::new("invertive", LocalizedLabel::native("Invertive", "Invertierend")),
        ActionArgOption::new("range", LocalizedLabel::native("Range", "Bereich")),
    ];
    let method_options =
        vec![ActionArgOption::new("pick", LocalizedLabel::native("Pick", "Auswahl")), ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")), ActionArgOption::new("lasso", LocalizedLabel::native("Lasso", "Lasso"))];
    let mode_options = vec![ActionArgOption::new("single", LocalizedLabel::native("Single", "Einzeln")), ActionArgOption::new("multiple", LocalizedLabel::native("Multiple", "Mehrfach"))];
    vec![
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(INTERACTION_SELECT_ACTION_ID, LocalizedLabel::native("Select", "Auswählen"), ActionKind::Interaction, "mouse-pointer") }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
            ActionArgDef::select("merge", LocalizedLabel::native("Merge", "Zusammenführen"), merge_options).required(),
            ActionArgDef::select("method", LocalizedLabel::native("Method", "Methode"), method_options).required(),
        ]),
        ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(INTERACTION_HOVER_ACTION_ID, LocalizedLabel::native("Hover", "Darüberfahren"), ActionKind::Interaction, "eye") }.with_args([
            ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(),
            ActionArgDef::text("channel", LocalizedLabel::native("Channel", "Kanal")).required(),
            ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele")).required(),
        ]),
        ActionDefinition { keys: Some("escape".into()), ..ActionDefinition::resumable_framework(CLEAR_SELECTION_ACTION_ID, LocalizedLabel::native("Clear Selection", "Auswahl aufheben"), ActionKind::Interaction, "mouse-pointer-2") },
        ActionDefinition { keys: Some("mod+a".into()), ..ActionDefinition::resumable_framework(SELECT_ALL_ACTION_ID, LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::Interaction, "maximize-2") },
        ActionDefinition::resumable_framework(SET_SELECTION_MODE_ACTION_ID, LocalizedLabel::native("Set Selection Mode", "Auswahlmodus festlegen"), ActionKind::Interaction, "sliders-horizontal")
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::select("mode", LocalizedLabel::native("Mode", "Modus"), mode_options).required()]),
        ActionDefinition::resumable_framework(SET_INTERACTION_GRANULARITY_ACTION_ID, LocalizedLabel::native("Set Granularity", "Granularität festlegen"), ActionKind::Interaction, "layers")
            .with_args([ActionArgDef::text("domainId", LocalizedLabel::native("Domain", "Domäne")).required(), ActionArgDef::text("granularityId", LocalizedLabel::native("Granularity", "Granularität")).required()]),
    ]
}

//#endregion 🔖️Interaction
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest

#[cfg(any(test, feature = "testkit"))]
#[path = "../🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
use replication::HierarchyProvider;
#[cfg(test)]
use replication::HoverSpec;
#[cfg(test)]
use replication::MergeMode;
#[cfg(test)]
use replication::SelectionMethod;
#[cfg(test)]
use replication::SelectionMode;
#[cfg(test)]
use replication::SelectionSpec;
#[cfg(test)]
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
#[cfg(test)]
use semio_framework_value::Number;
#[cfg(test)]
use semio_framework_interaction::GranularityDefinition;
#[cfg(test)]
use semio_framework_kernel::Effect;
#[cfg(test)]
use semio_framework_kernel_identity::{CapabilityId, RequestId};
#[cfg(test)]
use semio_framework_kernel_ui::UiDirtyScope;
#[cfg(test)]
use semio_framework_manifest_actions::{ActionSemantics, ApprovalMode, InteractiveJobClassification, InteractiveJobClassificationError, OsDefinition, PreviewMode, REVERT_TO_COMMAND_ACTION_ID, SWITCH_ALTERNATIVE_ACTION_ID, SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, UndoMode, history_action_definitions, set_active_utility_action_definition, validate_interactive_job_classification};
#[cfg(test)]
use semio_framework_manifest_address::{ActionAddress, ActionInvocation, CommandAddress, CommandInvocation, CommandOwnerAddress, Platform, PlatformKeybinding, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID, child_element_id, element_id_segment, is_element_id, panel_tab_element_id, panel_tab_first_draggable_element_id, window_element_id};
#[cfg(test)]
use semio_framework_manifest_arg_schema::{ActionArgControl, ArgPresentation};

// #region 🛂️Manifest
//#region 🔖️Manifest
#[cfg(test)]
#[path = "../🧪️tests/🔬️app-label/🦀️.rs"]
mod app_label_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️example-picker/🦀️.rs"]
mod example_picker_tests;

//#region 🔖️Dialog
#[cfg(test)]
#[path = "../🧪️tests/🧪️dialog-choices/🦀️.rs"]
mod dialog_choices_tests;

//#endregion 🔖️Dialog
//#region 🧩️ReservedContribution
#[cfg(test)]
#[path = "../🧪️tests/🧩️reserved-contributions/🦀️.rs"]
mod reserved_contribution_tests;
//#endregion 🧩️ReservedContribution
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
