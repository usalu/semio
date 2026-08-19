# Sync Surface Audit: Blocking Operations Census

**Date**: 2026-08-19  
**Scope**: Read-only census of all blocking/synchronous operations  
**EXCLUDE**: `compose/`, `*/target*`, `node_modules`, `.🧬semio/`

## Executive Summary

The Rust framework/os/plugins codebase contains **multiple dedicated blocking surfaces** preventing async-first execution. Despite async infrastructure landing (`semio-framework-async`, `AsyncEffectExecutor`), the execution core remains synchronous. This audit locates every `block_on`, blocking I/O primitive, and sync execution signature.

---

## 1. BLOCK_ON CALLS (Production)

### Database Storage Layer (🛢️db/)

#### PostgreSQL Backend (`🛢️db/🗄️storage/🐘️postgres/🦀️component.rs`)

**Owned Thread**: DB actor mailbox (plain `std::thread`, not tokio)

- **Line 102**: `runtime.block_on(async { pool_connect + bootstrap_schema })`
  - **Context**: `PostgresStorage::connect()`
  - **Purpose**: Bridges sync trait methods onto sqlx async-only Postgres driver
  - **Duration**: One-time at initialization

- **Lines 121-126**: `block_on()` method definition
  ```rust
  fn block_on<F: std::future::Future>(&self, fut: F) -> F::Output {
      match tokio::runtime::Handle::try_current() {
          Ok(handle) => tokio::task::block_in_place(|| handle.block_on(fut)),
          Err(_) => self.runtime.block_on(fut),
      }
  }
  ```
  - **Pattern**: Dual-path bridging (escape via `block_in_place` if inside tokio, else own runtime)
  - **Calls**: Every trait method (205+lines of impls)

- **Per-trait-method calls** in trait impls (non-exhaustive list):
  - Line 205: `WalStorage::create_segment`
  - Line 218: `WalStorage::append`
  - Line 247: `WalStorage::seal`
  - Line 257: `WalStorage::read`
  - Line 270: `WalStorage::segment_len`
  - Line 277: `WalStorage::list_segments`
  - Line 285: `WalStorage::truncate_tail`
  - Line 300: `WalStorage::delete_segment`
  - Line 312: `SnapshotStorage::write_generation`
  - Line 329: `SnapshotStorage::read_generation`
  - Line 340: `SnapshotStorage::latest_generation`
  - Line 347: `SnapshotStorage::list_generations`
  - Line 355: `SnapshotStorage::delete_generation`
  - Line 368: `PayloadStorage::put`
  - Line 375: `PayloadStorage::get`
  - Line 385: `PayloadStorage::contains`
  - Line 392: `PayloadStorage::delete`
  - Line 399: `PayloadStorage::len`
  - Line 410: `CatalogStorage::read_root`
  - Line 418: `CatalogStorage::cas_root`
  - Line 440: `IndexStorage::write_run`
  - Line 457: `IndexStorage::read_run`
  - Line 468: `IndexStorage::list_runs`
  - Line 476: `IndexStorage::delete_run`
  
  **Total**: 24+ trait method implementations, each with `self.block_on(async { ... })`

#### Neo4j Backend (`🛢️db/🗄️storage/🌐️neo4j/🦀️component.rs`)

**Owned Thread**: DB actor mailbox (plain `std::thread`, not tokio)

- **Line 361**: `runtime.block_on(Graph::connect(config))`
  - **Context**: `Neo4jStorage::connect_with_config()`
  - **Purpose**: Bridge sync method onto neo4rs async driver

- **Line 378**: `block_on()` method definition
  ```rust
  fn block_on<F: std::future::Future>(&self, fut: F) -> F::Output {
      self.runtime.block_on(fut)
  }
  ```

- **Per-trait-method calls** (measured from scope ~line 400-500+):
  - Line 399+: `WalStorage` trait methods (create_segment, append, seal, read, list_segments, truncate_tail, delete_segment)
  - Line 412+: `SnapshotStorage` trait methods (write_generation, read_generation, latest_generation, list_generations, delete_generation)
  - Line 426+: `PayloadStorage` trait methods (put, get, contains, delete, len)
  - Line 441+: `CatalogStorage` trait methods (read_root, cas_root)
  - Line 454+: `IndexStorage` trait methods (write_run, read_run, list_runs, delete_run)
  
  **Total**: 22+ trait method implementations

### Services Layer (🛎️services)

#### TokioHostRuntime (`🛎️services/🦀️component.rs`)

**Owned Thread**: Dedicated background threads (tokio I/O worker pool)

- **Line 263**: `epoch = runtime.block_on(async { tokio::time::Instant::now() })`
  - **Context**: `TokioHostRuntime::new()`
  - **Purpose**: Read clock from within the runtime's own context
  - **Duration**: One-time at initialization
  - **Thread**: Main/bootstrap thread (calling `.new()`)

### MCP Transport Layer (🌉️mcp)

#### HttpTransport (`🌉️mcp/🚚️transport/🦀️component.rs`)

**Owned Thread**: Process-wide/one-shot (stdio/single-threaded test harness)

- **Lines 239-240**: `runtime.block_on(self.run(server))`
  ```rust
  impl McpTransport for HttpTransport {
      fn serve(&mut self, server: McpServer) -> Result<(), GatewayError> {
          let runtime = tokio::runtime::Runtime::new().map_err(...)?;
          runtime.block_on(self.run(server))
      }
  }
  ```
  - **Context**: `HttpTransport::serve()` — Streamable HTTP gateway entry point
  - **Purpose**: Runs async axum server to completion from sync trait method
  - **Thread**: Stdio process thread (stdio transport test harness)

---

## 2. BLOCKING CHANNEL OPERATIONS

### Shard Executor (`🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`)

**Owned Thread**: Dedicated shard executor thread spawned per `ShardExecutor::spawn()`

#### recv_deadline (Blocking Receive with Timeout)

- **Line 136**: `let primed = park.recv_deadline(PARK_TIMEOUT);`
  - **Context**: Shard loop iteration, waiting for `ShardFrame` to arrive
  - **Type**: `ThreadTransport::recv_deadline(Duration::from_millis(5))`
  - **Duration**: Bounded by `PARK_TIMEOUT` (5ms), per-pump-iteration
  - **Purpose**: Non-blocking drain loop must still wake for job stepping and queued completions

#### recv_timeout (Blocking Receive with Timeout)

- **Line 168**: `let _ = ack_rx.recv_timeout(REGISTER_ACK_TIMEOUT);`
  - **Context**: Caller of `ShardExecutor::register()`, rendezvous synchronization
  - **Type**: `mpsc::Receiver::recv_timeout(REGISTER_ACK_TIMEOUT)` where `REGISTER_ACK_TIMEOUT = Duration::from_secs(5)`
  - **Duration**: Bounded by 5 seconds max
  - **Purpose**: Blocks caller until executor thread has applied registration to avoid race with immediate `Grant` frame

#### Test Harness recv_deadline

- **Line 241** (test): `transport.recv_deadline(Duration::from_millis(20))`
  - **Context**: `recv_with_retries()` test helper
  - **Scope**: Test-only, not production

---

## 3. BLOCKING I/O PRIMITIVES

### File System I/O (Plugin Host Compiled Cache)

#### Component Serialization/Deserialization (`🔌️plugin/🖥️host/🦀️component.rs`)

- **Line 284-290**: Compiled component cache write path
  ```rust
  pub fn store_compiled_component(component: &Component, path: &Path) -> std::io::Result<()> {
      if let Some(parent) = path.parent() {
          std::fs::create_dir_all(parent)?;      // BLOCKING
      }
      let bytes = component.serialize()?;
      std::fs::write(path, bytes)                // BLOCKING
  }
  ```
  - **Type**: `std::fs` operations
  - **Scope**: On-disk cache path (~/.semio/cache/wasmtime/)
  - **Thread**: Caller's thread (typically UI/plugin-host thread)
  - **Hotness**: Cold path (happens once per plugin package at load time)

- **Line 277-282**: Compiled component cache read path
  ```rust
  pub fn load_compiled_component(engine: &Engine, path: &Path) -> Option<Component> {
      if !path.exists() {                        // BLOCKING
          return None;
      }
      unsafe { Component::deserialize_file(engine, path).ok() }  // BLOCKING
  }
  ```
  - **Type**: `std::fs` operations
  - **Thread**: Caller's thread
  - **Hotness**: Cold path (load-time)

### HTTP I/O (ureq Blocking Client)

#### Directory Client (`📇️directory/🔌️client/🦀️component.rs`)

**Context**: Synchronous wrapper around async HTTP operations

- **Lines 374-486**: `UreqHttpTransport` implementation
  - **Type**: Uses `ureq::Agent::get()` / `ureq::Error` for blocking HTTP calls
  - **Pattern**: One `ureq::Agent` per transport, wrapped by `HttpPool` on `ComputePool` (dedicated thread pool)
  - **Hotness**: Active path (every HTTP request from plugins/DSL)

Example signature at line 385-406:
```rust
#[cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))]
pub struct UreqHttpTransport {
    agent: ureq::Agent,
}

impl HttpTransport for UreqHttpTransport {
    fn fetch(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        let method_obj = /* ureq::Agent method call */;
        // Blocks on HTTP round-trip
    }
}
```

#### Renderer Shell (`📺️renderer/.../🧊️wgpu/📦️glue.rs`)

- **Line 1577**: Direct `ureq::get()` call
  ```rust
  let response = ureq::get(&resolved).call().ok()?;
  ```
  - **Thread**: Likely UI/render thread
  - **Hotness**: Cold path (image asset loading)

### Thread Parking / Polling

#### pack/async Module (`🎒️pack/⏳️async/🦀️component.rs`)

- **Line 69**: `std::thread::sleep(std::time::Duration::from_micros(200))`
  ```rust
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
      if self.token.is_cancelled() {
          return Poll::Ready(Err(PackError::Io("read cancelled".to_string())));
      }
      cx.waker().wake_by_ref();
      std::thread::sleep(std::time::Duration::from_micros(200));
      Poll::Pending
  }
  ```
  - **Type**: Busy-polling with fixed sleep between checks
  - **Pattern**: Cooperative cancellation watch raced against real read futures
  - **Thread**: Async runtime worker (context depends on executor)
  - **Duration**: 200 microseconds per poll iteration (indefinite while pending)
  - **Issue**: Non-preemptive polling; true preemption via `CancelToken` event would eliminate this

#### Epoch Ticker Thread (`🔌️plugin/🖥️host/🦀️component.rs`)

- **Line 179**: `std::thread::sleep(std::time::Duration::from_millis(1))`
  ```rust
  loop {
      if !stop_thread.load(std::sync::atomic::Ordering::Relaxed) {
          std::thread::sleep(std::time::Duration::from_millis(1));
          engine.increment_epoch();
      }
  }
  ```
  - **Type**: Dedicated background thread sleep
  - **Thread**: Dedicated "semio-epoch-ticker" thread spawned per wasmtime Engine
  - **Duration**: 1ms interval, indefinite
  - **Purpose**: Drive wasmtime epoch counter for turn budget enforcement
  - **Owner**: `EpochTicker` (dropped to join thread on process shutdown)

#### Semio-Shard Binary (`🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs`)

- **Line 81**: `std::thread::sleep(Duration::from_millis(5))`
  - **Type**: Main loop sleep
  - **Thread**: Process main thread
  - **Duration**: 5ms per pump iteration
  - **Purpose**: Busy-loop backoff for `ShardLoop::pump()`
  - **Note**: Exact same `PARK_TIMEOUT` pattern as in-process `ShardExecutor`

---

## 4. SYNC EXECUTION CORE SIGNATURES

### GuestRuntime Trait (`🔌️plugin/🖥️host/🦀️component.rs`, lines ~490-510)

```rust
pub trait GuestRuntime: Send + Sync {
    fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError>;
    fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError>;
    fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault>;
    fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault>;
    fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault>;
    fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault>;
    fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError>;
    fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError>;
    fn drop_instance(&self, inst: GuestInstance);
}
```

**Characteristic**: Entirely synchronous. No `async fn`. All I/O (wasm instantiation, guest execution) is blocking.

### ShardLoop (`🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`, lines ~189-410)

#### pump method (line 275-276)
```rust
pub fn pump(&mut self) -> Result<usize, PluginHostError> {
    self.pump_primed(None)
}
```
- **Purpose**: Drive all registered actors one step, return actor count driven
- **Blocking**: Each `execute_turn` call blocks waiting for guest wasm to complete
- **Frequency**: Called repeatedly (per-iteration)

#### pump_primed method (line 285-410)
```rust
pub fn pump_primed(&mut self, primed: Option<Vec<u8>>) -> Result<usize, PluginHostError> {
    // Non-blocking drain of queued frames
    // Group by actor
    // For each actor: execute_turn (BLOCKING)
    // For each job: step_job (BLOCKING)
}
```

### ShardExecutor (`🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`, lines ~72-185)

```rust
pub struct ShardExecutor {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    register_tx: mpsc::Sender<RegisterRequest>,
}

// Main loop (lines 108-141):
loop {
    // Drain register requests (sync channel)
    while let Ok(RegisterRequest { actor, instance, ack }) = register_rx.try_recv() {
        shard.register(actor, instance);
        let _ = ack.send(());  // BLOCKING SYNC HANDOFF
    }
    // BLOCKING RECEIVE WITH TIMEOUT
    let primed = park.recv_deadline(PARK_TIMEOUT);
    // Pump the shard (all turns/jobs are BLOCKING)
    if let Err(error) = shard.pump_primed(primed) { ... }
}
```

**Owned Thread**: Dedicated "semio-shard-executor" OS thread spawned per `ShardExecutor::spawn()`

### ParallelRuntime Kernel Loop (`📺️renderer/.../🧊️wgpu/🎠️runtime.rs`)

**Reference**: `//#region 🎠️ terra-kernel-loop` in `📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` (lines 53-102, 286+)

**Owned Thread**: Winit event loop thread (real OS thread, not tokio)

- **Line 290**: `runtime: crate::parallel_runtime::ParallelRuntime`
  - Holds K `ShardExecutor` instances (K shards, K dedicated threads each)
  - Each shard runs its own `ShardLoop` pump cycle

- **Structure** (inferred from brief):
  1. Kernel submits envelopes to shards via `ThreadTransport` channels
  2. Each shard's executor thread runs `pump_primed` in a loop
  3. Main winit thread drains results and applies side effects
  4. **All plugin execution is synchronous blocking** within each shard's pump

---

## 5. SERVICES INTERNALS (Async vs Blocking Mix)

### TokioHostRuntime (`🛎️services/🦀️component.rs`, lines ~228-301)

**Status**: Pure async internally; bridges via `block_on()` only at edges

- **Line 263**: One `block_on()` for epoch read (initialization)
- **Trait implementation**: Fully async (`spawn_scoped`, `run_blocking`, `sleep_until`, `cancel_scope`, `now_ms`)
- **Threading**: Owns multi-thread `tokio::runtime::Runtime` with:
  - `plan.io_workers` I/O threads
  - `plan.compute` blocking-task threads
  - Dedicated epoch ticker thread

### TimerWheel (`🛎️services/🦀️component.rs`, lines ~304-484)

**Status**: Pure sync core (`WheelCore`), async driver task

- **Line 471**: `WheelCore::arm()` — synchronous, runs under `Mutex::lock()`
- **Line 490**: `spawn_driver()` — async background task
  - Driver loop polls `WheelCore::next_expiry_ms()` (sync)
  - Sleeps via `runtime.sleep_until()` (async)
  - Fires timers to `CompletionSink` (async callback)

### HttpPool / ComputePool (`🛎️services/🦀️component.rs`)

**Status**: Blocking work on dedicated thread pools

- **Line 635-637** (doc comment):
  ```
  /// call on a dedicated thread" technique `📇️directory/🔌️client` already uses for `ureq`.
  /// real transport (a `ureq`-backed one, or a real connection-pooling client)
  ```
  - Uses `tokio::task::spawn_blocking()` to push `ureq` calls onto compute pool
  - **Pattern**: Blocks within a tokio-bounded thread, not arbitrary blocking

### StorageScheduler

**Status**: Not fully visible in this audit, but contract declares:
- Takes sync `DbStorage` trait calls
- Runs them on **what kind of thread?** (File scope excluded `db` directory details)
- Likely uses `tokio::task::spawn_blocking()` like `ComputePool`

### EventRouter → CompletionSink

**Status**: Async wiring only
- No blocking surfaces found on this path

---

## 6. PACK/ASYNC MODULE DETAILS (`🎒️pack/⏳️async/🦀️component.rs`)

### AsyncPackSource Trait (lines 18-26)

```rust
#[async_trait::async_trait]
pub trait AsyncPackSource: Send + Sync {
    fn len(&self) -> u64;
    async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, PackError>;
}
```

**Status**: Pure async interface, no blocking surfaces

### ReadScheduler (lines 164-246)

- **Coalesces** overlapping reads
- **Deduplicates** identical in-flight requests
- **Priority-queues** via `BoundedDemand`

**Blocking surfaces**:
- **Line 69** (CancelWatch): 200µs `std::thread::sleep` in poll loop (cooperative cancellation)
- **Line 151** (WaitForGroup): `Mutex::lock()` on poll path (brief, no long-held locks)

**Provides**: Async-first API suitable for post-rewrite consumption

---

## 7. DISTINCTION: PRODUCTION vs TEST-ONLY

### Production Blocking

| Location | Line(s) | Type | Thread | Hotness | Duration |
|----------|---------|------|--------|---------|----------|
| PostgreSQL::connect | 102 | `block_on` | DB actor | Cold | One-time init |
| PostgreSQL trait impls | 205+ (24x) | `block_on` | DB actor | Hot | Per-I/O |
| Neo4j::connect | 361 | `block_on` | DB actor | Cold | One-time init |
| Neo4j trait impls | 399+ (22x) | `block_on` | DB actor | Hot | Per-I/O |
| HttpTransport::serve | 239-240 | `block_on` | Stdio/test | Cold | Process lifetime |
| Epoch ticker | 179 | `std::thread::sleep` | Dedicated bg | Always | 1ms per tick |
| ShardExecutor loop | 136 | `recv_deadline` | Shard executor | Always | 5ms per pump |
| register() ack | 168 | `recv_timeout` | Caller | On registration | 5s max (bounded) |
| Compiled cache I/O | 284-290, 277-282 | `std::fs` | Caller | Cold | Load-time |
| UreqHttpTransport | 385-406 | `ureq` blocking client | ComputePool | Hot | Per HTTP call |
| semio-shard binary | 81 | `std::thread::sleep` | Main process | Always | 5ms per pump |

### Test-Only Blocking

| Location | Line(s) | Type |
|----------|---------|------|
| ShardExecutor tests | 241 | `recv_deadline` (test harness retry loop) |
| Various `.expect()` calls | Throughout | Panic on lock poison (would be fatal in production anyway) |

---

## 8. WASMTIME CONFIG: ASYNC SUPPORT STATUS

### File: `🔌️plugin/🖥️host/🦀️component.rs` (~108-150)

```rust
fn build(pooling: bool) -> wasmtime::Result<Engine> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.consume_fuel(true);        // Epoch-based budget enforcement
    config.epoch_interruption(true);  // Per-turn deadline via epoch ticker
    // NO: config.async_support(true)  -- NOT ENABLED
    // NO: config.concurrency_support(true)  -- NOT ENABLED
    if pooling { ... }
    Engine::new(&config)
}
```

**Status**: `async_support` and `concurrency_support` are **NOT enabled** anywhere in this codebase. All wasm execution remains synchronous blocking.

---

## 9. SUMMARY TABLE: BLOCKING SITES BY CATEGORY

| Category | Count | Hotness | Severity | Rewrite Impact |
|----------|-------|---------|----------|---|
| **db block_on (init)** | 2 | Cold | Medium | Move to async startup |
| **db block_on (per-I/O)** | 46+ | **Hot** | **High** | Entire DB trait → async |
| **MCP block_on** | 1 | Cold | Low | Test harness only |
| **Services block_on** | 1 | Cold | Low | One-time init |
| **Shard recv_deadline** | 1 | **Always** | **High** | Core scheduling loop |
| **Shard recv_timeout** | 1 | On registration | Medium | Rendezvous sync point |
| **ureq blocking HTTP** | 2+ | **Hot** | **High** | Move to native async HTTP |
| **std::fs (cache I/O)** | 2 | Cold | Low | One-time plugin load |
| **Epoch ticker sleep** | 1 | **Always** | **High** | Dedicated bg thread, necessary |
| **pack/async sleep** | 1 | Frequent | Low | Replace with event-driven cancellation |
| **semio-shard sleep** | 1 | Always | **High** | Child process main loop |

---

## 10. CLAIMS: MEASURED vs INFERRED

### Measured (Exact line numbers, direct reading)

- ✓ All `block_on` locations (grep + file read)
- ✓ All `recv_deadline` / `recv_timeout` locations
- ✓ Compiled cache I/O (`std::fs`)
- ✓ `std::thread::sleep` calls
- ✓ `ureq::get()` call
- ✓ GuestRuntime trait signature
- ✓ ShardLoop pump/pump_primed signatures
- ✓ TokioHostRuntime initialization
- ✓ Wasmtime Config region (async support checks)

### Inferred (Architecture, not line-by-line verification)

- ⚠ ParallelRuntime kernel loop exact blocking surfaces (file excluded from search; inferred from references in glue.rs)
- ⚠ StorageScheduler thread affinity (file excluded; docs reference spawn_blocking pattern)
- ⚠ HttpPool internal use of `ComputePool` (type-level, not source-verified here)
- ⚠ Exact replica of blocking pattern across all 22+ neo4j trait impls (pattern verified at ~360, extrapolated)

---

## 11. IMPLEMENTATION NOTES FOR REWRITE

### Pre-Rewrite Checklist

1. **Identify ownership**: Every `block_on` call owns a `tokio::runtime::Runtime`. Consolidate to ONE per process.
2. **Thread safety**: `GuestRuntime` trait is `Send + Sync`. Async variant must preserve this.
3. **Shard isolation**: Each shard runs on its own thread + has its own `ShardLoop`. Rewrite must preserve this thread-per-shard model OR introduce cross-shard coordination.
4. **DB trait unification**: All 46+ `block_on` calls bridge the same gap (sync trait → async driver). Single unified async trait will eliminate all of them.
5. **Epoch ticker**: Not eliminable via async rewrite; it's a background service. Migrate to async timer via `HostAsyncRuntime::sleep_until()`.

### Critical Paths (No Hiding)

These blocking surfaces are on the critical path and cannot be hidden:

1. **Per-turn plugin execution** (ShardLoop pump) — every guest turn blocks
2. **DB reads/writes** (storage trait impls) — every I/O blocks
3. **HTTP calls** (ureq) — every remote call blocks
4. **Shard receive loop** (recv_deadline) — every pump iteration blocks briefly

**Rewrite scope**: ALL of these must become async or the codebase remains blocking-first.

---

## Appendix A: File Paths (Absolute)

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/⏳️async/🦀️component.rs`

---

**End of Audit Report**
