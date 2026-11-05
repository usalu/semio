# UI Native Receiving Field Audit

Exact declarations and wire attributes used for the full typed receiving cascade.

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TreeProps {
    #[serde(default, skip_serializing_if = "TreePresentation::is_standard")]
    #[value(default, skip_serializing_if = "TreePresentation::is_standard")]
    pub presentation: TreePresentation,
    /// 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain — selection/hover for
    /// bound items is owned by the framework's presence channel, not by per-item props.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<crate::UiText>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs
#[serde(default, skip_serializing_if = "TreePresentation::is_standard")]
#[value(default, skip_serializing_if = "TreePresentation::is_standard")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum TreePresentation {
    #[default]
    Standard,
    Compact,
    Touch,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum TreeWindowRowExtent {
    #[default]
    Standard,
    CompactText,
    CompactSmallControl,
    CompactControl,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TreeWindow {
    pub total: u32,
    pub offset: u32,
    pub row_extent: TreeWindowRowExtent,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TreeSectionProps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    /// 🎛️ The section's direct horizontal Toolbar child whose Button children render in the header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_toolbar: Option<crate::UiNodeId>,
    /// 🔪️ The materialised slice of this section's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeItemProps {
    pub label: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<crate::UiFixedMap<crate::UiText>>,
    /// 👁️ Domain "eye toggle": the row stays visible, dimmed, and clickable (to un-hide). NOT the
    /// same axis as the record's `activity`/`disabled` — a dimmed row is still fully interactive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimmed: Option<bool>,
    /// ☑️ The row's choice state when it is one option of a single-choice list: `Some(true)` is the chosen option, `Some(false)` a
    /// choosable one that is not chosen, `None` not an option row. Every renderer exposes it as the row's selected state (React
    /// `aria-selected`, the wgpu ARIA mirror's `selected`) and paints the chosen row selected — never by its icon alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// 📇️ The materialised slice of this row's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    /// 🏹️ The interaction granularity this row picks when the tree carries the domain binding: the
    /// row is a pick target of the tree's `interaction_domain`, keyed by its own record key, so it
    /// needs no argument map and no per-row binding of its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<crate::UiText>,
    /// 🧰️ A direct `ContainerRole::Toolbar` child laid out horizontally inside this row. The
    /// document validator requires the target to be one of this record's direct children and every
    /// toolbar child to be a real [`Component::Button`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_toolbar: Option<crate::UiNodeId>,
    /// 🎞️ A direct Surface child placed in a bounded detail band below this row. It is neither an
    /// inline control nor a nested TreeItem, and remains a real independently hosted scene.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<crate::UiNodeId>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎚️slider-geometry-and-key-handling/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderKey {
    Decrement,
    Increment,
    PageDown,
    PageUp,
    Home,
    End,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎚️slider-geometry-and-key-handling/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputKey {
    Enter,
    Escape,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎚️slider-geometry-and-key-handling/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextInputModifiers {
    pub primary: bool,
    pub shift: bool,
    pub alt: bool,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎚️slider-geometry-and-key-handling/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputKeyAction {
    /// ↵️ The field inserts a line break at the caret — its own editing, nothing is dispatched.
    Newline,
    /// ✅️ The draft is committed (`Trigger::Commit`) without waiting for focus to leave.
    Commit,
    /// ⏮️ The draft is dropped: the field shows the published value again.
    Revert,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum ContainerRole {
    #[default]
    Plain,
    Section,
    Group,
    Field,
    Form,
    Toolbar,
    Sheet,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum InputKind {
    #[default]
    Text,
    LongText,
    Number,
    Date,
    Color,
    File,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum RowActionPlacement {
    #[default]
    Row,
    Menu,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct DropOverlaySpec {
    pub title: Label,
    pub hint: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<crate::UiText>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct SelectItem {
    pub value: crate::UiText,
    pub label: Label,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct KeyValueEntry {
    pub label: Label,
    pub value: crate::UiText,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct RowAction {
    /// 🖼️ Icon key. See [`ButtonProps::icon`] for why this is a plain `String`, not a closed enum.
    pub icon: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    /// ▶️ The verb this action fires on the row's [`RowTarget`].
    pub verb: crate::UiText,
    #[serde(default, skip_serializing_if = "is_default_row_action_placement")]
    #[value(default, skip_serializing_if = "is_default_row_action_placement")]
    pub placement: RowActionPlacement,
    /// 🚫️ A disabled action paints and announces disabled and never dispatches ([`RowTarget::action_binding`] refuses it
    /// typed). Only a disabled action carries the flag, so an enabled row costs nothing for it.
    #[serde(default, skip_serializing_if = "is_enabled_row_action")]
    #[value(default, skip_serializing_if = "is_enabled_row_action")]
    pub disabled: bool,
    /// 💬️ Why a disabled action cannot run, producer-localized. A disabled action stays focusable (`aria-disabled`, WAI-ARIA
    /// focusable-when-disabled) and every renderer names this reason as its description (`aria-describedby`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Label>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowActionRefusal {
    /// 🔕️ The action is disabled on this row.
    Disabled,
    /// 🧮️ The target's argument map could not be admitted into one more binding.
    Credit,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowTarget {
    pub scope: crate::UiText,
    pub version: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<crate::UiMap>,
    /// 🏹️ The verb the row's primary activation fires on this target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<crate::UiText>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ContainerProps {
    #[serde(default, skip_serializing_if = "is_default_container_role")]
    #[value(default, skip_serializing_if = "is_default_container_role")]
    pub role: ContainerRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drop_overlay: Option<DropOverlaySpec>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TextProps {
    pub value: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emphasize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<crate::UiFixedMap<crate::UiText>>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ButtonProps {
    /// 🖼️ Icon key. The old `IconName` is generated per-consuming-crate via a `#[path]` mount (see
    /// `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`), not a publishable
    /// dependency this crate's `Cargo.toml` — which this packet is forbidden from editing — could
    /// take on. A plain `String` icon key is the only viable choice here; flagged as a
    /// registrar-request in `📓️terra-contract-doc-report.md` in case a shared icon crate should
    /// exist instead.
    pub icon: crate::UiText,
    pub label: Label,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SeparatorProps {}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct InputProps {
    #[serde(default, skip_serializing_if = "is_default_input_kind")]
    #[value(default, skip_serializing_if = "is_default_input_kind")]
    pub kind: InputKind,
    pub value: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    /// 🫳️ Commit convention string carried verbatim from the old wire shape (e.g. `"blur"`) — no
    /// closed set of these was found in the fleet, unlike `input_kind`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<crate::UiText>,
    /// 🔣️ Fraction digits a `InputKind::Number` field shows and commits — see [`round_ui_number`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    /// 📌️ Detents of a `InputKind::Number` field under the detent law of [`SliderProps::snaps`] against `min`/`max`
    /// (unbounded when absent): its page keys jump between them ([`ui_number_key_value`]), typing never snaps.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub snaps: crate::UiFixedList<f64>,
    /// 🔁️ A `InputKind::Number` field's `value` is the stored number; it shows and reads `stored × display_factor`
    /// ([`ui_number_display_text`]/[`ui_number_typed_value`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    /// ⛔️ The hard range a typed number must keep (`min`/`max` themselves when absent), see [`UiNumberLimits`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
    /// 🎯️ Opaque producer-owned identity for retaining an in-progress draft across
    /// revisions while the same semantic edit target remains mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub draft_target: Option<crate::UiText>,
    /// 🧾️ Decimal u64 revision of the native document projection that authored this input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub publication_revision: Option<UiPublicationRevision>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct SelectProps {
    pub value: crate::UiText,
    pub items: crate::UiFixedList<SelectItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    #[serde(default, skip_serializing_if = "SelectAppearance::is_menu")]
    #[value(default, skip_serializing_if = "SelectAppearance::is_menu")]
    pub appearance: SelectAppearance,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "SelectAppearance::is_menu")]
#[value(default, skip_serializing_if = "SelectAppearance::is_menu")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum SelectAppearance {
    #[default]
    Menu,
    Segmented,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ToggleProps {
    #[serde(default, skip_serializing_if = "ToggleAppearance::is_button")]
    #[value(default, skip_serializing_if = "ToggleAppearance::is_button")]
    pub appearance: ToggleAppearance,
    pub on: bool,
    pub icon: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Label>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[serde(default, skip_serializing_if = "ToggleAppearance::is_button")]
#[value(default, skip_serializing_if = "ToggleAppearance::is_button")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum ToggleAppearance {
    #[default]
    Button,
    Checkbox,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct KeyValueListProps {
    pub entries: crate::UiFixedList<KeyValueEntry>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct RingProps {
    pub orb_id: crate::UiText,
    pub t: f64,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct IconSelectProps {
    pub value: crate::UiText,
    pub uniform: bool,
    pub classifier_kind: crate::UiText,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ProgressProps {
    pub completed: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    pub value_text: Label,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum UiNumberScale {
    #[default]
    Linear,
    Log,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum SliderAppearance {
    #[default]
    Track,
    Dial,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct UiNumberBound {
    pub value: f64,
    #[serde(default, skip_serializing_if = "is_inclusive")]
    #[value(default, skip_serializing_if = "is_inclusive")]
    pub exclusive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<Label>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs
#[serde(default, skip_serializing_if = "is_inclusive")]
#[value(default, skip_serializing_if = "is_inclusive")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct UiNumberLimits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<UiNumberBound>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<UiNumberBound>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct SliderProps {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub snaps: crate::UiFixedList<f64>,
    #[serde(default, skip_serializing_if = "SliderAppearance::is_track")]
    #[value(default, skip_serializing_if = "SliderAppearance::is_track")]
    pub appearance: SliderAppearance,
    #[serde(default, skip_serializing_if = "UiNumberScale::is_linear")]
    #[value(default, skip_serializing_if = "UiNumberScale::is_linear")]
    pub scale: UiNumberScale,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct NumberStepperProps {
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub snaps: crate::UiFixedList<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TableProps {
    /// 🏷️ The table's accessible name — what assistive technology announces on entering it.
    pub label: Label,
    /// 🗂️ The column headers, in cell order.
    pub columns: crate::UiFixedList<Label>,
    /// ↕️ Domain name for one logical row, localized by the artifact (for example Row or Frame).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub row_label: Option<Label>,
    /// ↔️ Domain name for one logical column, localized by the artifact (for example Column or Channel).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_label: Option<Label>,
    /// 🎬️ Header of the trailing actions column a renderer adds when any row carries row actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub actions_label: Option<Label>,
    /// 🪟️ The materialised slice of the logical row list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    /// 🍕️ The materialised slice of the logical column list. `columns` and every row's `cells`
    /// contain exactly this slice and retain their logical indices through `offset`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_window: Option<TreeWindow>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProps {
    /// 📝️ The row's cells, positional to the table's `columns`.
    pub cells: crate::UiFixedList<crate::UiText>,
    /// 🔘️ Row-scoped actions, rendered in the table's trailing actions column.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ImageProps {
    pub src: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<Label>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs
#[serde(default, skip_serializing_if = "Option::is_none")]
#[value(default, skip_serializing_if = "Option::is_none")]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionProps {
    pub extension: crate::UiText,
    /// ⚠️ Decision: `crate::UiValue` is referenced, not defined, here. The packet brief's explicit
    /// "leave unresolved" list (`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`/`ActionBinding`/
    /// `MenuRef`/`Activity`/`SurfaceProps`) does not name `UiValue`, but `📋️master.md`'s "1. Contract
    /// crate" section places `UiValue` right beside `ActionId`/`Trigger`/`UiIntent` — the action
    /// model, owned by packet `contract-action`'s `🎬️action.rs`, not this file. Defining it here
    /// risks the exact duplicate-definition collision U2 calls out as worse than an unresolved name.
    pub props: crate::UiValue,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Component {
    Container(ContainerProps),
    Text(TextProps),
    Button(ButtonProps),
    Separator(SeparatorProps),
    Input(InputProps),
    Select(SelectProps),
    Toggle(ToggleProps),
    KeyValueList(KeyValueListProps),
    Slider(SliderProps),
    NumberStepper(NumberStepperProps),
    Ring(RingProps),
    IconSelect(IconSelectProps),
    Progress(ProgressProps),
    Tree(TreeProps),
    TreeSection(TreeSectionProps),
    TreeItem(TreeItemProps),
    Image(ImageProps),
    Surface(crate::SurfaceProps),
    Extension(ExtensionProps),
    Table(TableProps),
    TableRow(TableRowProps),
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiDocumentAssemblyProgress {
    pub progressed: bool,
    pub metadata_items: usize,
    pub allocated_bytes: usize,
    pub initialized_bytes: usize,
    pub compared_bytes: usize,
    pub moved_bytes: usize,
    pub complete: bool,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentAssemblyErrorKind {
    Occupied,
    MissingSurface,
    InvalidGeneration,
    ArenaFull,
    Stale,
    Closing,
    Contended,
    Poisoned,
    DuplicateNode,
    Allocation,
    MissingRoot,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiDocumentAssemblyError {
    pub kind: UiDocumentAssemblyErrorKind,
    pub allocated_bytes: usize,
    pub initialized_bytes: usize,
    pub compared_bytes: usize,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiDocumentAssemblyIdentity {
    pub generation: u64,
    pub revision: UiRevision,
    pub root: Option<UiNodeId>,
    pub layout_epoch: u64,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🦀️.rs
#[derive(Debug, Default)]
pub struct UiDocumentAssembly {
    builder: Option<UiDocumentBuilder>,
    checking: Option<UiNodeId>,
    compared: usize,
    root_ordinal: Option<usize>,
    closing: bool,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiDocumentRootIdentity(UiDocumentHandle);

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🎟️assembly/🦀️.rs

pub struct UiDocumentRead<'a> {
    guard: MutexGuard<'a, UiDocumentArena>,
    handle: UiDocumentHandle,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiNodeRecord {
    pub id: UiNodeId,
    /// 🔑️ Reconciliation key, unique only among this node's own siblings (not surface-wide).
    pub key: crate::UiText,
    pub component: crate::Component,
    pub layout: crate::LayoutSpec,
    pub style: crate::StyleSpec,
    pub activity: crate::Activity,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<TransitionHint>,
    pub accessibility: crate::AccessibilitySpec,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub bindings: UiNodeBindings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<crate::MenuRef>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub children: UiNodeChildren,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSnapshot {
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: UiNodeId,
    pub nodes: UiSnapshotNodes,
    /// 📐️ Bumped by the layout engine whenever geometry may have changed for reasons a patch does not
    /// itself carry (e.g. a host window resize) — renderers use this to decide whether cached layout
    /// results are still trustworthy without diffing every record.
    pub layout_epoch: u64,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiPatchOp {
    Upsert(UiNodeRecord),
    SetComponent {
        id: UiNodeId,
        component: crate::Component,
    },
    SetLayout {
        id: UiNodeId,
        layout: crate::LayoutSpec,
    },
    SetActivity {
        id: UiNodeId,
        activity: crate::Activity,
        disabled: bool,
    },
    SetChildren {
        id: UiNodeId,
        children: UiNodeChildren,
    },
    SetStyle {
        id: UiNodeId,
        style: crate::StyleSpec,
    },
    SetAccessibility {
        id: UiNodeId,
        accessibility: crate::AccessibilitySpec,
    },
    SetBindings {
        id: UiNodeId,
        bindings: UiNodeBindings,
    },
    SetMenu {
        id: UiNodeId,
        menu: Option<crate::MenuRef>,
    },
    /// 🗑️ Removes the node and its whole orphaned subtree. A struct variant, not a newtype: an
    /// internally-tagged enum cannot serialize a newtype whose payload is not a map, and a bare
    /// `UiNodeId` is an integer — that shape compiles clean and fails only at runtime.
    Remove {
        id: UiNodeId,
    },
    SetRoot {
        id: UiNodeId,
    },
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiPatch {
    pub surface: SurfaceId,
    pub base_revision: UiRevision,
    pub revision: UiRevision,
    pub ops: UiPatchOps,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug, Default, PartialEq)]
pub struct UiSnapshotState {
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: Option<UiNodeId>,
    pub nodes: UiNodeTable,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug, Default, PartialEq)]
pub struct UiNodeTable {
    entries: UiSnapshotNodes,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs

pub struct UiSubtreeIter<'a> {
    state: &'a UiSnapshotState,
    stack: crate::UiFixedList<UiNodeId, UI_DOCUMENT_NODES>,
    rejected: Option<UiNodeId>,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentBuildError {
    InvalidGeneration,
    ArenaFull,
    StaleHandle,
    DuplicateNode,
    NodeCapacity,
    MissingRoot,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentLeaseError {
    StaleHandle,
    Closing,
    AliasCapacity,
    PageCapacity,
    NodeIdentity,
    Contended,
    Poisoned,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiDocumentLeaseHeader {
    pub generation: u64,
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: UiNodeId,
    pub layout_epoch: u64,
    pub node_count: usize,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug)]
pub struct UiDocumentNodePage {
    generation: u64,
    revision: UiRevision,
    index: usize,
    record: UiNodeRecord,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug)]
pub struct UiDocumentBuilder {
    handle: Option<UiDocumentHandle>,
    released: bool,
    claimed: bool,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Debug)]
pub struct UiDocumentLease {
    handle: Option<UiDocumentHandle>,
    released: bool,
    claimed: bool,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiPagedTextError {
    Lease(UiDocumentLeaseError),
    MissingNode(UiNodeId),
    Cyclic,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentPublishError {
    Empty,
    Permit { fault: UiResidentFault, items: usize, bytes: usize },
    Assembly(UiDocumentAssemblyError),
    Budget,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[value(crate = "::semio_framework_value")]
pub enum SurfaceKind {
    /// 🖌️ The plainest surface a host can always satisfy, so a `SurfaceProps` built from defaults
    /// names something renderable rather than a kind no backend registered.
    #[default]
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

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs
#[serde(rename = "canvas-2d")]
#[value(rename = "canvas-2d")]
#[serde(rename = "world-3d")]
#[value(rename = "world-3d")]
#[serde(rename = "node-graph")]
#[value(rename = "node-graph")]
#[serde(rename = "text-editor")]
#[value(rename = "text-editor")]
#[serde(rename = "table")]
#[value(rename = "table")]
#[serde(rename = "paint-2d")]
#[value(rename = "paint-2d")]
#[serde(rename = "virtual-file-system")]
#[value(rename = "virtual-file-system")]
#[serde(rename = "tiled-map")]
#[value(rename = "tiled-map")]
#[serde(rename = "board-2d")]
#[value(rename = "board-2d")]
#[serde(rename = "icon-render")]
#[value(rename = "icon-render")]
#[serde(rename = "ink-canvas")]
#[value(rename = "ink-canvas")]
#[serde(rename = "graph-timeline")]
#[value(rename = "graph-timeline")]
#[serde(rename = "block-list")]
#[value(rename = "block-list")]
#[serde(rename = "diff-view")]
#[value(rename = "diff-view")]
#[serde(rename = "event-feed")]
#[value(rename = "event-feed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SurfaceDoc {
    pub bytes: crate::UiFixedBytes,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceProps {
    pub kind: SurfaceKind,
    /// 🏷️ `"<kind>@<version>"`, e.g. `"world3d@1"` — the axis a renderer gates its own per-kind decode
    /// logic on. Never validated against `kind` by this crate (see [`parse_doc_schema`]); a mismatch
    /// between the two is a `🎬️scene`-crate-level authoring bug, not a contract violation.
    pub doc_schema: crate::UiText,
    pub doc: SurfaceDoc,
    /// 🔗️ Surface-level intents — bindings that fire against the surface itself (e.g. a "focus"/
    /// "reset view" action a host chrome offers around the embedded content), as opposed to intents the
    /// embedded content's own scene graph interprets internally via `doc`'s opaque bytes.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub bindings: crate::UiNodeBindings,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceSchema<'a> {
    pub kind: &'a str,
    pub version: u32,
}

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceSchemaFault {
    /// 🕳️ `doc_schema` was the empty string.
    Empty,
    /// ✂️ No `@` separator — the string is not `"<kind>@<version>"` shaped at all.
    MissingVersionSeparator,
    /// 🪫️ An `@` was found but the kind half in front of it was empty.
    EmptyKind,
    /// 🔢️ The version half after `@` did not parse as a `u32`.
    InvalidVersion,
}
