# Thread Census: Complete Inventory for Phase 1 Refactor

**Date:** 2026-08-20  
**Scope:** All OS-thread and thread-pool creation sites in /Users/ueli/Documents/semio  
**Exclusions:** Target build artifacts, node_modules, compose directory, archived ticket code

---

## A. Known Anchor Points (Verified Current State)

### A.1. ThreadPlan & ThreadBudget (`🧰️framework/🔨️modules/⏳️async/🦀️component.rs`)
- **Lines 275-365:** ThreadPlan struct and thread_plan() function
- **What:** Pure arithmetic computing thread counts per role
  - `kernel: 1`
  - `shards: ceil(cores/2)` clamped [2, 8]
  - `io_workers: ceil(cores/4)` clamped [1, 4]
  - `compute: max(1, cores - shards - io_workers - 1)`
  - `epoch_ticker: 1`
- **ThreadBudget struct (lines 321-365):** Atomic counters tracking permit ledger
  - Lines 355-359: `checkout()` uses `fetch_sub()` then `debug_assert!(previous >= n)`
  - **ISSUE:** In release builds, wrapping is silently allowed on over-draw (line 359 `wrapping_sub`)
- **Threads:** 0 threads created here; pure bookkeeping
- **Owned by:** Semio

### A.2. TokioHostRuntime (`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`)
- **Lines 238-326:** TokioHostRuntime struct and impl
- **Line 271-276:** `tokio::runtime::Builder::new_multi_thread()`
  - `.worker_threads(plan.io_workers as usize)` — I/O worker threads from ThreadPlan
  - `.max_blocking_threads(plan.compute as usize)` — bounded blocking pool from ThreadPlan
  - `.thread_name("semio-os-services")`
- **ScopeTable (lines 87-230):** Manages scopes; no threads
- **Threads:** Creates exactly `plan.io_workers + plan.compute` OS threads inside tokio runtime
  - I/O workers: configured as `worker_threads`
  - Blocking pool: configured as `max_blocking_threads`
- **Owned by:** Semio
- **Thread Lifetime:** Process lifetime (runtime never stopped in normal flow)

### A.3. ShardExecutor::spawn() (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`)
- **Lines 102-155:** ShardExecutor::spawn() and thread spawning
- **Line 108-153:** `std::thread::Builder::new()`
  - `.name("semio-shard-executor")`
  - Runs `block_on(async { ... })` with inner loop:
    - `park.recv_deadline(PARK_TIMEOUT)` polling loop (line 146, PARK_TIMEOUT = 5ms)
    - `shard.pump_primed()` per iteration
- **Threads:** 1 OS thread per ShardExecutor instance
  - In native interactive: K threads (where K = shard_count, typically 1-8 based on core count)
  - Each thread runs a single `ShardLoop` permanently
- **Owned by:** Semio
- **Thread Lifetime:** Process lifetime (pinned shard ownership)
- **Parking:** Blocks on `ThreadTransport::recv_deadline()`, wakes on incoming frames or 5ms timeout

### A.4. Shard Outcome Forwarders (`🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs`)
- **Lines 102-122:** Shard outcome forwarder threads
- **Line 102-121:** `std::thread::Builder::new()`
  - `.name(format!("semio-os-host-kernel-shard-forward-{index}"))`
  - Runs `block_on(forward_side.recv_deadline(FORWARD_POLL))`
    - FORWARD_POLL = 250ms (line 60)
  - Forwards ShardFrame outcomes back through mpsc::channel
- **Threads:** 1 OS thread per shard (K threads total, same as ShardExecutor count)
- **Owned by:** Semio
- **Thread Lifetime:** Process lifetime
- **Parking:** Blocks on `recv_deadline(250ms)`, wakes on frame or 250ms timeout

---

## B. All Thread Creation Sites (Complete Census)

### B.1. Rust: std::thread::spawn (Ad-Hoc Threads)

| # | File | Lines | Thread Name | Count | Role | Lifetime | Owned By | Phase 1 |
|---|------|-------|------------|-------|------|----------|----------|---------|
| 1 | `🔨️modules/🎒️pack/🌐️http/🦀️component.rs` | 189 | (unnamed) | 1 | HTTP range fetch thread | Request lifetime | Semio | Re-host |
| 2 | `🔨️modules/🎒️pack/🌐️http/🦀️component.rs` | 306 | (unnamed) | 1 | HTTP range request | Request lifetime | Semio | Re-host |
| 3 | `🔨️modules/🎒️pack/⏳️async/🦀️component.rs` | 557 | (unnamed) | 1 | Test: cancel token simulation | Test lifetime | Semio | Delete |
| 4 | `🔨️modules/🎒️pack/⏳️async/🦀️component.rs` | 628 | (unnamed) | 1 | Test: cancel token simulation | Test lifetime | Semio | Delete |
| 5 | `🔨️modules/🛢️db/🎭️actor/🦀️component.rs` | 1013 | (unnamed) | 1 | Test: channel test | Test lifetime | Semio | Delete |
| 6 | `🔨️modules/🛢️db/🎭️actor/🦀️component.rs` | 1057 | (unnamed) | 1 | Test: channel test | Test lifetime | Semio | Delete |
| 7 | `🔨️modules/🛎️services/🦀️component.rs` | 2255 | (unnamed) | 1 | Test: HTTP server accept loop | Test lifetime | Semio | Delete |
| 8 | `🔨️modules/🛎️services/🦀️component.rs` | 2259 | (unnamed) | N (1 per connection) | Test: HTTP connection handler | Test lifetime | Semio | Delete |
| 9 | `🔨️modules/📺️renderer/.../Shell/🧊️component.rs` | 3297 | (unnamed) | 1 | Test: socket listener | Test lifetime | Semio | Delete |
| 10 | `🔨️modules/📺️renderer/.../Shell/🧊️component.rs` | 3355 | (unnamed) | 1 | Test: socket read | Test lifetime | Semio | Delete |
| 11 | `🧫️fixtures/.../host-turn/🦀️main.rs` | 159 | (unnamed) | 1 | Fixture: turn loop polling | Fixture lifetime | Semio | Delete |
| 12 | `🧫️fixtures/.../host/🦀️main.rs` | 239 | (unnamed) | 1 | Fixture: epoch ticker loop | Fixture lifetime | Semio | Re-host as lane |
| 13 | `🧫️fixtures/.../host/🦀️main.rs` | 377 | (unnamed) | 1 | Fixture: outcome pusher loop | Fixture lifetime | Semio | Re-host as lane |
| 14 | `🦑️repo/.../cli/.../📦️glue.rs` | 759 | (unnamed) | 1 | CLI: test harness | Test lifetime | Semio | Delete |

### B.2. Rust: std::thread::Builder (Named Threads)

| # | File | Lines | Thread Name | Count | Role | Lifetime | Owned By | Phase 1 |
|---|------|-------|------------|-------|------|----------|----------|---------|
| 15 | `🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs` | 108-153 | `"semio-shard-executor"` | K (1-8) | Shard executor loop | Process | Semio | Real restructure |
| 16 | `🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` | 197-205 | `"semio-epoch-ticker"` | 1 | Epoch increment polling (1ms) | Process | Semio | Delete or re-host |
| 17 | `🔨️modules/🛢️db/📄️artifact/🦀️component.rs` | 1184 | (named from caller) | N per artifact | DB artifact actor loop | Artifact lifetime | Semio | Re-host |
| 18 | `🔨️modules/🛢️db/🎭️actor/🦀️component.rs` | 730 | (named per actor) | N per actor | DB actor message loop | Actor lifetime | Semio | Real restructure |
| 19 | `🔨️modules/🛢️db/⚙️engine/🦀️component.rs` | 1009-1018 | `"db-engine-submit-bridge"` | 1 per submit | DB engine submit bridge | Submit lifetime | Semio | Re-host as lane |
| 20 | `🔨️modules/🏪️store/🔄️sync/🦀️component.rs` | 1505-1514 | `"sync-actor-{doc_id}"` | N per doc | Store sync actor loop | Actor lifetime | Semio | Real restructure |
| 21 | `💻️os/🖥️host/🎠️activation.rs` | 102-121 | `"semio-os-host-kernel-shard-forward-{idx}"` | K (1-8) | Shard outcome forwarder | Process | Semio | Real restructure |

### B.3. Rust: tokio::runtime::Builder

| # | File | Lines | Type | Workers | Threads | Lifetime | Owned By | Phase 1 |
|---|------|-------|------|---------|---------|----------|----------|---------|
| 22 | `🔨️modules/🛎️services/🦀️component.rs` | 271-276 | `new_multi_thread()` | `plan.io_workers` + `plan.compute` | `io_workers + compute` | Process | Semio | Keep as main pool |
| 23 | `🧫️fixtures/.../host-turn/🦀️main.rs` | 534 | `new_multi_thread()` | Default | ~(2*cores - 4) | Fixture | Semio | Delete |
| 24 | `🔨️modules/🏪️store/🔄️sync/🦀️component.rs` | 1504 | `new_current_thread()` | 1 (embedded) | 0 (current thread) | Actor lifetime | Semio | Delete or merge |

### B.4. TypeScript/JavaScript: Web Workers

| # | File | Lines | URL | Count | Role | Lifetime | Owned By | Phase 1 |
|---|------|-------|-----|-------|------|----------|----------|---------|
| 25 | `🔨️modules/🎭️actor/📦️typescript/🧵️shard-runtime.ts` | 52 | SHARD_WORKER_URL | N per shard | Shard worker (browser) | Shard lifetime | Semio | Platform boundary |
| 26 | `🧑️‍💻️dev/📦️typescript/🧪️bench-web-harness.ts` | 98 | url param | N per bench | Bench worker harness | Bench lifetime | Semio | Delete (test) |
| 27 | `📺️renderer/.../ShellHost/🟦️component.tsx` | 1356 | `backbone-worker.ts` | 1 | Backbone worker (WASM host) | App lifetime | Semio | Platform boundary |
| 28 | `✏️s/🌐️spatial-kernel/.../🟦️component.ts` | 2499 | index.ts | 1 per kernel | Spatial kernel compute | Kernel lifetime | Semio | Platform boundary |

---

## C. Thread Count Arithmetic (10-Core Machine, Interactive Native)

### ThreadPlan Calculation (cores=10):
```
shards = clamp(ceil(10/2), 2, 8) = 5
io_workers = clamp(ceil(10/4), 1, 4) = 3
compute = max(1, 10 - 5 - 3 - 1) = 1
kernel = 1
epoch_ticker = 1
Total planned: 5 + 3 + 1 + 1 + 1 = 11 threads (but roles don't create separately)
```

### Actual OS Threads Created:

| Component | Count | Notes |
|-----------|-------|-------|
| TokioHostRuntime (worker_threads) | 3 | io_workers from plan |
| TokioHostRuntime (max_blocking_threads) | 1 | compute from plan |
| ShardExecutor threads | 5 | K shards (typically 1, but can scale) |
| Shard outcome forwarders | 5 | 1 per shard, K = 5 |
| Epoch ticker | 1 | semio-epoch-ticker polling loop |
| DB engine submit bridges | M | Ad-hoc per submit (unbounded) |
| DB actor threads | N | Ad-hoc per actor (unbounded) |
| Store sync actor threads | P | Ad-hoc per document (unbounded) |
| HTTP range fetch threads | R | Ad-hoc per request (unbounded) |
| **TOTAL (deterministic)** | **15 + M + N + P + R** | Core pools + ad-hoc |

**For native interactive baseline (no concurrent DB/store/HTTP ops):**
- **Deterministic threads:** 15 (3 I/O + 1 compute + 5 shards + 5 forwarders + 1 epoch)
- **Problem:** Unbounded ad-hoc threads can grow this arbitrarily

---

## D. Over-Allocation & Wrapping Issues

### D.1. ThreadBudget::checkout Wrapping
- **Location:** `🧰️framework/🔨️modules/⏳️async/🦀️component.rs:355-359`
- **Issue:** `debug_assert!` only fires in debug builds
  - Release builds silently allow `wrapping_sub()` 
  - Symptom: Second site checking out more threads than available wraps counter silently
  - Example: `checkout(Kernel, 2)` when only 1 remains → returns u32::MAX (wrapped)
- **Who can over-allocate:**
  - DB actor spawning (line 730, db_actor.rs) — per-actor threads, no budget check
  - Store sync actors (line 1505, sync.rs) — per-document threads, no budget check
  - DB engine submit (line 1009, engine.rs) — per-submit threads, no budget check
  - HTTP fetch (line 189, http.rs) — per-request threads, no budget check

### D.2. Ad-Hoc Thread Spawning (No Budget)
- **db_actor::StdThreadSpawner (line 730)** — spawns one OS thread per actor instance
  - No core count limit, no ThreadBudget permission
  - Real system could spawn 100+ threads for 100 actors
- **store/sync spawn_actor (line 1505)** — one thread per active document
  - Again unbounded, no consultation with ThreadBudget
- **db_engine submit bridge (line 1009)** — one per submit call
  - Temporary but still unbounded peak
- **HTTP range fetch (line 189)** — one per concurrent range request
  - Unbounded by request volume

---

## E. Sites Requiring Real Restructuring (Not Just Call-Site Swaps)

### E.1. ShardExecutor (`executor.rs` lines 108-153)
- **Assumption:** Owns dedicated OS thread with no other work
- **Why restructure:** Loop runs `recv_deadline(5ms)` polling continuously
- **To become lane:** Needs cooperative handoff of "ready" state to main worker pool
- **Complexity:** HIGH — must change from dedicated thread to task spawned on shared pool

### E.2. Shard Outcome Forwarders (`activation.rs` lines 102-121)
- **Assumption:** Owns dedicated OS thread polling `recv_deadline(250ms)`
- **Why restructure:** Same polling loop issue as ShardExecutor, but with 250ms cadence
- **To become lane:** Route outcomes through shared notification mechanism
- **Complexity:** HIGH — must detach from direct thread ownership

### E.3. DB Actor Threads (db_actor.rs line 730)
- **Assumption:** Each Actor spawns its own OS thread, runs message loop there
- **Why restructure:** Unbounded per-actor thread creation; no core-count awareness
- **To become lane:** Route messages through shared executor; pin actors to shards like plugins
- **Complexity:** VERY HIGH — requires redesigning Actor trait's threading model

### E.4. Store Sync Actor Threads (`sync.rs` lines 1505-1514)
- **Assumption:** Each document's actor gets its own dedicated thread + embedded tokio runtime
- **Why restructure:** One-thread-per-doc is unbounded; embedded tokio runtime is redundant
- **To become lane:** Route through main TokioHostRuntime, share I/O reactor
- **Complexity:** HIGH — needs actor model change + runtime unification

### E.5. Epoch Ticker (plugin_host/host.rs lines 197-205)
- **Assumption:** Dedicated thread with 1ms sleep loop
- **Why restructure:** Continuous polling consumes a full core on idle systems
- **To become lane:** Replace with event-driven epoch increment (e.g., wasmtime callback)
- **Complexity:** MEDIUM — depends on wasmtime API changes

---

## F. Sites That Can Be Re-Hosted Without Restructuring

### F.1. DB Engine Submit Bridge (`engine.rs` lines 1009-1018)
- **Current:** Spawns one OS thread per `submit()` call via `block_on()`
- **Can become:** Run via ComputePool or equivalent bounded lane
- **Effort:** Call-site swap to `runtime.run_blocking()` on main async executor
- **Complexity:** LOW — already uses block_on pattern

### F.2. HTTP Range Fetch Threads (`pack/http.rs` lines 189, 306)
- **Current:** Spawn OS thread per range request
- **Can become:** ComputePool workitem (already has bounded semaphore)
- **Effort:** Move to existing ComputePool pathway
- **Complexity:** LOW — already exists in codebase

### F.3. Test Threads (Various)
- **Action:** Simply delete; they're test-only
- **Effort:** Minimal

---

## G. Platform-Owned Threads (Not Semio's To Delete)

### G.1. Tokio Worker Threads (Implicit)
- **Owned by:** tokio runtime
- **Visible in:** TokioHostRuntime creation (line 271)
- **Count:** Already bounded by ThreadPlan
- **Semio role:** Record in inventory, but tokio manages lifecycle

### G.2. Web Workers (TypeScript Side)
- **Owned by:** Browser/Node.js runtime
- **Visible in:** ShellHost (line 1356), shard-runtime.ts (line 52)
- **Count:** Bounded by lane model (fixed K shards = K workers)
- **Semio role:** Record in inventory, call-site audit only

### G.3. wasmtime Engine Threads
- **Owned by:** wasmtime runtime
- **Configuration:** Per-engine epoch-interruption threads (hidden from Semio code)
- **Semio role:** Not explicitly created; just record in audit trail

---

## H. Summary Table: Thread Creation Sites

| Site | File | Count | Type | Lifetime | Restructure? |
|------|------|-------|------|----------|----------------|
| **Deterministic (Process)** | | | | | |
| Tokio I/O workers | services.rs:271 | `plan.io_workers` (3) | tokio pool | Process | Keep/Audit |
| Tokio blocking threads | services.rs:271 | `plan.compute` (1) | tokio pool | Process | Keep/Audit |
| Shard executors | executor.rs:108 | K (5) | dedicated | Process | YES - Real restructure |
| Shard forwarders | activation.rs:102 | K (5) | dedicated | Process | YES - Real restructure |
| Epoch ticker | host.rs:197 | 1 | dedicated | Process | YES - Real restructure |
| **Ad-Hoc (Unbounded)** | | | | | |
| DB actor threads | db_actor.rs:730 | N per actor | per-actor | Actor | YES - Very high restructure |
| Store sync actors | sync.rs:1505 | M per doc | per-doc | Doc | YES - High restructure |
| DB engine submit | engine.rs:1009 | P per call | per-submit | Submit | NO - Re-host via lane |
| HTTP range fetch | http.rs:189 | R per req | per-request | Request | NO - Re-host via lane |
| Test threads | various | N/A | test-only | Test | DELETE |

---

## I. Answers to Diagnostic Questions

### Q1: On a 10-core machine, how many OS threads does the current architecture create in total for a native interactive session?

**Answer:** 
- **Deterministic baseline:** 15 threads
  - 3 I/O workers (tokio)
  - 1 compute thread (tokio blocking pool)
  - 5 shard executors
  - 5 shard forwarders
  - 1 epoch ticker
- **With active operations:** Unbounded
  - Each DB actor → +1 thread
  - Each active document (store/sync) → +1 thread (+1 embedded tokio runtime)
  - Each submit operation → +1 temporary thread
  - Each HTTP range request → +1 temporary thread
- **Worst case:** 15 + (hundreds if many actors/docs are active)

### Q2: Where exactly can the current thread accounting over-allocate or wrap, and what would the symptom be?

**Answer:**
1. **ThreadBudget wrapping (debug_assert only):**
   - Location: `async/component.rs:355-359`
   - Trigger: Two sites checking out same role's budget
   - Symptom: Counter wraps to large value silently in release builds
   - Example: `checkout(Kernel, 2)` when 1 remains → returns u32::MAX

2. **Ad-hoc threads bypass budget entirely:**
   - No ThreadBudget.checkout() calls in db_actor (line 730), store/sync (1505), engine (1009), or http (189)
   - Symptom: Process spawns unlimited threads; core count awareness disappears
   - Observable: `ps` shows 100+ threads in heavy DB/store workload

3. **Embedded tokio runtimes:**
   - store/sync creates `tokio::runtime::Builder::new_current_thread()` per document (line 1504)
   - Symptom: N runtimes, each with its own reactor, blocked on current thread
   - Observable: Thread per doc busy-waits on block_on()

### Q3: Which sites assume they own a dedicated thread and will need real restructuring (not just a call-site swap) to become cooperative lanes?

**Answer:**
1. **ShardExecutor** (executor.rs:108-153)
   - Owns entire thread, runs `recv_deadline(5ms)` polling loop
   - Must yield control back to pool between iterations
   - Restructuring: Convert to task that yields; coordinate frame arrival with lane wake

2. **Shard Outcome Forwarders** (activation.rs:102-121)
   - Similar polling loop with 250ms timeout
   - Must integrate outcome flow with main dispatcher
   - Restructuring: Route via shared notification channel; integrate into Kernel::tick loop

3. **DB Actor Threads** (db_actor.rs:730)
   - Each actor spawns dedicated thread with hand-rolled message pump
   - No current-thread runtime; manual channel handling
   - Restructuring: Must add async trait impl; integrate with actor crate redesign

4. **Store Sync Actor** (sync.rs:1505-1514)
   - Embeds its own `tokio::runtime` per document
   - Blocks thread on `block_on()` waiting for that runtime
   - Restructuring: Eliminate embedded runtime; use main TokioHostRuntime; make message handlers async

5. **Epoch Ticker** (host.rs:197-205)
   - Continuous 1ms polling loop
   - Restructuring: Replace with epoch callback hook from wasmtime/guest-runtime

---

## J. Total Thread Creation Site Count

**Grand Total: 28 distinct thread creation sites**

- **std::thread::spawn:** 14 sites (6 named, 8 test/fixture)
- **std::thread::Builder:** 7 sites  
- **tokio::runtime::Builder:** 3 sites
- **Web Worker (`new Worker`):** 4 sites (TypeScript)

---

## K. Artifacts & References

- **Threading audit files:**
  - ThreadPlan enum: `🧰️framework/🔨️modules/⏳️async/🦀️component.rs:275-281`
  - ThreadBudget impl: `🧰️framework/🔨️modules/⏳️async/🦀️component.rs:321-365`
  - TokioHostRuntime: `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs:238-326`

- **Design reference:**
  - `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md`

---

## L. Recommendations for Phase 1

1. **Keep unchanged:** TokioHostRuntime (lines 271-276) — this IS the single worker pool
2. **Mark for deletion:** All test/fixture threads (sites 3-14, 23)
3. **Mark for real restructuring:** ShardExecutor (15), Shard forwarders (21), DB actors (18), Store sync (20), Epoch ticker (16)
4. **Mark for re-hosting:** Submit bridge (19), HTTP range (1-2), DB artifact (17)
5. **Audit for ThreadBudget integration:** Ensure all Phase 2 restructures actually call `ThreadBudget::checkout()`

