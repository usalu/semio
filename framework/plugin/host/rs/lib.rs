//! 🛡️ Sandboxed wasmtime plugin host with capability-gated imports.

use semio_framework_core::{
    Capability, PluginManifest, ToolNode, UiNode, ViewState, WindowEngagement, WindowMeasure,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use wasmtime::{Engine, Instance, Linker, Module, Store};

//#region 🔖WasmPluginRuntime
pub struct WasmPluginRuntime {
    engine: Engine,
    module: Module,
    store: Mutex<Store<()>>,
    instance: Mutex<Instance>,
    pub manifest: PluginManifest,
    pub path: PathBuf,
}

impl WasmPluginRuntime {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();
        let wasm_bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
        let engine = Engine::default();
        let module = Module::new(&engine, &wasm_bytes).map_err(|error| error.to_string())?;
        let manifest = Self::read_manifest(&engine, &module)?;
        let (store, instance) = Self::instantiate_with_manifest(&engine, &module, &manifest)?;
        Ok(Self {
            engine,
            module,
            store: Mutex::new(store),
            instance: Mutex::new(instance),
            manifest,
            path,
        })
    }

    pub fn hot_reload(&mut self) -> Result<(), String> {
        let wasm_bytes = std::fs::read(&self.path).map_err(|error| error.to_string())?;
        let module = Module::new(&self.engine, &wasm_bytes).map_err(|error| error.to_string())?;
        self.manifest = Self::read_manifest(&self.engine, &module)?;
        let (store, instance) = Self::instantiate_with_manifest(&self.engine, &module, &self.manifest)?;
        self.module = module;
        *self.store.lock().map_err(|_| "plugin store lock poisoned")? = store;
        *self.instance.lock().map_err(|_| "plugin instance lock poisoned")? = instance;
        Ok(())
    }

    pub fn manifest_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.manifest).map_err(|error| error.to_string())
    }

    pub fn create_app(&self, app_id: &str) -> Result<u32, String> {
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let instance = self.instance.lock().map_err(|_| "plugin instance lock poisoned")?;
        let app_ptr = write_guest_c_string(&mut store, &instance, app_id)?;
        let id = call_export_i32_args(&mut store, &instance, "semio_plugin_create_app", &[app_ptr as i32])? as u32;
        free_guest_string(&mut store, &instance, app_ptr)?;
        if id == u32::MAX {
            return Err(format!("create_app failed for {app_id}"));
        }
        Ok(id)
    }

    pub fn destroy_app(&self, instance_id: u32) {
        if let (Ok(mut store), Ok(instance)) = (self.store.lock(), self.instance.lock()) {
            let _ = call_export_i32_args(&mut store, &instance, "semio_plugin_destroy_app", &[instance_id as i32]);
        }
    }

    pub fn handle_command(
        &self,
        instance_id: u32,
        command_json: &str,
        view_state: &ViewState,
    ) -> Result<Vec<String>, String> {
        let view_state_json = serde_json::to_string(view_state).map_err(|error| error.to_string())?;
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let instance = self.instance.lock().map_err(|_| "plugin instance lock poisoned")?;
        let command_ptr = write_guest_c_string(&mut store, &instance, command_json)?;
        let view_ptr = write_guest_c_string(&mut store, &instance, &view_state_json)?;
        let out_ptr = call_export_i32_args(
            &mut store,
            &instance,
            "semio_plugin_handle_command",
            &[instance_id as i32, command_ptr as i32, view_ptr as i32],
        )? as u32;
        free_guest_string(&mut store, &instance, command_ptr)?;
        free_guest_string(&mut store, &instance, view_ptr)?;
        let json = read_guest_c_string(&mut store, &instance, out_ptr)?;
        free_guest_string(&mut store, &instance, out_ptr)?;
        serde_json::from_str(&json).map_err(|error| error.to_string())
    }

    pub fn render(&self, instance_id: u32, body_key: &str, view_state: &ViewState) -> Result<UiNode, String> {
        let view_state_json = serde_json::to_string(view_state).map_err(|error| error.to_string())?;
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let instance = self.instance.lock().map_err(|_| "plugin instance lock poisoned")?;
        let body_ptr = write_guest_c_string(&mut store, &instance, body_key)?;
        let view_ptr = write_guest_c_string(&mut store, &instance, &view_state_json)?;
        let out_ptr = call_export_i32_args(
            &mut store,
            &instance,
            "semio_plugin_render",
            &[instance_id as i32, body_ptr as i32, view_ptr as i32],
        )? as u32;
        free_guest_string(&mut store, &instance, body_ptr)?;
        free_guest_string(&mut store, &instance, view_ptr)?;
        let json = read_guest_c_string(&mut store, &instance, out_ptr)?;
        free_guest_string(&mut store, &instance, out_ptr)?;
        serde_json::from_str(&json).map_err(|error| error.to_string())
    }

    pub fn tools(&self, instance_id: u32, view_state: &ViewState) -> Result<Vec<ToolNode>, String> {
        let view_state_json = serde_json::to_string(view_state).map_err(|error| error.to_string())?;
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let instance = self.instance.lock().map_err(|_| "plugin instance lock poisoned")?;
        let view_ptr = write_guest_c_string(&mut store, &instance, &view_state_json)?;
        let out_ptr = call_export_i32_args(
            &mut store,
            &instance,
            "semio_plugin_tools",
            &[instance_id as i32, view_ptr as i32],
        )? as u32;
        free_guest_string(&mut store, &instance, view_ptr)?;
        let json = read_guest_c_string(&mut store, &instance, out_ptr)?;
        free_guest_string(&mut store, &instance, out_ptr)?;
        serde_json::from_str(&json).map_err(|error| error.to_string())
    }

    pub fn window_engagements(
        &self,
        instance_id: u32,
        view_state: &ViewState,
    ) -> Result<HashMap<String, WindowEngagement>, String> {
        let view_state_json = serde_json::to_string(view_state).map_err(|error| error.to_string())?;
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let instance = self.instance.lock().map_err(|_| "plugin instance lock poisoned")?;
        let view_ptr = write_guest_c_string(&mut store, &instance, &view_state_json)?;
        let out_ptr = call_export_i32_args(
            &mut store,
            &instance,
            "semio_plugin_window_engagements",
            &[instance_id as i32, view_ptr as i32],
        )? as u32;
        free_guest_string(&mut store, &instance, view_ptr)?;
        let json = read_guest_c_string(&mut store, &instance, out_ptr)?;
        free_guest_string(&mut store, &instance, out_ptr)?;
        serde_json::from_str(&json).map_err(|error| error.to_string())
    }

    pub fn window_measures(
        &self,
        instance_id: u32,
        view_state: &ViewState,
    ) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
        let view_state_json = serde_json::to_string(view_state).map_err(|error| error.to_string())?;
        let mut store = self.store.lock().map_err(|_| "plugin store lock poisoned")?;
        let instance = self.instance.lock().map_err(|_| "plugin instance lock poisoned")?;
        let view_ptr = write_guest_c_string(&mut store, &instance, &view_state_json)?;
        let out_ptr = call_export_i32_args(
            &mut store,
            &instance,
            "semio_plugin_window_measures",
            &[instance_id as i32, view_ptr as i32],
        )? as u32;
        free_guest_string(&mut store, &instance, view_ptr)?;
        let json = read_guest_c_string(&mut store, &instance, out_ptr)?;
        free_guest_string(&mut store, &instance, out_ptr)?;
        serde_json::from_str(&json).map_err(|error| error.to_string())
    }

    fn read_manifest(engine: &Engine, module: &Module) -> Result<PluginManifest, String> {
        let (mut store, instance) = Self::instantiate_module(engine, module, true)?;
        let manifest_ptr = call_export_i32(&mut store, &instance, "semio_plugin_manifest")? as u32;
        let json = read_guest_c_string(&mut store, &instance, manifest_ptr)?;
        free_guest_string(&mut store, &instance, manifest_ptr)?;
        serde_json::from_str(&json).map_err(|error| error.to_string())
    }

    fn instantiate_module(
        engine: &Engine,
        module: &Module,
        link_backbone: bool,
    ) -> Result<(Store<()>, Instance), String> {
        let mut linker = Linker::new(engine);
        linker
            .func_wrap("env", "semio_host_log", |_ptr: i32, _len: i32| Ok(()))
            .map_err(|error| error.to_string())?;
        if link_backbone {
            linker
                .func_wrap("semio_host", "backbone_read", |_uri_ptr: i32, _uri_len: i32| -> i32 {
                    0
                })
                .map_err(|error| error.to_string())?;
            linker
                .func_wrap(
                    "semio_host",
                    "backbone_write",
                    |_uri_ptr: i32, _uri_len: i32, _payload_ptr: i32, _payload_len: i32| -> i32 { 0 },
                )
                .map_err(|error| error.to_string())?;
        }
        let mut store = Store::new(engine, ());
        let instance = linker
            .instantiate(&mut store, module)
            .map_err(|error| error.to_string())?;
        call_start(&mut store, &instance)?;
        Ok((store, instance))
    }

    fn instantiate_with_manifest(engine: &Engine, module: &Module, manifest: &PluginManifest) -> Result<(Store<()>, Instance), String> {
        Self::instantiate_module(
            engine,
            module,
            manifest.capabilities.contains(&Capability::LocalBackboneStorage),
        )
    }
}
//#endregion 🔖WasmPluginRuntime

fn call_start(store: &mut Store<()>, instance: &Instance) -> Result<(), String> {
    if let Ok(start) = instance.get_typed_func::<(), ()>(&mut *store, "semio_plugin_start") {
        start.call(&mut *store, ()).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn call_export_i32(store: &mut Store<()>, instance: &Instance, name: &str) -> Result<i32, String> {
    instance
        .get_typed_func::<(), i32>(&mut *store, name)
        .map_err(|error| error.to_string())?
        .call(&mut *store, ())
        .map_err(|error| error.to_string())
}

fn call_export_i32_args(store: &mut Store<()>, instance: &Instance, name: &str, args: &[i32]) -> Result<i32, String> {
    match args.len() {
        1 => instance
            .get_typed_func::<i32, i32>(&mut *store, name)
            .map_err(|error| error.to_string())?
            .call(&mut *store, args[0])
            .map_err(|error| error.to_string()),
        2 => instance
            .get_typed_func::<(i32, i32), i32>(&mut *store, name)
            .map_err(|error| error.to_string())?
            .call(&mut *store, (args[0], args[1]))
            .map_err(|error| error.to_string()),
        3 => instance
            .get_typed_func::<(i32, i32, i32), i32>(&mut *store, name)
            .map_err(|error| error.to_string())?
            .call(&mut *store, (args[0], args[1], args[2]))
            .map_err(|error| error.to_string()),
        _ => Err(format!("unsupported arg count for {name}")),
    }
}

fn memory_data(store: &mut Store<()>, instance: &Instance) -> Result<Vec<u8>, String> {
    let memory = instance
        .get_memory(&mut *store, "memory")
        .ok_or_else(|| "plugin memory export missing".to_string())?;
    Ok(memory.data(&mut *store).to_vec())
}

fn read_guest_c_string(store: &mut Store<()>, instance: &Instance, ptr: u32) -> Result<String, String> {
    if ptr == 0 {
        return Ok(String::new());
    }
    let data = memory_data(store, instance)?;
    let start = ptr as usize;
    if start >= data.len() {
        return Err("guest string pointer out of bounds".into());
    }
    let end = data[start..]
        .iter()
        .position(|byte| *byte == 0)
        .map(|offset| start + offset)
        .ok_or_else(|| "unterminated guest string".to_string())?;
    let slice = &data[start..end];
    std::str::from_utf8(slice)
        .map(|value| value.to_string())
        .map_err(|error| error.to_string())
}

fn write_guest_c_string(store: &mut Store<()>, instance: &Instance, value: &str) -> Result<u32, String> {
    let bytes: Vec<u8> = value.as_bytes().iter().chain(std::iter::once(&0u8)).copied().collect();
    let alloc = instance
        .get_typed_func::<i32, i32>(&mut *store, "semio_plugin_alloc")
        .map_err(|error| error.to_string())?;
    let ptr = alloc
        .call(&mut *store, bytes.len() as i32)
        .map_err(|error| error.to_string())? as u32;
    let memory = instance
        .get_memory(&mut *store, "memory")
        .ok_or_else(|| "plugin memory export missing".to_string())?;
    memory
        .write(&mut *store, ptr as usize, &bytes)
        .map_err(|error| error.to_string())?;
    Ok(ptr)
}

fn free_guest_string(store: &mut Store<()>, instance: &Instance, ptr: u32) -> Result<(), String> {
    if ptr == 0 {
        return Ok(());
    }
    if let Ok(free) = instance.get_typed_func::<i32, ()>(&mut *store, "semio_plugin_free_string") {
        free.call(&mut *store, ptr as i32).map_err(|error| error.to_string())?;
        return Ok(());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_plugin_runtime_api_exists() {
        let _ = std::mem::size_of::<WasmPluginRuntime>();
    }
}
