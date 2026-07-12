//! 🧩 WASI P2 component exports for the plugin world contract.

use crate::plugin_runtime::{
    ensure_plugin_initialized, plugin_create_app, plugin_handle_action, plugin_manifest,
    plugin_render_with_document, plugin_tools, plugin_window_engagements, plugin_window_measures,
};
use wit_bindgen::generate;

generate!({
    world: "plugin-world",
    path: "../../wit",
});

use exports::semio::framework::plugin::Guest;
use semio::framework::types::{
    ActionContextJson, ActionInvocationJson, ActionResponseJson, MigrateDocumentInput,
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

    fn migrate_document(_input: MigrateDocumentInput) -> Result<MigrateDocumentOutput, PluginError> {
        Err(PluginError::Message("migrate-document not implemented".into()))
    }
}

export!(ComponentGuest);

pub fn component_export_anchor() {}

pub fn host_backbone_read(uri: &str) -> Result<String, String> {
    semio::framework::host::backbone_read(uri)
}

pub fn host_backbone_write(uri: &str, payload: &str) -> Result<(), String> {
    semio::framework::host::backbone_write(uri, payload)
}

pub fn host_now_ms() -> i64 {
    semio::framework::host::now_ms()
}
