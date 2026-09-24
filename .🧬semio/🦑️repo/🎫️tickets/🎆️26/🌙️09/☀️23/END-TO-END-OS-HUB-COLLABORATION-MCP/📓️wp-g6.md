# WP-G6: Binding Jobs, Relay Hop, Pool Parking (Session 10)

Slice G6 · session 10 · 2026-09-24. Ports: hubs 7890–7899, serves 6390–6399 (note serve :6390, stopped). Continues G5 §8 (gaps 1–3).
Status: DONE. All three gaps are fixed at the root, two more root causes turned up along the way (§4), and all gates are green (§6).

## 0. Headline

| | G5 | G6 | capture |
|---|---|---|---|
| artifact binding in `inference_run` | 6–29 s, no progress, cancel answered after 19.8 s | **0.05 s**, one progress row per phase; cancel answered **1 ms** live, ≤ 13.7 ms in the law under load 40 | `g6-after2-live.txt`, `g6-cancel-binding1-live.txt`, `g6-binding-law-final.txt` |
| `artifact_create` (first call in a process) | 7.8–19.8 s | ~2.2 s (the component is hashed once per build, not once per open) | `g6-after2-live.txt` |
| relay pool queue + main hop per solve | 8.1 s + 3.4 s (22.6 % of relay wall) | 0.08–0.2 s + 0.05–0.18 s (< 1 %) | `g6-anatomy1-live.txt` vs `g6-after2-live.txt` |
| idle pool CPU (idle gateway, 30 s) | 3.78 s | **0.00 s** | `g6-idle-cpu.txt` |
| pool workers during a solve | 9 workers × ~5.7 % of a core (1 ms epoch tick + notify_all) | 1 keeper thread (~8 %) and the rest asleep | `g6-threads1-ps.txt` vs `g6-threads2-ps.txt` |
| over-MCP solve vs headless native, **CPU of the solving thread** | — | 17.5–18.1 s vs 13.3–13.6 s = **1.29–1.33×** | `g6-t1..3-live.txt`, `g6-native-t1..3.txt` |
| same, whole-process CPU | — | 20.3–22.5 s vs 13.3–13.6 s = 1.50–1.66× (the rest is the 1 ms epoch keeper) | same |
| same, wall, back to back | 2.07× (load ~23) | 1.30–2.0× at load 27–45; wall is dominated by preemption at this load (native alone ranged 14.4–72.9 s) | same, `g6-native-last*.txt` |
| cancel mid-solve | 10 ms | 4–17 ms | `g6-cancel-solve1-live.txt`, `g6-probe-final.txt` |

Gates: client-e2e **38/38**, stdio transcript probe **17/17**, live-agent-loop **20/20** (note serve :6390), G5's crossing-grant law green, new binding-cancellation law 1/1 (§6).


## 1. Artifact Binding As A Job (gap 1)

### 1.1 Anatomy (measured, `g6-anatomy1-live.txt`, temporary `[DEBUG] g6` probes, since removed)

| phase | cost |
|---|---|
| registry + descriptor + reading the 125 MB component | 37–128 ms |
| **content hash of the component (own BLAKE3, debug build)** | **8 491 ms** (binding) / 11 513 ms (`artifact_create`, same process) |
| compiled-code lookup (`.cwasm` deserialize) | 309 ms (first open), 0 after that |
| instantiate + InstanceOpen (3 turns) | 35–65 ms |
| ReadDocument | 4–9 ms |

Root cause: every channel open read the whole component and hashed it before it could ask the compiled cache anything. `artifact_create` and the root adapter's session each opened a channel, so one session paid for the hash twice.

### 1.2 Fixes (`🌉️mcp/🏠️workspace/🦀️.rs`)

- **Component file identity → content hash.** Keyed by (path, length, mtime ns). The map is held in the process and persisted as one 32-byte file per identity under `~/.semio/cache/wasmtime/component-identities/`, beside the `.cwasm` store and in the same trust domain. A staged build is now read and hashed once per build, not once per channel open. `scoped_compiled_component` replaces the read+hash+compile triple for every caller (channel open, `activate_plugin_instance`, hub bytes, codec registration).
- **Binding is a job.** `crate::actions::BindingPhase` (schema: `💡️inference/🧬️schema` `InferenceBindingPhaseV1`) covers `resolving-component`, `reading-component`, `hashing-component` (reported per 4 MiB chunk), `loading-compiled-code`, `opening-guest` and `reading-document`. `ActivationScope` carries the caller's cancel token and a progress sink. Every interruptible boundary is a checkpoint, including hash chunks and each InstanceOpen turn (a half-open guest is dropped).
- `ArtifactChannel::activate` is new on all six implementors. Routing does everything `exchange` would open, in the same order (channel, guest, hub document load + backbone). `ActionAdapter::activate_session` and `HeadlessWorkspace::{bind_artifact_document, bind_artifact_document_cancellably, activate_plugin_session_cancellably}` expose it. The job runs on the process pool's Io lane. The caller waits on `select2(answer, cancel.cancelled())`, so a cancel is answered the moment it lands, whatever phase is in flight. Compiling is the one step a cancel does not cut short: it is shared and content-addressed, so it finishes into the cache.
- `inference_run` mints its cancel token and binds the job hook **before** binding; before this change the hook was bound after binding, which is why a cancel waited 19.8 s. A cancel during binding answers `CANCELLED`. `map_fault` maps `job.cancelled` → `CANCELLED`.

### 1.3 Binding-cancellation law

- Fixture `💡️inference/🧫️fixtures/⏱️binding-cancellation-law.json` (schema `InferenceBindingCancellationLawV1`) defines the ceiling 100 ms, the six phases with progress/interruptibility, and seven cases.
- Replay: `[[test]] binding_cancellation_law` (`💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs`). It runs in its own process against the real staged wfc component, with a private `SEMIO_HOME` whose `.cwasm` store links to the shared one. Every phase is genuinely reached. **1/1**: cancels answered in 81, 21, 60, 28, 28 and 109 µs. The uncancelled binding returns the genesis document byte-identical to a warm rebind (`g6-binding-law2.txt`). The test also asserts that the Rust phase ids, progress values and interruptibility equal the fixture's.
- Live stdio (`g6-cancel-binding1-live.txt`): `notifications/cancelled` sent on the `binding: resolving-component` row → **CANCELLED 1 ms later**, ping ok.

## 2. Relay Crossing Hop (gap 2)

### 2.1 The law

`♻️relay-lifecycle.json` / `RelayLifecycleV1` states slot capacity, generation fencing, detach/reap and exact caller output. The code's own rule is "each host poll admits at most one relay or close opportunity; the pool never owns an internal run-to-completion chain". Both are real invariants. Neither requires a crossing to run on a pool worker. G5's proposed fix (loop on the worker until Done) would have broken the second rule. **The law is not over-specified, so the schema is unchanged.**

### 2.2 Design kept inside the law (`🔌️plugin/🖥️host/🦀️.rs`)

- A relay request's **first poll runs on the thread that owns the relay**: the caller's own poll of the mounted future, or the reaper's after a detach. `GuestRelayCompletionSlot` now holds the request (`Owed → Polling → Suspended/Handed`). A crossing that completes inside that poll (every `step-job` that does not suspend) pays no pool queue and no hop back to the caller. One crossing per poll still holds.
- A request that suspends (instance gate held, guest yield, waiting on cancel) is **handed to the pool on its wake**, exactly as before. So a detached or dropped relay still finishes and releases its instance on its own. Panics still reach the same recovery.
- `wait_for_guest_relay_cancellation` polled the token every 1 ms on a pool timer. It is deleted, replaced by `CancelToken::cancelled()` (§3).
- Latent bug found by the inline path: in `close_step` a cleanup `Cancel` that came back `Rejected` (instance already quarantined) never cleared `cleanup_required`. The job's retirement stayed Blocked forever and starved every later parked retirement in the process-wide array. `step()` already cleared it; `close_step` now does the same.
- Measured: queue 8 092 ms → 82–203 ms, hop 3 374 ms → 49–177 ms per solve (`g6-anatomy1-live.txt` vs `g6-after1/2-live.txt`, `g6-clocks1-live.txt`).

## 3. Idle Pool Worker Parking (gap 3)

`⏳️async` (`🔔️worker-parking/` new: `🦀️.rs`, `🧬️schema`, `🧫️fixtures`, `🧪️tests`):
- **Before:** every idle worker parked for ≤ 4 ms (`MAX_IDLE_PARK_MS`), and every submit, timer or permit release did `notify_all`. An idle pool woke all workers 250×/s, and each scanned every lane queue plus the maintenance registry under their locks.
- **After:** `WorkerParking` uses one signal generation plus an idle count (SeqCst store/load pairing, so no lost wakeup). A work signal wakes **one** sleeper. Exactly one idle worker is the **timer keeper** and sleeps until the earliest wheel deadline. `TimerWheel` announces a new earliest deadline through a hook (`signal_timer`). A worker about to run a job while timers are pending hands the wheel to a sleeper. Shutdown wakes all. `MAX_IDLE_PARK_MS` and `notify_all` are gone.
- **`CancelToken::cancelled()`:** waker registration on every node of the chain. An ancestor's `cancel_now` wakes descendants directly, and dropping the future removes its registrations.
- Laws: fixture `🔔️worker-parking/🧫️fixtures/🔣️.json` covers 5 protocol cases, the idle quiet window, a timer deadline and a cancellation chain. Tests: `worker_parking_protocol_matches_the_neutral_fixture`, `an_ancestor_cancel_wakes_a_descendant_waiter_once_and_drop_retains_nothing`, `an_idle_native_pool_sleeps_without_a_poll_interval` (0 re-parks in a 250 ms quiet window) and `a_timer_deadline_wakes_exactly_the_parked_keeper`: **4/4** (`g6-async-tests1.txt`).

## 4. Guest Cost: Fuel Metering And Clock Reads (the remaining factor)

- **Fuel metering (the big one).** The shared engine compiled every guest with `consume_fuel(true)`, but every budget the gateway arms sets fuel to `u64::MAX`, so the metering only instrumented every block. An A/B with a temporary unmetered build, interleaved and at the same load: **35.9 / 53.9 s unmetered vs 84.1 / 79.6 s metered** (`g6-nofuel-warm*-live.txt`, `g6-fuel-warm*-live.txt`).
  - Fix: `SharedEngineConfig::fuel_metering` (default `true`). `consume_fuel` follows it, and `arm_store_fuel` grants fuel only on a metered engine.
  - The engine-config hash writes `fuel={0|1}`. The metered string is byte-identical to before, so **no existing `.cwasm` cache was invalidated** for any other host. The gateway's `shared_plugin_runtime` builds unmetered, with its own cache namespace; the first solve after that paid one cold compile.
  - Law: `an_unmetered_engine_is_its_own_cache_namespace_and_grants_no_fuel` (7/7 engine tests).
- **Clock reads.** A counting wasi clock (temporary) found **6 692 297 `monotonic-clock.now` host calls** per solve, one per WFC unit.
  - Fix: `semio_framework_job::ClockStride` (`🧵️job`). A job's per-unit clock is read for real every `stride` units, recalibrated to about 64 µs between reads, and read first under every new budget. Law `🧫️fixtures/🪜️clock-stride-law.json` + `🧪️tests/🔬️clock-stride` 2/2.
  - `🀄️wfc` `WfcJob`/`WfcRestore` use it, and `emit_preview` takes its caller's reading. W1 rebuild `afcc3a1f…` (descriptor/shared) / `562d05ba…` (dist/staged, post-p5): `wp-w1/requests/g6.txt`.
  - Measured: 2.72 M calls remain after the change. The rest come from framework transitions I could not attribute without a profiler (§8).
- **Epoch ticker.** `PeriodicPoolTimer` resubmitted a lane job every 1 ms, so each tick woke the keeper plus a second worker. The tick now runs inline as a timer-wheel callback. A deadline re-armed from the firing worker (`FIRING_TIMERS`) wakes nobody else. Law `a_timer_re_armed_from_its_own_callback_wakes_only_the_keeper`. Measured: process CPU during a solve dropped from 43–60 s to 20–24 s at the same load.

## 5. Relay-Lifecycle Law Verdict

The law encodes real invariants (§2.1) and is not over-specified, so it stays unchanged and green. The per-crossing hop was never part of it. It came from pool-hosted execution, which the new design keeps only for suspended requests.

## 6. Gates (final tree unless noted)

| gate | measured | capture |
|---|---|---|
| client-e2e | **38/38** (genesis `inference_run` SUCCEEDED) | `g6-client-e2e.txt` |
| stdio probe transcript | **17/17** (cancel answered 4 ms after `notifications/cancelled`) | `g6-probe-final.txt` |
| live-agent-loop (note react serve :6390, local-only) | **20/20** | `g6-live-agent-loop-final.txt` |
| crossing-grant law + wfc inference module | **19/19** | `g6-law-native.txt` |
| binding-cancellation law | **1/1** | `g6-binding-law-final.txt` |
| worker-parking laws (Rust) + AJV/JS oracle (`worker-parking-check`) | **5/5** + oracle green | `g6-async-parking-final.txt` |
| async lib | 73/75; the 2 red are pre-existing, not mine (a cooperative DRR test pumps once but needs 2 pumps; a source-grep test for the cooperative pump call G5 removed) | `g6-async-tests-final.txt` |
| plugin-host relay + wasmtime_runtime + shared engine | **44/44** | `g6-host-tests-final2.txt` |
| plugin-host shard tests | 65/69, the same 4 red before and after my timer change (mock `ShardLoop` pump; no relay/pool involved) — not proven pre-existing against a baseline | `g6-host-shard-tests.txt` |
| framework-job lib | **30/30** serial; in parallel `retained_ownership_tests` race on process-wide session slots (SIGABRT, untouched) | `g6-job-tests-serial.txt` |
| wfc engine | 300/301; `every_large_domain_unit…` (8 ms max-sample wall clock) is also red with my change backed out at load 23 (54 ms / 8.3 ms) | `g6-wfc-engine-tests.txt` |
| os-mcp lib (inference/workspace/dispatch/actions/channel/notify/ui) | **228/228** (before the fuel/ticker edits, which touch no MCP code path under test) | `g6-mcp-lib-tests.txt` |

## 7. Pids

Note serve on :6390 (pids 37318/37321/37348/37436, mine) was stopped after the live-agent-loop run. None left running.
Removed: `wp-g6/target-nofuel` and the debug engine's `.cwasm` dir `bacd0bf8…` (both mine). Kept: the gateway's unmetered `.cwasm` dir `5178f2cd…` and `component-identities/`.

## 8. Files Changed

| path | change |
|---|---|
| `🧰️framework/🔨️modules/⏳️async/🦀️.rs` | `CancelToken` waiters + `cancelled()`; `TimerWheel` earlier-deadline hook; native pool parks on `WorkerParking` (no 4 ms poll, no notify_all); timers re-armed from the firing worker wake nobody |
| `…/⏳️async/🔔️worker-parking/{🦀️.rs,🧬️schema/🔣️.json,🧫️fixtures/🔣️.json,🧪️tests/🔬️unit/🦀️.rs}` (new) | parking primitive, schema, law, tests |
| `…/⏳️async/🧪️tests/🔬️native-pool-unit/🦀️.rs` | idle quiet-window, keeper-deadline and re-armed-tick laws |
| `…/⏳️async/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}` | `worker-parking-check` (AJV + JS model, `--native` laws) |
| `🧰️framework/🔨️modules/🧵️job/🦀️.rs`, `🧫️fixtures/🪜️clock-stride-law.json`, `🧪️tests/🔬️clock-stride/🦀️.rs` | `ClockStride` + law |
| `✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/🦀️.rs` | WFC reads its clock through `ClockStride` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` | inline-first relay requests (`GuestRelayDriveState`), cancel via `cancelled()`, `close_step` Rejected clears cleanup, `SharedEngineConfig::fuel_metering`, `PeriodicPoolTimer` ticks inline |
| `…/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs` | heartbeat on the new `PeriodicPoolTimer` signature |
| `…/🖥️host/🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs` | unmetered-engine law |
| `…/🌉️mcp/🔀️dispatch/🦀️.rs` | `BindingPhase`, `ActivationScope`, `ArtifactChannel::activate`, `ActionAdapter::activate_session`, `job.cancelled` → `CANCELLED` |
| `…/🌉️mcp/🏠️workspace/🦀️.rs` | persisted component identity → hash, `scoped_compiled_component`, scoped open/instance, Routing/Plugin/ShellRouted `activate`, cancellable binding jobs, unmetered gateway engine |
| `…/🌉️mcp/🐚️channel/🦀️.rs` | Shell `activate` |
| `…/🌉️mcp/💡️inference/🦀️.rs` | cancel hook before binding, binding as a job with phase progress |
| `…/🌉️mcp/💡️inference/🧬️schema/🔣️.json`, `🧫️fixtures/⏱️binding-cancellation-law.json`, `🧪️tests/⏱️binding-cancellation-law/🦀️.rs` (new) | schema-first binding phases + law |
| `…/🌉️mcp/📦️packages/🦀️rust/Cargo.toml` | `[[test]] binding_cancellation_law` |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | `🛠️dev⏳️async🛌️worker-parking-check` |
| ticket | `wp-g6/{g6-solve-probe.ts,g6-transcript-probe.ts,g6-build-mcp.sh,g6-native-solve.sh,g6-native-cpu.sh,g6-idle-cpu.sh,g6-sample-solve.sh,g6-serve-note.sh}`, `wp-w1/requests/g6.txt` |

All temporary `[DEBUG] g6` probes (relay counters, counting wasi clock, binding timers) were removed. A grep over `🧰️framework` and wfc finds none.

## 9. Honest Gaps

1. **Wall-clock factor at this load is not a clean number.** At load 27–45, the native reference alone swung from 14.4 to 72.9 s. The load-robust figure is the solving thread's CPU: **1.29–1.33×** (< 1.5×). Whole-process CPU is 1.50–1.66× because it includes the 1 ms epoch keeper. Rerun `wp-g6/g6-native-cpu.sh` + `g6-solve-probe.ts` on a calm machine for a wall figure.
2. **Epoch keeper.** It still wakes 1 000×/s while any engine exists, about 8 % of a core. A ticker that runs only while a store is inside a guest call would remove it; not done.
3. **2.72 M clock reads remain** after `ClockStride`. They come from framework-side per-transition reads (the infer Pump / step contexts). `sample` hangs in this environment, so I could not attribute them. A framework-wide `ClockStride` in `StepContext` is the next step.
4. **Uninterruptible compile.** A cancel during `loading-compiled-code` is answered at once, but a cold cranelift compile (tens of seconds) finishes into the cache in the background. The law documents this as `interruptible: false`.
5. `artifact_create` is not yet a scoped job: it hashes and compiles without progress on the first call per build. It reuses the same scoped pipeline, so wiring it is small.
6. The clock-stride and binding laws are replayed in Rust only; worker parking also has the AJV/JS oracle.
7. Shard (4), ui-patch (3), schema-parity (2), owned-codec (1), async-cooperative (2) and job retained-ownership (parallel) reds were not caused by this slice as far as I can tell. The shard ones are not proven against a clean baseline.
8. W1's descriptor/shared build is `afcc3a1f…` (pre-p5) and dist/staged is `562d05ba…` (post-p5). Both contain the ClockStride change; W1's final pass re-aligns them.
