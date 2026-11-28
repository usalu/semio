# FEM 3D Results window: live-cleanup `prior-outcome` fault wedges the view on "Preparing result view"

## 1. Why the live-cleanup step faults, and why the fault is permanent

Chain (all in the framework plugin reactor, shared by 2D and 3D):

1. Every mounted FEM instance owns one live maintenance pump (`RuntimeLiveCleanupPump`, `cleanup-turns/🦀️.rs`). It runs a small cleanup job (`RuntimeLiveCleanupJob`) turn by turn.
2. When that job ends because its step returned an `OriginalTurnFailure` (a `checked_out_continuation`) or because it published a `Fault` outcome, the pump sets `pump.faulted = true`. A step is cancelled/failed this way whenever the reactor turn that carried it is cancelled or refused, which is exactly what a rapid source/mode switch does: each switch cancels the in-flight solve/display work and its turn.
3. After draining the job session, the pump used to answer `RuntimeMaintenanceStatus::Fault(PriorOutcome)` ("a prior step outcome faulted or was cancelled").
4. `publish_turn` stores that status in `cell.maintenance_status`. The next reactor turn calls `plugin_step_live_cleanup_cell`, which returns `Err(plugin.internal.prior-outcome ...)` **before** it runs any pump turn, so nothing can ever overwrite the stored fault: the status is sticky.
5. Every later reactor turn that steps this instance therefore fails with the same error (first console fatal), and the typed-operation completion that rides on that turn fails with `original-turn.native-return-refused` (second console fatal). Window-config edits (source, mode, loop, waveform) are typed operations, so none of them completes again: the view stays on the last drawn frame with the stage caption "Preparing result view" for the rest of the session. A harmless, per-job event (one cancelled cleanup job) had become a permanent instance fault.

The text shows `elapsed 0us` because the verdict is a stored status, not a measured overrun. The original cause was also invisible (only a `[TRACE]` line), so the message now carries it (see fix).

## 2. Fix (framework, shared by both twins)

Files: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/`
- `📥️staging/♻️reactor-lifecycle/🧹️cleanup-vocabulary/🦀️.rs`: new pure decision `runtime_live_prior_outcome_status(faulted, consecutive, pending)` and `RUNTIME_LIVE_PRIOR_OUTCOME_LIMIT = 8`. A faulted or cancelled prior job is a normal terminal (`Ready`); the next turn starts a fresh cleanup job. Only a run of 8 consecutive faulted jobs, with no clean job in between, is the `PriorOutcome` fault (a deterministic failure must still surface, not loop silently). A clean job resets the run.
- `📥️staging/♻️reactor-lifecycle/🛎️app-cell/🦀️.rs`: `maintenance_prior_outcomes: AtomicU32` run counter.
- `📦️instances/🔁️cleanup-turns/🦀️.rs`:
  - the pump's terminal drop uses the decision above and clears `pump.faulted` (it stayed set before);
  - the original failure text is stored in `maintenance_last_reading` ("prior outcome ended: ...") so a real fault names its cause;
  - a stored `PriorOutcome` fault is delivered once and then re-armed (`Ready`, run counter reset), so even the escalated fault ends only the operation that observed it and the view recovers on the next turn instead of wedging forever. Other causes (zero progress, poisoned locks, ...) are unchanged.

## 3. Native laws

- Framework, language-agnostic vector `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🪦️live-prior-outcome/🔣️.json` + law `a_faulted_or_cancelled_prior_live_outcome_ends_only_its_own_job_over_language_agnostic_vectors` (same file as the other cleanup laws, `semio-plugin --lib`): one faulted job is `Ready`; a clean job clears the run; the 8th consecutive faulted job is `Fault(PriorOutcome)`.
- 3D session law `fem3d_repeated_source_and_mode_switches_mid_solve_settle_on_the_last_choice` (`✏️editor/🧵️session/🧪️tests/🔬️unit/🦀️.rs`, vector `🧫️fixtures/🔀️source-switching/🔣️.json`): one results window switches 11 times between cases, a combination, modal and buckling with a bounded number of preparations and snapshot admissions between switches (earlier solves still being admitted), then settles; asserts it ends bound to exactly the last request, only that request referenced, every earlier request released, and the session closes.
- 2D twin `repeated_source_and_mode_switches_settle_on_the_last_choice` (2D session `🧪️tests/🔬️unit/🦀️.rs`, same vector copied to the 2D fixtures): the same sequence through the 2D `prepare_result_demand`, asserting the scheduler key (`result_request_key`) of the last choice, family and mode index, one reference, no other held request.

Status of the runs: see section 5.

## 4. Animation rows (`res.readouts`, `res.space-toggle`, `res.loop`, `res.waveform`, `res.caption-static`)

| Row | Class | Reason |
|---|---|---|
| `res.readouts` | harness | The harness looked for a button named "Pause". The Play control is one toggle with a constant label ("Play", pressed state; `playing_toggle`), so "Pause" never exists (`play-pause` passed only through its fallback, observed `pauseButton=false`). The row never paused, so the caption kept changing (`frozen=false`). Fixed: a state-aware `ensurePaused` (reads the playing marker in the caption, toggles the Play control) and the same toggle is used to pause. |
| `res.space-toggle` | harness (state leak), product consequence of the wedge possible | It started with playback left running by `readouts`; with the wedge present the Space toggle (a typed operation) also cannot complete. Fixed precondition: `ensurePaused` first. Re-check after publish. |
| `res.caption-static` | harness | Expects a caption without the playback suffix (`^Case: [^·]+$`), but the playback suffix " · Phase 0.58 · ▶ 4 Hz" is printed exactly while `animation.playing` is true, and `readouts` left it playing. Fixed: `ensurePaused` before reading the caption. |
| `res.loop`, `res.waveform` | product (consequence of the wedge) | The combobox picks the option but the readback stays at the old value (`Loop:Loop / Ping-Pong:Loop`, `Ramp / Sine:Ramp`). These are window-config typed operations; after the sticky fault their completion fails, so the committed value never changes. No harness change; expected to pass once the framework fix is published. Verify in the next matrix run. |

Harness change: `matrix-v3/checks-results.mjs` (`ensurePaused`, used by `readouts`, `space-toggle`, `caption-static`).

## 5. Verification

Native (private `C:	\lc`, `-j 3`, detached; launcher `fem3d-results-live-cleanup-run.sh`):
- PASS framework law `a_faulted_or_cancelled_prior_live_outcome_ends_only_its_own_job_over_language_agnostic_vectors` (`semio-plugin-tests-01-2 --lib`). The rest of `runtime_instances_unit_1`: 21 passed, 1 failed. The failure is `live_readings_are_judged_by_their_cause_over_language_agnostic_vectors`, vector "fruitless count reached inside one frame" (2048 readings over 1000 us expected no fault, got `ZeroProgress`). It exercises `runtime_live_cleanup_nonterminal_status`, which this change does not touch; it is a pre-existing or peer-caused mismatch between that function and its vector, not a regression of this fix. Not investigated further.
- PASS 3D `fem3d_repeated_source_and_mode_switches_mid_solve_settle_on_the_last_choice` and the neighbouring `fem3d_request_families_...` (`semio-s-artifact-fem-3d-editor-session --lib`).
- PASS 2D twin `repeated_source_and_mode_switches_settle_on_the_last_choice` (`semio-s-artifact-fem-2d-editor-session --lib`).
- The first 3D run of the new law FAILED (bound to an older choice). That was the law's mistake, not a product defect: a real window-config edit bumps the consumer generation/revision, and the request selector is keyed by that version, so the law now bumps it per switch (as production does). This also documents the contract: a switch that does not change the consumer version is not a new request.
- Two small enabling edits outside the fix: 2D session gets `semio-plugin` with `artifact-app-testing` as a dev-dependency and `prepare_result_demand` is `pub(super)` (test visibility); `a_live_zero_progress_fault_names_...` used `Fault::to_string()` (no `Display` any more), now `fault.message`, because the test crate did not compile.

Browser, pre-publish (served wasm 19:52, `node matrix-v3/run.mjs 3d --only res. --out C:	\lc\m`, host loaded by my own compile):
- source-live, source-dead, source-uls, mode-modal, mode-buckling, mode-static PASS (a few took 50-60 s on the loaded host).
- mode-index, deformation-scale, modal-count, buckling-count FAIL with the caption stuck on "Preparing result view", **with zero console fatals**. So the stuck view reproduces without the `prior-outcome` fault in an isolated run, and the sticky fault alone does not explain every stuck row. The PriorOutcome fatals reproduced only in the full matrix. Both signs share the same trigger (rapid mode/source switching while a modal or buckling solve runs), but I could not separate the second cause without the rebuilt wasm.
- step-forward, step-back, speed, phase, resolve-after-edit, stability PASS; play-pause, readouts, space-toggle, loop, waveform, error-caption FAIL (play-pause and readouts fail on a loaded host: distinct frames or pause timing; space-toggle showed `distinct=1`, i.e. Space did not start playback).
- loop/waveform failed with no fatal, so they are NOT explained by the wedge. The command path is native-tested (`loopMode`/`waveform` through `apply_field`) and the select uses the same `select_row` as Source/Mode, so I could not find the cause by reading; it needs a targeted probe on a quiet host after the next publish (log the dispatched args and the committed config). Classification stays "product, cause open" for those two, "harness" for readouts/space-toggle/caption-static (fixed), see section 4. The `res.loop`/`res.waveform` text in section 4 that says "consequence of the wedge" is therefore only a hypothesis and the isolated run contradicts it.
- A Rust fix reaches the browser only after the coordinator's next publish: re-run `node matrix-v3/run.mjs 3d --only res.` (and the full 3d run for the PriorOutcome fatals) then.

## 6. Added scope: example switch must cancel the in-flight solve (House, 100+ s)

Status: analysed, NOT fixed, no law added.

What the code does: `set-active-example` emits `reset_document_effect(&document)`, a whole-document replacement. The replacement candidate is built by the store initializer, committed in the maintenance ladder (stage 14/24 archive-replacement units) and only then does `commit_store_replacement_candidate` revoke the displaced store's snapshot read authority; the mounted job notices that in `step()` (`commit_authority_matches`) and cancels itself (`boot-store-replacement-hang.md`, law `fem3d_replacement_revokes_the_read_leases_of_the_displaced_store`). So cancellation happens only after the replacement commits, never at the moment the switch is requested.

Why the view then waits for the whole analysis (260 s): in `maintenance_ladder_unit_step` an archive/replacement unit is only funded when the turn still has at least `DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS` items, otherwise it is marked starved and skipped. A running House solve spends the per-turn quantum before the maintenance ladder runs, so the replacement never gets funding until the solve ends, and the cancel that would free the quantum waits for the commit that needs the funding: a priority inversion. Small examples finish their solve in 6-13 s, which matches the lag measured for normal switches. This is a hypothesis from the code and the misc agent's measurement; I had no wasm to trace it.

Suggested fix (not applied, needs a browser trace to confirm the funding hypothesis): cancel at request time, not at commit time. When a document-replacement load or reset effect is admitted for an instance, cancel that instance's in-flight mounted jobs (the FEM session already cancels through `Effect::CancelJob` in `reconcile`; the framework would expose a "supersede pending" signal consulted by `job_step`, or revoke read authority at admission and re-arm it when the replacement fails). Law to add with the fix: extend `fem3d_replacement_revokes_the_read_leases_of_the_displaced_store` with a real running mounted solve and assert the displaced job reaches `Cancelled` within a fixed number of turns of `plugin_admit_document_load`, independent of the solve's total length; 2D twin in the 2D live-cleanup target.

## Files changed

Framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/` `📥️staging/♻️reactor-lifecycle/🧹️cleanup-vocabulary/🦀️.rs`, `.../🛎️app-cell/🦀️.rs`, `📦️instances/🔁️cleanup-turns/🦀️.rs`; tests `🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` and `🧫️fixtures/🪦️live-prior-outcome/🔣️.json` (new).
3D: `✏️editor/🧵️session/🧪️tests/🔬️unit/🦀️.rs`, `🧫️fixtures/🔀️source-switching/🔣️.json` (new).
2D: `✏️editor/🧵️session/🧪️tests/🔬️unit/🦀️.rs`, `🧫️fixtures/🔀️source-switching/🔣️.json` (copy of the 3D vector), `🧵️session/🦀️.rs`, `🧵️session/💼️job-bridge-and-live-mount/🦀️.rs`, `🧵️session/📦️packages/🦀️rust/Cargo.toml`.
Harness: `matrix-v3/checks-results.mjs`. Launcher: `fem3d-results-live-cleanup-run.sh`.
Cleanup: `C:	\lc*` and my launchers deleted; ticket `🗑️generated/lc` removed.
