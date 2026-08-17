# 📓️ Design — runtime / kernel

Source of truth for packets `A1-actor`, `B1-host-native`, `H1`–`H4`, `T1-tasks`, `P1-process`.

## 1. New framework module `🧰️framework/🔨️modules/🎭️actor/`

Crate `semio-framework-actor` (`📦️packages/🦀️rust`, `crate-type = ["rlib","cdylib"]`), package `@semio-tech/framework-actor` (`📦️packages/🟦️typescript`). Domain-neutral and **pure**: no I/O, no clock (callers pass `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread` in the crate core. Compiles for native, `wasm32-unknown-unknown` and (by construction) any future mobile target. Layout:

```
🎭️actor/
  🦀️component.rs            # all regions below
  🟦️component.ts            # TS surface: re-exports 🤖️generated + ShardClient types
  🤖️generated/🟦️actor.ts    # ts-rs mirror (typegen feature)
  📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs,📜️script.ts,📋️project.json}
  📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,🧵️shard-client.ts}
```

`📦️glue.rs` carries the `wasm_bindgen` `KernelHost` wrapper **behind `#[cfg(target_arch = "wasm32")]`** (bytes in / bytes out only), so the pure core stays clean. `📜️script.ts` gains a `wasm` command using `buildWasmPack` from `🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`. **This build is a hard prerequisite of the React dev target** — the `store_worker` cdylib precedent in this repo is declared but never built, so `🟦️backbone-worker.ts` silently falls back forever; do not repeat that.

`🎠️kernel/🦀️component.rs` gets `//#region 🎭️RuntimeActors` re-exporting `ActorId as RuntimeActorId, Lane, Budget, Envelope, TurnResult, ActorStatus, FailureStage, KernelMetrics` (mirrors the existing `protocol_core` re-export idiom at L40).

### Types (Rust SSOT; serde + `cfg_attr(typegen, derive(ts_rs::TS))`; hand-rolled pack codec beside each; one `//#region` per type)

- `PackageId(String)` = `<plugin>` or `<plugin>/<extension>`; `PackageHash([u8;32])` (blake3 of component bytes; the compiled-cache key).
- `ActorId(u64)` bit-packed `plugin_ordinal:u16 | kind:u2 | ordinal:u32 | generation:u14`. Generation makes restart-after-trap addressable without id reuse.
- `ActorKind { PluginApp{plugin, app_id, instance_id:u32}, Extension{plugin, extension_id}, Job{owner: ActorId, job_id:u64} }`.
- `Lane { Interactive, UserVisible, Background, Maintenance }`.
- `Budget { fuel:u64, wall_ms:u32, memory_bytes:u64, ui_nodes:u32, mailbox_len:u16, max_effects:u32, max_patch_bytes:u32 }`; `//#region ⚖️LaneDefaults` — Interactive 4 ms / 2 M fuel, UserVisible 16 ms, Background 50 ms, Maintenance 200 ms. **Replaces `PLUGIN_FUEL_BUDGET`.**
- `Envelope { to: ActorId, from: Origin{Ui{window}|Actor(ActorId)|Kernel|Bus{topic}}, lane, seq:u64, deadline_ms: Option<u64>, coalesce: Option<CoalesceKey>, cancel_of: Option<u64>, payload: Payload }`; `Payload { Event(Vec<u8>) | Suspend{checkpoint:bool} | Resume{checkpoint:Option<Vec<u8>>} | Cancel(u64) | JobStep{job:u64} }`.
- `CoalesceKey(String)` — latest-wins per `(actor, key)`. Pointer-move, resize, presence, refresh all coalesce; 200 stale mouse-moves must never queue.
- `TurnResult { ui_patches: Vec<UiPatch>, effects: Vec<Effect>, next_wake: Option<u64>, status: TurnStatus{Idle|MoreWork|CheckpointReady|Faulted(Vec<u8>)}, usage: Usage{fuel, wall_us, memory_bytes} }`.
- `Mailbox` — bounded ring per actor, lane priority, coalescing map, `pressure()`. `enqueue` returns `Backpressure { Accept | Coalesced | Dropped(Lane) | Rejected }`; `Rejected` surfaces as a busy badge in the UI, never as a silent drop of a user action.
- `ActorRecord { id, kind, package, shard: ShardId, capabilities: Vec<CapabilityGrant>, budget, mailbox, status: ActorStatus{Cold|Activating|Active|Suspended{checkpoint}|Draining|Trapped|Quarantined|Disabled}, failure: FailureState, metrics: ActorMetrics }`.

### `//#region ⏱️Scheduler`

Hierarchical **deficit round-robin**, two levels: (1) across plugins, (2) across that plugin's actors (app instances / extensions / jobs). Without level 1 a plugin with 50 extensions would take 50× the share of a plugin with none. Pure function:

```rust
fn tick(&mut self, now_ms: u64) -> Decision   // Decision { run: Vec<TurnGrant>, wake_at: Option<u64> }
// TurnGrant { actor, shard, budget, envelopes: Vec<Envelope> }
```

Lane weights bias selection; `deadline_ms` on interactive envelopes short-circuits ahead of background deficit. Throttled actors get `budget × factor` and reduced weight.

### `ShardTable`

Fixed pool: native `available_parallelism()-1` clamped `[2,8]`; web `min(hardwareConcurrency-1, 4)`. `ShardKind { Thread, WebWorker, Process }`. An actor is **pinned** to a shard; migration happens only at a quiescent point via application-level checkpoint (never a linear-memory snapshot: raw memory carries runtime-specific representation and stale handles). `request_exclusive(actor)` moves a heavy/untrusted actor onto one of ≤2 exclusive shards for the duration of foreground work — a lease, not a permanent per-plugin worker.

### `//#region 🚑️FailurePolicy`

`FailureStage { Healthy, Warned, Throttled{factor}, Suspended{until}, Cancelled, Trapped{restarts}, Quarantined{until}, Disabled }` driven by `FailureSignal { DeadlineOverrun{ratio}, FuelExhausted, MemoryLimit, MailboxOverflow, UiQuota, Trap(String), HeartbeatMissed{count}, ManualReset }`. Exponential thresholds per lane; decays to `Healthy` after N clean turns. `Trapped` → drop + re-instantiate (generation++) + restore last checkpoint. `Quarantined` is **package-wide** (all actors of the plugin). `Disabled` persists until the user re-enables in the task manager. Both existing `ProgramSupervisorState` enums are deleted; `PluginHost.supervisor` becomes a read view over `KernelMetrics`.

Watchdog: **native** = epoch deadline (hard, enforced by wasmtime). **Web** = SAB heartbeat (`Atomics.store(sab, shardIdx, turnSeq)`; COOP/COEP is already served so `SharedArrayBuffer` is available, with a `postMessage` fallback path that must keep correctness without SAB). Turn started with no heartbeat within `2×wall_ms` → `HeartbeatMissed`; ×3 → `ShardClient.rebuild()`, every actor on that shard `Trapped` and restored. Terminating a worker is the browser's only hard recovery, so guests additionally carry SDK-side budget checks between commands and inside job steps.

### `//#region 🖼️Scene`

`SceneStore` per window holds immutable `Arc<Scene>` snapshots. Patches apply into a builder; `Kernel::commit_frame(now)` publishes a new snapshot **once per frame boundary**. If an actor misses the deadline the previous snapshot is reused. **The UI thread never waits on a plugin.** `Budget.ui_nodes` and `max_patch_bytes` are enforced host-side while applying (no guest trust); breach → `FailureSignal::UiQuota`, truncate + warn.

### `//#region 📈️Metrics`

`ActorMetrics { turns, fuel_total, wall_us_total, wall_us_p95 (ring 64), memory_bytes, mailbox_len, mailbox_lag_ms, coalesced, dropped, traps, restarts, stage, shard }`, `ShardMetrics { actors, busy_ratio, heartbeat_age_ms }`, `KernelMetrics { actors, shards, packages }`. Sampled by `Kernel::metrics()` and published as bus topic `os.runtime.metrics` at 2 Hz.

### `ShardTransport`

```rust
pub trait ShardTransport: Send { fn send(&self, bytes: &[u8]); fn recv(&self) -> Option<Vec<u8>>; fn heartbeat(&self) -> u64; fn kill(&self); }
```
Impls: `ThreadTransport` (mpsc), `WorkerTransport` (postMessage, injected from TS), `ProcessTransport` (stdio, length-prefixed, **last wave**). All three carry the **same** `Envelope`/`TurnResult` pack encoding — that is the thread-or-process seam.

### `Kernel`

`submit(Envelope)->Backpressure`, `tick(now_ms)->Decision`, `complete(actor, TurnResult)`, `activate(package, ActivationEvent)`, `suspend/resume(actor)`, `request_exclusive(actor)`, `commit_frame(now)`, `metrics()`. One implementation, three hosts:

- **wgpu native** — `Kernel` on a dedicated kernel thread; shards are OS threads; the winit thread only submits envelopes and drains outbound in `about_to_wait`/`user_event`, woken via `EventLoopProxy`.
- **wgpu web** — the same `Kernel` inside the already-wasm wgpu module on the main thread; shards driven through `ShardClient` via wasm-bindgen imports.
- **React web** — `Kernel` loaded as the `semio-framework-actor` wasm (`KernelHost`) on the main thread.

## 2. `GuestRuntime` (packet B1, in `🔌️plugin/🖥️host/🦀️component.rs`)

Replaces regions `🔖️WasmPluginRuntime` and `🔖️ExtensionRuntime` entirely.

```rust
pub trait GuestRuntime: Send + Sync {
    fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError>;
    fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[CapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError>;
    fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault>;
    fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault>;
    fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError>;
    fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError>;
    fn drop_instance(&self, inst: GuestInstance);
}
```

**`WasmtimeRuntime`** (`//#region 🦀️WasmtimeRuntime`): ONE shared `Engine` for the process — `Config` with `wasm_component_model`, `consume_fuel`, `epoch_interruption`, `allocation_strategy(Pooling)` (`total_component_instances`, `max_memory_size = 512 MiB`, `linear_memory_keep_resident`), and an **on-demand fallback knob** if pooling rejects the configuration on a given host. An epoch ticker thread calls `engine.increment_epoch()` every 1 ms; `execute_turn` sets `store.set_epoch_deadline(budget.wall_ms)` **and** `store.set_fuel(budget.fuel)` per turn — the current code sets fuel once and `u64::MAX` epochs, which is why nothing is enforced today. `ResourceLimiter` per store bounds memory/tables/instances against the budget. Compiled artifacts cache to `~/.semio/cache/wasmtime/<engine-config-hash>/<PackageHash>.cwasm` via `Component::serialize` / `deserialize_file`. **One `Store<HostState>` per actor**; `HostState` slims to `{ plugin_id, actor, caps, effect_sink, asset_map }`. `IoRouter`, `InferenceRouter`, `MutationRouter`, `TransactionCoordinator`, `AppRouter`, `InstanceDirectory` stay in the file but are invoked by the kernel **after** a turn, from returned effects — never re-entrantly during one.

`ShardLoop` lives at `🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`: the loop a thread shard runs in-process and, in wave P1, the `semio-shard` `[[bin]]` runs over stdio.

`MockGuestRuntime` (`#[cfg(test)]`) with scripted turns and controllable time backs the scheduler/failure tests and replaces the `loadPluginModuleUncached` main-thread fallback that vitest relies on today (TS twin: `createMockShard()`).

## 3. Web shard (packet H2)

`🌐plugin-web-materialize.ts`: `pluginWorkerSource()` → `shardWorkerSource()`, emitting one **package-agnostic** `🟨️shard-worker.js` served from `/plugin-modules/_shard/`. It receives `{type:"activate", actorId, moduleUrl, caps, budget}`, dynamically imports the jco bridge, keeps `Map<actorId, {api, instance}>`, runs **one turn at a time per actor** (different actors interleave only at await points), heartbeats, and replies with `TurnResult` bytes tagged by `actorId`. `pluginComponentBridgeSource` drops the `runSerialized` retry/reload loop (recovery is the kernel's job now) and exposes `poll/startJob/stepJob/cancelJob/checkpoint/restore`. `🟨️host-shim.js` implements **only** the `pure` interface — the synchronous XHR blob calls and the `backbonePoll` shared queue disappear.

`ShardClient` (`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`): one `MessagePort` per shard, actor-id multiplexing, `terminate()+rebuild()`, `leaseExclusive(actorId)`. It replaces **both** `PluginWorkerClient`s and `pluginHandleForBridge`.

`ActivationRegistry` (`🎭️actor/🟦️component.ts` + Rust twin) replaces the kernel `LeasePool`: manifest-only records seeded from `PLUGIN_CATALOG` + build-time descriptors; activation events map to `Kernel::activate`; suspension by LRU + memory pressure with checkpoint; resume by restore. The generic `createLeasePool` moves **unchanged** to `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` for its three non-plugin users.

`guestSlimAssets` (the typst font blob) becomes a declared asset preloaded as a `blob` resource in `instance-open` — it is read during render, so it must be resident before the first `surface-visible`.

## 4. Task manager (packet T1)

OS module `💻️os/🔨️modules/📊️tasks/{🦀️component.rs,🟦️component.ts,📦️packages/…}`. Rust builds **one** `UiNode` tree (rows: plugin → actors; columns: cpu/fuel, memory, mailbox lag, stage, shard; actions: suspend / resume / restart / quarantine / enable / exclusive-lease), so both renderers display it through their existing UiNode paths. Window kind `os.tasks`, shell command `os.openTasks` registered via append-only regions in `ShellHost/🟦️component.tsx` and `Shell/🧊️component.rs` (sol applies both).

## 5. Mobile readiness (no backend built now)

The `🎭️actor` core and `GuestRuntime` carry no platform assumption; transports and clocks are injected. What a future mobile ticket must add: a `GuestRuntime` backed by wasmtime's Pulley interpreter where JIT pages are unavailable (iOS), a curated rather than open extension catalog, per-package explicit permission grants, and a searchable index of available software — Apple's Guideline 4.7 route, not a desktop-style marketplace. Recorded here so the kernel never grows a web/desktop-only assumption in the meantime.
