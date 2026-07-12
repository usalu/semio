//! 🔌 Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM components.

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub mod component {
    //! 🧩 WASI P2 component exports for the plugin world contract.

    use crate::plugin_runtime::{
        ensure_plugin_initialized, plugin_app_labels, plugin_create_app, plugin_handle_action, plugin_manifest,
        plugin_render_with_document, plugin_tools, plugin_window_engagements, plugin_window_measures,
    };
    use wit_bindgen::generate;

    generate!({
        world: "plugin-world",
        path: "../../wit",
    });

    use exports::semio::framework::plugin::Guest;
    use semio::framework::types::{
        ActionContextJson, ActionInvocationJson, ActionResponseJson, AppLabelsJson, MigrateDocumentInput,
        MigrateDocumentOutput, PluginError, PluginManifestJson, PluginToolsJson, PluginWindowEngagementsJson,
        PluginWindowMeasuresJson, WindowInputJson, WindowOutputJson,
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

        fn list_tools(
            instance_id: u32,
            context: ActionContextJson,
        ) -> Result<PluginToolsJson, PluginError> {
            ensure_plugin_initialized();
            let tools = plugin_tools(instance_id, &context.json).map_err(PluginError::Message)?;
            Ok(PluginToolsJson {
                json: serde_json::to_string(&tools).unwrap_or_else(|_| "[]".into()),
            })
        }

        fn window_engagements(
            instance_id: u32,
            context: ActionContextJson,
        ) -> Result<PluginWindowEngagementsJson, PluginError> {
            ensure_plugin_initialized();
            let engagements =
                plugin_window_engagements(instance_id, &context.json).map_err(PluginError::Message)?;
            Ok(PluginWindowEngagementsJson {
                json: serde_json::to_string(&engagements).unwrap_or_else(|_| "{}".into()),
            })
        }

        fn window_measures(
            instance_id: u32,
            context: ActionContextJson,
        ) -> Result<PluginWindowMeasuresJson, PluginError> {
            ensure_plugin_initialized();
            let measures =
                plugin_window_measures(instance_id, &context.json).map_err(PluginError::Message)?;
            Ok(PluginWindowMeasuresJson {
                json: serde_json::to_string(&measures).unwrap_or_else(|_| "{}".into()),
            })
        }

        fn app_labels(
            instance_id: u32,
            context: ActionContextJson,
        ) -> Result<AppLabelsJson, PluginError> {
            ensure_plugin_initialized();
            let overlay = plugin_app_labels(instance_id, &context.json).map_err(PluginError::Message)?;
            Ok(AppLabelsJson {
                json: serde_json::to_string(&overlay).unwrap_or_else(|_| "{}".into()),
            })
        }

        fn migrate_document(_input: MigrateDocumentInput) -> Result<MigrateDocumentOutput, PluginError> {
            Err(PluginError::Message("migrate-document not implemented".into()))
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
    collect_window_kind_ids_from_layout, history_action_definitions, kernel::{
        ActorId, CapabilityRequirement, ActionInvocationId, ActionResult, HybridLogicalTimestamp,
        InverseOperation, KernelOperation, DocumentDiff, DocumentHandle, DocumentVersion, OperationId, Rights,
        ResourceKind, SchemaId, Scope, UndoGroup, UndoPolicy,
    },
    ActionRef, AppDefinition, AppLabelsOverlay, ActionDefinition, ActionDescriptor, ActionKind, Contribution, ExampleDefinition, Keybinding,
    ModeDefinition, Modes, NamedLayout, PanelGroup, PanelTabDefinition, PanelTabKind, PluginManifest, ProgramDefinition, ToolNode,
    UiNode, ViewState, WindowEngagement, WindowEngagementSlot, WindowKindDefinition, WindowKinds, WindowLayout, WindowMeasure,
    WindowOptions, SurfaceKind,
};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub struct ModeSpec {
    pub id: String,
    pub label: String,
    pub tools: Vec<ToolNode>,
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

    pub fn mode_tools(mut self, mode_id: impl AsRef<str>, tools: Vec<ToolNode>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.tools = tools;
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

    /// @emoji 📇 Declares a fully specified action (icon, args schema, keybinding, palette visibility, category).
    pub fn action_with(mut self, action: ActionDefinition) -> Self {
        self.actions.push(action);
        self
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
        }
        let app_declared_actions = !self.actions.is_empty();
        let mut actions = self.actions;
        for history_action in history_action_definitions() {
            if declared_action_ids.insert(history_action.id.clone()) {
                actions.push(history_action);
            }
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
            named_layouts: self.named_layouts,
            default_layout: self.default_layout,
            terminologies: self.terminologies,
        }
    }
}

#[cfg(test)]
mod app_builder_tests {
    use super::*;
    use semio_framework_core::create_default_layout;

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

pub trait PluginApp: Send {
    fn app_id(&self) -> &str;
    fn initial_document_json(&self) -> String;
    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        view_state: &ViewState,
    ) -> Vec<String>;
    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode;
    fn tools(&self, _document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        Vec::new()
    }
    fn window_engagements(
        &self,
        _document_json: &str,
        _view_state: &ViewState,
    ) -> std::collections::HashMap<String, semio_framework_core::layout::WindowEngagement> {
        std::collections::HashMap::new()
    }
    fn window_measures(
        &self,
        _document_json: &str,
        _view_state: &ViewState,
    ) -> std::collections::HashMap<String, Vec<semio_framework_core::WindowMeasure>> {
        std::collections::HashMap::new()
    }
    /// 🗣️ Locale/terminology-aware overlay for this app's window-kind/mode labels, resolved fresh per `ViewState`
    /// (unlike the static `AppDefinition` labels baked in at manifest-build time). Framework panel-tab labels
    /// (Document/Catalogue/Inspection/Parameters) are merged in automatically by `plugin_runtime::plugin_app_labels`
    /// and do not need to be supplied here.
    fn app_labels(&self, _view_state: &ViewState) -> AppLabelsOverlay {
        AppLabelsOverlay::default()
    }
}

pub struct AppInstance {
    pub id: u32,
    pub app: Box<dyn PluginApp>,
    pub document_json: String,
}

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
use semio_framework_core::{
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
        child: Box::new(semio_framework_core::ui_control_to_node(child)),
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
use semio_framework_core::{
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

use crate::app::{AppInstance, Plugin, PluginBundle};
use semio_framework_core::{
    kernel::{
        ActorId, ActionInvocationId, ActionResult, HybridLogicalTimestamp, InverseOperation,
        KernelOperation, DocumentDiff, DocumentHandle, DocumentVersion, OperationId, SchemaId, UndoGroup,
        UndoPolicy,
    },
    framework_panel_tab_label, AppLabelsOverlay, PluginManifest, UiNode, ViewState,
};
use serde::Deserialize;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicU32, Ordering};

const JSON_PATCH_SCHEMA_ID: &str = "semio.kernel.json-patch";

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

fn with_instances<R, F: FnOnce(&Vec<AppInstance>) -> Result<R, String>>(f: F) -> Result<R, String> {
    let _guard = InstanceGuard::enter()?;
    INSTANCES.with(|instances| f(&instances.borrow()))
}

pub fn action_result_from_patch_ops(
    patch_ops: Vec<String>,
    action: &str,
    instance_id: u32,
    generation: u64,
    actor: &str,
) -> ActionResult {
    let invocation_id = ActionInvocationId(format!("{action}:{instance_id}:{generation}"));
    let actor_id = ActorId(actor.to_string());
    let document = DocumentHandle(instance_id as u128);
    let base_version = DocumentVersion(generation);
    let operations: Vec<KernelOperation> = patch_ops
        .iter()
        .enumerate()
        .map(|(index, op_json)| {
            let payload: serde_json::Value =
                serde_json::from_str(op_json).unwrap_or(serde_json::Value::Null);
            let operation_id = OperationId(format!("{}:{index}", invocation_id.0));
            KernelOperation {
                id: operation_id.clone(),
                document,
                base_version,
                action_id: invocation_id.clone(),
                diff: DocumentDiff {
                    schema_id: SchemaId(JSON_PATCH_SCHEMA_ID.into()),
                    payload,
                },
                inverse: InverseOperation {
                    target_operation: operation_id,
                    inverse_diff: DocumentDiff {
                        schema_id: SchemaId("semio.kernel.json-patch.inverse".into()),
                        payload: serde_json::Value::Null,
                    },
                    base_version,
                    dependencies: vec![],
                    undo_policy: UndoPolicy::ExactBaseOnly,
                },
                dependencies: vec![],
                author: actor_id.clone(),
                timestamp: HybridLogicalTimestamp::new(0, 0),
            }
        })
        .collect();
    let operation_ids: Vec<OperationId> = operations.iter().map(|op| op.id.clone()).collect();
    let inverse_operations: Vec<InverseOperation> =
        operations.iter().map(|op| op.inverse.clone()).collect();
    ActionResult {
        output: serde_json::Value::Null,
        operations,
        inverse_group: UndoGroup {
            action_id: invocation_id,
            operations: operation_ids,
            inverse_operations,
        },
        diagnostics: vec![],
        requested_effects: vec![],
        events: vec![],
    }
}

fn sync_instance_document(document_json: &str, result: &ActionResult) -> Result<String, String> {
    let mut document = document_json.to_string();
    for operation in &result.operations {
        if operation.diff.schema_id.0 != JSON_PATCH_SCHEMA_ID {
            continue;
        }
        let op_json = serde_json::to_string(&operation.diff.payload).map_err(|error| error.to_string())?;
        document = apply_document_op(&document, &op_json)?;
    }
    Ok(document)
}

fn apply_document_op(document_json: &str, op_json: &str) -> Result<String, String> {
    let mut document: serde_json::Value =
        serde_json::from_str(document_json).map_err(|error| error.to_string())?;
    let op: serde_json::Value = serde_json::from_str(op_json).map_err(|error| error.to_string())?;
    match op.get("op").and_then(|value| value.as_str()) {
        Some("setDocument") => {
            if let Some(next) = op.get("document") {
                document = next.clone();
            }
        }
        Some("patch") => {
            if let Some(patch) = op.get("patch") {
                merge_json(&mut document, patch);
            }
        }
        _ => {}
    }
    serde_json::to_string(&document).map_err(|error| error.to_string())
}

fn merge_json(target: &mut serde_json::Value, patch: &serde_json::Value) {
    match (target, patch) {
        (serde_json::Value::Object(target_map), serde_json::Value::Object(patch_map)) => {
            for (key, value) in patch_map {
                if value.is_null() {
                    target_map.remove(key);
                } else {
                    let entry = target_map
                        .entry(key.clone())
                        .or_insert(serde_json::Value::Null);
                    merge_json(entry, value);
                }
            }
        }
        (target_slot, patch_value) => {
            *target_slot = patch_value.clone();
        }
    }
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
        let document_json = app.initial_document_json();
        with_instances_mut(|list| {
            list.push(AppInstance {
                id,
                app,
                document_json,
            });
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
    let action_name = action
        .get("action")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    let args = action.get("args").cloned();
    let actor = context
        .get("actor")
        .and_then(|value| value.as_str())
        .unwrap_or("local");
  with_instances_mut(|list| {
        let instance = list
            .iter_mut()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        let generation = 0_u64;
        let patch_ops = instance.app.handle_action_patch_ops(
            action_name,
            args.as_ref(),
            &instance.document_json,
            &view_state,
        );
        let result = action_result_from_patch_ops(
            patch_ops,
            action_name,
            instance_id,
            generation,
            actor,
        );
        instance.document_json = sync_instance_document(&instance.document_json, &result)?;
        Ok(result)
    })
}

pub fn plugin_render(instance_id: u32, body_key: &str, view_state_json: &str) -> Result<UiNode, String> {
    plugin_render_with_document(instance_id, body_key, None, view_state_json)
}

pub fn plugin_render_with_document(
    instance_id: u32,
    body_key: &str,
    document_json: Option<&str>,
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
    let (resolved_body_key, view_state, override_document) = if body_key.is_empty() {
        let input: WindowRenderInput =
            serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
        (
            if input.body_key.is_empty() {
                body_key.to_string()
            } else {
                input.body_key
            },
            input.view_state,
            input.document_json,
        )
    } else if let Ok(input) = serde_json::from_str::<WindowRenderInput>(view_state_json) {
        (
            if input.body_key.is_empty() {
                body_key.to_string()
            } else {
                input.body_key
            },
            input.view_state,
            input
                .document_json
                .or_else(|| document_json.map(str::to_string)),
        )
    } else {
        let view_state: ViewState =
            serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
        (
            body_key.to_string(),
            view_state,
            document_json.map(str::to_string),
        )
    };
    with_instances(|list| {
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        let document = override_document
            .as_deref()
            .unwrap_or(instance.document_json.as_str());
        Ok(instance
            .app
            .render(&resolved_body_key, document, &view_state))
    })
}

pub fn plugin_tools(instance_id: u32, view_state_json: &str) -> Result<Vec<semio_framework_core::ToolNode>, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    with_instances(|list| {
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .tools(&instance.document_json, &view_state))
    })
}

pub fn plugin_window_engagements(
    instance_id: u32,
    view_state_json: &str,
) -> Result<std::collections::HashMap<String, semio_framework_core::layout::WindowEngagement>, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    with_instances(|list| {
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .window_engagements(&instance.document_json, &view_state))
    })
}

pub fn plugin_window_measures(
    instance_id: u32,
    view_state_json: &str,
) -> Result<std::collections::HashMap<String, Vec<semio_framework_core::WindowMeasure>>, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    with_instances(|list| {
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .window_measures(&instance.document_json, &view_state))
    })
}

pub fn plugin_app_labels(instance_id: u32, view_state_json: &str) -> Result<AppLabelsOverlay, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    with_instances(|list| {
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        let mut overlay = instance.app.app_labels(&view_state);
        let app_id = instance.app.app_id();
        let panel_tab_ids: Vec<String> = plugin_manifest()
            .apps
            .iter()
            .find(|app| app.id == app_id)
            .map(|app| app.panel_tabs.iter().map(|tab| tab.id().to_string()).collect())
            .unwrap_or_default();
        for id in panel_tab_ids {
            if let Some(label) = framework_panel_tab_label(&id, is_de) {
                overlay.panel_tab_labels.entry(id).or_insert_with(|| label.into());
            }
        }
        Ok(overlay)
    })
}

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
/// pairs an `App`-returning factory function with the `PluginApp` type instantiated for it — that
/// type must implement `Default` (multi-app crates list one entry per app, e.g. puzzle's
/// `d2::create_puzzle2d_app => d2::Puzzle2dPlayApp, d3::create_puzzle3d_app => d3::Puzzle3dPlayApp`).
/// Expands to the equivalent `bundle()` fn plus a `plugin_exports!(bundle)` call, and a
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
                $( .register_app(($app_fn)(), || ::std::boxed::Box::new(<$app_ty as ::std::default::Default>::default())) )+
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
    use crate::app::{App, PluginApp};
    use crate::SurfaceKind;
    use serde_json::Value;

    #[derive(Default)]
    struct SyntheticPlayApp;

    impl PluginApp for SyntheticPlayApp {
        fn app_id(&self) -> &str {
            "synthetic-play"
        }

        fn initial_document_json(&self) -> String {
            "{}".to_string()
        }

        fn handle_action_patch_ops(
            &mut self,
            _action: &str,
            _args: Option<&Value>,
            _document_json: &str,
            _view_state: &crate::ViewState,
        ) -> Vec<String> {
            Vec::new()
        }

        fn render(&self, _body_key: &str, _document_json: &str, _view_state: &crate::ViewState) -> crate::UiNode {
            crate::ui_text("synthetic")
        }
    }

    fn synthetic_play_app() -> App {
        App::from_builder(
            App::builder("synthetic-play", "Synthetic")
                .document(["state"])
                .window_kind("main", "Main", "synthetic.main", SurfaceKind::Canvas2d),
        )
    }

    fn synthetic_setup() {}

    crate::semio_plugin! {
        id: "synthetic", label: "Synthetic", version: "0.0.1",
        setup: synthetic_setup,
        apps: [ synthetic_play_app => SyntheticPlayApp ],
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
}
// #endregion plugin_runtime
}

pub mod scaffold {
// #region scaffold
//! 🧰 Helpers for scaffolding standard technology plugins.

use crate::{
    build_canvas_2d_scene, build_node_graph_scene, build_raster_scene, build_table_scene,
    build_text_editor_scene, build_world_3d_scene, default_world3d_selection, ui_stack_vertical,
    ui_text, world3d_default_meshes_json, App, Canvas2dScene, NodeGraphScene, PanelGroup, PluginApp,
    RasterScene, SurfaceKind, TableScene, TextEditorScene, UiNode, ViewState, World3dScene,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StandardApp {
    pub app_id: &'static str,
    pub label: &'static str,
    pub document: &'static [&'static str],
    pub program_id: Option<&'static str>,
    pub yields: Option<&'static str>,
    pub surface_id: &'static str,
    pub body_key: &'static str,
    pub surface_kind: SurfaceKind,
    pub initial_document_json: &'static str,
}

pub struct StandardPluginApp {
    pub spec: StandardApp,
}

fn document_body_key(body_key: &str) -> String {
    body_key.replace(".composite", ".document")
}

fn properties_body_key(body_key: &str) -> String {
    body_key.replace(".composite", ".properties")
}

fn json_field(document: &Value, keys: &[&str]) -> Option<Value> {
    keys.iter().find_map(|key| document.get(*key)).cloned()
}

fn canvas_layers_json(document: &Value, fallback: &str) -> String {
    json_field(
        document,
        &["layers", "tiles", "blocks", "features", "cells", "nodes"],
    )
    .map(|value| value.to_string())
    .unwrap_or_else(|| fallback.to_string())
}

fn world_instances_json(document: &Value, fallback: &str) -> String {
    json_field(
        document,
        &["instances", "entities", "meshes", "tiles", "cells", "parts"],
    )
    .map(|value| value.to_string())
    .unwrap_or_else(|| fallback.to_string())
}

fn node_graph_payload(document: &Value, fallback: &str) -> (String, String) {
    if let Some(nodes) = document.get("nodes") {
        let edges = document
            .get("edges")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        return (nodes.to_string(), edges.to_string());
    }
    if let Some(flow) = document.get("flow") {
        let nodes = flow
            .get("components")
            .or_else(|| flow.get("nodes"))
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        let edges = flow
            .get("edges")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        return (nodes.to_string(), edges.to_string());
    }
    if let Some(steps) = document.get("steps") {
        return (steps.to_string(), "[]".into());
    }
    (fallback.into(), "[]".into())
}

fn text_editor_payload(document: &Value, fallback: &str) -> (String, Option<String>) {
    if let Some(text) = document
        .get("text")
        .or_else(|| document.get("source"))
        .and_then(|value| value.as_str())
    {
        let language = document
            .get("language")
            .and_then(|value| value.as_str())
            .map(str::to_string);
        return (text.into(), language);
    }
    if document.is_string() {
        return (
            document.as_str().unwrap_or(fallback).into(),
            Some("plain".into()),
        );
    }
    (fallback.into(), Some("plain".into()))
}

fn table_payload(document: &Value, fallback: &str) -> (String, String) {
    let rows = json_field(document, &["rows", "edits", "records"])
        .map(|value| value.to_string())
        .unwrap_or_else(|| fallback.to_string());
    let columns = document
        .get("columns")
        .map(|value| value.to_string())
        .unwrap_or_else(|| r#"[{"id":"id","label":"Id"}]"#.into());
    (columns, rows)
}

fn raster_payload(document: &Value, fallback: &str) -> RasterScene {
    if let Ok(scene) = serde_json::from_value::<RasterScene>(document.clone()) {
        return scene;
    }
    let parsed: Value = serde_json::from_str(fallback).unwrap_or(Value::Null);
    let document_sync_json = if document.is_null() { parsed } else { document.clone() }.to_string();
    RasterScene {
        document_sync_json,
        assets_json: "[]".into(),
        camera_json: "{}".into(),
        selection_json: "{}".into(),
        hovered_id: None,
        active_tool: "brush".into(),
        brush_size: 8.0,
        brush_opacity: 1.0,
        view_mode: "edit".into(),
        composite_viewport_json: None,
    }
}

pub fn assert_standard_app_renders(spec: StandardApp) {
    let app = StandardPluginApp { spec };
    let node = app.render(spec.body_key, spec.initial_document_json, &ViewState::default());
    let json = serde_json::to_string(&node).expect("ui json");
    let tag = spec.surface_kind.as_str();
    assert!(json.contains(tag), "expected {tag} in {json}");
}

impl PluginApp for StandardPluginApp {
    fn app_id(&self) -> &str {
        self.spec.app_id
    }

    fn initial_document_json(&self) -> String {
        self.spec.initial_document_json.to_string()
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        if action == "setDocument" {
            if let Some(document) = args.and_then(|value| value.get("document")) {
                return vec![serde_json::json!({ "op": "setDocument", "document": document }).to_string()];
            }
        }
        if action == "patch" {
            if let Some(patch) = args.and_then(|value| value.get("patch")) {
                return vec![serde_json::json!({ "op": "patch", "patch": patch }).to_string()];
            }
        }
        let _ = document_json;
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let document_key = document_body_key(self.spec.body_key);
        let properties_key = properties_body_key(self.spec.body_key);
        if body_key == document_key {
            return render_document_panel(self.spec.label, document_json);
        }
        if body_key == properties_key {
            return render_properties_panel(self.spec.label, document_json);
        }
        if body_key != self.spec.body_key {
            return ui_text(format!("Unknown body: {body_key}"));
        }
        let document: Value =
            serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
        match self.spec.surface_kind {
            SurfaceKind::Canvas2d => build_canvas_2d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                Canvas2dScene {
                    camera_x: document
                        .get("camera")
                        .and_then(|camera| camera.get("x"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(0.0),
                    camera_y: document
                        .get("camera")
                        .and_then(|camera| camera.get("y"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(0.0),
                    zoom: document
                        .get("camera")
                        .and_then(|camera| camera.get("zoom"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(1.0),
                    layers_json: canvas_layers_json(&document, document_json),
                },
            ),
            SurfaceKind::World3d => build_world_3d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                World3dScene {
                    camera_json: document
                        .get("camera")
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| r#"{"x":0,"y":0,"z":5}"#.into()),
                    meshes_json: document
                        .get("meshes")
                        .map(|value| value.to_string())
                        .unwrap_or_else(world3d_default_meshes_json),
                    instances_json: world_instances_json(&document, document_json),
                    selection_json: document
                        .get("selection")
                        .map(|value| value.to_string())
                        .unwrap_or_else(default_world3d_selection),
                    vortices_json: None,
                    attractions_json: None,
                    target_volumes_json: None,
                    references_json: None,
                    brush_preview_json: None,
                    interaction_json: None,
                    engagement_preview_json: None,
                    lod_json: None,
                    chunking_json: None,
                    context_menu_json: None,
                    environment_json: None,
                    frame_json: None,
                    fit_json: None,
                },
            ),
            SurfaceKind::NodeGraph => {
                let (nodes_json, edges_json) = node_graph_payload(&document, document_json);
                build_node_graph_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    NodeGraphScene::base(
                        nodes_json,
                        edges_json,
                        document
                            .get("viewport")
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| r#"{"x":0,"y":0,"zoom":1}"#.into()),
                    ),
                )
            }
            SurfaceKind::TextEditor => {
                let (buffer, language) = text_editor_payload(&document, document_json);
                build_text_editor_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    TextEditorScene::base(buffer, language, None),
                )
            }
            SurfaceKind::Table => {
                let (columns_json, rows_json) = table_payload(&document, document_json);
                build_table_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    TableScene::base(columns_json, rows_json),
                )
            }
            SurfaceKind::Raster => build_raster_scene(
                self.spec.surface_id,
                self.spec.app_id,
                raster_payload(&document, document_json),
            ),
            _ => ui_text(format!("Unsupported surface: {}", self.spec.surface_kind.as_str())),
        }
    }
}

fn render_document_panel(label: &str, document_json: &str) -> UiNode {
    let document: Value =
        serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
    let schema = document
        .get("schema")
        .and_then(|value| value.as_str())
        .unwrap_or(label);
    let count = document
        .get("layers")
        .or_else(|| document.get("nodes"))
        .or_else(|| document.get("rows"))
        .or_else(|| document.get("entities"))
        .and_then(|value| value.as_array())
        .map(|rows| rows.len())
        .unwrap_or(0);
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {schema}")),
        ui_text(format!("Items: {count}")),
    ])
}

fn render_properties_panel(label: &str, document_json: &str) -> UiNode {
    let document: Value =
        serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
    let id = document
        .get("id")
        .and_then(|value| value.as_str())
        .unwrap_or(label);
    ui_stack_vertical(vec![
        ui_text(format!("App: {label}")),
        ui_text(format!("Id: {id}")),
    ])
}

pub fn standard_app(spec: StandardApp) -> App {
    let document_key = document_body_key(spec.body_key);
    let properties_key = properties_body_key(spec.body_key);
    let app = App::from_builder(
        App::builder(spec.app_id, spec.label)
            .document(spec.document.iter().copied())
            .icon_id(spec.app_id)
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .mode_tools("edit", vec![])
            .window_kind("main", "Main", spec.body_key, spec.surface_kind)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                &document_key,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                &properties_key,
            ),
    );
    if let (Some(program_id), Some(yields)) = (spec.program_id, spec.yields) {
        app.program(program_id, spec.label, yields)
    } else {
        app
    }
}

pub fn standard_factory(spec: StandardApp) -> Box<dyn PluginApp> {
    Box::new(StandardPluginApp { spec })
}

pub fn register_standard_app(bundle: crate::PluginBundle, spec: StandardApp) -> crate::PluginBundle {
    let app = standard_app(spec);
    bundle.register_app(app, move || standard_factory(spec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-canvas",
            label: "Canvas",
            document: &["semio", "test", "canvas"],
            program_id: None,
            yields: None,
            surface_id: "test.canvas",
            body_key: "test.canvas.composite",
            surface_kind: SurfaceKind::Canvas2d,
            initial_document_json: r#"{"schema":"test","id":"test","layers":[]}"#,
        });
    }

    #[test]
    fn node_graph_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-graph",
            label: "Graph",
            document: &["semio", "test", "graph"],
            program_id: None,
            yields: None,
            surface_id: "test.graph",
            body_key: "test.graph.composite",
            surface_kind: SurfaceKind::NodeGraph,
            initial_document_json: r#"{"nodes":[],"edges":[]}"#,
        });
    }

    #[test]
    fn world_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-world",
            label: "World",
            document: &["semio", "test", "world"],
            program_id: None,
            yields: None,
            surface_id: "test.world",
            body_key: "test.world.composite",
            surface_kind: SurfaceKind::World3d,
            initial_document_json: r#"{"schema":"test","id":"test","entities":[]}"#,
        });
    }

    #[test]
    fn text_editor_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-text",
            label: "Text",
            document: &["semio", "test", "text"],
            program_id: None,
            yields: None,
            surface_id: "test.text",
            body_key: "test.text.composite",
            surface_kind: SurfaceKind::TextEditor,
            initial_document_json: r#"{"schema":"test","id":"test","source":""}"#,
        });
    }

    #[test]
    fn table_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-table",
            label: "Table",
            document: &["semio", "test", "table"],
            program_id: None,
            yields: None,
            surface_id: "test.table",
            body_key: "test.table.composite",
            surface_kind: SurfaceKind::Table,
            initial_document_json: r#"{"schema":"test","id":"test","rows":[]}"#,
        });
    }

    #[test]
    fn raster_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-raster",
            label: "Raster",
            document: &["semio", "test", "raster"],
            program_id: None,
            yields: None,
            surface_id: "test.raster",
            body_key: "test.raster.composite",
            surface_kind: SurfaceKind::Raster,
            initial_document_json: r#"{"schema":"raster.document","id":"raster","width":64,"height":64,"pixelsBase64":""}"#,
        });
    }
}
// #endregion scaffold
}

pub mod world3d_host {
// #region world3d_host
//! 🌐 Shared world-3d scene payload builders for plugin apps.

use semio_framework_core::{
    mesh_from_kind, mesh_to_glb, mesh_to_obj, ActionDescriptor, MeshData, WindowMeasure,
    World3dScene, world3d_camera_json, world3d_default_selection_json,
};
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
    App, AppBuilder, AppInstance, KeybindingSpec, ModeSpec, PanelTabSpec, Plugin, PluginApp, PluginBundle,
    WindowKindSpec,
};
pub use semio_framework_core::AppLabelsOverlay;
pub use generate_mode::{
    add_generation, handle_generation_action, initial_generation_values, remove_generation,
    rename_generation, render_generation_form_body, render_generation_preview_text,
    render_generations_tree, select_generation, selected_generation, selected_generation_mut,
    update_generation_values, FormGeneration, GenerationPlayState,
};
pub use protocol_mode::{
    add_block_op, add_step_op, build_palette, build_protocol_list_scene, move_block_op, move_step_op,
    protocol_builder_action, remove_block_op, remove_step_op, render_protocol_builder, update_protocol_title_op,
    ProtocolBuilderConfig, ProtocolBuilderLabels, PROTOCOL_BUILDER_LABELS_EN,
};
pub use engagement::{engagement_token_matches, strip_engagement_prefix};
pub use host_port::{
    host_backbone_poll, host_backbone_send, host_now_ms, register_host_backbone_channel, HostBackboneChannel,
};
pub use plugin_runtime::{action_result_from_patch_ops, install_plugin_bundle};
pub use scaffold::{
    assert_standard_app_renders, register_standard_app, standard_app,
    standard_factory, StandardApp, StandardPluginApp,
};
pub use world3d_host::{
    apply_world3d_sun_action, default_world3d_selection, export_mesh_glb_bytes, export_mesh_obj,
    merge_world_selection_ids, mesh_kind_from_json, world3d_default_camera,
    world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds,
    world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_scene,
    world3d_scene_extended, world3d_selection_json, world3d_sun_measures, WorldSunConfig,
};
pub use semio_framework_core::*;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle($bundle);
    };
}
