//! 🔌 Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM components.

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub mod component {
    //! 🧩 WASI P2 component exports for the plugin world contract.

    use crate::plugin_runtime::{
        ensure_plugin_initialized, plugin_attach_backbone, plugin_create_app,
        plugin_detach_backbone, plugin_document, plugin_handle_action, plugin_handle_command,
        plugin_ingest_operations, plugin_load_document, plugin_manifest, plugin_refresh_ui,
        plugin_render_with_document,
    };
    use wit_bindgen::generate;

    generate!({
        world: "plugin-world",
        path: "../../wit",
    });

    use exports::semio::framework::plugin::Guest;
    use semio::framework::types::{
        ActionInvocationJson, CommandInvocationJson, InvocationContextJson, InvocationResponseJson, MigrateDocumentInput,
        MigrateDocumentOutput, PluginError, PluginManifestJson, UiRefreshRequestJson, UiRefreshResponseJson, WindowInputJson, WindowOutputJson,
    };

    pub struct ComponentGuest;

    impl Guest for ComponentGuest {
        fn manifest() -> PluginManifestJson {
            ensure_plugin_initialized();
            PluginManifestJson {
                json: serde_json::to_string(&plugin_manifest()).unwrap_or_else(|_| "{}".into()),
            }
        }

        fn instantiate_app(app_id: String, _instance_id: String) -> Result<u32, PluginError> {
            ensure_plugin_initialized();
            plugin_create_app(&app_id).map_err(PluginError::Message)
        }

        fn handle_action(
            instance_id: u32,
            action: ActionInvocationJson,
            context: InvocationContextJson,
        ) -> Result<InvocationResponseJson, PluginError> {
            ensure_plugin_initialized();
            let result = plugin_handle_action(instance_id, &action.json, &context.json)
                .map_err(PluginError::Message)?;
            Ok(InvocationResponseJson {
                json: serde_json::to_string(&result).unwrap_or_else(|_| "{}".into()),
            })
        }

        fn handle_command(
            instance_id: u32,
            command: CommandInvocationJson,
            context: InvocationContextJson,
        ) -> Result<InvocationResponseJson, PluginError> {
            ensure_plugin_initialized();
            let result = plugin_handle_command(instance_id, &command.json, &context.json)
                .map_err(PluginError::Message)?;
            Ok(InvocationResponseJson {
                json: serde_json::to_string(&result).unwrap_or_else(|_| "{}".into()),
            })
        }

        fn update_window(
            instance_id: u32,
            input: WindowInputJson,
        ) -> Result<WindowOutputJson, PluginError> {
            ensure_plugin_initialized();
            let node = plugin_render_with_document(instance_id, "", None, &input.json)
                .map_err(PluginError::Message)?;
            Ok(WindowOutputJson {
                json: serde_json::to_string(&node).unwrap_or_else(|_| "{}".into()),
            })
        }

        fn refresh_ui(
            instance_id: u32,
            request: UiRefreshRequestJson,
        ) -> Result<UiRefreshResponseJson, PluginError> {
            ensure_plugin_initialized();
            let json = plugin_refresh_ui(instance_id, &request.json).map_err(PluginError::Message)?;
            Ok(UiRefreshResponseJson { json })
        }

        fn migrate_document(_input: MigrateDocumentInput) -> Result<MigrateDocumentOutput, PluginError> {
            Err(PluginError::Message("migrate-document not implemented".into()))
        }

        fn apply_operations(instance_id: u32, operations_json: String) -> Result<(), PluginError> {
            ensure_plugin_initialized();
            plugin_ingest_operations(instance_id, &operations_json).map_err(PluginError::Message)
        }

        fn read_app_document(instance_id: u32) -> Result<String, PluginError> {
            ensure_plugin_initialized();
            plugin_document(instance_id).map_err(PluginError::Message)
        }

        fn load_app_document(instance_id: u32, document_json: String) -> Result<(), PluginError> {
            ensure_plugin_initialized();
            plugin_load_document(instance_id, &document_json).map_err(PluginError::Message)
        }

        fn attach_backbone(instance_id: u32, uri: String) -> Result<(), PluginError> {
            ensure_plugin_initialized();
            plugin_attach_backbone(instance_id, &uri).map_err(PluginError::Message)
        }

        fn detach_backbone(instance_id: u32) -> Result<(), PluginError> {
            ensure_plugin_initialized();
            plugin_detach_backbone(instance_id).map_err(PluginError::Message)
        }
    }

    export!(ComponentGuest);

    pub fn component_export_anchor() {}

    pub fn host_backbone_send(uri: &str, message_json: &str) -> Result<(), String> {
        semio::framework::host::backbone_send(uri, message_json)
    }

    pub fn host_backbone_poll(uri: &str) -> Result<Vec<String>, String> {
        semio::framework::host::backbone_poll(uri)
    }

    pub fn host_backbone_status(uri: &str) -> Result<String, String> {
        semio::framework::host::backbone_status(uri)
    }

    pub fn host_now_ms() -> i64 {
        semio::framework::host::now_ms()
    }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub use component::component_export_anchor;

pub mod app {
// #region app
//! 🧩 Declarative app builder and plugin trait.

use semio_framework_core::{
    effective_action_args, history_action_definitions, missing_required_args, kernel::{
        ActorId, AppEvent, CapabilityRequirement, InvocationId, InvocationResult, HostEffect, HybridLogicalTimestamp,
        InverseOperation, KernelOperation, DocumentDiff, DocumentHandle, DocumentVersion, OpEnvelope, OperationId, Rights,
        ResourceKind, SchemaId, Scope, UndoGroup, UndoPolicy,
    },
    set_active_utility_action_definition, start_introduction_action_definition, ActionArgDef, ActionRef, AppDefinition,
    AppLabelsOverlay, ActionDefinition, ActionKind, CommandDefinition, CommandRef, CommandScope, Contribution, DialogDefinition, ExampleDefinition,
    IntroductionAdvance, IntroductionAnchor, IntroductionDefinition, Keybinding, MediaForm, MediaPortDirection, MediaPortSpec,
    ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, PluginManifest, ProgramDefinition, UtilityDefinition,
    UtilityRef, ViewState, WindowKindDefinition, WindowKinds, SET_ACTIVE_UTILITY_ACTION_ID, START_INTRODUCTION_ACTION_ID,
};
use ui_wgpu::{
    collect_window_kind_ids_from_layout, ui_control_to_node, ui_stack_vertical, ui_text, ActionDescriptor, NamedLayout, UiButtonNode,
    UiControlNode, UiFieldNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiNode, UiSectionNode, UiTreeItemNode, UiTreeNode,
    UiTreeSectionNode, WindowEngagement, WindowEngagementSlot, WindowLayout, WindowMeasure, WindowOptions, SurfaceKind,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use vcs::{
    build_history_columns, create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope,
    DocumentVcsStore, HistoryColumn, Operation, StudioConflict,
};

pub struct ModeSpec {
    pub id: String,
    pub label: String,
    pub utilities: Vec<UtilityRef>,
    pub layout_id: Option<String>,
    pub commands: Vec<CommandRef>,
}

pub struct WindowKindSpec {
    pub id: String,
    pub label: String,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    pub icon_id: Option<String>,
    pub measures: Vec<WindowMeasure>,
    pub engagement: Option<WindowEngagement>,
    pub actions: Vec<ActionRef>,
    pub utilities: Vec<UtilityRef>,
}

/// 🌳 A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it) — exactly one of the two.
pub struct PanelTabSpec {
    pub kind: PanelTabKind,
    pub label: String,
    pub group: PanelGroup,
    pub body_key: Option<String>,
    pub children: Vec<PanelTabSpec>,
}

impl PanelTabSpec {
    /// 🍃 An app-declared leaf tab; `group` is only meaningful on the root entry passed to `.panel_tab_tree`.
    pub fn leaf(id: impl Into<String>, label: impl Into<String>, group: PanelGroup, body_key: impl Into<String>) -> Self {
        Self { kind: PanelTabKind::App(id.into()), label: label.into(), group, body_key: Some(body_key.into()), children: Vec::new() }
    }

    /// 🌳 An app-declared branch tab; its `children` render as the tab row below it when active.
    pub fn group(id: impl Into<String>, label: impl Into<String>, group: PanelGroup, children: Vec<PanelTabSpec>) -> Self {
        Self { kind: PanelTabKind::App(id.into()), label: label.into(), group, body_key: None, children }
    }

    /// 🏛️ A framework-predefined tab — only the framework shell itself should ever pass a
    /// non-`App` `PanelTabKind` here; plugins must go through `leaf`/`group`.
    pub fn framework(kind: PanelTabKind, label: impl Into<String>, group: PanelGroup, body_key: Option<String>, children: Vec<PanelTabSpec>) -> Self {
        Self { kind, label: label.into(), group, body_key, children }
    }
}

/// 🌳 Asserts every tab in the tree has a non-empty, unique id and sets exactly one of `body_key`/`children`.
fn validate_panel_tab_spec(app_id: &str, tab: &PanelTabSpec, seen_ids: &mut HashSet<String>) {
    let id = tab.kind.id_str();
    assert!(!id.trim().is_empty(), "app {} panel tab id must be non-empty", app_id);
    assert!(seen_ids.insert(id.to_string()), "app {} duplicate panel tab id {}", app_id, id);
    assert!(
        tab.body_key.is_some() != !tab.children.is_empty(),
        "app {} panel tab {} must set exactly one of body_key or children",
        app_id,
        id
    );
    if let Some(body_key) = &tab.body_key {
        assert!(!body_key.trim().is_empty(), "app {} panel tab {} body_key must be non-empty", app_id, id);
    }
    for child in &tab.children {
        validate_panel_tab_spec(app_id, child, seen_ids);
    }
}

/// 🌳 Converts one plugin-declared `PanelTabSpec` (recursively) into a `PanelTabDefinition`.
fn panel_tab_spec_to_definition(tab: PanelTabSpec) -> PanelTabDefinition {
    PanelTabDefinition {
        kind: tab.kind,
        label: tab.label,
        group: tab.group,
        body_key: tab.body_key,
        children: tab.children.into_iter().map(panel_tab_spec_to_definition).collect(),
    }
}

/// 📝 Asserts every `ActionArgDef` in `args` (belonging to `owner`, e.g. an action or dialog id) has
/// a non-empty, unique id and that any `Select` control declares at least one option — shared by
/// per-action arg validation and dialog arg validation so both stay in lockstep.
fn validate_arg_defs(app_id: &str, owner: &str, args: &[ActionArgDef]) {
    let mut arg_ids = HashSet::new();
    for arg in args {
        assert!(
            arg_ids.insert(arg.id.clone()),
            "app {} {} declares duplicate arg id {}",
            app_id,
            owner,
            arg.id
        );
        if let semio_framework_core::ActionArgControl::Select { options } = &arg.control {
            assert!(
                !options.is_empty(),
                "app {} {} arg {} is a Select with no options",
                app_id,
                owner,
                arg.id
            );
        }
    }
}

pub struct KeybindingSpec {
    pub keys: String,
    pub controller_id: String,
    pub action: String,
}

//#region 🔖ResourceKind
/// 🗂️ `OsMediaCapability`/`ResourceKindSpec` now live in `semio-framework-core` (both this crate and
/// `semio-framework-os` already depend on it) the same way `OsMediaFormat` already does — re-exported
/// here verbatim instead of duplicated, so `AppBuilder::resource_kind(...)` and
/// `semio_framework_os`'s resource catalog registry share one definition.
pub use semio_framework_core::{OsMediaCapability, ResourceKindSpec};
//#endregion 🔖ResourceKind

//#region 🔖MediaPort
/// 🧬 `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
/// `ResourceKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
/// without a direct `semio-framework-core` dependency.
pub use semio_framework_core::{MediaClass, MediaType};
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `DocumentApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
pub use semio_framework_core::{Media, MediaError, MediaFingerprint, MediaPayload};
//#endregion 🔖MediaPort

pub struct AppBuilder {
    id: String,
    label: String,
    document: Vec<String>,
    icon_id: Option<String>,
    controller_id: String,
    modes: Vec<ModeSpec>,
    default_mode_id: Option<String>,
    window_kinds: Vec<WindowKindSpec>,
    panel_tabs: Vec<PanelTabSpec>,
    keybindings: Vec<KeybindingSpec>,
    actions: Vec<ActionDefinition>,
    utilities: Vec<UtilityDefinition>,
    commands: Vec<CommandDefinition>,
    named_layouts: Vec<NamedLayout>,
    default_layout: Option<WindowLayout>,
    terminologies: Vec<String>,
    terminology_documents: std::collections::HashMap<String, Vec<String>>,
    introduction: Option<IntroductionDefinition>,
    dialogs: Vec<DialogDefinition>,
    resource_kinds: Vec<ResourceKindSpec>,
    media_inputs: Vec<MediaPortSpec>,
    media_outputs: Vec<MediaPortSpec>,
}

impl AppBuilder {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            controller_id: id.clone(),
            id,
            label: label.into(),
            document: Vec::new(),
            icon_id: None,
            modes: Vec::new(),
            default_mode_id: None,
            window_kinds: Vec::new(),
            panel_tabs: Vec::new(),
            keybindings: Vec::new(),
            actions: Vec::new(),
            utilities: Vec::new(),
            commands: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_documents: std::collections::HashMap::new(),
            introduction: None,
            dialogs: Vec::new(),
            resource_kinds: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
        }
    }

    /// 🗂️ Declares one resource kind this app produces/consumes (see `ResourceKindSpec`). Repeatable.
    pub fn resource_kind(mut self, spec: ResourceKindSpec) -> Self {
        self.resource_kinds.push(spec);
        self
    }

    /// 🔌 Declares one media graph input port this app accepts (see `MediaPortSpec`). Repeatable;
    /// validated in `build_definition` (non-empty/unique id, `direction` must be `In`).
    pub fn media_input(mut self, spec: MediaPortSpec) -> Self {
        self.media_inputs.push(spec);
        self
    }

    /// 🔌 Declares one media graph output port this app produces (see `MediaPortSpec`). Repeatable;
    /// validated in `build_definition` (non-empty/unique id, `direction` must be `Out`, `MediaForm::Any`
    /// is rejected — `Any` is only ever legal on the accepting/input side, see `media_types_compatible`).
    pub fn media_output(mut self, spec: MediaPortSpec) -> Self {
        self.media_outputs.push(spec);
        self
    }

    /// 🗣️ Declares an alternative terminology id this app supports beyond the implicit "native" default.
    pub fn terminology(mut self, id: impl Into<String>) -> Self {
        self.terminologies.push(id.into());
        self
    }

    /// 🗺️ Replaces the full document path (product + app segments) while terminology `id` is active;
    /// `id` must also be declared via `terminology` — validated in `build_definition`.
    pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.terminology_documents.insert(id.into(), document.into_iter().map(Into::into).collect());
        self
    }

    /// @emoji 🎓 Declares this app's first-run introduction walkthrough. Step anchors/advance
    /// conditions are validated against declared window kinds/utilities/actions/panel tabs in
    /// `build_definition`; declaring one auto-injects the `startIntroduction` action.
    pub fn introduction(mut self, introduction: IntroductionDefinition) -> Self {
        self.introduction = Some(introduction);
        self
    }

    /// @emoji 🗨️ Declares a modal form dialog (repeatable). `submit_action`/`cancel_action` and its
    /// `args` are validated in `build_definition`; opened only via `HostEffect::OpenDialog`.
    pub fn dialog(mut self, dialog: DialogDefinition) -> Self {
        self.dialogs.push(dialog);
        self
    }

    pub fn icon_id(mut self, icon_id: impl Into<String>) -> Self {
        self.icon_id = Some(icon_id.into());
        self
    }

    pub fn document<I, S>(mut self, document: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.document = document.into_iter().map(Into::into).collect();
        self
    }

    pub fn mode(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.modes.push(ModeSpec {
            id: id.into(),
            label: label.into(),
            utilities: Vec::new(),
            layout_id: None,
            commands: Vec::new(),
        });
        self
    }

    /// 🎛️ Scopes commands to a mode — references ids declared via `.mode_command()`/`.command()`
    /// (each of which must be `CommandScope::Mode`).
    pub fn mode_commands(mut self, mode_id: impl AsRef<str>, command_ids: Vec<CommandRef>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.commands = command_ids;
        }
        self
    }

    pub fn mode_layout(mut self, mode_id: impl AsRef<str>, layout_id: impl Into<String>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.layout_id = Some(layout_id.into());
        }
        self
    }

    /// 🧰 Scopes utilities to a mode — references ids declared via `.utility()`/`.utility_simple()`.
    pub fn mode_utilities(mut self, mode_id: impl AsRef<str>, utility_ids: Vec<UtilityRef>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.utilities = utility_ids;
        }
        self
    }

    pub fn default_mode_id(mut self, id: impl Into<String>) -> Self {
        self.default_mode_id = Some(id.into());
        self
    }

    pub fn window_kind(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        body_key: impl Into<String>,
        surface_kind: SurfaceKind,
    ) -> Self {
        self.window_kinds.push(WindowKindSpec {
            id: id.into(),
            label: label.into(),
            body_key: body_key.into(),
            surface_kind,
            icon_id: None,
            measures: Vec::new(),
            engagement: None,
            actions: Vec::new(),
            utilities: Vec::new(),
        });
        self
    }

    pub fn window_kind_with_engagement(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        body_key: impl Into<String>,
        surface_kind: SurfaceKind,
        engagement: WindowEngagement,
    ) -> Self {
        self.window_kinds.push(WindowKindSpec {
            id: id.into(),
            label: label.into(),
            body_key: body_key.into(),
            surface_kind,
            icon_id: None,
            measures: Vec::new(),
            engagement: Some(engagement),
            actions: Vec::new(),
            utilities: Vec::new(),
        });
        self
    }

    /// 🎛️ Attaches measure controls (sliders/selects/toggles/groups) to an already-declared window kind.
    pub fn window_kind_measures(mut self, window_kind_id: impl AsRef<str>, measures: Vec<WindowMeasure>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.measures = measures;
        }
        self
    }

    /// 📇 Scopes actions to a window kind — references ids declared via `.operation()/.view_action()/.shell_action()`.
    pub fn window_kind_actions(mut self, window_kind_id: impl AsRef<str>, action_ids: Vec<ActionRef>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.actions = action_ids;
        }
        self
    }

    /// 🧰 Scopes utilities to a window kind — references ids declared via `.utility()`/`.utility_simple()`. Mirrors
    /// `window_kind_actions`: the referenced utility ids are validated to resolve in `build_definition`.
    pub fn window_kind_utilities(mut self, window_kind_id: impl AsRef<str>, utility_ids: Vec<UtilityRef>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.utilities = utility_ids;
        }
        self
    }

    pub fn named_layout(mut self, layout: NamedLayout) -> Self {
        self.named_layouts.push(layout);
        self
    }

    pub fn default_layout(mut self, layout: WindowLayout) -> Self {
        self.default_layout = Some(layout);
        self
    }

    pub fn panel_tab(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        group: PanelGroup,
        body_key: impl Into<String>,
    ) -> Self {
        self.panel_tabs.push(PanelTabSpec::leaf(id, label, group, body_key));
        self
    }

    /// 🌳 Declares a root panel tab that may itself be a nested tree — build `tab` via `PanelTabSpec::leaf`/`PanelTabSpec::group`.
    pub fn panel_tab_tree(mut self, tab: PanelTabSpec) -> Self {
        self.panel_tabs.push(tab);
        self
    }

    /// 🏛️ Declares a framework-predefined panel tab (workbench/display/details/settings category or
    /// leaf) — only the framework shell itself should call this; plugins must use `.panel_tab()`/`.panel_tab_tree()`.
    pub fn panel_tab_framework(mut self, tab: PanelTabSpec) -> Self {
        self.panel_tabs.push(tab);
        self
    }

    pub fn keybinding(mut self, keys: impl Into<String>, action: impl Into<String>) -> Self {
        self.keybindings.push(KeybindingSpec {
            keys: keys.into(),
            controller_id: self.controller_id.clone(),
            action: action.into(),
        });
        self
    }

    /// @emoji ✏️ Declares a document-mutating action — dispatched as VCS operations with a true inverse.
    pub fn operation(self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.action_with(ActionDefinition::new(id, label, ActionKind::Operation))
    }

    /// @emoji 👁️ Declares an ephemeral view action (camera, selection, hover, active utility) — not recorded in history.
    pub fn view_action(self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.action_with(ActionDefinition::new(id, label, ActionKind::View))
    }

    /// @emoji 🐚 Declares a shell-only effect action (navigate, export, spawn) — no document mutation.
    pub fn shell_action(self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.action_with(ActionDefinition::new(id, label, ActionKind::Shell))
    }

    /// @emoji 📇 Declares a fully specified action (icon, args, keybinding, palette visibility, category).
    pub fn action_with(mut self, action: ActionDefinition) -> Self {
        self.actions.push(action);
        self
    }

    /// @emoji 📝 Attaches typed argument declarations to an already-declared action (post-hoc, mirroring
    /// `window_kind_actions`). If the id isn't declared yet at call time the args are dropped; the
    /// mismatch surfaces in `build_definition`, which asserts every declared action's args are consistent.
    pub fn action_args(mut self, action_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
        let action_id = action_id.as_ref();
        if let Some(action) = self.actions.iter_mut().find(|entry| entry.id == action_id) {
            action.args = args;
        }
        self
    }

    /// @emoji 🎛️ Declares a fully specified command. There are no window-level commands — only
    /// `CommandScope::App`/`CommandScope::Mode` may be declared here (`Os`/`Plugin` are rejected in
    /// `build_definition`); `Mode`-scope commands must additionally be referenced via `.mode_commands()`.
    pub fn command(mut self, command: CommandDefinition) -> Self {
        self.commands.push(command);
        self
    }

    /// @emoji 🎛️ Declares an app-scope command (applies whenever this app is focused, in any mode).
    pub fn app_command(self, id: impl Into<String>, label: impl Into<String>, category: impl Into<String>) -> Self {
        self.command(CommandDefinition::new(id, label, CommandScope::App, category))
    }

    /// @emoji 🎛️ Declares a mode-scope command definition — still requires `.mode_commands(mode_id, ..)`
    /// to actually scope it to the mode(s) it applies to.
    pub fn mode_command(self, id: impl Into<String>, label: impl Into<String>, category: impl Into<String>) -> Self {
        self.command(CommandDefinition::new(id, label, CommandScope::Mode, category))
    }

    /// @emoji 📝 Attaches typed argument declarations to an already-declared command (post-hoc,
    /// mirroring `action_args`).
    pub fn command_args(mut self, command_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
        let command_id = command_id.as_ref();
        if let Some(command) = self.commands.iter_mut().find(|entry| entry.id == command_id) {
            command.args = args;
        }
        self
    }

    /// @emoji 🧰 Declares an interactive utility this app exposes (referenced by `window_kind_utilities`/`mode_utilities`).
    pub fn utility(mut self, utility: UtilityDefinition) -> Self {
        self.utilities.push(utility);
        self
    }

    /// @emoji 🧰 Declares a utility with default settings (no group/keys/cursor/category, gates actions while active).
    pub fn utility_simple(self, id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<String>) -> Self {
        self.utility(UtilityDefinition::new(id, label, icon_id))
    }

    /// @emoji 🧷 Keybinding-vs-action-registry consistency is only enforced for apps that declare
    /// actions via `.operation()`/`.view_action()`/`.shell_action()` — apps with an empty action
    /// registry keybind directly against controller actions instead, so there is nothing to check.
    pub fn build_definition(self) -> AppDefinition {
        assert!(
            !self.document.is_empty() && self.document.iter().all(|segment| !segment.trim().is_empty()),
            "app {} document must contain non-empty segments",
            self.id
        );
        for (terminology_id, document) in &self.terminology_documents {
            assert!(
                self.terminologies.iter().any(|id| id == terminology_id),
                "app {} declares terminology_document for undeclared terminology {}",
                self.id,
                terminology_id
            );
            assert!(
                !document.is_empty() && document.iter().all(|segment| !segment.trim().is_empty()),
                "app {} terminology_document for {} must contain non-empty segments",
                self.id,
                terminology_id
            );
        }
        assert!(
            !self.window_kinds.is_empty(),
            "app {} must declare at least one window kind",
            self.id
        );
        assert!(
            !self.modes.is_empty(),
            "app {} must declare at least one mode",
            self.id
        );
        let mut window_kind_ids = HashSet::new();
        for window in &self.window_kinds {
            assert!(!window.id.trim().is_empty(), "app {} window kind id must be non-empty", self.id);
            assert!(
                !window.body_key.trim().is_empty(),
                "app {} window kind {} body_key must be non-empty",
                self.id,
                window.id
            );
            assert!(
                window_kind_ids.insert(window.id.clone()),
                "app {} duplicate window kind id {}",
                self.id,
                window.id
            );
        }
        let mut panel_tab_ids = HashSet::new();
        for tab in &self.panel_tabs {
            validate_panel_tab_spec(&self.id, tab, &mut panel_tab_ids);
        }
        let mut layout_window_ids = Vec::new();
        if let Some(layout) = &self.default_layout {
            layout_window_ids.extend(collect_window_kind_ids_from_layout(layout));
        }
        for named in &self.named_layouts {
            layout_window_ids.extend(collect_window_kind_ids_from_layout(&named.layout));
        }
        for window_kind_id in layout_window_ids {
            assert!(
                window_kind_ids.contains(&window_kind_id),
                "app {} layout references undeclared window kind {}",
                self.id,
                window_kind_id
            );
        }
        let default_mode_id = self
            .default_mode_id
            .clone()
            .unwrap_or_else(|| self.modes[0].id.clone());
        assert!(
            self.modes.iter().any(|mode| mode.id == default_mode_id),
            "app {} default_mode_id {} does not reference a declared mode",
            self.id,
            default_mode_id
        );
        let mut declared_action_ids = HashSet::new();
        for action in &self.actions {
            assert!(
                declared_action_ids.insert(action.id.clone()),
                "app {} duplicate action id {}",
                self.id,
                action.id
            );
            validate_arg_defs(&self.id, &format!("action {}", action.id), &action.args);
        }
        let mut declared_utility_ids = HashSet::new();
        for utility in &self.utilities {
            assert!(!utility.id.trim().is_empty(), "app {} utility id must be non-empty", self.id);
            assert!(
                declared_utility_ids.insert(utility.id.clone()),
                "app {} duplicate utility id {}",
                self.id,
                utility.id
            );
        }
        let mut declared_command_scopes: HashMap<String, CommandScope> = HashMap::new();
        for command in &self.commands {
            assert!(
                matches!(command.scope, CommandScope::App | CommandScope::Mode),
                "app {} command {} must be declared CommandScope::App or CommandScope::Mode (Os/Plugin commands are not declared via AppBuilder)",
                self.id,
                command.id
            );
            assert!(
                declared_command_scopes.insert(command.id.clone(), command.scope).is_none(),
                "app {} duplicate command id {}",
                self.id,
                command.id
            );
            validate_arg_defs(&self.id, &format!("command {}", command.id), &command.args);
        }
        let app_declared_actions = !self.actions.is_empty();
        let mut actions = self.actions;
        for history_action in history_action_definitions() {
            if declared_action_ids.insert(history_action.id.clone()) {
                actions.push(history_action);
            }
        }
        if !self.utilities.is_empty() && declared_action_ids.insert(SET_ACTIVE_UTILITY_ACTION_ID.to_string()) {
            actions.push(set_active_utility_action_definition());
        }
        if self.introduction.is_some() && declared_action_ids.insert(START_INTRODUCTION_ACTION_ID.to_string()) {
            actions.push(start_introduction_action_definition());
        }
        let mut bound_keys: HashSet<String> = self.keybindings.iter().map(|binding| binding.keys.clone()).collect();
        let mut keybindings: Vec<Keybinding> = self
            .keybindings
            .into_iter()
            .map(|binding| Keybinding {
                keys: binding.keys,
                action: ActionDescriptor {
                    controller_id: binding.controller_id,
                    action: binding.action,
                    args: None,
                },
            })
            .collect();
        for history_action in actions.iter().filter(|action| action.kind == ActionKind::History) {
            if let Some(keys) = &history_action.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding {
                        keys: keys.clone(),
                        action: ActionDescriptor {
                            controller_id: self.controller_id.clone(),
                            action: history_action.id.clone(),
                            args: None,
                        },
                    });
                }
            }
        }
        for utility in &self.utilities {
            if let Some(keys) = &utility.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding {
                        keys: keys.clone(),
                        action: ActionDescriptor {
                            controller_id: self.controller_id.clone(),
                            action: SET_ACTIVE_UTILITY_ACTION_ID.to_string(),
                            args: Some(serde_json::json!({ "utilityId": utility.id })),
                        },
                    });
                }
            }
        }
        if app_declared_actions {
            for binding in &keybindings {
                assert!(
                    declared_action_ids.contains(&binding.action.action),
                    "app {} keybinding {} references undeclared action {}",
                    self.id,
                    binding.keys,
                    binding.action.action
                );
            }
        }
        for window in &self.window_kinds {
            for action_ref in &window.actions {
                assert!(
                    declared_action_ids.contains(action_ref.as_str()),
                    "app {} window kind {} references undeclared action {}",
                    self.id,
                    window.id,
                    action_ref.as_str()
                );
            }
            for utility_ref in &window.utilities {
                assert!(
                    declared_utility_ids.contains(utility_ref.as_str()),
                    "app {} window kind {} references undeclared utility {}",
                    self.id,
                    window.id,
                    utility_ref.as_str()
                );
            }
        }
        for mode in &self.modes {
            for command_ref in &mode.commands {
                assert!(
                    declared_command_scopes.get(command_ref.as_str()).copied() == Some(CommandScope::Mode),
                    "app {} mode {} references undeclared or non-Mode-scope command {}",
                    self.id,
                    mode.id,
                    command_ref.as_str()
                );
            }
            for utility_ref in &mode.utilities {
                assert!(
                    declared_utility_ids.contains(utility_ref.as_str()),
                    "app {} mode {} references undeclared utility {}",
                    self.id,
                    mode.id,
                    utility_ref.as_str()
                );
            }
        }
        let mode_referenced_commands: HashSet<&str> = self
            .modes
            .iter()
            .flat_map(|mode| mode.commands.iter().map(|command_ref| command_ref.as_str()))
            .collect();
        for (id, scope) in &declared_command_scopes {
            assert!(
                *scope != CommandScope::Mode || mode_referenced_commands.contains(id.as_str()),
                "app {} mode-scope command {} is not referenced by any mode",
                self.id,
                id
            );
        }
        if let Some(introduction) = &self.introduction {
            assert!(
                !introduction.steps.is_empty(),
                "app {} introduction must declare at least one step",
                self.id
            );
            let mut step_ids = HashSet::new();
            for step in &introduction.steps {
                assert!(!step.id.trim().is_empty(), "app {} introduction step id must be non-empty", self.id);
                assert!(
                    step_ids.insert(step.id.clone()),
                    "app {} duplicate introduction step id {}",
                    self.id,
                    step.id
                );
                match &step.anchor {
                    IntroductionAnchor::Screen | IntroductionAnchor::Navbar | IntroductionAnchor::Footer | IntroductionAnchor::Slot(_) => {}
                    IntroductionAnchor::WindowKind(id) => assert!(
                        window_kind_ids.contains(id),
                        "app {} introduction step {} anchors undeclared window kind {}",
                        self.id,
                        step.id,
                        id
                    ),
                    IntroductionAnchor::Utility(utility_ref) => assert!(
                        declared_utility_ids.contains(utility_ref.as_str()),
                        "app {} introduction step {} anchors undeclared utility {}",
                        self.id,
                        step.id,
                        utility_ref.as_str()
                    ),
                    IntroductionAnchor::Action(action_ref) => assert!(
                        declared_action_ids.contains(action_ref.as_str()),
                        "app {} introduction step {} anchors undeclared action {}",
                        self.id,
                        step.id,
                        action_ref.as_str()
                    ),
                    IntroductionAnchor::PanelTab(id) => assert!(
                        panel_tab_ids.contains(id),
                        "app {} introduction step {} anchors undeclared panel tab {}",
                        self.id,
                        step.id,
                        id
                    ),
                }
                match &step.advance {
                    IntroductionAdvance::Next => {}
                    IntroductionAdvance::Action(action_ref) => assert!(
                        declared_action_ids.contains(action_ref.as_str()),
                        "app {} introduction step {} advance references undeclared action {}",
                        self.id,
                        step.id,
                        action_ref.as_str()
                    ),
                    IntroductionAdvance::Utility(utility_ref) => assert!(
                        declared_utility_ids.contains(utility_ref.as_str()),
                        "app {} introduction step {} advance references undeclared utility {}",
                        self.id,
                        step.id,
                        utility_ref.as_str()
                    ),
                }
            }
        }
        let mut dialog_ids = HashSet::new();
        for dialog in &self.dialogs {
            assert!(!dialog.id.trim().is_empty(), "app {} dialog id must be non-empty", self.id);
            assert!(
                dialog_ids.insert(dialog.id.clone()),
                "app {} duplicate dialog id {}",
                self.id,
                dialog.id
            );
            assert!(
                declared_action_ids.contains(dialog.submit_action.as_str()),
                "app {} dialog {} submit_action references undeclared action {}",
                self.id,
                dialog.id,
                dialog.submit_action.as_str()
            );
            if let Some(cancel_action) = &dialog.cancel_action {
                assert!(
                    declared_action_ids.contains(cancel_action.as_str()),
                    "app {} dialog {} cancel_action references undeclared action {}",
                    self.id,
                    dialog.id,
                    cancel_action.as_str()
                );
            }
            validate_arg_defs(&self.id, &format!("dialog {}", dialog.id), &dialog.args);
        }
        let mut media_port_ids = HashSet::new();
        for port in self.media_inputs.iter().chain(self.media_outputs.iter()) {
            assert!(!port.id.trim().is_empty(), "app {} media port id must be non-empty", self.id);
            assert!(
                media_port_ids.insert(port.id.clone()),
                "app {} duplicate media port id {}",
                self.id,
                port.id
            );
        }
        for port in &self.media_inputs {
            assert!(
                port.direction == MediaPortDirection::In,
                "app {} media input {} must declare direction In",
                self.id,
                port.id
            );
        }
        for port in &self.media_outputs {
            assert!(
                port.direction == MediaPortDirection::Out,
                "app {} media output {} must declare direction Out",
                self.id,
                port.id
            );
            assert!(
                !matches!(port.media_type.form, MediaForm::Any),
                "app {} media output {} must not declare MediaForm::Any (Any is only legal on inputs, see media_types_compatible)",
                self.id,
                port.id
            );
        }
        AppDefinition {
            id: self.id,
            label: self.label,
            document: self.document,
            icon_id: self.icon_id,
            controller_id: self.controller_id,
            modes: Modes::try_from(
                self.modes
                    .into_iter()
                    .map(|mode| ModeDefinition {
                        id: mode.id,
                        label: mode.label,
                        utilities: mode.utilities,
                        layout_id: mode.layout_id,
                        commands: mode.commands,
                    })
                    .collect::<Vec<_>>(),
            )
            .expect("app must declare at least one mode (checked above)"),
            default_mode_id,
            window_kinds: WindowKinds::try_from(
                self.window_kinds
                    .into_iter()
                    .map(|window| WindowKindDefinition {
                        id: window.id,
                        label: window.label,
                        body_key: window.body_key,
                        surface_kind: window.surface_kind,
                        icon_id: window.icon_id,
                        options: WindowOptions {
                            measures: window.measures,
                            engagement: window.engagement.map_or(WindowEngagementSlot::None, WindowEngagementSlot::Some),
                        },
                        actions: window.actions,
                        utilities: window.utilities,
                        params_schema: None,
                        document_projection_schema: None,
                        input_event_schema: None,
                        output_schema: None,
                        capabilities: vec![],
                    })
                    .collect::<Vec<_>>(),
            )
            .expect("app must declare at least one window kind (checked above)"),
            panel_tabs: self.panel_tabs.into_iter().map(panel_tab_spec_to_definition).collect(),
            keybindings,
            actions,
            utilities: self.utilities,
            commands: self.commands,
            named_layouts: self.named_layouts,
            default_layout: self.default_layout,
            terminologies: self.terminologies,
            terminology_documents: self.terminology_documents,
            introduction: self.introduction,
            dialogs: self.dialogs,
            media_inputs: self.media_inputs,
            media_outputs: self.media_outputs,
            resource_kinds: self.resource_kinds,
        }
    }
}

//#region 🔖PanelKit
// 🌳 Shared panel-tree builders — lifts the verbatim-duplicated `tree_item*`/`selection_ids` helpers
// and the `build_document_tree`/`build_inspector_tree`/`build_catalogue_tree` skeleton found across
// ~15 plugin crates (flow, procedural, layout, gis, puzzle, sequence, trinity, dag, …) into the SDK.

/// 🌳 A bare tree item — thin wrapper over `UiTreeItemNode::base`.
pub fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode::base(id, label)
}

/// 🌳 A tree item with a description line.
pub fn tree_item_desc(id: impl Into<String>, label: impl Into<String>, description: Option<String>) -> UiTreeItemNode {
    UiTreeItemNode { description, ..UiTreeItemNode::base(id, label) }
}

/// 🌳 A tree item that dispatches `action` on click.
pub fn tree_item_with_action(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    action: ActionDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode { description, action: Some(action), ..UiTreeItemNode::base(id, label) }
}

/// 🌳 A draggable tree item: `drag_data` is a JSON object whose entries become the item's
/// MIME-type -> payload drag-data map (string values are used verbatim; non-string values are
/// serialized), e.g. `json!({ "application/x-my-widget": descriptor.to_string() })`. Generalizes the
/// single-hardcoded-MIME-key pattern duplicated per app (each app previously baked its own MIME
/// constant into this function) — the caller now supplies the key(s) explicitly.
pub fn tree_item_with_action_draggable(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    action: ActionDescriptor,
    drag_data: &Value,
) -> UiTreeItemNode {
    let mut item = tree_item_with_action(id, label, description, action);
    item.draggable = Some(true);
    item.drag_data = drag_data.as_object().map(|entries| {
        entries
            .iter()
            .map(|(key, value)| (key.clone(), value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string())))
            .collect()
    });
    item
}

/// 🎯 Parses a selection-action's `ids` array arg into a plain `Vec<String>` — the shape used by the
/// majority of duplicate copies (`layout`, `gis`, `presentation`, …). A handful of apps additionally
/// fall back to a singular `id`/`nodeId`/`nodeIds` key (`puzzle`, `sequence`, `trinity`, `procedural`,
/// `mindmap`); those apps keep their own fallback wrapper around this shared core for now.
pub fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

/// 🌳 Fluent builder for the `build_document_tree`/`build_inspector_tree`/`build_catalogue_tree`
/// skeleton duplicated across plugin crates: namespaced item ids, sections (optionally substituting a
/// single "(none)" placeholder item for the empty state), a selected/highlighted id set, a
/// selection-change action, and a drop action — ending in `.build()` -> a `UiNode::Tree`.
pub struct PanelTreeBuilder {
    namespace: String,
    sections: Vec<UiTreeSectionNode>,
    selected_ids: Option<Vec<String>>,
    highlighted_ids: Option<Vec<String>>,
    selection_change: Option<ActionDescriptor>,
    drop_action: Option<ActionDescriptor>,
}

impl PanelTreeBuilder {
    /// 🌳 `namespace` prefixes every id built via `.item_id()`, e.g. `"flow-play-document"`.
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            sections: Vec::new(),
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        }
    }

    /// 🌳 Builds a namespaced item id: `"{namespace}.{kind}.{id}"`.
    pub fn item_id(&self, kind: &str, id: &str) -> String {
        format!("{}.{kind}.{id}", self.namespace)
    }

    /// 🌳 Adds a section verbatim.
    pub fn section(mut self, id: impl Into<String>, label: Option<String>, default_open: bool, items: Vec<UiTreeItemNode>) -> Self {
        self.sections.push(UiTreeSectionNode { id: id.into(), label, default_open: Some(default_open), items, loading: None });
        self
    }

    /// 🌳 Adds a section, substituting a single "(none)"-style placeholder item when `items` is empty —
    /// the empty-state pattern duplicated in `build_document_tree`/`build_catalogue_tree` across apps.
    pub fn section_or_placeholder(
        mut self,
        id: impl Into<String>,
        label: Option<String>,
        default_open: bool,
        items: Vec<UiTreeItemNode>,
        placeholder_label: impl Into<String>,
    ) -> Self {
        let id = id.into();
        let items = if items.is_empty() { vec![tree_item(format!("{id}.empty"), placeholder_label)] } else { items };
        self.sections.push(UiTreeSectionNode { id, label, default_open: Some(default_open), items, loading: None });
        self
    }

    pub fn selected(mut self, ids: Vec<String>) -> Self {
        self.selected_ids = Some(ids);
        self
    }

    pub fn highlighted(mut self, ids: Vec<String>) -> Self {
        self.highlighted_ids = Some(ids);
        self
    }

    pub fn selection_change(mut self, action: ActionDescriptor) -> Self {
        self.selection_change = Some(action);
        self
    }

    pub fn drop_action(mut self, action: ActionDescriptor) -> Self {
        self.drop_action = Some(action);
        self
    }

    pub fn build(self) -> UiNode {
        UiNode::Tree(UiTreeNode {
            sections: self.sections,
            loading: None,
            selected_ids: self.selected_ids,
            highlighted_ids: self.highlighted_ids,
            selection_change: self.selection_change,
            drop_action: self.drop_action,
        })
    }
}

#[cfg(test)]
mod panel_kit_tests {
    use super::*;

    #[test]
    fn tree_item_builds_a_bare_item() {
        let item = tree_item("ns.kind.a", "A");
        assert_eq!(item.id, "ns.kind.a");
        assert_eq!(item.label, "A");
        assert!(item.description.is_none());
        assert!(item.action.is_none());
    }

    #[test]
    fn tree_item_with_action_draggable_maps_json_object_to_string_drag_data() {
        let action = ActionDescriptor { controller_id: "app".into(), action: "addWidget".into(), args: None };
        let item = tree_item_with_action_draggable(
            "ns.kind.a",
            "A",
            None,
            action,
            &serde_json::json!({ "application/x-widget": "{\"kind\":\"a\"}" }),
        );
        assert_eq!(item.draggable, Some(true));
        assert_eq!(
            item.drag_data.unwrap().get("application/x-widget").map(String::as_str),
            Some("{\"kind\":\"a\"}")
        );
    }

    #[test]
    fn selection_ids_reads_the_ids_array_arg() {
        let args = serde_json::json!({ "ids": ["a", "b"] });
        assert_eq!(selection_ids(Some(&args)), vec!["a".to_string(), "b".to_string()]);
        assert!(selection_ids(None).is_empty());
    }

    #[test]
    fn panel_tree_builder_produces_a_namespaced_tree_with_placeholder() {
        let builder = PanelTreeBuilder::new("ns-play-document");
        let item_id = builder.item_id("widget", "w1");
        assert_eq!(item_id, "ns-play-document.widget.w1");
        let node = builder
            .section("ns-play-document.widgets", Some("Widgets".into()), true, vec![tree_item(item_id, "W1")])
            .section_or_placeholder("ns-play-document.synapses", Some("Synapses".into()), false, vec![], "(none)")
            .selected(vec!["ns-play-document.widget.w1".into()])
            .build();
        let UiNode::Tree(tree) = node else { panic!("expected a Tree node") };
        assert_eq!(tree.sections.len(), 2);
        assert_eq!(tree.sections[0].items.len(), 1);
        assert_eq!(tree.sections[1].items[0].label, "(none)");
        assert_eq!(tree.selected_ids, Some(vec!["ns-play-document.widget.w1".to_string()]));
    }
}
//#endregion 🔖PanelKit

//#region 🔖FormKit
// 📋 Shared form-panel builder — lifts the `Section > labeled Field rows > submit Button` skeleton
// (and the sibling `entity_detail` read-only `KeyValue` summary block) duplicated across plugin crates
// that render declarative forms/detail panels, mirroring `PanelTreeBuilder`'s namespaced builder-pattern
// shape above (`namespace` prefixes every id, method chaining ends in `.build() -> UiNode`).

/// 📋 Fluent builder for a `Section` of labeled `Field` rows ending in an optional submit `Button` —
/// same namespaced-id / method-chaining shape as `PanelTreeBuilder`.
pub struct FormPanelBuilder {
    namespace: String,
    fields: Vec<UiNode>,
    submit: Option<UiButtonNode>,
}

impl FormPanelBuilder {
    /// 📋 `namespace` prefixes every field id built via `.field_id()`/`.field()`/`.from_dictionary()`.
    pub fn new(namespace: impl Into<String>) -> Self {
        Self { namespace: namespace.into(), fields: Vec::new(), submit: None }
    }

    /// 📋 Builds a namespaced field id: `"{namespace}.field.{id}"` — mirrors `PanelTreeBuilder::item_id`.
    pub fn field_id(&self, id: &str) -> String {
        format!("{}.field.{id}", self.namespace)
    }

    /// 📋 Adds one labeled field row: `control` wraps into a `UiFieldNode` via `ui_control_to_node`.
    pub fn field(mut self, id: &str, label: &str, description: Option<String>, control: UiControlNode) -> Self {
        let field_id = self.field_id(id);
        self.fields.push(UiNode::Field(UiFieldNode {
            id: field_id,
            label: label.into(),
            description,
            required: None,
            error: None,
            child: Box::new(ui_control_to_node(control)),
        }));
        self
    }

    /// 📋 Routes the OS `form.dictionary` resource shape (see the `ResourceKindSpec { id:
    /// "form.dictionary", source_format: "forms.dictionary", .. }` registered by `forms/plugin`'s
    /// `create_forms_app`) into a sequence of text-input field rows: each top-level entry in the
    /// `dictionary_json` array — `{ "id", "label"?, "description"?, "value"? }` — becomes one field
    /// dispatching the shared `on_change` action (its `args` are left to the caller; the emitted input's
    /// own id already carries which field changed).
    pub fn from_dictionary(mut self, dictionary_json: &Value, on_change: ActionDescriptor) -> Self {
        let Some(entries) = dictionary_json.as_array() else { return self };
        for entry in entries {
            let Some(id) = entry.get("id").and_then(Value::as_str) else { continue };
            let label = entry.get("label").and_then(Value::as_str).unwrap_or(id).to_string();
            let description = entry.get("description").and_then(Value::as_str).map(str::to_string);
            let value = entry.get("value").and_then(Value::as_str).unwrap_or_default().to_string();
            let field_id = self.field_id(id);
            let control = UiControlNode::Input(UiInputNode {
                id: field_id,
                input_kind: "text".into(),
                value,
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: on_change.clone(),
            });
            self = self.field(id, &label, description, control);
        }
        self
    }

    /// 📋 Sets the trailing submit `Button` row.
    pub fn submit(mut self, label: &str, action: ActionDescriptor) -> Self {
        self.submit = Some(UiButtonNode {
            id: Some(self.field_id("submit")),
            icon_id: String::new(),
            label: label.into(),
            action,
            style: None,
            disabled: None,
            loading: None,
        });
        self
    }

    /// 📋 Builds `Section > Fields > Button` — the section id is the builder's namespace.
    pub fn build(self) -> UiNode {
        let mut children = self.fields;
        if let Some(submit) = self.submit {
            children.push(UiNode::Button(submit));
        }
        UiNode::Section(UiSectionNode { id: self.namespace, label: None, default_open: Some(true), loading: None, children })
    }
}

/// 📋 A read-only entity-detail panel: `title`/`subtitle` header text, a `KeyValue` summary block built
/// from `entries` (reusing `ui_wgpu`'s existing `UiKeyValueEntry` rather than a duplicate local type),
/// and trailing action buttons.
pub fn entity_detail(title: &str, subtitle: Option<&str>, entries: Vec<UiKeyValueEntry>, actions: Vec<UiButtonNode>) -> UiNode {
    let mut children = vec![ui_text(title)];
    if let Some(subtitle) = subtitle {
        children.push(ui_text(subtitle));
    }
    children.push(UiNode::KeyValue(UiKeyValueNode { entries }));
    children.extend(actions.into_iter().map(UiNode::Button));
    ui_stack_vertical(children)
}

#[cfg(test)]
mod form_kit_tests {
    use super::*;

    #[test]
    fn form_panel_builder_wraps_a_field_control_and_submit_button() {
        let on_change = ActionDescriptor { controller_id: "app".into(), action: "setValue".into(), args: None };
        let submit_action = ActionDescriptor { controller_id: "app".into(), action: "submit".into(), args: None };
        let node = FormPanelBuilder::new("ns-play-form")
            .field(
                "name",
                "Name",
                Some("Full name".into()),
                UiControlNode::Input(UiInputNode {
                    id: "ns-play-form.field.name".into(),
                    input_kind: "text".into(),
                    value: String::new(),
                    placeholder: None,
                    commit: None,
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    on_change,
                }),
            )
            .submit("Submit", submit_action)
            .build();
        let UiNode::Section(section) = node else { panic!("expected a Section node") };
        assert_eq!(section.id, "ns-play-form");
        assert_eq!(section.children.len(), 2);
        let UiNode::Field(field) = &section.children[0] else { panic!("expected a Field node") };
        assert_eq!(field.id, "ns-play-form.field.name");
        assert_eq!(field.description.as_deref(), Some("Full name"));
        let UiNode::Button(button) = &section.children[1] else { panic!("expected a Button node") };
        assert_eq!(button.label, "Submit");
    }

    #[test]
    fn form_panel_builder_from_dictionary_routes_entries_into_field_rows() {
        let on_change = ActionDescriptor { controller_id: "app".into(), action: "setValue".into(), args: None };
        let dictionary = serde_json::json!([
            { "id": "email", "label": "Email", "description": "Contact email", "value": "a@b.com" },
            { "id": "phone" },
        ]);
        let node = FormPanelBuilder::new("ns-play-form").from_dictionary(&dictionary, on_change).build();
        let UiNode::Section(section) = node else { panic!("expected a Section node") };
        assert_eq!(section.children.len(), 2);
        let UiNode::Field(email_field) = &section.children[0] else { panic!("expected a Field node") };
        assert_eq!(email_field.id, "ns-play-form.field.email");
        assert_eq!(email_field.label, "Email");
        let UiNode::Field(phone_field) = &section.children[1] else { panic!("expected a Field node") };
        assert_eq!(phone_field.label, "phone");
    }

    #[test]
    fn entity_detail_builds_a_stack_with_header_key_value_and_actions() {
        let action = ActionDescriptor { controller_id: "app".into(), action: "edit".into(), args: None };
        let node = entity_detail(
            "Widget",
            Some("A widget"),
            vec![UiKeyValueEntry { label: "Kind".into(), value: "gizmo".into() }],
            vec![UiButtonNode { id: None, icon_id: "edit".into(), label: "Edit".into(), action, style: None, disabled: None, loading: None }],
        );
        let UiNode::Stack(stack) = node else { panic!("expected a Stack node") };
        assert_eq!(stack.children.len(), 4);
        let UiNode::KeyValue(key_value) = &stack.children[2] else { panic!("expected a KeyValue node") };
        assert_eq!(key_value.entries[0].value, "gizmo");
    }
}
//#endregion 🔖FormKit

//#region 🔖Terminology
// 🗣️ Shared locale-label resolution — replaces the ~25x hand-rolled `struct XLabels { .. }` +
// `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels` pattern duplicated per app,
// plus the per-app `(id, en, de)` action/utility label-map builder functions.

/// 🗣️ A locale label set: a `&'static` accessor per locale. Implement via `app_labels!` rather than by
/// hand — accessors (not associated consts) because Rust's constant-promotion of `&Self::CONST` to
/// `&'static Self` only fires for a concrete type, not through a generic type parameter; each concrete
/// `impl LocaleLabels for XLabels` promotes its own `&Self::EN`/`&Self::DE` internally instead.
pub trait LocaleLabels: Sized + 'static {
    fn locale_labels_en() -> &'static Self;
    fn locale_labels_de() -> &'static Self;
}

/// 🗣️ True when `view_state.locale` names a German variant ("de", "de-DE", …).
pub fn is_de_locale(view_state: &ViewState) -> bool {
    view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"))
}

/// 🗣️ Resolves the active label set for the shell-provided locale; unknown/absent locales fall back
/// to the English set. Replaces the ~25x hand-rolled `fn x_labels(view_state) -> &'static XLabels { if
/// is_de { &X_LABELS_DE } else { &X_LABELS_EN } }`.
pub fn resolve_labels<L: LocaleLabels>(view_state: &ViewState) -> &'static L {
    if is_de_locale(view_state) { L::locale_labels_de() } else { L::locale_labels_en() }
}

/// 🗣️ Declares a locale label struct plus its `EN`/`DE` consts and `LocaleLabels` impl in one compact
/// block — resolve the active set with `resolve_labels::<XLabels>(view_state)`. Replaces the ~25x
/// hand-rolled `struct XLabels { .. }` + two `const` items + resolver fn.
///
/// ```ignore
/// semio_framework_plugin::app_labels! {
///     struct FlowPlayLabels {
///         widgets: &'static str = en: "Widgets", de: "Widgets";
///         synapses: &'static str = en: "Synapses", de: "Synapsen";
///     }
/// }
/// ```
#[macro_export]
macro_rules! app_labels {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident {
            $( $field:ident: $ty:ty = en: $en_value:expr, de: $de_value:expr );+ $(;)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $Name {
            $( $vis $field: $ty ),+
        }

        impl $Name {
            const EN: Self = Self { $( $field: $en_value ),+ };
            const DE: Self = Self { $( $field: $de_value ),+ };
        }

        impl $crate::app::LocaleLabels for $Name {
            fn locale_labels_en() -> &'static Self {
                &Self::EN
            }
            fn locale_labels_de() -> &'static Self {
                &Self::DE
            }
        }
    };
}

/// 🗣️ Picks `de` or `en` by `is_de` — replaces the ~inline `if is_de { "..." } else { "..." }` pairs
/// duplicated per app for one-off labels that don't warrant a full `app_labels!` struct or a
/// `localized_label_map` entry.
pub fn bilingual(en: &str, de: &str, is_de: bool) -> String {
    (if is_de { de } else { en }).to_string()
}

/// 🗣️ Builds an (id -> localized label) map from `(id, en, de)` triples — replaces the per-crate
/// hand-rolled action/utility label-map builder functions (e.g. `flow_action_labels`,
/// `protocol_play_action_labels`).
pub fn localized_label_map(is_de: bool, entries: &[(&str, &str, &str)]) -> HashMap<String, String> {
    entries.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}

/// 🗣️ Fluent builder extension for `AppLabelsOverlay` — an *extension trait*, not inherent methods:
/// `AppLabelsOverlay` is defined in `semio-framework-core`, so Rust's orphan rules permit a local trait
/// impl on it but not inherent methods from this downstream crate. Replaces the large hand-constructed
/// `AppLabelsOverlay { .. }` struct literals every app's `DocumentApp::app_labels` currently writes.
pub trait AppLabelsOverlayExt: Sized {
    fn window_kind_label(self, id: impl Into<String>, label: impl Into<String>) -> Self;
    fn panel_tab_label(self, id: impl Into<String>, label: impl Into<String>) -> Self;
    fn mode_label(self, id: impl Into<String>, label: impl Into<String>) -> Self;
    fn action_labels(self, labels: HashMap<String, String>) -> Self;
    fn utility_labels(self, labels: HashMap<String, String>) -> Self;
    fn example_labels(self, labels: HashMap<String, String>) -> Self;
    fn action_arg_label(self, key: impl Into<String>, label: impl Into<String>) -> Self;
    fn dialog_labels(self, labels: HashMap<String, String>) -> Self;
    fn introduction_labels(self, labels: HashMap<String, String>) -> Self;
}

impl AppLabelsOverlayExt for AppLabelsOverlay {
    fn window_kind_label(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.window_kind_labels.insert(id.into(), label.into());
        self
    }
    fn panel_tab_label(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.panel_tab_labels.insert(id.into(), label.into());
        self
    }
    fn mode_label(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.mode_labels.insert(id.into(), label.into());
        self
    }
    fn action_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.action_labels = labels;
        self
    }
    fn utility_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.utility_labels = labels;
        self
    }
    fn example_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.example_labels = labels;
        self
    }
    fn action_arg_label(mut self, key: impl Into<String>, label: impl Into<String>) -> Self {
        self.action_arg_labels.insert(key.into(), label.into());
        self
    }
    fn dialog_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.dialog_labels = labels;
        self
    }
    fn introduction_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.introduction_labels = labels;
        self
    }
}

#[cfg(test)]
mod terminology_tests {
    use super::*;

    app_labels! {
        struct SampleLabels {
            greeting: &'static str = en: "Hello", de: "Hallo";
        }
    }

    #[test]
    fn resolve_labels_picks_en_or_de_by_locale() {
        let en = ViewState { locale: Some("en-US".into()), ..ViewState::default() };
        let de = ViewState { locale: Some("de-DE".into()), ..ViewState::default() };
        let none = ViewState::default();
        assert_eq!(resolve_labels::<SampleLabels>(&en).greeting, "Hello");
        assert_eq!(resolve_labels::<SampleLabels>(&de).greeting, "Hallo");
        assert_eq!(resolve_labels::<SampleLabels>(&none).greeting, "Hello");
    }

    #[test]
    fn bilingual_picks_en_or_de_by_flag() {
        assert_eq!(bilingual("Hello", "Hallo", false), "Hello");
        assert_eq!(bilingual("Hello", "Hallo", true), "Hallo");
    }

    #[test]
    fn localized_label_map_selects_by_locale() {
        let entries: &[(&str, &str, &str)] = &[("addStep", "Add Step", "Schritt hinzufuegen")];
        let en = localized_label_map(false, entries);
        let de = localized_label_map(true, entries);
        assert_eq!(en.get("addStep").map(String::as_str), Some("Add Step"));
        assert_eq!(de.get("addStep").map(String::as_str), Some("Schritt hinzufuegen"));
    }

    #[test]
    fn app_labels_overlay_ext_builds_fluently() {
        let overlay = AppLabelsOverlay::default()
            .window_kind_label("main", "Main")
            .mode_label("edit", "Edit")
            .action_labels(localized_label_map(false, &[("addStep", "Add Step", "Schritt hinzufuegen")]));
        assert_eq!(overlay.window_kind_labels.get("main").map(String::as_str), Some("Main"));
        assert_eq!(overlay.mode_labels.get("edit").map(String::as_str), Some("Edit"));
        assert_eq!(overlay.action_labels.get("addStep").map(String::as_str), Some("Add Step"));
    }
}
//#endregion 🔖Terminology

//#region 🔖Testkit
pub mod testkit {
//! 🧪 Generic test-harness helpers for `DocumentApp` implementors. Factors out the ~24x duplicated
//! `meta()`/`new_app()`/`new_app_with_registry()`/`paired_apps()` boilerplate plus the repeated
//! undo-redo / two-instance-convergence / ingest-idempotency test *bodies* (parameterized by closures
//! for the app-specific action names/projection shape, so only the control flow is shared). Not
//! `#[cfg(test)]` — apps' own `#[cfg(test)]` modules call these as a regular dependency; see
//! `terminology_tests`/`panel_kit_tests` above for the sibling pattern of testing SDK primitives
//! themselves inline.

use super::{ActionMeta, App, AppActionRegistry, DocumentApp, PluginApp, VcsDocumentApp};
use semio_framework_core::ViewState;
use vcs::{Backbone, BackboneMessage, MemoryBackbone, StudioConflict};

/// 🪪 A local-actor `ActionMeta` for test dispatch (`instance_id: 1`).
pub fn meta(actor: &str) -> ActionMeta {
    ActionMeta { actor: actor.into(), instance_id: 1 }
}

/// 🧬 A registry-less wrapper (`VcsDocumentApp::new`) — contract enforcement (required args, kind
/// discipline) is skipped, matching most apps' plain unit tests.
pub fn new_app<A: DocumentApp + Default>() -> VcsDocumentApp<A> {
    VcsDocumentApp::new(A::default())
}

/// 🧬 A registry-backed wrapper carrying `manifest`'s real `AppActionRegistry` — needed whenever a
/// test must exercise declared-arg defaults/required-arg enforcement or View/Shell kind discipline.
pub fn new_app_with_registry<A: DocumentApp + Default>(manifest: fn() -> App) -> VcsDocumentApp<A> {
    let definition = manifest().definition;
    VcsDocumentApp::with_registry(A::default(), AppActionRegistry::from_definition(&definition))
}

/// 🔗 Two registry-less instances joined by an in-memory backbone on `channel` — the standard fixture
/// for convergence tests.
pub fn paired_apps<A: DocumentApp + Default>(channel: &str) -> (VcsDocumentApp<A>, VcsDocumentApp<A>) {
    let mut a = new_app::<A>();
    let mut b = new_app::<A>();
    let (backbone_a, backbone_b) = MemoryBackbone::pair(channel, channel);
    a.attach_backbone(Box::new(backbone_a)).expect("attach a");
    b.attach_backbone(Box::new(backbone_b)).expect("attach b");
    (a, b)
}

/// 🧪 Runs `action` once, asserts `probe(app)` matches `after`, undoes and asserts `before`, redoes and
/// asserts `after` again — the repeated undo/redo round-trip test body.
pub fn assert_undo_redo_round_trip<A, P>(
    app: &mut VcsDocumentApp<A>,
    action: &str,
    args: Option<&serde_json::Value>,
    probe: impl Fn(&VcsDocumentApp<A>) -> P,
    before: P,
    after: P,
) where
    A: DocumentApp,
    P: PartialEq + std::fmt::Debug,
{
    app.handle_action(action, args, &ViewState::default(), &meta("local")).expect("apply action");
    assert_eq!(probe(app), after, "action did not produce the expected projection");
    app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
    assert_eq!(probe(app), before, "undo did not revert to the expected projection");
    app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
    assert_eq!(probe(app), after, "redo did not reapply the expected projection");
}

/// 🧪 `action_a`/`action_b` are applied to two `paired_apps` instances, a neutral history action
/// (`commitCheckpoint`) pumps each side's inbound ops, then `probe` must agree on both — the repeated
/// two-instance-convergence test body (see `protocol-plugin`'s
/// `two_instances_converge_disjoint_edits_via_backbone` for the original, app-specific version).
pub fn assert_two_instances_converge<A, P>(
    channel: &str,
    action_a: (&str, Option<&serde_json::Value>),
    action_b: (&str, Option<&serde_json::Value>),
    probe: impl Fn(&VcsDocumentApp<A>) -> P,
) where
    A: DocumentApp + Default,
    P: PartialEq + std::fmt::Debug,
{
    let (mut instance_a, mut instance_b) = paired_apps::<A>(channel);
    instance_a.handle_action(action_a.0, action_a.1, &ViewState::default(), &meta("actor-a")).expect("a applies its edit");
    instance_b.handle_action(action_b.0, action_b.1, &ViewState::default(), &meta("actor-b")).expect("b applies its edit");
    instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");
    assert_eq!(probe(&instance_a), probe(&instance_b), "both instances must converge on the same projection");
}

/// 🧪 The `Operation::reconcile` counterpart to `assert_two_instances_converge`: `action_delete`/
/// `action_wire` race on two `paired_apps` instances (typically one deletes a graph node, the other
/// concurrently wires an edge to it), a `commitCheckpoint` pumps each side's inbound ops, then both
/// sides' post-reconcile `probe` results (`(projection, conflicts)`) must agree, `has_dangling_ref`
/// must be false for the converged projection, and at least one `StudioConflict` must have been
/// reported (dropping a dangling reference silently, with no conflict, would hide real data loss).
pub fn assert_graph_merge_preserves_referential_integrity<A, P>(
    channel: &str,
    action_delete: (&str, Option<&serde_json::Value>),
    action_wire: (&str, Option<&serde_json::Value>),
    probe: impl Fn(&VcsDocumentApp<A>) -> (P, Vec<StudioConflict>),
    has_dangling_ref: impl Fn(&P) -> bool,
) where
    A: DocumentApp + Default,
    P: PartialEq + std::fmt::Debug,
{
    let (mut instance_a, mut instance_b) = paired_apps::<A>(channel);
    instance_a.handle_action(action_delete.0, action_delete.1, &ViewState::default(), &meta("actor-a")).expect("a deletes the node");
    instance_b.handle_action(action_wire.0, action_wire.1, &ViewState::default(), &meta("actor-b")).expect("b wires the edge");
    instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");
    let (projection_a, conflicts_a) = probe(&instance_a);
    let (projection_b, conflicts_b) = probe(&instance_b);
    assert_eq!(projection_a, projection_b, "both instances must converge on the same reconciled projection");
    assert!(!has_dangling_ref(&projection_a), "converged projection must not retain a dangling reference");
    assert!(!conflicts_a.is_empty(), "dropping the dangling reference must surface a StudioConflict");
    assert_eq!(conflicts_a, conflicts_b, "both instances must report the same reconciliation conflicts");
}

/// 🧪 Applies `action` on a sender attached to a backbone, replays the resulting envelopes onto a
/// fresh receiver twice, and asserts `probe` sees the same result both times — feeding the same op
/// twice must not double-apply.
pub fn assert_ingest_idempotent<A, P>(
    action: &str,
    args: Option<&serde_json::Value>,
    probe: impl Fn(&VcsDocumentApp<A>) -> P,
) where
    A: DocumentApp + Default,
    P: PartialEq + std::fmt::Debug,
{
    let mut sender = new_app::<A>();
    let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent");
    sender.attach_backbone(Box::new(near)).expect("attach sender");
    sender.handle_action(action, args, &ViewState::default(), &meta("local")).expect("apply action");

    let mut envelopes = Vec::new();
    for message in far.receive().expect("receive") {
        if let BackboneMessage::Ops { envelopes: ops } = message {
            envelopes.extend(ops);
        }
    }
    let operations_json = serde_json::to_string(&envelopes).expect("serialize envelopes");

    let mut receiver = new_app::<A>();
    receiver.ingest_operations(&operations_json).expect("ingest once");
    let once = probe(&receiver);
    receiver.ingest_operations(&operations_json).expect("ingest twice");
    assert_eq!(probe(&receiver), once, "feeding the same op twice must not double-apply");
}

#[cfg(test)]
mod testkit_tests {
    //! 🧪 Proves each `testkit` primitive against a minimal dummy `DocumentApp` before any real app
    //! adopts them.
    use super::*;
    use crate::app::{ActionEmit, DocumentView};
    use crate::{ui_text, UiNode};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use vcs::{Operation, OperationDiff};

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DummyProjection {
        count: i32,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DummyDiff {
        count: Option<i32>,
    }

    impl OperationDiff<DummyProjection> for DummyDiff {
        fn apply(&self, projection: &DummyProjection) -> DummyProjection {
            DummyProjection { count: self.count.unwrap_or(projection.count) }
        }

        fn absorb(&mut self, other: Self) {
            if other.count.is_some() {
                self.count = other.count;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op", rename_all = "camelCase")]
    enum DummyOp {
        SetCount { value: i32 },
    }

    impl Operation<DummyProjection> for DummyOp {
        type Diff = DummyDiff;

        fn diff(&self, _projection: &DummyProjection) -> DummyDiff {
            match self {
                DummyOp::SetCount { value } => DummyDiff { count: Some(*value) },
            }
        }

        fn backwards(&self, projection: &DummyProjection) -> Vec<Self> {
            vec![DummyOp::SetCount { value: projection.count }]
        }
    }

    #[derive(Default)]
    struct DummyApp;

    impl DocumentApp for DummyApp {
        type Projection = DummyProjection;
        type Op = DummyOp;

        fn app_id(&self) -> &str {
            "testkit-dummy"
        }

        fn document_schema(&self) -> &str {
            "semio.testkit/v1"
        }

        fn initial_projection(&self) -> DummyProjection {
            DummyProjection::default()
        }

        fn handle_action(
            &mut self,
            action: &str,
            _args: Option<&Value>,
            doc: &DocumentView<'_, DummyProjection>,
            _view_state: &ViewState,
        ) -> ActionEmit<DummyOp> {
            match action {
                "increment" => ActionEmit {
                    ops: vec![DummyOp::SetCount { value: doc.projection.count + 1 }],
                    description: Some("increment".into()),
                    ..Default::default()
                },
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, _body_key: &str, doc: &DocumentView<'_, DummyProjection>, _view_state: &ViewState) -> UiNode {
            ui_text(format!("count={}", doc.projection.count))
        }
    }

    #[test]
    fn meta_carries_actor_and_local_instance_id() {
        let m = meta("actor-x");
        assert_eq!(m.actor, "actor-x");
        assert_eq!(m.instance_id, 1);
    }

    #[test]
    fn new_app_constructs_a_registry_less_wrapper() {
        let mut app = new_app::<DummyApp>();
        app.handle_action("increment", None, &ViewState::default(), &meta("local")).expect("increment");
        assert_eq!(app.projection().unwrap().count, 1);
    }

    #[test]
    fn assert_undo_redo_round_trip_passes_for_a_real_operation() {
        let mut app = new_app::<DummyApp>();
        assert_undo_redo_round_trip(&mut app, "increment", None, |app| app.projection().unwrap().count, 0, 1);
    }

    #[test]
    fn assert_two_instances_converge_on_disjoint_edits() {
        assert_two_instances_converge::<DummyApp, i32>(
            "mem://testkit-converge",
            ("increment", None),
            ("increment", None),
            |app| app.projection().unwrap().count,
        );
    }

    #[test]
    fn assert_ingest_idempotent_does_not_double_apply() {
        assert_ingest_idempotent::<DummyApp, i32>("increment", None, |app| app.projection().unwrap().count);
    }
}
}
//#endregion 🔖Testkit

#[cfg(test)]
mod app_builder_tests {
    use super::*;
    use ui_wgpu::create_default_layout;

    #[test]
    fn build_definition_rejects_layout_with_unknown_window_kind() {
        let result = std::panic::catch_unwind(|| {
            App::builder("bad-app", "Bad")
                .document(["semio", "bad"])
                .mode("edit", "Edit")
                .mode_utilities("edit", vec![])
                .window_kind("main", "Main", "bad.main", SurfaceKind::Canvas2d)
                .default_layout(create_default_layout(&["missing".into()], "row", None, None))
                .build_definition();
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_accepts_valid_manifest() {
        let definition = App::builder("good-app", "Good")
            .document(["semio", "good"])
            .mode("edit", "Edit")
            .mode_utilities("edit", vec![])
            .window_kind("main", "Main", "good.main", SurfaceKind::Canvas2d)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, "good.document")
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .build_definition();
        assert_eq!(definition.window_kinds.len(), 1);
        assert_eq!(definition.panel_tabs.len(), 1);
    }

    #[test]
    fn build_definition_rejects_terminology_document_for_undeclared_terminology() {
        let result = std::panic::catch_unwind(|| {
            App::builder("bad-terminology-app", "Bad")
                .document(["semio", "bad"])
                .mode("edit", "Edit")
                .mode_utilities("edit", vec![])
                .window_kind("main", "Main", "bad.main", SurfaceKind::Canvas2d)
                .default_layout(create_default_layout(&["main".into()], "row", None, None))
                .terminology_document("reuse", ["Entwerfen mit Bestand", "Bad"])
                .build_definition();
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_accepts_declared_terminology_document() {
        let definition = App::builder("good-terminology-app", "Good")
            .document(["semio", "good"])
            .mode("edit", "Edit")
            .mode_utilities("edit", vec![])
            .window_kind("main", "Main", "good.main", SurfaceKind::Canvas2d)
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])
            .build_definition();
        assert_eq!(
            definition.terminology_documents.get("reuse").map(Vec::as_slice),
            Some(["Entwerfen mit Bestand".to_string(), "Aggregator".to_string()].as_slice())
        );
    }

    fn minimal_app(id: &str) -> AppBuilder {
        App::builder(id, "App")
            .document(["semio", id])
            .mode("edit", "Edit")
            .window_kind("main", "Main", format!("{id}.main"), SurfaceKind::Canvas2d)
    }

    #[test]
    fn build_definition_auto_injects_history_actions_and_keybindings() {
        let definition = minimal_app("history-app").build_definition();
        let history_ids: HashSet<&str> = definition.actions.iter().map(|c| c.id.as_str()).collect();
        assert!(history_ids.contains("undo"));
        assert!(history_ids.contains("redo"));
        assert!(history_ids.contains("commitCheckpoint"));
        assert!(history_ids.contains("createAlternative"));
        assert!(history_ids.contains("switchAlternative"));
        assert!(history_ids.contains("checkoutCheckpoint"));
        let undo_binding = definition
            .keybindings
            .iter()
            .find(|binding| binding.keys == "mod+z")
            .expect("undo keybinding auto-injected");
        assert_eq!(undo_binding.action.action, "undo");
        assert_eq!(undo_binding.action.controller_id, "history-app");
    }

    #[test]
    fn build_definition_does_not_duplicate_manually_declared_history_keybinding() {
        let definition = minimal_app("manual-undo-app")
            .keybinding("mod+z", "undo")
            .build_definition();
        assert_eq!(definition.keybindings.iter().filter(|b| b.keys == "mod+z").count(), 1);
    }

    #[test]
    fn operation_view_and_shell_actions_are_declared_with_their_kind() {
        let definition = minimal_app("typed-actions-app")
            .operation("addLayer", "Add Layer")
            .view_action("setCamera", "Set Camera")
            .shell_action("exportPng", "Export PNG")
            .build_definition();
        let by_id = |id: &str| definition.actions.iter().find(|c| c.id == id).expect("declared");
        assert_eq!(by_id("addLayer").kind, ActionKind::Operation);
        assert_eq!(by_id("setCamera").kind, ActionKind::View);
        assert_eq!(by_id("exportPng").kind, ActionKind::Shell);
    }

    #[test]
    fn build_definition_rejects_duplicate_action_ids() {
        let result = std::panic::catch_unwind(|| {
            minimal_app("dupe-action-app")
                .operation("addLayer", "Add Layer")
                .operation("addLayer", "Add Layer Again")
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_keybinding_for_undeclared_action_once_opted_in() {
        let result = std::panic::catch_unwind(|| {
            minimal_app("undeclared-keybinding-app")
                .operation("addLayer", "Add Layer")
                .keybinding("mod+l", "removeLayer")
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn declaring_utilities_injects_set_active_utility_action_and_keybinding() {
        use semio_framework_core::{ActionKind, UtilityDefinition, SET_ACTIVE_UTILITY_ACTION_ID};
        let definition = minimal_app("utility-app")
            .utility(UtilityDefinition { keys: Some("b".into()), ..UtilityDefinition::new("brush", "Brush", "icon.brush") })
            .utility_simple("eraser", "Eraser", "icon.eraser")
            .build_definition();
        let set_active_utility = definition
            .actions
            .iter()
            .find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID)
            .expect("setActiveUtility injected");
        assert_eq!(set_active_utility.kind, ActionKind::View);
        assert!(!set_active_utility.in_palette);
        let binding = definition
            .keybindings
            .iter()
            .find(|binding| binding.keys == "b")
            .expect("utility keybinding auto-injected");
        assert_eq!(binding.action.action, SET_ACTIVE_UTILITY_ACTION_ID);
        assert_eq!(binding.action.args, Some(serde_json::json!({ "utilityId": "brush" })));
    }

    #[test]
    fn no_utilities_means_no_set_active_utility_action() {
        use semio_framework_core::SET_ACTIVE_UTILITY_ACTION_ID;
        let definition = minimal_app("no-utility-app").build_definition();
        assert!(!definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID));
    }

    #[test]
    fn action_args_attaches_declared_arguments() {
        let definition = minimal_app("args-app")
            .operation("resize", "Resize")
            .action_args("resize", vec![ActionArgDef::slider("scale", "Scale", 0.0, 4.0).required()])
            .build_definition();
        let resize = definition.actions.iter().find(|action| action.id == "resize").expect("declared");
        assert_eq!(resize.args.len(), 1);
        assert_eq!(resize.args[0].id, "scale");
        assert!(resize.args[0].required);
    }

    #[test]
    fn build_definition_rejects_window_kind_utility_referencing_undeclared_utility() {
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-utility-ref-app")
                .utility_simple("brush", "Brush", "icon.brush")
                .window_kind_utilities("main", vec!["missing".into()])
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_window_kind_action_referencing_undeclared_action() {
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-action-ref-app")
                .operation("addLayer", "Add Layer")
                .window_kind_actions("main", vec!["removeLayer".into()])
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_select_arg_with_no_options() {
        use semio_framework_core::{ActionArgControl, ActionArgDef};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-select-app")
                .operation("pick", "Pick")
                .action_args("pick", vec![ActionArgDef {
                    control: ActionArgControl::Select { options: vec![] },
                    ..ActionArgDef::text("choice", "Choice")
                }])
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn declaring_introduction_injects_start_introduction_action() {
        use semio_framework_core::{ActionKind, IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition, START_INTRODUCTION_ACTION_ID};
        let definition = minimal_app("intro-app")
            .introduction(IntroductionDefinition {
                title: "Welcome".into(),
                steps: vec![IntroductionStepDefinition::new("welcome", "Welcome", "Hi there", IntroductionAnchor::Screen)],
            })
            .build_definition();
        let start_introduction = definition
            .actions
            .iter()
            .find(|action| action.id == START_INTRODUCTION_ACTION_ID)
            .expect("startIntroduction injected");
        assert_eq!(start_introduction.kind, ActionKind::View);
        assert!(!start_introduction.in_palette, "the shell-owned Introduce App command owns palette discovery");
    }

    #[test]
    fn no_introduction_means_no_start_introduction_action() {
        use semio_framework_core::START_INTRODUCTION_ACTION_ID;
        let definition = minimal_app("no-intro-app").build_definition();
        assert!(!definition.actions.iter().any(|action| action.id == START_INTRODUCTION_ACTION_ID));
    }

    #[test]
    fn build_definition_rejects_introduction_with_no_steps() {
        use semio_framework_core::IntroductionDefinition;
        let result = std::panic::catch_unwind(|| {
            minimal_app("empty-intro-app")
                .introduction(IntroductionDefinition { title: "Welcome".into(), steps: vec![] })
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_duplicate_introduction_step_ids() {
        use semio_framework_core::{IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("dupe-step-app")
                .introduction(IntroductionDefinition {
                    title: "Welcome".into(),
                    steps: vec![
                        IntroductionStepDefinition::new("step", "A", "a", IntroductionAnchor::Screen),
                        IntroductionStepDefinition::new("step", "B", "b", IntroductionAnchor::Screen),
                    ],
                })
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_introduction_step_anchoring_undeclared_window_kind() {
        use semio_framework_core::{IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-anchor-window-app")
                .introduction(IntroductionDefinition {
                    title: "Welcome".into(),
                    steps: vec![IntroductionStepDefinition::new(
                        "step",
                        "A",
                        "a",
                        IntroductionAnchor::WindowKind("missing".into()),
                    )],
                })
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_introduction_step_anchoring_undeclared_utility() {
        use semio_framework_core::{IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-anchor-utility-app")
                .introduction(IntroductionDefinition {
                    title: "Welcome".into(),
                    steps: vec![IntroductionStepDefinition::new(
                        "step",
                        "A",
                        "a",
                        IntroductionAnchor::Utility("missing".into()),
                    )],
                })
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_introduction_step_anchoring_undeclared_action() {
        use semio_framework_core::{IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-anchor-action-app")
                .introduction(IntroductionDefinition {
                    title: "Welcome".into(),
                    steps: vec![IntroductionStepDefinition::new(
                        "step",
                        "A",
                        "a",
                        IntroductionAnchor::Action("missing".into()),
                    )],
                })
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_introduction_step_anchoring_undeclared_panel_tab() {
        use semio_framework_core::{IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-anchor-panel-tab-app")
                .introduction(IntroductionDefinition {
                    title: "Welcome".into(),
                    steps: vec![IntroductionStepDefinition::new(
                        "step",
                        "A",
                        "a",
                        IntroductionAnchor::PanelTab("missing".into()),
                    )],
                })
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_introduction_step_advancing_on_undeclared_utility() {
        use semio_framework_core::{IntroductionAdvance, IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-advance-utility-app")
                .introduction(IntroductionDefinition {
                    title: "Welcome".into(),
                    steps: vec![IntroductionStepDefinition::new("step", "A", "a", IntroductionAnchor::Screen)
                        .advance_on(IntroductionAdvance::Utility("missing".into()))],
                })
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_accepts_introduction_anchored_at_declared_window_utility_and_action() {
        use semio_framework_core::{IntroductionAdvance, IntroductionAnchor, IntroductionDefinition, IntroductionStepDefinition};
        let definition = minimal_app("good-intro-app")
            .operation("addLayer", "Add Layer")
            .utility_simple("brush", "Brush", "icon.brush")
            .window_kind_utilities("main", vec!["brush".into()])
            .window_kind_actions("main", vec!["addLayer".into()])
            .introduction(IntroductionDefinition {
                title: "Welcome".into(),
                steps: vec![
                    IntroductionStepDefinition::new("welcome", "Welcome", "Hi", IntroductionAnchor::Screen),
                    IntroductionStepDefinition::new("main-window", "Main Window", "…", IntroductionAnchor::WindowKind("main".into())),
                    IntroductionStepDefinition::new("brush-utility", "Brush", "…", IntroductionAnchor::Utility("brush".into()))
                        .advance_on(IntroductionAdvance::Utility("brush".into())),
                    IntroductionStepDefinition::new("add-layer", "Add Layer", "…", IntroductionAnchor::Action("addLayer".into()))
                        .advance_on(IntroductionAdvance::Action("addLayer".into())),
                ],
            })
            .build_definition();
        let introduction = definition.introduction.expect("introduction present");
        assert_eq!(introduction.steps.len(), 4);
    }

    #[test]
    fn declaring_dialog_appends_to_definition() {
        use semio_framework_core::{ActionRef, DialogDefinition};
        let definition = minimal_app("dialog-app")
            .operation("addLayer", "Add Layer")
            .dialog(DialogDefinition::new("addLayer", "Add Layer", ActionRef::new("addLayer")))
            .build_definition();
        assert_eq!(definition.dialogs.len(), 1);
        assert_eq!(definition.dialogs[0].id, "addLayer");
        assert_eq!(definition.dialogs[0].submit_label, "OK");
    }

    #[test]
    fn build_definition_rejects_duplicate_dialog_ids() {
        use semio_framework_core::{ActionRef, DialogDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("dupe-dialog-app")
                .operation("addLayer", "Add Layer")
                .dialog(DialogDefinition::new("addLayer", "Add Layer", ActionRef::new("addLayer")))
                .dialog(DialogDefinition::new("addLayer", "Add Layer Again", ActionRef::new("addLayer")))
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_dialog_submit_action_referencing_undeclared_action() {
        use semio_framework_core::{ActionRef, DialogDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-dialog-submit-app")
                .dialog(DialogDefinition::new("addLayer", "Add Layer", ActionRef::new("missing")))
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_dialog_cancel_action_referencing_undeclared_action() {
        use semio_framework_core::{ActionRef, DialogDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-dialog-cancel-app")
                .operation("addLayer", "Add Layer")
                .dialog(DialogDefinition::new("addLayer", "Add Layer", ActionRef::new("addLayer")).on_cancel(ActionRef::new("missing")))
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn dialog_submit_action_may_reference_an_injected_history_action() {
        use semio_framework_core::{ActionRef, DialogDefinition};
        let definition = minimal_app("dialog-injected-action-app")
            .dialog(DialogDefinition::new("confirmUndo", "Undo?", ActionRef::new("undo")))
            .build_definition();
        assert_eq!(definition.dialogs[0].submit_action, ActionRef::new("undo"));
    }

    #[test]
    fn build_definition_rejects_dialog_duplicate_arg_ids() {
        use semio_framework_core::{ActionArgDef, ActionRef, DialogDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("dupe-dialog-arg-app")
                .operation("addLayer", "Add Layer")
                .dialog(
                    DialogDefinition::new("addLayer", "Add Layer", ActionRef::new("addLayer"))
                        .args(vec![ActionArgDef::text("name", "Name"), ActionArgDef::text("name", "Name Again")]),
                )
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_dialog_select_arg_with_no_options() {
        use semio_framework_core::{ActionArgControl, ActionArgDef, ActionRef, DialogDefinition};
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-dialog-select-app")
                .operation("addLayer", "Add Layer")
                .dialog(
                    DialogDefinition::new("addLayer", "Add Layer", ActionRef::new("addLayer"))
                        .args(vec![ActionArgDef { control: ActionArgControl::Select { options: vec![] }, ..ActionArgDef::text("kind", "Kind") }]),
                )
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_accepts_app_and_mode_scope_commands() {
        use semio_framework_core::{CommandDefinition, CommandRef, CommandScope};
        let definition = minimal_app("command-app")
            .app_command("app.export", "Export", "document")
            .command(CommandDefinition::new("mode.focus", "Focus", CommandScope::Mode, "view"))
            .mode_commands("edit", vec![CommandRef::new("mode.focus")])
            .build_definition();
        assert_eq!(definition.commands.len(), 2);
        assert_eq!(definition.modes[0].commands, vec![CommandRef::new("mode.focus")]);
    }

    #[test]
    fn build_definition_rejects_duplicate_command_ids() {
        let result = std::panic::catch_unwind(|| {
            minimal_app("dupe-command-app")
                .app_command("app.export", "Export", "document")
                .app_command("app.export", "Export Again", "document")
                .build_definition()
        });
        assert!(result.is_err());
    }

    #[test]
    fn build_definition_rejects_os_or_plugin_scope_command() {
        use semio_framework_core::{CommandDefinition, CommandScope};
        let result = std::panic::catch_unwind(|| {
            minimal_app("os-scope-command-app")
                .command(CommandDefinition::new("os.theme", "Theme", CommandScope::Os, "appearance"))
                .build_definition()
        });
        assert!(result.is_err(), "AppBuilder must reject Os/Plugin-scope commands — those are declared by the shell/PluginBundle, not an app");
    }

    #[test]
    fn build_definition_rejects_mode_command_ref_to_undeclared_or_wrong_scope_command() {
        use semio_framework_core::CommandRef;
        let undeclared = std::panic::catch_unwind(|| {
            minimal_app("undeclared-mode-command-app")
                .mode_commands("edit", vec![CommandRef::new("nope")])
                .build_definition()
        });
        assert!(undeclared.is_err());

        let wrong_scope = std::panic::catch_unwind(|| {
            minimal_app("wrong-scope-mode-command-app")
                .app_command("app.export", "Export", "document")
                .mode_commands("edit", vec![CommandRef::new("app.export")])
                .build_definition()
        });
        assert!(wrong_scope.is_err(), "an App-scope command must not be referenceable from a mode's commands list");
    }

    #[test]
    fn build_definition_rejects_mode_scope_command_referenced_by_no_mode() {
        use semio_framework_core::{CommandDefinition, CommandScope};
        let result = std::panic::catch_unwind(|| {
            minimal_app("orphan-mode-command-app")
                .command(CommandDefinition::new("mode.focus", "Focus", CommandScope::Mode, "view"))
                .build_definition()
        });
        assert!(result.is_err());
    }
}

pub struct App {
    pub definition: AppDefinition,
    pub examples: Vec<ExampleDefinition>,
    pub program: Option<ProgramDefinition>,
}

impl App {
    pub fn builder(id: impl Into<String>, label: impl Into<String>) -> AppBuilder {
        AppBuilder::new(id, label)
    }

    /// 🗂️ Resource kinds declared via `.resource_kind(...)` — see `AppDefinition.resource_kinds`
    /// (round-trips through the plugin manifest; `semio_framework_os`'s resource catalog registry
    /// consumes it from there at plugin registration time).
    pub fn from_builder(builder: AppBuilder) -> Self {
        Self {
            definition: builder.build_definition(),
            examples: Vec::new(),
            program: None,
        }
    }

    pub fn example(mut self, id: impl Into<String>, label: impl Into<String>, document_json: impl Into<String>) -> Self {
        self.examples.push(ExampleDefinition {
            id: id.into(),
            label: label.into(),
            document_json: document_json.into(),
            app_id: String::new(),
        });
        self
    }

    pub fn program(mut self, program_id: impl Into<String>, label: impl Into<String>, yields: impl Into<String>) -> Self {
        self.program = Some(ProgramDefinition {
            program_id: program_id.into(),
            app_id: self.definition.id.clone(),
            label: label.into(),
            document: self.definition.document.clone(),
            yields: yields.into(),
        });
        self
    }
}

//#region 🔖DocumentContract
/// @emoji 🧾 Read-only view of an app's document handed to `DocumentApp::handle_action`/`render`:
/// the materialized projection plus the history metadata (checkpoints/alternatives/undo state)
/// derived from the owning {@link VcsDocumentApp}'s persistent {@link DocumentVcsStore}.
pub struct DocumentView<'a, P> {
    pub projection: &'a P,
    pub history: &'a HistoryView,
}

/// @emoji 📜 Checkpoint/alternative history summary exposed to apps — the swimlane columns plus the
/// undo/redo availability and the current checkout position. Built once per store generation.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryView {
    pub columns: Vec<HistoryColumn>,
    pub can_undo: bool,
    pub can_redo: bool,
    pub active_alternative_id: Option<String>,
    pub current_checkpoint_id: Option<String>,
}

/// @emoji 📤 What a typed `DocumentApp::handle_action` emits: zero-or-more typed operations (applied
/// through the store with a true inverse), an optional description/coalesce key for the resulting
/// edit, host effects (navigate/export/spawn…), and app events. A view action returns an empty
/// `ops` (no history entry); an operation action returns one or more `ops`.
pub struct ActionEmit<Op> {
    pub ops: Vec<Op>,
    pub description: Option<String>,
    pub coalesce_key: Option<String>,
    pub effects: Vec<HostEffect>,
    pub events: Vec<AppEvent>,
    /// 🐢 Which rendered UI sections this action actually invalidates — `Full` (the default) preserves
    /// today's whole-shell-refresh behavior for every app that doesn't opt in to narrower scopes.
    pub ui_scope: semio_framework_core::kernel::UiDirtyScope,
}

impl<Op> Default for ActionEmit<Op> {
    fn default() -> Self {
        Self {
            ops: Vec::new(),
            description: None,
            coalesce_key: None,
            effects: Vec::new(),
            events: Vec::new(),
            ui_scope: semio_framework_core::kernel::UiDirtyScope::default(),
        }
    }
}

impl<Op> ActionEmit<Op> {
    /// @emoji ✏️ An operation emission carrying `ops` and nothing else.
    pub fn ops(ops: Vec<Op>) -> Self {
        Self { ops, ..Default::default() }
    }

    /// @emoji 🔁 Preview pattern (a): a per-tick coalesced emission. The `coalesce_key` folds every
    /// tick of one live gesture (drag/scrub) into a single amendable edit, so the whole gesture is one
    /// undo. Use for cheap per-tick ops (camera/opacity). See the `🔖UtilityPreviewContract` doc region.
    pub fn amend(ops: Vec<Op>, coalesce_key: impl Into<String>) -> Self {
        Self { ops, coalesce_key: Some(coalesce_key.into()), ..Default::default() }
    }

    /// @emoji 📌 Preview pattern (b): the gesture-end commit of an app-runtime scratch draft as one
    /// described edit (`coalesce_key: None`). Use for megabyte-scale content where per-tick amending
    /// would be O(N²) (draw drafts, lowpoly strokes). See the `🔖UtilityPreviewContract` doc region.
    pub fn commit(ops: Vec<Op>, description: impl Into<String>) -> Self {
        Self { ops, description: Some(description.into()), ..Default::default() }
    }

    /// @emoji 🐚 A single host effect and no operations (a shell action).
    pub fn effect(effect: HostEffect) -> Self {
        Self { effects: vec![effect], ..Default::default() }
    }

    /// @emoji 📣 A single app event and no operations.
    pub fn event(event: AppEvent) -> Self {
        Self { events: vec![event], ..Default::default() }
    }
}

/// @emoji 🪪 Per-invocation runtime metadata handed to the object-safe {@link PluginApp} — the local
/// actor id (author of resulting operations, drives `UndoPolicy` foreign-edit classification) and
/// the instance id used to stamp operation/document handles.
#[derive(Clone, Debug, Default)]
pub struct ActionMeta {
    pub actor: String,
    pub instance_id: u32,
}

/// @emoji 🔤 Parses the raw action id crossing the WASM ABI (`DocumentApp::handle_action`'s `action: &str`)
/// into a closed, per-app enum — the seam where "stringly-typed at the edge" becomes exhaustively
/// matched one line in. Not yet wired into `DocumentApp` itself (that would break every existing
/// implementer at once); adopt it per app by matching on the parsed variant instead of the raw string
/// inside `handle_action`, e.g. `let action = MyAppAction::from_action_id(action)?; match action { ... }`.
pub trait AppAction: Sized {
    fn from_action_id(id: &str) -> Result<Self, String>;
}

/// @emoji 🏭 Generates a closed per-app action enum plus its `AppAction` impl from a list of
/// `Variant = "actionId"` pairs — the ids should match what's passed to `.operation()/.view_action()/
/// .shell_action()` on the app's `AppBuilder` so the declared action registry and the dispatch match
/// can't drift apart silently.
#[macro_export]
macro_rules! app_action_enum {
    ($vis:vis enum $Name:ident { $($Variant:ident = $id:literal),* $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis enum $Name {
            $($Variant),*
        }

        impl $crate::app::AppAction for $Name {
            fn from_action_id(id: &str) -> Result<Self, String> {
                match id {
                    $($id => Ok(Self::$Variant),)*
                    other => Err(format!("unknown action id {other}")),
                }
            }
        }
    };
}

/// @emoji 🧩 Typed, per-app author surface. An app declares its `Projection` and `Op` (a
/// `vcs::Operation<Projection>`), mutates nothing directly, and returns an {@link ActionEmit} whose
/// operations flow through a persistent `DocumentVcsStore` owned by {@link VcsDocumentApp}. Ephemeral
/// view state (selection/camera/active utility) lives in the app struct itself, not in the document.
///
/// # 🔖UtilityPreviewContract
/// The formalized actions-vs-utilities contract:
/// - **Actions** are non-interactive: they carry optional declared `ActionArgDef`s, stage in the
///   renderer, and execute once. `Operation`-kind actions emit ops; `View`/`Shell`-kind actions must
///   emit **zero** ops ({@link VcsDocumentApp} enforces this — a View/Shell action returning ops is a
///   hard error).
/// - **Utilities** are interactive live-preview pointer modes. Exactly one utility is active per window kind;
///   the active utility arrives via `view_state.active_utility_id` and is **never** stored in the document
///   nor emitted as an op. Switching utilities dispatches the framework `setActiveUtility` View action; on a
///   switch the app must clear any in-progress preview scratch.
/// - **Two blessed preview patterns** (both funnel through {@link ActionEmit}):
///   1. per-tick coalesced — {@link ActionEmit::amend} folds each tick of a gesture into one amendable
///      edit (one undo per gesture); use for cheap ops (camera/opacity drags).
///   2. scratch + commit — hold a draft in app-runtime state, render it as an overlay, and on gesture
///      end emit {@link ActionEmit::commit} once; use for megabyte-scale content where per-tick
///      amending is O(N²) (draw drafts, lowpoly strokes).
/// - The pointer vocabulary (`canvasPointerDown/Move/Up`, `worldPointerDown/Move/Up`,
///   `paintStrokeBegin/End`) are `View`-kind internal action ids driving the above.
pub trait DocumentApp: Send + 'static {
    type Projection: Clone + PartialEq + Serialize + DeserializeOwned + Send;
    type Op: Operation<Self::Projection> + PartialEq + Send;

    fn app_id(&self) -> &str;
    fn document_schema(&self) -> &str;
    fn initial_projection(&self) -> Self::Projection;
    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, Self::Projection>,
        view_state: &ViewState,
    ) -> ActionEmit<Self::Op>;
    /// 🎛️ Handles a dispatched `CommandDefinition` (os/plugin/app/mode-scoped — never window-level).
    /// Default no-op: apps that declare no `AppDefinition.commands` never need to override this.
    fn handle_command(
        &mut self,
        _command: &str,
        _args: Option<&Value>,
        _doc: &DocumentView<'_, Self::Projection>,
        _view_state: &ViewState,
    ) -> ActionEmit<Self::Op> {
        ActionEmit::default()
    }
    fn render(&self, body_key: &str, doc: &DocumentView<'_, Self::Projection>, view_state: &ViewState) -> UiNode;
    fn window_engagements(
        &self,
        _doc: &DocumentView<'_, Self::Projection>,
        _view_state: &ViewState,
    ) -> HashMap<String, WindowEngagement> {
        HashMap::new()
    }
    fn window_measures(
        &self,
        _doc: &DocumentView<'_, Self::Projection>,
        _view_state: &ViewState,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    /// 🗣️ Locale/terminology-aware overlay for this app's window-kind/mode labels, resolved fresh per `ViewState`
    /// (unlike the static `AppDefinition` labels baked in at manifest-build time). Framework panel-tab labels
    /// (Document/Catalogue/Inspection/Parameters) are merged in automatically by `plugin_runtime::plugin_app_labels`
    /// and do not need to be supplied here.
    fn app_labels(&self, _view_state: &ViewState) -> AppLabelsOverlay {
        AppLabelsOverlay::default()
    }
    /// 🌱 One-time hook for seeding the store's history (checkpoints/alternatives) beyond the bare
    /// `initial_projection` — called once from `VcsDocumentApp::new`, right after the store is
    /// constructed, via direct `store.dispatch(...)` calls. Default no-op; only apps whose fixture is
    /// itself a rich history (e.g. a history-UI demo/exerciser) need this — every plugin driven purely
    /// by user actions leaves it untouched.
    fn seed(&self, _store: &mut DocumentVcsStore<Self::Projection, Self::Op>) {}

    /// 🎞️ Declares this app's media-graph ports (empty by default — an app with no ports simply
    /// cannot be wired into a media graph; every other capability is unaffected).
    fn media_ports(&self) -> Vec<MediaPortSpec> {
        Vec::new()
    }
    /// 🎞️ Pure projection of the current document onto one declared output port — must not mutate
    /// anything. Called by both the UI (preview/export) and a headless runner (moving media along a
    /// media-graph edge). Default: `MediaError::NotImplemented` for apps that declare no ports.
    fn export_media(
        &self,
        _port: &str,
        _doc: &DocumentView<'_, Self::Projection>,
    ) -> Result<Media, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 🎞️ Translates an incoming media value on one declared input port into ops — never mutates
    /// state directly, so a headless import is exactly as undoable/syncable as a UI edit.
    fn import_media(
        &mut self,
        _port: &str,
        _media: &Media,
        _doc: &DocumentView<'_, Self::Projection>,
    ) -> Result<ActionEmit<Self::Op>, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 🎞️ Cheap identity for one output port's current value, without serializing the payload.
    /// Default re-derives it from `export_media`; override when a fingerprint is derivable without
    /// materializing the full export (e.g. from a cached head edit id).
    fn media_fingerprint(
        &self,
        port: &str,
        doc: &DocumentView<'_, Self::Projection>,
    ) -> Result<MediaFingerprint, MediaError> {
        self.export_media(port, doc).map(|media| MediaFingerprint::of(&media))
    }
}

/// @emoji 🗄️ Object-safe runtime contract every hosted app satisfies. Owns persistent document state
/// (via {@link VcsDocumentApp}'s store) across calls — no per-call document JSON is threaded in.
/// History actions (undo/redo/checkpoint/alternative) are intercepted by the wrapper; typed
/// operations are dispatched with real inverses; ops flow to/from the backbone as the wire format.
pub trait PluginApp: Send {
    fn app_id(&self) -> &str;
    fn document_schema(&self) -> &str;
    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        view_state: &ViewState,
        meta: &ActionMeta,
    ) -> Result<InvocationResult, String>;
    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        view_state: &ViewState,
        meta: &ActionMeta,
    ) -> Result<InvocationResult, String>;
    fn ingest_operations(&mut self, operations_json: &str) -> Result<(), String>;
    fn document_json(&self) -> Result<String, String>;
    fn load_document(&mut self, document_json: &str) -> Result<(), String>;
    fn attach_backbone(&mut self, backbone: Box<dyn vcs::Backbone>) -> Result<(), String>;
    fn detach_backbone(&mut self);
    fn render(
        &mut self,
        body_key: &str,
        projection_override_json: Option<&str>,
        view_state: &ViewState,
    ) -> Result<UiNode, String>;
    fn window_engagements(&mut self, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        HashMap::new()
    }
    fn window_measures(&mut self, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }
    fn app_labels(&mut self, _view_state: &ViewState) -> AppLabelsOverlay {
        AppLabelsOverlay::default()
    }
    /// 🎞️ Object-safe counterpart to `DocumentApp::export_media` — the seam a headless media-graph
    /// runner calls without knowing the app's concrete `Projection`/`Op` types.
    fn export_media(&mut self, _port: &str) -> Result<Media, MediaError> {
        Err(MediaError::NotImplemented)
    }
    /// 🎞️ Object-safe counterpart to `DocumentApp::import_media` — dispatches through the same
    /// `DocumentVcsStore` as `handle_action`, so a headless import is an ordinary, undoable edit.
    fn import_media(&mut self, _port: &str, _media: &Media, _meta: &ActionMeta) -> Result<InvocationResult, String> {
        Err(MediaError::NotImplemented.to_string())
    }
    fn media_fingerprint(&mut self, _port: &str) -> Result<MediaFingerprint, MediaError> {
        Err(MediaError::NotImplemented)
    }
}

/// @emoji 📇 An app's action declarations indexed by id, built from its {@link AppDefinition}. Threaded
/// into {@link VcsDocumentApp} at registration time so the wrapper can enforce the actions contract
/// (default materialization, required-arg validation, kind discipline) without the plugin re-checking.
/// An empty registry (the test/registry-less construction path) skips all enforcement.
#[derive(Clone, Default)]
pub struct AppActionRegistry {
    actions: HashMap<String, ActionDefinition>,
    commands: HashMap<String, CommandDefinition>,
}

impl AppActionRegistry {
    /// @emoji 📇 Indexes an app definition's declared actions and commands (including
    /// framework-injected ones) by id.
    pub fn from_definition(definition: &AppDefinition) -> Self {
        Self {
            actions: definition.actions.iter().map(|action| (action.id.clone(), action.clone())).collect(),
            commands: definition.commands.iter().map(|command| (command.id.clone(), command.clone())).collect(),
        }
    }

    fn get(&self, id: &str) -> Option<&ActionDefinition> {
        self.actions.get(id)
    }

    fn get_command(&self, id: &str) -> Option<&CommandDefinition> {
        self.commands.get(id)
    }
}

/// @emoji 🧬 Generic wrapper turning any typed {@link DocumentApp} into the object-safe runtime
/// {@link PluginApp}. Owns a persistent `DocumentVcsStore<Projection, Op>` — the single source of
/// truth for the app's document across every call — intercepts the six injected history actions into
/// `DocumentVcsCommand`s, dispatches `Apply`/`AmendLast` for typed operations, and builds an
/// `InvocationResult` whose inverses come from the just-recorded `Edit.backwards`. A projection cache
/// keyed on the store's generation counter keeps renders O(1). Holds an {@link AppActionRegistry} to
/// enforce the actions contract before/after delegating to the app.
pub struct VcsDocumentApp<A: DocumentApp> {
    app: A,
    store: DocumentVcsStore<A::Projection, A::Op>,
    cache: Option<(u64, A::Projection, HistoryView)>,
    registry: AppActionRegistry,
}

const HISTORY_ACTION_IDS: [&str; 6] = [
    "undo",
    "redo",
    "commitCheckpoint",
    "createAlternative",
    "switchAlternative",
    "checkoutCheckpoint",
];

impl<A: DocumentApp> VcsDocumentApp<A> {
    /// @emoji 🧬 Constructs a wrapper with an empty registry — contract enforcement is skipped. Used by
    /// tests and any registry-less construction path.
    pub fn new(app: A) -> Self {
        Self::with_registry(app, AppActionRegistry::default())
    }

    /// @emoji 🧬 Constructs a wrapper carrying the app's {@link AppActionRegistry} so `handle_action`
    /// enforces default materialization, required-arg validation, and kind discipline.
    pub fn with_registry(app: A, registry: AppActionRegistry) -> Self {
        let envelope = create_document_vcs_envelope::<A::Projection, A::Op>(
            app.document_schema(),
            app.app_id(),
            app.initial_projection(),
            None,
        );
        let mut store = DocumentVcsStore::new(envelope);
        app.seed(&mut store);
        Self {
            app,
            store,
            cache: None,
            registry,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_projection(&self) -> A::Projection {
        self.store.projection().expect("materialize projection")
    }

    /// @emoji 📸 Materializes and returns the current projection — the typed counterpart to
    /// `render`'s `UiNode` output, for callers (host code, downstream plugin crates' own tests) that
    /// need direct structural access to document state instead of a rendered node.
    pub fn projection(&self) -> Result<A::Projection, String> {
        self.store.projection().map_err(|error| error.to_string())
    }

    /// @emoji 🤝 Fresh replay plus whatever `Op::reconcile` reports for the result — the typed
    /// counterpart to `vcs::DocumentVcsStore::projection_with_conflicts`.
    pub fn projection_with_conflicts(&self) -> Result<(A::Projection, Vec<StudioConflict>), String> {
        self.store.projection_with_conflicts().map_err(|error| error.to_string())
    }

    fn build_history_view(&self) -> HistoryView {
        HistoryView {
            columns: build_history_columns(self.store.envelope()),
            can_undo: !self.store.applied_edit_ids().is_empty(),
            can_redo: !self.store.redo_edit_ids().is_empty(),
            active_alternative_id: self.store.envelope().active_alternative_id.clone(),
            current_checkpoint_id: self.store.current_checkpoint_id().map(str::to_string),
        }
    }

    /// @emoji 🗂️ Refreshes the projection cache if the store advanced since the last materialization.
    fn refresh_cache(&mut self) -> Result<(), String> {
        let generation = self.store.generation();
        if self.cache.as_ref().map(|(gen, _, _)| *gen) != Some(generation) {
            let projection = self.store.projection().map_err(|error| error.to_string())?;
            let history = self.build_history_view();
            self.cache = Some((generation, projection, history));
        }
        Ok(())
    }

    /// @emoji 🕰️ Maps one of the six injected history action ids to its `DocumentVcsCommand`.
    fn history_command(action: &str, args: Option<&Value>) -> Option<DocumentVcsCommand<A::Op>> {
        let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "undo" => Some(DocumentVcsCommand::Undo),
            "redo" => Some(DocumentVcsCommand::Redo),
            "commitCheckpoint" => Some(DocumentVcsCommand::CommitCheckpoint {
                message: arg_str("message"),
                authors: Vec::new(),
            }),
            "createAlternative" => Some(DocumentVcsCommand::CreateAlternative {
                name: arg_str("name").unwrap_or_else(|| "Alternative".into()),
            }),
            "switchAlternative" => arg_str("alternativeId")
                .map(|alternative_id| DocumentVcsCommand::SwitchAlternative { alternative_id }),
            "checkoutCheckpoint" => arg_str("checkpointId")
                .map(|checkpoint_id| DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id }),
            _ => None,
        }
    }

    /// @emoji 📇 An empty `InvocationResult` carrying only host effects/events (view/shell actions,
    /// no-op commands, and history notifications produce no `KernelOperation`s).
    fn empty_result(verb: &str, meta: &ActionMeta, effects: Vec<HostEffect>, events: Vec<AppEvent>, ui_scope: semio_framework_core::kernel::UiDirtyScope) -> InvocationResult {
        let invocation_id = InvocationId(format!("{verb}:{}", meta.instance_id));
        InvocationResult {
            output: Value::Null,
            operations: Vec::new(),
            inverse_group: UndoGroup {
                invocation_id,
                operations: Vec::new(),
                inverse_operations: Vec::new(),
            },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
        }
    }

    /// @emoji 🧱 Builds the `InvocationResult` for a just-dispatched edit: one `KernelOperation` per
    /// forward operation, each carrying the edit's true `backwards` as its inverse diff.
    fn result_from_last_edit(&self, verb: &str, meta: &ActionMeta, effects: Vec<HostEffect>, events: Vec<AppEvent>, ui_scope: semio_framework_core::kernel::UiDirtyScope) -> InvocationResult {
        let schema = self.app.document_schema().to_string();
        let invocation_id = InvocationId(format!("{verb}:{}:{}", meta.instance_id, self.store.generation()));
        let document = DocumentHandle(meta.instance_id as u128);
        let mut operations: Vec<KernelOperation> = Vec::new();
        if let Some((forwards, backwards, operation_meta)) = self.store.edit_operations() {
            let inverse_payload = serde_json::json!({ "backwards": backwards });
            for (index, forward) in forwards.iter().enumerate() {
                let entry = operation_meta.get(index);
                let operation_id = OperationId(
                    entry
                        .map(|meta| meta.operation_id.clone())
                        .unwrap_or_else(|| format!("{}:{index}", invocation_id.0)),
                );
                let base_version = DocumentVersion(entry.map(|meta| meta.base_version).unwrap_or(0));
                let undo_policy = entry.map(|meta| meta.undo_policy).unwrap_or(UndoPolicy::ExactBaseOnly);
                let author = ActorId(entry.map(|meta| meta.author_id.clone()).unwrap_or_else(|| meta.actor.clone()));
                let timestamp = entry
                    .map(|meta| meta.timestamp.clone())
                    .unwrap_or_else(|| HybridLogicalTimestamp::new(0, 0));
                operations.push(KernelOperation {
                    id: operation_id.clone(),
                    document,
                    base_version,
                    invocation_id: invocation_id.clone(),
                    diff: DocumentDiff {
                        schema_id: SchemaId(format!("{schema}.op")),
                        payload: serde_json::to_value(forward).unwrap_or(Value::Null),
                    },
                    inverse: InverseOperation {
                        target_operation: operation_id,
                        inverse_diff: DocumentDiff {
                            schema_id: SchemaId(format!("{schema}.op.inverse")),
                            payload: inverse_payload.clone(),
                        },
                        base_version,
                        dependencies: Vec::new(),
                        undo_policy,
                    },
                    dependencies: Vec::new(),
                    author,
                    timestamp,
                });
            }
        }
        let operation_ids: Vec<OperationId> = operations.iter().map(|op| op.id.clone()).collect();
        let inverse_operations: Vec<InverseOperation> = operations.iter().map(|op| op.inverse.clone()).collect();
        InvocationResult {
            output: Value::Null,
            operations,
            inverse_group: UndoGroup {
                invocation_id,
                operations: operation_ids,
                inverse_operations,
            },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
        }
    }

    /// @emoji 🧮 Shared arg materialization for `handle_action`/`handle_command`: fills declared
    /// defaults over the staged args and rejects if any required arg is still missing.
    fn materialize_args(label: &str, defs: &[ActionArgDef], args: Option<&Value>) -> Result<Value, String> {
        let staged = args.and_then(Value::as_object).cloned().unwrap_or_default();
        let effective = effective_action_args(defs, &staged);
        let missing = missing_required_args(defs, &effective);
        if !missing.is_empty() {
            return Err(format!("{label} missing required args: {missing:?}"));
        }
        let mut merged = staged;
        for (key, value) in effective {
            merged.entry(key).or_insert(value);
        }
        Ok(Value::Object(merged))
    }

    /// @emoji 🧬 Shared dispatch tail for `handle_action`/`handle_command`: given the app's `ActionEmit`,
    /// either returns an empty result (no ops) or commits `Apply`/`AmendLast` and builds the
    /// `InvocationResult` from the just-recorded edit. `verb` is the action/command id, used only to
    /// synthesize the `InvocationId`.
    fn dispatch_emit(&mut self, verb: &str, emit: ActionEmit<A::Op>, meta: &ActionMeta) -> Result<InvocationResult, String> {
        let ActionEmit { ops, description, coalesce_key, effects, events, ui_scope } = emit;
        if ops.is_empty() {
            return Ok(Self::empty_result(verb, meta, effects, events, ui_scope));
        }
        self.store.set_local_actor_id(Some(meta.actor.clone()));
        let vcs_command = match coalesce_key {
            Some(key) => DocumentVcsCommand::AmendLast {
                operations: ops,
                coalesce_key: Some(key),
            },
            None => DocumentVcsCommand::Apply {
                operations: ops,
                description,
            },
        };
        self.store.dispatch(vcs_command).map_err(|error| error.to_string())?;
        self.cache = None;
        Ok(self.result_from_last_edit(verb, meta, effects, events, ui_scope))
    }
}

/// @emoji 📣 Signals the shell that the document's checkpoint/alternative history changed (after an
/// undo/redo/checkpoint/alternative command) so it can re-render history-dependent surfaces.
fn history_changed_event() -> AppEvent {
    AppEvent {
        kind: "history-changed".into(),
        payload: Value::Null,
    }
}

impl<A: DocumentApp> PluginApp for VcsDocumentApp<A> {
    fn app_id(&self) -> &str {
        self.app.app_id()
    }

    fn document_schema(&self) -> &str {
        self.app.document_schema()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        view_state: &ViewState,
        meta: &ActionMeta,
    ) -> Result<InvocationResult, String> {
        if HISTORY_ACTION_IDS.contains(&action) {
            let command = Self::history_command(action, args)
                .ok_or_else(|| format!("history action {action} missing required argument"))?;
            match self.store.dispatch(command) {
                Ok(()) => {
                    self.cache = None;
                    // 🐢 History actions (undo/redo/checkpoint/alternative) can touch any part of the
                    // document — always Full, never opt into a narrower scope.
                    Ok(Self::empty_result(action, meta, Vec::new(), vec![history_changed_event()], semio_framework_core::kernel::UiDirtyScope::Full))
                }
                // Benign no-ops (nothing to undo/redo, foreign tail) collapse to an empty result.
                Err(vcs::VcsError::NothingToUndo)
                | Err(vcs::VcsError::NothingToRedo)
                | Err(vcs::VcsError::ForeignEdit(_)) => {
                    Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), semio_framework_core::kernel::UiDirtyScope::None))
                }
                Err(error) => Err(error.to_string()),
            }
        } else {
            self.refresh_cache()?;
            let definition = self.registry.get(action).cloned();
            let materialized_args: Option<Value> = definition.as_ref()
                .map(|def| Self::materialize_args(&format!("action '{action}'"), &def.args, args))
                .transpose()?;
            let dispatch_args = materialized_args.as_ref().or(args);
            let emit = {
                let VcsDocumentApp { app, cache, .. } = self;
                let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
                let doc = DocumentView { projection, history };
                app.handle_action(action, dispatch_args, &doc, view_state)
            };
            if let Some(def) = &definition {
                if matches!(def.kind, ActionKind::View | ActionKind::Shell) && !emit.ops.is_empty() {
                    return Err(format!(
                        "{:?}-kind action '{action}' must not emit operations",
                        def.kind
                    ));
                }
            }
            self.dispatch_emit(action, emit, meta)
        }
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        view_state: &ViewState,
        meta: &ActionMeta,
    ) -> Result<InvocationResult, String> {
        self.refresh_cache()?;
        let definition = self.registry.get_command(command).cloned()
            .ok_or_else(|| format!("unknown command '{command}'"))?;
        let materialized_args = Self::materialize_args(&format!("command '{command}'"), &definition.args, args)?;
        let emit = {
            let VcsDocumentApp { app, cache, .. } = self;
            let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
            let doc = DocumentView { projection, history };
            app.handle_command(command, Some(&materialized_args), &doc, view_state)
        };
        self.dispatch_emit(command, emit, meta)
    }

    fn ingest_operations(&mut self, operations_json: &str) -> Result<(), String> {
        let envelopes: Vec<OpEnvelope> =
            serde_json::from_str(operations_json).map_err(|error| error.to_string())?;
        for envelope in envelopes {
            self.store.ingest_remote(envelope).map_err(|error| error.to_string())?;
        }
        self.cache = None;
        Ok(())
    }

    fn document_json(&self) -> Result<String, String> {
        self.store.envelope_json().map_err(|error| error.to_string())
    }

    fn load_document(&mut self, document_json: &str) -> Result<(), String> {
        let envelope: DocumentVcsEnvelope<A::Projection, A::Op> =
            serde_json::from_str(document_json).map_err(|error| error.to_string())?;
        let applied: Vec<String> = envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
        self.store.set_envelope(envelope, applied);
        self.cache = None;
        Ok(())
    }

    fn attach_backbone(&mut self, backbone: Box<dyn vcs::Backbone>) -> Result<(), String> {
        self.store.attach_backbone(backbone).map_err(|error| error.to_string())?;
        self.cache = None;
        Ok(())
    }

    fn detach_backbone(&mut self) {
        self.store.detach_backbone();
        self.cache = None;
    }

    fn render(
        &mut self,
        body_key: &str,
        projection_override_json: Option<&str>,
        view_state: &ViewState,
    ) -> Result<UiNode, String> {
        self.refresh_cache()?;
        if let Some(json) = projection_override_json {
            let projection: A::Projection =
                serde_json::from_str(json).map_err(|error| error.to_string())?;
            let history = self.build_history_view();
            let doc = DocumentView { projection: &projection, history: &history };
            return Ok(self.app.render(body_key, &doc, view_state));
        }
        let VcsDocumentApp { app, cache, .. } = self;
        let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
        let doc = DocumentView { projection, history };
        Ok(app.render(body_key, &doc, view_state))
    }

    fn window_engagements(&mut self, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        if self.refresh_cache().is_err() {
            return HashMap::new();
        }
        let VcsDocumentApp { app, cache, .. } = self;
        let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
        let doc = DocumentView { projection, history };
        app.window_engagements(&doc, view_state)
    }

    fn window_measures(&mut self, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        if self.refresh_cache().is_err() {
            return HashMap::new();
        }
        let VcsDocumentApp { app, cache, .. } = self;
        let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
        let doc = DocumentView { projection, history };
        app.window_measures(&doc, view_state)
    }

    fn export_media(&mut self, port: &str) -> Result<Media, MediaError> {
        self.refresh_cache().map_err(|error| MediaError::Payload(port.to_string(), error))?;
        let VcsDocumentApp { app, cache, .. } = self;
        let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
        let doc = DocumentView { projection, history };
        app.export_media(port, &doc)
    }

    fn import_media(&mut self, port: &str, media: &Media, meta: &ActionMeta) -> Result<InvocationResult, String> {
        self.refresh_cache()?;
        let emit = {
            let VcsDocumentApp { app, cache, .. } = self;
            let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
            let doc = DocumentView { projection, history };
            app.import_media(port, media, &doc).map_err(|error| error.to_string())?
        };
        self.dispatch_emit(&format!("import-media:{port}"), emit, meta)
    }

    fn media_fingerprint(&mut self, port: &str) -> Result<MediaFingerprint, MediaError> {
        self.refresh_cache().map_err(|error| MediaError::Payload(port.to_string(), error))?;
        let VcsDocumentApp { app, cache, .. } = self;
        let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
        let doc = DocumentView { projection, history };
        app.media_fingerprint(port, &doc)
    }

    fn app_labels(&mut self, view_state: &ViewState) -> AppLabelsOverlay {
        self.app.app_labels(view_state)
    }
}

pub struct AppInstance {
    pub id: u32,
    pub app: Box<dyn PluginApp>,
}
//#endregion 🔖DocumentContract

pub trait Plugin: Send {
    fn manifest(&self) -> PluginManifest;
    fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>>;
}

pub struct PluginBundle {
    pub manifest: PluginManifest,
    apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
}

impl PluginBundle {
    pub fn new(plugin_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            manifest: PluginManifest {
                plugin_id: plugin_id.into(),
                label: label.into(),
                version: version.into(),
                apps: Vec::new(),
                programs: Vec::new(),
                examples: Vec::new(),
                capabilities: Vec::new(),
                contributions: Vec::new(),
                commands: Vec::new(),
            },
            apps: HashMap::new(),
        }
    }

    /// @emoji 🎛️ Declares a plugin-scope command (applies whenever any of this plugin's apps is
    /// focused). Panics if `command.scope != CommandScope::Plugin`.
    pub fn plugin_command(mut self, command: CommandDefinition) -> Self {
        assert!(
            command.scope == CommandScope::Plugin,
            "plugin {} command {} must be declared CommandScope::Plugin",
            self.manifest.plugin_id,
            command.id
        );
        self.manifest.commands.push(command);
        self
    }

    pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
        if !self.manifest.capabilities.contains(&capability) {
            self.manifest.capabilities.push(capability);
        }
        self
    }

    pub fn contributes(mut self, contribution: Contribution) -> Self {
        self.manifest.contributions.push(contribution);
        self
    }

    pub fn local_backbone_storage(self) -> Self {
        self.capability(CapabilityRequirement {
            resource: ResourceKind::Backbone,
            rights: Rights::Read,
            scope: Scope::Plugin,
        })
        .capability(CapabilityRequirement {
            resource: ResourceKind::Backbone,
            rights: Rights::Write,
            scope: Scope::Plugin,
        })
    }

    pub fn register_app(
        mut self,
        app: App,
        factory: impl Fn() -> Box<dyn PluginApp> + Send + 'static,
    ) -> Self {
        let app_id = app.definition.id.clone();
        self.manifest.apps.push(app.definition);
        for mut example in app.examples {
            example.app_id = app_id.clone();
            self.manifest.examples.push(example);
        }
        if let Some(program) = app.program {
            self.manifest.programs.push(program);
        }
        self.apps
            .insert(self.manifest.apps.last().unwrap().id.clone(), Box::new(factory));
        self
    }

    /// @emoji 🧬 Registers a typed {@link DocumentApp}, wrapping each instance in a
    /// {@link VcsDocumentApp} so it satisfies the object-safe runtime {@link PluginApp} contract with
    /// a persistent op store. Sibling to {@link register_app}, which takes a pre-boxed `PluginApp`.
    pub fn register_document_app<A>(
        self,
        app: App,
        factory: impl Fn() -> A + Send + 'static,
    ) -> Self
    where
        A: DocumentApp,
    {
        let registry = AppActionRegistry::from_definition(&app.definition);
        self.register_app(app, move || {
            Box::new(VcsDocumentApp::with_registry(factory(), registry.clone()))
        })
    }

    pub fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
        self.apps.get(app_id).map(|factory| factory())
    }
}

impl Plugin for PluginBundle {
    fn manifest(&self) -> PluginManifest {
        self.manifest.clone()
    }

    fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
        PluginBundle::create_app(self, app_id)
    }
}
// #endregion app
}

pub mod plugin_runtime {
// #region plugin_runtime
//! 📤 WASM component export glue for plugin bundles.

use crate::app::{ActionMeta, AppInstance, Plugin, PluginBundle};
use semio_framework_core::{kernel::InvocationResult, PluginManifest, ViewState};
use ui_wgpu::{framework_panel_tab_label, UiNode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicU32, Ordering};

thread_local! {
    static PLUGIN: RefCell<Option<PluginBundle>> = const { RefCell::new(None) };
    static INSTANCES: RefCell<Vec<AppInstance>> = const { RefCell::new(Vec::new()) };
    static INSTANCE_GUARD: Cell<u32> = const { Cell::new(0) };
}

struct InstanceGuard;

impl InstanceGuard {
    fn enter() -> Result<Self, String> {
        if INSTANCE_GUARD.get() > 0 {
            return Err("plugin instance busy".to_string());
        }
        INSTANCE_GUARD.set(1);
        Ok(Self)
    }
}

impl Drop for InstanceGuard {
    fn drop(&mut self) {
        INSTANCE_GUARD.set(0);
    }
}

fn with_instances_mut<R, F: FnOnce(&mut Vec<AppInstance>) -> Result<R, String>>(f: F) -> Result<R, String> {
    let _guard = InstanceGuard::enter()?;
    INSTANCES.with(|instances| f(&mut instances.borrow_mut()))
}


static NEXT_INSTANCE_ID: AtomicU32 = AtomicU32::new(1);

pub fn install_plugin_bundle(bundle: PluginBundle) {
    PLUGIN.with(|slot| {
        *slot.borrow_mut() = Some(bundle);
    });
}

static PLUGIN_INIT_ONCE: std::sync::Once = std::sync::Once::new();

extern "C" {
    fn semio_plugin_install_bundle();
}

/// Ensures the embedding plugin crate's bundle installer ran before any WIT export is served.
pub fn ensure_plugin_initialized() {
    PLUGIN_INIT_ONCE.call_once(|| {
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        crate::host_port::register_host_backbone_channel();
        unsafe {
            semio_plugin_install_bundle();
        }
    });
}

pub fn plugin_manifest() -> PluginManifest {
    ensure_plugin_initialized();
    PLUGIN.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|plugin| plugin.manifest())
            .unwrap_or_else(|| PluginManifest {
                plugin_id: "empty".into(),
                label: "Empty".into(),
                version: "0.0.0".into(),
                apps: vec![],
                programs: vec![],
                examples: vec![],
                capabilities: vec![],
                contributions: vec![],
                commands: vec![],
            })
    })
}

pub fn plugin_create_app(app_id: &str) -> Result<u32, String> {
    PLUGIN.with(|slot| {
        let plugin = slot.borrow();
        let plugin = plugin.as_ref().ok_or_else(|| "plugin not initialized".to_string())?;
        let app = plugin
            .create_app(app_id)
            .ok_or_else(|| format!("unknown app: {app_id}"))?;
        let id = NEXT_INSTANCE_ID.fetch_add(1, Ordering::SeqCst);
        with_instances_mut(|list| {
            list.push(AppInstance { id, app });
            Ok(())
        })?;
        Ok(id)
    })
}

pub fn plugin_destroy_app(instance_id: u32) -> Result<(), String> {
    with_instances_mut(|list| {
        let index = list
            .iter()
            .position(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        list.remove(index);
        Ok(())
    })
}

fn find_instance(list: &mut [AppInstance], instance_id: u32) -> Result<&mut AppInstance, String> {
    list.iter_mut()
        .find(|instance| instance.id == instance_id)
        .ok_or_else(|| format!("unknown instance: {instance_id}"))
}

pub fn plugin_handle_action(
    instance_id: u32,
    action_json: &str,
    context_json: &str,
) -> Result<InvocationResult, String> {
    let action: serde_json::Value =
        serde_json::from_str(action_json).map_err(|error| error.to_string())?;
    let context: serde_json::Value =
        serde_json::from_str(context_json).map_err(|error| error.to_string())?;
    let view_state: ViewState = context
        .get("viewState")
        .cloned()
        .map(|value| serde_json::from_value(value).unwrap_or_default())
        .unwrap_or_default();
    let action_name = action.get("action").and_then(|value| value.as_str()).unwrap_or("");
    let args = action.get("args").cloned();
    let actor = context
        .get("actor")
        .and_then(|value| value.as_str())
        .unwrap_or("local")
        .to_string();
    let meta = ActionMeta { actor, instance_id };
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.handle_action(action_name, args.as_ref(), &view_state, &meta)
    })
}

/// @emoji 🎛️ Dispatches a scoped command (os/plugin/app/mode) through the same instance/context
/// parsing as `plugin_handle_action` — mirrors its shape exactly.
pub fn plugin_handle_command(
    instance_id: u32,
    command_json: &str,
    context_json: &str,
) -> Result<InvocationResult, String> {
    let command: serde_json::Value =
        serde_json::from_str(command_json).map_err(|error| error.to_string())?;
    let context: serde_json::Value =
        serde_json::from_str(context_json).map_err(|error| error.to_string())?;
    let view_state: ViewState = context
        .get("viewState")
        .cloned()
        .map(|value| serde_json::from_value(value).unwrap_or_default())
        .unwrap_or_default();
    let command_name = command.get("command").and_then(|value| value.as_str()).unwrap_or("");
    let args = command.get("args").cloned();
    let actor = context
        .get("actor")
        .and_then(|value| value.as_str())
        .unwrap_or("local")
        .to_string();
    let meta = ActionMeta { actor, instance_id };
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.handle_command(command_name, args.as_ref(), &view_state, &meta)
    })
}

/// @emoji 📥 Ingests a JSON array of remote `OpEnvelope`s into the instance's document store
/// (idempotent — duplicate op ids are dropped by the causal DAG / edit-id dedupe).
pub fn plugin_ingest_operations(instance_id: u32, operations_json: &str) -> Result<(), String> {
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.ingest_operations(operations_json)
    })
}

/// @emoji 📖 Serializes the instance's full persistent document (the `DocumentVcsEnvelope`).
pub fn plugin_document(instance_id: u32) -> Result<String, String> {
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.document_json()
    })
}

/// @emoji 📂 Replaces the instance's document from a serialized `DocumentVcsEnvelope`.
pub fn plugin_load_document(instance_id: u32, document_json: &str) -> Result<(), String> {
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.load_document(document_json)
    })
}

/// @emoji 🔗 Attaches a backbone channel by URI. The URI is resolved to a `vcs::PortBackbone`
/// (a pure queue relayed across the wasm sandbox to the host); the host owns the real IO endpoint.
pub fn plugin_attach_backbone(instance_id: u32, uri: &str) -> Result<(), String> {
    let backbone: Box<dyn vcs::Backbone> = Box::new(vcs::PortBackbone::new(uri));
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.attach_backbone(backbone)
    })
}

/// @emoji ✂️ Detaches the instance's backbone channel; the document graph stays in memory.
pub fn plugin_detach_backbone(instance_id: u32) -> Result<(), String> {
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.detach_backbone();
        Ok(())
    })
}

pub fn plugin_render(instance_id: u32, body_key: &str, view_state_json: &str) -> Result<UiNode, String> {
    plugin_render_with_document(instance_id, body_key, None, view_state_json)
}

pub fn plugin_render_with_document(
    instance_id: u32,
    body_key: &str,
    projection_override_json: Option<&str>,
    view_state_json: &str,
) -> Result<UiNode, String> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct WindowRenderInput {
        #[serde(default)]
        body_key: String,
        view_state: ViewState,
        #[serde(default)]
        document_json: Option<String>,
    }
    let (resolved_body_key, view_state, override_projection) = if body_key.is_empty() {
        let input: WindowRenderInput =
            serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
        (input.body_key, input.view_state, input.document_json)
    } else if let Ok(input) = serde_json::from_str::<WindowRenderInput>(view_state_json) {
        let key = if input.body_key.is_empty() { body_key.to_string() } else { input.body_key };
        (
            key,
            input.view_state,
            input.document_json.or_else(|| projection_override_json.map(str::to_string)),
        )
    } else {
        let view_state: ViewState =
            serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
        (body_key.to_string(), view_state, projection_override_json.map(str::to_string))
    };
    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        instance
            .app
            .render(&resolved_body_key, override_projection.as_deref(), &view_state)
    })
}

//#region 🔖RefreshUi
/// 🐢 A tiny non-cryptographic hash (fnv1a-64) for cheap "did this section's content change" checks —
/// not a security boundary, just change detection, so speed over collision-resistance is the right
/// tradeoff (mirrors the identical pattern already used for `cached_fixture_json` in puzzle's plugin).
fn ui_refresh_fnv1a_hash(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

/// 🐢 Hashes `value`'s canonical JSON serialization and returns `(hash, Some(value))` when it differs
/// from `known_hash`, or `(hash, None)` when unchanged — the response omits the payload either way the
/// caller doesn't need it, keeping the wire payload proportional to what actually changed.
fn ui_refresh_section<T: Serialize>(value: &T, known_hash: Option<&str>) -> (String, Option<Value>) {
    let payload = serde_json::to_value(value).unwrap_or(Value::Null);
    let hash = ui_refresh_fnv1a_hash(serde_json::to_string(&payload).unwrap_or_default().as_bytes());
    if known_hash == Some(hash.as_str()) {
        (hash, None)
    } else {
        (hash, Some(payload))
    }
}

/// 🐢 Batched, hash-conditional UI refresh: replaces the individual
/// `render`/`windowEngagements`/`windowMeasures`/`appLabels` WASM round trips a full `refreshUi` used
/// to make with **one** call. `request_json` lists every section the host wants (windows/panels by
/// `{key, bodyKey, hash}`, engagements/measures/labels each `{hash}`); the response includes a payload
/// only for sections whose hash differs from what the host already holds. Toolbars are no longer a
/// plugin section — the renderer derives them from the utility registry via `derive_utility_nodes`.
pub fn plugin_refresh_ui(instance_id: u32, request_json: &str) -> Result<String, String> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SectionRequest {
        key: String,
        #[serde(default)]
        body_key: String,
        #[serde(default)]
        hash: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SingleRequest {
        #[serde(default)]
        hash: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RefreshRequest {
        view_state: ViewState,
        #[serde(default)]
        windows: Vec<SectionRequest>,
        #[serde(default)]
        panels: Vec<SectionRequest>,
        #[serde(default)]
        utilities: Vec<SectionRequest>,
        #[serde(default)]
        engagements: Option<SingleRequest>,
        #[serde(default)]
        measures: Option<SingleRequest>,
        #[serde(default)]
        labels: Option<SingleRequest>,
    }
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct SectionResponse {
        key: String,
        hash: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Value>,
    }
    #[derive(Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct RefreshResponse {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        windows: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        panels: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        utilities: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        engagements: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        measures: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        labels: Option<SectionResponse>,
    }

    let request: RefreshRequest = serde_json::from_str(request_json).map_err(|error| error.to_string())?;
    let is_de = request.view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    let manifest = plugin_manifest();

    with_instances_mut(|list| {
        let instance = find_instance(list, instance_id)?;
        let app_id = instance.app.app_id().to_string();
        let panel_tab_ids: Vec<String> = manifest
            .apps
            .iter()
            .find(|app| app.id == app_id)
            .map(|app| app.panel_tabs.iter().map(|tab| tab.id().to_string()).collect())
            .unwrap_or_default();

        let mut response = RefreshResponse::default();

        for entry in &request.windows {
            let node = instance.app.render(&entry.body_key, None, &request.view_state)?;
            let (hash, value) = ui_refresh_section(&node, entry.hash.as_deref());
            response.windows.push(SectionResponse { key: entry.key.clone(), hash, value });
        }
        for entry in &request.panels {
            let node = instance.app.render(&entry.body_key, None, &request.view_state)?;
            let (hash, value) = ui_refresh_section(&node, entry.hash.as_deref());
            response.panels.push(SectionResponse { key: entry.key.clone(), hash, value });
        }
        // 🚧 `utilities` intentionally unhandled here: `PluginApp`/`DocumentApp` currently expose no
        // object-safe `utilities()` accessor (mid-refactor elsewhere toward a declarative `mode_utilities(...)`
        // builder — unrelated to this ticket). No puzzle2d scope ever requests `utilities: true` (it uses
        // static mode-level utilities only), so `request.utilities` is always empty in practice; wire this up
        // once the utilities API refactor lands.
        let _ = &request.utilities;
        if let Some(requested) = &request.engagements {
            let engagements = instance.app.window_engagements(&request.view_state);
            let (hash, value) = ui_refresh_section(&engagements, requested.hash.as_deref());
            response.engagements = Some(SectionResponse { key: "engagements".into(), hash, value });
        }
        if let Some(requested) = &request.measures {
            let measures = instance.app.window_measures(&request.view_state);
            let (hash, value) = ui_refresh_section(&measures, requested.hash.as_deref());
            response.measures = Some(SectionResponse { key: "measures".into(), hash, value });
        }
        if let Some(requested) = &request.labels {
            let mut overlay = instance.app.app_labels(&request.view_state);
            for id in &panel_tab_ids {
                if let Some(label) = framework_panel_tab_label(id, is_de) {
                    overlay.panel_tab_labels.entry(id.clone()).or_insert_with(|| label.into());
                }
            }
            let (hash, value) = ui_refresh_section(&overlay, requested.hash.as_deref());
            response.labels = Some(SectionResponse { key: "labels".into(), hash, value });
        }

        Ok(serde_json::to_string(&response).unwrap_or_else(|_| "{}".into()))
    })
}
//#endregion 🔖RefreshUi

#[macro_export]
macro_rules! plugin_exports {
    ($bundle_fn:expr) => {
        #[doc(hidden)]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_plugin_install_bundle() {
            $crate::plugin_runtime::install_plugin_bundle(($bundle_fn)());
        }

        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[used]
        static _SEMIO_PLUGIN_COMPONENT_LINK: fn() = $crate::component_export_anchor;
    };
}

/// 🧵 Collapses a plugin crate's hand-written `bundle()` fn + `plugin_exports!` call into one
/// declarative block: `semio_framework_plugin::semio_plugin! { id: "note", label: "Note", version:
/// "0.1.0", setup: register_note_exports, apps: [ create_note_app => NoteApp ] }`. Each `apps` entry
/// pairs an `App`-returning factory function with the [`DocumentApp`](crate::DocumentApp) type
/// instantiated for it — that type must implement `Default` (multi-app crates list one entry per
/// app, e.g. puzzle's `d2::create_puzzle2d_app => d2::Puzzle2dApp, d3::create_puzzle3d_app =>
/// d3::Puzzle3dApp`). Each app is wrapped in a [`VcsDocumentApp`](crate::VcsDocumentApp) so it
/// satisfies the object-safe runtime [`PluginApp`](crate::PluginApp) contract with a persistent op
/// store. Expands to the equivalent `bundle()` fn plus a `plugin_exports!(bundle)` call, and a
/// `#[cfg(test)]` regression check asserting every declared app id actually lands in the built
/// `PluginBundle`'s manifest.
#[macro_export]
macro_rules! semio_plugin {
    (
        id: $id:literal,
        label: $label:literal,
        version: $version:literal,
        setup: $setup:path,
        apps: [ $( $app_fn:path => $app_ty:path ),+ $(,)? ] $(,)?
    ) => {
        fn __semio_plugin_bundle() -> $crate::PluginBundle {
            ($setup)();
            $crate::PluginBundle::new($id, $label, $version)
                $( .register_document_app(($app_fn)(), || <$app_ty as ::std::default::Default>::default()) )+
        }

        $crate::plugin_exports!(__semio_plugin_bundle);

        #[cfg(test)]
        #[test]
        fn __semio_plugin_sanity_declared_apps_appear_in_bundle_manifest() {
            let manifest = __semio_plugin_bundle().manifest;
            $(
                let expected_id = ($app_fn)().definition.id;
                assert!(
                    manifest.apps.iter().any(|app| app.id == expected_id),
                    "semio_plugin({}): app `{}` (from `{}`) missing from bundle manifest",
                    $id,
                    expected_id,
                    stringify!($app_fn),
                );
            )+
        }
    };
}

#[cfg(test)]
mod semio_plugin_macro_tests {
    //! 🧪 The plugin contract's own unit test: a `TestApp` implementing the typed `DocumentApp`
    //! surface, wrapped in `VcsDocumentApp`, exercising typed operations with true inverses, view
    //! actions that emit no ops, history interception, and remote-op ingest idempotency.

    use crate::app::{
        ActionEmit, ActionMeta, App, AppActionRegistry, DocumentApp, DocumentView, PluginApp, VcsDocumentApp,
    };
    use crate::{ui_text, SurfaceKind, UiNode, ViewState};
    use semio_framework_core::kernel::{AppEvent, HostEffect};
    use semio_framework_core::ActionArgDef;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use vcs::{Backbone, BackboneMessage, MemoryBackbone, Operation, OperationDiff};

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct TestProjection {
        count: i32,
        label: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct TestDiff {
        count: Option<i32>,
        label: Option<String>,
    }

    impl OperationDiff<TestProjection> for TestDiff {
        fn apply(&self, projection: &TestProjection) -> TestProjection {
            TestProjection {
                count: self.count.unwrap_or(projection.count),
                label: self.label.clone().unwrap_or_else(|| projection.label.clone()),
            }
        }

        fn absorb(&mut self, other: Self) {
            if other.count.is_some() {
                self.count = other.count;
            }
            if other.label.is_some() {
                self.label = other.label;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op", rename_all = "camelCase")]
    enum TestOp {
        SetCount { value: i32 },
        SetLabel { value: String },
    }

    impl Operation<TestProjection> for TestOp {
        type Diff = TestDiff;

        fn diff(&self, _projection: &TestProjection) -> TestDiff {
            match self {
                TestOp::SetCount { value } => TestDiff { count: Some(*value), label: None },
                TestOp::SetLabel { value } => TestDiff { count: None, label: Some(value.clone()) },
            }
        }

        fn backwards(&self, projection: &TestProjection) -> Vec<Self> {
            match self {
                TestOp::SetCount { .. } => vec![TestOp::SetCount { value: projection.count }],
                TestOp::SetLabel { .. } => vec![TestOp::SetLabel { value: projection.label.clone() }],
            }
        }
    }

    /// 🧪 App under test: `selected` is ephemeral view state living in the app struct (never in the
    /// document), demonstrating that view actions mutate the struct and emit no operations.
    #[derive(Default)]
    struct TestApp {
        selected: Option<String>,
    }

    impl DocumentApp for TestApp {
        type Projection = TestProjection;
        type Op = TestOp;

        fn app_id(&self) -> &str {
            "synthetic-play"
        }

        fn document_schema(&self) -> &str {
            "semio.test/v1"
        }

        fn initial_projection(&self) -> TestProjection {
            TestProjection::default()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, TestProjection>,
            view_state: &ViewState,
        ) -> ActionEmit<TestOp> {
            let label_arg = || {
                args.and_then(|value| value.get("value"))
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string()
            };
            match action {
                "increment" => ActionEmit {
                    ops: vec![TestOp::SetCount { value: doc.projection.count + 1 }],
                    description: Some("increment".into()),
                    ..Default::default()
                },
                "setLabel" | "setLabelRequired" | "setLabelDefault" => {
                    ActionEmit {
                        ops: vec![TestOp::SetLabel { value: label_arg() }],
                        coalesce_key: Some("label".into()),
                        ..Default::default()
                    }
                }
                "amendLabel" => ActionEmit::amend(vec![TestOp::SetLabel { value: label_arg() }], "label"),
                "commitLabel" => ActionEmit::commit(vec![TestOp::SetLabel { value: label_arg() }], "commit label"),
                // 🧪 A deliberately mis-behaving View action: emits ops it must not — the registry-backed
                // kind-discipline check rejects it.
                "badView" => ActionEmit::ops(vec![TestOp::SetCount { value: 99 }]),
                // 🧪 Reads the host-owned active utility from view state (never the document) and echoes it
                // as an event — proving `setActiveUtility` forwards `view_state.active_utility_id` and emits no ops.
                "setActiveUtility" => ActionEmit::event(AppEvent {
                    kind: "active-utility".into(),
                    payload: json!({ "utilityId": view_state.active_utility_id.clone().unwrap_or_default() }),
                }),
                "select" => {
                    self.selected = args
                        .and_then(|value| value.get("id"))
                        .and_then(Value::as_str)
                        .map(str::to_string);
                    ActionEmit::default()
                }
                "navigate" => ActionEmit::effect(HostEffect::Navigate { uri: "semio://home".into() }),
                _ => ActionEmit::default(),
            }
        }

        fn handle_command(
            &mut self,
            command: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, TestProjection>,
            _view_state: &ViewState,
        ) -> ActionEmit<TestOp> {
            match command {
                "incrementViaCommand" => ActionEmit {
                    ops: vec![TestOp::SetCount { value: doc.projection.count + 1 }],
                    description: Some("increment via command".into()),
                    ..Default::default()
                },
                "setLabelViaCommand" => {
                    let value = args.and_then(|value| value.get("value")).and_then(Value::as_str).unwrap_or_default().to_string();
                    ActionEmit::ops(vec![TestOp::SetLabel { value }])
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, _body_key: &str, doc: &DocumentView<'_, TestProjection>, _view_state: &ViewState) -> UiNode {
            ui_text(format!("count={}", doc.projection.count))
        }
    }

    fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1 }
    }

    fn synthetic_play_app() -> App {
        App::from_builder(
            App::builder("synthetic-play", "Synthetic")
                .document(["state"])
                .mode("edit", "Edit")
                .window_kind("main", "Main", "synthetic.main", SurfaceKind::Canvas2d),
        )
    }

    /// 🧪 A registry-backed app declaring the contract-enforcement fixtures: an operation with a
    /// required arg, one with a defaulted optional arg, a mis-behaving View action, and a utility (which
    /// auto-injects the `setActiveUtility` View action).
    fn contract_registry() -> AppActionRegistry {
        let app = App::from_builder(
            App::builder("synthetic-play", "Synthetic")
                .document(["state"])
                .mode("edit", "Edit")
                .window_kind("main", "Main", "synthetic.main", SurfaceKind::Canvas2d)
                .operation("setLabelRequired", "Set Label")
                .action_args("setLabelRequired", vec![ActionArgDef::text("value", "Value").required()])
                .operation("setLabelDefault", "Set Label Default")
                .action_args("setLabelDefault", vec![ActionArgDef::text("value", "Value").default_value("seed")])
                .view_action("badView", "Bad View")
                .utility_simple("brush", "Brush", "icon.brush")
                .app_command("incrementViaCommand", "Increment", "counter")
                .app_command("setLabelViaCommand", "Set Label", "counter")
                .command_args("setLabelViaCommand", vec![ActionArgDef::text("value", "Value").required()]),
        );
        AppActionRegistry::from_definition(&app.definition)
    }

    fn contract_app_under_test() -> VcsDocumentApp<TestApp> {
        VcsDocumentApp::with_registry(TestApp::default(), contract_registry())
    }

    fn synthetic_setup() {}

    crate::semio_plugin! {
        id: "synthetic", label: "Synthetic", version: "0.0.1",
        setup: synthetic_setup,
        apps: [ synthetic_play_app => TestApp ],
    }

    #[test]
    fn semio_plugin_macro_builds_bundle_from_declarative_spec() {
        let bundle = __semio_plugin_bundle();
        assert_eq!(bundle.manifest.plugin_id, "synthetic");
        assert_eq!(bundle.manifest.label, "Synthetic");
        assert_eq!(bundle.manifest.version, "0.0.1");
        assert!(bundle.manifest.apps.iter().any(|app| app.id == "synthetic-play"));
    }

    #[test]
    fn semio_plugin_macro_wires_app_factory_for_create_app() {
        let bundle = __semio_plugin_bundle();
        let app = bundle.create_app("synthetic-play").expect("registered app");
        assert_eq!(app.app_id(), "synthetic-play");
        assert!(bundle.create_app("unknown-app").is_none());
    }

    #[test]
    fn operation_action_emits_kernel_op_with_true_inverse() {
        let mut app = VcsDocumentApp::new(TestApp::default());
        let result = app.handle_action("increment", None, &ViewState::default(), &meta()).expect("increment");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(result.operations[0].diff.payload, json!({ "op": "setCount", "value": 1 }));
        assert_eq!(
            result.operations[0].inverse.inverse_diff.payload,
            json!({ "backwards": [{ "op": "setCount", "value": 0 }] })
        );
        assert_eq!(result.inverse_group.operations.len(), 1);
        assert_eq!(app.test_projection().count, 1);
    }

    #[test]
    fn view_action_emits_no_operations() {
        let mut app = VcsDocumentApp::new(TestApp::default());
        let result = app
            .handle_action("select", Some(&json!({ "id": "node-1" })), &ViewState::default(), &meta())
            .expect("select");
        assert!(result.operations.is_empty());
        assert!(result.requested_effects.is_empty());
        // A view action never advances the document.
        assert_eq!(app.test_projection(), TestProjection::default());
    }

    #[test]
    fn shell_action_emits_host_effect_without_operations() {
        let mut app = VcsDocumentApp::new(TestApp::default());
        let result = app.handle_action("navigate", None, &ViewState::default(), &meta()).expect("navigate");
        assert!(result.operations.is_empty());
        assert_eq!(result.requested_effects, vec![HostEffect::Navigate { uri: "semio://home".into() }]);
    }

    #[test]
    fn coalesced_operations_amend_a_single_edit() {
        let mut app = VcsDocumentApp::new(TestApp::default());
        for value in ["a", "ab", "abc"] {
            app.handle_action("setLabel", Some(&json!({ "value": value })), &ViewState::default(), &meta())
                .expect("setLabel");
        }
        assert_eq!(app.test_projection().label, "abc");
        // One undo reverts the whole coalesced gesture back to the empty label.
        app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert_eq!(app.test_projection().label, "");
    }

    #[test]
    fn history_actions_round_trip_through_the_store() {
        let mut app = VcsDocumentApp::new(TestApp::default());
        app.handle_action("increment", None, &ViewState::default(), &meta()).expect("inc1");
        app.handle_action("increment", None, &ViewState::default(), &meta()).expect("inc2");
        assert_eq!(app.test_projection().count, 2);

        let undo = app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert!(undo.operations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.test_projection().count, 1);

        app.handle_action("redo", None, &ViewState::default(), &meta()).expect("redo");
        assert_eq!(app.test_projection().count, 2);

        let checkpoint = app.handle_action("commitCheckpoint", None, &ViewState::default(), &meta()).expect("checkpoint");
        assert!(checkpoint.operations.is_empty());
        assert!(checkpoint.events.iter().any(|event| event.kind == "history-changed"));
    }

    #[test]
    fn undo_on_empty_history_is_a_benign_no_op() {
        let mut app = VcsDocumentApp::new(TestApp::default());
        let result = app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert!(result.operations.is_empty());
        assert!(result.events.is_empty());
    }

    #[test]
    fn document_round_trips_through_serialization() {
        let mut app = VcsDocumentApp::new(TestApp::default());
        app.handle_action("increment", None, &ViewState::default(), &meta()).expect("inc");
        app.handle_action("setLabel", Some(&json!({ "value": "hi" })), &ViewState::default(), &meta()).expect("label");
        let json = app.document_json().expect("document json");

        let mut restored = VcsDocumentApp::new(TestApp::default());
        restored.load_document(&json).expect("load document");
        assert_eq!(restored.test_projection(), TestProjection { count: 1, label: "hi".into() });
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        let mut sender = VcsDocumentApp::new(TestApp::default());
        let (near, mut far) = MemoryBackbone::pair("mem://doc", "mem://doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        sender.handle_action("increment", None, &ViewState::default(), &meta()).expect("increment");

        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Ops { envelopes: ops } = message {
                envelopes.extend(ops);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied op to flow onto the channel");
        let operations_json = serde_json::to_string(&envelopes).expect("serialize envelopes");

        let mut receiver = VcsDocumentApp::new(TestApp::default());
        receiver.ingest_operations(&operations_json).expect("ingest once");
        receiver.ingest_operations(&operations_json).expect("ingest twice");
        assert_eq!(receiver.test_projection().count, 1, "feeding the same op twice must not double-apply");
    }

    #[test]
    fn required_arg_missing_is_rejected() {
        let mut app = contract_app_under_test();
        let error = app
            .handle_action("setLabelRequired", None, &ViewState::default(), &meta())
            .expect_err("missing required arg must be a hard error");
        assert!(error.contains("missing required args"), "unexpected error: {error}");
        assert!(error.contains("value"));
        assert_eq!(app.test_projection(), TestProjection::default(), "nothing dispatched on rejection");
    }

    #[test]
    fn required_arg_present_is_accepted() {
        let mut app = contract_app_under_test();
        app.handle_action("setLabelRequired", Some(&json!({ "value": "hi" })), &ViewState::default(), &meta())
            .expect("required arg provided");
        assert_eq!(app.test_projection().label, "hi");
    }

    #[test]
    fn default_arg_is_materialized_when_absent() {
        let mut app = contract_app_under_test();
        app.handle_action("setLabelDefault", None, &ViewState::default(), &meta())
            .expect("default materialized");
        assert_eq!(app.test_projection().label, "seed", "declared default fills the missing arg");
    }

    #[test]
    fn view_action_emitting_ops_is_rejected() {
        let mut app = contract_app_under_test();
        let error = app
            .handle_action("badView", None, &ViewState::default(), &meta())
            .expect_err("a View action emitting ops must be rejected");
        assert!(error.contains("must not emit operations"), "unexpected error: {error}");
        assert_eq!(app.test_projection(), TestProjection::default());
    }

    #[test]
    fn set_active_utility_forwards_view_state_active_utility_and_emits_no_ops() {
        let mut app = contract_app_under_test();
        let view_state = ViewState { active_utility_id: Some("brush".into()), ..ViewState::default() };
        let result = app
            .handle_action("setActiveUtility", Some(&json!({ "utilityId": "brush" })), &view_state, &meta())
            .expect("setActiveUtility is a valid View action");
        assert!(result.operations.is_empty(), "utility switching must not create history");
        let event = result.events.iter().find(|event| event.kind == "active-utility").expect("echoed active utility");
        assert_eq!(event.payload, json!({ "utilityId": "brush" }));
    }

    #[test]
    fn action_emit_amend_coalesces_while_commit_does_not() {
        let mut app = contract_app_under_test();
        for value in ["a", "ab", "abc"] {
            app.handle_action("amendLabel", Some(&json!({ "value": value })), &ViewState::default(), &meta())
                .expect("amendLabel");
        }
        assert_eq!(app.test_projection().label, "abc");
        // One undo reverts the whole coalesced amend gesture.
        app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo amend");
        assert_eq!(app.test_projection().label, "");

        for value in ["x", "xy"] {
            app.handle_action("commitLabel", Some(&json!({ "value": value })), &ViewState::default(), &meta())
                .expect("commitLabel");
        }
        assert_eq!(app.test_projection().label, "xy");
        // Each commit is its own edit: one undo only reverts the last commit.
        app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo commit");
        assert_eq!(app.test_projection().label, "x");
    }

    #[test]
    fn operation_command_emits_kernel_op_with_true_inverse() {
        let mut app = contract_app_under_test();
        let result = app
            .handle_command("incrementViaCommand", None, &ViewState::default(), &meta())
            .expect("incrementViaCommand");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(result.operations[0].diff.payload, json!({ "op": "setCount", "value": 1 }));
        assert_eq!(
            result.operations[0].inverse.inverse_diff.payload,
            json!({ "backwards": [{ "op": "setCount", "value": 0 }] })
        );
        assert_eq!(app.test_projection().count, 1);
    }

    #[test]
    fn unknown_command_is_a_hard_error() {
        let mut app = contract_app_under_test();
        let error = app
            .handle_command("nope", None, &ViewState::default(), &meta())
            .expect_err("an undeclared command id must be rejected");
        assert!(error.contains("unknown command"), "unexpected error: {error}");
    }

    #[test]
    fn command_required_arg_is_enforced_and_materialized_like_an_action() {
        let mut app = contract_app_under_test();
        let error = app
            .handle_command("setLabelViaCommand", None, &ViewState::default(), &meta())
            .expect_err("missing required arg must be a hard error");
        assert!(error.contains("missing required args"), "unexpected error: {error}");

        app.handle_command("setLabelViaCommand", Some(&json!({ "value": "hi" })), &ViewState::default(), &meta())
            .expect("required arg provided");
        assert_eq!(app.test_projection().label, "hi");
    }

    #[test]
    fn command_op_records_history_exactly_like_an_operation_action() {
        let mut app = contract_app_under_test();
        app.handle_command("incrementViaCommand", None, &ViewState::default(), &meta()).expect("inc");
        app.handle_command("incrementViaCommand", None, &ViewState::default(), &meta()).expect("inc");
        assert_eq!(app.test_projection().count, 2);
        app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert_eq!(app.test_projection().count, 1);
    }

    #[test]
    fn registry_less_construction_skips_enforcement() {
        // The empty-registry path (VcsDocumentApp::new) passes actions through unchecked.
        let mut app = VcsDocumentApp::new(TestApp::default());
        app.handle_action("badView", None, &ViewState::default(), &meta())
            .expect("no registry ⇒ kind discipline skipped");
    }
}
// #endregion plugin_runtime
}

pub mod world3d_host {
// #region world3d_host
//! 🌐 Shared world-3d scene payload builders for plugin apps.

use semio_framework_core::{mesh_from_kind, mesh_to_glb, mesh_to_obj, MeshData};
use ui_wgpu::{ActionDescriptor, WindowMeasure, World3dScene, world3d_camera_json, world3d_default_selection_json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🌞 WorldSunConfig
/** 🌞 Plugin-owned directional-light state for a `world-3d` scene; off by default so meshes render flat until a dev opts in via the window-options Sun toggle. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WorldSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

impl Default for WorldSunConfig {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
    }
}

/** 🌞 Builds the `environment_json` payload consumed by `world-3d-host.tsx`'s `WorldEnvironmentRecord.sun`. */
pub fn world3d_environment_json(sun: &WorldSunConfig) -> String {
    json!({ "sun": sun }).to_string()
}

/** 🌞 Shared "Sun" window-options group (enable toggle + azimuth/elevation/intensity sliders), see `lowpoly_window_measures`'s "Show Edges" toggle for the sibling pattern. */
pub fn world3d_sun_measures(id_prefix: &str, sun: &WorldSunConfig, action: impl Fn(&str, Option<Value>) -> ActionDescriptor) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{id_prefix}-measure-sun"),
        label: "Sun".into(),
        default_open: Some(false),
        active_utility_id: None,
        children: vec![
            WindowMeasure::Toggle {
                id: format!("{id_prefix}-measure-sun-enabled"),
                icon_id: "sun".into(),
                label: Some("Enabled".into()),
                pressed: sun.enabled,
                text: None,
                on_change: action("toggleSun", None),
            },
            WindowMeasure::Slider {
                id: format!("{id_prefix}-measure-sun-azimuth"),
                label: Some("Azimuth".into()),
                value: sun.azimuth,
                min: 0.0,
                max: 360.0,
                step: Some(1.0),
                on_change: action("setSunAzimuth", None),
            },
            WindowMeasure::Slider {
                id: format!("{id_prefix}-measure-sun-elevation"),
                label: Some("Elevation".into()),
                value: sun.elevation,
                min: 0.0,
                max: 90.0,
                step: Some(1.0),
                on_change: action("setSunElevation", None),
            },
            WindowMeasure::Slider {
                id: format!("{id_prefix}-measure-sun-intensity"),
                label: Some("Intensity".into()),
                value: sun.intensity,
                min: 0.0,
                max: 4.0,
                step: Some(0.05),
                on_change: action("setSunIntensity", None),
            },
        ],
    }
}

/** 🌞 Applies a sun-related action id to `sun`, returning whether it was handled — mirrors `lowpoly`'s `"toggleShowEdges"` action-handler shape. */
pub fn apply_world3d_sun_action(sun: &mut WorldSunConfig, action_id: &str, args: Option<&Value>) -> bool {
    match action_id {
        "toggleSun" => {
            sun.enabled = !sun.enabled;
            true
        }
        "setSunAzimuth" => {
            if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
                sun.azimuth = value;
            }
            true
        }
        "setSunElevation" => {
            if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
                sun.elevation = value;
            }
            true
        }
        "setSunIntensity" => {
            if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
                sun.intensity = value;
            }
            true
        }
        _ => false,
    }
}
//#endregion 🌞 WorldSunConfig

pub fn mesh_kind_from_json(mesh_json: &str) -> String {
    serde_json::from_str::<Value>(mesh_json)
        .ok()
        .and_then(|value| value.get("kind").and_then(|v| v.as_str()).map(str::to_string))
        .unwrap_or_else(|| "box".into())
}

pub fn world3d_meshes_json_from_kinds(kinds: &[String]) -> String {
    let meshes: Vec<Value> = kinds
        .iter()
        .map(|kind| {
            let data = mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world3d_mesh_id_from_url(url: &str) -> String {
    let slug = url
        .trim_start_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(url)
        .trim_end_matches(".glb")
        .trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

pub fn world3d_meshes_json_from_urls(urls: &[String]) -> String {
    let meshes: Vec<Value> = urls
        .iter()
        .map(|url| {
            json!({
                "id": world3d_mesh_id_from_url(url),
                "url": url,
            })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world3d_meshes_json_from_kinds_and_urls(kinds: &[String], urls: &[String]) -> String {
    let mut meshes: Vec<Value> = kinds
        .iter()
        .map(|kind| {
            let data = mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    for url in urls {
        let id = world3d_mesh_id_from_url(url);
        if meshes.iter().any(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(id.as_str())) {
            continue;
        }
        meshes.push(json!({ "id": id, "url": url }));
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world3d_selection_json(method: &str, ids: &[String], hovered_id: Option<&str>) -> String {
    world3d_selection_json_with_granularity(method, ids, hovered_id, None)
}

pub fn world3d_selection_json_with_granularity(
    method: &str,
    ids: &[String],
    hovered_id: Option<&str>,
    granularity: Option<&str>,
) -> String {
    let mut value = json!({
        "method": method,
        "mode": "replace",
        "ids": ids,
        "hoveredId": hovered_id,
    });
    if let Some(entry) = granularity {
        if let Some(object) = value.as_object_mut() {
            object.insert("granularity".into(), json!(entry));
        }
    }
    value.to_string()
}

pub fn world3d_scene(
    camera_json: String,
    meshes_json: String,
    instances_json: String,
    selection_json: String,
    sun: &WorldSunConfig,
) -> World3dScene {
    world3d_scene_extended(
        camera_json,
        meshes_json,
        instances_json,
        selection_json,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(world3d_environment_json(sun)),
    )
}

pub fn world3d_scene_extended(
    camera_json: String,
    meshes_json: String,
    instances_json: String,
    selection_json: String,
    vortices_json: Option<String>,
    attractions_json: Option<String>,
    target_volumes_json: Option<String>,
    references_json: Option<String>,
    brush_preview_json: Option<String>,
    interaction_json: Option<String>,
    engagement_preview_json: Option<String>,
    lod_json: Option<String>,
    chunking_json: Option<String>,
    context_menu_json: Option<String>,
    environment_json: Option<String>,
) -> World3dScene {
    World3dScene {
        camera_json,
        meshes_json,
        instances_json,
        selection_json,
        vortices_json,
        attractions_json,
        target_volumes_json,
        references_json,
        brush_preview_json,
        interaction_json,
        engagement_preview_json,
        lod_json,
        chunking_json,
        context_menu_json,
        environment_json,
        frame_json: None,
        fit_json: None,
        terrain_json: None,
    }
}

pub fn world3d_default_camera() -> String {
    world3d_camera_json([4.0, -4.0, 3.0], [0.0, 0.0, 0.0], 45.0)
}

pub fn export_mesh_obj(mesh: &MeshData, name: &str) -> (String, String) {
    (mesh_to_obj(mesh, name), "text/plain".into())
}

pub fn export_mesh_glb_bytes(mesh: &MeshData) -> (Vec<u8>, String) {
    (mesh_to_glb(mesh), "model/gltf-binary".into())
}

pub fn merge_world_selection_ids(existing: &[String], incoming: &[String], merge: &str) -> Vec<String> {
    match merge {
        "add" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(id.clone());
                }
            }
            merged
        }
        "toggle" | "invertive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id.clone());
                }
            }
            merged
        }
        "remove" | "subtractive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                merged.retain(|entry| entry != id);
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

pub fn default_world3d_selection() -> String {
    world3d_default_selection_json()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_world_selection_ids_supports_add_toggle_invertive_and_remove() {
        assert_eq!(
            merge_world_selection_ids(&["a".into()], &["b".into()], "add"),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(
            merge_world_selection_ids(&["a".into(), "b".into()], &["b".into(), "c".into()], "toggle"),
            vec!["a".to_string(), "c".to_string()]
        );
        assert_eq!(
            merge_world_selection_ids(&["a".into(), "b".into()], &["b".into()], "invertive"),
            vec!["a".to_string()]
        );
        assert_eq!(
            merge_world_selection_ids(&["a".into()], &["b".into()], "replace"),
            vec!["b".to_string()]
        );
        assert_eq!(
            merge_world_selection_ids(&["a".into(), "b".into(), "c".into()], &["b".into()], "remove"),
            vec!["a".to_string(), "c".to_string()]
        );
        assert_eq!(
            merge_world_selection_ids(&["a".into(), "b".into(), "c".into()], &["b".into()], "subtractive"),
            vec!["a".to_string(), "c".to_string()]
        );
    }
}
// #endregion world3d_host
}

pub mod host_port {
// #region host_port
//! 🗄️ Host-capability access for WASI component builds — the backbone duplex channel and wall-clock time.

/** @emoji 📤 Sends a backbone message through the component host; errs when no host is linked. */
pub fn host_backbone_send(uri: &str, message_json: &str) -> Result<(), String> {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    {
        return crate::component::host_backbone_send(uri, message_json);
    }
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {
        let _ = message_json;
        Err(format!("host backbone unavailable: {uri}"))
    }
}

/** @emoji 📥 Polls queued backbone messages through the component host; errs when no host is linked. */
pub fn host_backbone_poll(uri: &str) -> Result<Vec<String>, String> {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    {
        return crate::component::host_backbone_poll(uri);
    }
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    Err(format!("host backbone unavailable: {uri}"))
}

/** @emoji 🩺 Queries the host for the sync status of a backbone uri; errs when no host is linked. */
pub fn host_backbone_status(uri: &str) -> Result<String, String> {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    {
        return crate::component::host_backbone_status(uri);
    }
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    Err(format!("host backbone unavailable: {uri}"))
}

/** @emoji ⏱️ Wall-clock milliseconds from the component host, falling back to system time. */
pub fn host_now_ms() -> f64 {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    {
        return crate::component::host_now_ms() as f64;
    }
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as f64)
        .unwrap_or(0.0)
}

/** @emoji 🔌 vcs backbone channel backed by the component host's duplex capability. */
pub struct HostBackboneChannel;

impl vcs::BackboneChannelPort for HostBackboneChannel {
    fn send(&self, uri: &str, message_json: &str) -> Result<(), vcs::VcsError> {
        host_backbone_send(uri, message_json).map_err(vcs::VcsError::Backbone)
    }

    fn poll(&self, uri: &str) -> Result<Vec<String>, vcs::VcsError> {
        host_backbone_poll(uri).map_err(vcs::VcsError::Backbone)
    }
}

/** @emoji 🧷 Installs the component host as the vcs backbone channel so the plugin's document store
    can synchronize across the wasm sandbox boundary. */
pub fn register_host_backbone_channel() {
    vcs::set_host_backbone_channel(std::sync::Arc::new(HostBackboneChannel));
}
// #endregion host_port
}

pub mod engagement {
// #region engagement
//! 🎛️ Parses engagement command-line drafts submitted by the React shell, which PascalCases every
//! draft and strips separators (`ui/js/react/index.tsx` `normalizeEngagementActionText`) before
//! dispatching — so `"fill 20"` arrives as `"Fill20"`, not `"fill 20"`.

/** @emoji ✂️ Strips a leading `command` token from `raw`, ignoring case and separators on both
    sides, and returns the trimmed remainder (e.g. `strip_engagement_prefix("Fill20", "fill")`
    and `strip_engagement_prefix("fill 20", "fill")` both yield `Some("20")`). Decimal points
    inside numeric remainders are preserved. Returns `None` when `raw` doesn't start with `command`. */
pub fn strip_engagement_prefix<'a>(raw: &'a str, command: &str) -> Option<&'a str> {
    let raw_bytes = raw.as_bytes();
    let mut raw_index = 0usize;
    let mut command_chars = command.chars().filter(|ch| ch.is_alphanumeric());
    while let Some(expected) = command_chars.next() {
        while raw_index < raw_bytes.len() {
            let ch = raw[raw_index..].chars().next().unwrap();
            if ch.is_alphanumeric() {
                break;
            }
            raw_index += ch.len_utf8();
        }
        let Some(actual) = raw[raw_index..].chars().next() else {
            return None;
        };
        if !actual.eq_ignore_ascii_case(&expected) {
            return None;
        }
        raw_index += actual.len_utf8();
    }
    let mut remainder_start = raw_index;
    while remainder_start < raw_bytes.len() {
        let ch = raw[remainder_start..].chars().next().unwrap();
        if ch.is_alphanumeric() || ch == '.' {
            break;
        }
        remainder_start += ch.len_utf8();
    }
    Some(raw[remainder_start..].trim())
}

/** @emoji 🔤 True when `raw` matches `command` in full, ignoring case and separators (e.g.
    `engagement_token_matches("LineNumbers", "line numbers")` is `true`). */
pub fn engagement_token_matches(raw: &str, command: &str) -> bool {
    strip_engagement_prefix(raw, command).is_some_and(str::is_empty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_engagement_prefix_accepts_normalized_and_raw_forms() {
        assert_eq!(strip_engagement_prefix("Fill20", "fill"), Some("20"));
        assert_eq!(strip_engagement_prefix("fill 20", "fill"), Some("20"));
        assert_eq!(strip_engagement_prefix("fill  20", "fill"), Some("20"));
        assert_eq!(strip_engagement_prefix("Fill", "fill"), Some(""));
        assert_eq!(strip_engagement_prefix("FILL20", "fill"), Some("20"));
    }

    #[test]
    fn strip_engagement_prefix_preserves_decimal_points() {
        assert_eq!(strip_engagement_prefix("SetHeight3.5", "set height"), Some("3.5"));
        assert_eq!(strip_engagement_prefix("set height 3.5", "set height"), Some("3.5"));
    }

    #[test]
    fn strip_engagement_prefix_rejects_non_matching_commands() {
        assert_eq!(strip_engagement_prefix("Brush", "fill"), None);
        assert_eq!(strip_engagement_prefix("Filled", "fill"), Some("ed"));
    }

    #[test]
    fn engagement_token_matches_full_token_only() {
        assert!(engagement_token_matches("LineNumbers", "line numbers"));
        assert!(engagement_token_matches("linenumbers", "line numbers"));
        assert!(!engagement_token_matches("LineNumbers2", "line numbers"));
        assert!(!engagement_token_matches("Line", "line numbers"));
    }
}
// #endregion engagement
}

pub use app::{
    ActionEmit, ActionMeta, App, AppBuilder, AppInstance, DocumentApp, DocumentView, HistoryView,
    KeybindingSpec, MediaClass, MediaType, ModeSpec, OsMediaCapability, PanelTabSpec, PanelTreeBuilder, Plugin, PluginApp,
    PluginBundle, ResourceKindSpec, VcsDocumentApp, WindowKindSpec,
};
pub use semio_framework_core::{MediaForm, MediaPortDirection, MediaPortSpec};
pub use app::{
    is_de_locale, localized_label_map, resolve_labels, selection_ids, tree_item, tree_item_desc,
    tree_item_with_action, tree_item_with_action_draggable, AppLabelsOverlayExt, LocaleLabels,
};
pub use app::testkit;
pub use semio_framework_core::AppLabelsOverlay;
pub use engagement::{engagement_token_matches, strip_engagement_prefix};
pub use host_port::{
    host_backbone_poll, host_backbone_send, host_backbone_status, host_now_ms,
    register_host_backbone_channel, HostBackboneChannel,
};
pub use plugin_runtime::{
    install_plugin_bundle, plugin_attach_backbone, plugin_detach_backbone, plugin_document,
    plugin_ingest_operations, plugin_load_document,
};
pub use world3d_host::{
    apply_world3d_sun_action, default_world3d_selection, export_mesh_glb_bytes, export_mesh_obj,
    merge_world_selection_ids, mesh_kind_from_json, world3d_default_camera,
    world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds,
    world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_scene,
    world3d_scene_extended, world3d_selection_json, world3d_sun_measures, WorldSunConfig,
};
pub use semio_framework_core::*;
// 🧩 Declarative component model (UiNode, layouts, utilities) — moved into ui_wgpu; re-exported here so
// apps keep the flat `semio_framework_plugin::*` import surface with zero Cargo.toml churn.
pub use ui_wgpu::*;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle($bundle);
    };
}
