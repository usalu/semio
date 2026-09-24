# WP-G5: Over-MCP Inference Throughput (Session 10)

Slice G5 · session 10 · 2026-09-24. Ports: hubs 7860–7869, serves 6360–6369. This slice continues G4 §8.1: the wfc
genesis `inference_run` (16×16 input, N=3, sym 8, 24×24 output) finishes natively but not over the semio MCP.

## 0. Headline

- **Before:** did not finish. After 582 s it had made 541 650 host crossings.
- **After (warm, back to back at load ~23):** the guest solve takes **38.4 s** over MCP. The same snapshot takes **18.5 s** in
  the headless native driver (`solve_with_job`, debug). That is **2.07×** for the solve and **2.57×** for the whole `inference_run`
  (47.5 s including ~7.8 s of artifact binding, §8.1). The <2× target is missed narrowly; §8 lists the remaining levers.
  Across load 15–31 I measured 42.3–51.4 s end to end.
- **Cold compile:** the first solve after a guest rebuild waited **~9 min** for cranelift. It now waits **~36 s** (`g5-final-cold-live.txt`).
- **client-e2e 38/38** on the final tree (`g5-client-e2e.txt`).
- **Cancellation still works:** an in-flight cancel during the guest solve returns CANCELLED **10 ms** after
  `notifications/cancelled` (`g5-final-cancel2-live.txt`).
- **Regression law:** a whole-solve grant takes **41 crossings** (the law allows 48). Forcing the old walk back in makes the
  same law fail at **736 776 crossings**.

## 1. What The Other 90 % Was (measured)

Measured with a temporary `[DEBUG] g5` relay probe (removed) and `sample`. Per crossing: queue wait, main-thread hop,
and guest time.

| cause | evidence | fix |
|---|---|---|
| **One transition per crossing.** Guest `semio.infer` `Pump` advanced one mounted-session transition per host `step-job`, then used a second crossing for `OutcomeClose`. WFC publishes a preview every 16 units, so the host paid 2 round trips per 16 units | 541 650 crossings in 582 s at ~1.07 ms each (guest 0.41, pool queue 0.39, main hop 0.25); the guest's JIT code at the top of the stack in ~26 of ~14 000 worker samples | §2.1 (guest) |
| **Debug-profile wasmtime/cranelift** in every native dev host | with the fix above and a debug host: 112.7 s; the same with wasmtime/cranelift at opt-level 3: 63.9 s. Cold cranelift compile ~9 min → ~36 s | §2.2 (root `Cargo.toml`) |
| **3 host clock calls per WFC propagation edge** (`set_stage` trace timestamp, preview `now_us`, `should_yield`). Each goes through the component-model host-call path, monomorphized in the debug plugin-host | opt-host profile: wasi clock + wasmtime host frames ≈ 2 400 samples next to 4 060 JIT samples | §2.3 (guest) |
| **opt-level-0 guest** (wasm-dev): the JIT share itself | JIT is the largest single share after the above | §2.3 (`[profile.wasm-dev.package]`, same precedent as lowpoly, puzzle3d and remodel) |
| **Deadlock found while profiling:** `run_stdio` held `stderr.lock()` for the whole serve, so any worker-thread `eprintln!` (for example a guest `[actor:…]` log) hung the MCP process forever | my first probe hung at 0 % CPU; `sample` showed the worker parked in `__eprint → lock` | §2.4 |

These were not the cause: per-step instance setup (gate wait ≈ 0.1 %), state copying across the boundary (the progress
payload is ~1.5 KB per preview), fuel/epoch (the host arms a 30 s watchdog, and the guest honours its own grant).

## 2. Fixes

### 2.1 `semio.infer` Pump spends the host's crossing grant (guest; W1 rebuild #1 `e36b9d88…`)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs`: `pump(budget)` loops `pump_transition()` until
  `budget.fuel` (at `WORK_UNITS_PUMP` each) or `budget.deadline_ms` (monotonic clock) is spent. `Yield`/`PreviewReady` outcomes
  are retired in place (`retire_absorbed_outcome`, ≤ `ABSORB_CLOSE_PAGES` = 4) and resumed. The latest preview coalesces into
  the crossing's progress. Lossless and terminal outcomes still leave through `OutcomeClose`. With no clock, one transition runs per crossing.
- `🧰️framework/🔨️modules/🧵️job/🦀️.rs`: new `MountedWorkerJobSession::step_on_caller`. The crossing already owns its slice, so
  the transition runs on the caller: there is no pool submit plus spin natively, and no cooperative-pool pump on wasm.
- `…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`: the now-dead `pump_process_worker_pool_for_inference` and its ceiling are deleted; the turn
  pump is inlined and its constants are gated to wasm32.
- The GJ3 self-check (`🖥️host/🧫️fixtures/♻️inference-pool-pump-law.json` + its `🔬️wasmtime-runtime` test) is retired, because the mechanism it described is gone.

### 2.2 Native dev hosts optimize the plugin runtime (root `Cargo.toml`)

`[profile.dev.package.<27 wasmtime*/cranelift*/regalloc2/wasmparser/pulley/gimli/object/wit-parser crates>] opt-level = 3`.
Measured rebuild: those crates plus relinking their dependents, 1 min 44 s. This has no guest impact.

### 2.3 Guest compute (W1 rebuild #2 `8cf17102…`)

- `✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/🦀️.rs`: the WFC step loop reads the clock **once** per unit (preview due + deadline
  from the same reading, exactly as bounded as `should_yield`). It records a stage change only when the stage actually changes.
- Root `Cargo.toml`: `[profile.wasm-dev.package.semio-s-plugin-wfc-engine]` and `…semio-s-artifact-wfc-bitmap` set `opt-level = 2`.

### 2.4 stdio serve no longer holds the stderr lock

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` `run_stdio`: `stderr.lock()` → `std::io::stderr()`.

## 3. Regression Law

- Fixture: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧫️fixtures/⏱️relay-crossing-grant-law.json`.
  The workload is the gateway's exact wire request (genesis pack/spr, `workUnits` 1024, 1 MiB allocation). `wholeSolveGrant`
  budget: **maxCrossings 48**, **maxWallRatioToHeadlessNative 2.0**.
- Replay: `a_whole_solve_grant_settles_the_genesis_inference_in_a_bounded_state_walk` in `…/💡️inferences/🧪️tests/🔬️unit/🦀️.rs`.
  It drives `start_job`/`step_job` exactly as the host relay does and compares against `solve_with_job` in the same process.
  The crossing count does not depend on load.
- Measured: 41 crossings, 14.9 s vs ~14 s native. With the old walk forced back in: **FAIL, 736 776 crossings** (`g5-law-native-oldwalk.txt`).

## 4. Measurements (`wp-g5/generated/`)

| run | host | guest | inference_run | notes |
|---|---|---|---|---|
| `g5-baseline5` | debug | pre-fix | >580 s, not finished | 541 650 crossings |
| `g5-after2` | debug | #1 | 112.7 s | 36 927 crossings; guest 86 s, queue 14.7 s, main 5.1 s |
| `g5-opt1` | opt deps | #1 | 63.9 s | |
| `g5-final-cold` | opt deps | #2 | 49.8 s (+ artifact_create 35.6 s incl. cold compile) | |
| `g5-final-warm` / `-warm2` | opt deps | #2 | 43.2 s / 47.5 s | warm2 back to back with native 18.5 s → solve 38.4 s = **2.07×** |
| `g5-final-repeat` | opt deps | #2 | 51.4 s, 51.4 s (load 25–31) | the second call's binding took 0.04 s |
| `g5-final-cancel2` | opt deps | #2 | CANCELLED 8 006 ms, answered 10 ms after the cancel | ping ok |

## 5. Gates

| gate | measured | capture |
|---|---|---|
| client-e2e | **38/38** | `g5-client-e2e.txt` |
| wfc-bitmap lib | 152/152 (+1 ignored) | `g5-wfc-bitmap-lib.txt` |
| relay-crossing law + genesis test | 2/2 | `g5-law-native.txt` |
| wfc engine | 300/301 in parallel; the one failure is a wall-clock max-sample test (9.1 ms > 8 ms under parallel load), which passes 2/2 in isolation | `g5-wfc-engine-tests.txt` |
| semio-framework-plugin `jobs::` | 39/39 | `g5-plugin-jobs-tests.txt` |
| semio-framework-job lib | 28/28 | `g5-job-tests.txt` |
| plugin-host `wasmtime_runtime` + `guest_cold_relay` | 37/37 | `g5-host-tests.txt` |
| wasm32 compile of the guest changes | clean inside W1's `describe` (#1, #2) | `wp-w1/generated/stage-wfc-plugin.txt` |
| cargo check plugin / job / plugin-host | green, warning counts unchanged | — |

## 6. Pids

None left running. I did not use G4's hub :7830 / serve :6330.

## 7. Files Changed

| path | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs` | batched Pump on the caller |
| `🧰️framework/🔨️modules/🧵️job/🦀️.rs` | `MountedWorkerJobSession::step_on_caller` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | dead inference pump removed, turn pump inlined |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️wasmtime-runtime/🦀️.rs`, `…/🖥️host/🧫️fixtures/♻️inference-pool-pump-law.json` (deleted) | GJ3 self-check retired |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` | stdio serve no longer holds the stderr lock |
| `✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/🦀️.rs` | one clock read per unit, stage change only on change |
| `Cargo.toml` | dev opt-level 3 for the wasmtime/cranelift set; wasm-dev opt-level 2 for wfc engine + bitmap |
| `✏️s/…/💡️inferences/🧫️fixtures/⏱️relay-crossing-grant-law.json`, `…/🧪️tests/🔬️unit/🦀️.rs` | law + replay |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` | temporary `[DEBUG] g5` probe added and fully removed (net zero) |
| ticket | `wp-g5/g5-solve-probe.ts`, `wp-g5/g5-batch-pump.py`, `wp-g5/g5-build-opt.sh`, `wp-w1/requests/g5.txt` |

## 8. Honest Gaps

1. **2.07× is just above the <2× target** for the solve, and **2.57×** end to end. The remaining levers, in measured order:
   - **Host relay round trip per crossing** (pool queue + main hop ≈ 19 % of relay wall in `g5-after2`). The mounted registry
     throws away `Preview` outcomes, so letting the relay keep stepping on the worker between `Running` crossings (only
     completing on Done/Failed/cancel) would remove it. This conflicts with the `♻️relay-lifecycle` state-machine law and the
     per-step relay tests, so I did not change it.
   - **Artifact binding** in `inference_run` (`resolve_inference_artifact_document` → `read_live_session_artifact_bytes`) takes
     6–29 s on the first call of a session. It is not cancellable: a cancel during binding answered after 19.8 s
     (`g5-final-cancel-live.txt`). A second call in the same session takes 0.04 s.
   - Idle native pool workers contend on `WorkerMaintenanceRegistry`/queue mutexes (`notify_all` + full lane scan), which is the
     largest non-idle host share in every sample. This is a generic `semio-framework-async` cost.
   - WFC publishes a ~1.5 KB JSON trace preview every 16 units (both natively and in wasm); only the latest per crossing leaves the guest.
2. Native and over-MCP timings swing ±25 % with fleet load (13–31 during this slice). The factor is measured back to back, not averaged.
3. `🔬️guest-cold-relay` was run 37/37 in combination with `wasmtime_runtime`. The full plugin-host suite was not run.
