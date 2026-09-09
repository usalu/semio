# Wave Q — step-budget quarantine is now a law about the job's work (2026-09-09)

Scope: the per-step interactive law must stop killing jobs because the MACHINE descheduled the thread.
No budget constant changed. `⚛️reactor/🔄️turn/🦀️.rs`, `🕹️interaction/**` and the coordinator's `[DEBUG]`
traces were not touched; no wasm was rebuilt and no server was started.

Toolchain for every command below (all foreground):

```
RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 \
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d \
cargo <cmd>
```

---

## 1. Inventory — every site that turned a wall-clock overrun into a terminal fault

| # | site | what it did on overrun | now |
| --- | --- | --- | --- |
| 1 | `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:99` `interactive_step_contract_violated` | `elapsed_us >= INTERACTIVE_STEP_CEILING_US` (8 000). Pure predicate over WALL microseconds. | unchanged — it is still the definition of "this step did not stay interactive" |
| 2 | `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:394` `CallbackVerdict::is_fault` | `clock_fault OR over-ceiling`; every caller treated it as a quarantine authority | unchanged behaviour, re-documented as a MEASUREMENT of one sample; the quarantine authority is now `StepOverrunLedger` |
| 3 | `🧰️framework/🔨️modules/🧵️job/🦀️.rs:2478` (`drive_worker_job_authority`) | **the source of every `job-session.terminal-fault` in the puzzle3d failures.** One over-ceiling verdict replaced the step's real outcome with the pre-admitted `job-session.terminal-fault` page → session terminal → the whole retained operation faults | quarantines only when the session's own `StepOverrunLedger` attributes the breach to the step |
| 4 | `🧰️framework/🔨️modules/🧵️job/🦀️.rs:1235` (`drive_step_with_payload_ledger`) | an over-ceiling verdict suppressed the `record_preview_published`/`record_checkpoint`/`record_committed`/… trace record for a step that really did the work | suppresses the record only for an unusable clock reading (`clock_fault`), because then nothing was measured at all |
| 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:17668` (`run_framework_reserved_job`) | `overrun = callback_verdict().is_fault()` → `cancellation_lease.cancel()` on a non-terminal step and `interactive-job.step-overrun` on the terminal one | one per-invocation ledger; the fault message now carries the consecutive count and worst reading |
| 6 | `…/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:244` (`Deadlines`) | `quarantine_overrun` → `record_frame_fault` + cancel the whole frame | one `StepOverrunLedger` per `ActiveFrameBuild` |
| 7 | `…/🧵️frame-job/🦀️.rs:283` (`ApplyPending`) | idem | idem |
| 8 | `…/🧵️frame-job/🦀️.rs:304` (`Build`/transaction) | idem | idem |
| 9 | `…/🧵️frame-job/🦀️.rs:326` (`Prepare`) | idem | idem |

Sites checked and deliberately **not** changed:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs:170/173`, `:224`, `:287`,
  `:625/627`, `:665` — the reactor executor's `deadline` is a **cooperative yield**, never a fault:
  `run_until_deadline` stops handing out slices (`while … && Instant::now() < deadline`) and
  `close_instance_step`/`shutdown_step` return `Pending` when the grant is spent. Nothing there is
  terminal, so nothing there needed the ledger. Its only terminal verdicts are the unit/byte-budget
  violations at `:265` and `:330`, which are protocol violations by the task, not clock readings.
- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🏁️tail/🦀️.rs` (`WatchdogAdmission`) — mints verdicts, takes no
  terminal decision, and has **no consumer outside the trace crate** (`grep -rn 'admission_checkpoint\|finish_after_telemetry' --include='*.rs' .` returns only that module and its own test).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28386` `runtime_live_cleanup_publish_turn` — the
  coordinator already made this non-fatal; left exactly as found.
- `✏️s/…/◻️2d/…/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:132` — a TEST asserting its fill session's sample stayed
  clean. That is a legitimate use of `is_fault()` as a measurement and still compiles unchanged.

---

## 2. The law, and why this mechanism

`🧰️framework/🔨️modules/⏱️trace/🦀️.rs` — new `//#region 📒️OverrunLedger`:

- `SUSTAINED_OVERRUN_QUARANTINE_STEPS: u32 = 4` — how many CONSECUTIVE steps of ONE session must each
  breach `INTERACTIVE_STEP_CEILING_US` before the breach is attributed to the step.
- `StepQuarantine { Admitted, RecordedOverrun { consecutive }, SustainedOverrun { consecutive }, ClockFault(_) }`
  with `is_terminal()` true only for `SustainedOverrun` and `ClockFault`.
- `StepOverrunLedger` — four `u32`/`u64` counters, `Copy`, no allocation, no lock, usable from any
  thread and from wasm. `admit(&CallbackVerdict) -> StepQuarantine`; any admitted step resets the run.
- `step_overrun_counts() -> (recorded, sustained_quarantines)` — process-wide, alongside the existing
  `Watchdog::violations()` / `violation_count()` ring that already records every single breach.

**Every budget constant is unchanged**: `INTERACTIVE_STEP_CEILING_US` (8 000),
`INTERACTIVE_LANE_WALL_US` (1 000), the lane fuels, `GUEST_LIFECYCLE_TURN_CEILING_US`, the soft
targets. The new constant is a quarantine threshold, not a budget.

### Why (a) consecutive-overrun count and not (b) thread CPU time or (c) fuel vs. elapsed

- **(b) thread CPU time** attributes an overrun exactly, and it is the mechanism I would pick if this
  were a native-only runtime. It cannot be the law here: `clock_gettime(CLOCK_THREAD_CPUTIME_ID)`
  exists only off-`wasm32`. WASI p2's `wasi:clocks` publishes a monotonic clock and a wall clock and
  **no per-thread CPU clock**; the browser build has neither, only `performance.now()`. So a CPU-time
  law would be native-only, and the one target where descheduling is worst — a browser worker sharing
  a core with ~20 booting WASM plugins (`📓️…hidden-browser-pane-throttles-plugin-boot`) — would keep
  running the exact law this wave replaces. The trace module is also declared zero-dependency pure
  `std`, and `std` exposes no thread-CPU clock, so (b) would need a per-target host seam as well.
  **What wasm does under the chosen law: exactly the same thing native does**, because a consecutive
  count needs only the one monotonic clock every target already installs.
- **(c) declared/consumed fuel vs. elapsed** fails on the measured evidence: the steps that trip this
  ceiling are the cheap ones. `📓️…wave-R2…` §3 measured a Concrete Forest maintenance unit at
  **14 571 µs wall for 1–43 µs of work**, and `📓️…wave-D2…` records the same shape; those steps consume
  little or no fuel, so a fuel-to-elapsed ratio is dominated by the very noise it is supposed to
  filter. Fuel is also job-declared: a job that under-declares would be quarantined for honesty and one
  that over-declares would be immune.
- **(a) consecutive count** is the only mechanism that is (i) identical on native and wasm, (ii) needs
  nothing a job has to declare, (iii) still stops a genuinely runaway step. Sized at 4: a descheduling
  burst has to survive four separate scheduler decisions in a row, while a step that genuinely costs
  more than 8 ms overruns *every* step and is stopped after at most `4 × 8 ms = 32 ms`.

### Measurements behind the threshold

| evidence | number |
| --- | --- |
| one descheduled maintenance unit, full 584-test suite, this box (`📓️…wave-R2…` §3) | **14 571 µs** wall |
| the same unit, same document, run alone | **1–43 µs** — a ~340× ratio |
| a second full-suite round: stage 0 / stage 8 | 2 220 µs / 1 160 µs, against 3 µs / 2 µs isolated |
| browser (release wasm) readings for single-digit-µs native units | 8 401 / 9 500 / 10 201 µs |
| a step whose work really is over the ceiling (`fillBuildTick` turn 763, `📓️…wave-R2…` §6.1) | 25 593 µs — and it repeats, which is exactly what the ledger keys on |
| puzzle3d failures of this class in the run-2 audit snapshot (`📓️…remaining-test-failures-audit…` §5 + §6) | 22 of 38 |

The fixture case `runaway-every-step` uses those two real numbers (`14 571`, `25 593`) as its samples,
so the law is asserted against measured readings rather than invented ones.

---

## 3. Changes, file:line

### `🧰️framework/🔨️modules/⏱️trace/🦀️.rs`

- `:18-22` module doc — `CallbackVerdict` is the measurement, `StepOverrunLedger` the quarantine
  authority, and why (the only guaranteed clock measures wall time).
- `:368-370` `ContractViolation` doc — points at the ledger.
- `:393-400` `CallbackVerdict::is_fault` doc — "a measurement, never on its own a quarantine
  authority", with the 19 µs / 14 571 µs measurement inline. Behaviour unchanged.
- `:508-611` new `//#region 📒️OverrunLedger`: `SUSTAINED_OVERRUN_QUARANTINE_STEPS`,
  `RECORDED_STEP_OVERRUNS`, `SUSTAINED_OVERRUN_QUARANTINES`, `StepQuarantine`, `StepOverrunLedger`,
  `step_overrun_counts`.

### `🧰️framework/🔨️modules/🧵️job/🦀️.rs`

- `:1235` `drive_step_with_payload_ledger` — the trace record is now skipped only on
  `verdict.clock_fault().is_some()`, not on any over-ceiling sample.
- `:1888` `WorkerJobAuthority` gains `overruns: semio_framework_trace::StepOverrunLedger`.
- `:1900-1913` `try_new` seeds it.
- `:2050-2056` `WorkerJobOutcome::overrun_ledger()`, mirrored on `MountedWorkerJobSession` and
  `BatchJobSession` (`overrun_ledger()` next to the existing `callback_verdict()`), so a host can read
  the counters without waiting for a quarantine.
- `:2456-2462` `drive_worker_job_authority` docstring — states the new law.
- `:2495-2503` the quarantine decision: `authority.overruns.admit(&verdict).is_terminal()` instead of
  `verdict.is_fault()`. A quarantined step's original outcome still moves to `quarantined_outcome`,
  and the panic / invalid-deadline branches are untouched and still terminal.

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

- `:17744` `run_framework_reserved_job` — one `StepOverrunLedger` per invocation.
- `:17769` `overrun` is now `step_overruns.admit(&verdict).is_terminal()`.
- `:17815-17821` the `interactive-job.step-overrun` fault message now names the consecutive count and
  the worst reading instead of asserting a bare "exceeded the 8 ms step ceiling".

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs`

- `:172` `ActiveFrameBuild` gains `overruns: StepOverrunLedger`; `:212` seeds it.
- `:225-229` `quarantine_overrun` docstring — states what now reaches it.
- `:248-249`, `:287`, `:308`, `:330-331` — the four phase checks route through the frame's own ledger.
  One ledger per frame, because the frame is the job: a frame that overruns four phases in a row is
  runaway regardless of which phase.

### Tests and the language-agnostic law

- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧫️fixtures/🧪️contention/🔣️.json` — added
  `sustainedOverrunSteps: 4`, `sessionTerminal` on every `verdicts` row (whether ONE such step
  terminates its session), and a new `overruns` array of six sequence laws.
- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧬️contention/🧬️schema/🔣️.json` — the schema module for both.
- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🧪️contention/🦀️.rs:126-181` — the Rust reader of that
  fixture plus `microsecond_unusable_clock_reading_stays_terminal_and_resets_the_overrun_run`.
- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts` — the
  **third-party oracle**: ajv validates the fixture against the schema module, and an independent
  TypeScript reference implementation recomputes `terminalIndex`/`consecutive`/`longestRun`/`total`/
  `worstElapsedUs` for every case and rejects a threshold of 1. Registered already through
  `📜️script.ts:5/31023` and `🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts:1854`, so no new registration and no
  new taxonomy entry was needed.
- `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs` — `SlowProbe` really sleeps
  `INTERACTIVE_STEP_CEILING_US + 2 000 µs` and reports the wall time it actually burned to the
  installed clock, plus the two required laws:
  `descheduled_step_over_the_ceiling_is_recorded_and_the_job_keeps_running` and
  `every_step_over_the_ceiling_is_quarantined_within_the_sustained_threshold`.

### One pre-existing test defect fixed on the way

`🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs:6-8` bound the fixture clock as a
thread-local **defaulting to `Some(1_000)`**, and `microsecond_exact_callback_quarantine_…` installs
that function PROCESS-WIDE via `install_microsecond_clock`. Since `default_now_us()` is
`semio_framework_trace::try_now_us()`, every later test in the binary then read a frozen 1 000 µs, and
`microsecond_platform_clock_and_real_half_ms_worker_progress` failed with `platform clock lost
microsecond precision` — verified failing **with my two new tests skipped**, i.e. pre-existing, not
introduced here. Fixed by making the binding explicit: `CLOCK: Cell<Option<Option<u64>>>` defaulting to
unbound, `bind_clock` / `unbind_clock`, and a real `platform_now_us()` fallback for unbound threads.
All call sites were converted; the crate suite is now 24/24 green.

---

## 4. Verification

### 4.1 `semio-framework-job` — its own suite

```
cargo test -p semio-framework-job -j 4 -- --test-threads=1
```

Three consecutive runs:

```
run 1: test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
run 2: test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
run 3: test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s
```

Including the two new laws:

```
test component::microsecond_budget_tests::descheduled_step_over_the_ceiling_is_recorded_and_the_job_keeps_running ... ok
test component::microsecond_budget_tests::every_step_over_the_ceiling_is_quarantined_within_the_sustained_threshold ... ok
```

`--test-threads=1` is this crate's own mode, not a concession to the change: with default parallelism
`worker_session_slots_max_plus_one_…` / `retained_payload_max_plus_one_…` abort the process (SIGABRT,
"panic in a destructor during cleanup") because they assert on process-global fixed slot/byte
registries. Reproduced **with my two tests skipped**, so it is pre-existing.

### 4.2 `semio-framework-trace`

```
cargo test -p semio-framework-trace -j 4
```

Both new laws pass in every run:

```
test component::microsecond_telemetry_contention_tests::microsecond_sustained_overrun_ledger_quarantines_only_attributable_steps ... ok
test component::microsecond_telemetry_contention_tests::microsecond_unusable_clock_reading_stays_terminal_and_resets_the_overrun_run ... ok
```

This crate has two **pre-existing** parallel-execution failures,
`component::tests::cancellation_latency_measures_requested_to_observed` and
`component::watchdog_tail_tests::watchdog_tail_uses_the_original_guard_for_admission_and_terminal`
(`sample_count().unwrap()` on a `try_lock` that loses to the contention tests, which deliberately hold
the site registry for 100 ms). Reproduced identically with `-- --skip sustained_overrun --skip
unusable_clock`, i.e. with none of this wave's tests running, in three consecutive runs. Not fixed
here — a different owner's file and a different defect class — but recorded.
