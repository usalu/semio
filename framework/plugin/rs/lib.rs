//! 🔌 Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM components.

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub mod component {
    //! 🧩 WASI P2 component exports for the plugin world contract.

    use crate::plugin_runtime::{
        ensure_plugin_initialized, plugin_attach_backbone, plugin_create_app,
        plugin_detach_backbone, plugin_document, plugin_handle_action, plugin_ingest_operations,
        plugin_load_document, plugin_manifest, plugin_refresh_ui, plugin_render_with_document,
    };
    use wit_bindgen::generate;

    generate!({
        world: "plugin-world",
        path: "../../wit",
    });

    use exports::semio::framework::plugin::Guest;
    use semio::framework::types::{
        ActionContextJson, ActionInvocationJson, ActionResponseJson, MigrateDocumentInput,
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
            context: ActionContextJson,
        ) -> Result<ActionResponseJson, PluginError> {
            ensure_plugin_initialized();
            let result = plugin_handle_action(instance_id, &action.json, &context.json)
                .map_err(PluginError::Message)?;
            Ok(ActionResponseJson {
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
        ActorId, AppEvent, CapabilityRequirement, ActionInvocationId, ActionResult, HostEffect, HybridLogicalTimestamp,
        InverseOperation, KernelOperation, DocumentDiff, DocumentHandle, DocumentVersion, OpEnvelope, OperationId, Rights,
        ResourceKind, SchemaId, Scope, UndoGroup, UndoPolicy,
    },
    set_active_tool_action_definition, ActionArgDef, ActionRef, AppDefinition, AppLabelsOverlay, ActionDefinition,
    ActionKind, Contribution, ExampleDefinition, Keybinding, ModeDefinition, Modes, PanelGroup, PanelTabDefinition,
    PanelTabKind, PluginManifest, ProgramDefinition, ToolDefinition, ToolRef, ViewState, WindowKindDefinition,
    WindowKinds, SET_ACTIVE_TOOL_ACTION_ID,
};
use ui_wgpu::{
    collect_window_kind_ids_from_layout, ActionDescriptor, NamedLayout, UiNode, WindowEngagement,
    WindowEngagementSlot, WindowLayout, WindowMeasure, WindowOptions, SurfaceKind,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use vcs::{
    build_history_columns, create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope,
    DocumentVcsStore, HistoryColumn, Operation,
};

pub struct ModeSpec {
    pub id: String,
    pub label: String,
    pub tools: Vec<ToolRef>,
    pub layout_id: Option<String>,
    pub actions: Vec<ActionRef>,
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
    pub tools: Vec<ToolRef>,
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

pub struct KeybindingSpec {
    pub keys: String,
    pub controller_id: String,
    pub action: String,
}

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
    tools: Vec<ToolDefinition>,
    named_layouts: Vec<NamedLayout>,
    default_layout: Option<WindowLayout>,
    terminologies: Vec<String>,
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
            tools: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
        }
    }

    /// 🗣️ Declares an alternative terminology id this app supports beyond the implicit "native" default.
    pub fn terminology(mut self, id: impl Into<String>) -> Self {
        self.terminologies.push(id.into());
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
            tools: Vec::new(),
            layout_id: None,
            actions: Vec::new(),
        });
        self
    }

    /// 📇 Scopes actions to a mode — references ids declared via `.operation()/.view_action()/.shell_action()`.
    pub fn mode_actions(mut self, mode_id: impl AsRef<str>, action_ids: Vec<ActionRef>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.actions = action_ids;
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

    /// 🧰 Scopes tools to a mode — references ids declared via `.tool()`/`.tool_simple()`.
    pub fn mode_tools(mut self, mode_id: impl AsRef<str>, tool_ids: Vec<ToolRef>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.tools = tool_ids;
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
            tools: Vec::new(),
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
            tools: Vec::new(),
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

    /// 🧰 Scopes tools to a window kind — references ids declared via `.tool()`/`.tool_simple()`. Mirrors
    /// `window_kind_actions`: the referenced tool ids are validated to resolve in `build_definition`.
    pub fn window_kind_tools(mut self, window_kind_id: impl AsRef<str>, tool_ids: Vec<ToolRef>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.tools = tool_ids;
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

    /// @emoji 👁️ Declares an ephemeral view action (camera, selection, hover, active tool) — not recorded in history.
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

    /// @emoji 🧰 Declares an interactive tool this app exposes (referenced by `window_kind_tools`/`mode_tools`).
    pub fn tool(mut self, tool: ToolDefinition) -> Self {
        self.tools.push(tool);
        self
    }

    /// @emoji 🧰 Declares a tool with default settings (no group/keys/cursor/category, gates actions while active).
    pub fn tool_simple(self, id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<String>) -> Self {
        self.tool(ToolDefinition::new(id, label, icon_id))
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
            let mut arg_ids = HashSet::new();
            for arg in &action.args {
                assert!(
                    arg_ids.insert(arg.id.clone()),
                    "app {} action {} declares duplicate arg id {}",
                    self.id,
                    action.id,
                    arg.id
                );
                if let semio_framework_core::ActionArgControl::Select { options } = &arg.control {
                    assert!(
                        !options.is_empty(),
                        "app {} action {} arg {} is a Select with no options",
                        self.id,
                        action.id,
                        arg.id
                    );
                }
            }
        }
        let mut declared_tool_ids = HashSet::new();
        for tool in &self.tools {
            assert!(!tool.id.trim().is_empty(), "app {} tool id must be non-empty", self.id);
            assert!(
                declared_tool_ids.insert(tool.id.clone()),
                "app {} duplicate tool id {}",
                self.id,
                tool.id
            );
        }
        let app_declared_actions = !self.actions.is_empty();
        let mut actions = self.actions;
        for history_action in history_action_definitions() {
            if declared_action_ids.insert(history_action.id.clone()) {
                actions.push(history_action);
            }
        }
        if !self.tools.is_empty() && declared_action_ids.insert(SET_ACTIVE_TOOL_ACTION_ID.to_string()) {
            actions.push(set_active_tool_action_definition());
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
        for tool in &self.tools {
            if let Some(keys) = &tool.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding {
                        keys: keys.clone(),
                        action: ActionDescriptor {
                            controller_id: self.controller_id.clone(),
                            action: SET_ACTIVE_TOOL_ACTION_ID.to_string(),
                            args: Some(serde_json::json!({ "toolId": tool.id })),
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
            for tool_ref in &window.tools {
                assert!(
                    declared_tool_ids.contains(tool_ref.as_str()),
                    "app {} window kind {} references undeclared tool {}",
                    self.id,
                    window.id,
                    tool_ref.as_str()
                );
            }
        }
        for mode in &self.modes {
            for action_ref in &mode.actions {
                assert!(
                    declared_action_ids.contains(action_ref.as_str()),
                    "app {} mode {} references undeclared action {}",
                    self.id,
                    mode.id,
                    action_ref.as_str()
                );
            }
            for tool_ref in &mode.tools {
                assert!(
                    declared_tool_ids.contains(tool_ref.as_str()),
                    "app {} mode {} references undeclared tool {}",
                    self.id,
                    mode.id,
                    tool_ref.as_str()
                );
            }
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
                        tools: mode.tools,
                        layout_id: mode.layout_id,
                        actions: mode.actions,
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
                        tools: window.tools,
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
            tools: self.tools,
            named_layouts: self.named_layouts,
            default_layout: self.default_layout,
            terminologies: self.terminologies,
        }
    }
}

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
                .mode_tools("edit", vec![])
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
            .mode_tools("edit", vec![])
            .window_kind("main", "Main", "good.main", SurfaceKind::Canvas2d)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, "good.document")
            .default_layout(create_default_layout(&["main".into()], "row", None, None))
            .build_definition();
        assert_eq!(definition.window_kinds.len(), 1);
        assert_eq!(definition.panel_tabs.len(), 1);
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
    fn declaring_tools_injects_set_active_tool_action_and_keybinding() {
        use semio_framework_core::{ActionKind, ToolDefinition, SET_ACTIVE_TOOL_ACTION_ID};
        let definition = minimal_app("tool-app")
            .tool(ToolDefinition { keys: Some("b".into()), ..ToolDefinition::new("brush", "Brush", "icon.brush") })
            .tool_simple("eraser", "Eraser", "icon.eraser")
            .build_definition();
        let set_active_tool = definition
            .actions
            .iter()
            .find(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID)
            .expect("setActiveTool injected");
        assert_eq!(set_active_tool.kind, ActionKind::View);
        assert!(!set_active_tool.in_palette);
        let binding = definition
            .keybindings
            .iter()
            .find(|binding| binding.keys == "b")
            .expect("tool keybinding auto-injected");
        assert_eq!(binding.action.action, SET_ACTIVE_TOOL_ACTION_ID);
        assert_eq!(binding.action.args, Some(serde_json::json!({ "toolId": "brush" })));
    }

    #[test]
    fn no_tools_means_no_set_active_tool_action() {
        use semio_framework_core::SET_ACTIVE_TOOL_ACTION_ID;
        let definition = minimal_app("no-tool-app").build_definition();
        assert!(!definition.actions.iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID));
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
    fn build_definition_rejects_window_kind_tool_referencing_undeclared_tool() {
        let result = std::panic::catch_unwind(|| {
            minimal_app("bad-tool-ref-app")
                .tool_simple("brush", "Brush", "icon.brush")
                .window_kind_tools("main", vec!["missing".into()])
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
    /// undo. Use for cheap per-tick ops (camera/opacity). See the `🔖ToolPreviewContract` doc region.
    pub fn amend(ops: Vec<Op>, coalesce_key: impl Into<String>) -> Self {
        Self { ops, coalesce_key: Some(coalesce_key.into()), ..Default::default() }
    }

    /// @emoji 📌 Preview pattern (b): the gesture-end commit of an app-runtime scratch draft as one
    /// described edit (`coalesce_key: None`). Use for megabyte-scale content where per-tick amending
    /// would be O(N²) (draw drafts, lowpoly strokes). See the `🔖ToolPreviewContract` doc region.
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
/// view state (selection/camera/active tool) lives in the app struct itself, not in the document.
///
/// # 🔖ToolPreviewContract
/// The formalized actions-vs-tools contract:
/// - **Actions** are non-interactive: they carry optional declared `ActionArgDef`s, stage in the
///   renderer, and execute once. `Operation`-kind actions emit ops; `View`/`Shell`-kind actions must
///   emit **zero** ops ({@link VcsDocumentApp} enforces this — a View/Shell action returning ops is a
///   hard error).
/// - **Tools** are interactive live-preview pointer modes. Exactly one tool is active per window kind;
///   the active tool arrives via `view_state.active_tool_id` and is **never** stored in the document
///   nor emitted as an op. Switching tools dispatches the framework `setActiveTool` View action; on a
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
    ) -> Result<ActionResult, String>;
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
}

/// @emoji 📇 An app's action declarations indexed by id, built from its {@link AppDefinition}. Threaded
/// into {@link VcsDocumentApp} at registration time so the wrapper can enforce the actions contract
/// (default materialization, required-arg validation, kind discipline) without the plugin re-checking.
/// An empty registry (the test/registry-less construction path) skips all enforcement.
#[derive(Clone, Default)]
pub struct AppActionRegistry {
    actions: HashMap<String, ActionDefinition>,
}

impl AppActionRegistry {
    /// @emoji 📇 Indexes an app definition's declared actions (including framework-injected ones) by id.
    pub fn from_definition(definition: &AppDefinition) -> Self {
        Self {
            actions: definition.actions.iter().map(|action| (action.id.clone(), action.clone())).collect(),
        }
    }

    fn get(&self, id: &str) -> Option<&ActionDefinition> {
        self.actions.get(id)
    }
}

/// @emoji 🧬 Generic wrapper turning any typed {@link DocumentApp} into the object-safe runtime
/// {@link PluginApp}. Owns a persistent `DocumentVcsStore<Projection, Op>` — the single source of
/// truth for the app's document across every call — intercepts the six injected history actions into
/// `DocumentVcsCommand`s, dispatches `Apply`/`AmendLast` for typed operations, and builds an
/// `ActionResult` whose inverses come from the just-recorded `Edit.backwards`. A projection cache
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

    /// @emoji 📇 An empty `ActionResult` carrying only host effects/events (view/shell actions and
    /// history notifications produce no `KernelOperation`s).
    fn empty_result(action: &str, meta: &ActionMeta, effects: Vec<HostEffect>, events: Vec<AppEvent>, ui_scope: semio_framework_core::kernel::UiDirtyScope) -> ActionResult {
        let invocation_id = ActionInvocationId(format!("{action}:{}", meta.instance_id));
        ActionResult {
            output: Value::Null,
            operations: Vec::new(),
            inverse_group: UndoGroup {
                action_id: invocation_id,
                operations: Vec::new(),
                inverse_operations: Vec::new(),
            },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
        }
    }

    /// @emoji 🧱 Builds the `ActionResult` for a just-dispatched edit: one `KernelOperation` per
    /// forward operation, each carrying the edit's true `backwards` as its inverse diff.
    fn result_from_last_edit(&self, action: &str, meta: &ActionMeta, effects: Vec<HostEffect>, events: Vec<AppEvent>, ui_scope: semio_framework_core::kernel::UiDirtyScope) -> ActionResult {
        let schema = self.app.document_schema().to_string();
        let invocation_id = ActionInvocationId(format!("{action}:{}:{}", meta.instance_id, self.store.generation()));
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
                    action_id: invocation_id.clone(),
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
        ActionResult {
            output: Value::Null,
            operations,
            inverse_group: UndoGroup {
                action_id: invocation_id,
                operations: operation_ids,
                inverse_operations,
            },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
        }
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
    ) -> Result<ActionResult, String> {
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
            let materialized_args: Option<Value> = definition.as_ref().map(|def| {
                let staged = args.and_then(Value::as_object).cloned().unwrap_or_default();
                let effective = effective_action_args(&def.args, &staged);
                let missing = missing_required_args(&def.args, &effective);
                if !missing.is_empty() {
                    return Err(format!("action '{action}' missing required args: {missing:?}"));
                }
                let mut merged = staged;
                for (key, value) in effective {
                    merged.entry(key).or_insert(value);
                }
                Ok(Value::Object(merged))
            }).transpose()?;
            let dispatch_args = materialized_args.as_ref().or(args);
            let emit = {
                let VcsDocumentApp { app, cache, .. } = self;
                let (_, projection, history) = cache.as_ref().expect("cache refreshed above");
                let doc = DocumentView { projection, history };
                app.handle_action(action, dispatch_args, &doc, view_state)
            };
            let ActionEmit { ops, description, coalesce_key, effects, events, ui_scope } = emit;
            if let Some(def) = &definition {
                if matches!(def.kind, ActionKind::View | ActionKind::Shell) && !ops.is_empty() {
                    return Err(format!(
                        "{:?}-kind action '{action}' must not emit operations",
                        def.kind
                    ));
                }
            }
            if ops.is_empty() {
                return Ok(Self::empty_result(action, meta, effects, events, ui_scope));
            }
            self.store.set_local_actor_id(Some(meta.actor.clone()));
            let command = match coalesce_key {
                Some(key) => DocumentVcsCommand::AmendLast {
                    operations: ops,
                    coalesce_key: Some(key),
                },
                None => DocumentVcsCommand::Apply {
                    operations: ops,
                    description,
                },
            };
            self.store.dispatch(command).map_err(|error| error.to_string())?;
            self.cache = None;
            Ok(self.result_from_last_edit(action, meta, effects, events, ui_scope))
        }
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
            },
            apps: HashMap::new(),
        }
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

    pub fn local_backbone_storage(mut self) -> Self {
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

pub mod generate_mode {
// #region generate_mode
//! 🧬 Shared Generate mode state, CRUD, and declarative UI helpers.

use protocol::{default_value_for_block, flatten_protocol_blocks, is_block_visible, ProtocolBlock, ProtocolSpec};
use ui_wgpu::{
    build_text_editor_scene, ui_stack_vertical, ui_text, ActionDescriptor, TextEditorScene, UiControlNode,
    UiFieldNode, UiInputNode, UiNode, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemAction,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormGeneration {
    pub id: String,
    pub name: String,
    pub values: Map<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationPlayState {
    #[serde(default)]
    pub generations: Vec<FormGeneration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_generation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
}
//#endregion 🔖Types

//#region 🔖Crud
fn next_generation_id(generations: &[FormGeneration]) -> String {
    format!("generation-{}", generations.len() + 1)
}

fn next_generation_name(generations: &[FormGeneration]) -> String {
    format!("Generation {}", generations.len() + 1)
}

pub fn initial_generation_values(spec: &ProtocolSpec) -> Map<String, Value> {
    let mut values = Map::new();
    for question in flatten_protocol_blocks(spec) {
        values.insert(question.id.clone(), default_value_for_block(question));
    }
    values
}

pub fn add_generation(state: &mut GenerationPlayState, spec: &ProtocolSpec) -> String {
    let id = next_generation_id(&state.generations);
    let name = next_generation_name(&state.generations);
    state.generations.push(FormGeneration {
        id: id.clone(),
        name,
        values: initial_generation_values(spec),
    });
    state.selected_generation_id = Some(id.clone());
    id
}

pub fn remove_generation(state: &mut GenerationPlayState, generation_id: &str) {
    state.generations.retain(|entry| entry.id != generation_id);
    if state.selected_generation_id.as_deref() == Some(generation_id) {
        state.selected_generation_id = state.generations.first().map(|entry| entry.id.clone());
    }
}

pub fn rename_generation(state: &mut GenerationPlayState, generation_id: &str, name: &str) {
    if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
        entry.name = name.to_string();
    }
}

pub fn select_generation(state: &mut GenerationPlayState, generation_id: &str) {
    if state.generations.iter().any(|entry| entry.id == generation_id) {
        state.selected_generation_id = Some(generation_id.to_string());
    }
}

pub fn selected_generation<'a>(state: &'a GenerationPlayState) -> Option<&'a FormGeneration> {
    let selected_id = state.selected_generation_id.as_deref()?;
    state.generations.iter().find(|entry| entry.id == selected_id)
}

pub fn selected_generation_mut<'a>(state: &'a mut GenerationPlayState) -> Option<&'a mut FormGeneration> {
    let selected_id = state.selected_generation_id.clone()?;
    state.generations.iter_mut().find(|entry| entry.id == selected_id)
}

pub fn update_generation_values(
    state: &mut GenerationPlayState,
    generation_id: &str,
    question_id: &str,
    value: Value,
) {
    if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
        entry.values.insert(question_id.to_string(), value);
    }
}

pub fn handle_generation_action(
    action: &str,
    args: Option<&Value>,
    state: &mut GenerationPlayState,
    spec: &ProtocolSpec,
    controller_id: &str,
) -> bool {
    match action {
        "addGeneration" => {
            add_generation(state, spec);
            true
        }
        "removeGeneration" => {
            if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                remove_generation(state, id);
            }
            true
        }
        "selectGeneration" => {
            if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                select_generation(state, id);
            }
            true
        }
        "renameGeneration" => {
            let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
            let name = args.and_then(|value| value.get("name")).and_then(|value| value.as_str());
            if let (Some(id), Some(name)) = (id, name) {
                rename_generation(state, id, name);
            }
            true
        }
        "updateGenerationValues" => {
            let generation_id = args
                .and_then(|value| value.get("generationId"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
                .or_else(|| state.selected_generation_id.clone());
            let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str());
            let value = args.and_then(|value| value.get("value"));
            if let (Some(generation_id), Some(question_id), Some(value)) = (generation_id, question_id, value) {
                update_generation_values(state, &generation_id, question_id, value.clone());
            }
            let _ = controller_id;
            true
        }
        _ => false,
    }
}
//#endregion 🔖Crud

//#region 🔖Ops
/// @emoji 🧬 Typed, invertible Generate-mode operation vocabulary. WS-F embeds this as a variant in
/// `forms/module/procedural`'s own `Op` enum so generation edits flow through the document store with
/// true inverses (replacing the in-place-mutating CRUD helpers as the document mutation surface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GenerationOp {
    Add { generation: FormGeneration },
    Remove { id: String },
    Rename { id: String, name: String },
    UpdateValues { id: String, question_id: String, value: Value },
}

/// @emoji 🎛️ Maps a Generate-mode action id to the document operations it produces, or `None` for
/// non-document (view) actions like `selectGeneration`. Pure — reads `state`/`spec` but mutates
/// nothing; the caller applies the returned ops through its store.
pub fn generation_ops(
    action: &str,
    args: Option<&Value>,
    state: &GenerationPlayState,
    spec: &ProtocolSpec,
) -> Option<Vec<GenerationOp>> {
    let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
    match action {
        "addGeneration" => Some(vec![GenerationOp::Add {
            generation: FormGeneration {
                id: next_generation_id(&state.generations),
                name: next_generation_name(&state.generations),
                values: initial_generation_values(spec),
            },
        }]),
        "removeGeneration" => arg_str("id").map(|id| vec![GenerationOp::Remove { id }]),
        "renameGeneration" => {
            let id = arg_str("id")?;
            let name = arg_str("name")?;
            Some(vec![GenerationOp::Rename { id, name }])
        }
        "updateGenerationValues" => {
            let id = arg_str("generationId").or_else(|| state.selected_generation_id.clone())?;
            let question_id = arg_str("questionId")?;
            let value = args.and_then(|value| value.get("value")).cloned()?;
            Some(vec![GenerationOp::UpdateValues { id, question_id, value }])
        }
        _ => None,
    }
}

/// @emoji ▶️ Applies a {@link GenerationOp} to `state` in place.
pub fn apply_generation_op(state: &mut GenerationPlayState, op: &GenerationOp) {
    match op {
        GenerationOp::Add { generation } => {
            state.generations.push(generation.clone());
            state.selected_generation_id = Some(generation.id.clone());
        }
        GenerationOp::Remove { id } => remove_generation(state, id),
        GenerationOp::Rename { id, name } => rename_generation(state, id, name),
        GenerationOp::UpdateValues { id, question_id, value } => {
            update_generation_values(state, id, question_id, value.clone())
        }
    }
}

/// @emoji ↩️ Computes the inverse of a {@link GenerationOp} from the pre-state `state`.
pub fn invert_generation_op(state: &GenerationPlayState, op: &GenerationOp) -> Vec<GenerationOp> {
    match op {
        GenerationOp::Add { generation } => vec![GenerationOp::Remove { id: generation.id.clone() }],
        GenerationOp::Remove { id } => state
            .generations
            .iter()
            .find(|entry| entry.id == *id)
            .map(|entry| vec![GenerationOp::Add { generation: entry.clone() }])
            .unwrap_or_default(),
        GenerationOp::Rename { id, .. } => state
            .generations
            .iter()
            .find(|entry| entry.id == *id)
            .map(|entry| vec![GenerationOp::Rename { id: id.clone(), name: entry.name.clone() }])
            .unwrap_or_default(),
        GenerationOp::UpdateValues { id, question_id, .. } => state
            .generations
            .iter()
            .find(|entry| entry.id == *id)
            .map(|entry| {
                vec![GenerationOp::UpdateValues {
                    id: id.clone(),
                    question_id: question_id.clone(),
                    value: entry.values.get(question_id).cloned().unwrap_or(Value::Null),
                }]
            })
            .unwrap_or_default(),
    }
}
//#endregion 🔖Ops

//#region 🔖Render
fn generation_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args,
    }
}

pub fn render_generations_tree(
    controller_id: &str,
    surface_prefix: &str,
    generations: &[FormGeneration],
    selected_id: Option<&str>,
) -> UiNode {
    let items: Vec<UiTreeItemNode> = generations
        .iter()
        .map(|generation| {
            let mut actions = vec![UiTreeItemAction {
                icon_id: "trash-2".into(),
                label: Some("Remove".into()),
                action: generation_action(
                    controller_id,
                    "removeGeneration",
                    Some(json!({ "id": generation.id })),
                ),
                reveal_on_hover: Some(true),
            }];
            actions.insert(
                0,
                UiTreeItemAction {
                    icon_id: "pencil".into(),
                    label: Some("Rename".into()),
                    action: generation_action(
                        controller_id,
                        "renameGeneration",
                        Some(json!({ "id": generation.id, "name": format!("{} copy", generation.name) })),
                    ),
                    reveal_on_hover: Some(true),
                },
            );
            UiTreeItemNode {
                id: format!("{surface_prefix}.generation.{}", generation.id),
                label: generation.name.clone(),
                description: Some(format!("{} values", generation.values.len())),
                icon_id: Some("layers".into()),
                selected: Some(selected_id == Some(generation.id.as_str())),
                default_open: None,
                action: Some(generation_action(
                    controller_id,
                    "selectGeneration",
                    Some(json!({ "id": generation.id })),
                )),
                hover_action: None,
                unhover_action: None,
                actions: Some(actions),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    let mut sections = vec![UiTreeSectionNode {
        id: format!("{surface_prefix}.generations"),
        label: Some("Generations".into()),
        default_open: Some(true),
        items: if items.is_empty() {
            vec![UiTreeItemNode {
                id: format!("{surface_prefix}.generations.empty"),
                label: "(no generations)".into(),
                description: None,
                icon_id: None,
                selected: None,
                default_open: None,
                action: None,
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }]
        } else {
            items
        },
    }];
    sections.push(UiTreeSectionNode {
        id: format!("{surface_prefix}.actions"),
        label: Some("Actions".into()),
        default_open: Some(true),
        items: vec![UiTreeItemNode {
            id: format!("{surface_prefix}.add-generation"),
            label: "Add Generation".into(),
            description: None,
            icon_id: Some("plus".into()),
            selected: None,
            default_open: None,
            action: Some(generation_action(controller_id, "addGeneration", None)),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }],
    });
    UiNode::Tree(UiTreeNode {
        sections,
        selected_ids: selected_id.map(|id| vec![format!("{surface_prefix}.generation.{id}")]),
        highlighted_ids: None,
        selection_change: Some(generation_action(controller_id, "selectGeneration", None)),
        drop_action: None,
    })
}

fn render_question_field(
    question: &ProtocolBlock,
    values: &Map<String, Value>,
    controller_id: &str,
    patch_action: &str,
    generation_id: &str,
) -> Option<UiNode> {
    if !is_block_visible(question, values) {
        return None;
    }
    let value = values
        .get(&question.id)
        .cloned()
        .unwrap_or_else(|| default_value_for_block(question));
    let field_id = format!("generate.form.{}", question.id);
    let on_change = || {
        generation_action(
            controller_id,
            patch_action,
            Some(json!({
                "generationId": generation_id,
                "questionId": question.id,
            })),
        )
    };
    let child = match question.kind.as_str() {
        "text" | "longText" => UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: if question.kind == "longText" { "textarea".into() } else { "text".into() },
            value: value.as_str().unwrap_or_default().to_string(),
            placeholder: question.placeholder.clone(),
            commit: None,
            on_change: on_change(),
            min: None,
            max: None,
            step: None,
            accept: None,
        }),
        "number" => UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: value.as_f64().map(|number| number.to_string()).unwrap_or_default(),
            placeholder: question.placeholder.clone(),
            commit: None,
            on_change: on_change(),
            min: None,
            max: None,
            step: None,
            accept: None,
        }),
        "slider" => UiControlNode::Slider(UiSliderNode {
            id: format!("{field_id}.slider"),
            value: value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0)),
            min: question.min.unwrap_or(0.0),
            max: question.max.unwrap_or(100.0),
            step: question.step.unwrap_or(1.0),
            on_change: on_change(),
            unit: None,
        }),
        "boolean" => UiControlNode::Toggle(UiToggleNode {
            id: format!("{field_id}.toggle"),
            icon_id: "toggle-left".into(),
            pressed: value.as_bool().unwrap_or(false),
            text: Some(question.label.clone()),
            on_change: on_change(),
        }),
        "single" => {
            let items = question
                .options
                .as_ref()
                .map(|options| {
                    options
                        .iter()
                        .map(|option| UiSelectItem {
                            value: option.value.clone(),
                            label: option.label.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            UiControlNode::Select(UiSelectNode {
                id: format!("{field_id}.select"),
                value: value.as_str().unwrap_or_default().to_string(),
                items,
                placeholder: question.placeholder.clone(),
                on_change: on_change(),
            })
        }
        "vector" => {
            let numbers = value
                .as_array()
                .cloned()
                .unwrap_or_else(|| {
                    question
                        .fields
                        .as_ref()
                        .map(|fields| fields.iter().map(|field| json!(field.value.unwrap_or(0.0))).collect())
                        .unwrap_or_default()
                });
            let labels: Vec<String> = question
                .fields
                .as_ref()
                .map(|fields| {
                    fields
                        .iter()
                        .map(|field| field.label.clone().unwrap_or_else(|| field.key.clone()))
                        .collect()
                })
                .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());
            let children: Vec<UiNode> = numbers
                .iter()
                .enumerate()
                .map(|(index, number)| {
                    let label = labels.get(index).cloned().unwrap_or_else(|| format!("Field {}", index + 1));
                    UiNode::Field(UiFieldNode {
                        id: format!("{field_id}.vector.{index}"),
                        label,
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: format!("{field_id}.vector.{index}.input"),
                            input_kind: "number".into(),
                            value: number.as_f64().map(|entry| entry.to_string()).unwrap_or_default(),
                            placeholder: None,
                            commit: None,
                            on_change: generation_action(
                                controller_id,
                                patch_action,
                                Some(json!({
                                    "generationId": generation_id,
                                    "questionId": question.id,
                                    "fieldIndex": index,
                                })),
                            ),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                    })
                })
                .collect();
            return Some(ui_stack_vertical(children));
        }
        "note" => return Some(ui_text(question.text.clone().unwrap_or_default())),
        "image" => return Some(ui_text(question.src.clone().unwrap_or_else(|| "(no image)".into()))),
        _ => UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: value.to_string(),
            placeholder: question.placeholder.clone(),
            commit: None,
            on_change: on_change(),
            min: None,
            max: None,
            step: None,
            accept: None,
        }),
    };
    Some(UiNode::Field(UiFieldNode {
        id: field_id,
        label: question.label.clone(),
        child: Box::new(ui_wgpu::ui_control_to_node(child)),
        description: None,
        required: None,
        error: None,
    }))
}

pub fn render_generation_form_body(
    form_spec: &ProtocolSpec,
    values: &Map<String, Value>,
    controller_id: &str,
    patch_action: &str,
    generation_id: &str,
) -> UiNode {
    let mut children = Vec::new();
    for step in &form_spec.steps {
        if !step.blocks.is_empty() {
            children.push(ui_text(step.title.clone()));
        }
        for question in &step.blocks {
            if let Some(field) = render_question_field(question, values, controller_id, patch_action, generation_id) {
                children.push(field);
            }
        }
    }
    if children.is_empty() {
        return ui_text("No input widgets to generate from.");
    }
    ui_stack_vertical(children)
}

pub fn render_generation_preview_text(surface: &str, controller_id: &str, text: &str) -> UiNode {
    build_text_editor_scene(
        surface,
        controller_id,
        TextEditorScene::base(text.to_string(), Some("json".into()), None),
    )
}
//#endregion 🔖Render

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{ProtocolBlock, ProtocolStep, PROTOCOL_DOCUMENT_SCHEMA};

    fn sample_spec() -> ProtocolSpec {
        ProtocolSpec {
            schema: PROTOCOL_DOCUMENT_SCHEMA.into(),
            id: "sample".into(),
            version: "1".into(),
            title: None,
            steps: vec![ProtocolStep {
                id: "s".into(),
                title: "Inputs".into(),
                description: None,
                blocks: vec![ProtocolBlock {
                    id: "width".into(),
                    label: "Width".into(),
                    kind: "slider".into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(json!(1.0)),
                    min: Some(0.0),
                    max: Some(10.0),
                    step: Some(0.5),
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                }],
            }],
        }
    }

    #[test]
    fn generation_crud_round_trip() {
        let spec = sample_spec();
        let mut state = GenerationPlayState::default();
        let id = add_generation(&mut state, &spec);
        assert_eq!(state.generations.len(), 1);
        rename_generation(&mut state, &id, "Variant A");
        update_generation_values(&mut state, &id, "width", json!(4.0));
        assert_eq!(selected_generation(&state).unwrap().name, "Variant A");
        remove_generation(&mut state, &id);
        assert!(state.generations.is_empty());
    }

    #[test]
    fn render_generations_tree_contains_add_action() {
        let json = serde_json::to_string(&render_generations_tree(
            "flow-play",
            "flow-generate",
            &[],
            None,
        ))
        .unwrap();
        assert!(json.contains("addGeneration"));
    }
}
// #endregion generate_mode
}

pub mod protocol_mode {
// #region protocol_mode
//! 🧩 Shared strict-list, Blockly-like builder engine: generic step/block CRUD op-builders and
//! [`ProtocolListScene`] rendering, reused by `protocol-plugin` (standalone) and `forms-plugin`
//! (embedded Blueprint mode). Block-kind-specific property editing stays with the host app.

use protocol::{ProtocolBlock, ProtocolOp, ProtocolSpec, ProtocolStep};
use ui_wgpu::{
    ActionDescriptor, ProtocolListScene, ProtocolPaletteEntry, SurfaceKind, UiComponentSceneNode, UiNode,
};
use serde_json::Value;

//#region 🔖Config
#[derive(Clone, Debug)]
pub struct ProtocolBuilderLabels {
    pub add_step: &'static str,
    pub remove_step: &'static str,
    pub move_up: &'static str,
    pub move_down: &'static str,
    pub add_block: &'static str,
}

pub const PROTOCOL_BUILDER_LABELS_EN: ProtocolBuilderLabels = ProtocolBuilderLabels {
    add_step: "Add Step",
    remove_step: "Remove Step",
    move_up: "Move Up",
    move_down: "Move Down",
    add_block: "Add Block",
};

/// 🧩 Configures the generic strict-list builder for a host app: an action-namespace prefix
/// (used for element/surface ids so multiple embeddings don't collide), and its labels.
#[derive(Clone, Debug)]
pub struct ProtocolBuilderConfig {
    pub action_namespace: &'static str,
    pub controller_id: &'static str,
    pub labels: ProtocolBuilderLabels,
}
//#endregion 🔖Config

//#region 🔖OpBuilders
pub fn add_step_op(spec: &ProtocolSpec, step_id: String) -> ProtocolOp {
    ProtocolOp::AddStep {
        step: ProtocolStep {
            id: step_id,
            title: format!("Step {}", spec.steps.len() + 1),
            description: None,
            blocks: Vec::new(),
        },
        index: None,
    }
}

pub fn remove_step_op(step_id: &str) -> ProtocolOp {
    ProtocolOp::RemoveStep { step_id: step_id.into() }
}

pub fn move_step_op(step_id: &str, index: usize) -> ProtocolOp {
    ProtocolOp::MoveStep {
        step_id: step_id.into(),
        index,
    }
}

pub fn add_block_op(step_id: &str, block: ProtocolBlock, index: Option<usize>) -> ProtocolOp {
    ProtocolOp::AddBlock {
        step_id: step_id.into(),
        block,
        index,
    }
}

pub fn remove_block_op(step_id: &str, block_id: &str) -> ProtocolOp {
    ProtocolOp::RemoveBlock {
        step_id: step_id.into(),
        block_id: block_id.into(),
    }
}

pub fn move_block_op(block_id: &str, from_step_id: &str, to_step_id: &str, index: usize) -> ProtocolOp {
    ProtocolOp::MoveBlock {
        block_id: block_id.into(),
        from_step_id: from_step_id.into(),
        to_step_id: to_step_id.into(),
        index,
    }
}

pub fn update_protocol_title_op(title: Option<String>) -> ProtocolOp {
    ProtocolOp::UpdateProtocol { title }
}
//#endregion 🔖OpBuilders

//#region 🔖Render
pub fn protocol_builder_action(config: &ProtocolBuilderConfig, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: config.controller_id.into(),
        action: action.into(),
        args,
    }
}

/// 🧩 Builds the palette of insertable block kinds from a host app's built-in kinds plus any
/// `Contribution::ProtocolBlockKind` modules already resolved by the caller into label/icon pairs.
pub fn build_palette(builtin: &[(&str, &str, &str)], extensions: &[(String, String, String)]) -> Vec<ProtocolPaletteEntry> {
    let mut entries: Vec<ProtocolPaletteEntry> = builtin
        .iter()
        .map(|(kind, label, icon_id)| ProtocolPaletteEntry {
            block_kind: (*kind).into(),
            label: (*label).into(),
            icon_id: (*icon_id).into(),
        })
        .collect();
    entries.extend(extensions.iter().map(|(kind, label, icon_id)| ProtocolPaletteEntry {
        block_kind: kind.clone(),
        label: label.clone(),
        icon_id: icon_id.clone(),
    }));
    entries
}

pub fn build_protocol_list_scene(spec: &ProtocolSpec, palette: &[ProtocolPaletteEntry], selected_id: Option<&str>) -> ProtocolListScene {
    ProtocolListScene {
        steps_json: serde_json::to_string(&spec.steps).unwrap_or_else(|_| "[]".into()),
        palette_json: serde_json::to_string(palette).unwrap_or_else(|_| "[]".into()),
        selected_id: selected_id.map(String::from),
        dragging_id: None,
    }
}

/// 🧩 Renders the strict-list Blockly-like builder as a [`SurfaceKind::ProtocolList`] component
/// scene, handed off to the dedicated `protocol-list-host.tsx` React host for drag-and-drop.
pub fn render_protocol_builder(
    surface_id: &str,
    spec: &ProtocolSpec,
    palette: &[ProtocolPaletteEntry],
    selected_id: Option<&str>,
    config: &ProtocolBuilderConfig,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: config.controller_id.into(),
        component_kind: SurfaceKind::ProtocolList,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        raster: None,
        virtual_file_system: None,
        gis_map: None,
        puzzle2d_board: None,
        icon_render: None,
        note_canvas: None,
        vcs_history: None,
        protocol_list: Some(build_protocol_list_scene(spec, palette, selected_id)),
    })
}
//#endregion 🔖Render

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::empty_protocol_projection;

    fn sample_config() -> ProtocolBuilderConfig {
        ProtocolBuilderConfig {
            action_namespace: "protocol-play",
            controller_id: "protocol-play",
            labels: PROTOCOL_BUILDER_LABELS_EN,
        }
    }

    #[test]
    fn add_step_op_names_step_by_position() {
        let spec = empty_protocol_projection();
        let op = add_step_op(&spec, "step-2".into());
        assert_eq!(
            op,
            ProtocolOp::AddStep {
                step: ProtocolStep {
                    id: "step-2".into(),
                    title: "Step 2".into(),
                    description: None,
                    blocks: Vec::new(),
                },
                index: None,
            }
        );
    }

    #[test]
    fn render_protocol_builder_emits_protocol_list_component_scene() {
        let spec = empty_protocol_projection();
        let config = sample_config();
        let node = render_protocol_builder("surface", &spec, &[], None, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"componentKind\":\"protocol-list\""));
        assert!(json.contains("\"protocolList\""));
    }
}
// #endregion protocol_mode
}

pub mod plugin_runtime {
// #region plugin_runtime
//! 📤 WASM component export glue for plugin bundles.

use crate::app::{ActionMeta, AppInstance, Plugin, PluginBundle};
use semio_framework_core::{kernel::ActionResult, PluginManifest, ViewState};
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
) -> Result<ActionResult, String> {
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
/// plugin section — the renderer derives them from the tool registry via `derive_tool_nodes`.
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
//#endregion RefreshUi

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
                // 🧪 Reads the host-owned active tool from view state (never the document) and echoes it
                // as an event — proving `setActiveTool` forwards `view_state.active_tool_id` and emits no ops.
                "setActiveTool" => ActionEmit::event(AppEvent {
                    kind: "active-tool".into(),
                    payload: json!({ "toolId": view_state.active_tool_id.clone().unwrap_or_default() }),
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
    /// required arg, one with a defaulted optional arg, a mis-behaving View action, and a tool (which
    /// auto-injects the `setActiveTool` View action).
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
                .tool_simple("brush", "Brush", "icon.brush"),
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
    fn set_active_tool_forwards_view_state_active_tool_and_emits_no_ops() {
        let mut app = contract_app_under_test();
        let view_state = ViewState { active_tool_id: Some("brush".into()), ..ViewState::default() };
        let result = app
            .handle_action("setActiveTool", Some(&json!({ "toolId": "brush" })), &view_state, &meta())
            .expect("setActiveTool is a valid View action");
        assert!(result.operations.is_empty(), "tool switching must not create history");
        let event = result.events.iter().find(|event| event.kind == "active-tool").expect("echoed active tool");
        assert_eq!(event.payload, json!({ "toolId": "brush" }));
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
    KeybindingSpec, ModeSpec, PanelTabSpec, Plugin, PluginApp, PluginBundle, VcsDocumentApp,
    WindowKindSpec,
};
pub use semio_framework_core::AppLabelsOverlay;
pub use generate_mode::{
    add_generation, apply_generation_op, generation_ops, handle_generation_action,
    initial_generation_values, invert_generation_op, remove_generation, rename_generation,
    render_generation_form_body, render_generation_preview_text, render_generations_tree,
    select_generation, selected_generation, selected_generation_mut, update_generation_values,
    FormGeneration, GenerationOp, GenerationPlayState,
};
pub use protocol_mode::{
    add_block_op, add_step_op, build_palette, build_protocol_list_scene, move_block_op, move_step_op,
    protocol_builder_action, remove_block_op, remove_step_op, render_protocol_builder, update_protocol_title_op,
    ProtocolBuilderConfig, ProtocolBuilderLabels, PROTOCOL_BUILDER_LABELS_EN,
};
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
// 🧩 Declarative component model (UiNode, layouts, tools) — moved into ui_wgpu; re-exported here so
// apps keep the flat `semio_framework_plugin::*` import surface with zero Cargo.toml churn.
pub use ui_wgpu::*;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle($bundle);
    };
}
