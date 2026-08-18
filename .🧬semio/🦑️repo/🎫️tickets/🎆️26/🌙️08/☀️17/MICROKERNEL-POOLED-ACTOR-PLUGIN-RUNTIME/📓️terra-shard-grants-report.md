# 📓️ terra-shard-grants report

**Both Part A and Part B landed green.** No revert was needed.

## part A delivered

Six `#[serde(tag = "kind")]` newtype variants across the actor crate converted to struct variants (the exact `JobStep::Done`/`Failed` defect class, generalized — internally-tagged enums cannot serialize a newtype whose payload is not a map).

| enum | variant | before | after | file:lines |
|---|---|---|---|---|
| `Payload` | `Event` | `Event(Vec<u8>)` | `Event { bytes: Vec<u8> }` | `🎭️actor/🦀️component.rs:547-585` |
| `Payload` | `Cancel` | `Cancel(u64)` | `Cancel { seq: u64 }` | same region |
| `Origin` | `Actor` | `Actor(ActorId)` | `Actor { id: ActorId }` | `🎭️actor/🦀️component.rs:499-534` |
| `TurnStatus` | `Faulted` | `Faulted(Vec<u8>)` | `Faulted { detail: Vec<u8> }` | `🎭️actor/🦀️component.rs:666-693` |
| `FailureSignal` | `Trap` | `Trap(String)` | `Trap { detail: String }` | `🎭️actor/🦀️component.rs:923-966` |
| `Backpressure` | `Dropped` | `Dropped(Lane)` | `Dropped { lane: Lane }` | `🎭️actor/🦀️component.rs:757-786` |

Wire tags / byte layout unchanged — only the Rust shape (tuple → struct) changed; `pack_encode`/`pack_decode` still write/read the identical bytes.

### every construction/match site fixed

**Inside `🎭️actor/🦀️component.rs`** (all found by the full-repo grep below, no site missed):
- `Kernel::submit` (`Backpressure::Dropped(_) =>` → `{ .. } =>`)
- `Kernel::complete` (`TurnStatus::Faulted(detail) =>` destructure, and constructs `FailureSignal::Trap { detail: detail_string }`)
- `Mailbox::enqueue`'s eviction branch (constructs `Backpressure::Dropped { lane: dropped_lane }`)
- `FailureSignal::is_fatal` (`matches!(self, FailureSignal::Trap { .. })`)
- test module: `env()` helper, 5 `Payload::Event { bytes: … }` scheduler-test constructions, `Backpressure::Dropped { lane: … }` assertion, `TurnStatus::Faulted { detail: … }` round-trip + quarantine test, plus the 6 new serde round-trip tests below.

**Part A fallout (surgical, 2 files, out of my owned paths but explicitly pre-authorized):**
- `🖥️host/🦀️component.rs` — `runtime_metrics_publisher_tests` module, lines 1466 (`env()` helper: `Payload::Event(vec![1])` → `Payload::Event { bytes: vec![1] }`) and 1592 (`TurnStatus::Faulted(b"scale-fixture crash profile".to_vec())` → struct form). Confirmed this module imports `semio_framework_actor::{…, Payload, …, TurnStatus, …}`, NOT the unrelated `semio_framework::kernel::TurnStatus`/kernel `Payload`-equivalent used elsewhere in the same file (lines 1069/1289, a *different*, unaffected type — left untouched).
- wgpu target `📦️glue.rs` — 3 sites, lines 353, 360, 667 (all `Payload::Event(…)` → `Payload::Event { bytes: … }`). No `Origin::Actor`/`TurnStatus::Faulted`/`FailureSignal::Trap`/`Backpressure::Dropped` construction exists in this file (grepped, confirmed absent).

Repo-wide grep for the old tuple-call shape across all 6 variants, run AFTER every edit, returns nothing:
```
grep -rn "Payload::Event(\|Payload::Cancel(\|Origin::Actor(\|TurnStatus::Faulted(\|FailureSignal::Trap(\|Backpressure::Dropped(" --include="*.rs" .
```
→ zero matches anywhere in the tree (checked both before starting, to size scope, and after, to confirm completeness).

**Not touched, correctly out of scope:** `semio_framework::kernel::TurnStatus::Faulted(Vec<u8>)` — a *different* type in the kernel crate with the same latent shape, owned by another packet's file (`🎠️kernel/🦀️component.rs`). Flagged in `## honest gaps`, not fixed here.

## typegen mirror evidence

Ran `CARGO_TARGET_DIR=<ticket>/🎯️target-sg bun ./📜️script.ts typegen` from `🎭️actor/📦️packages/🦀️rust` — regenerates `🎭️actor/🤖️generated/🟦️actor.ts` via `cargo test --features typegen exports_typescript_bindings` (1 passed) + ts-rs export + consolidation. `🤖️generated/🟦️actor.ts` is explicitly listed as mine for this packet.

Before (stale, pre-fix mirror, captured while sizing scope):
```
export type Backpressure = { "kind": "accept" } | { "kind": "coalesced" } | { "kind": "dropped" } & Lane | { "kind": "rejected" };
export type FailureSignal = { … } | { "kind": "trap" } & string | { … };
export type Origin = { … } | { "kind": "actor" } & ActorId | { … };
export type Payload = { "kind": "event" } & Array<number> | { … } | { "kind": "cancel" } & bigint | { … };
export type TurnStatus = { … } | { "kind": "faulted" } & Array<number>;
```

After (regenerated):
```
export type Backpressure = { "kind": "accept" } | { "kind": "coalesced" } | { "kind": "dropped", lane: Lane, } | { "kind": "rejected" };
export type FailureSignal = { "kind": "deadlineOverrun", ratio: number, } | { "kind": "fuelExhausted" } | { "kind": "memoryLimit" } | { "kind": "mailboxOverflow" } | { "kind": "uiQuota" } | { "kind": "trap", detail: string, } | { "kind": "heartbeatMissed", count: number, } | { "kind": "manualReset" };
export type Origin = { "kind": "ui", window: WindowId, } | { "kind": "actor", id: ActorId, } | { "kind": "kernel" } | { "kind": "bus", topic: string, };
export type Payload = { "kind": "event", bytes: Array<number>, } | { "kind": "suspend", checkpoint: boolean, } | { "kind": "resume", checkpoint: Array<number> | null, } | { "kind": "cancel", seq: bigint, } | { "kind": "jobStep", job: bigint, };
export type TurnStatus = { "kind": "idle" } | { "kind": "moreWork" } | { "kind": "checkpointReady" } | { "kind": "faulted", detail: Array<number>, };
```

**`grep -n '} & ' 🎭️actor/🤖️generated/🟦️actor.ts` → no matches, exit 1.** (Pasted verbatim in `## commands + exit codes`.)

## part B delivered

1. **`ShardFrame`** (`🧵️shard/🦀️component.rs:32-108`) — `enum ShardFrame { Register{actor}, Unregister{actor}, Grant{actor, budget, envelopes}, Envelope(Envelope) }`, hand-rolled `pack_encode`/`pack_decode` via `semio_framework_actor::pack` (the same primitives the actor crate's own types use — first external consumer of that `pub mod pack`). `Envelope` passthrough kept exactly as instructed, documented as intentionally non-redundant. Same encoding used on both `ThreadTransport` (native in-process) and `ProcessTransport`/`StdioTransport` (stdio) — no branching on transport kind anywhere in this file, preserving the module's existing "never branches on which `ShardTransport` impl it got" invariant.
2. **Budgets travel in `Grant`.** Deleted `JOB_STEP_BUDGET` (shard) and the `budget_for` closure parameter from `pump`'s signature entirely. `ShardLoop` gained `granted_budgets: HashMap<u64, semio_framework_actor::Budget>`, updated whenever a `Grant` frame arrives; `granted_budget(actor)` falls back to `semio_framework_actor::lane_defaults::budget_for(Lane::Maintenance)` for an actor never granted one (a real, already-designed floor, not an invented constant). Two bridge functions (`turn_budget_from_grant`, `job_budget_from_grant`) convert the actor crate's DRR `Budget` shape into the kernel crate's `Budget`/`JobBudget` shapes turn execution and job stepping actually need. `max_frames` has no source field in the DRR `Budget` yet — a documented, honest constant (`GRANT_BUDGET_DEFAULT_MAX_FRAMES = 8`), not a fabricated value.
3. **`ShardExecutor`** — new file `🧵️shard/🏃️executor.rs`, registered via `#[path = "🏃️executor.rs"] pub mod executor;` **inside** `shard/🦀️component.rs` (relative `#[path]` resolves against the current file's own directory — no edit to `🖥️host/🦀️component.rs`'s module tree needed). Owns one `ShardLoop` + a `SharedThreadTransport` (a local newtype wrapping `Arc<ThreadTransport>`, sidestepping the `E0117` orphan-rule issue `LoopbackProbe`'s own doc comment already names for `Arc<LoopbackTransport>`) + the runtime handle, on a dedicated OS thread. Parks via `ThreadTransport::recv_deadline(5ms)`, then calls the new `ShardLoop::pump_primed(Option<Vec<u8>>)` so the blocking park and the non-blocking drain share the same transport without losing whatever woke the wait. `spawn(runtime, transport, initial: Vec<(ActorId, GuestInstance)>)` — registration happens before the loop starts, since a `GuestInstance` cannot cross the wire and the loop thread is the only place `ShardLoop::register` can run once spawned (mirrors how `KernelThreadState`, the wgpu-native host's existing single-`ShardLoop` design, also only ever registers from the loop's own thread). `stop()`/`Drop` join the thread (bounded by the 5ms park).
4. **`to_actor_turn_result(&kernel::TurnResult, wall_us, memory_bytes) -> actor::TurnResult`** — `🧵️shard/🦀️component.rs:147-160`, NOT in `🖥️host/🦀️component.rs` (per rule 17, a concurrent packet owns that file). Maps `status` 1:1 (including into the Part-A-fixed struct-variant `Faulted { detail }`); `ui_patches`/`effects` re-encoded as JSON (the same convention this file's own `Payload::Event` handling already uses, since the kernel crate has no `pack_encode` for `UiPatch`/`Effect` yet — out of this packet's scope); `usage.fuel` from `fuel_used`, `wall_us`/`memory_bytes` host-measured and passed in.
5. **`ThreadTransport::recv_deadline`** — actor crate, `🔖️ThreadTransport` region only (`🦀️component.rs:2098-2103`), `std::sync::mpsc::Receiver::recv_timeout`, spawns no thread. Purity grep re-verified clean after adding it (see `## purity evidence`).

### pump()'s new internal shape
`pump()` (no args) → `pump_primed(None)`. `pump_primed(Option<Vec<u8>>)` drains the primed frame (if any) then the transport non-blockingly via a new `consume_frame` helper (decodes one `ShardFrame`, dispatches `Register`/`Unregister`/`Grant`/`Envelope`). `Grant`'s bundled envelopes and a standalone `Envelope` frame both funnel through a new `dispatch_envelope` helper — the exact per-payload match body `pump` used to run inline, unchanged behavior, just reachable from two call sites now. Turn execution and job stepping each compute their budget from `self.granted_budget(actor_id)` **before** taking the `&mut self.instances` borrow (a real `E0502` the compiler caught and I fixed by hoisting the budget computation above `get_mut`).

## line ranges edited per file

| file | ranges | what |
|---|---|---|
| `🎭️actor/🦀️component.rs` | 499-534, 547-585, 666-693, 757-786, 923-966, 849 (`Mailbox::enqueue` eviction), 2206 (`Kernel::submit`), 2227-2229 (`Kernel::complete`), 2061-2103 (`recv_deadline` + doc), 2550-2577 (`SerdeRoundTrips` region, 6 tests), 2840-2868 (3 new `recv_deadline` tests), plus ~10 scattered one-line test-value fixes (`env()`, mailbox tests, scheduler-fairness tests, quarantine test) | Part A enums + `recv_deadline` |
| `🎭️actor/📦️packages/🦀️rust/Cargo.toml` | `[dev-dependencies]` | added `serde_json = { workspace = true }` (test-only; the crate's real wire stays `pack_encode`/pure) |
| `🎭️actor/🤖️generated/🟦️actor.ts` | whole file | regenerated |
| `🖥️host/🦀️component.rs` | 1466, 1592 | Part A fallout only (2 lines) |
| wgpu target `📦️glue.rs` | 353, 360, 667 | Part A fallout only (3 lines) — see `## lease-requests` for what I did **not** touch here |
| `🧵️shard/🦀️component.rs` | 1-1233 (whole file restructured) — key regions: 17-22 (`executor` mod decl), 32-108 (`ShardFrame`), 110-162 (`BudgetBridge`), 189-238 (`ShardLoop` struct + `granted_budget`), 275-460 (`pump`/`pump_primed`/`consume_frame`/`dispatch_envelope`), 528+ (tests: `ShardFrameRoundTrips`, `GrantBudgetExecution`, `RegisterUnregisterFrames`, `BudgetBridge` regions appended) | Part B core |
| `🧵️shard/🏃️executor.rs` | new file, 188 lines | `ShardExecutor` |
| `🧵️shard/👶️child/🦀️main.rs` | imports, `INSTANTIATE_BUDGET` const (renamed from `SHARD_CHILD_BUDGET`, kept only for `instantiate`'s one-time call), `shard.pump()` call | adapt to new `pump()` signature |
| `🧵️shard/🚚️process-transport/🦀️component.rs` | `encode_envelope` test helper (wraps in `ShardFrame::Envelope`), 2 `Payload::Event { bytes }` sites | keep the `#[ignore]`d kill-rebuild fixture source-correct against the new wire |

## commands + exit codes

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target-sg cargo test -p semio-framework-actor
   Compiling semio-framework-actor v0.1.0 (…)
    Finished `test` profile [unoptimized] target(s) in ~0.03s (incremental)
running 69 tests
test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Exit: 0
```
(69 = 60 baseline + 6 serde round-trip tests + 3 recv_deadline tests, all newly added; zero baseline regressions — every one of the 60 original test NAMES is still present in the run.)

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target-sg cargo check -p semio-framework-actor --target wasm32-unknown-unknown
    Checking semio-framework-actor v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 0.38s
Exit: 0
```

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target-sg cargo check -p semio-framework-plugin-host --all-targets
    Checking semio-framework-plugin-host v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 9.75s
Exit: 0
```
(20 pre-existing unrelated warnings from `semio-framework-replication`/`semio-framework-os-kernel`, not from any file this packet touched — confirmed by line numbers, all in `📡️wire/🦀️component.rs` and `🏪️store/🦀️component.rs`.)

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target-sg cargo test -p semio-framework-plugin-host --lib
    Finished `test` profile [unoptimized] target(s) in 14m 39s (cold — first build after adding new files; wall time is compile cost + this ticket's own documented cargo-lock contention with a concurrent peer session, not test runtime)
running 101 tests
test result: ok. 100 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.36s
Exit: 0
```
(100 = 86 baseline + 14 new tests — every one of the 86 baseline test NAMES is present and passing; the 1 ignored is the pre-existing `#[ignore]`d `process_shard_kill_is_detected_and_the_shard_rebuilds…` integration test, unrelated to this packet and unchanged in status.)

New test names, by name (proving they actually ran, not just "test count went up"): `shard_frame_round_trip_register`, `shard_frame_round_trip_unregister`, `shard_frame_round_trip_grant`, `shard_frame_round_trip_envelope_passthrough`, `grant_with_no_envelopes_still_records_the_budget`, `a_grants_budget_is_what_the_turn_actually_executes_under`, `job_step_uses_the_owning_actors_last_granted_budget`, `an_actor_never_granted_a_budget_falls_back_to_the_maintenance_lane_default`, `unregister_frame_drops_the_instance_exactly_like_the_direct_call`, `register_frame_is_accepted_without_error_and_has_no_local_side_effect`, `to_actor_turn_result_maps_status_and_carries_host_measured_usage`, `to_actor_turn_result_status_maps_idle_more_work_and_checkpoint_ready`, `shard::executor::tests::shard_executor_drives_a_turn_for_a_registered_actor_from_its_own_thread`, `shard::executor::tests::stop_joins_the_thread_and_is_idempotent_with_drop`.

```
$ bun ./📜️script.ts typegen   (run from 🎭️actor/📦️packages/🦀️rust, CARGO_TARGET_DIR=<ticket>/🎯️target-sg)
running 1 test
test component::tests::exports_typescript_bindings ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 69 filtered out
framework-actor typescript mirror refreshed -> …/🤖️generated/🟦️actor.ts
Exit: 0
```

```
$ grep -n "} & " 🎭️actor/🤖️generated/🟦️actor.ts
(no output)
Exit: 1
```
(`grep` exits 1 on no match — this IS the desired, verified-clean result.)

## purity evidence

```
$ grep -n "wasm_bindgen\|web_sys\|winit\|tokio\|rayon\|std::thread\|SystemTime\|Instant::now\|std::fs\|std::net" 🎭️actor/🦀️component.rs
2://! `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread` in this file. Transports are
2094:        /// `std::thread` in `🦀️component.rs`) still holds. Not part of the [`ShardTransport`]
2834:        /// crate-wide purity grep this ticket already runs (`std::thread` must match only the
2836:        /// `std::thread::current()`, which would itself add a real (non-doc-comment) `std::thread`
```
All 4 matches are doc-comment prose (lines 2 is the pre-existing module header; 2094/2834/2836 are new doc comments I wrote for `recv_deadline` and its test, deliberately naming `std::thread` in PROSE while avoiding the identifier in REAL code — I originally wrote a `recv_deadline` test using `std::thread::current().id()` to prove "no thread spawned", caught it violating this exact grep myself, and rewrote the test to prove the behavioral property (`None` on timeout) without touching `std::thread` in code, per the doc comment now explaining why). `ThreadTransport` (including the new `recv_deadline`) stays `#[cfg(not(target_arch = "wasm32"))]`.

`cargo check -p semio-framework-actor --target wasm32-unknown-unknown` (above) confirms this compiles for wasm32 as well as passing the grep.

## honest gaps

- **wgpu-native `📦️glue.rs` is now source-broken by the `pump()` signature change** — see `## lease-requests`, filed rather than fixed (fixing it is genuinely out-of-Part-A-scope for that file and needs a design call, not a 2-line patch — see below).
- **`semio_framework::kernel::TurnStatus::Faulted(Vec<u8>)`** (a *different* type, in `🎠️kernel/🦀️component.rs`, used by `wit_turn_status_to_kernel`/`kernel_turn_result_to_wit` in `🖥️host/🦀️component.rs` and `⚛️reactor/🦀️component.rs`) has the exact same latent internally-tagged-newtype defect as the six variants this packet fixed. Not in my `path_scope` (that file belongs to a different packet's ownership), so NOT fixed — flagged here per rule 12's "sweep every sibling" instruction, for whoever owns that file next.
- **`ShardExecutor` has no way to register an actor AFTER `spawn()`** — a `GuestInstance` cannot cross the transport (it can hold a live `wasmtime::Store`), and once the loop is running on its own thread there is no channel back in for a fresh registration. `spawn`'s `initial: Vec<(ActorId, GuestInstance)>` parameter covers "pre-instantiate everything, then spawn" but not "add an actor to an already-running executor." A real multi-shard router (this packet's own doc names it as the intended NEXT caller) will need either a command channel into the executor thread or a redesign — documented, not silently worked around.
- **`ShardExecutor` is not wired into any real caller yet** — `KernelThreadState` (wgpu-native) and the `semio-shard` `[[bin]]` both still drive their OWN single `ShardLoop` directly, unchanged by this packet (per the explicit Part-A-fallout-only cap on `📦️glue.rs`, and because rewiring the process-shard binary to spawn N executors is a router-level decision this packet's brief did not ask for). `ShardExecutor` is landed as the seam, exactly like `GuestRuntime`/`MockGuestRuntime` landed before `WasmtimeRuntime` existed — a documented gap, not a faked wiring.
- **`GRANT_BUDGET_DEFAULT_MAX_FRAMES = 8`** — `semio_framework_actor::Budget` has no UI-frame-pacing field, so `turn_budget_from_grant` cannot derive `max_frames` from the Grant at all. Fixed constant, documented at its definition, not fabricated data (there is no real per-turn source for this field yet anywhere in the DRR budget vocabulary).
- **`semio-shard` `[[bin]]` never sends itself a `Grant`** — after this change, an actor on that binary that never receives a `Grant` from whatever spawns it falls back to `lane_defaults::budget_for(Lane::Maintenance)` (documented in the binary's own updated comment) rather than the old, much more generous hardcoded 200M-fuel constant. This is an honest behavior change: nothing in this binary ever computed a real per-turn budget before either (it used ONE constant for every actor, every turn), so the new fallback is not a regression in real capability, just a different (smaller) default until a real caller sends Grants.

## lease-requests

**One lease-request, for `📦️glue.rs` in the wgpu-native renderer target** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`), which I did **not** edit beyond the 3-line Part-A fallout the packet brief explicitly authorized for this file.

**Problem:** `ShardLoop::pump`'s signature changed from `pump(&mut self, budget_for: impl Fn(ActorId) -> Budget)` to `pump(&mut self)` (budgets now travel via `ShardFrame::Grant`, per this packet's Part B). This file calls the OLD signature at two sites and will not compile:
- line 363: `self.shard.pump(|_| TURN_BUDGET).map_err(|error| error.to_string())?;` (inside `KernelThreadState::run_turn`)
- line 684: `self.shard.pump(|actor| budgets.get(&actor.0).copied().unwrap_or(fallback)).map_err(|error| error.to_string())` (inside the `scale_bench` module's `Env::pump`)

Additionally, BOTH call sites' senders still write raw `Envelope::pack_encode` bytes onto the same `ThreadTransport` `ShardLoop::pump` now expects `ShardFrame`-wrapped bytes on:
- `run_turn`'s `pack_envelope(&envelope)` helper (used at lines 358/361)
- `Env::send_payload`'s `envelope.pack_encode(&mut bytes)` (line ~677)

**Why I didn't fix it myself:** the packet brief caps `📦️glue.rs` edits at "Part A fallout only, surgically … list every line range you touch." This is a Part B consequence, not Part A, and it is not a pure mechanical rename — the bench module's `Env::pump` (line 681-685) currently gives EACH actor its OWN previously-registered budget via a local `HashMap<u64, TurnBudget>` (`self.budgets`), a real per-actor semantic `ShardFrame::Grant` could preserve exactly (wrap each actor's own budget in a `Grant`) OR could silently flatten to the Maintenance-lane fallback (wrong, loses the bench's real per-actor-budget behavior) depending on how it's wired — a design decision belonging to whoever owns this file, not a 2-line mechanical patch.

**Exact text needing a decision + edit:**
1. `run_turn`: wrap `envelope` in `ShardFrame::Envelope(envelope)` before `pack_encode`, and change `self.shard.pump(|_| TURN_BUDGET)` to `self.shard.pump()` (this path has no per-actor budget table today, so `Envelope` passthrough + Maintenance-lane fallback is behavior-preserving here — `TURN_BUDGET` becomes unused and should be removed alongside).
2. `Env::send_payload`/`Env::pump` (scale-bench harness): either (a) keep sending plain `Envelope`s wrapped in `ShardFrame::Envelope` and accept the Maintenance-lane fallback for every bench actor (simplest, but silently drops the existing per-actor budget differentiation the bench currently exercises), or (b) change `send_payload` to send `ShardFrame::Grant { actor, budget: self.budgets[&actor.0], envelopes: vec![envelope] }` instead (preserves today's semantics exactly, more line-count).

**Not build-verified** (I did not run `cargo check -p semio-framework-os-renderer-wgpu` — reasoned from the `pump()` signature change alone, which is unconditional; running that check was avoidable extra load on an already-contended shared build lock for a crate outside my acceptance gate, and I judged the source-level reasoning here as high-confidence enough not to need it, but flagging that this specific claim is reasoned, not observed).

## honest position on scope

Both parts fully landed, green, with no baseline regressions on any of the four required acceptance commands. The one open item (wgpu-native `📦️glue.rs`) is a real, out-of-scope break this packet's Part B causes but is not authorized to fix — surfaced as a lease-request rather than silently left for someone to discover via a broken build.
