//! 🧩 WASI P2 component exports for the plugin world contract.

use crate::plugin_runtime::{
    plugin_create_app, plugin_handle_command, plugin_manifest, plugin_render,
};
use wit_bindgen::generate;

generate!({
    world: "plugin-world",
    path: "../../wit",
});

use exports::semio::framework::plugin::Guest;
use semio::framework::types::{
    CommandContextJson, CommandInvocationJson, CommandResponseJson, MigrateModelInput,
    MigrateModelOutput, PluginError, PluginManifestJson, WindowInputJson, WindowOutputJson,
};

pub struct ComponentGuest;

impl Guest for ComponentGuest {
    fn manifest() -> PluginManifestJson {
        PluginManifestJson {
            json: serde_json::to_string(&plugin_manifest()).unwrap_or_else(|_| "{}".into()),
        }
    }

    fn instantiate_app(app_id: String, _instance_id: String) -> Result<u32, PluginError> {
        plugin_create_app(&app_id).map_err(PluginError::Message)
    }

    fn handle_command(
        instance_id: u32,
        command: CommandInvocationJson,
        context: CommandContextJson,
    ) -> Result<CommandResponseJson, PluginError> {
        let result = plugin_handle_command(instance_id, &command.json, &context.json)
            .map_err(PluginError::Message)?;
        Ok(CommandResponseJson {
            json: serde_json::to_string(&result).unwrap_or_else(|_| "{}".into()),
        })
    }

    fn update_window(
        instance_id: u32,
        input: WindowInputJson,
    ) -> Result<WindowOutputJson, PluginError> {
        let node = plugin_render(instance_id, "", &input.json).map_err(PluginError::Message)?;
        Ok(WindowOutputJson {
            json: serde_json::to_string(&node).unwrap_or_else(|_| "{}".into()),
        })
    }

    fn migrate_model(_input: MigrateModelInput) -> Result<MigrateModelOutput, PluginError> {
        Err(PluginError::Message("migrate-model not implemented".into()))
    }
}

export!(ComponentGuest);

pub fn component_export_anchor() {}
