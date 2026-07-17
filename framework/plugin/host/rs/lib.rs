//! 🛡️ Sandboxed wasmtime component plugin host with capability-gated imports.

use semio_framework_core::{
    kernel::{CapabilityRequirement, ResourceKind, Rights, Scope},
    InvocationResult, PluginManifest, ViewState,
};
use ui_wgpu::{UtilityNode, UiNode, WindowEngagement, WindowMeasure};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
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
    backbones: HashMap<String, Box<dyn vcs::Backbone>>,
    /// @emoji 📦 Backing store for `write-blob`/`read-blob`, injected via
    /// {@link WasmPluginRuntime::register_host_blob_store} — `None` until a caller registers one
    /// (mirrors `backbones`' explicit-registration convention, not a stub-forever like `read-asset`).
    blob_store: Option<Arc<dyn vcs::BlobStore>>,
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

    /// @emoji 🔌 Looks up the real, native-side backbone for a plugin-attached uri — the plugin only
    /// ever sees an opaque channel; this host process owns the actual sync endpoint. Native URI→IO
    /// resolution left this crate with WS-A (`vcs::resolve_backbone` is wasm-only now); the endpoint
    /// must be registered up front via {@link WasmPluginRuntime::register_host_backbone}. WS-E wires a
    /// `sync::DocumentHost`-backed backbone in here; until then this is an explicit-registration map.
    fn backbone_for(&mut self, uri: &str) -> Result<&mut Box<dyn vcs::Backbone>, String> {
        self.backbones.get_mut(uri).ok_or_else(|| {
            format!("no host backbone registered for {uri}; call register_host_backbone (WS-E wires DocumentHost here)")
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

    fn read_document(&mut self, _handle: u64) -> Result<String, String> {
        Err("read-document not implemented".into())
    }

    fn write_document(&mut self, _handle: u64, _payload_json: String) -> Result<(), String> {
        Err("write-document not implemented".into())
    }

    fn open_window(&mut self, _kind: String, _params_json: String) -> Result<u64, String> {
        Err("open-window not implemented".into())
    }

    fn invoke_action(&mut self, _target: String, _invocation_json: String) -> Result<String, String> {
        Err("invoke-action not implemented".into())
    }

    fn read_asset(&mut self, _handle: u64) -> Result<Vec<u8>, String> {
        Err("read-asset not implemented".into())
    }

    fn network_fetch(&mut self, _origin: String, _path: String) -> Result<Vec<u8>, String> {
        Err("network-fetch not implemented".into())
    }

    fn write_blob(&mut self, data: Vec<u8>, media_type: String) -> Result<String, String> {
        let store = self
            .blob_store
            .as_ref()
            .ok_or("no host blob store registered; call register_host_blob_store")?;
        store.put(&data, &media_type).map(|blob_ref| blob_ref.hash).map_err(|error| error.to_string())
    }

    fn read_blob(&mut self, hash: String) -> Result<Vec<u8>, String> {
        let store = self
            .blob_store
            .as_ref()
            .ok_or("no host blob store registered; call register_host_blob_store")?;
        store
            .get(&hash)
            .map_err(|error| error.to_string())?
            .ok_or_else(|| format!("blob not found: {hash}"))
    }

    fn backbone_send(&mut self, uri: String, message_json: String) -> Result<(), String> {
        if !self.has_backbone_access(Rights::Write) {
            return Err("backbone write capability missing".into());
        }
        let message: vcs::BackboneMessage =
            serde_json::from_str(&message_json).map_err(|error| error.to_string())?;
        self.backbone_for(&uri)?.send(message).map_err(|error| error.to_string())
    }

    fn backbone_poll(&mut self, uri: String) -> Result<Vec<String>, String> {
        if !self.has_backbone_access(Rights::Read) {
            return Err("backbone read capability missing".into());
        }
        let messages = self.backbone_for(&uri)?.receive().map_err(|error| error.to_string())?;
        messages
            .into_iter()
            .map(|message| serde_json::to_string(&message).map_err(|error| error.to_string()))
            .collect()
    }

    fn backbone_status(&mut self, uri: String) -> Result<String, String> {
        Ok(if self.backbones.contains_key(&uri) { "attached".into() } else { "detached".into() })
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
            backbones: HashMap::new(),
            blob_store: None,
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

    /// @emoji 🔗 Registers the native-side backbone endpoint the sandboxed plugin's `backbone-send`/
    /// `backbone-poll`/`backbone-status` host calls operate against, keyed by uri. WS-E calls this
    /// with a `sync::DocumentHost`-backed backbone once the actor layer is wired; until then it is an
    /// explicit in-process registration (there is no native URI→IO resolution in this crate anymore).
    pub fn register_host_backbone(&self, uri: &str, backbone: Box<dyn vcs::Backbone>) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        store.data_mut().backbones.insert(uri.to_string(), backbone);
        Ok(())
    }

    /// @emoji ✂️ Removes a previously registered native backbone endpoint.
    pub fn deregister_host_backbone(&self, uri: &str) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        store.data_mut().backbones.remove(uri);
        Ok(())
    }

    /// @emoji 📦 Registers the native-side `BlobStore` the sandboxed plugin's `write-blob`/`read-blob`
    /// host calls operate against. Not granted by default (unlike backbones there is no capability
    /// gate on these two calls today — every plugin that links `write-blob`/`read-blob` gets them once
    /// a store is registered); callers that embed this runtime decide when/whether to call this.
    pub fn register_host_blob_store(&self, store: Arc<dyn vcs::BlobStore>) -> Result<(), String> {
        let mut plugin_store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        plugin_store.data_mut().blob_store = Some(store);
        Ok(())
    }

    /// @emoji ✂️ Removes the previously registered native blob store.
    pub fn deregister_host_blob_store(&self) -> Result<(), String> {
        let mut plugin_store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        plugin_store.data_mut().blob_store = None;
        Ok(())
    }

    /// @emoji 📥 Ingests a JSON array of remote `OpEnvelope`s into the plugin instance's store.
    pub fn apply_operations(&self, instance_id: u32, operations_json: &str) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        bindings
            .semio_framework_plugin()
            .call_apply_operations(&mut *store, instance_id, operations_json)
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })
    }

    /// @emoji 📖 Reads the plugin instance's full persistent document JSON.
    pub fn read_app_document(&self, instance_id: u32) -> Result<String, String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        bindings
            .semio_framework_plugin()
            .call_read_app_document(&mut *store, instance_id)
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })
    }

    /// @emoji 📂 Replaces the plugin instance's document from a serialized envelope.
    pub fn load_app_document(&self, instance_id: u32, document_json: &str) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        bindings
            .semio_framework_plugin()
            .call_load_app_document(&mut *store, instance_id, document_json)
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })
    }

    /// @emoji 🔗 Asks the plugin to attach a backbone by uri (resolved to a `PortBackbone` inside the
    /// sandbox, relayed to the endpoint registered here via {@link register_host_backbone}).
    pub fn attach_backbone(&self, instance_id: u32, uri: &str) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        bindings
            .semio_framework_plugin()
            .call_attach_backbone(&mut *store, instance_id, uri)
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })
    }

    /// @emoji ✂️ Asks the plugin to detach its backbone channel.
    pub fn detach_backbone(&self, instance_id: u32) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let bindings = self.bindings.lock().map_err(|_| "plugin bindings lock poisoned")?;
        Self::prepare_call(&mut store);
        bindings
            .semio_framework_plugin()
            .call_detach_backbone(&mut *store, instance_id)
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })
    }

    pub fn handle_action(
        &self,
        instance_id: u32,
        action_json: &str,
        view_state: &ViewState,
    ) -> Result<InvocationResult, String> {
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
            .call_handle_action(
                &mut *store,
                instance_id,
                &semio::framework::types::ActionInvocationJson {
                    json: action_json.to_string(),
                },
                &semio::framework::types::InvocationContextJson {
                    json: context_json,
                },
            )
            .map_err(|error| error.to_string())?
            .map_err(|error| match error {
                semio::framework::types::PluginError::Message(message) => message,
            })?;
        serde_json::from_str(&response.json).map_err(|error| error.to_string())
    }

    /// @emoji 🎛️ Dispatches a scoped command (os/plugin/app/mode) — the command mirror of `handle_action`.
    pub fn handle_command(
        &self,
        instance_id: u32,
        command_json: &str,
        view_state: &ViewState,
    ) -> Result<InvocationResult, String> {
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
                &semio::framework::types::InvocationContextJson {
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
        self.render_with_document(instance_id, body_key, view_state, None)
    }

    pub fn render_with_document(
        &self,
        instance_id: u32,
        body_key: &str,
        view_state: &ViewState,
        document_json: Option<&str>,
    ) -> Result<UiNode, String> {
        let mut input = serde_json::json!({
            "bodyKey": body_key,
            "viewState": view_state,
        });
        if let Some(document) = document_json {
            input["documentJson"] = serde_json::Value::String(document.to_string());
        }
        let input_json = input.to_string();
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

    pub fn utilities(&self, _instance_id: u32, _view_state: &ViewState) -> Result<Vec<UtilityNode>, String> {
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

    pub fn app_labels(
        &self,
        _instance_id: u32,
        _view_state: &ViewState,
    ) -> Result<semio_framework_core::AppLabelsOverlay, String> {
        Ok(semio_framework_core::AppLabelsOverlay::default())
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
            contributions: vec![],
            commands: vec![],
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
