# Live cleanup zero-progress at FEM 3D boot

## Verdict
The native law (below) does NOT reproduce the fault on the current tree. No root cause found yet; the cause is not in the native-reachable
part of the live maintenance ladder. Everything native-reachable is cleared; the remaining suspects are wasm/host-only.

## Law added (kept as a regression guard)
- `semio-plugin-testing-laws`: `LiveCleanupFixture` / `LiveCleanupReport`
  (`🧪️testing/🧪️artifact-app-laws/🔁️live-cleanup-zero-progress-law/🦀️.rs`, wired in `🧪️artifact-app-laws/🦀️.rs`, dep `semio-plugin-runtime-instances`).
  It admits a mounted app into the production `PluginRuntime`/`RuntimeAppCell` and pumps the very `plugin_step_live_cleanup` (the pump whose
  `runtime_live_cleanup_nonterminal_status` raises `zero-progress`), under the browser policy (`MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`),
  then retires through `plugin_destroy_app` + `plugin_step_close_cleanup`.
- `editor_live_cleanup` target of `semio-s-artifact-fem-3d-editor` (`✏️editor/🧪️tests/🔁️live-cleanup/🦀️.rs`), runner `lc-law.sh`:
  1. `fem3d_live_cleanup_never_reaches_the_zero_progress_verdict`: boot, 400 pump turns, one AddNode edit, 400 turns, idle counter 0 and
     `maintenance_terminal_is_empty`. PASSES (3.6 s).
  A second experiment (results window mounted, `SpawnJob`/`CancelJob` driven through `RegistryJobDriver`, three superseding edits, live cleanup pumped every
  round) also never faulted over 1800 rounds but is not kept: its close trips an unrelated heap-accounting assertion of the testing caller
  ("the receipted close releases every born root", 21320 vs 21448) when jobs were in flight.

## Findings
- Fault path: `maintenance_ladder_unit_step` ends in `Progress(empty)` when no rung moved and `maintenance_terminal_is_empty()` is false, or
  returns `Progress(empty)` at its first `cx.should_yield()`. Either is a zero-progress step. `mounted_owner_maintenance_step` maps
  `Complete(empty)` to `Progress(empty)` too when the terminal witness is false.
- Natively neither happens: after boot, after an edit and with live solve jobs superseded three times the pump keeps `stalled == 0`.
- Native test harness gotcha (explains the playback agent's "Preparing analysis" forever): the native editor tests never call
  `session::initialize()` (only `🌎️hub/🧩️compositions/🏗️fem/🦀️.rs` does), so the job kind `semio.fem3d.mounted-live-visual` resolves `Missing`
  and the job sits in an admission-failed body forever (`Running(None)`). With `initialize()` the job runs (stages ReserveNodes, Assembly, Pcg).
  That is a test-harness omission, not the browser bug.
- Ruled out: parked-cache wiring (live ladder no longer references `cache_parked`); `world3d`/`recovered_fem3d_backing` recovery (idle at boot natively).
- Not reproducible natively, therefore the real suspects (wasm/host only): (a) the host StepContext clock/deadline: `deadline_exceeded() =
  now_us().is_none_or(..)`, so a clock that reads `None` or a frozen deadline makes every `should_yield()` true and every ladder step an empty
  Progress at 0 us (matches "elapsed 0us", an empty viewport and "nothing works"); recent changes: `⏱️context` (04:47 `semantic_step_limit`
  loan), `🕰️clock-identity-and-budget` (01:44), `⚛️reactor/🔄️turn` (04:16); (b) a document-load / store-replacement / archive-load rung at boot
  (stages 14/24), which the native fixture does not drive; (c) `world3d_snapshot_recovery_close_step` returns `(0,0)` forever for an orphaned
  slot whose pages are not all `None` while `admitted_pages == 0`.

## Next step for the coordinator
Add a `[DEBUG]` line in `runtime_live_cleanup_pump_one` / `maintenance_ladder_unit_step` (cleanup-turns / ladder) printing
`cx.should_yield()`, `cx.fuel_remaining()`, `cx.now_us()`, the last `LAST_MAINTENANCE_STAGE` and `maintenance_terminal_is_empty()` for the first 10
live turns in the wasm build; the first two numbers tell (a) from (b)/(c) immediately.

## Sourcing coordinator findings
- Same fault, same cause as FEM 3D. It is NOT the clock (None/frozen deadline) and not a boot rung: the live pump verdict was count-only (256 empty
  readings) while the uncommitted `drive_turn_retirement` change (entry-lifecycle-and-retirement) now repeats the live pump up to 32 times per
  ingress-free turn whenever a document archive is runnable, so the count is reached in microseconds ("elapsed 0us" quotes the last turn cost).
- The fix is already in the tree (11:53, XLSX ticket `COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/claude-cleanup-pump-stall.md`):
  `runtime_live_cleanup_nonterminal_status` now needs 256 readings AND 8000us of wall time. FEM 3D's browser build must be rebuilt with it; do not
  add another fix. Sourcing verification: `SOURCING-END-TO-END/🧯️boot-zero-progress.md`.
