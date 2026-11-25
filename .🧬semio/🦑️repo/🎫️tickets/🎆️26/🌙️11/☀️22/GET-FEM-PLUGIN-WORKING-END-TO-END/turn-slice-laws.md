# Turn slice, typed-operation spin and reconcile spin

Script: `turn-slice-laws.sh` (native, own target dir `C:\t\law2`, log `🗑️generated/turn-slice-laws.log`), retried by `turn-slice-retry.sh`.
Runs: reactor-turn laws (`quiet_`), publication law (`ack_wait`), patches law (`reported_output_fault`) and `cargo check` of
instance-app, instance-jobs and runtime-ops.

## Verified

First change set (quiet slice): `semio-plugin-reactor-turn` `quiet_`: 2 passed. With `QUIET_TURN_SLICE_US` 0: 2 failed
(`left: 10048 right: 63`). Constant restored.

## Status (native, 2026-11-25 ~12:20)

- `semio-plugin-reactor-turn` (`-- starved quiet_`): 4 passed (2 quiet-slice, 2 arena-starved). This build includes the poll-kernel,
  command-ingress and cold-pair loop edits.
- `semio-plugin-reactor-patches` (`-- reported_output_fault`): 1 passed.
- `cargo check` of instance-app, instance-jobs, runtime-ops (typed-operation trace, `has_runnable_tool_operation`, diagnostic format): ok.
- `semio-plugin-composition-publication` lib tests do not compile: its sibling `🧪️tests/🧩️private-child-metadata` calls
  `advance(...)` with a stale argument list (3 E0308). The `ack_wait` law (`an_operation_waiting_for_the_host_acknowledgement_owes_the_guest_no_step`)
  is therefore written and the crate itself type-checks, but the law has not run.
- Another agent's temporary `debug-ledger.rs` `#[path]` lines blocked every build for a while; stubbed by the coordinator.

## Spin 1: FEM 2D click, `["typed_operation"]` every turn, no progress (> 6 400 turns)

Cause (by reading, not reproduced): `has_runnable_typed_operations` answered `true` for `!tool_operations.is_empty()`. An
operation in `AwaitingAck` whose result page is already out has no step the guest can take
(`next_advanceable_typed_operation` skips it, the unit is `Idle`), only the host's acknowledgement event moves it, yet
it kept the reactor in more-work for as long as the ack was missing: each poll spends 10-11 work units and copies nothing.
The B16 comment in `next_advanceable_typed_operation` documents this as intended; it is the unbounded form of it.

Fix: `MountedTypedCommandFullOperationStage::owes_guest_step(result_page_presented)` (new module
`🧩️composition/📬️publication/⏳️ack-wait/` with fixture `🔣️.json` and law
`an_operation_waiting_for_the_host_acknowledgement_owes_the_guest_no_step`); `has_runnable_tool_operation()` in
`🧵️instance/👷️jobs/📥️staging/🧾️typed-operations`; `has_runnable_typed_operations` uses it. An `AwaitingAck` operation
with an unpresented page still counts as runnable.

Diagnostics: `plugin_continue_typed_operations` prints `[TRACE] typed-operation silent units=N instance=I <diagnostic>` at every
power of two from 64 consecutive units that returned nothing. `typed_operation_diagnostic` now names, per operation, verb,
stage and `page=<present>/<presented>`.

If the click still spins after rebuild, read that line: it names the operation and whether its page was ever presented. A
page that is present but never presented means the turn grant refuses its presentation demand (`plugin_turn_admitted`),
which is a second, separate bug (a permanent refusal that should fault).

The root cause of the missing ack (host side or the page never being presented) is not established.

## Spin 2: FEM 3D `addMaterial`, `["reconcile"]` and `["reconcile","ui_retirement"]` for 14 000 turns

Cause (by reading): a retained output fault on the patch tracker (`state.output_fault`) is never cleared until its instance
closes, and `has_drivable_work`, `has_publishable_work` and `has_work` answered `true` for as long as it stays, although the
reactor reports it to the shell once (`take_render_fault` sets `reported`). `drive_one` returns at once while it is set and
`close_step` refuses to run, so after the first fault the tracker does nothing and the reactor spins on it: no patch, no
effect, and no surface (the Artifact tree) updates again. That matches "the material never appears".

Fix: `output_fault_unreported(state)`; the three predicates arm only for an unreported fault. Law
`a_reported_output_fault_stops_arming_the_reconcile_drive` in `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🚨️reported-output-fault`.

The fault itself is not fixed (it is still sticky until close, as before). Its text is shown once as a shell fault
`ui.surface-render` and in every more-work trace line as `output_fault=<text>` inside the `patches=[...]` field
(`PatchTracker::debug_state`). Please read that text: it names the surface and the failing step
(`receive_job_into`, `try_reserve` or the output close), which is the real defect behind the missing material. The
`ui_retirement` part is not explained by this and is unverified.

## Spin 3: FEM 3D render and remount loop (artifact panel rev 1 704, window slots g1775-1777)

Measured by the coordinator: `output_fault=none` in every trace, so spin 2's fault hypothesis does not explain it (the arming fix is
kept, it is correct). The three surfaces that churn (artifact panel, `fem3d-model`, `fem3d-results`) are exactly the ones the
arena-starved redirty can touch, and `ui_retirement` turns are the retirement of the previous generation of each.

Cause (by reading, not reproduced): `redirty_arena_starved_surfaces` re-dirties a surface "as soon as the arena can price the rows
it is missing". The surface's own retirement returns the credit that triggers the next retry, and a surface whose rows exceed what
the arena can ever hold comes back short by the same rows every time: render, retire, credit returns, redirty, render, ... The loop
never converges, each pass mints a new slot generation (windows) or revision (panel) and arms `reconcile`.

Fix: `ArenaStarvedSurfaces` remembers the rows a surface was re-dirtied at; a retry that is short by as many or more rows is
forgotten instead of retried again, a retry that gains rows is followed by another, a complete render forgets the surface.
Laws (`⚛️reactor/🔄️turn/🎯️dirty-poll-surfaces`): `a_starved_surface_whose_retry_gains_nothing_is_not_redirtied_again`,
`a_starved_surface_whose_retry_gains_rows_is_retried_until_complete`.
Trace: `[TRACE] reactor arena-starved surfaces re-dirtied taken=N rows=R` on every redirty.

Not done: the requested law that rendering the same 3D snapshot twice yields an identical artifact panel and keeps the window slot
generations. It needs the FEM 3D editor and a native render harness, which is not available here (the keyed fixture harness does
not run, see the first report). If this loop persists after the rebuild (the trace line repeats with the same `taken` and `rows`, or
slot generations still climb without it), the next suspects are a changing render output (iteration order, counter, key) or a window
identity that changes per render; compare two successive revisions of `framework.panel.artifact` from the wire.
The arena itself is also worth a look: a 3D artifact tree that needs more rows than the arena holds renders truncated.

## `document_load` (2D 3 882, 3D 4 922 boot turns)

- Restored loads (`step_restored_document_loads`): looped inside the slice (previous change).
- Cold pair loads (`step_cold_pair_document_load`, driven from `drive_cold_pair_ingress`): one archive poll per turn, answering
  `Loading`. Now polls again within the same turn while the status stays `Loading` and the quiet slice is open. This is the
  likely source of the remaining turns. The host still hands one page per turn; pages already received are decoded in
  the guest, so only the decode polls were batched.

## Earlier changes (see the first report)

- Typed-operation outer loop no longer stops on `!advanced`; quiet loops share `quiet_slice_open`/`QUIET_TURN_SLICE_US`
  (`🔄️turn/⏱️quiet-slice-turn`).

## Rust changed, rebuild wasm

`⚛️reactor/🔄️turn/{🌀️poll-kernel-turn,⌨️command-ingress-turn,🧊️cold-pair-loads-and-process-pool,⏱️quiet-slice-turn}`,
`🏃️runtime/🧰️ops/🧾️typed-operations`, `🧵️instance/{🔌️plugin-app-impl,👷️jobs/📥️staging/🧾️typed-operations}`,
`🧩️composition/📬️publication`, `⚛️reactor/🩹️patches/🏃️tracker-drive-and-work-predicates`. Also `🌱️`-free: `⚛️reactor/🔄️turn/🎯️dirty-poll-surfaces`. All compile and pass or check as listed in Status, except the `ack_wait` law.
