//! 🔌 Plugin host with hot-swap support.

use crate::instance::OsInstanceState;
use crate::registry::PluginRegistry;
use semio_framework_core::{AppDefinition, PluginManifest, UiNode, ViewState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginHotSwapEvent {
    pub plugin_id: String,
    pub version: String,
    pub added_apps: Vec<String>,
    pub removed_apps: Vec<String>,
}

pub struct LoadedPlugin {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    pub artifact_uri: String,
}

pub struct PluginHost {
    registry: PluginRegistry,
    instances: HashMap<u32, OsInstanceState>,
    next_instance_id: u32,
    plugins: HashMap<String, LoadedPlugin>,
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            instances: HashMap::new(),
            next_instance_id: 1,
            plugins: HashMap::new(),
        }
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    pub fn load_plugin(&mut self, plugin: LoadedPlugin) -> PluginHotSwapEvent {
        let previous_apps: Vec<String> = self
            .plugins
            .get(&plugin.plugin_id)
            .map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect())
            .unwrap_or_default();
        let next_apps: Vec<String> = plugin.manifest.apps.iter().map(|app| app.id.clone()).collect();
        for app in &plugin.manifest.apps {
            self.registry.register_app(app.clone());
        }
        for program in &plugin.manifest.programs {
            self.registry.register_program(program.clone());
        }
        self.plugins.insert(plugin.plugin_id.clone(), plugin);
        PluginHotSwapEvent {
            plugin_id: self.plugins.keys().next().cloned().unwrap_or_default(),
            version: self
                .plugins
                .values()
                .last()
                .map(|plugin| plugin.manifest.version.clone())
                .unwrap_or_default(),
            added_apps: next_apps
                .iter()
                .filter(|app| !previous_apps.contains(app))
                .cloned()
                .collect(),
            removed_apps: previous_apps
                .iter()
                .filter(|app| !next_apps.contains(app))
                .cloned()
                .collect(),
        }
    }

    pub fn hot_swap_plugin(&mut self, plugin: LoadedPlugin) -> PluginHotSwapEvent {
        let event = self.load_plugin(plugin);
        for instance in self.instances.values_mut() {
            instance.generation += 1;
        }
        event
    }

    pub fn apps(&self) -> Vec<AppDefinition> {
        self.registry.apps()
    }

    pub fn create_instance(&mut self, app_id: &str, document_json: String) -> Option<u32> {
        let app = self.registry.find_app(app_id)?;
        let id = self.next_instance_id;
        self.next_instance_id += 1;
        self.instances.insert(
            id,
            OsInstanceState {
                id,
                app_id: app.id.clone(),
                controller_id: app.controller_id.clone(),
                document_json,
                view_state: ViewState::default(),
                generation: 0,
            },
        );
        Some(id)
    }

    pub fn instance(&self, instance_id: u32) -> Option<&OsInstanceState> {
        self.instances.get(&instance_id)
    }

    pub fn instance_mut(&mut self, instance_id: u32) -> Option<&mut OsInstanceState> {
        self.instances.get_mut(&instance_id)
    }

    pub fn apply_ops(&mut self, instance_id: u32, ops: &[String]) -> bool {
        let Some(instance) = self.instances.get_mut(&instance_id) else {
            return false;
        };
        for op in ops {
            if let Ok(next) = apply_document_op(&instance.document_json, op) {
                instance.document_json = next;
                instance.generation += 1;
            }
        }
        true
    }

    pub fn set_view_state(&mut self, instance_id: u32, view_state: ViewState) {
        if let Some(instance) = self.instances.get_mut(&instance_id) {
            instance.view_state = view_state;
            instance.generation += 1;
        }
    }

    pub fn render_body(&self, instance_id: u32, body_key: &str, ui: UiNode) -> UiNode {
        let _ = (instance_id, body_key);
        ui
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_core::{ModeDefinition, PluginManifest, WindowKindDefinition};

    #[test]
    fn loads_plugin_apps_into_registry() {
        let mut host = PluginHost::new();
        let manifest = PluginManifest {
            plugin_id: "draw".into(),
            label: "Draw".into(),
            version: "0.1.0".into(),
            apps: vec![AppDefinition {
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
                }],
                panel_tabs: vec![],
                keybindings: vec![],
            }],
            programs: vec![],
            examples: vec![],
        };
        host.load_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest,
            artifact_uri: "plugin://draw".into(),
        });
        assert_eq!(host.apps().len(), 1);
    }
}
