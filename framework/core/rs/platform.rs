//! 🖥️ Root shell: apps, URI chrome, panel toggles, and shared command bus.

use crate::command_bus::CommandBus;
use crate::ui::AppDefinition;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PanelVisibility {
    pub left_side_panel: bool,
    pub right_side_panel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct PlatformSpec {
    pub id: String,
    pub name: String,
    pub default_active_app_id: Option<String>,
    pub initial_panel_visibility: Option<PanelVisibility>,
}

pub struct Platform {
    pub command_bus: CommandBus,
    pub apps: Vec<AppDefinition>,
    pub active_app_id: String,
    pub generation: u64,
    pub chrome_generation: u64,
    pub uri: String,
    pub panel_visibility: PanelVisibility,
    pub id: String,
    pub name: String,
    generation_counter: AtomicU64,
    chrome_generation_counter: AtomicU64,
}

impl Platform {
    pub fn new(spec: Option<PlatformSpec>) -> Self {
        let spec = spec.unwrap_or_default();
        let panel_visibility = spec.initial_panel_visibility.clone().unwrap_or_default();
        Self {
            command_bus: CommandBus::new(),
            apps: Vec::new(),
            active_app_id: spec.default_active_app_id.clone().unwrap_or_default(),
            generation: 0,
            chrome_generation: 0,
            uri: "/".into(),
            panel_visibility,
            id: spec.id,
            name: spec.name,
            generation_counter: AtomicU64::new(0),
            chrome_generation_counter: AtomicU64::new(0),
        }
    }

    pub fn add_app(&mut self, app: AppDefinition) {
        if self.active_app_id.is_empty() {
            self.active_app_id = app.id.clone();
        }
        self.apps.push(app);
        self.notify();
    }

    pub fn get_active_app(&self) -> Option<&AppDefinition> {
        self.apps
            .iter()
            .find(|app| app.id == self.active_app_id)
            .or_else(|| self.apps.first())
    }

    pub fn set_active_app_id(&mut self, id: String) {
        if self.active_app_id == id {
            return;
        }
        self.active_app_id = id;
        self.notify_chrome();
    }

    pub fn set_panel_visibility(&mut self, next: PanelVisibility) {
        if self.panel_visibility == next {
            return;
        }
        self.panel_visibility = next;
        self.notify_chrome();
    }

    pub fn notify(&mut self) {
        self.generation = self.generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }

    pub fn notify_chrome(&mut self) {
        self.chrome_generation = self.chrome_generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{ModeDefinition, WindowKindDefinition};

    #[test]
    fn adds_first_app_as_active() {
        let mut platform = Platform::new(None);
        platform.add_app(AppDefinition {
            id: "draw-play".into(),
            label: "Draw".into(),
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: vec![ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
            }],
            default_mode_id: Some("edit".into()),
            window_kinds: vec![WindowKindDefinition {
                id: "composite".into(),
                label: "Canvas".into(),
                body_key: "composite".into(),
                icon_id: None,
                measures: Vec::new(),
                engagement: None,
            }],
            panel_tabs: vec![],
            keybindings: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
        });
        assert_eq!(platform.active_app_id, "draw-play");
    }
}
