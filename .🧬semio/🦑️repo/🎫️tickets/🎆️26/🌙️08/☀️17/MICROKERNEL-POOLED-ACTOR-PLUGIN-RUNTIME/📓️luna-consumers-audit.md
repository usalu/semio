# Consumer Map: Plugin Execution Path Replacement

Audit of all call sites that will require changes when the plugin execution path is replaced.  
Generated 2026-08-17. Each entry: `path:line | context` | owned by packet (to be filled by coordinator).

---

## 1. WasmPluginRuntime

### Definition & Construction

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2726 | struct WasmPluginRuntime definition | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2748 | impl WasmPluginRuntime { | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2869 | pub fn load() constructor | |

### Arc<WasmPluginRuntime> Fields

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:319 | Wasm(Arc<WasmPluginRuntime>) variant in enum ProgramEntrypoint | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:223 | runtimes: HashMap<String, Arc<WasmPluginRuntime>> in IoRouterState | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:694 | runtimes: Mutex<HashMap<String, Arc<WasmPluginRuntime>>> in ArtifactInferenceRouter | |

### IoRouter::register_plugin & Method Calls

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:379 | fn register_plugin(&self, plugin_id: &str, runtime: Arc<WasmPluginRuntime>) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:461 | runtime.artifact_compose() call in IoRouter::compose | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:531 | runtime.io_run() call in IoRouter::run_io | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:563 | runtime.io_sniff() call in IoRouter::identify | |

### ProgramBridge

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:353 | pub fn from_wasm(plugin_id: String, runtime: Arc<WasmPluginRuntime>) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:358 | pub fn wasm_runtime(&self) -> Option<Arc<WasmPluginRuntime>> | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:103 | runtime.exchange() call in dispatch_command | |

---

## 2. ExtensionRuntime

### Definition

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3463 | pub struct ExtensionRuntime definition | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3469 | impl ExtensionRuntime { | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3486 | pub fn new() constructor | |

### Method Calls & References

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3648 | fn load_extension(arc: Arc<ExtensionRuntime>, ...) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3723 | Arc<ExtensionRuntime>::new() in load_extension | |

---

## 3. ProgramSupervisorState

### Definition

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2737 | pub enum ProgramSupervisorState definition | |

### Enum Variants

```rust
Loaded, Running, Crashed, TimedOut, Restarting, Quarantined, Unloaded
```

### References in WasmPluginRuntime

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2734 | pub supervisor_state: Mutex<ProgramSupervisorState> field | |

### State Transitions

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3063 | supervisor_state mutation in hello() | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3085 | supervisor_state check in exchange() | |

### Host-side ProgramSupervisorState (separate definition)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:46 | pub enum ProgramSupervisorState in HostState | |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:62 | supervisor: HashMap<String, ProgramSupervisorState> field | |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:76 | pub fn supervisor_state(&self, plugin_id: &str) -> Option<ProgramSupervisorState> | |

---

## 4. Exchange Methods (.exchange / plugin_exchange / wasm_program_exchange)

### WasmPluginRuntime::exchange

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3095 | pub fn exchange(&self, instance_id: u64, encoded_commands: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, PluginHostError> | |

### Call Sites (Rust)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:103 | runtime.exchange(instance_id, encoded) in dispatch_command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1910 | cluster.exchange(plugin_id, instance_id, command) in transaction callback | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1930 | cluster.exchange() in undo_group callback | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1945 | cluster.exchange() in run_transaction callback (contributed step) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1960 | cluster.exchange() in run_transaction callback (owner route) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3063 | self.exchange(instance_id, ...) in hello() | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3105 | self.exchange() in set_merge_policy() | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3126 | self.exchange() in resolve_conflict() | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3141 | self.exchange() in read_conflicts() | |

### Call Sites (TypeScript)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🟦️component.ts:2709 | await this.handle.exchange(this.instanceId, [encodeAppCommand(command)]) | |
| 🧰️framework/🛍️products/💻️os/🟦️component.ts:2826 | await this.handle.exchange(this.instanceId, []) in poll() | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:1146 | await handle.exchange(1, []) in concurrent test | |

### Host Cluster Layer

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1895 | pub fn exchange(&self, plugin_id: &str, instance_id: u64, command: Vec<u8>) in HostCluster | |

### Plugin-Web Materialize Bridge

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:89 | reply(..., "exchange", { value: await api.exchange(...) }) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:180 | return await plugin.exchange(instanceId, frames) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:187 | exchange: (instanceId, frames) => runSerialized(...) | |

### Run Module

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:992 | let frames = self.host.exchange(handle, commands) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:1395 | let frames = runtime.exchange(...) in apply_artifact_mutation_plan | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:1430 | let frames = runtime.exchange(...) in dispatch_artifact_inference | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:1585 | let response = runtime.exchange(*instance_id, encoded) | |

---

## 5. PluginWorkerClient

### Definition 1: Kernel (default/canonical)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1446 | //#region PluginWorkerClient | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1498 | class PluginWorkerClient { | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1619 | const pluginWorkerClients = new Map<string, PluginWorkerClient>() | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1620 | const activeWorkerByPluginId = new Map<string, PluginWorkerClient>() | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1642 | //#endregion PluginWorkerClient | |

### Definition 2: WGPU target (alternate/divergent)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts:49 | class PluginWorkerClient { | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts:223 | const client = new PluginWorkerClient(pluginId, moduleUrl) | |

### Divergence Notes

- **Kernel version**: 5s timeout + restart policy, generic message types
- **WGPU version**: Tuned for wgpu event loop integration, different restart strategy
- **Both**: Send/receive `AppCommand`/`AppFrame` over worker message port

### Constructor & Methods

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1500 | constructor(pluginId: string, moduleUrl: string) | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1545 | send(command: AppCommand): Promise<AppFrame[]> | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1615 | stop() | |

---

## 6. LeasePool / PluginModuleLease / createLeasePool / acquirePluginModule

### Definitions

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1699 | //#region 🪶️LeasePool | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1700 | export interface Lease<T> | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1715 | export interface LeasePool<T> | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1741 | export function createLeasePool<T>(...) | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1821 | //#region 🐚️PluginModuleLease | |

### PluginModuleLease Implementation

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1821 | export interface PluginModuleLease | |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1842 | const pluginModulePool = createLeasePool<PluginModuleContent>(...) | |

### acquirePluginModule Implementation

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:1850 | export async function acquirePluginModule(pluginId: string, moduleUrl: string): Promise<PluginModuleLease> | |

### Plugin-related Consumers (to be deleted)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:1098 | const { acquirePluginModule } = await import("@semio-tech/framework") | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:1104 | const lease = await acquirePluginModule("mock", moduleUrl) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️index.ts:6 | import { acquirePluginModule, pluginHandleForBridge } from "@semio-tech/framework" | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️index.ts:130 | Promise.all(pluginEntries.map((entry) => acquirePluginModule(entry.pluginId, entry.moduleUrl))) | |

### Generic Consumers (must survive relocation)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts:39 | export { acquirePluginModule, ... } | |
| 🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts:41 | export { createLeasePool, ... } | |
| 🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts:563 | describe("LeasePool evictNow (hot-swap reload eviction)", ...) test | |

### Target Location (after move to glue.ts)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts | (LeasePool, createLeasePool will be defined here) | |

---

## 7. pollster::block_on

### WGPU Glue (primary plugin-blocking calls)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:93 | pollster::block_on(future) in entry — GPU init | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:307 | pollster::block_on(self.shell.boot()) in Wgpu::init — **PLUGIN GUEST BOOT** | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:346 | pollster::block_on(self.shell.pump_sync_events()) in pump — GPU events | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:528 | pollster::block_on(self.shell.poll_world3d_assets()) — world state | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:541 | pollster::block_on(fetch_url_bytes(&item.url)) — asset fetch | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:551 | pollster::block_on(fetch_url_bytes(&item.url)) — asset fetch | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:555 | pollster::block_on(self.shell.poll_world3d_assets()) — world state | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:1206 | pollster::block_on(async { ... }) in run_smoke — **TEST RUNNER** | |

### Shell Component (primarily UI/rendering)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:3169 | pollster::block_on(mint_or_restore(&client, &env)) — session restore | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:5508 | pollster::block_on(shell.finish_dock_drag(...)) — UI interaction | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:6990 | pollster::block_on(shell.apply_os_command("os.toggleFullscreen", None)) — UI command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:7103 | pollster::block_on(shell.apply_os_command("os.resetDock", None)) — test UI command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:7111 | pollster::block_on(shell.apply_os_command("os.setLocale", Some("de"))) — test UI command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:7119 | pollster::block_on(shell.apply_os_command("os.setDriver", Some("compact"))) — test UI command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:7121 | pollster::block_on(shell.apply_os_command("os.setDriver", Some("default"))) — test UI command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:7132 | pollster::block_on(shell.apply_os_command("os.setThemeId", Some("mono"))) — test UI command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:7134 | pollster::block_on(shell.apply_os_command("os.setThemeId", Some("semio"))) — test UI command | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:12709 | pollster::block_on(shell.handle_shell_hit(&hit)) — UI interaction | |

### Other (unrelated to plugins)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🔨️modules/◻2d/⚙️engine/🦀️component.rs:27 | pollster::block_on(future) in 2D engine | |
| ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs:661 | use pollster::block_on in video plugin | |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:133 | pollster::block_on(future) in stdio plugin | |

---

## 8. load_wasm_plugins / run_smoke / run_native

### load_wasm_plugins Definition

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:676 | pub fn load_wasm_plugins(plugin_filter: &str, modules_root: &std::path::Path) -> Result<Vec<ProgramBridgeEntry>, String> | |

### load_wasm_plugins Call Sites (eager plugin loading path)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:60 | use program_bridge::load_wasm_plugins | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:299 | let entries = match load_wasm_plugins(&plugin_filter, &modules_root) — Wgpu::init | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:1100 | let entries = filter_plugins(load_wasm_plugins(...), &plugin_filter) — run_smoke | |

### run_native Definition & Call Site

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:1188 | pub fn run_native(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️bin.rs:18 | run_native(&plugin_filter, modules_root) — main entry | |

### run_smoke Definition & Call Sites

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:1205 | pub fn run_smoke(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) -> i32 | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️bin.rs:16 | std::process::exit(run_smoke(&plugin_filter, modules_root)) — main entry | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:1207 | let loaded = match load_wasm_plugins(...) — inside run_smoke | |

---

## 9. set_host_backbone_channel / HostBackboneChannel

### Definition

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:19455 | pub struct HostBackboneChannel | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:19457 | impl store::BackboneChannelPort for HostBackboneChannel | |

### Registration

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:19470 | store::set_host_backbone_channel(std::sync::Arc::new(HostBackboneChannel)) | |

### Exports

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:19699 | pub use host_port::{..., HostBackboneChannel} | |

---

## 10. INSTANCE_GUARD / clear_instance_guard / clearInstanceGuard

### Definition (Rust)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14817 | static INSTANCE_GUARD: Cell<u32> = const { Cell::new(0) } | |

### Implementation

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14904 | if INSTANCE_GUARD.get() > 0 { /* guard check */ } | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14907 | INSTANCE_GUARD.set(1) — guard set | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14914 | INSTANCE_GUARD.set(0) — guard clear (success) | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14920 | INSTANCE_GUARD.set(0) — guard clear (panic cleanup) | |

### clear_instance_guard Export (Rust)

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14932 | pub fn plugin_clear_instance_guard() | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:54 | fn clear_instance_guard() { plugin_clear_instance_guard() } in bindings | |

### Host-side clear_instance_guard

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3273 | pub fn clear_instance_guard(&self) -> Result<(), PluginHostError> | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:3277 | bindings.semio_framework_plugin().call_clear_instance_guard(&mut *store) | |

### TypeScript clearInstanceGuard Consumer

| Path:Line | Context | Packet |
|-----------|---------|--------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:109 | // clearInstanceGuard export comment | |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:147 | try { plugin.clearInstanceGuard?.(); } catch { /* guard heal is best-effort */ } | |

---

## 11. AppCommand:: and AppFrame:: Variant Usage

### Overview

Total occurrences: **304 for AppCommand**, **317 for AppFrame** across Rust and TypeScript.

### Key Variants Mentioned (C12→C13 removal scope)

| Variant | Usage Count | Likely Affected Packet |
|---------|------------|------------------------|
| UiSection | TBD | L1-effects? |
| SectionProbe | TBD | L1-effects? |
| RefreshUi | TBD | L0-commands |
| Effects | TBD | L1-effects |
| Events | TBD | L1-effects |
| Welcome | TBD | L0-commands |
| Hello | ~1-5 | L0-commands |
| AttachBackbone | TBD | L1-backbone |
| DetachBackbone | TBD | L1-backbone |

### Recommendation

Requires targeted variant-by-variant grep (too broad for manual audit). Suggest:
```bash
rg -n "AppCommand::UiSection|AppCommand::SectionProbe" /path/to/repo --glob "*.rs" --glob "*.ts"
```

---

## Files Touched by Multiple Packets

| File | Symbols Involved | Packets (potential) |
|------|------------------|-------------------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs | WasmPluginRuntime, ExtensionRuntime, ProgramSupervisorState, .exchange(), HostBackboneChannel, clear_instance_guard | L0-consumers, L1-effects |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs | WasmPluginRuntime, .exchange(), load_wasm_plugins | L0-consumers, L1-loader |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs | pollster::block_on, load_wasm_plugins, run_native, run_smoke | L0-consumers, renderer-specific |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts | PluginWorkerClient, LeasePool, createLeasePool, acquirePluginModule, PluginModuleLease | L0-consumers, L1-kernel |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts | PluginWorkerClient, .exchange(), clearInstanceGuard | L0-consumers, L1-bridge |

---

## Summary of Blast Radius

- **Total files with changes required**: 20+
- **Most-touched files**: plugin host component.rs, wgpu glue.rs, kernel component.ts
- **Critical path**: WasmPluginRuntime.exchange + .load + supervisor_state
- **Async conversion scope**: pollster::block_on callsites (8 in wgpu, 11 in Shell)
- **Registrar lease scope**: IoRouter + ArtifactInferenceRouter (both hold Arc<WasmPluginRuntime>)
- **LeasePool relocation impact**: 4+ consumers affected if helper moves to glue.ts

---

**Report generated**: 2026-08-17 | **Status**: Ready for coordinator packet mapping
