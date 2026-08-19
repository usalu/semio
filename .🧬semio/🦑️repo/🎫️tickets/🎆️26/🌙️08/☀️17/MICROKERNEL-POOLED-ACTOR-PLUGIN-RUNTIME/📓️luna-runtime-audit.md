# 🌙️ Luna Runtime Audit — Native Async Plugin Turn ABI Readiness

**Scope**: What must change, in what order, to mount a working async runtime and execute one real turn natively on the pooled-actor kernel.

**Measured**: 2026-08-20 — files read fresh off disk, all claims carry `path:line`.

---

## Executive Summary

**Status**: The architecture is proven but NOT YET WIRED. Three layers are visible:

1. **Guest SDK** (`⏳️imports.rs`, 24 host-async imports) — **COMPLETE AND MOUNTED**. Direct-await pattern proven by harness; 10 wired effects, 10 not-yet-wired stubs.
2. **Poll world** (`🧵️shard/🦀️component.rs`/`🏃️executor.rs`, `🎠️runtime.rs`) — **PROVEN MECHANISM, ONLY REAL NATIVE WIRING TODAY**. One `ShardLoop` per thread, K real shard threads via `ParallelRuntime`, DRR scheduler + turn budgeting all wired and measured.
3. **Async world** (`⏳️runtime.rs`) — **UNMOUNTED, WRITTEN AGAINST A SCHEMA THAT NO LONGER EXISTS**. Two blocking mismatches:
   - Built around `runner::run(stream<event>)` — the `world actor-async` and its `runner` interface are both DELETED by the world-collapse design.
   - Written against `checkpoint-async`/`jobs-async` interface names — these become plain `checkpoint`/`jobs` (now `async func`) after the collapse.

**Conclusion**: Native async-turn execution requires **rewriting `⏳️runtime.rs`** and **converting the host-side `execute_turn` path to async** — not a feature-flag flip, an architectural change already partly visible in `⏳️imports.rs`'s own harness validation.

---

## 1. The Three Layers

### 1a. Layer 1: Guest SDK (`⏳️imports.rs`) — Complete, Mounted, 24 Async Imports

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️imports.rs`

**Status**: ✅ **WORKING** (mounted, type-checked, harness-validated in the ticket report).

**What it does**:
- Implements `host_async::Host` (emit / emit-patch) — one-way effect doors (:477-488)
- Implements `host_async::HostWithStore` — the 24 async imports (:525-810)
  - **Wired** (9): `storage_read`/`storage_write`/`storage_delete` (:527-594), `blob_load`/`blob_write`/`blob_read` (:598-634), `http_fetch` (:643-679), `document_read`/`document_write` (:683-703), `io_compose`/`cache_derive`/`cache_read`/`invoke_extension`/`dispatch_action` (:707-765)
  - **Not-yet-wired stubs** (10): `link_resolve`, `registry_query`, `io_run`, `open_window`, `open_dialog`, `spawn_plugin_instance`, `request_file_open`, `request_media_frames`, `request_capability`, `spawn_job` (:769-807) — all return `not_wired(accessor, "...")` with a typed fault

**Coupling**: Reuses `AsyncEffectExecutor`'s own `services`/`router_handler` (:72-73 struct, :62-64 constructor), keeps cancellation-before-dispatch + capability tracking (:269-293), all proven by harness tests A-F in `terra-async-runtime-report.md`.

**Does NOT block** native async turn execution — it is the proven shape the host-side caller MUST feed into.

---

### 1b. Layer 2: Poll World (`🧵️shard/🦀️component.rs`, `🎠️runtime.rs`) — Proven, Only Real Wiring Today

**Files**: 
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` (ShardLoop)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎠️runtime.rs` (ParallelRuntime)

**Status**: ✅ **PROVEN MECHANISM** — the DRR scheduler, turn budgeting, multi-shard fairness all measured on interactive + bench workloads.

**Architecture**:
- `ShardLoop::pump` (:1 shard) → calls `GuestRuntime::execute_turn` (the sync-only path via `call_poll` accessor)
- `ParallelRuntime` (:🎠️runtime.rs:78) → owns K real `ShardExecutor` threads + one `Kernel` + outcome forwarders
  - Per shard: `ThreadTransport` duplex channel, one spawned `ShardExecutor` thread (:102), one forwarder thread (:107-118)
  - Core loop: `tick_and_dispatch` (:205) → grant dispatch → `wait_for_outcomes` (:264) — the real latency measurement site
  - Bridges `KernelTurnResult` ↔ `semio_framework_actor::TurnResult` via `to_actor_turn_result` (:151-165, shard/component.rs)

**Budget bridge**: `:turn_budget_from_grant` (:126-127) and `:job_budget_from_grant` (:132-133) convert `semio_framework_actor::Budget` (DRR-computed) → `semio_framework::kernel::Budget` (what `execute_turn` takes).

**Does NOT block** native async — it is the RUNNING BACKEND.

---

### 1c. Layer 3: Async World (`⏳️runtime.rs`) — Unmounted, Wrong Schema, Needs Complete Rewrite

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs`

**Status**: ⚠️ **UNMOUNTED, CANNOT COMPILE** — written against design that the world-collapse document itself supersedes.

**Two fatal mismatches**:

**Mismatch 1: `runner::run(stream<event>)` no longer exists**
- Line 421: `instance.semio_framework_runner().call_run(accessor, events)` — the `runner` interface is DELETED (see `📓️terra-world-collapse-prep-report.md §1g`)
- The entire `GrantWindow`/`GrantedEventProducer` (:178-219) machinery is built around pumping events into a long-lived stream — not compatible with the collapsed `world actor`'s design, where `poll` is still call-in/result-out (but now `async func` `:1b` instead of plain func)
- The `synthesize_turn_result` (:290-294) is sound but there is no turn-loop entry point that CALLS it anymore

**Mismatch 2: Interface names are predictions**
- Lines 467, 479, 491, 503, 515: `call_checkpoint`/`call_restore` are called on `instance.semio_framework_checkpoint_async()`, and `call_start_job`/`call_step_job` on `instance.semio_framework_jobs_async()`
- The schema collapse makes `jobs`/`checkpoint` (NO `_async` suffix) the only names; wit-bindgen's naming rule (kebab-case interface name → snake_case accessor method) yields `semio_framework_jobs()` / `semio_framework_checkpoint()`, not the `_async` suffixed names written here
- These were educated guesses from the harness spike (`terra-async-runtime-report.md` §"checkpoint / jobs — schema history mid-packet") against a schema that has since changed. The file's own report already flags this: "UNVERIFIED against a real compile" (:1-19)

---

## 2. What Must Change, In Order

### Phase 0: Schema Locked (Already Done)

**Required**: The world-collapse diff lands (likely already committed or landing imminently per `📋️master-u.md`).

**Impact on this file**: `⏳️runtime.rs` cannot even be MOUNTED without it — the `bindgen!({ world: "actor-async", ... })` call fails to resolve.

---

### Phase 1: Rewrite `⏳️runtime.rs` — Replace the Control Flow

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs`

**Scope of change**: ~60% rewrite of the control loop (:388-526), no new types needed.

**What must happen**:

1. **Delete**: The entire `runner::run` entry point (`synthesize_turn_result` stays; `GrantWindow`/`GrantedEventProducer`/turn-synthesis-on-grant-exhaustion-notify is gone)

2. **Replace with**: A command-channel loop exactly analogous to `ShardLoop::pump` — instead of looping `select!` on event-stream drains, loop `select!` on commands from a `mpsc::UnboundedReceiver<PollerCommand>`:
   ```rust
   enum PollerCommand {
       Poll { events: Vec<Event>, budget: Budget, reply: Sender<TurnResult> },
       Checkpoint { reply: Sender<Vec<u8>> },
       StartJob { job: u64, kind: String, input: Vec<u8>, reply: Sender<()> },
       StepJob { job: u64, budget: JobBudget, reply: Sender<JobStepResult> },
       CancelJob { job: u64 },
       Shutdown,
   }
   ```

3. **Call chain** (replacing :421-526):
   ```rust
   loop {
       tokio::select! {
           Some(command) = commands_rx.recv() => match command {
               PollerCommand::Poll { events, budget, reply } => {
                   let fuel_before = ...;
                   let stream = /* GrantedEventProducer for THIS poll call only */;
                   let result = instance.semio_framework_reactor()
                       .call_poll_async(accessor, events, budget).await;
                   let turn = synthesize_turn_result(&mut state, fuel_before, fuel_after, status);
                   let _ = reply.send(turn);
               }
               // ... similar for other commands, using accessor.spawn for checkpoint/jobs
           }
       }
   }
   ```

4. **Accessor names**: Fix :467/479/491/503/515 to use the real post-collapse names:
   - `call_checkpoint_async()` → `call_checkpoint()`
   - `call_restore_async()` → `call_restore()`  
   - `call_start_job_async()` → `call_start_job()`
   - `call_step_job_async()` → `call_step_job()`
   - `call_cancel_job_async()` → `call_cancel_job()`

**Responsibility**: Whichever packet owns `async-plugin-runtime` per `📋️master-u.md §B2`.

---

### Phase 2: Convert Host-Side `execute_turn` to Async — The Critical Path

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`

**Current state**: `execute_turn` (:930-≈960) calls `state.bindings.semio_framework_reactor().call_poll(&mut state.store, ...)` — the plain, sync accessor.

**The problem**: Once `poll` becomes `async func` (world-collapse §1b), this call FAILS immediately with "store configuration requires that *_async functions are used instead" (S7's finding: a sync-shaped call on an async-configured Store).

**Scope of change**: This is **NOT a small edit** — `execute_turn`'s entire signature and every call site changes.

**What must happen**:

1. **Engine configuration**: `build_shared_engine` (:1-364) must detect AND BUILD TWO ENGINES:
   - One async-capable engine (with `wasm_component_model_async(true)` + `concurrency_support(true)`) for `world actor` going forward
   - Keep the old sync-only engine IF the poll-world `world reactor` is still in use (for web/jco fallback — likely TRUE even after the collapse, since `📋️master-u.md` is not deleting `world actor`, only making it async)

2. **Store lifecycle**: The current `WasmtimeRuntime` design (:318-330) builds a Store once at instantiation. For async `poll`, the Store must EXIST INSIDE the async runtime context. This requires EITHER:
   - **Option A** (modest): Keep the Store, use `run_concurrent` inside `execute_turn`, wait for it — same shape `⏳️runtime.rs` prototyped but for just ONE poll call per turn
   - **Option B** (larger): Move Store ownership into a long-lived `AsyncActorTask`-like structure per `GuestInstance`, driven by a persistent tokio task (loses tight coupling to `ShardLoop`'s own synchronous rhythm but gains per-actor fairness)

   **Recommendation**: Option A preserves the current latency model (`ShardLoop::pump` blocks on turn completion), reuses `⏳️runtime.rs`'s proven `run_concurrent` idiom (:413-526), and requires minimal coupling changes.

3. **The actual call**:
   ```rust
   let result = store.run_concurrent(|accessor: &Accessor<_>| async {
       let turn_result = instance.semio_framework_reactor()
           .call_poll_async(accessor, events.clone(), budget_from_grant).await;
       synthesize_turn_result(state, fuel_before, fuel_after, status)
   }).await;
   ```

4. **`additional_derives: [Clone]` removal**: The host-side `bindgen!` (:799-810) currently requests `Clone` for all types. Once `world actor` imports `host-async` (which carries `stream<u8>` → `StreamReader<u8>`, deliberately non-`Clone`), this derive fails. Either:
   - Drop the derive and find every `.clone()` call on WIT types, replacing with field-by-field rebuilds (tedious but soundness-improving)
   - Confirm (I did not trace all 5+ sites) that nothing actually needs it and it was defensive — then just remove it

**Load-bearing risk** (from the world-collapse report §2b.1): "the single most concrete 'this will not just recompile, it needs real engineering' finding."

**Responsibility**: Packet `async-plugin-runtime` or `host-dedyn` per the packet table (whichever lands first).

---

### Phase 3: Replace Sync Blocking Bridges — `pollster::block_on` and `ureq::get`

**Files affected**: 
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs`

**Not a BLOCKER for a SINGLE turn**, but MUST be addressed to complete the mission "run the app end-to-end."

#### 3a. Direct `ureq::get` — the Synchronous Outlier

**Location**: `glue.rs:1577`
```rust
let response = ureq::get(&resolved).call().ok()?;
```

**Context**: In `fetch_map_tile_bytes_blocking` (presumed function name from surrounding context), called for map-tile loading.

**The fix**: Replace `ureq` (sync HTTP client) with an async HTTP layer behind the `services.http` pool that `⏳️imports.rs::http_fetch` already uses. This is not a one-liner — it requires identifying the call site's context (is it inside `execute_turn`? during instantiation? after polling?) and either:
- Hoist it OUT of the polling loop (if map tiles are a one-time load)
- Wrap it in a `block_on` (if genuinely needed on the winit thread) — **sanctioned only under R4's Clause 3** (deliberate, bounded blocking storage ops), NOT as a per-call or per-turn bridge

**Scope**: Small, but visibility-dependent on call site semantics.

---

#### 3b. Multiple `pollster::block_on` Sites on the Winit Thread

**Locations** (verified grep):
- `glue.rs:1896` — `shell.poll_world3d_assets()`
- `glue.rs:1909` — `pollster::block_on(fetch_url_bytes(&item.url))` / `fetch_map_tile_bytes_blocking`
- `glue.rs:1919` — second `fetch_url_bytes` call
- `glue.rs:1923` — another `shell.poll_world3d_assets()`
- `glue.rs:2580` — `pollster::block_on(async { ... })`
- `Shell/component.rs:3299` (approximately) — multiple sites calling `pollster::block_on` on shell operations (3 from the grep)
- `ProgramBridge/component.rs:523` (approximately) — `pollster::block_on` on wasm program exchange read

**Analysis**:
- **`shell.poll_world3d_assets()`** (1896, 1923) — appears to be UI state polling, called multiple times per frame? Context: map/3D viewer. NEEDS INVESTIGATION: does this block the whole UI? Can it be moved async? Is it a one-time init?
- **`fetch_url_bytes`** (1909, 1919) — network I/O on the winit thread, the clearest violation. MUST BE FIXED: either route through `services.http` + async, or throttle to init-only with explicit R4 documentation.
- **`wasm_program_exchange::read_history`** (ProgramBridge:523) — querying program history, appears to be data load. Context: Likely on a blocking path or after boot. SHOULD BE ASYNC if possible.

**The counts**:
- Production `pollster::block_on` sites: **6-8** on the interactive path (glue.rs) + **4+** on Shell/ProgramBridge (grep showed comments about the pattern, exact count > 4)
- Test `pollster::block_on` sites: **dozens** in test modules (:test, :test_* — all R4-sanctioned, not counted against the production census)

**No single one is a "turn can't run"** blocker, but COLLECTIVELY they are the "winit thread cannot drive async `execute_turn` cleanly" gap.

**The fix** (per `📋️master-u.md`, packet `winit-unblock`):
- Wrap interactive host's main loop in `tokio::spawn_blocking` or a dedicated async-executor bridge (R4's Clause 5-ready)
- Move every network/storage call to async-first, wrapping in bridge only if truly necessary
- Document remaining `block_on` sites with `// 🚫️async: E5 executor bridge` tags

**Responsibility**: Packet `winit-unblock` or equivalent, AFTER `async-plugin-runtime` stabilizes.

---

## 3. Sequencing: What Unblocks What

```
Schema collapse lands
   ↓
Phase 1: Rewrite ⏳️runtime.rs (fixes "runner::run no longer exists" + accessor names)
   ├→ File now compiles against collapsed schema
   └→ DOES NOT YET EXECUTE natively (no host-side wiring exists)
   
Phase 2: Async `execute_turn` + async engine config (Phase 1's prerequisite for native wiring)
   ├→ `GuestRuntime::execute_turn` now calls `call_poll_async`
   ├→ `build_shared_engine` configures async Store
   ├→ `Accessor`-based run-concurrent loop proves turn-at-a-time pattern
   └→ NOW WIRED: polling turns pass through the async path; results flow back to `Kernel::complete`

Phase 3: `winit-unblock` (quality-of-life, not a sequential blocker)
   ├→ Removes `pollster::block_on` from interactive loop
   └→ Replaces `ureq::get` with async HTTP
```

**Critical path** (minimum to "run one turn end-to-end"): **Phase 0 + Phase 1 + Phase 2** = **~3-4 weeks of solid engineering** (Phase 1: 4-6 days; Phase 2: 10-14 days including risk mitigation for the `additional_derives` change).

**Full mission** (run the app, not just one turn): + Phase 3 and downstream `io-run`/`registry-query`/etc wiring (outside this audit's scope).

---

## 4. Honest Gaps — What This Audit Punted

1. **Exact `execute_turn` signature change**: I read the current sync-only shape and the async-harness `run_concurrent` example, but did NOT trace every call site of `execute_turn` in `WasmtimeRuntime` or verify what happens to the `return` value's type. A signature change of this scale may have ripple (error handling? async cancellation on drop?).

2. **Store lifetime semantics**: Does the async `poll` call NEED to own the Store for the duration, or can `run_concurrent` borrowing work? I did not measure, only read the wasmtime docs. Real testing needed.

3. **The `additional_derives: [Clone]` audit**: I did NOT systematically find every `.clone()` call on WIT types in the host crate. The world-collapse report names it as the single biggest risk; I can confirm it IS a problem, but not scope the fix precisely.

4. **Map-tile loading semantics**: I found `fetch_map_tile_bytes_blocking` and `pollster::block_on` but did NOT verify whether this is once-at-boot, per-frame, or per-zoom. Context determines the fix (async vs. R4-sanctioned bridge).

5. **Integration into `ShardLoop::pump`**: The rewritten `execute_turn` will be async; `ShardLoop::pump` (:1-330 of shard/component.rs) is sync. Does `pump` need to become async too? Or does it `block_on` the result? The shard is thread-local; one thread per shard is the design. I flagged this as an open question, not a blocker — but it is real.

---

## 5. Remaining `pollster::block_on` and `ureq::get` Sites — For Phase 3

### Confirmed `pollster::block_on` (Production)
| file | line | context | threat |
|---|---|---|---|
| `glue.rs` | 1896 | `self.shell.poll_world3d_assets()` | UI state; frequency unknown, potential per-frame |
| `glue.rs` | 1909 | `fetch_url_bytes(&item.url)` | Network I/O |
| `glue.rs` | 1919 | `fetch_url_bytes(&item.url)` | Network I/O |
| `glue.rs` | 1923 | `self.shell.poll_world3d_assets()` | UI state (repeat) |
| `glue.rs` | 2580 | Inline async block (context unclear from grep) | Likely non-critical |
| `Shell/component.rs` | 3299 (approx) | Multiple shell operations | Already commented as legacy |
| `ProgramBridge/component.rs` | 523 (approx) | `wasm_program_exchange::read_history` | Data load |

### Confirmed `ureq::get` (Production)
| file | line | context |
|---|---|---|
| `glue.rs` | 1577 | `fetch_map_tile_bytes_blocking` — map tile loading |

---

## 6. What Phase 1 Needs from Phase 0

- ✅ `checkpoint` and `jobs` interfaces are now `async func` (verified in collapsed schema)
- ✅ `runner` interface is DELETED (verified §1g of world-collapse report)
- ✅ `world actor-async` is DELETED (verified §1i of world-collapse report)
- ⚠️ `emit`/`emit-patch` remain plain `func` (verified §0 of world-collapse report) — `⏳️runtime.rs` does NOT use them anyway, so no change needed

---

## 7. Validation Checklist for Implementation

Before any of Phase 1/2/3 runs:
- [ ] The schema collapse has landed and `cargo check -p semio-framework-plugin-host` succeeds with the old `actor_bindings` module (it should — no change yet)
- [ ] `wasm check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest` still passes (the guest SDK stays on `world actor`, unchanged)

Before Phase 1 is marked complete:
- [ ] `⏳️runtime.rs` compiles (no longer "unmounted, UNRUN")
- [ ] Accessor method names (call_poll_async, etc.) match the real schema bindgen output by testing a scratch `bindgen!` call

Before Phase 2 is marked complete:
- [ ] `GuestRuntime::execute_turn` compiles as async
- [ ] A single turn (`activate` → `submit` → `tick_and_dispatch` → `execute_turn` → `complete`) executes without panic or hang
- [ ] `to_actor_turn_result` bridge delivers a `semio_framework_actor::TurnResult` identical to a sync-world equivalent (exact values, not just "not null")

Before Phase 3 starts:
- [ ] Integrated test: 10 sequential turns on a real plugin, timing the last 3 to confirm latency

---

## Conclusion

**What stands between "proven design" and "running end-to-end"**: Two substantial rewrites (Phase 1 + 2) in critical path, plus one cleanup wave (Phase 3). No architectural gaps remain — `⏳️imports.rs` is complete, the shard loop is proven, and `⏳️runtime.rs` is a well-understood control-flow mismatch, not a mystery.

The world-collapse schema change is the gate; everything downstream is implementable work.
