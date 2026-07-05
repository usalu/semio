//! 🖥️ OS shell chrome — navbar, footer, floating panels, overlays, and studio mode.

use crate::dock::{
    compute_dock_drop_zone, dock_from_window_layout, parse_path, DockDragKind, DockDragPayload,
    DockDragState, DockDropZone, DockRenderContext, DockState,
};
use crate::interpreter::{framework_widget_context, render_ui_node};
use crate::scenes::{clear_graph_node_context, resolve_graph_context_command, seed_vfs_expanded, toggle_vfs_row_expanded, vfs_selection_for_click};
use infinite_world::{
    fetch_pending_glb_meshes, fetch_pending_reference_images, handle_world3d_paint_commands,
    handle_world3d_pointer_button,
    handle_world3d_pointer_drag, handle_world3d_pointer_move, handle_world3d_wheel, World3dState,
};
use crate::plugin_bridge::{is_studio_mode, PluginBridgeEntry};
use semio_framework_core::{
    app_hierarchy_label, app_window_hierarchy_label, AppDefinition, CommandDescriptor, ExampleDefinition, ModeDefinition, PanelTabDefinition,
    ToolNode, UiButtonNode, UiNode, UiSelectItem, UiSelectNode, UiStackNode, UiTextNode, ViewState, WindowEngagement,
    WindowEngagementControl, WindowEngagementInput, WindowEngagementOption, WindowMeasure,
};
use semio_framework_core::layout::{
    WindowEngagementPossible, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ui_wgpu::{
    chrome_item_bg, chrome_item_text, draw_text, push_chrome_group_border, DrawList, DragAxis, FontAtlas, HitKind,
    HitTarget, IconAtlas, InputState, PointerModifiers, Rect, Rgba, Theme, TreeDragState, TreeDropPosition,
    WidgetInteractionMaps,
};

const S_HOME_APP_ID: &str = "home";
const S_PLAY_APP_ID: &str = "studio";
const S_PLAY_CONTROLLER_ID: &str = "s-play";
const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";
const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID: &str = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID: &str = "framework.display.layout";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID: &str = "framework.settings.general";
const DEFAULT_MEASURES_RAIL_WIDTH: f32 = 240.0;
const DEFAULT_ENGAGEMENT_RAIL_WIDTH: f32 = 280.0;
const CHROME_ICON_TINY: f32 = 14.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LeftPanelKind {
    #[default]
    Workbench,
    Display,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RightPanelKind {
    #[default]
    Details,
    Settings,
}

#[derive(Clone, Debug)]
pub struct SearchPaletteItem {
    pub id: String,
    pub label: String,
    pub group: String,
    pub command: Option<CommandDescriptor>,
    pub action: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ShellFindItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub surface_id: String,
    pub node_id: String,
}

thread_local! {
    static FIND_ITEM_SINK: std::cell::RefCell<Vec<ShellFindItem>> = std::cell::RefCell::new(Vec::new());
    static CONTEXT_MENU_SINK: std::cell::RefCell<Vec<ContextMenuItem>> = std::cell::RefCell::new(Vec::new());
}

pub fn push_find_item(item: ShellFindItem) {
    FIND_ITEM_SINK.with(|cell| cell.borrow_mut().push(item));
}

pub fn take_find_items() -> Vec<ShellFindItem> {
    FIND_ITEM_SINK.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

pub fn push_context_menu_item(item: ContextMenuItem) {
    CONTEXT_MENU_SINK.with(|cell| cell.borrow_mut().push(item));
}

pub fn take_context_menu_items() -> Vec<ContextMenuItem> {
    CONTEXT_MENU_SINK.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

//#region ShellTypes
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioProgramEntry {
    pub plugin_id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub hierarchy: Vec<String>,
    pub yields: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedAppEntry {
    pub id: String,
    pub plugin_id: String,
    pub instance_id: u32,
    pub app_id: String,
    pub label: String,
    pub hierarchy: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioPanelState {
    pub active_panel_tab: String,
    pub programs: Vec<StudioProgramEntry>,
    pub spawned_apps: Vec<SpawnedAppEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_spawned_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
    pub command: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuState {
    pub x: f32,
    pub y: f32,
    pub items: Vec<ContextMenuItem>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum OverlayState {
    #[default]
    None,
    ThemeSelect,
    Search,
    Find,
    Dropdown(String),
}

#[derive(Clone, Debug, Default)]
pub struct RightClickState {
    pub pending: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct ActiveSession {
    pub plugin_id: String,
    pub instance_id: u32,
    pub app: AppDefinition,
    pub view_state: ViewState,
}

pub struct ShellState {
    pub plugins: Vec<PluginBridgeEntry>,
    pub plugin_filter: String,
    pub studio_mode: bool,
    pub session: Option<ActiveSession>,
    pub window_ui: HashMap<String, UiNode>,
    pub panel_ui: HashMap<String, UiNode>,
    pub spawned_ui: Option<UiNode>,
    pub active_window_id: Option<String>,
    pub left_panel_open: bool,
    pub right_panel_open: bool,
    pub left_panel_width: f32,
    pub right_panel_width: f32,
    pub scroll_offsets: HashMap<String, f32>,
    pub overlay_state: OverlayState,
    pub collapsed_sections: HashMap<String, bool>,
    pub open_selects: HashMap<String, bool>,
    pub active_right_tab: Option<String>,
    pub context_menu: Option<ContextMenuState>,
    pub search_open: bool,
    pub find_open: bool,
    pub theme_id: String,
    pub right_click: RightClickState,
    pub uri_history: Vec<String>,
    pub uri_index: usize,
    pub panel_resize_origin_width: f32,
    pub error: Option<String>,
    pub screen_w: f32,
    pub screen_h: f32,
    pub world3d_states: HashMap<String, World3dState>,
    pub dock: DockState,
    pub active_left_kind: LeftPanelKind,
    pub active_right_kind: RightPanelKind,
    pub search_query: String,
    pub search_selected: usize,
    pub find_query: String,
    pub split_resize_path: Option<Vec<usize>>,
    pub split_resize_index: usize,
    pub split_resize_axis_total: f32,
    pub active_example_id: Option<String>,
    pub active_left_tab: Option<String>,
    pub find_items: Vec<ShellFindItem>,
    pub find_selected: usize,
    pub engagement_expanded: HashMap<String, bool>,
    pub measures_folded: HashMap<String, bool>,
    pub measures_expanded: HashMap<String, bool>,
    pub measures_width: HashMap<String, f32>,
    pub measures_resize_origin_width: f32,
    pub engagement_inputs: HashMap<String, String>,
    pub compact_mode: bool,
    pub expertise: String,
    pub tree_drag: Option<TreeDragState>,
    pub tree_hovered_id: Option<String>,
    pub widget_maps: WidgetInteractionMaps<CommandDescriptor>,
    pub pending_tree_drag: Option<(String, HashMap<String, String>)>,
    pub tree_drag_origin: (f32, f32),
    pub dock_drag: Option<DockDragState>,
    pub pending_dock_drag: Option<(DockDragPayload, (f32, f32))>,
    pub dock_drag_snapshot: Option<semio_framework_core::layout::WindowLayout>,
    pub dock_canvas_bounds: Rect,
    pub dock_drop_tab_bars: Vec<(Vec<usize>, Rect, Vec<f32>)>,
    pub dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    pub layout_override: Option<semio_framework_core::layout::WindowLayout>,
    pub split_resize_origin: Vec<f32>,
    pub split_resize_secondary_path: Option<Vec<usize>>,
    pub split_resize_secondary_index: usize,
    pub split_resize_secondary_axis_total: f32,
    pub split_resize_secondary_origin: Vec<f32>,
    pub measures_resize_window_id: Option<String>,
    pub deferred_commands: Vec<CommandDescriptor>,
    pub active_tools: Vec<ToolNode>,
    pub window_engagements: HashMap<String, WindowEngagement>,
    pub tool_collection_expanded: HashMap<String, bool>,
}
//#endregion ShellTypes

//#region ShellLifecycle
impl ShellState {
    pub fn new(plugins: Vec<PluginBridgeEntry>, plugin_filter: String) -> Self {
        let studio_mode = is_studio_mode(&plugin_filter);
        Self {
            plugins,
            plugin_filter,
            studio_mode,
            session: None,
            window_ui: HashMap::new(),
            panel_ui: HashMap::new(),
            spawned_ui: None,
            active_window_id: None,
            left_panel_open: true,
            right_panel_open: true,
            left_panel_width: 280.0,
            right_panel_width: 320.0,
            scroll_offsets: HashMap::new(),
            overlay_state: OverlayState::None,
            collapsed_sections: HashMap::new(),
            open_selects: HashMap::new(),
            active_right_tab: None,
            context_menu: None,
            search_open: false,
            find_open: false,
            theme_id: "system".into(),
            right_click: RightClickState::default(),
            uri_history: vec!["os://home".into()],
            uri_index: 0,
            panel_resize_origin_width: 280.0,
            error: None,
            screen_w: 1280.0,
            screen_h: 720.0,
            world3d_states: HashMap::new(),
            dock: DockState::default(),
            active_left_kind: LeftPanelKind::Workbench,
            active_right_kind: RightPanelKind::Details,
            search_query: String::new(),
            search_selected: 0,
            find_query: String::new(),
            split_resize_path: None,
            split_resize_index: 0,
            split_resize_axis_total: 1.0,
            active_example_id: None,
            active_left_tab: None,
            find_items: Vec::new(),
            find_selected: 0,
            engagement_expanded: HashMap::new(),
            measures_folded: HashMap::new(),
            measures_expanded: HashMap::new(),
            measures_width: HashMap::new(),
            measures_resize_origin_width: DEFAULT_MEASURES_RAIL_WIDTH,
            engagement_inputs: HashMap::new(),
            compact_mode: false,
            expertise: "standard".into(),
            tree_drag: None,
            tree_hovered_id: None,
            widget_maps: WidgetInteractionMaps::default(),
            pending_tree_drag: None,
            tree_drag_origin: (0.0, 0.0),
            dock_drag: None,
            pending_dock_drag: None,
            dock_drag_snapshot: None,
            dock_canvas_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            dock_drop_tab_bars: Vec::new(),
            dock_drop_bodies: Vec::new(),
            layout_override: None,
            split_resize_origin: Vec::new(),
            split_resize_secondary_path: None,
            split_resize_secondary_index: 0,
            split_resize_secondary_axis_total: 1.0,
            split_resize_secondary_origin: Vec::new(),
            measures_resize_window_id: None,
            deferred_commands: Vec::new(),
            active_tools: Vec::new(),
            window_engagements: HashMap::new(),
            tool_collection_expanded: HashMap::new(),
        }
    }

    pub fn build_studio_programs(&self) -> Vec<StudioProgramEntry> {
        self.plugins
            .iter()
            .flat_map(|plugin| {
                plugin.manifest.programs.iter().map(|program| StudioProgramEntry {
                    plugin_id: plugin.plugin_id.clone(),
                    program_id: program.program_id.clone(),
                    app_id: program.app_id.clone(),
                    label: program.label.clone(),
                    hierarchy: program.hierarchy.clone(),
                    yields: program.yields.clone(),
                })
            })
            .collect()
    }

    pub fn panel_state_from_view(view_state: &ViewState) -> Option<StudioPanelState> {
        view_state
            .panel_json
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok())
    }

    pub fn panel_json(state: &StudioPanelState) -> String {
        serde_json::to_string(state).unwrap_or_default()
    }

    pub async fn boot(&mut self) -> Result<(), String> {
        if self.studio_mode {
            let s_plugin = self
                .plugins
                .iter()
                .find(|p| p.plugin_id == "s")
                .ok_or("s studio plugin missing")?;
            let s_app = s_plugin
                .manifest
                .apps
                .iter()
                .find(|app| app.id == S_HOME_APP_ID)
                .or_else(|| s_plugin.manifest.apps.first())
                .ok_or("s home app missing")?
                .clone();
            let programs = self.build_studio_programs();
            let panel_state = StudioPanelState {
                active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
                programs,
                spawned_apps: vec![],
                active_spawned_id: None,
            };
            let instance_id = s_plugin.create_app(&s_app.id).await?;
            let view_state = ViewState {
                active_mode_id: s_app.default_mode_id.clone().or_else(|| s_app.modes.first().map(|m| m.id.clone())),
                active_window_kind_id: s_app.window_kinds.first().map(|w| w.id.clone()),
                selection_json: None,
                panel_json: Some(Self::panel_json(&panel_state)),
            };
            self.active_window_id = s_app.window_kinds.first().map(|w| w.id.clone());
            self.session = Some(ActiveSession {
                plugin_id: s_plugin.plugin_id.clone(),
                instance_id,
                app: s_app,
                view_state,
            });
        } else if let Some(plugin) = self.plugins.first() {
            let app = plugin
                .manifest
                .apps
                .first()
                .ok_or("plugin has no apps")?
                .clone();
            let instance_id = plugin.create_app(&app.id).await?;
            self.active_window_id = app.window_kinds.first().map(|w| w.id.clone());
            self.session = Some(ActiveSession {
                plugin_id: plugin.plugin_id.clone(),
                instance_id,
                app,
                view_state: ViewState {
                    active_mode_id: None,
                    active_window_kind_id: self.active_window_id.clone(),
                    selection_json: None,
                    panel_json: None,
                },
            });
        }
        self.sync_dock();
        self.sync_session_chrome();
        self.refresh_ui().await
    }

    fn sync_session_chrome(&mut self) {
        let Some(session) = &self.session else {
            return;
        };
        let examples = self
            .plugins
            .iter()
            .find(|p| p.plugin_id == session.plugin_id)
            .map(|p| p.manifest.examples.as_slice())
            .unwrap_or(&[]);
        if examples.is_empty() {
            self.active_example_id = None;
        } else {
            let current = self.active_example_id.clone();
            self.active_example_id = current
                .filter(|id| examples.iter().any(|ex| &ex.id == id))
                .or_else(|| examples.first().map(|ex| ex.id.clone()));
        }
        if let Some(mode_id) = session.view_state.active_mode_id.clone() {
            let _ = mode_id;
        }
    }

    fn active_plugin_examples(&self) -> Vec<ExampleDefinition> {
        let Some(session) = &self.session else {
            return Vec::new();
        };
        self.plugins
            .iter()
            .find(|p| p.plugin_id == session.plugin_id)
            .map(|p| p.manifest.examples.clone())
            .unwrap_or_default()
    }

    fn synthetic_panel_tab(id: &str, label: &str, group: &str) -> PanelTabDefinition {
        PanelTabDefinition {
            id: id.into(),
            label: label.into(),
            group: group.into(),
            body_key: String::new(),
        }
    }

    fn sync_dock(&mut self) {
        if let Some(session) = &self.session {
            if let Some(layout) = self.layout_override.clone() {
                self.dock.root = dock_from_window_layout(&layout.root);
                self.dock.active_window_id = self
                    .active_window_id
                    .clone()
                    .or_else(|| session.view_state.active_window_kind_id.clone());
            } else {
                self.dock = DockState::from_app(&session.app, self.active_window_id.as_deref());
            }
            if let Some(id) = &self.active_window_id {
                self.dock.sync_active_window(id);
            }
        }
    }

    fn persist_dock_layout(&mut self) {
        self.layout_override = Some(self.dock.to_window_layout());
        self.dock_drag_snapshot = None;
    }

    fn restore_dock_drag_snapshot(&mut self) {
        if let Some(layout) = self.dock_drag_snapshot.take() {
            self.layout_override = Some(layout);
            self.sync_dock();
        }
    }

    fn begin_pending_dock_drag(&mut self, payload: DockDragPayload, x: f32, y: f32) {
        self.dock_drag_snapshot = Some(self.dock.to_window_layout());
        self.pending_dock_drag = Some((payload, (x, y)));
    }

    fn promote_dock_drag(&mut self, x: f32, y: f32) {
        let Some((payload, _)) = self.pending_dock_drag.take() else {
            return;
        };
        self.dock.remove_window(&payload.window_id);
        self.dock_drag = Some(DockDragState {
            payload,
            x,
            y,
            drop_zone: None,
        });
    }

    fn dock_tab_bars_for_drop(
        &self,
        atlas: &mut FontAtlas,
        theme: &Theme,
        canvas: Rect,
        labels: &HashMap<String, String>,
    ) -> Vec<(Vec<usize>, Rect, Vec<f32>)> {
        self.dock
            .stack_tab_bar_rects(canvas, theme)
            .into_iter()
            .filter_map(|(path, rect)| {
                let windows = self.dock.stack_windows_at_path(&path)?;
                let widths: Vec<f32> = windows
                    .iter()
                    .map(|id| {
                        let label = labels.get(id).map(String::as_str).unwrap_or(id);
                        atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0
                    })
                    .collect();
                Some((path, rect, widths))
            })
            .collect()
    }

    pub async fn refresh_ui(&mut self) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        self.sync_dock();
        self.ensure_framework_panel_ui(&session);
        let plugin = self
            .plugins
            .iter()
            .find(|p| p.plugin_id == session.plugin_id)
            .ok_or("session plugin missing")?;
        self.window_ui.clear();
        for kind in &session.app.window_kinds {
            let node = plugin
                .render(session.instance_id, &kind.body_key, &session.view_state)
                .await?;
            self.window_ui.insert(kind.id.clone(), node);
        }
        self.panel_ui.clear();
        for tab in &session.app.panel_tabs {
            let node = plugin
                .render(session.instance_id, &tab.body_key, &session.view_state)
                .await?;
            self.panel_ui.insert(tab.id.clone(), node);
        }
        self.active_tools = plugin
            .tools(session.instance_id, &session.view_state)
            .await
            .unwrap_or_default();
        self.window_engagements = plugin
            .window_engagements(session.instance_id, &session.view_state)
            .await
            .unwrap_or_default();
        if self.studio_mode {
            if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
                if let Some(spawned) = panel
                    .active_spawned_id
                    .as_ref()
                    .and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id))
                {
                    if let Some(spawn_plugin) = self.plugins.iter().find(|p| p.plugin_id == spawned.plugin_id) {
                        let spawned_app = spawn_plugin
                            .manifest
                            .apps
                            .iter()
                            .find(|app| app.id == spawned.app_id);
                        if let Some(app) = spawned_app {
                            let body_key = app
                                .window_kinds
                                .first()
                                .map(|k| k.body_key.clone())
                                .unwrap_or_default();
                            let view_state = ViewState {
                                active_mode_id: app.default_mode_id.clone(),
                                active_window_kind_id: app.window_kinds.first().map(|w| w.id.clone()),
                                selection_json: None,
                                panel_json: None,
                            };
                            self.spawned_ui = Some(
                                spawn_plugin
                                    .render(spawned.instance_id, &body_key, &view_state)
                                    .await?,
                            );
                        }
                    }
                } else {
                    self.spawned_ui = None;
                }
            }
        }
        Ok(())
    }

    fn ensure_framework_panel_ui(&mut self, session: &ActiveSession) {
        let windows_ui = self.build_display_windows_ui(session);
        self.panel_ui
            .insert(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(), windows_ui);
        let layout_ui = self.build_display_layout_ui(session);
        self.panel_ui
            .insert(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID.into(), layout_ui);
        let settings_ui = self.build_settings_general_ui();
        self.panel_ui
            .insert(FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into(), settings_ui);
    }

    fn build_display_windows_ui(&self, session: &ActiveSession) -> UiNode {
        let items: Vec<UiNode> = session
            .app
            .window_kinds
            .iter()
            .map(|kind| {
                UiNode::Text(UiTextNode {
                    value: format!("{} — {}", kind.label, kind.id),
                    emphasize: None,
                    data_attributes: None,
                })
            })
            .collect();
        if items.is_empty() {
            return UiNode::Text(UiTextNode {
                value: "—".into(),
                emphasize: None,
                data_attributes: None,
            });
        }
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            children: items,
        })
    }

    fn build_display_layout_ui(&self, session: &ActiveSession) -> UiNode {
        let items: Vec<UiNode> = session
            .app
            .named_layouts
            .iter()
            .map(|layout| {
                UiNode::Button(UiButtonNode {
                    id: Some(format!("shell.layout.{}", layout.id)),
                    icon_id: layout.icon_id.clone().unwrap_or_else(|| "layout-grid".into()),
                    label: format!("{} ({})", layout.label, layout.origin),
                    command: CommandDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        command: "noop".into(),
                        args: None,
                    },
                    style: None,
                })
            })
            .collect();
        if items.is_empty() {
            return UiNode::Text(UiTextNode {
                value: "No saved layouts".into(),
                emphasize: None,
                data_attributes: None,
            });
        }
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            children: items,
        })
    }

    fn build_settings_general_ui(&self) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            children: vec![
                UiNode::Text(UiTextNode {
                    value: "General".into(),
                    emphasize: Some(true),
                    data_attributes: None,
                }),
                UiNode::Select(UiSelectNode {
                    id: "framework.settings.theme".into(),
                    value: self.theme_id.clone(),
                    items: vec![
                        UiSelectItem {
                            value: "system".into(),
                            label: "System".into(),
                        },
                        UiSelectItem {
                            value: "light".into(),
                            label: "Light".into(),
                        },
                        UiSelectItem {
                            value: "dark".into(),
                            label: "Dark".into(),
                        },
                    ],
                    placeholder: None,
                    on_change: CommandDescriptor {
                        controller_id: "framework".into(),
                        command: "setTheme".into(),
                        args: None,
                    },
                }),
                UiNode::Select(UiSelectNode {
                    id: "framework.settings.expertise".into(),
                    value: "standard".into(),
                    items: vec![
                        UiSelectItem {
                            value: "standard".into(),
                            label: "Standard".into(),
                        },
                        UiSelectItem {
                            value: "expert".into(),
                            label: "Expert".into(),
                        },
                    ],
                    placeholder: None,
                    on_change: CommandDescriptor {
                        controller_id: "framework".into(),
                        command: "setExpertise".into(),
                        args: None,
                    },
                }),
            ],
        })
    }
}
//#endregion ShellLifecycle

//#region ShellCommands
impl ShellState {
    pub async fn dispatch_command(&mut self, command: CommandDescriptor) -> Result<(), String> {
        if command.controller_id == "framework" {
            match command.command.as_str() {
                "setTheme" => {
                    if let Some(value) = command
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_str())
                    {
                        self.theme_id = value.to_string();
                    }
                    return Ok(());
                }
                "setExpertise" => {
                    if let Some(value) = command
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_str())
                    {
                        self.expertise = value.to_string();
                    }
                    return Ok(());
                }
                "setCompact" => {
                    if let Some(value) = command
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_bool())
                    {
                        self.compact_mode = value;
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let plugin = self
            .plugins
            .iter()
            .find(|p| {
                p.manifest
                    .apps
                    .iter()
                    .any(|app| app.controller_id == command.controller_id)
            })
            .or_else(|| self.plugins.iter().find(|p| p.plugin_id == session.plugin_id))
            .ok_or("command plugin missing")?;
        let command_json = serde_json::to_string(&command).map_err(|err| err.to_string())?;
        let ops = plugin
            .handle_command(session.instance_id, &command_json, &session.view_state)
            .await?;
        self.apply_ops(&ops).await
    }

    pub async fn apply_ops(&mut self, ops: &[String]) -> Result<(), String> {
        let mut view_state = self.session.as_ref().map(|s| s.view_state.clone());
        for op_json in ops {
            let op: serde_json::Value = serde_json::from_str(op_json).unwrap_or(serde_json::Value::Null);
            if op.get("op").and_then(|v| v.as_str()) == Some("setPanel") {
                if let Some(panel) = op.get("panel") {
                    if let Some(mut vs) = view_state.take() {
                        vs.panel_json = Some(panel.to_string());
                        view_state = Some(vs);
                    }
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("downloadMediaExport") {
                if let (Some(filename), Some(mime_type), Some(data)) = (
                    op.get("filename").and_then(|v| v.as_str()),
                    op.get("mimeType").and_then(|v| v.as_str()),
                    op.get("data").and_then(|v| v.as_str()),
                ) {
                    download_media_export(filename, mime_type, data);
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("spawnProgram") {
                if let (Some(program_id), Some(session)) = (op.get("programId").and_then(|v| v.as_str()), &self.session) {
                    self.spawn_program(program_id, session.view_state.clone()).await?;
                }
            }
        }
        if let (Some(mut session), Some(vs)) = (self.session.take(), view_state) {
            session.view_state = vs;
            self.session = Some(session);
            self.sync_session_chrome();
            self.refresh_ui().await?;
        }
        Ok(())
    }

    async fn spawn_program(&mut self, program_id: &str, mut view_state: ViewState) -> Result<(), String> {
        let programs = self.build_studio_programs();
        let Some(program) = programs.iter().find(|p| p.program_id == program_id).cloned() else {
            return Ok(());
        };
        let plugin = self
            .plugins
            .iter()
            .find(|p| p.plugin_id == program.plugin_id)
            .ok_or("spawn plugin missing")?;
        let instance_id = plugin.create_app(&program.app_id).await?;
        let mut panel = Self::panel_state_from_view(&view_state).unwrap_or(StudioPanelState {
            active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
            programs: programs.clone(),
            spawned_apps: vec![],
            active_spawned_id: None,
        });
        let spawned_id = format!("{}-{}", program.plugin_id, instance_id);
        panel.spawned_apps.push(SpawnedAppEntry {
            id: spawned_id.clone(),
            plugin_id: program.plugin_id.clone(),
            instance_id,
            app_id: program.app_id.clone(),
            label: program.label.clone(),
            hierarchy: program.hierarchy.clone(),
        });
        panel.active_spawned_id = Some(spawned_id);
        view_state.panel_json = Some(Self::panel_json(&panel));
        if let Some(session) = self.session.as_mut() {
            session.view_state = view_state;
        }
        Ok(())
    }
}
//#endregion ShellCommands

//#region ShellInput
impl ShellState {
    pub async fn handle_pointer_button(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        input: &mut InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.pointer_button = button;
        if !down {
            if self.dock_drag.is_some() {
                self.finish_dock_drag(x, y, input).await?;
            } else if let Some((payload, _)) = self.pending_dock_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if let Some(rest) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("dock.tab.")) {
                        if let Some((path_str_value, window_id)) = rest.split_once('.') {
                            if window_id == payload.window_id {
                                let path = parse_path(path_str_value);
                                self.dock.set_stack_active(&path, window_id);
                                self.active_window_id = Some(window_id.to_string());
                            }
                        }
                    }
                }
                self.restore_dock_drag_snapshot();
            }
            if self.tree_drag.is_some() {
                self.finish_tree_drag(x, y, input).await?;
            } else if let Some((item_id, _)) = self.pending_tree_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if hit.control_id.as_deref() == Some(&format!("tree.label.{item_id}")) {
                        self.dispatch_tree_selection(&item_id).await?;
                        if let Some(command) = hit.event.clone() {
                            self.dispatch_command(command).await?;
                        }
                    }
                }
            }
            if input.drag.active {
                let drag_target = input.drag.target_id.clone();
                self.dispatch_widget_drag_values(input).await?;
                input.end_drag();
                if drag_target
                    .as_deref()
                    .is_some_and(|id| id.starts_with("dock.split.") || id.starts_with("dock.corner."))
                {
                    self.persist_dock_layout();
                }
            }
            return Ok(());
        }
        if button == 2 {
            let hit = input.hit_at(x, y).cloned();
            self.open_context_menu(x, y, hit);
            self.right_click = RightClickState { pending: true, x, y };
            return Ok(());
        }
        if self.dismiss_overlays(x, y, input) {
            return Ok(());
        }
        if let Some(hit) = input.hit_at(x, y).cloned() {
            if hit.kind == HitKind::PanelResize {
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(window_id) = id.strip_prefix("shell.measures.resize.") {
                        self.measures_resize_window_id = Some(window_id.to_string());
                        self.measures_resize_origin_width = *self
                            .measures_width
                            .get(window_id)
                            .unwrap_or(&DEFAULT_MEASURES_RAIL_WIDTH);
                        input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal));
                        return Ok(());
                    }
                }
                let width = if hit.control_id.as_deref() == Some("panel.resize.left") {
                    self.left_panel_width
                } else {
                    self.right_panel_width
                };
                self.panel_resize_origin_width = width;
                input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal));
                return Ok(());
            }
            if hit.kind == HitKind::ScrollRegion {
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(rest) = id.strip_prefix("dock.corner.r/") {
                        if let Some((row_part, col_part)) = rest.split_once("/c/") {
                            if let Some((row_path_str, row_index_str)) = row_part.rsplit_once('/') {
                                if let Some((col_path_str, col_index_str)) = col_part.rsplit_once('/') {
                                    self.split_resize_path = Some(parse_path(row_path_str));
                                    self.split_resize_index = row_index_str.parse().unwrap_or(0);
                                    self.split_resize_secondary_path = Some(parse_path(col_path_str));
                                    self.split_resize_secondary_index = col_index_str.parse().unwrap_or(0);
                                    if let Some(path) = &self.split_resize_path {
                                        self.split_resize_origin = self.dock.begin_split_drag(path);
                                    }
                                    if let Some(path) = &self.split_resize_secondary_path {
                                        self.split_resize_secondary_origin = self.dock.begin_split_drag(path);
                                    }
                                    input.begin_drag(x, y, button, Some(id.to_string()), Some(DragAxis::Both));
                                    return Ok(());
                                }
                            }
                        }
                    }
                    if let Some(rest) = id.strip_prefix("dock.split.") {
                        if let Some((path_str, index_str)) = rest.rsplit_once('.') {
                            let path = parse_path(path_str);
                            let index: usize = index_str.parse().unwrap_or(0);
                            self.split_resize_path = Some(path.clone());
                            self.split_resize_index = index;
                            self.split_resize_origin = self.dock.begin_split_drag(&path);
                            input.begin_drag(x, y, button, Some(id.to_string()), hit.drag_axis);
                            return Ok(());
                        }
                    }
                }
            }
            if self.handle_shell_hit(&hit).await? {
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if let Some(rest) = id.strip_prefix("dock.tab.") {
                    if let Some((path_str_value, window_id)) = rest.split_once('.') {
                        let path = parse_path(path_str_value);
                        let tab_index = self.dock.tab_index(&path, window_id).unwrap_or(0);
                        let ghost_label = self
                            .session
                            .as_ref()
                            .and_then(|s| {
                                s.app
                                    .window_kinds
                                    .iter()
                                    .find(|k| k.id == window_id)
                                    .map(|k| k.label.clone())
                            })
                            .unwrap_or_else(|| window_id.to_string());
                        self.begin_pending_dock_drag(
                            DockDragPayload {
                                kind: DockDragKind::Tab,
                                window_id: window_id.to_string(),
                                source_path: path,
                                tab_index,
                                ghost_label,
                            },
                            x,
                            y,
                        );
                        return Ok(());
                    }
                }
                if let Some(path_str_value) = id.strip_prefix("dock.stack.") {
                    let path = parse_path(path_str_value);
                    let windows = self.dock.stack_windows_at_path(&path).unwrap_or_default();
                    let active = windows
                        .iter()
                        .find(|wid| self.active_window_id.as_deref() == Some(wid.as_str()))
                        .or_else(|| windows.first())
                        .cloned()
                        .unwrap_or_default();
                    if !active.is_empty() {
                        let tab_index = self.dock.tab_index(&path, &active).unwrap_or(0);
                        let ghost_label = self
                            .session
                            .as_ref()
                            .and_then(|s| {
                                s.app
                                    .window_kinds
                                    .iter()
                                    .find(|k| k.id == active)
                                    .map(|k| k.label.clone())
                            })
                            .unwrap_or_else(|| active.clone());
                        self.begin_pending_dock_drag(
                            DockDragPayload {
                                kind: DockDragKind::Stack,
                                window_id: active,
                                source_path: path,
                                tab_index,
                                ghost_label,
                            },
                            x,
                            y,
                        );
                        return Ok(());
                    }
                }
            }
            if hit.kind == HitKind::Slider {
                if let Some(id) = hit.control_id.clone() {
                    input.begin_drag(x, y, button, Some(id), hit.drag_axis);
                    return Ok(());
                }
            }
            if hit.kind == HitKind::Select || hit.kind == HitKind::Toggle || hit.kind == HitKind::DropdownItem {
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if id.contains(".vfs.") && !id.contains(".chevron.") {
                    if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.") {
                        let additive = input.modifiers.meta || input.modifiers.ctrl;
                        let shift = input.modifiers.shift;
                        let ordered = vec![row_id.to_string()];
                        let ids = vfs_selection_for_click(surface_id, row_id, &ordered, shift, additive);
                        self.dispatch_command(CommandDescriptor {
                            controller_id: self
                                .session
                                .as_ref()
                                .map(|s| s.app.controller_id.clone())
                                .unwrap_or_default(),
                            command: "selectRows".into(),
                            args: Some(serde_json::json!({ "surfaceId": surface_id, "ids": ids })),
                        })
                        .await?;
                        return Ok(());
                    }
                }
            }
            if let Some(drag_data) = hit.drag_data.clone() {
                if hit.control_id.as_deref().is_some_and(|id| id.starts_with("tree.label.")) {
                    if let Some(item_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                        self.tree_drag_origin = (x, y);
                        self.pending_tree_drag = Some((item_id.to_string(), drag_data));
                        return Ok(());
                    }
                }
            }
            if let Some(command) = hit.event.clone() {
                self.dispatch_command(command).await?;
            } else if hit.kind == HitKind::Input {
                if let Some(id) = &hit.control_id {
                    let seed = self
                        .widget_maps
                        .input_metas
                        .get(id)
                        .map(|meta| meta.value.as_str())
                        .unwrap_or("");
                    input.focus_input(id, seed);
                }
            }
        }
        self.flush_deferred_commands().await?;
        Ok(())
    }

    pub fn handle_pointer_move(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
    ) {
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.update_hover(x, y);
        self.update_tree_hover(input);
        if let Some((ref item_id, ref drag_data)) = self.pending_tree_drag {
            if down {
                let dx = x - self.tree_drag_origin.0;
                let dy = y - self.tree_drag_origin.1;
                if self.tree_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    self.tree_drag = Some(TreeDragState {
                        source_id: item_id.clone(),
                        drag_data: drag_data.clone(),
                        x,
                        y,
                        drop_target_id: None,
                        drop_position: TreeDropPosition::Inside,
                    });
                    self.pending_tree_drag = None;
                }
            }
        }
        if let Some(drag) = &mut self.tree_drag {
            drag.x = x;
            drag.y = y;
            if let Some(hit) = input.hit_at(x, y) {
                if let Some(target_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                    drag.drop_target_id = Some(target_id.to_string());
                    let rel = (y - hit.rect.y) / hit.rect.h.max(1.0);
                    drag.drop_position = if rel < 0.25 {
                        TreeDropPosition::Before
                    } else if rel > 0.75 {
                        TreeDropPosition::After
                    } else {
                        TreeDropPosition::Inside
                    };
                } else if hit.kind == HitKind::World3d || hit.kind == HitKind::Window {
                    drag.drop_target_id = hit.control_id.clone();
                    drag.drop_position = TreeDropPosition::Inside;
                } else {
                    drag.drop_target_id = None;
                }
            }
        }
        if let Some((payload, origin)) = &self.pending_dock_drag {
            if down {
                let dx = x - origin.0;
                let dy = y - origin.1;
                if self.dock_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    let payload = payload.clone();
                    let origin = *origin;
                    self.pending_dock_drag = None;
                    self.dock.remove_window(&payload.window_id);
                    self.dock_drag = Some(DockDragState {
                        payload,
                        x: origin.0,
                        y: origin.1,
                        drop_zone: None,
                    });
                }
            }
        }
        if let Some(drag) = &mut self.dock_drag {
            drag.x = x;
            drag.y = y;
            drag.drop_zone = compute_dock_drop_zone(
                x,
                y,
                &self.dock_drop_tab_bars,
                &self.dock_drop_bodies,
                self.dock_canvas_bounds,
            );
        }
        if input.drag.active {
            input.update_drag(x, y);
            if let Some(id) = input.drag.target_id.as_deref() {
                let dx = x - input.drag.start_x;
                let dy = y - input.drag.start_y;
                match id {
                    id if id.starts_with("shell.measures.resize.") => {
                        if let Some(window_id) = self.measures_resize_window_id.clone() {
                            let next = (self.measures_resize_origin_width + dx).clamp(160.0, 640.0);
                            self.measures_width.insert(window_id, next);
                        }
                    }
                    "panel.resize.left" => {
                        self.left_panel_width = (self.panel_resize_origin_width + dx)
                            .clamp(theme.panel_min_width, theme.panel_max_width);
                    }
                    "panel.resize.right" => {
                        self.right_panel_width = (self.panel_resize_origin_width - dx)
                            .clamp(theme.panel_min_width, theme.panel_max_width);
                    }
                    dock_id if dock_id.starts_with("dock.corner.") => {
                        if let Some(path) = self.split_resize_path.clone() {
                            self.dock.apply_split_drag_with_origin(
                                &path,
                                self.split_resize_index,
                                dx,
                                self.split_resize_axis_total,
                                &self.split_resize_origin,
                            );
                        }
                        if let Some(path) = self.split_resize_secondary_path.clone() {
                            self.dock.apply_split_drag_with_origin(
                                &path,
                                self.split_resize_secondary_index,
                                dy,
                                self.split_resize_secondary_axis_total,
                                &self.split_resize_secondary_origin,
                            );
                        }
                    }
                    dock_id if dock_id.starts_with("dock.split.") => {
                        if let (Some(path), axis) = (&self.split_resize_path, input.drag.axis) {
                            let delta = match axis {
                                Some(DragAxis::Horizontal) => dx,
                                Some(DragAxis::Vertical) => dy,
                                _ => dx,
                            };
                            self.dock.apply_split_drag_with_origin(
                                path,
                                self.split_resize_index,
                                delta,
                                self.split_resize_axis_total,
                                &self.split_resize_origin,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    async fn finish_dock_drag(
        &mut self,
        x: f32,
        y: f32,
        input: &InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        let Some(mut drag) = self.dock_drag.take() else {
            return Ok(());
        };
        drag.x = x;
        drag.y = y;
        if drag.drop_zone.is_none() {
            drag.drop_zone = compute_dock_drop_zone(
                x,
                y,
                &self.dock_drop_tab_bars,
                &self.dock_drop_bodies,
                self.dock_canvas_bounds,
            );
        }
        if let Some(zone) = drag.drop_zone {
            if self.dock.apply_drop(&drag.payload, &zone) {
                self.active_window_id = Some(drag.payload.window_id.clone());
                self.dock.sync_active_window(&drag.payload.window_id);
                self.persist_dock_layout();
            } else {
                self.restore_dock_drag_snapshot();
            }
        } else {
            self.restore_dock_drag_snapshot();
        }
        self.dock_drag_snapshot = None;
        let _ = input;
        Ok(())
    }

    pub fn handle_pointer_wheel(
        &mut self,
        x: f32,
        y: f32,
        delta: f32,
        input: &InputState<CommandDescriptor>,
    ) {
        if let Some(hit) = input.hit_at(x, y) {
            if hit.kind == HitKind::ScrollRegion {
                if let Some(id) = &hit.control_id {
                    let entry = self.scroll_offsets.entry(id.clone()).or_insert(0.0);
                    *entry = (*entry + delta * 24.0).max(0.0);
                }
            }
        }
    }

    pub async fn handle_world3d_input(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
        wheel_delta: f32,
        drag_dx: f32,
        drag_dy: f32,
    ) -> Result<(), String> {
        if wheel_delta.abs() > 0.0 {
            for state in self.world3d_states.values_mut() {
                if state.bounds.contains(x, y) {
                    handle_world3d_wheel(state, wheel_delta);
                }
            }
        }
        if (drag_dx.abs() > 0.0 || drag_dy.abs() > 0.0) && down {
            let modifiers = PointerModifiers { shift, ctrl, alt, meta };
            for state in self.world3d_states.values_mut() {
                if state.bounds.contains(x, y) {
                    handle_world3d_pointer_drag(state, x, y, drag_dx, drag_dy, button, &modifiers);
                }
            }
        }
        let mut commands = Vec::new();
        let modifiers = PointerModifiers { shift, ctrl, alt, meta };
        for state in self.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(command) = handle_world3d_pointer_button(state, x, y, down, button, &modifiers) {
                commands.push(command);
            }
            commands.extend(handle_world3d_paint_commands(state, x, y, down, button));
            if let Some(command) = handle_world3d_pointer_move(state, x, y, down, button) {
                commands.push(command);
            }
        }
        for command in commands {
            self.dispatch_command(command).await?;
        }
        Ok(())
    }

    pub async fn poll_world3d_assets(&mut self) {
        fetch_pending_glb_meshes(&mut self.world3d_states).await;
        fetch_pending_reference_images(&mut self.world3d_states).await;
    }

    async fn handle_shell_hit(&mut self, hit: &HitTarget<CommandDescriptor>) -> Result<bool, String> {
        let Some(id) = hit.control_id.as_deref() else {
            return Ok(false);
        };
        match id {
            "ui.nav.back" => {
                if self.uri_index > 0 {
                    self.uri_index -= 1;
                }
                return Ok(true);
            }
            "ui.nav.forward" => {
                if self.uri_index + 1 < self.uri_history.len() {
                    self.uri_index += 1;
                }
                return Ok(true);
            }
            "ui.nav.up" => {
                let uri = self.shell_uri();
                if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                    if !parent.is_empty() {
                        self.push_uri(parent);
                    }
                }
                return Ok(true);
            }
            "playground.navbar.fixture" => {
                self.overlay_state = OverlayState::Dropdown("example".to_string());
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.modes.") => {
                let mode_id = id.trim_start_matches("playground.navbar.modes.");
                if let Some(session) = self.session.as_mut() {
                    session.view_state.active_mode_id = Some(mode_id.to_string());
                }
                self.refresh_ui().await?;
                return Ok(true);
            }
            id if id.starts_with("framework.tool.collection.") => {
                let collection_id = id.trim_start_matches("framework.tool.collection.");
                let expanded = self
                    .tool_collection_expanded
                    .get(collection_id)
                    .copied()
                    .unwrap_or(false);
                self.tool_collection_expanded
                    .insert(collection_id.to_string(), !expanded);
                return Ok(true);
            }
            id if id.starts_with("shell.example.") => {
                let example_id = id.trim_start_matches("shell.example.");
                self.active_example_id = Some(example_id.to_string());
                self.overlay_state = OverlayState::None;
                if let Some(session) = &self.session {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        command: "setActiveExample".into(),
                        args: Some(serde_json::json!({ "exampleId": example_id })),
                    })
                    .await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.find.item.") => {
                let index: usize = id.trim_start_matches("shell.find.item.").parse().unwrap_or(0);
                self.activate_find_item(index).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.engagement.toggle.") => {
                let window_id = id.trim_start_matches("shell.engagement.toggle.");
                let expanded = self.engagement_expanded.get(window_id).copied().unwrap_or(false);
                self.engagement_expanded
                    .insert(window_id.to_string(), !expanded);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.fold.") => {
                let window_id = id.trim_start_matches("shell.measures.fold.");
                self.measures_folded.insert(window_id.to_string(), true);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.unfold.") => {
                let window_id = id.trim_start_matches("shell.measures.unfold.");
                self.measures_folded.insert(window_id.to_string(), false);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.focus.") => {
                let window_id = id.trim_start_matches("shell.measures.focus.");
                let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
                self.measures_expanded
                    .insert(window_id.to_string(), !expanded);
                return Ok(true);
            }
            "ui.search.toggle" => {
                self.search_open = !self.search_open;
                self.find_open = false;
                self.overlay_state = if self.search_open {
                    OverlayState::Search
                } else {
                    OverlayState::None
                };
                return Ok(true);
            }
            "ui.find.toggle" => {
                self.find_open = !self.find_open;
                self.search_open = false;
                self.overlay_state = if self.find_open {
                    OverlayState::Find
                } else {
                    OverlayState::None
                };
                return Ok(true);
            }
            "ui.panelToggle.display" => {
                if self.left_panel_open && self.active_left_kind == LeftPanelKind::Display {
                    self.left_panel_open = false;
                } else {
                    self.active_left_kind = LeftPanelKind::Display;
                    self.left_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.workbench" => {
                if self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench {
                    self.left_panel_open = false;
                } else {
                    self.active_left_kind = LeftPanelKind::Workbench;
                    self.left_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.details" => {
                if self.right_panel_open && self.active_right_kind == RightPanelKind::Details {
                    self.right_panel_open = false;
                } else {
                    self.active_right_kind = RightPanelKind::Details;
                    self.right_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.settings" => {
                if self.right_panel_open && self.active_right_kind == RightPanelKind::Settings {
                    self.right_panel_open = false;
                } else {
                    self.active_right_kind = RightPanelKind::Settings;
                    self.right_panel_open = true;
                }
                return Ok(true);
            }
            "ui.fullscreen.toggle" => {
                toggle_fullscreen();
                return Ok(true);
            }
            "studio.canvas.home" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "goHome".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            "studio.canvas.back" => {
                if let Some(session) = &self.session {
                    if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
                        if panel.active_spawned_id.is_some() {
                            self.dispatch_command(CommandDescriptor {
                                controller_id: S_PLAY_CONTROLLER_ID.into(),
                                command: "closeFocusedInstance".into(),
                                args: None,
                            })
                            .await?;
                        }
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") => {
                return Ok(true);
            }
            id if id.starts_with("dock.focus.") => {
                let path = parse_path(id.trim_start_matches("dock.focus."));
                self.dock.toggle_maximize(&path);
                self.persist_dock_layout();
                return Ok(true);
            }
            id if id.starts_with("dock.close.") => {
                let path = parse_path(id.trim_start_matches("dock.close."));
                if self.dock.close_active_in_stack(&path) {
                    self.active_window_id = self.dock.active_window_id.clone();
                    self.persist_dock_layout();
                }
                return Ok(true);
            }
            id if id.starts_with("shell.layout.") => {
                let layout_id = id.trim_start_matches("shell.layout.");
                if let Some(session) = &self.session {
                    if let Some(named) = session.app.named_layouts.iter().find(|entry| entry.id == layout_id) {
                        self.layout_override = Some(named.layout.clone());
                        self.sync_dock();
                        self.active_window_id = self.dock.active_window_id.clone();
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("shell.mode.") => {
                let mode_id = id.trim_start_matches("shell.mode.");
                self.dispatch_command(CommandDescriptor {
                    controller_id: self
                        .session
                        .as_ref()
                        .map(|s| s.app.controller_id.clone())
                        .unwrap_or_default(),
                    command: "setMode".into(),
                    args: Some(serde_json::json!({ "modeId": mode_id })),
                })
                .await?;
                return Ok(true);
            }
            id if id.starts_with("framework.settings.theme.") => {
                self.theme_id = id.trim_start_matches("framework.settings.theme.").to_string();
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.left.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.left.");
                self.select_left_panel_tab(tab_id).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.right.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.right.");
                self.active_right_tab = Some(tab_id.to_string());
                if self.studio_mode {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        command: "setActivePanelTab".into(),
                        args: Some(serde_json::json!({ "tabId": tab_id })),
                    })
                    .await?;
                }
                return Ok(true);
            }
            "framework.footer.undo" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "undo".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            "framework.footer.redo" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "redo".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            "framework.footer.checkpoint" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "commitCheckpoint".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            id if self.context_menu.as_ref().is_some_and(|menu| menu.items.iter().any(|item| item.id == id)) => {
                if let Some(menu) = &self.context_menu {
                    if let Some(item) = menu.items.iter().find(|item| item.id == id) {
                        if let Some(command) = item.command.clone() {
                            self.dispatch_command(command).await?;
                        }
                    }
                }
                self.context_menu = None;
                return Ok(true);
            }
            id if id.starts_with("shell.theme.") => {
                self.theme_id = id.trim_start_matches("shell.theme.").to_string();
                self.overlay_state = OverlayState::None;
                return Ok(true);
            }
            id if id.starts_with("section.chevron.") => {
                let section_id = id.trim_start_matches("section.chevron.");
                let key = format!("section.{section_id}");
                let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
                self.collapsed_sections.insert(key, !collapsed);
                return Ok(true);
            }
            id if id.starts_with("tree.chevron.") => {
                let item_id = id.trim_start_matches("tree.chevron.");
                let key = format!("tree.{item_id}");
                let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
                self.collapsed_sections.insert(key, !collapsed);
                return Ok(true);
            }
            id if id.contains(".vfs.chevron.") => {
                if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.chevron.") {
                    toggle_vfs_row_expanded(surface_id, row_id);
                    return Ok(true);
                }
            }
            id if self.widget_maps.select_metas.contains_key(id) => {
                let opening = !self.open_selects.get(id).copied().unwrap_or(false);
                for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
                    self.open_selects.insert(key, false);
                }
                self.open_selects.insert(id.to_string(), opening);
                return Ok(true);
            }
            id if id.contains(".item.") => {
                if let Some((select_id, value)) = id.rsplit_once(".item.") {
                    if let Some(cmd) = self.widget_maps.select_metas.get(select_id).cloned() {
                        self.open_selects.insert(select_id.to_string(), false);
                        self.dispatch_command(CommandDescriptor {
                            controller_id: cmd.controller_id,
                            command: cmd.command,
                            args: Some(serde_json::json!({ "value": value })),
                        })
                        .await?;
                        return Ok(true);
                    }
                }
            }
            id if self.widget_maps.toggle_metas.contains_key(id) => {
                if let Some((pressed, cmd)) = self.widget_maps.toggle_metas.get(id).cloned() {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: cmd.controller_id,
                        command: cmd.command,
                        args: Some(serde_json::json!({ "pressed": !pressed })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".minus") => {
                let base = id.trim_end_matches(".minus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: meta.on_delta.controller_id,
                        command: meta.on_delta.command,
                        args: Some(serde_json::json!({ "delta": -meta.step })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".plus") => {
                let base = id.trim_end_matches(".plus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: meta.on_delta.controller_id,
                        command: meta.on_delta.command,
                        args: Some(serde_json::json!({ "delta": meta.step })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.starts_with("tree.label.") => {
                let item_id = id.trim_start_matches("tree.label.");
                if hit.drag_data.is_some() {
                    return Ok(true);
                }
                self.queue_tree_selection(item_id);
                return Ok(false);
            }
            _ => {}
        }
        Ok(false)
    }

    async fn execute_search_item(&mut self, item_id: &str) -> Result<(), String> {
        if let Ok(index) = item_id.parse::<usize>() {
            self.activate_search_item(index).await?;
            return Ok(());
        }
        let items = self.filtered_search_items();
        if let Some(index) = items.iter().position(|item| item.id == item_id) {
            self.activate_search_item(index).await?;
        }
        Ok(())
    }

    fn update_tree_hover(&mut self, input: &InputState<CommandDescriptor>) {
        let hovered = input
            .hovered_id
            .as_deref()
            .and_then(|id| id.strip_prefix("tree.label."));
        if self.tree_hovered_id.as_deref() == hovered {
            return;
        }
        if let Some(prev) = self.tree_hovered_id.take() {
            if let Some(cmd) = self.widget_maps.tree_unhover_commands.get(&prev) {
                self.deferred_commands.push(cmd.clone());
            }
        }
        if let Some(id) = hovered {
            if let Some(cmd) = self.widget_maps.tree_hover_commands.get(id) {
                self.deferred_commands.push(cmd.clone());
            }
            self.tree_hovered_id = Some(id.to_string());
        }
    }

    fn queue_tree_selection(&mut self, item_id: &str) {
        let Some(cmd) = self.widget_maps.tree_selection_change.clone() else {
            return;
        };
        self.deferred_commands.push(CommandDescriptor {
            controller_id: cmd.controller_id,
            command: cmd.command,
            args: Some(serde_json::json!({ "ids": [item_id] })),
        });
    }

    async fn dispatch_tree_selection(&mut self, item_id: &str) -> Result<(), String> {
        self.queue_tree_selection(item_id);
        self.flush_deferred_commands().await
    }

    pub async fn flush_deferred_commands(&mut self) -> Result<(), String> {
        let commands = std::mem::take(&mut self.deferred_commands);
        for command in commands {
            self.dispatch_command(command).await?;
        }
        Ok(())
    }

    async fn dispatch_widget_drag_values(&mut self, input: &InputState<CommandDescriptor>) -> Result<(), String> {
        let Some(id) = input.drag.target_id.as_deref() else {
            return Ok(());
        };
        if let Some(value) = self.widget_maps.slider_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.slider_metas.get(id).cloned() {
                self.dispatch_command(CommandDescriptor {
                    controller_id: meta.on_change.controller_id,
                    command: meta.on_change.command,
                    args: Some(serde_json::json!({ "value": value })),
                })
                .await?;
            }
        } else if let Some(value) = self.widget_maps.ring_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.ring_metas.get(id).cloned() {
                self.dispatch_command(CommandDescriptor {
                    controller_id: meta.on_change.controller_id,
                    command: meta.on_change.command,
                    args: Some(serde_json::json!({ "value": value })),
                })
                .await?;
            }
        }
        Ok(())
    }

    async fn commit_focused_input(&mut self, input: &mut InputState<CommandDescriptor>) -> Result<(), String> {
        let Some(id) = input.focused_id.clone() else {
            return Ok(());
        };
        if let Some((vec3_id, axis)) = id.rsplit_once('.') {
            if let Ok(axis_index) = axis.parse::<usize>() {
                if axis_index < 3 {
                    if let Some(meta) = self.widget_maps.vec3_metas.get(vec3_id).cloned() {
                        let parsed = input.text_buffer.parse::<f64>().unwrap_or(0.0);
                        let mut value = meta.value;
                        value[axis_index] = parsed;
                        self.dispatch_command(CommandDescriptor {
                            controller_id: meta.on_change.controller_id,
                            command: meta.on_change.command,
                            args: Some(serde_json::json!({ "value": value })),
                        })
                        .await?;
                        input.blur_input();
                        return Ok(());
                    }
                }
            }
        }
        if id.ends_with(".input") {
            let base = id.trim_end_matches(".input");
            if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                let parsed = input.text_buffer.parse::<f64>().unwrap_or(meta.value);
                self.dispatch_command(CommandDescriptor {
                    controller_id: meta.on_absolute.controller_id,
                    command: meta.on_absolute.command,
                    args: Some(serde_json::json!({ "value": parsed })),
                })
                .await?;
                input.blur_input();
                return Ok(());
            }
        }
        if let Some(meta) = self.widget_maps.input_metas.get(&id).cloned() {
            self.dispatch_command(CommandDescriptor {
                controller_id: meta.on_change.controller_id,
                command: meta.on_change.command,
                args: Some(serde_json::json!({ "value": input.text_buffer })),
            })
            .await?;
            input.blur_input();
        }
        Ok(())
    }

    async fn finish_tree_drag(
        &mut self,
        x: f32,
        y: f32,
        input: &InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        let Some(drag) = self.tree_drag.take() else {
            return Ok(());
        };
        if let Some(hit) = input.hit_at(x, y) {
            if hit.kind == HitKind::World3d || hit.kind == HitKind::Window {
                if let Some(raw) = drag.drag_data.get("application/x-semio-catalogue-item") {
                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(raw) {
                        let program_id = payload.get("programId").and_then(|v| v.as_str());
                        let app_id = payload.get("appId").and_then(|v| v.as_str());
                        if let (Some(program_id), Some(app_id)) = (program_id, app_id) {
                            self.dispatch_command(CommandDescriptor {
                                controller_id: S_PLAY_CONTROLLER_ID.into(),
                                command: "spawnApp".into(),
                                args: Some(serde_json::json!({
                                    "programId": program_id,
                                    "appId": app_id,
                                    "position": { "x": x, "y": y },
                                })),
                            })
                            .await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render_tree_drag_overlay(&self, overlay: &mut DrawList, input: &InputState<CommandDescriptor>, theme: &Theme) {
        let Some(drag) = &self.tree_drag else {
            return;
        };
        overlay.push_solid(
            [drag.x - 60.0, drag.y - 12.0, 120.0, 24.0],
            theme.selected.with_alpha(0.85),
        );
        if let Some(hit) = input.hit_at(drag.x, drag.y) {
            if let Some(target_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                let _ = target_id;
                match drag.drop_position {
                    TreeDropPosition::Before => overlay.push_solid(
                        [hit.rect.x, hit.rect.y, hit.rect.w, 2.0],
                        theme.accent,
                    ),
                    TreeDropPosition::After => overlay.push_solid(
                        [hit.rect.x, hit.rect.y + hit.rect.h - 2.0, hit.rect.w, 2.0],
                        theme.accent,
                    ),
                    TreeDropPosition::Inside => overlay.push_rounded(
                        [hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h],
                        theme.accent.with_alpha(0.15),
                        theme.border_radius,
                    ),
                }
            }
        }
    }

    async fn select_left_panel_tab(&mut self, tab_id: &str) -> Result<(), String> {
        if self.studio_mode {
            if let Some(session) = &self.session {
                if session.app.id == S_PLAY_APP_ID {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        command: "setActivePanelTab".into(),
                        args: Some(serde_json::json!({ "tabId": tab_id })),
                    })
                    .await?;
                }
            }
        }
        Ok(())
    }

    fn dismiss_overlays(&mut self, x: f32, y: f32, input: &InputState<CommandDescriptor>) -> bool {
        let hit = input.hit_at(x, y);
        let on_overlay = hit.is_some_and(|h| {
            matches!(
                h.kind,
                HitKind::ContextMenu | HitKind::DropdownItem | HitKind::NavbarItem | HitKind::Select
            )
        });
        if self.open_selects.values().any(|open| *open) && !on_overlay {
            for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
                self.open_selects.insert(key, false);
            }
            return true;
        }
        if self.context_menu.is_some() && !on_overlay {
            self.context_menu = None;
            return true;
        }
        if self.overlay_state != OverlayState::None && !on_overlay {
            self.overlay_state = OverlayState::None;
            self.search_open = false;
            self.find_open = false;
            return true;
        }
        false
    }

    fn open_context_menu(&mut self, x: f32, y: f32, hit: Option<HitTarget<CommandDescriptor>>) {
        let node_id = hit.as_ref().and_then(|hit| {
            hit.control_id.as_deref().and_then(|id| {
                id.rsplit_once(".node.").map(|(_, node_id)| node_id.to_string())
            })
        });
        let mut items = take_context_menu_items()
            .into_iter()
            .map(|mut item| {
                if let Some(command) = item.command.take() {
                    item.command = Some(resolve_graph_context_command(&command, node_id.as_deref()));
                }
                item
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            if let (Some(node_id), Some(session)) = (node_id.as_deref(), &self.session) {
                items.push(ContextMenuItem {
                    id: format!("shell.context.node.select.{node_id}"),
                    label: "Select node".into(),
                    command: Some(CommandDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        command: "setMediaNodeSelection".into(),
                        args: Some(serde_json::json!({ "nodeIds": [node_id] })),
                    }),
                });
            }
        }
        if items.is_empty() {
            items = vec![
                ContextMenuItem {
                    id: "shell.context.copy".into(),
                    label: "Copy".into(),
                    command: None,
                },
                ContextMenuItem {
                    id: "shell.context.paste".into(),
                    label: "Paste".into(),
                    command: None,
                },
            ];
        }
        if self.studio_mode {
            items.push(ContextMenuItem {
                id: "shell.context.home".into(),
                label: "Go Home".into(),
                command: Some(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "goHome".into(),
                    args: None,
                }),
            });
        }
        self.context_menu = Some(ContextMenuState { x, y, items });
        self.overlay_state = OverlayState::None;
    }

    fn build_search_items(&self) -> Vec<SearchPaletteItem> {
        let Some(session) = &self.session else {
            return Vec::new();
        };
        let mut items = Vec::new();
        for tab in &session.app.panel_tabs {
            items.push(SearchPaletteItem {
                id: format!("panel.{}", tab.id),
                label: tab.label.clone(),
                group: "Panels".into(),
                command: Some(CommandDescriptor {
                    controller_id: session.app.controller_id.clone(),
                    command: "setActivePanelTab".into(),
                    args: Some(serde_json::json!({ "tabId": tab.id })),
                }),
                action: None,
            });
        }
        for kind in &session.app.window_kinds {
            items.push(SearchPaletteItem {
                id: format!("window.{}", kind.id),
                label: kind.label.clone(),
                group: "Windows".into(),
                command: None,
                action: Some(format!("window:{}", kind.id)),
            });
        }
        for binding in &session.app.keybindings {
            items.push(SearchPaletteItem {
                id: format!("keybinding.{}", binding.keys),
                label: binding.command.command.clone(),
                group: "Commands".into(),
                command: Some(binding.command.clone()),
                action: None,
            });
        }
        if self.studio_mode {
            for cmd in ["undo", "redo", "commitCheckpoint"] {
                items.push(SearchPaletteItem {
                    id: format!("studio.{cmd}"),
                    label: cmd.into(),
                    group: "Studio".into(),
                    command: Some(CommandDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        command: cmd.into(),
                        args: None,
                    }),
                    action: None,
                });
            }
        }
        items
    }

    fn filtered_search_items(&self) -> Vec<SearchPaletteItem> {
        let query = self.search_query.to_lowercase();
        let items = self.build_search_items();
        if query.trim().is_empty() {
            return items.into_iter().take(20).collect();
        }
        items
            .into_iter()
            .filter(|item| {
                item.label.to_lowercase().contains(&query)
                    || item.group.to_lowercase().contains(&query)
            })
            .take(20)
            .collect()
    }

    fn filtered_find_items(&self) -> Vec<ShellFindItem> {
        let query = self.find_query.to_lowercase();
        if query.trim().is_empty() {
            return self.find_items.iter().take(20).cloned().collect();
        }
        self.find_items
            .iter()
            .filter(|item| {
                item.label.to_lowercase().contains(&query)
                    || item
                        .description
                        .as_ref()
                        .is_some_and(|d| d.to_lowercase().contains(&query))
            })
            .take(20)
            .cloned()
            .collect()
    }

    pub async fn activate_search_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.filtered_search_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(command) = item.command.clone() {
            self.dispatch_command(command).await?;
        } else if let Some(action) = &item.action {
            if let Some(window_id) = action.strip_prefix("window:") {
                self.active_window_id = Some(window_id.to_string());
            }
        }
        self.search_open = false;
        self.overlay_state = OverlayState::None;
        self.search_query.clear();
        self.search_selected = 0;
        Ok(())
    }

    pub async fn activate_find_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.filtered_find_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(session) = &self.session {
            self.dispatch_command(CommandDescriptor {
                controller_id: session.app.controller_id.clone(),
                command: "setMediaNodeSelection".into(),
                args: Some(serde_json::json!({
                    "surfaceId": item.surface_id,
                    "nodeIds": [item.node_id],
                })),
            })
            .await?;
        }
        self.find_open = false;
        self.overlay_state = OverlayState::None;
        self.find_query.clear();
        self.find_selected = 0;
        Ok(())
    }

    pub fn handle_keyboard(
        &mut self,
        action: ui_wgpu::KeyAction,
        modifiers: &ui_wgpu::PointerModifiers,
        input: &mut InputState<CommandDescriptor>,
    ) {
        if action == ui_wgpu::KeyAction::Escape {
            if self.dock_drag.take().is_some() || self.pending_dock_drag.take().is_some() {
                self.restore_dock_drag_snapshot();
                self.dock_drag_snapshot = None;
                return;
            }
        }
        let meta = modifiers.meta || modifiers.ctrl;
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("p")) {
            self.search_open = !self.search_open;
            self.find_open = false;
            self.overlay_state = if self.search_open {
                OverlayState::Search
            } else {
                OverlayState::None
            };
            if self.search_open {
                input.focused_id = Some("shell.search.input".into());
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("f")) {
            self.find_open = !self.find_open;
            self.search_open = false;
            self.overlay_state = if self.find_open {
                OverlayState::Find
            } else {
                OverlayState::None
            };
            if self.find_open {
                input.focused_id = Some("shell.find.input".into());
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c == "[") {
            if self.uri_index > 0 {
                self.uri_index -= 1;
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c == "]") {
            if self.uri_index + 1 < self.uri_history.len() {
                self.uri_index += 1;
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::ArrowUp) {
            let uri = self.shell_uri();
            if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                if !parent.is_empty() {
                    self.push_uri(parent);
                }
            }
            return;
        }
        if meta && modifiers.shift && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("b")) {
            self.right_panel_open = !self.right_panel_open;
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("b")) {
            self.left_panel_open = !self.left_panel_open;
            return;
        }
        let palette_open = matches!(
            self.overlay_state,
            OverlayState::Search | OverlayState::Find
        );
        if palette_open {
            match action {
                ui_wgpu::KeyAction::Escape => {
                    self.overlay_state = OverlayState::None;
                    self.search_open = false;
                    self.find_open = false;
                    input.focused_id = None;
                }
                ui_wgpu::KeyAction::ArrowDown => {
                    if self.overlay_state == OverlayState::Search {
                        let len = self.filtered_search_items().len();
                        if len > 0 {
                            self.search_selected = (self.search_selected + 1).min(len - 1);
                        }
                    } else {
                        let len = self.filtered_find_items().len();
                        if len > 0 {
                            self.find_selected = (self.find_selected + 1).min(len - 1);
                        }
                    }
                }
                ui_wgpu::KeyAction::ArrowUp => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_selected = self.search_selected.saturating_sub(1);
                    } else {
                        self.find_selected = self.find_selected.saturating_sub(1);
                    }
                }
                ui_wgpu::KeyAction::Enter => {
                    let runtime = ();
                    let _ = runtime;
                }
                ui_wgpu::KeyAction::Char(key) => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_query.push_str(&key);
                        self.search_selected = 0;
                    } else {
                        self.find_query.push_str(&key);
                        self.find_selected = 0;
                    }
                }
                ui_wgpu::KeyAction::Backspace => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_query.pop();
                        self.search_selected = 0;
                    } else {
                        self.find_query.pop();
                        self.find_selected = 0;
                    }
                }
                _ => {}
            }
            return;
        }
        if input.focused_id.is_some() {
            match action {
                ui_wgpu::KeyAction::Char(key) => input.text_buffer.push_str(&key),
                ui_wgpu::KeyAction::Backspace => input.backspace(),
                ui_wgpu::KeyAction::Delete => input.delete_forward(),
                _ => {}
            }
        }
    }

    pub async fn handle_keyboard_async(
        &mut self,
        action: ui_wgpu::KeyAction,
        modifiers: &ui_wgpu::PointerModifiers,
        input: &mut InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        if matches!(self.overlay_state, OverlayState::Search) && action == ui_wgpu::KeyAction::Enter {
            self.activate_search_item(self.search_selected).await?;
            return Ok(());
        }
        if matches!(self.overlay_state, OverlayState::Find) && action == ui_wgpu::KeyAction::Enter {
            self.activate_find_item(self.find_selected).await?;
            return Ok(());
        }
        if input.focused_id.is_some() {
            match action {
                ui_wgpu::KeyAction::Enter | ui_wgpu::KeyAction::Escape => {
                    self.commit_focused_input(input).await?;
                    return Ok(());
                }
                _ => {}
            }
        }
        self.handle_keyboard(action, modifiers, input);
        Ok(())
    }

    fn push_uri(&mut self, uri: String) {
        self.uri_history.truncate(self.uri_index + 1);
        self.uri_history.push(uri);
        self.uri_index = self.uri_history.len().saturating_sub(1);
    }
}
//#endregion ShellInput

fn chrome_text(
    target: &mut DrawList,
    atlas: &mut FontAtlas,
    input: &mut InputState<CommandDescriptor>,
    theme: &Theme,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Rgba,
) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = framework_widget_context(
        target,
        None,
        atlas,
        None,
        input,
        theme,
        &mut scroll,
        &mut collapsed,
        &mut selects,
        None,
    );
    draw_text(&mut ctx, text, x, y, size, color);
}

fn chrome_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

fn chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    push_chrome_group_border(draw, rect, theme);
}

struct ChromeGroupItem<'a> {
    control_id: &'a str,
    icon_id: Option<&'a str>,
    label: Option<&'a str>,
    active: bool,
    kind: HitKind,
}

fn measure_chrome_group_item(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> f32 {
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let text_w = item
        .label
        .map(|label| atlas.measure_text(label, theme.font_size_small).0)
        .unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

fn render_chrome_group(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<CommandDescriptor>,
    theme: &Theme,
    rect: Rect,
    items: &[ChromeGroupItem<'_>],
) {
    if items.is_empty() {
        return;
    }
    let hair = theme.stroke_hairline;
    let inner_y = rect.y + hair;
    let inner_h = (rect.h - hair * 2.0).max(0.0);
    let mut x = rect.x;
    for (index, item) in items.iter().enumerate() {
        let item_w = measure_chrome_group_item(atlas, theme, item);
        let item_rect = Rect::new(x, inner_y, item_w, inner_h);
        let hovered = item_rect.contains(input.pointer_x, input.pointer_y);
        let bg = chrome_item_bg(theme, item.active, hovered);
        if bg.a > 0.0 {
            draw.push_solid([item_rect.x, item_rect.y, item_rect.w, item_rect.h], bg);
        }
        let mut content_x = item_rect.x + theme.padding_standard;
        if let Some(icon_id) = item.icon_id {
            let icon_color = chrome_item_text(theme, item.active, hovered);
            chrome_icon(
                draw,
                icons,
                icon_id,
                content_x,
                item_rect.y + (item_rect.h - CHROME_ICON_TINY) * 0.5,
                CHROME_ICON_TINY,
                icon_color,
            );
            content_x += CHROME_ICON_TINY + theme.gap_standard;
        }
        if let Some(label) = item.label {
            chrome_text(
                draw,
                atlas,
                input,
                theme,
                label,
                content_x,
                item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                chrome_item_text(theme, item.active, hovered),
            );
        }
        input.register_hit(HitTarget {
            rect: item_rect,
            event: None,
            control_id: Some(item.control_id.into()),
            kind: item.kind.clone(),
            drag_axis: None,
            drag_data: None,
        });
        x += item_w;
        if index + 1 < items.len() {
            draw.push_solid([x, inner_y, hair, inner_h], theme.border_normal);
        }
    }
    chrome_group_border(draw, rect, theme);
}

fn footer_tool_label<'a>(label: &'a Option<String>, text: &'a Option<String>, title: &'a Option<String>, id: &'a str) -> &'a str {
    title
        .as_deref()
        .or(label.as_deref())
        .or(text.as_deref())
        .unwrap_or(id)
}

fn render_footer_tool_nodes(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<CommandDescriptor>,
    theme: &Theme,
    mut x: f32,
    btn_y: f32,
    btn_h: f32,
    tools: &[ToolNode],
    collection_expanded: &HashMap<String, bool>,
) -> f32 {
    for tool in tools {
        match tool {
            ToolNode::Separator { .. } => {
                draw.push_solid(
                    [x + theme.gap_standard * 0.5, btn_y + 4.0, theme.stroke_hairline, btn_h - 8.0],
                    theme.border_normal,
                );
                x += theme.gap_standard;
            }
            ToolNode::Button {
                id,
                icon_id,
                label,
                text,
                title,
                disabled,
                on_press,
                ..
            } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = footer_tool_label(label, text, title, id);
                let item = ChromeGroupItem {
                    control_id: "framework.tool.button",
                    icon_id: Some(icon_id.as_str()),
                    label: Some(label_text),
                    active: false,
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item]);
                input.register_hit(HitTarget {
                    rect,
                    event: Some(on_press.clone()),
                    control_id: Some(format!("framework.tool.button.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
                x += item_w + theme.gap_standard * 0.5;
            }
            ToolNode::Toggle {
                id,
                icon_id,
                label,
                text,
                title,
                pressed,
                disabled,
                on_change,
                ..
            } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = footer_tool_label(label, text, title, id);
                let item = ChromeGroupItem {
                    control_id: "framework.tool.toggle",
                    icon_id: Some(icon_id.as_str()),
                    label: Some(label_text),
                    active: pressed.unwrap_or(false),
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item]);
                input.register_hit(HitTarget {
                    rect,
                    event: Some(on_change.clone()),
                    control_id: Some(format!("framework.tool.toggle.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
                x += item_w + theme.gap_standard * 0.5;
            }
            ToolNode::Collection {
                id,
                icon_id,
                label,
                text,
                title,
                disabled,
                children,
                ..
            } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let expanded = collection_expanded.get(id).copied().unwrap_or(false);
                let label_text = footer_tool_label(label, text, title, id);
                let item = ChromeGroupItem {
                    control_id: "framework.tool.collection",
                    icon_id: Some(icon_id.as_str()),
                    label: Some(label_text),
                    active: expanded,
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item]);
                input.register_hit(HitTarget {
                    rect,
                    event: None,
                    control_id: Some(format!("framework.tool.collection.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
                x += item_w + theme.gap_standard * 0.5;
                if expanded {
                    let leaves: Vec<ToolNode> = children
                        .iter()
                        .filter(|child| !matches!(child, ToolNode::Collection { .. }))
                        .cloned()
                        .collect();
                    x = render_footer_tool_nodes(
                        draw,
                        atlas,
                        icons,
                        input,
                        theme,
                        x,
                        btn_y,
                        btn_h,
                        &leaves,
                        collection_expanded,
                    );
                }
            }
        }
    }
    x
}

fn panel_tab_icon_id(tab: &PanelTabDefinition) -> &'static str {
    if tab.id == S_PLAY_CATALOGUE_TAB_ID || tab.group == "workbench" {
        return "library";
    }
    if tab.id.contains("parameters") {
        return "settings";
    }
    if tab.id.contains("inspector") || tab.id.contains("inspection") || tab.id == FRAMEWORK_PANEL_TAB_INSPECTION_ID {
        return "text-search";
    }
    if tab.id == FRAMEWORK_PANEL_TAB_HIERARCHY_ID {
        return "list-tree";
    }
    if tab.id == FRAMEWORK_DISPLAY_WINDOWS_TAB_ID {
        return "layout-grid";
    }
    if tab.id == FRAMEWORK_DISPLAY_LAYOUT_TAB_ID {
        return "layout";
    }
    if tab.id == FRAMEWORK_SETTINGS_GENERAL_TAB_ID {
        return "settings-2";
    }
    if tab.id == FRAMEWORK_PANEL_TAB_CATALOGUE_ID {
        return "library";
    }
    "circle-dot"
}

fn app_icon_id<'a>(app: &'a AppDefinition, icons: &IconAtlas) -> &'a str {
    if let Some(id) = app.icon_id.as_deref() {
        if icons.icon_uv(id).is_some() {
            return id;
        }
    }
    "component"
}

fn panel_toggle_icon_id(kind: &str, session: Option<&ActiveSession>) -> &'static str {
    match kind {
        "display" => "layout-grid",
        "workbench" => session
            .and_then(|s| s.app.panel_tabs.iter().find(|tab| tab.group == "left" || tab.group == "workbench"))
            .map(|tab| panel_tab_icon_id(tab))
            .unwrap_or("folder"),
        "details" => session
            .and_then(|s| s.app.panel_tabs.iter().find(|tab| tab.group == "right" || tab.group == "inspection"))
            .map(|tab| panel_tab_icon_id(tab))
            .unwrap_or("info"),
        "settings" => "settings-2",
        _ => "circle-dot",
    }
}

//#region ShellChrome
impl ShellState {
    pub fn render_chrome(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let w = self.screen_w;
        let h = self.screen_h;
        draw.set_screen_height(h);
        overlay.set_screen_height(h);
        overlay.clear();
        draw.push_solid([0.0, 0.0, w, h], theme.background);
        let body = self.body_rect(theme);
        FIND_ITEM_SINK.with(|cell| cell.borrow_mut().clear());
        CONTEXT_MENU_SINK.with(|cell| cell.borrow_mut().clear());
        clear_graph_node_context();
        self.widget_maps.clear_frame();
        let mut overlay_slot = Some(overlay);
        self.render_main_window(draw, &mut overlay_slot, atlas, icons, input, theme, body, gpu);
        self.find_items = take_find_items();
        if self.left_panel_open && self.has_left_tabs() {
            self.render_left_panel(draw, overlay_slot.as_deref_mut(), atlas, icons, input, theme, body, gpu);
        }
        if self.right_panel_open && self.has_right_tabs() {
            self.render_right_panel(draw, overlay_slot.as_deref_mut(), atlas, icons, input, theme, body, gpu);
        }
        self.render_navbar(draw, atlas, icons, input, theme, w);
        self.render_footer(draw, atlas, icons, input, theme, w, h);
        if let Some(overlay) = overlay_slot.as_deref_mut() {
            self.render_overlay(overlay, atlas, input, theme, w, h);
            self.render_tree_drag_overlay(overlay, input, theme);
        }
        if let Some(error) = &self.error {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let mut ctx = framework_widget_context(
                draw,
                None,
                atlas,
                Some(icons),
                input,
                theme,
                scroll_offsets,
                collapsed_sections,
                open_selects,
                None,
            );
            draw_text(
                &mut ctx,
                error,
                12.0,
                h - theme.footer_height - 24.0,
                theme.font_size_small,
                Rgba::new(0.95, 0.35, 0.35, 1.0),
            );
        }
    }

    fn body_rect(&self, theme: &Theme) -> Rect {
        Rect::new(
            0.0,
            theme.navbar_height,
            self.screen_w,
            self.screen_h - theme.navbar_height - theme.footer_height,
        )
    }

    fn shell_uri(&self) -> String {
        self.uri_history
            .get(self.uri_index)
            .cloned()
            .unwrap_or_else(|| {
                self.session.as_ref().map(|s| {
                    format!("os://{}/{}", s.plugin_id, s.app.id)
                }).unwrap_or_else(|| "os://home".into())
            })
    }

    fn panel_side_for_group(group: &str) -> &'static str {
        if group == "workbench" || group == "hierarchy" || group == "display" {
            "left"
        } else {
            "right"
        }
    }

    fn has_left_tabs(&self) -> bool {
        self.session.is_some()
    }

    fn has_right_tabs(&self) -> bool {
        self.session.is_some()
    }

    fn left_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
        match self.active_left_kind {
            LeftPanelKind::Display => vec![
                PanelTabDefinition {
                    id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(),
                    label: "Windows".into(),
                    group: "display".into(),
                    body_key: String::new(),
                },
                PanelTabDefinition {
                    id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID.into(),
                    label: "Layout".into(),
                    group: "display".into(),
                    body_key: String::new(),
                },
            ],
            LeftPanelKind::Workbench => {
                let mut tabs: Vec<PanelTabDefinition> = session
                    .app
                    .panel_tabs
                    .iter()
                    .filter(|tab| Self::panel_side_for_group(&tab.group) == "left")
                    .cloned()
                    .collect();
                let has_hierarchy = tabs.iter().any(|t| t.id == FRAMEWORK_PANEL_TAB_HIERARCHY_ID);
                if !has_hierarchy {
                    tabs.insert(
                        0,
                        PanelTabDefinition {
                            id: FRAMEWORK_PANEL_TAB_HIERARCHY_ID.into(),
                            label: "Hierarchy".into(),
                            group: "hierarchy".into(),
                            body_key: String::new(),
                        },
                    );
                }
                tabs
            }
        }
    }

    fn right_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
        match self.active_right_kind {
            RightPanelKind::Settings => vec![PanelTabDefinition {
                id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into(),
                label: "General".into(),
                group: "settings".into(),
                body_key: String::new(),
            }],
            RightPanelKind::Details => session
                .app
                .panel_tabs
                .iter()
                .filter(|tab| Self::panel_side_for_group(&tab.group) == "right")
                .cloned()
                .collect(),
        }
    }

    fn active_left_tab_id(&self, session: &ActiveSession) -> String {
        match self.active_left_kind {
            LeftPanelKind::Display => FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(),
            LeftPanelKind::Workbench => {
                if self.studio_mode && session.app.id == S_PLAY_APP_ID {
                    Self::panel_state_from_view(&session.view_state)
                        .map(|p| p.active_panel_tab)
                        .unwrap_or_else(|| S_PLAY_CATALOGUE_TAB_ID.into())
                } else {
                    self.left_tabs(session)
                        .first()
                        .map(|t| t.id.clone())
                        .unwrap_or_else(|| FRAMEWORK_PANEL_TAB_HIERARCHY_ID.into())
                }
            }
        }
    }

    fn active_right_tab_id(&self, session: &ActiveSession) -> String {
        if self.active_right_kind == RightPanelKind::Settings {
            return FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into();
        }
        if let Some(id) = &self.active_right_tab {
            return id.clone();
        }
        self.right_tabs(session)
            .first()
            .map(|t| t.id.clone())
            .unwrap_or_default()
    }

    fn has_display_tabs(&self) -> bool {
        self.session
            .as_ref()
            .is_some_and(|s| !s.app.window_kinds.is_empty())
    }

    fn floating_panel_rect(&self, left: bool, body: Rect, theme: &Theme) -> Rect {
        let inset = theme.panel_inset;
        if left {
            Rect::new(
                body.x + inset,
                body.y + inset,
                self.left_panel_width,
                body.h - inset * 2.0,
            )
        } else {
            Rect::new(
                body.x + body.w - inset - self.right_panel_width,
                body.y + inset,
                self.right_panel_width,
                body.h - inset * 2.0,
            )
        }
    }

    fn render_navbar(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        width: f32,
    ) {
        let navbar_rect = Rect::new(0.0, 0.0, width, theme.navbar_height);
        let navbar_hovered = navbar_rect.contains(input.pointer_x, input.pointer_y);
        draw.push_solid([0.0, 0.0, width, theme.navbar_height], theme.navbar);
        let border_color = if navbar_hovered {
            theme.border_emphasized
        } else {
            theme.border_normal
        };
        draw.push_solid(
            [0.0, theme.navbar_height - theme.stroke_hairline, width, theme.stroke_hairline],
            border_color,
        );
        let btn_h = theme.control_height;
        let btn_y = (theme.navbar_height - btn_h) * 0.5;
        let mut x = theme.padding_standard;
        let logo_size = btn_h - theme.gap_standard;
        chrome_icon(
            draw,
            icons,
            "semio-logo",
            x,
            btn_y + (btn_h - logo_size) * 0.5,
            logo_size,
            theme.text,
        );
        x += logo_size + theme.gap_standard;
        let title = self
            .session
            .as_ref()
            .map(|s| app_hierarchy_label(&s.app.hierarchy))
            .unwrap_or_else(|| if self.studio_mode { "semio · s · studio".into() } else { "semio · os".into() });
        chrome_text(
            draw,
            atlas,
            input,
            theme,
            &title,
            x,
            btn_y + (btn_h + theme.font_size_body) * 0.5 - 2.0,
            theme.font_size_body,
            theme.text,
        );
        x += atlas.measure_text(&title, theme.font_size_body).0 + theme.gap_standard * 2.0;
        let examples = self.active_plugin_examples();
        if !examples.is_empty() && !self.studio_mode {
            let active_label = examples
                .iter()
                .find(|ex| Some(&ex.id) == self.active_example_id.as_ref())
                .map(|ex| ex.label.as_str())
                .unwrap_or("Example");
            let fixture_w = atlas.measure_text(active_label, theme.font_size_small).0
                + theme.padding_standard * 2.0
                + theme.gap_standard;
            let fixture_rect = Rect::new(x, btn_y, fixture_w.max(120.0), btn_h);
            render_chrome_group(
                draw,
                atlas,
                icons,
                input,
                theme,
                fixture_rect,
                &[ChromeGroupItem {
                    control_id: "playground.navbar.fixture",
                    icon_id: None,
                    label: Some(active_label),
                    active: self.overlay_state == OverlayState::Dropdown("example".to_string()),
                    kind: HitKind::NavbarItem,
                }],
            );
            x += fixture_rect.w + theme.gap_standard;
        }
        let mut rx = width - theme.padding_standard;
        let fullscreen_item = ChromeGroupItem {
            control_id: "ui.fullscreen.toggle",
            icon_id: Some("maximize-2"),
            label: Some("Fullscreen"),
            active: false,
            kind: HitKind::Toggle,
        };
        let fullscreen_w = measure_chrome_group_item(atlas, theme, &fullscreen_item);
        rx -= fullscreen_w;
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(rx, btn_y, fullscreen_w, btn_h),
            &[fullscreen_item],
        );
        rx -= theme.gap_standard;
        let mut toggle_items: Vec<ChromeGroupItem<'_>> = Vec::new();
        if self.has_display_tabs() {
            toggle_items.push(ChromeGroupItem {
                control_id: "ui.panelToggle.display",
                icon_id: Some(panel_toggle_icon_id("display", self.session.as_ref())),
                label: Some("Display"),
                active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Display,
                kind: HitKind::Toggle,
            });
        }
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.workbench",
            icon_id: Some(panel_toggle_icon_id("workbench", self.session.as_ref())),
            label: Some("Workbench"),
            active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.details",
            icon_id: Some(panel_toggle_icon_id("details", self.session.as_ref())),
            label: Some("Details"),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Details,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.settings",
            icon_id: Some(panel_toggle_icon_id("settings", self.session.as_ref())),
            label: Some("Settings"),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Settings,
            kind: HitKind::Toggle,
        });
        let toggle_w: f32 = toggle_items
            .iter()
            .map(|item| measure_chrome_group_item(atlas, theme, item))
            .sum();
        rx -= toggle_w;
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(rx, btn_y, toggle_w, btn_h),
            &toggle_items,
        );
        rx -= theme.gap_standard;
        if let Some(session) = &self.session {
            if session.app.modes.len() > 1 {
                let mode_control_ids: Vec<String> = session
                    .app
                    .modes
                    .iter()
                    .rev()
                    .map(|mode| format!("playground.navbar.modes.{}", mode.id))
                    .collect();
                let mode_items: Vec<ChromeGroupItem<'_>> = session
                    .app
                    .modes
                    .iter()
                    .rev()
                    .zip(mode_control_ids.iter())
                    .map(|(mode, control_id)| {
                        let active_mode = session
                            .view_state
                            .active_mode_id
                            .as_deref()
                            .or(session.app.default_mode_id.as_deref())
                            .unwrap_or(&mode.id);
                        ChromeGroupItem {
                            control_id: control_id.as_str(),
                            icon_id: None,
                            label: Some(mode.label.as_str()),
                            active: active_mode == mode.id,
                            kind: HitKind::NavbarItem,
                        }
                    })
                    .collect();
                let mode_w: f32 = mode_items
                    .iter()
                    .map(|item| measure_chrome_group_item(atlas, theme, item))
                    .sum();
                rx -= mode_w;
                render_chrome_group(
                    draw,
                    atlas,
                    icons,
                    input,
                    theme,
                    Rect::new(rx, btn_y, mode_w, btn_h),
                    &mode_items,
                );
            }
        }
    }

    fn render_footer(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        width: f32,
        height: f32,
    ) {
        let y = height - theme.footer_height;
        let footer_rect = Rect::new(0.0, y, width, theme.footer_height);
        let footer_hovered = footer_rect.contains(input.pointer_x, input.pointer_y);
        draw.push_solid([0.0, y, width, theme.footer_height], theme.navbar);
        let border_color = if footer_hovered {
            theme.border_emphasized
        } else {
            theme.border_normal
        };
        draw.push_solid([0.0, y, width, theme.stroke_hairline], border_color);
        let session = match &self.session {
            Some(s) => s,
            None => return,
        };
        let btn_h = theme.control_height;
        let btn_y = y + (theme.footer_height - btn_h) * 0.5;
        let x = theme.padding_standard;
        let app_label = app_hierarchy_label(&session.app.hierarchy);
        let mut footer_items = vec![ChromeGroupItem {
            control_id: "framework.footer.app",
            icon_id: Some(app_icon_id(&session.app, icons)),
            label: Some(app_label.as_str()),
            active: false,
            kind: HitKind::Button,
        }];
        if self.studio_mode && session.app.controller_id == S_PLAY_CONTROLLER_ID {
            footer_items.extend([
                ChromeGroupItem {
                    control_id: "framework.footer.undo",
                    icon_id: Some("rotate-ccw"),
                    label: Some("Undo"),
                    active: false,
                    kind: HitKind::Button,
                },
                ChromeGroupItem {
                    control_id: "framework.footer.redo",
                    icon_id: Some("rotate-cw"),
                    label: Some("Redo"),
                    active: false,
                    kind: HitKind::Button,
                },
                ChromeGroupItem {
                    control_id: "framework.footer.checkpoint",
                    icon_id: Some("save"),
                    label: Some("Checkpoint"),
                    active: false,
                    kind: HitKind::Button,
                },
            ]);
        }
        let group_w: f32 = footer_items
            .iter()
            .map(|item| measure_chrome_group_item(atlas, theme, item))
            .sum();
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(x, btn_y, group_w, btn_h),
            &footer_items,
        );
        let mut tool_x = x + group_w + theme.gap_standard;
        tool_x = render_footer_tool_nodes(
            draw,
            atlas,
            icons,
            input,
            theme,
            tool_x,
            btn_y,
            btn_h,
            &self.active_tools,
            &self.tool_collection_expanded,
        );
        let _ = width;
        let _ = tool_x;
    }

    fn render_floating_panel(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        panel: Rect,
        tabs: &[PanelTabDefinition],
        active_tab_id: &str,
        side_left: bool,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let panel_hovered = panel.contains(input.pointer_x, input.pointer_y);
        let border_color = if panel_hovered {
            theme.border_emphasized
        } else {
            theme.border_normal
        };
        draw.push_rounded(
            [panel.x, panel.y, panel.w, panel.h],
            theme.glass_panel_fill(),
            theme.border_radius,
        );
        let hair = theme.stroke_hairline;
        draw.push_solid([panel.x, panel.y, panel.w, hair], border_color);
        draw.push_solid([panel.x, panel.y + panel.h - hair, panel.w, hair], border_color);
        draw.push_solid([panel.x, panel.y, hair, panel.h], border_color);
        draw.push_solid([panel.x + panel.w - hair, panel.y, hair, panel.h], border_color);
        let tab_bar_h = theme.panel_header_height;
        draw.push_solid(
            [panel.x, panel.y + tab_bar_h - hair, panel.w, hair],
            border_color,
        );
        let mut tab_x = panel.x;
        for (index, tab) in tabs.iter().enumerate() {
            let icon_id = panel_tab_icon_id(tab);
            let label_w = atlas.measure_text(&tab.label, theme.font_size_small).0;
            let tw = theme.padding_standard * 2.0 + CHROME_ICON_TINY + theme.gap_standard + label_w;
            let rect = Rect::new(tab_x, panel.y, tw, tab_bar_h);
            if index > 0 {
                draw.push_solid([tab_x, panel.y, hair, tab_bar_h], border_color);
            }
            let active = tab.id == active_tab_id;
            let hovered = rect.contains(input.pointer_x, input.pointer_y);
            let bg = if active {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.panel
            };
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
            let icon_x = rect.x + theme.padding_standard;
            let icon_y = rect.y + (rect.h - CHROME_ICON_TINY) * 0.5;
            chrome_icon(draw, icons, icon_id, icon_x, icon_y, CHROME_ICON_TINY, theme.text);
            chrome_text(
                draw,
                atlas,
                input,
                theme,
                &tab.label,
                icon_x + CHROME_ICON_TINY + theme.gap_standard,
                rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                if active || hovered {
                    theme.active_foreground
                } else {
                    theme.text
                },
            );
            let prefix = if side_left {
                "shell.panel.tab.left."
            } else {
                "shell.panel.tab.right."
            };
            input.register_hit(HitTarget {
                rect,
                event: None,
                control_id: Some(format!("{prefix}{}", tab.id)),
                kind: HitKind::PanelTab,
                drag_axis: None,
            drag_data: None,
            });
            tab_x += tw;
        }
        let content = Rect::new(
            panel.x + theme.gap_standard,
            panel.y + tab_bar_h,
            panel.w - theme.gap_standard * 2.0,
            panel.h - tab_bar_h - theme.gap_standard,
        );
        let scroll_key = format!(
            "panel.{}.{}",
            if side_left { "left" } else { "right" },
            active_tab_id
        );
        let scroll_y = *self.scroll_offsets.get(&scroll_key).unwrap_or(&0.0);
        draw.push_scissor(content);
        input.register_hit(HitTarget {
            rect: content,
            event: None,
            control_id: Some(scroll_key.clone()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
        drag_data: None,
        });
        if let Some(ui) = self.panel_ui.get(active_tab_id).cloned() {
            let scrolled = Rect::new(content.x, content.y - scroll_y, content.w, content.h);
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let widget_maps = &mut self.widget_maps;
            let mut ctx = framework_widget_context(
                draw,
                overlay,
                atlas,
                Some(icons),
                input,
                theme,
                scroll_offsets,
                collapsed_sections,
                open_selects,
                Some(widget_maps),
            );
            render_ui_node(&ui, scrolled, &mut ctx, gpu, &mut self.world3d_states);
        }
        draw.pop_scissor();
        let handle_w = 5.0;
        let resize_id = if side_left {
            "panel.resize.left"
        } else {
            "panel.resize.right"
        };
        let handle = if side_left {
            Rect::new(panel.x + panel.w - handle_w, panel.y, handle_w, panel.h)
        } else {
            Rect::new(panel.x, panel.y, handle_w, panel.h)
        };
        draw.push_solid([handle.x, handle.y, handle.w, handle.h], theme.separator);
        input.register_hit(HitTarget {
            rect: handle,
            event: None,
            control_id: Some(resize_id.into()),
            kind: HitKind::PanelResize,
            drag_axis: Some(DragAxis::Horizontal),
        drag_data: None,
        });
    }

    fn render_left_panel(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        body: Rect,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let tabs = self.left_tabs(&session);
        if tabs.is_empty() {
            return;
        }
        let active = self.active_left_tab_id(&session);
        let panel = self.floating_panel_rect(true, body, theme);
        self.render_floating_panel(
            draw,
            overlay,
            atlas,
            icons,
            input,
            theme,
            panel,
            &tabs,
            &active,
            true,
            gpu,
        );
    }

    fn render_right_panel(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        body: Rect,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let tabs = self.right_tabs(&session);
        if tabs.is_empty() {
            return;
        }
        let active = self.active_right_tab_id(&session);
        let panel = self.floating_panel_rect(false, body, theme);
        self.render_floating_panel(
            draw,
            overlay,
            atlas,
            icons,
            input,
            theme,
            panel,
            &tabs,
            &active,
            false,
            gpu,
        );
    }

    fn render_main_window(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.background);
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let mut canvas = bounds.inset(theme.panel_inset);
        canvas = self.render_studio_canvas_bars(draw, atlas, icons, input, theme, canvas, &session);
        if self.studio_mode {
            if let Some(spawned_ui) = self.spawned_ui.clone() {
                self.render_window_content(
                    draw, overlay.as_deref_mut(), atlas, icons, input, theme, canvas, &spawned_ui, "spawned", gpu,
                );
                return;
            }
        }
        let window_labels: HashMap<String, String> = session
            .app
            .window_kinds
            .iter()
            .map(|kind| {
                (
                    kind.id.clone(),
                    app_window_hierarchy_label(&session.app, &kind.label),
                )
            })
            .collect();
        self.split_resize_axis_total = canvas.w.max(canvas.h);
        self.dock_canvas_bounds = canvas;
        self.dock_drop_tab_bars = self.dock_tab_bars_for_drop(atlas, theme, canvas, &window_labels);
        self.dock_drop_bodies = self
            .dock
            .stack_body_rects(canvas, theme, &window_labels, atlas)
            .into_iter()
            .map(|(path, rect, active)| (path, rect, active))
            .collect();
        let mut dock_ctx = DockRenderContext {
            draw,
            atlas,
            icons,
            input,
            theme,
            window_labels: &window_labels,
        };
        self.dock.register_hits(&mut dock_ctx, canvas);
        let placements = self.dock.stack_body_rects(canvas, theme, &window_labels, atlas);
        let show_fallback = placements.is_empty();
        for (_, mut content, window_id) in placements {
            let window_kind = session
                .app
                .window_kinds
                .iter()
                .find(|kind| kind.id == window_id)
                .cloned();
            if let Some(kind) = window_kind {
                self.render_window_measures_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &mut content,
                    &window_id,
                    &kind,
                    gpu,
                );
                self.render_window_engagement_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &mut content,
                    &window_id,
                    &kind,
                    gpu,
                );
            }
            if let Some(ui) = self.window_ui.get(&window_id).cloned() {
                self.render_window_content(
                    draw, overlay.as_deref_mut(), atlas, icons, input, theme, content, &ui, &window_id, gpu,
                );
            }
        }
        if show_fallback {
            chrome_text(
                draw,
                atlas,
                input,
                theme,
                &app_hierarchy_label(&session.app.hierarchy),
                canvas.x + 16.0,
                canvas.y + 32.0,
                theme.font_size_body,
                theme.text_muted,
            );
        }
        if let Some(drag) = &self.dock_drag {
            let ghost = Rect::new(drag.x - 48.0, drag.y - 12.0, 120.0, theme.control_height);
            draw.push_rounded([ghost.x, ghost.y, ghost.w, ghost.h], theme.panel, theme.border_radius);
            chrome_text(
                draw,
                atlas,
                input,
                theme,
                &drag.payload.ghost_label,
                ghost.x + theme.padding_standard,
                ghost.y + (ghost.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                theme.text,
            );
        }
    }

    fn render_studio_canvas_bars(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        mut canvas: Rect,
        session: &ActiveSession,
    ) -> Rect {
        if !self.studio_mode || session.app.id != S_PLAY_APP_ID {
            return canvas;
        }
        let bar_h = theme.control_height;
        if self.spawned_ui.is_none() {
            let item = ChromeGroupItem {
                control_id: "studio.canvas.home",
                icon_id: Some("home"),
                label: Some("Home"),
                active: false,
                kind: HitKind::Button,
            };
            let bar_w = measure_chrome_group_item(atlas, theme, &item);
            let bar = Rect::new(canvas.x, canvas.y, bar_w, bar_h);
            render_chrome_group(draw, atlas, icons, input, theme, bar, &[item]);
            canvas.y += bar_h + theme.gap_standard;
            canvas.h -= bar_h + theme.gap_standard;
            return canvas;
        }
        if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
            if let Some(spawned) = panel
                .active_spawned_id
                .as_ref()
                .and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id))
            {
                let label = format!(
                    "Back to Media Graph · {}",
                    app_hierarchy_label(&spawned.hierarchy)
                );
                let item = ChromeGroupItem {
                    control_id: "studio.canvas.back",
                    icon_id: Some("chevron-left"),
                    label: Some(&label),
                    active: false,
                    kind: HitKind::Button,
                };
                let bar_w = measure_chrome_group_item(atlas, theme, &item).min(canvas.w);
                let bar = Rect::new(canvas.x, canvas.y, bar_w, bar_h);
                render_chrome_group(draw, atlas, icons, input, theme, bar, &[item]);
                canvas.y += bar_h + theme.gap_standard;
                canvas.h -= bar_h + theme.gap_standard;
            }
        }
        canvas
    }

    fn render_window_content(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        content: Rect,
        ui: &UiNode,
        window_id: &str,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let scroll_key = format!("window.{window_id}");
        let scroll_y = *self.scroll_offsets.get(&scroll_key).unwrap_or(&0.0);
        draw.push_scissor(content);
        input.register_hit(HitTarget {
            rect: content,
            event: None,
            control_id: Some(scroll_key.clone()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
        drag_data: None,
        });
        let scrolled = Rect::new(content.x, content.y - scroll_y, content.w, content.h);
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let widget_maps = &mut self.widget_maps;
        let mut ctx = framework_widget_context(
            draw,
            overlay,
            atlas,
            Some(icons),
            input,
            theme,
            scroll_offsets,
            collapsed_sections,
            open_selects,
            Some(widget_maps),
        );
        render_ui_node(ui, scrolled, &mut ctx, gpu, &mut self.world3d_states);
        draw.pop_scissor();
    }

    fn render_overlay(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        width: f32,
        height: f32,
    ) {
        if let Some(menu) = &self.context_menu {
            self.render_context_menu(overlay, atlas, input, theme, menu);
        }
        match &self.overlay_state {
            OverlayState::Search => {
                let items: Vec<(String, String, usize)> = self
                    .filtered_search_items()
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| (item.group, item.label, index))
                    .collect();
                self.render_command_list(
                    overlay,
                    atlas,
                    input,
                    theme,
                    width * 0.5 - 200.0,
                    theme.navbar_height + 8.0,
                    400.0,
                    height * 0.55,
                    "Search",
                    &self.search_query,
                    "shell.search.input",
                    self.search_selected,
                    &items,
                    "shell.search.item",
                );
            }
            OverlayState::Find => {
                let items: Vec<(String, String, usize)> = self
                    .filtered_find_items()
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| {
                        (
                            item.category.clone().unwrap_or_default(),
                            item.label.clone(),
                            index,
                        )
                    })
                    .collect();
                self.render_command_list(
                    overlay,
                    atlas,
                    input,
                    theme,
                    width * 0.5 - 200.0,
                    theme.navbar_height + 8.0,
                    400.0,
                    height * 0.55,
                    "Find in page",
                    &self.find_query,
                    "shell.find.input",
                    self.find_selected,
                    &items,
                    "shell.find.item",
                );
            }
            OverlayState::Dropdown(id) if id == "example" => {
                let examples = self.active_plugin_examples();
                let items: Vec<(String, String, usize)> = examples
                    .iter()
                    .enumerate()
                    .map(|(index, ex)| (String::new(), ex.label.clone(), index))
                    .collect();
                let id_items: Vec<(String, String, usize)> = examples
                    .iter()
                    .enumerate()
                    .map(|(index, ex)| (String::new(), ex.label.clone(), index))
                    .collect();
                let mapped: Vec<(String, String, usize)> = examples
                    .iter()
                    .enumerate()
                    .map(|(index, ex)| ("Examples".into(), ex.label.clone(), index))
                    .collect();
                self.render_example_dropdown(
                    overlay,
                    atlas,
                    input,
                    theme,
                    width * 0.25,
                    theme.navbar_height + 4.0,
                    220.0,
                    &mapped,
                    &examples,
                );
                let _ = (items, id_items);
            }
            OverlayState::Dropdown(_) => {}
            OverlayState::ThemeSelect => {}
            OverlayState::None => {}
        }
        for (id, open) in &self.open_selects {
            if *open {
                self.render_palette(overlay, atlas, input, theme, width * 0.4, theme.navbar_height + 40.0, 220.0, "Options", id);
            }
        }
    }

    fn render_example_dropdown(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
        w: f32,
        items: &[(String, String, usize)],
        examples: &[ExampleDefinition],
    ) {
        let row_h = theme.control_height;
        let h = items.len() as f32 * row_h + theme.padding_standard * 2.0;
        overlay.push_rounded([x, y, w, h.max(row_h + 8.0)], theme.overlay_bg, theme.border_radius);
        for (index, (_group, label, _)) in items.iter().enumerate() {
            let row = Rect::new(
                x + theme.gap_standard,
                y + theme.gap_standard + index as f32 * row_h,
                w - theme.gap_standard * 2.0,
                row_h,
            );
            let selected = examples
                .get(index)
                .is_some_and(|ex| self.active_example_id.as_deref() == Some(ex.id.as_str()));
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let bg = if selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                label,
                row.x + theme.padding_standard,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                if selected || hovered {
                    theme.active_foreground
                } else {
                    theme.text
                },
            );
            if let Some(example) = examples.get(index) {
                input.register_hit(HitTarget {
                    rect: row,
                    event: None,
                    control_id: Some(format!("shell.example.{}", example.id)),
                    kind: HitKind::DropdownItem,
                    drag_axis: None,
                drag_data: None,
                });
            }
        }
    }

    fn render_command_list(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        title: &str,
        query: &str,
        input_id: &str,
        selected: usize,
        items: &[(String, String, usize)],
        item_prefix: &str,
    ) {
        overlay.push_rounded([x, y, w, h], theme.overlay_bg, theme.border_radius);
        chrome_text(
            overlay,
            atlas,
            input,
            theme,
            title,
            x + 12.0,
            y + 20.0,
            theme.font_size_body,
            theme.text,
        );
        let filter_rect = Rect::new(x + 8.0, y + 32.0, w - 16.0, theme.control_height);
        overlay.push_rounded(
            [filter_rect.x, filter_rect.y, filter_rect.w, filter_rect.h],
            theme.input_bg,
            theme.border_radius,
        );
        let display_query = if query.is_empty() { "Type to filter…" } else { query };
        chrome_text(
            overlay,
            atlas,
            input,
            theme,
            display_query,
            filter_rect.x + 8.0,
            filter_rect.y + (filter_rect.h + theme.font_size_small) * 0.5 - 1.0,
            theme.font_size_small,
            if query.is_empty() {
                theme.text_muted
            } else {
                theme.text
            },
        );
        input.register_hit(HitTarget {
            rect: filter_rect,
            event: None,
            control_id: Some(input_id.into()),
            kind: HitKind::Input,
            drag_axis: None,
        drag_data: None,
        });
        let list_top = y + 32.0 + theme.control_height + 8.0;
        let list_h = h - (list_top - y) - 8.0;
        let mut row_y = list_top;
        let mut last_group = String::new();
        for (group, label, index) in items {
            if !group.is_empty() && group != &last_group {
                chrome_text(
                    overlay,
                    atlas,
                    input,
                    theme,
                    group,
                    x + 12.0,
                    row_y + 12.0,
                    theme.font_size_small,
                    theme.text_muted,
                );
                row_y += 18.0;
                last_group = group.clone();
            }
            let row = Rect::new(x + 8.0, row_y, w - 16.0, theme.control_height);
            if row_y + theme.control_height > list_top + list_h {
                break;
            }
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let is_selected = *index == selected;
            let bg = if is_selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                label,
                row.x + 8.0,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                if is_selected || hovered {
                    theme.active_foreground
                } else {
                    theme.text
                },
            );
            input.register_hit(HitTarget {
                rect: row,
                event: None,
                control_id: Some(format!("{item_prefix}.{index}")),
                kind: HitKind::DropdownItem,
                drag_axis: None,
            drag_data: None,
            });
            row_y += theme.control_height + 2.0;
        }
    }

    fn render_window_measures_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        content: &mut Rect,
        window_id: &str,
        kind: &semio_framework_core::WindowKindDefinition,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        if kind.measures.is_empty() {
            return;
        }
        let folded = self.measures_folded.get(window_id).copied().unwrap_or(false);
        let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
        let rail_w = *self
            .measures_width
            .get(window_id)
            .unwrap_or(&DEFAULT_MEASURES_RAIL_WIDTH);
        if folded {
            let item = ChromeGroupItem {
                control_id: "",
                icon_id: Some("chevron-left"),
                label: Some("Window Options"),
                active: false,
                kind: HitKind::Button,
            };
            let chip_w = measure_chrome_group_item(atlas, theme, &item);
            let chip = Rect::new(content.x, content.y + 8.0, chip_w, theme.control_height);
            render_chrome_group(draw, atlas, icons, input, theme, chip, &[item]);
            input.register_hit(HitTarget {
                rect: chip,
                event: None,
                control_id: Some(format!("shell.measures.unfold.{window_id}")),
                kind: HitKind::Button,
                drag_axis: None,
            drag_data: None,
            });
            return;
        }
        let width = if expanded { content.w * 0.45 } else { rail_w };
        let rail = Rect::new(content.x, content.y, width, content.h);
        draw.push_rounded([rail.x, rail.y, rail.w, rail.h], theme.panel, theme.border_radius);
        let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
        draw.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
        let focus_label = if expanded { "Unfocus" } else { "Focus" };
        let focus_item = ChromeGroupItem {
            control_id: "shell.measures.focus",
            icon_id: Some(if expanded { "minimize-2" } else { "maximize-2" }),
            label: Some(focus_label),
            active: false,
            kind: HitKind::Button,
        };
        let fold_item = ChromeGroupItem {
            control_id: "shell.measures.fold",
            icon_id: Some("chevron-right"),
            label: Some("Window Options"),
            active: false,
            kind: HitKind::Button,
        };
        let focus_w = measure_chrome_group_item(atlas, theme, &focus_item);
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(header.x, header.y, focus_w, header.h),
            &[focus_item],
        );
        input.register_hit(HitTarget {
            rect: Rect::new(header.x, header.y, focus_w, header.h),
            event: None,
            control_id: Some(format!("shell.measures.focus.{window_id}")),
            kind: HitKind::Button,
            drag_axis: None,
        drag_data: None,
        });
        let fold_w = measure_chrome_group_item(atlas, theme, &fold_item);
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(header.x + header.w - fold_w, header.y, fold_w, header.h),
            &[fold_item],
        );
        input.register_hit(HitTarget {
            rect: Rect::new(header.x + header.w - fold_w, header.y, fold_w, header.h),
            event: None,
            control_id: Some(format!("shell.measures.fold.{window_id}")),
            kind: HitKind::Button,
            drag_axis: None,
        drag_data: None,
        });
        let body = Rect::new(
            rail.x + theme.gap_standard,
            rail.y + theme.panel_header_height + theme.gap_standard,
            rail.w - theme.gap_standard * 2.0,
            rail.h - theme.panel_header_height - theme.gap_standard * 2.0,
        );
        for measure in &kind.measures {
            self.render_window_measure(
                draw,
                overlay,
                atlas,
                icons,
                input,
                theme,
                body,
                measure,
                gpu,
            );
        }
        if !expanded {
            let resize = Rect::new(rail.x + rail.w - 3.0, rail.y, 6.0, rail.h);
            input.register_hit(HitTarget {
                rect: resize,
                event: None,
                control_id: Some(format!("shell.measures.resize.{window_id}")),
                kind: HitKind::PanelResize,
                drag_axis: Some(DragAxis::Horizontal),
                drag_data: None,
            });
        }
        content.x += width + theme.gap_standard;
        content.w -= width + theme.gap_standard;
    }

    fn render_window_measure(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        measure: &WindowMeasure,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        use semio_framework_core::layout::MeasureSelectItem;
        use ui_wgpu::widgets::{render_widget, ControlNode, WidgetNode};
        let mut y = bounds.y;
        match measure {
            WindowMeasure::Group {
                id,
                label,
                default_open,
                children,
            } => {
                let open = !self.collapsed_sections.get(id).copied().unwrap_or(!default_open.unwrap_or(false));
                chrome_text(
                    draw,
                    atlas,
                    input,
                    theme,
                    &format!("{} {}", if open { "v" } else { ">" }, label),
                    bounds.x,
                    y + 14.0,
                    theme.font_size_small,
                    theme.text,
                );
                input.register_hit(HitTarget {
                    rect: Rect::new(bounds.x, y, bounds.w, theme.control_height),
                    event: None,
                    control_id: Some(format!("shell.measure.group.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                drag_data: None,
                });
                y += theme.control_height;
                if open {
                    for child in children {
                        self.render_window_measure(
                            draw, overlay, atlas, icons, input, theme,
                            Rect::new(bounds.x + 12.0, y, bounds.w - 12.0, bounds.h - (y - bounds.y)),
                            child, gpu,
                        );
                    }
                }
            }
            WindowMeasure::Select {
                id,
                label,
                value,
                items,
                on_change,
            } => {
                if let Some(label) = label {
                    chrome_text(draw, atlas, input, theme, label, bounds.x, y + 14.0, theme.font_size_small, theme.text_muted);
                }
                let node = WidgetNode::Select {
                    id: id.clone(),
                    value: value.clone(),
                    items: items
                        .iter()
                        .map(|item: &MeasureSelectItem| ui_wgpu::widgets::SelectItem {
                            value: item.value.clone(),
                            label: item.label.clone(),
                        })
                        .collect(),
                    placeholder: None,
                    on_change: Some(on_change.clone()),
                };
                let rect = Rect::new(bounds.x, y + 16.0, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(
                    draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
                    scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
                render_widget(&node, rect, &mut ctx);
            }
            WindowMeasure::Slider {
                id,
                label,
                value,
                min,
                max,
                step,
                on_change,
            } => {
                if let Some(label) = label {
                    chrome_text(draw, atlas, input, theme, label, bounds.x, y + 14.0, theme.font_size_small, theme.text_muted);
                }
                let node = WidgetNode::Slider {
                    id: id.clone(),
                    value: *value,
                    min: *min,
                    max: *max,
                    step: step.unwrap_or(0.01),
                    on_change: Some(on_change.clone()),
                };
                let rect = Rect::new(bounds.x, y + 16.0, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(
                    draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
                    scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
                render_widget(&node, rect, &mut ctx);
            }
            WindowMeasure::Toggle {
                id,
                icon_id,
                label,
                pressed,
                text,
                on_change,
            } => {
                let node = WidgetNode::Toggle {
                    id: id.clone(),
                    icon_id: icon_id.clone(),
                    pressed: *pressed,
                    text: text.clone().or(label.clone()),
                    on_change: Some(on_change.clone()),
                };
                let rect = Rect::new(bounds.x, y, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(
                    draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
                    scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
                render_widget(&node, rect, &mut ctx);
            }
        }
    }

    fn render_window_engagement_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        content: &mut Rect,
        window_id: &str,
        kind: &semio_framework_core::WindowKindDefinition,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let engagement = self
            .window_engagements
            .get(&kind.id)
            .cloned()
            .or_else(|| kind.engagement.clone());
        let Some(engagement) = engagement else {
            return;
        };
        let expanded = self.engagement_expanded.get(window_id).copied().unwrap_or(false);
        if !expanded {
            let item = ChromeGroupItem {
                control_id: "shell.engagement.toggle",
                icon_id: Some("chevron-right"),
                label: Some("Command"),
                active: false,
                kind: HitKind::Button,
            };
            let chip_w = measure_chrome_group_item(atlas, theme, &item);
            let chip = Rect::new(
                content.x + content.w - chip_w,
                content.y + 8.0,
                chip_w,
                theme.control_height,
            );
            render_chrome_group(draw, atlas, icons, input, theme, chip, &[item]);
            input.register_hit(HitTarget {
                rect: chip,
                event: None,
                control_id: Some(format!("shell.engagement.toggle.{window_id}")),
                kind: HitKind::Button,
                drag_axis: None,
            drag_data: None,
            });
            return;
        }
        let rail_w = DEFAULT_ENGAGEMENT_RAIL_WIDTH;
        let rail = Rect::new(
            content.x + content.w - rail_w,
            content.y,
            rail_w,
            content.h,
        );
        draw.push_rounded([rail.x, rail.y, rail.w, rail.h], theme.panel, theme.border_radius);
        let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
        draw.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
        let toggle_item = ChromeGroupItem {
            control_id: "shell.engagement.toggle",
            icon_id: Some("chevron-left"),
            label: Some("Command"),
            active: false,
            kind: HitKind::Button,
        };
        let toggle_w = measure_chrome_group_item(atlas, theme, &toggle_item);
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(header.x, header.y, toggle_w, header.h),
            &[toggle_item],
        );
        input.register_hit(HitTarget {
            rect: Rect::new(header.x, header.y, toggle_w, header.h),
            event: None,
            control_id: Some(format!("shell.engagement.toggle.{window_id}")),
            kind: HitKind::Button,
            drag_axis: None,
        drag_data: None,
        });
        let mut y = rail.y + theme.panel_header_height;
        if let Some(options) = &engagement.options {
            for option in options {
                let label = option.label.clone().unwrap_or_else(|| option.id.clone());
                let pressed = option.pressed.unwrap_or(false);
                let item = ChromeGroupItem {
                    control_id: "shell.engagement.option",
                    icon_id: None,
                    label: Some(&label),
                    active: pressed,
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(rail.x + 8.0, y, item_w, theme.control_height);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item]);
                if let Some(command) = &option.command {
                    input.register_hit(HitTarget {
                        rect,
                        event: Some(command.clone()),
                        control_id: Some(format!("shell.engagement.option.{}.{}", window_id, option.id)),
                        kind: HitKind::Button,
                        drag_axis: None,
                    drag_data: None,
                    });
                }
                y += theme.control_height + 4.0;
            }
        }
        if let Some(input_spec) = &engagement.input {
            self.render_engagement_input(
                draw, overlay, atlas, icons, input, theme,
                Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height * 2.0),
                window_id, input_spec, gpu,
            );
            y += theme.control_height * 2.0 + 8.0;
        }
        if let Some(control) = &engagement.control {
            self.render_engagement_control(
                draw, overlay, atlas, icons, input, theme,
                Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height),
                control, gpu,
            );
        }
        if let Some(status_rows) = &engagement.status {
            for row in status_rows {
                y += theme.control_height;
                chrome_text(
                    draw, atlas, input, theme, &row.text,
                    rail.x + 8.0, y, theme.font_size_small, theme.text_muted,
                );
            }
        }
        if let Some(possibles) = &engagement.possible_engagements {
            for possible in possibles {
                y += theme.control_height + 2.0;
                let rect = Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height);
                draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius);
                chrome_text(
                    draw, atlas, input, theme, &possible.label,
                    rect.x + 8.0, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0,
                    theme.font_size_small, theme.text,
                );
                if let Some(command) = &possible.command {
                    input.register_hit(HitTarget {
                        rect,
                        event: Some(command.clone()),
                        control_id: Some(format!("shell.engagement.possible.{}.{}", window_id, possible.id)),
                        kind: HitKind::Button,
                        drag_axis: None,
                    drag_data: None,
                    });
                }
            }
        }
        content.w -= rail_w + theme.gap_standard;
    }

    fn render_engagement_input(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        spec: &WindowEngagementInput,
        _gpu: &mut ui_wgpu::GpuContext,
    ) {
        let id = spec
            .id
            .clone()
            .unwrap_or_else(|| format!("engagement-input-{window_id}"));
        let value = self
            .engagement_inputs
            .get(&id)
            .cloned()
            .or_else(|| spec.value.clone())
            .unwrap_or_default();
        let node = ui_wgpu::widgets::WidgetNode::Input {
            id: id.clone(),
            input_kind: "text".into(),
            value,
            placeholder: spec.placeholder.clone(),
            commit: None,
            on_change: spec.on_change.clone(),
        };
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let mut ctx = framework_widget_context(
            draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
            scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
        ui_wgpu::widgets::render_widget(&node, bounds, &mut ctx);
    }

    fn render_engagement_control(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        control: &WindowEngagementControl,
        _gpu: &mut ui_wgpu::GpuContext,
    ) {
        use ui_wgpu::widgets::{render_widget, WidgetNode};
        let node = match control {
            WindowEngagementControl::Slider { id, value, min, max, step, on_change, .. } => {
                WidgetNode::Slider {
                    id: id.clone().unwrap_or_else(|| "engagement-slider".into()),
                    value: *value,
                    min: *min,
                    max: *max,
                    step: step.unwrap_or(0.01),
                    on_change: on_change.clone(),
                }
            }
            WindowEngagementControl::Stepper { id, value, step, on_change, .. } => {
                WidgetNode::NumberStepper {
                    id: id.clone().unwrap_or_else(|| "engagement-stepper".into()),
                    value: *value,
                    step: step.unwrap_or(1.0),
                    uniform: false,
                    on_absolute: on_change.clone(),
                    on_delta: on_change.clone(),
                }
            }
            WindowEngagementControl::Select { id, value, items, on_change, .. } => {
                WidgetNode::Select {
                    id: id.clone().unwrap_or_else(|| "engagement-select".into()),
                    value: value.clone().unwrap_or_default(),
                    items: items
                        .iter()
                        .map(|item| ui_wgpu::widgets::SelectItem {
                            value: item.value.clone(),
                            label: item.label.clone(),
                        })
                        .collect(),
                    placeholder: None,
                    on_change: on_change.clone(),
                }
            }
            WindowEngagementControl::Ring { id, value, on_select, .. } => {
                WidgetNode::Ring {
                    id: id.clone().unwrap_or_else(|| "engagement-ring".into()),
                    t: value
                        .as_ref()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.5),
                    disabled: false,
                    on_change: on_select.clone(),
                }
            }
            WindowEngagementControl::ToggleGroup { id, value, options, on_select, .. } => {
                let label = value
                    .clone()
                    .or_else(|| options.first().map(|o| o.id.clone()))
                    .unwrap_or_else(|| "toggle".into());
                WidgetNode::Toggle {
                    id: id.clone().unwrap_or_else(|| "engagement-toggle".into()),
                    icon_id: String::new(),
                    pressed: false,
                    text: Some(label),
                    on_change: on_select.clone(),
                }
            }
        };
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let mut ctx = framework_widget_context(
            draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
            scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
        render_widget(&node, bounds, &mut ctx);
    }

    fn render_context_menu(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        menu: &ContextMenuState,
    ) {
        let row_h = theme.control_height;
        let w = 180.0;
        let h = menu.items.len() as f32 * row_h + 8.0;
        let rect = Rect::new(menu.x, menu.y, w, h);
        overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.overlay_bg, theme.border_radius);
        for (index, item) in menu.items.iter().enumerate() {
            let row = Rect::new(rect.x + 4.0, rect.y + 4.0 + index as f32 * row_h, w - 8.0, row_h);
            overlay.push_rounded([row.x, row.y, row.w, row.h], theme.button, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, &item.label,
                row.x + 8.0,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                theme.text);
            input.register_hit(HitTarget {
                rect: row,
                event: item.command.clone(),
                control_id: Some(item.id.clone()),
                kind: HitKind::ContextMenu,
                drag_axis: None,
            drag_data: None,
            });
        }
    }

    fn render_theme_dropdown(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
    ) {
        let options = [("system", "System"), ("light", "Light"), ("dark", "Dark")];
        let row_h = theme.control_height;
        let w = 112.0;
        let h = options.len() as f32 * row_h + theme.padding_standard * 2.0;
        overlay.push_rounded([x, y, w, h], theme.overlay_bg, theme.border_radius);
        for (index, (value, label)) in options.iter().enumerate() {
            let row = Rect::new(
                x + theme.gap_standard,
                y + theme.gap_standard + index as f32 * row_h,
                w - theme.gap_standard * 2.0,
                row_h,
            );
            let selected = *value == self.theme_id;
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let bg = if selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, label,
                row.x + theme.padding_standard,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                if selected || hovered { theme.active_foreground } else { theme.text });
            input.register_hit(HitTarget {
                rect: row,
                event: None,
                control_id: Some(format!("shell.theme.{value}")),
                kind: HitKind::DropdownItem,
                drag_axis: None,
            drag_data: None,
            });
        }
    }

    fn render_palette(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
        w: f32,
        title: &str,
        hint: &str,
    ) {
        let h = 120.0;
        overlay.push_rounded([x, y, w, h], theme.overlay_bg, theme.border_radius);
        chrome_text(overlay, atlas, input, theme, title,
            x + 12.0,
            y + 24.0,
            theme.font_size_body,
            theme.text);
        if !hint.is_empty() {
            chrome_text(overlay, atlas, input, theme, hint,
                x + 12.0,
                y + 48.0,
                theme.font_size_small,
                theme.text_muted,);
        }
        let filter_rect = Rect::new(x + 8.0, y + h - theme.control_height - 8.0, w - 16.0, theme.control_height);
        overlay.push_rounded(
            [filter_rect.x, filter_rect.y, filter_rect.w, filter_rect.h],
            theme.input_bg,
            theme.border_radius,
        );
        input.register_hit(HitTarget {
            rect: filter_rect,
            event: None,
            control_id: Some(format!("shell.palette.{title}")),
            kind: HitKind::Input,
            drag_axis: None,
        drag_data: None,
        });
    }
}
//#endregion ShellChrome

#[cfg(target_arch = "wasm32")]
fn download_media_export(filename: &str, mime_type: &str, data: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, HtmlAnchorElement, Url};

    let window = match web_sys::window() {
        Some(window) => window,
        None => return,
    };
    let document = match window.document() {
        Some(document) => document,
        None => return,
    };
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(data));
    let blob = Blob::new_with_str_sequence(&parts).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap();
    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.set_attribute("type", mime_type).ok();
    anchor.click();
    Url::revoke_object_url(&url).ok();
}

#[cfg(not(target_arch = "wasm32"))]
fn download_media_export(_filename: &str, _mime_type: &str, _data: &str) {}

#[cfg(target_arch = "wasm32")]
fn toggle_fullscreen() {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    if document.fullscreen_element().is_some() {
        let _ = document.exit_fullscreen();
    } else if let Some(element) = document.document_element() {
        let _ = element
            .dyn_ref::<web_sys::HtmlElement>()
            .map(|el| el.request_fullscreen());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_fullscreen() {}
