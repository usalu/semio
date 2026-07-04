//! 🔌 JS bridge for wasm-bindgen plugin modules.

use js_sys::{Array, Function, Reflect};
use semio_framework_core::{PluginManifest, UiNode, ViewState};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone)]
pub struct PluginBridgeEntry {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    handle: Rc<JsValue>,
}

impl PluginBridgeEntry {
    pub fn from_js(plugin_id: String, handle: JsValue) -> Result<Self, String> {
        let manifest_fn = Reflect::get(&handle, &JsValue::from_str("manifest"))
            .map_err(|_| "missing manifest")?;
        let manifest_fn: Function = manifest_fn.dyn_into().map_err(|_| "manifest not fn")?;
        let manifest_json = manifest_fn
            .call0(&JsValue::NULL)
            .map_err(|_| "manifest call failed")?
            .as_string()
            .ok_or("manifest not string")?;
        let manifest: PluginManifest =
            serde_json::from_str(&manifest_json).map_err(|err| format!("manifest parse: {err}"))?;
        let _create_app = get_fn(&handle, "createApp")?;
        let _render = get_fn(&handle, "render")?;
        Ok(Self {
            plugin_id,
            manifest,
            handle: Rc::new(handle),
        })
    }

    pub async fn create_app(&self, app_id: &str) -> Result<u32, String> {
        let create_app = get_fn(self.handle.as_ref(), "createApp")?;
        let result = create_app
            .call1(&JsValue::NULL, &JsValue::from_str(app_id))
            .map_err(|_| "create_app failed")?;
        if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            let resolved = JsFuture::from(promise.clone())
                .await
                .map_err(|_| "create_app promise failed")?;
            resolved.as_f64().map(|v| v as u32).ok_or("create_app not number".into())
        } else {
            result.as_f64().map(|v| v as u32).ok_or("create_app not number".into())
        }
    }

    pub fn destroy_app(&self, instance_id: u32) {
        if let Ok(destroy) = Reflect::get(self.handle.as_ref(), &JsValue::from_str("destroyApp"))
            .and_then(|v| v.dyn_into::<Function>())
        {
            let _ = destroy.call1(&JsValue::NULL, &JsValue::from_f64(instance_id as f64));
        }
    }

    pub async fn handle_command(
        &self,
        instance_id: u32,
        command_json: &str,
        view_state: &ViewState,
    ) -> Result<Vec<String>, String> {
        let handle = Reflect::get(self.handle.as_ref(), &JsValue::from_str("handleCommand"))
            .ok()
            .and_then(|v| v.dyn_into::<Function>().ok());
        let Some(handle) = handle else {
            return Ok(Vec::new());
        };
        let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
        let result = handle
            .call3(
                &JsValue::NULL,
                &JsValue::from_f64(instance_id as f64),
                &JsValue::from_str(command_json),
                &JsValue::from_str(&view_json),
            )
            .map_err(|_| "handle_command failed")?;
        let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            JsFuture::from(promise.clone())
                .await
                .map_err(|_| "handle_command promise failed")?
        } else {
            result
        };
        if let Some(array) = resolved.dyn_ref::<Array>() {
            let mut ops = Vec::new();
            for index in 0..array.length() {
                if let Some(value) = array.get(index).as_string() {
                    ops.push(value);
                }
            }
            return Ok(ops);
        }
        if let Some(text) = resolved.as_string() {
            let parsed: Vec<String> = serde_json::from_str(&text).unwrap_or_default();
            return Ok(parsed);
        }
        Ok(Vec::new())
    }

    pub async fn render(
        &self,
        instance_id: u32,
        body_key: &str,
        view_state: &ViewState,
    ) -> Result<UiNode, String> {
        let render = get_fn(self.handle.as_ref(), "render")?;
        let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
        let result = render
            .call3(
                &JsValue::NULL,
                &JsValue::from_f64(instance_id as f64),
                &JsValue::from_str(body_key),
                &JsValue::from_str(&view_json),
            )
            .map_err(|_| "render failed")?;
        let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            JsFuture::from(promise.clone())
                .await
                .map_err(|_| "render promise failed")?
        } else {
            result
        };
        let json = resolved.as_string().ok_or("render not string")?;
        serde_json::from_str(&json).map_err(|err| format!("render parse: {err}"))
    }
}

fn get_fn(obj: &JsValue, key: &str) -> Result<Function, String> {
    Reflect::get(obj, &JsValue::from_str(key))
        .map_err(|_| format!("missing {key}"))?
        .dyn_into()
        .map_err(|_| format!("{key} not fn"))
}

pub fn parse_plugin_entries(plugins: JsValue) -> Result<Vec<PluginBridgeEntry>, String> {
    let array = plugins.dyn_into::<Array>().map_err(|_| "plugins not array")?;
    let mut entries = Vec::new();
    for index in 0..array.length() {
        let item = array.get(index);
        let plugin_id = Reflect::get(&item, &JsValue::from_str("pluginId"))
            .ok()
            .and_then(|v| v.as_string())
            .ok_or("pluginId missing")?;
        let handle = Reflect::get(&item, &JsValue::from_str("handle")).map_err(|_| "handle missing")?;
        entries.push(PluginBridgeEntry::from_js(plugin_id, handle)?);
    }
    Ok(entries)
}

pub fn is_studio_mode(plugin_filter: &str) -> bool {
    plugin_filter == "s"
}

pub fn filter_plugins(entries: Vec<PluginBridgeEntry>, plugin_filter: &str) -> Vec<PluginBridgeEntry> {
    if is_studio_mode(plugin_filter) {
        entries
    } else {
        entries
            .into_iter()
            .filter(|entry| entry.plugin_id == plugin_filter)
            .collect()
    }
}
