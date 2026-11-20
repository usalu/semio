# Analysis queued and Note Shell Command stall (2026-11-19)

No file was edited. The causes sit in guest Rust (inside the running component build's read closure) and one host-TS mitigation is not worth making. Findings come from code reading plus the live page on 127.0.0.1:6087 (several reloads).

## What the two symptoms are

1. "Analysis queued" is `labels.stage_queued` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🗣️terminology/🦀️.rs:129`). The results window shows it from `progress_caption` (`…/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:361`, `None => stage_queued`) whenever `Fem3dLiveSession::result_stage` returns `None`.
2. `result_stage` (`…/✏️editor/🧵️session/🚀️session-api/🦀️.rs:360`) returns `None` before any job exists: no request-owner binding for the window, or `registry.current[slot]` is empty (no mounted job for this revision). The job is created only by `reconcile` (same file, line 168). `reconcile` silently returns `Vec::new()` unless `registry.pending[slot]` holds a completed snapshot preflight, which `prepare_snapshot_read` / `prepare_snapshot_admission` (lines 76-150) fill in over many retained-grant opportunities. The guest prints `[TRACE] fem3d live_visual reconcile spawn` at line 258 when a job is spawned. That line never appeared in the console, and the gate probe stays `reconcile false` / `0 opportunities`. So the stage that never fires is the spawn in `reconcile`: no job, hence no analysis, hence "queued" forever.
3. "Note Shell Command" is the reserved framework action `noteShellCommand` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs:402`). The shell sends it from `noteShellCommand` in `…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🖱️shell-actions/🟦️.ts:218` for every chrome change (dock, theme, window layout), through the normal `onAction` funnel. It is a normal command crossing: `runQueuedTurn` in `…/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts:326` serializes it per actor behind `serializeCommandIngressForActor` and watches it with `commandStallWatch` (`⏱️command-stall/🟦️.ts`, bound about 25 s). The banner means that turn holds the actor's command lane longer than the bound. It names whichever command holds the lane.

## Common root cause: `Set Active Example` (seq 7) never completes and its failure retirement never ends

Evidence on the live page after a fresh reload (the host boot always sends `setActiveExample` first, window `fem3d-model`):

- Host: `[DEBUG] command ingress fem action=setActiveExample continuation N: terminal command-pending status more-work ... "seq":"7","itemCount":0`, N counting up past 5800 (`handle-turn-drive/🟦️.ts:383`). Banner: `semio · fem · fem3d has not answered "Set Active Example" for 29 s — later input waits for it.` Every later command, including `noteShellCommand`, waits on the lane or on the guest's single command-lease FIFO. That is the "Note Shell Command" banner seen after the first command failed and the next took the lane.
- Guest, phase 1 (about 1000 turns): `exchange_generic: seq 7 command decoded output false applied Some(false)`; neither `exchange.produced` nor `exchange_generic: command seq 7 refused` is printed, so `plugin_exchange_turn` yields forever without producing. This is the same stall as `📓️set-active-example-no-progress.md`.
- Guest, phase 2: after `turns 1062` the command is retained as `FailedDecoded` with `first_failure = Value(InvariantViolated "command producer lost its original command, encoding or wire")` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs:25`, raised by `broken()`, which has about ten raise sites, among them `validate` at line 34 and the `fault.is_some() && phase != BIRTH` check at line 50). The host then rejects the command with `input #1 setActiveExample refused: dispatch-failed ... plugin.command-failed: command producer lost its original command, encoding or wire`.
- Guest, phase 3 (forever): `failed-decoded state: ... owner_terminal_empty false close_failure false exchange_open false ... turns 1062` repeats every turn with only the retained grant draining (about 13.4 KB per iteration). `exchange_generic` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs:164-166`) returns `Ok(())` each turn because `retire_failed_step_on_original(...)` answers `Pending` (the failed owner never reaches `terminal_is_empty`). Retirement never completes, so the owner stays in the ingress slot, the guest keeps reporting `more-work`, and the actor never reaches the reconcile or snapshot-preparation work that spawns the analysis job.

The `[DEBUG] exchange.retire-refused` probe from the "record action before close" fix does not appear, so that fix works (no refusal), but the retirement itself is a new non-terminating state: this is the stage to fix.

## Fixes

Host TS: none made. A host fix cannot create the missing guest progress. The host's behaviour (re-ask at macrotask cadence up to 300 s, `COMMAND_CONTINUATION_WALL_MS` in `handle-turn-drive/🟦️.ts:364`) is by design; the visible stall banner with the Cancel button is the correct escape. A cancel clears the lane but the guest owner still retires forever, so it does not unblock analysis.

Guest Rust, to apply after the build publishes (not edited because the build reads these crates):

1. Find why a decoded `setActiveExample` makes no progress in `advance_manifest_step` (`⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs`). Build 7's component carries no `manifest pending` probe lines, so first add `debug_state("manifest.stage", ...)` there naming the returning branch (see candidate list in `📓️set-active-example-no-progress.md` lines 38-47), then reload and read pattern `manifest.stage`.
2. Retirement of `FailedDecoded` (`semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded` via `retire_failed_step_on_original`): after `close_exchange_owner` returns true (`command-ingress-turn/🦀️.rs:166`), the owner whose producer is `OpBinaryProducer` must release `command`, `bytes`, `input` and `failure`. `close_quote` (`op-binary/🦀️.rs:~38`) returns `Default` demand while `construction.failure_untaken()` is false and nothing else is held, so `owned_is_empty` and `terminal_is_empty` can stay false when `retirement` is non-empty but `step_granted` never gets a paid grant. Add a probe in `retire_failed_step_on_original` printing `owner.terminal_is_empty()`, which of `command/bytes/input/failure/retirement` is still held, and the demand quote, then fix the branch that never consumes it (it must either make progress or return `Failed`). Add a native regression test: a failed decoded command with each held-owner combination retires within a bounded number of turns.
3. Because `broken()` hides which invariant failed, give each raise site a distinct literal (for example "command producer binding does not match the receiving context", "... fault arrived after birth", "... wire page admission refused") so the next browser run names the failure.

## Verification

- Page reload on 127.0.0.1:6087 reproduced the same sequence three times: seq 7 pending, banner at 29 s, then `dispatch-failed ... lost its original command`, then `failed-decoded` forever. Windows (Editor, Model, Results, Projection, Actions, Utilities) mount, results text stays "Analysis queued".
- No `[TRACE] fem3d live_visual reconcile spawn` line was seen in any run; the gates show `reconcile false`.
- Unproven: the exact line that returns `Ok(None)` in the manifest stage (no probe in the published component) and the exact branch that keeps the failed owner non-terminal. Both need the probes in fixes 1 and 2 on the next component build.
- No cargo, wasm or build commands were started.

## Fix: failed command retirement

Root cause is NOT proven statically; the fixes below remove the silent-forever branches that can be read from the code.

### Findings (static)

- Task B: no field of `OriginalSourceBinding` legitimately changes within one actor context. `OriginalActorContext::validate_currency/begin` pin operation, generation, issuer clock binding and absolute deadline across epoch renewals; `borrow_context` re-binds the same payload ledger and clock fn. `native_receiving_identity()` is stable while the decoder receipt exists (None only inside a `receive` scope or after decoder close). The held command-scope lease therefore does not change either value.
- The one producer check that CAN trip after many turns is `closing || (fault.is_some() && phase != BIRTH)` in `advance`: the typed source is shared across lanes (action lanes call `original_typed_command.close_step` at `🚦️actions` 435, `plugin-app-impl` 512/569), which sets `closing` under a pending command; and a bridge that returns Ok but leaves the staged fault set trips the second half on the turn after birth. Both raised the opaque `broken!()`.
- Retirement stall candidates read: (1) `advance_original` answered Pending forever when the failure identity differed from the cx (no progress, no error); (2) `close_original_admission` answers a silent `Pending` when a quoted demand exceeds the REMAINING turn grant but fits the whole grant; the producer quoted close steps with the body of its BIRTH turn (a stale snapshot). Not changed: shared infra, and the real turn grants were not observable.
- Not provable here: which branch of the manifest receiving close chain (refresh/projection/metadata/loan) holds the failed owner at `owner_terminal_empty false`.

### Fixes

- `op-binary`: closing a producer under `advance` now fails with a named Canceled refusal; a staged fault after birth is returned as the app's own fault instead of `broken!()`; `close_step` returns true for a terminal producer before validating; quotes take the body as a parameter (`step_body`, currently the birth body, identical to the previous behaviour).
- `ops_ingress::advance_original` (FailedDecoded): identity mismatch now answers `Refused` with a named invariant (terminal report through the existing refused path) instead of Pending forever.

### Tests (`🎮️command/🎟️typed/🏃️runtime/🦀️.rs`)

- `producer_failed_mid_production_reports_its_fault_and_closes_to_terminal`: passes (fault after 1..4 production turns, then closure reaches `terminal_is_empty`, further `advance` is refused by name).
- `producer_closed_under_a_smaller_body_than_its_birth_still_retires`: passes.
- Existing `producer_cancelled_at_every_phase_...` and `owned_producer_limit_...` fail with "a paid close step must spend its quoted Work" in `semio-plugin-command`; not compared against a pre-change baseline, so unknown whether pre-existing.
- `cargo check -p semio-plugin-runtime-ops` passes; wasm32-wasip2 check of the reactor turn package: see result in the chat summary.

### Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/📥️ingress/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🏃️runtime/🦀️.rs` (tests)
- Script inputs: `🗑️generated/retire-run.sh`, `retire-run2.sh`
- wasm32-wasip2 check of the reactor turn package: Finished without errors.

## Fix: producer advanced after take

### Cause

- Line 62 of `op-binary/🦀️.rs` is the `_` arm of `OpBinaryProducer::advance`, reached with phase READY or TAKEN and `ready()` false.
- The typed source never reaches it with TAKEN: `produce_action` / `produce_runtime` / `produce_host_event` check `command.is_some()||wire.is_some()` first and close the producer; `command_ready()` requires `producer.is_none()`, so `admit_wire` cannot consume the command or wire while a producer lives; a foreign `close_step` sets `closing`, which `advance` refuses by name earlier.
- The real hole is inside the producer: `SEAL=>{stepped(..)?;self.phase=RELEASE}` advanced the phase even when `stepped` answered `Ok(false)` (the turn could not fund or had to yield, so the seal was never applied). RELEASE then retired the bytes and set READY with an UNSEALED input. `ready()` requires `original_is_sealed()`, so it stayed false and the next `advance` fell into the `_` arm. Any yielded or underfunded SEAL turn under the long-held command lease triggers it.

### Fix

- `op-binary`: `SEAL=>{if stepped(..)?{self.phase=RELEASE}}`. READY is now reachable only after the seal settled; TAKEN is only set by `take_into`, which the typed source always closes before anything consumes the command or wire. The `_` arm is unreachable by construction.

### Test

- `runtime/🦀️.rs` `unfunded_turn_at_any_step_never_advances_a_producer_past_its_unsettled_phase`: drives advance and take_into over 64 calls, with call N (0..16) given a zero grant, and also exercises the SEAL quote: before the fix it fails with the line 62 refusal (for example call 17 and 19), after the fix the sealed wire equals the oracle bytes and the producer closes to terminal. `cargo test -p semio-plugin-command --lib`: 14 passed.
- A full typed-source lane test (produce_action + admit_wire + handoff) was not added: it needs a catalog, tool registration and a machine runtime fixture; the producer is the only owner with the faulty transition.
- `cargo check --target wasm32-wasip2` on the reactor turn package: Finished without errors.

### Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🏃️runtime/🦀️.rs` (test)
- Script input: `🗑️generated/take-run.sh`

## Fix: producer close quote

### Cause

- Not caused by today's `op-binary`, `ops_ingress` or typed-runtime edits. The `op-binary` diff is semantically a no-op on the close path: `step_body` returns `self.body` (the same value `close_demands` quotes), `broken!()` only names the line, and the `terminal_is_empty` check before `validate` is unreachable for the non-terminal steps the tests drive. Quote and spend agree.
- The failing assertion in `close_producer` (`🎟️typed/🏃️runtime`) measured the caller's `fuel_remaining()` before and after the turn. The fixture caller sets its fuel to `cx.original_source_fuel_remaining().min(issued_fuel)`, which reads the Source return budget. With the in-flight return-budget renewal (`renewable()` re-bases `spent/work` on each published origin, uncommitted work in `🧵️job`), the turn's origin refresh restores the stock, so a step that did spend (1 Work inside the context, quote 1 item) showed a net caller delta of 0. A first close step after the producer left its lifecycle early (cut 0, ordinary-limit refusal) hit it; steps without a renewal still showed a delta.
- Evidence: per-step trace showed `cx.fuel_remaining()` 749 to 748 inside the step (spent 1) while the caller fuel stayed 758 to 758.

### Fix

- `close_producer` now measures the Work the close step spends inside its own turn (`cx.fuel_remaining()` at entry versus after `close_step`) and asserts it is above zero. This is exactly what "a paid close step must spend its quoted Work" means and is independent of Source-budget renewal. No production code changed.

### Tests

- `semio-plugin-command --lib`: 14 passed, 0 failed (the two previously failing producer tests pass; a foreign, concurrently added `unfunded_turn_at_any_step...` test failed once mid-edit and passed on the final run).
- `semio-plugin-runtime-ops`: 6 passed, 0 failed.
- `cargo check --target wasm32-wasip2` on the reactor turn package: Finished, exit 0.
- Private build dir `C:/b/cmdtest-target` deleted; C: stayed above 27 GB.

### Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🏃️runtime/🦀️.rs` (test helper `close_producer` only)
