//! 🖥️ OS shell chrome — navbar, panels, sessions, and studio mode.

use crate::draw::DrawList;
use crate::input::{HitKind, HitTarget, InputState};
use crate::plugin_bridge::{is_studio_mode, PluginBridgeEntry};
use crate::text::FontAtlas;
use crate::theme::{
    Rect, Rgba, BORDER_RADIUS, FONT_SIZE_BODY, FONT_SIZE_SMALL,
    NAVBAR_HEIGHT, PANEL_HEADER_HEIGHT,
};
use crate::widgets::{draw_text, render_ui_node, WidgetContext};
use semio_framework_core::{
    AppDefinition, CommandDescriptor, PanelTabDefinition, UiNode, ViewState,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const S_HOME_APP_ID: &str = "home";
const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioProgramEntry {
    pub plugin_id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
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
    pub error: Option<String>,
    pub screen_w: f32,
    pub screen_h: f32,
}

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
            error: None,
            screen_w: 1280.0,
            screen_h: 720.0,
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
        self.refresh_ui().await
    }

    pub async fn refresh_ui(&mut self) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
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

    pub async fn handle_pointer(&mut self, x: f32, y: f32, down: bool, input: &InputState) -> Result<(), String> {
        if !down {
            return Ok(());
        }
        if let Some(hit) = input.hit_at(x, y) {
            if let Some(command) = &hit.command {
                self.dispatch_command(command.clone()).await?;
            } else if hit.kind == HitKind::Input {
                if let Some(id) = &hit.control_id {
                    let _ = id;
                }
            }
        }
        Ok(())
    }

    pub async fn dispatch_command(&mut self, command: CommandDescriptor) -> Result<(), String> {
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
            if op.get("op").and_then(|v| v.as_str()) == Some("spawnProgram") {
                if let (Some(program_id), Some(session)) = (op.get("programId").and_then(|v| v.as_str()), &self.session) {
                    self.spawn_program(program_id, session.view_state.clone()).await?;
                }
            }
        }
        if let (Some(mut session), Some(vs)) = (self.session.take(), view_state) {
            session.view_state = vs;
            self.session = Some(session);
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
        });
        panel.active_spawned_id = Some(spawned_id);
        view_state.panel_json = Some(Self::panel_json(&panel));
        if let Some(session) = self.session.as_mut() {
            session.view_state = view_state;
        }
        Ok(())
    }

    pub fn render_chrome(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState,
    ) {
        let w = self.screen_w;
        let h = self.screen_h;
        draw.push_solid([0.0, 0.0, w, h], Rgba::BACKGROUND);
        self.render_navbar(draw, atlas, input, w);
        let body_y = NAVBAR_HEIGHT;
        let body_h = h - NAVBAR_HEIGHT;
        let mut content_x = 0.0;
        let mut content_w = w;
        if self.left_panel_open {
            self.render_left_panel(draw, atlas, input, content_x, body_y, self.left_panel_width, body_h);
            content_x += self.left_panel_width;
            content_w -= self.left_panel_width;
        }
        if self.right_panel_open {
            self.render_right_panel(
                draw,
                atlas,
                input,
                w - self.right_panel_width,
                body_y,
                self.right_panel_width,
                body_h,
            );
            content_w -= self.right_panel_width;
        }
        self.render_main_window(draw, atlas, input, Rect::new(content_x, body_y, content_w, body_h));
        if let Some(error) = &self.error {
            draw_text(
                &mut WidgetContext { draw, atlas, input },
                error,
                12.0,
                h - 24.0,
                FONT_SIZE_SMALL,
                Rgba::new(0.95, 0.35, 0.35, 1.0),
            );
        }
    }

    fn render_navbar(&self, draw: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState, width: f32) {
        draw.push_solid([0.0, 0.0, width, NAVBAR_HEIGHT], Rgba::NAVBAR);
        let title = self
            .session
            .as_ref()
            .map(|s| s.app.label.as_str())
            .unwrap_or("semio");
        draw_text(
            &mut WidgetContext { draw, atlas, input },
            title,
            12.0,
            NAVBAR_HEIGHT * 0.5 + FONT_SIZE_BODY * 0.5 - 2.0,
            FONT_SIZE_BODY,
            Rgba::TEXT,
        );
        if self.studio_mode {
            draw_text(
                &mut WidgetContext { draw, atlas, input },
                "S Studio",
                120.0,
                NAVBAR_HEIGHT * 0.5 + FONT_SIZE_SMALL,
                FONT_SIZE_SMALL,
                Rgba::ACCENT,
            );
        }
        let toggle_rect = Rect::new(width - 80.0, 6.0, 32.0, NAVBAR_HEIGHT - 12.0);
        draw.push_rounded(
            [toggle_rect.x, toggle_rect.y, toggle_rect.w, toggle_rect.h],
            Rgba::BUTTON,
            BORDER_RADIUS,
        );
        input.register_hit(HitTarget {
            rect: toggle_rect,
            command: None,
            control_id: Some("panel.toggle.left".into()),
            kind: HitKind::NavbarItem,
        });
    }

    fn render_left_panel(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        draw.push_solid([x, y, width, height], Rgba::PANEL);
        draw.push_line(x + width, y, x + width, y + height, Rgba::PANEL_BORDER, 1.0);
        let session = match &self.session {
            Some(s) => s,
            None => return,
        };
        let tabs: &[PanelTabDefinition] = &session.app.panel_tabs;
        let left_tabs: Vec<&PanelTabDefinition> = tabs
            .iter()
            .filter(|tab| tab.group != "inspection" && tab.group != "parameters")
            .collect();
        let mut tab_y = y + 4.0;
        for tab in left_tabs {
            let rect = Rect::new(x + 4.0, tab_y, width - 8.0, PANEL_HEADER_HEIGHT);
            let active = session
                .view_state
                .panel_json
                .as_ref()
                .and_then(|_| Self::panel_state_from_view(&session.view_state))
                .map(|p| p.active_panel_tab == tab.id)
                .unwrap_or(false);
            let bg = if active { Rgba::SELECTED } else { Rgba::BUTTON };
            draw.push_rounded([rect.x, rect.y, rect.w, rect.h], bg, BORDER_RADIUS);
            draw_text(
                &mut WidgetContext { draw, atlas, input },
                &tab.label,
                rect.x + 8.0,
                rect.y + 20.0,
                FONT_SIZE_SMALL,
                Rgba::TEXT,
            );
            input.register_hit(HitTarget {
                rect,
                command: Some(CommandDescriptor {
                    controller_id: session.app.controller_id.clone(),
                    command: format!("panel.select.{}", tab.id),
                    args: None,
                }),
                control_id: Some(tab.id.clone()),
                kind: HitKind::PanelTab,
            });
            tab_y += PANEL_HEADER_HEIGHT + 4.0;
            if active {
                if let Some(ui) = self.panel_ui.get(&tab.id) {
                    let content = Rect::new(x + 8.0, tab_y, width - 16.0, height - (tab_y - y) - 8.0);
                    render_ui_node(ui, content, &mut WidgetContext { draw, atlas, input });
                }
            }
        }
    }

    fn render_right_panel(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        draw.push_solid([x, y, width, height], Rgba::PANEL);
        draw.push_line(x, y, x, y + height, Rgba::PANEL_BORDER, 1.0);
        let session = match &self.session {
            Some(s) => s,
            None => return,
        };
        let right_tabs: Vec<&PanelTabDefinition> = session
            .app
            .panel_tabs
            .iter()
            .filter(|tab| tab.group == "inspection" || tab.group == "parameters")
            .collect();
        let mut tab_y = y + 4.0;
        for tab in right_tabs {
            let rect = Rect::new(x + 4.0, tab_y, width - 8.0, PANEL_HEADER_HEIGHT);
            draw.push_rounded([rect.x, rect.y, rect.w, rect.h], Rgba::BUTTON, BORDER_RADIUS);
            draw_text(
                &mut WidgetContext { draw, atlas, input },
                &tab.label,
                rect.x + 8.0,
                rect.y + 20.0,
                FONT_SIZE_SMALL,
                Rgba::TEXT,
            );
            tab_y += PANEL_HEADER_HEIGHT + 4.0;
            if let Some(ui) = self.panel_ui.get(&tab.id) {
                let content = Rect::new(x + 8.0, tab_y, width - 16.0, height * 0.45);
                render_ui_node(ui, content, &mut WidgetContext { draw, atlas, input });
            }
        }
    }

    fn render_main_window(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState,
        bounds: Rect,
    ) {
        draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], Rgba::BACKGROUND);
        let session = match &self.session {
            Some(s) => s,
            None => return,
        };
        if self.studio_mode {
            if let Some(spawned_ui) = &self.spawned_ui {
                render_ui_node(spawned_ui, bounds.inset(8.0), &mut WidgetContext { draw, atlas, input });
                return;
            }
        }
        let window_id = self
            .active_window_id
            .as_ref()
            .or_else(|| session.app.window_kinds.first().map(|w| &w.id));
        if let Some(id) = window_id {
            if let Some(ui) = self.window_ui.get(id) {
                render_ui_node(ui, bounds.inset(8.0), &mut WidgetContext { draw, atlas, input });
                return;
            }
        }
        draw_text(
            &mut WidgetContext { draw, atlas, input },
            &session.app.label,
            bounds.x + 16.0,
            bounds.y + 32.0,
            FONT_SIZE_BODY,
            Rgba::TEXT_MUTED,
        );
    }
}
