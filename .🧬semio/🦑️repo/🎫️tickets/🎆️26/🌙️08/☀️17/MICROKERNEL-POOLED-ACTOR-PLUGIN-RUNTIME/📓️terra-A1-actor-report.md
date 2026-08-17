# 📓️ terra — packet A1-actor report

Note on process: the coordinator (sol) instructed me mid-task to stop waiting on further
background build/test jobs (sol is running the acceptance commands independently against the
same `CARGO_TARGET_DIR`) and to write this report now from confirmed results. The two `cargo
check` results below were run and observed directly by me, with pasted output, before that
instruction. The `cargo test` run was still in flight in my own background shell at that point;
I wrote the first version of this report without its result. Its completion notification then
arrived on its own (I did not resume polling for it) with a clean pass, so it is now included
below as a real, confirmed result too — not something I waited on further.

## Files created (SHA-256)

```
a66512424fb21f13d62c9a370a1eabb6c5968657080fbca29349022029d2d4e7  🧰️framework/🔨️modules/🎭️actor/🦀️component.rs
8a9ced851fc478f671220c933758900512a5038ea668f8be83825a43cbbb03f9  🧰️framework/🔨️modules/🎭️actor/🟦️component.ts
1c673a1ef37f7b13872a5855590d6344e028a56bf462064975a9ca2e51cc76f5  🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml
a4fc5e6dbc933adcb8c56de234d54fad70a06ee33f81c4ebfcbc9a5d476224d7  🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📦️glue.rs
bac2a29f68ff7f06d5270dbc5d1557a6debd11a96a5490e48260314a9bd81b01  🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts
92fef3b3101d893c34aff105249cf5153d2914a906cbbdc9c172afeed7398a64  🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json
```

No other files were touched. `🎭️actor/🤖️generated/🟦️actor.ts` was **not** generated (would
require a green `cargo test --features typegen`, which sol is now the one verifying — see below).

**Not created — intentional scope decision**: `🎭️actor/📦️packages/🟦️typescript/{package.json,
📋️project.json, 📜️script.ts, 🧵️shard-client.ts}`. `🧵️shard-client.ts` is explicitly packet H2's
(design §3: "`ShardClient` (`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`) ... packet H2").
Sibling pure-Rust framework modules with no hand-written TS logic of their own (`🗺️surface`,
`🔄️machine`) have no `📦️packages/🟦️typescript` directory either — TS consumers import the
wasm-pack `pkg/` output or the bare `🟦️component.ts` directly. I followed that precedent rather
than scaffold an empty npm package H2 would need to fill in anyway. `🟦️component.ts` at the
module root re-exports the (not-yet-generated) `🤖️generated/🟦️actor.ts` mirror.

## Region map of `🦀️component.rs`

| Region | Contents |
|---|---|
| `🧵️Pack` | Hand-rolled LEB128-varint/bytes/str/hash32/bool/opt/vec codec primitives, `PackError` |
| `📦️PackageId` | `PackageId(String)`, `PackageHash([u8;32])` + codecs |
| `🆔️ActorId` | Bit-packed `ActorId(u64)` (`plugin:u16\|kind:u2\|ordinal:u32\|generation:u14`), accessors, `next_generation` |
| `🎭️ActorKind` | `PluginApp`/`Extension`/`Job` enum + codec |
| `🛣️Lane` | `Interactive`/`UserVisible`/`Background`/`Maintenance`, priority rank, DRR weight |
| `⚖️Budget` (nested `⚖️LaneDefaults`) | `Budget` struct + codec; per-lane default budgets (4/16/50/200ms, 2M fuel for Interactive as specified) |
| `✉️Envelope` (nested `🔑️CoalesceKey`) | `WindowId`, `Origin`, `Payload`, `CoalesceKey`, `Envelope` + codecs |
| `🔁️TurnResult` | `TurnStatus`, `Usage`, `TurnResult` (`ui_patches`/`effects` as opaque `Vec<u8>`) + codecs |
| `📬️Mailbox` | `Backpressure` enum, `Mailbox` (per-lane `VecDeque`, coalescing, bounded-ring eviction) + codec |
| `🔐️CapabilityGrant` | Minimal local stand-in for `kernel::CapabilityGrant` (see seam note below) + codec |
| `🚑️FailurePolicy` | `FailureSignal`, `FailureStage`, `FailureState` (ladder + decay), `FailureEscalation` + codecs |
| `🗂️ActorRecord` | `ActorStatus`, `ActorRecord` + codecs |
| `🧩️ShardTable` | `ShardKind`, `ShardId`, sizing-policy helpers, `ShardTable` (pin/unpin/exclusive lease) + codec |
| `⏱️Scheduler` (nested `🔖️DeadlinePreemption`, `🔖️DrrRounds`) | `Decision`, `TurnGrant`, `Scheduler` — two-level DRR (plugin, then actor), deadline short-circuit |
| `🖼️Scene` | `SceneSnapshot`, `SceneStore` (builder, frame-boundary commit, ui-node/byte quota) + codec |
| `📈️Metrics` | `ActorMetrics` (64-sample ring, p95), `ShardMetrics`, `KernelMetrics` + codecs |
| `🚚️ShardTransport` (nested `🔖️ThreadTransport`) | `ShardTransport` trait; `ThreadTransport` (native-only, `std::sync::mpsc` duplex pair) |
| `🏛️Kernel` | `ActivationEvent`, `KernelError`, `ActorMeta`, `Kernel` façade (`activate`/`submit`/`tick`/`complete`/`suspend`/`resume`/`request_exclusive`/`release_exclusive`/`apply_scene_patch`/`commit_frame`/`metrics`/`actor_record`) |
| `mod tests::quick` | All behavioral + pack round-trip tests (see below) |
| `🔖️Typegen` (inside tests) | `#[cfg(feature = "typegen")] exports_typescript_bindings` test |

## What was implemented vs. stubbed — honest

- **`Scheduler` DRR**: fully implemented, not stubbed. Two levels: plugin-level deficit
  round-robin (`plugin_deficit`/`plugin_order`/`plugin_cursor`), actor-level deficit round-robin
  within the selected plugin (`entry.deficit` per actor). Lane-weighted quanta (Interactive 8 /
  UserVisible 4 / Background 2 / Maintenance 1), throttle-scaled. Deadline preemption is a
  separate pass ahead of the DRR loop.
- **`Mailbox` coalescing/backpressure**: fully implemented. Coalescing replaces in place (keeps
  queue position); backpressure evicts the lowest-priority nonempty lane to admit a
  higher-priority envelope (`Dropped(Lane)`), or rejects the incoming envelope when nothing lower
  exists to evict (`Rejected`) — never a silent drop.
- **`FailurePolicy` ladder + decay**: fully implemented. Per-lane exponential warn thresholds
  (`lane_escalation_thresholds`), `Trap`/`HeartbeatMissed{count>=3}` go straight to `Trapped` and
  escalate to package-wide `Quarantined` after `FAILURE_QUARANTINE_RESTART_THRESHOLD` (3) restarts;
  `on_clean_turn` decays `Suspended`→`Throttled`→`Warned`→`Healthy` (and `Quarantined`→`Warned`
  once its `until` timer expires) over `FAILURE_DECAY_CLEAN_TURNS` (10) consecutive clean turns.
  `ManualReset` resets immediately. **Numeric thresholds (warn-count cutoffs, backoff/quarantine
  durations, decay window) are my own reasonable choices** — the design spec names the *shape* of
  the ladder ("exponential thresholds per lane... decays after N clean turns") but not exact
  numbers, so there is nothing upstream to conform to; these are documented in code and easily
  retuned.
- **`SceneStore` commit + ui-node quota**: fully implemented, with one real simplification: since
  `UiPatch` is opaque bytes to this crate (per the packet's explicit instruction not to define
  it), `SceneStore` cannot inspect a patch to truncate it structurally. On quota breach it caps its
  own tracked `node_count` at the budget ceiling (never exceeds it) and returns
  `FailureSignal::UiQuota` so the caller (kernel, which owns the typed `UiPatch`) can do the actual
  structural truncation. This is a real architectural consequence of the opacity seam, not
  laziness — documented in the `apply_patch` docstring.
- **`ShardTable`**: fully implemented (pin/unpin round-robin over the non-exclusive pool,
  ≤2-shard exclusive lease/release). The *native* `available_parallelism()-1` and *web*
  `hardwareConcurrency-1` queries themselves are host-side I/O and cannot live in this pure crate;
  I provide the pure clamp policy (`clamp_native_shard_count`, `clamp_web_shard_count`) for the
  host to call after it does the actual OS/JS query — this is the intended split per "the actual
  OS query happens in the (non-pure) host binary", not a gap.
- **`ShardTransport`/`ThreadTransport`**: `ShardTransport` trait fully specified per design.
  `ThreadTransport` fully implemented as a symmetric duplex pair over `std::sync::mpsc`, gated
  `#[cfg(not(target_arch = "wasm32"))]` as required. `WorkerTransport`/`ProcessTransport` are
  explicitly **not** mine (design: host-supplied / later waves) — not implemented, correctly out
  of scope.
- **`Kernel` façade**: all nine required methods present (`submit`, `tick`, `complete`,
  `activate`, `suspend`, `resume`, `request_exclusive`, `commit_frame`, `metrics`), plus
  `release_exclusive`, `apply_scene_patch`, `actor_record`/`actor_status`/`actor_failure` as
  convenience accessors (not required, additive). **Simplification**: `Kernel::actor_record`
  reconstructs a fresh empty `Mailbox` rather than exposing the scheduler's live one (the
  scheduler owns the real per-actor mailbox internally, by design, so it can drive DRR without a
  second copy of mailbox state) — documented in its docstring. `ActivationEvent` is only
  loosely threaded through today (`activate` accepts it but doesn't yet branch on `WindowOpen` vs
  `Restart` differently) since the concrete activation-source semantics belong to the
  `ActivationRegistry` in the H2/kernel packets.
- **`CapabilityGrant`**: **not** the real `kernel::CapabilityGrant` — a minimal local stand-in
  (`{ capability: String, scope: Option<Vec<u8>> }`). `ActorRecord.capabilities` needed *some*
  concrete type and this framework-tier crate cannot depend on the os-product-tier
  `manifest`/`kernel` crate without inverting the layering the repo is actively cleaning up. Flagged
  in the module-level seam docstring for `B1-host-native` to reconcile at integration.
- **Metrics**: fully implemented (`ActorMetrics` 64-sample ring + p95, `ShardMetrics`,
  `KernelMetrics`). The 2Hz `os.runtime.metrics` bus publication itself is host orchestration
  (timers = I/O), correctly out of this pure crate's scope — `Kernel::metrics()` just supplies the
  sample.
- **`Effect`/`Event`/`UiPatch`**: correctly **not defined** — carried as pack-encoded `Vec<u8>` in
  `TurnResult.ui_patches`/`.effects` and `Payload::Event`, per explicit instruction.

## Tests (`mod tests::quick`, all in `🦀️component.rs`)

Pack round-trips (encode→decode→re-encode byte/value equality) for every listed type: `PackageId`,
`PackageHash`, `ActorId`, `ActorKind`, `Lane`, `Budget`, `WindowId`, `Origin`, `Payload`,
`CoalesceKey`, `Envelope`, `TurnStatus`, `Usage`, `TurnResult`, `Backpressure`,
`CapabilityGrant`, `FailureSignal`, `FailureStage`, `FailureState`, `ActorStatus`, `ShardId`,
`ShardKind`, `Decision`, `TurnGrant`, `SceneSnapshot`, `ShardMetrics`, `KernelMetrics`, plus
dedicated round-trip tests for `Mailbox`, `ActorRecord`, `ShardTable`, `ActorMetrics` (ring
wraparound at 70 samples > capacity 64).

Behavioral:
- `actor_id_bit_packing_round_trips_all_fields` / `actor_id_next_generation_bumps_only_generation`
  — bit-layout correctness, generation-only bump on restart.
- `mailbox_coalesces_latest_wins_older_dropped` — 200 coalesced pointer-moves collapse to 1 queued
  envelope, latest payload wins.
- `mailbox_backpressure_rejected_when_full_and_nothing_lower_priority` — full mailbox, nothing
  lower-priority to evict → `Rejected`.
- `mailbox_backpressure_drops_lower_priority_lane_to_admit_interactive` — full mailbox admits an
  Interactive envelope by evicting the queued Maintenance one → `Dropped(Maintenance)`.
- `mailbox_pop_next_honors_lane_priority_over_fifo` — pop order is Interactive > Background >
  Maintenance regardless of insertion order.
- `drr_fairness_plugin_with_50_actors_does_not_starve_plugin_with_1` — both plugins preloaded
  with abundant backlog (so neither runs dry mid-test, which would confound the measurement);
  asserts the quiet actor is never starved and the busy:quiet **plugin-level** grant ratio stays
  under 10x over 100 ticks (vs. the ~50x a naive single-level scheduler would produce).
- `deadline_preemption_runs_before_background_drr_deficit` — an overdue Interactive-lane deadline
  preempts a pending Background-lane envelope even with a 1-grant tick budget.
- `failure_ladder_escalates_and_decays_back_to_healthy` — Healthy→Warned→Throttled→Suspended
  across three signals on the Interactive lane, then 200 clean turns decay all the way back to
  Healthy.
- `failure_ladder_trap_then_quarantine_is_package_wide` — 3 consecutive `Faulted` turns on one
  actor quarantine **both** actors sharing its package.
- `failure_ladder_manual_reset_returns_to_healthy_immediately` — `ManualReset` clears state
  regardless of prior stage.
- `scene_revision_is_monotonic_and_reuses_snapshot_on_empty_commit` — revision 0 initially,
  increments only on a non-empty commit, reuses the identical `Arc` (pointer-equal) when nothing
  was pending.
- `scene_ui_node_quota_truncates_and_signals` — a 150-node patch against a 100-node budget is
  capped at exactly 100 and returns `UiQuota`.
- `scene_max_patch_bytes_rejects_oversized_patch` — an oversized patch is rejected with `UiQuota`
  before any node accounting.
- `thread_transport_duplex_send_recv_and_heartbeat` — bidirectional send/recv across the two ends
  of `ThreadTransport::new_pair`, shared heartbeat timestamp.
- `thread_transport_kill_stops_recv` — a killed transport yields `None` even for an
  already-queued message (no stale delivery).
- `kernel_activate_submit_tick_complete_round_trip` — full `activate`→`submit`→`tick`→`complete`
  cycle through the façade.
- `kernel_suspend_resume_round_trip` — `Suspended{checkpoint}` → `Active` transition.
- `kernel_request_exclusive_then_release` — leased shard comes from the reserved tail of the pool.
- `kernel_metrics_counts_actors_shards_packages` — `KernelMetrics` counts match.
- `shard_sizing_policy_clamps_native_and_web` — `clamp_native_shard_count`/`clamp_web_shard_count`
  boundary values.

`#[cfg(feature = "typegen")] exports_typescript_bindings` — not a `mod quick` test; calls
`TS::export()` for every mirrored type (framework's existing typegen-test convention).

## Purity confirmation

`grep -n "wasm_bindgen\|web_sys\|winit\|tokio\|std::thread\|SystemTime::now\|Instant::now"` against
`🦀️component.rs` matches **only** the module-level docstring line naming what must stay absent —
zero actual usages. `ThreadTransport` (the sole `std::sync::mpsc` user) is gated
`#[cfg(not(target_arch = "wasm32"))]`. The `wasm_bindgen`/`web_sys`-touching `KernelHost` wrapper
lives entirely in `📦️glue.rs`, gated `#[cfg(target_arch = "wasm32")]`, and passes only
pack-encoded byte buffers across every method (`activate`/`submit`/`tick`/`complete`/`metrics`).
No clock, no I/O anywhere in `component.rs`; every time-dependent function takes `now_ms: u64`.

## Acceptance

Update: the `cargo test` run I had already started before the coordinator's stop instruction
finished on its own (I did not wait/poll for it after writing the first version of this report —
its completion notification simply arrived). Result appended below since it is now a real,
confirmed pasted output, not a claim.

Commands (all three run and observed by me directly, with real pasted output below):

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target"
```

### `cargo check -p semio-framework-actor --all-targets`

```
    Blocking waiting for file lock on build directory
    Finished `dev` profile [unoptimized] target(s) in 2m 10s
```
Exit code: 0. Zero warnings, zero errors.

(One real fix cycle happened first: the initial run failed with `E0433 cannot find type Arc` —
`std::sync::Arc` used in a test but not imported into that inner `mod quick` scope even via
`use crate::*;`. Fixed by adding `use std::sync::Arc;` inside `mod quick`, then this clean run.)

### `cargo check -p semio-framework-actor --target wasm32-unknown-unknown`

```
    Blocking waiting for file lock on build directory
    Checking unicode-ident v1.0.24
    Checking cfg-if v1.0.4
   Compiling wasm-bindgen-shared v0.2.126
   Compiling serde_core v1.0.228
   Compiling serde v1.0.228
   Compiling thiserror v2.0.18
    Checking once_cell v1.21.4
   Compiling syn v2.0.117
   Compiling wasm-bindgen v0.2.126
   Compiling wasm-bindgen-macro-support v0.2.126
   Compiling thiserror-impl v2.0.18
   Compiling serde_derive v1.0.228
   Compiling wasm-bindgen-macro v0.2.126
    Checking semio-framework-actor v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 7m 25s
```
Exit code: 0. Zero warnings, zero errors. Confirms the `KernelHost`/`wasm_bindgen` glue and the
pure core both build on the real wasm32 target.

### `cargo test -p semio-framework-actor`

```
test result: ok. 52 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests semio_framework_actor

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Exit code: 0. All 52 tests pass, including `drr_fairness_plugin_with_50_actors_does_not_starve_plugin_with_1`
and `failure_ladder_escalates_and_decays_back_to_healthy` — the two I'd flagged as highest-risk
hand-reasoned logic. Full per-test list matches the "Tests" section above 1:1 (52 = 26 explicit
`round_trip!` macro invocations + 26 hand-written `#[test]` fns).

## `lease-request` blocks

None. `Cargo.toml` for this crate is mine to author (done); the root workspace member line is
sol's per the packet brief, added automatically once the file exists on disk.

## What I could not do / did not do, and why

- `🎭️actor/🤖️generated/🟦️actor.ts` not generated — requires a green `cargo test --features
  typegen exports_typescript_bindings` run, which I did not get to before being told to stop.
- `🎭️actor/📦️packages/🟦️typescript/*` (package.json, project.json, script.ts) not created —
  scope decision explained above (H2's `🧵️shard-client.ts` is the only concrete consumer named
  for that directory in the design doc; no TS package exists yet to configure it into).
