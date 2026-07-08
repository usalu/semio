//! 🛡️ Sandboxed wasmtime component plugin host with capability-gated imports.

use semio_framework_core::{
    kernel::{CapabilityRequirement, ResourceKind, Rights, Scope},
    CommandResult, PluginManifest, ToolNode, UiNode, ViewState, WindowEngagement, WindowMeasure,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

const PLUGIN_FUEL_BUDGET: u64 = 50_000_000;

bindgen!({
    world: "plugin-world",
    path: "../../../wit",
    async: false,
});

//#region 🔖HostState
struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    granted_capabilities: Vec<CapabilityRequirement>,
    plugin_id: String,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl HostState {
    fn has_backbone_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| {
            cap.resource == ResourceKind::Backbone
                && cap.rights == rights
                && matches!(cap.scope, Scope::Plugin | Scope::Global)
        })
    }
}

impl semio::framework::host::Host for HostState {
    fn log(&mut self, level: String, message: String) {
        eprintln!("[plugin:{}:{level}] {message}", self.plugin_id);
    }

    fn now_ms(&mut self) -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or(0)
    }

    fn read_model(&mut self, _handle: u64) -> Result<String, String> {
        Err("read-model not implemented".into())
    }

    fn write_model(&mut self, _handle: u64, _payload_json: String) -> Result<(), String> {
        Err("write-model not implemented".into())
    }

    fn open_window(&mut self, _kind: String, _params_json: String) -> Result<u64, String> {
        Err("open-window not implemented".into())
    }

    fn invoke_command(&mut self, _target: String, _invocation_json: String) -> Result<String, String> {
        Err("invoke-command not implemented".into())
    }

    fn read_asset(&mut self, _handle: u64) -> Result<Vec<u8>, String> {
        Err("read-asset not implemented".into())
    }

    fn network_fetch(&mut self, _origin: String, _path: String) -> Result<Vec<u8>, String> {
        Err("network-fetch not implemented".into())
    }

    fn backbone_read(&mut self, uri: String) -> Result<String, String> {
        if !self.has_backbone_access(Rights::Read) {
            return Err("backbone read capability missing".into());
        }
        let _ = uri;
        Ok(String::new())
    }

    fn backbone_write(&mut self, uri: String, payload: String) -> Result<(), String> {
        if !self.has_backbone_access(Rights::Write) {
            return Err("backbone write capability missing".into());
        }
        let _ = (uri, payload);
        Ok(())
    }
}
//#endregion 🔖HostState

//#region 🔖WasmPluginRuntime
pub struct WasmPluginRuntime {
    engine: Engine,
    component: Component,
    linker: Linker<HostState>,
    store: Mutex<Store<HostState>>,
    bindings: Mutex<PluginWorld>,
    pub manifest: PluginManifest,
    pub path: PathBuf,
    pub supervisor_state: Mutex<PluginSupervisorState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginSupervisorState {
    Loaded,
    Running,
    Crashed,
    TimedOut,
    Restarting,
    Quarantined,
    Unloaded,
}

impl WasmPluginRuntime {
    fn build_engine() -> Result<Engine, String> {
        let mut config = Config::new();
        config.consume_fuel(true);
        config.epoch_interruption(true);
        config.wasm_component_model(true);
        Engine::new(&config).map_err(|error| error.to_string())
    }

    fn build_linker(engine: &Engine) -> Result<Linker<HostState>, String> {
        let mut linker = Linker::new(engine);
        semio::framework::host::add_to_linker(&mut linker, |state| state)
            .map_err(|error| error.to_string())?;
        wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(|error| error.to_string())?;
        Ok(linker)
    }

    fn host_state(plugin_id: &str, manifest: &PluginManifest) -> HostState {
        HostState {
            wasi: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
            granted_capabilities: manifest.capabilities.clone(),
            plugin_id: plugin_id.to_string(),
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();
        let wasm_bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
        let engine = Self::build_engine()?;
        let component = Component::from_binary(&engine, &wasm_bytes).map_err(|error| error.to_string())?;
        let linker = Self::build_linker(&engine)?;
        let manifest = Self::read_manifest(&engine, &component, &linker)?;
        let store = Store::new(&engine, Self::host_state(&manifest.plugin_id, &manifest));
        let (store, bindings) = Self::instantiate(store, &component, &linker)?;
        Ok(Self {
            engine,
            component,
            linker,
            store: Mutex::new(store),
            bindings: Mutex::new(bindings),
            manifest,
            path,
            supervisor_state: Mutex::new(PluginSupervisorState::Running),
        })
    }

    pub fn hot_reload(&mut self) -> Result<(), String> {
        *self
            .supervisor_state
            .lock()
            .map_err(|_| "supervisor lock poisoned")? = PluginSupervisorState::Restarting;
        let wasm_bytes = std::fs::read(&self.path).map_err(|error| error.to_string())?;
        let component = Component::from_binary(&self.engine, &wasm_bytes).map_err(|error| error.to_string())?;
        self.manifest = Self::read_manifest(&self.engine, &component, &self.linker)?;
        let store = Store::new(
            &self.engine,
            Self::host_state(&self.manifest.plugin_id, &self.manifest),
        );
        let (store, bindings) = Self::instantiate(store, &component, &self.linker)?;
        self.component = component;
        *self.store.lock().map_err(|_| "plugin store lock poisoned")? = store;
        *self
            .bindings
            .lock()
            .map_err(|_| "plugin bindings lock poisoned")? = bindings;
        *self
            .supervisor_state
            .lock()
            .map_err(|_| "supervisor lock poisoned")? = PluginSupervisorState::Running;
        Ok(())
    }

    pub fn supervisor_state(&self) -> PluginSupervisorState {
        self.supervisor_state
            .lock()
            .map(|state| *state)
            .unwrap_or(PluginSupervisorState::Crashed)
    }

    fn prepare_call(store: &mut Store<HostState>) {
        store.set_fuel(PLUGIN_FUEL_BUDGET).ok();
    }

    pub fn manifest_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.manifest).map_err(|error| error.to_string())
    }

    pub fn create_app(&self, app_id: &str) -> Result<u32, String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        bindings
            .semio_framework_plugin()
            .call_instantiate_app(&mut *store, app_id, app_id)
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })
    }

    pub fn destroy_app(&self, _instance_id: u32) {}

    pub fn handle_command(
        &self,
        instance_id: u32,
        command_json: &str,
        view_state: &ViewState,
    ) -> Result<CommandResult, String> {
        let context_json = serde_json::json!({
            "viewState": view_state,
            "actor": "local",
        })
        .to_string();
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        let response = bindings
            .semio_framework_plugin()
            .call_handle_command(
                &mut *store,
                instance_id,
                &semio::framework::types::CommandInvocationJson {
                    json: command_json.to_string(),
                },
                &semio::framework::types::CommandContextJson {
                    json: context_json,
                },
            )
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })?;
        serde_json::from_str(&response.json).map_err(|error| error.to_string())
    }

    pub fn render(&self, instance_id: u32, body_key: &str, view_state: &ViewState) -> Result<UiNode, String> {
        let input_json = serde_json::json!({
            "bodyKey": body_key,
            "viewState": view_state,
        })
        .to_string();
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        let response = bindings
            .semio_framework_plugin()
            .call_update_window(
                &mut *store,
                instance_id,
                &semio::framework::types::WindowInputJson { json: input_json },
            )
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })?;
        serde_json::from_str(&response.json).map_err(|error| error.to_string())
    }

    pub fn tools(&self, _instance_id: u32, _view_state: &ViewState) -> Result<Vec<ToolNode>, String> {
        Ok(Vec::new())
    }

    pub fn window_engagements(
        &self,
        _instance_id: u32,
        _view_state: &ViewState,
    ) -> Result<HashMap<String, WindowEngagement>, String> {
        Ok(HashMap::new())
    }

    pub fn window_measures(
        &self,
        _instance_id: u32,
        _view_state: &ViewState,
    ) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
        Ok(HashMap::new())
    }

    fn read_manifest(
        engine: &Engine,
        component: &Component,
        linker: &Linker<HostState>,
    ) -> Result<PluginManifest, String> {
        let manifest = PluginManifest {
            plugin_id: "unknown".into(),
            label: "Unknown".into(),
            version: "0.0.0".into(),
            apps: vec![],
            programs: vec![],
            examples: vec![],
            capabilities: vec![],
        };
        let mut store = Store::new(engine, Self::host_state("bootstrap", &manifest));
        let (bindings, _instance) = PluginWorld::instantiate(&mut store, component, linker)
            .map_err(|error| error.to_string())?;
        let response = bindings
            .semio_framework_plugin()
            .call_manifest(&mut store)
            .map_err(|error| error.to_string())?;
        serde_json::from_str(&response.json).map_err(|error| error.to_string())
    }

    fn instantiate(
        mut store: Store<HostState>,
        component: &Component,
        linker: &Linker<HostState>,
    ) -> Result<(Store<HostState>, PluginWorld), String> {
        let (bindings, _instance) = PluginWorld::instantiate(&mut store, component, linker)
            .map_err(|error| error.to_string())?;
        Ok((store, bindings))
    }
}
//#endregion 🔖WasmPluginRuntime

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_plugin_runtime_api_exists() {
        let _ = std::mem::size_of::<WasmPluginRuntime>();
    }
}
