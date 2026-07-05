//! 📤 WASM component export glue for plugin bundles.

use crate::app::{AppInstance, Plugin, PluginBundle};
use semio_framework_core::{PluginManifest, UiNode, ViewState};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};

thread_local! {
    static PLUGIN: RefCell<Option<PluginBundle>> = const { RefCell::new(None) };
    static INSTANCES: RefCell<Vec<AppInstance>> = const { RefCell::new(Vec::new()) };
}

static NEXT_INSTANCE_ID: AtomicU32 = AtomicU32::new(1);

pub fn install_plugin_bundle(bundle: PluginBundle) {
    PLUGIN.with(|slot| {
        *slot.borrow_mut() = Some(bundle);
    });
}

pub fn plugin_manifest() -> PluginManifest {
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
        INSTANCES.with(|instances| {
            instances.borrow_mut().push(AppInstance {
                id,
                app,
                document_json,
            });
        });
        Ok(id)
    })
}

pub fn plugin_destroy_app(instance_id: u32) -> Result<(), String> {
    INSTANCES.with(|instances| {
        let mut list = instances.borrow_mut();
        let index = list
            .iter()
            .position(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        list.remove(index);
        Ok(())
    })
}

pub fn plugin_handle_command(
    instance_id: u32,
    command_json: &str,
    view_state_json: &str,
) -> Result<Vec<String>, String> {
    let command: serde_json::Value =
        serde_json::from_str(command_json).map_err(|error| error.to_string())?;
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    let command_name = command
        .get("command")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    let args = command.get("args").cloned();
    INSTANCES.with(|instances| {
        let mut list = instances.borrow_mut();
        let instance = list
            .iter_mut()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        let ops = instance.app.handle_command(
            command_name,
            args.as_ref(),
            &instance.document_json,
            &view_state,
        );
        for op in &ops {
            if let Ok(next) = apply_document_op(&instance.document_json, op) {
                instance.document_json = next;
            }
        }
        Ok(ops)
    })
}

pub fn plugin_render(instance_id: u32, body_key: &str, view_state_json: &str) -> Result<UiNode, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    INSTANCES.with(|instances| {
        let list = instances.borrow();
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .render(body_key, &instance.document_json, &view_state))
    })
}

pub fn plugin_tools(instance_id: u32, view_state_json: &str) -> Result<Vec<semio_framework_core::ToolNode>, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    INSTANCES.with(|instances| {
        let list = instances.borrow();
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .tools(&instance.document_json, &view_state))
    })
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

#[macro_export]
macro_rules! wasm_plugin_exports {
    () => {
        #[cfg(target_arch = "wasm32")]
        mod semio_wasm_exports {
            use super::_PLUGIN_INIT;
            use semio_framework_plugin::plugin_runtime::{
                plugin_create_app, plugin_destroy_app, plugin_handle_command, plugin_manifest, plugin_render,
                plugin_tools,
            };
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(start)]
            pub fn semio_plugin_start() {
                let _ = &*_PLUGIN_INIT;
            }

            #[wasm_bindgen]
            pub fn semio_plugin_manifest() -> String {
                serde_json::to_string(&plugin_manifest()).unwrap_or_else(|_| "{}".into())
            }

            #[wasm_bindgen]
            pub fn semio_plugin_create_app(app_id: &str) -> Result<u32, JsValue> {
                plugin_create_app(app_id).map_err(|error| JsValue::from_str(&error))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_destroy_app(instance_id: u32) -> Result<(), JsValue> {
                plugin_destroy_app(instance_id).map_err(|error| JsValue::from_str(&error))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_handle_command(
                instance_id: u32,
                command_json: &str,
                view_state_json: &str,
            ) -> Result<String, JsValue> {
                let ops = plugin_handle_command(instance_id, command_json, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&ops).map_err(|error| JsValue::from_str(&error.to_string()))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_render(
                instance_id: u32,
                body_key: &str,
                view_state_json: &str,
            ) -> Result<String, JsValue> {
                let node = plugin_render(instance_id, body_key, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&node).map_err(|error| JsValue::from_str(&error.to_string()))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_tools(instance_id: u32, view_state_json: &str) -> Result<String, JsValue> {
                let tools = plugin_tools(instance_id, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&tools).map_err(|error| JsValue::from_str(&error.to_string()))
            }
        }
    };
}
