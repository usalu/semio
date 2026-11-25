# L27 Status (2026-11-23 04:25)

## Resumed after restart
- On 2026-11-22 around 17:30, Claude Code's low-memory reaper stopped the sequential batch runner, the lasso rerun and the activation watcher. The 3D batch had compiled 909 crates with no errors when it was stopped.
- `/home` was at 442M free. This ticket's finished-lane logs (`l1a-*`, `l1b-*`, `l1c-*`, `l1d-*`, `l10-*`, `l11-*`, `l12-3d-editor-lib.log`) and the old `l12-spill` were deleted. `🗑️generated` is now about 100M.
- `l12-run.sh`: `L12_SPILL` can be overridden. `l23-seq.sh` puts the spill in `/dev/shm/l23-spill` (RAM) so it no longer writes to `/home`. Batch order: 3d → 2d (the 30 remaining packages, one cargo invocation) → 5d → hub → lasso rerun (`l25` fix). Logs: `🗑️generated/l23-<tag>.log`, milestones in `🗑️generated/l23-seq.log`.

## Fixes awaiting verification by these batches
- 3D: `l17a-3d-lifecycle.md` (ordered-map retirement in the fixture), `l17b-3d-commands.md` (receipts grant, import wire budget, topology counter, window generation, sqlite scale), `l24-3d-framework-close.md` (`TimeTravelDiscardCursor::native_header_release`, window-config birth in `ensure_window`). The mutation-latency analysis in `l21-3d-mutation-latency.md` still needs an instrumented run.
- 5D/hub: `l19-5d-hub-migration.md` (`ColdMutation` and `close_original_effect_step` call sites).
- 2D: `l25-lasso.md` (the test now sets lasso selection).

## Infrastructure fixes
- `l22-broken-imports.md`: corrected the import depth in the ui-contract companion `📜️script.ts`.
- `l26-graph-observation-budget.md`: `maximumAggregateNodes` raised to the output-bytes cap. The old graph service (pid 526318) has exited, so the next service loads the fix.

## Blocked
- Editor activation (`l4-activate.sh 3d|5d|2d`) and the time-travel runs (`--locales en`, `--locales de`) need 12G or more free on `/home`. At 04:25 it had about 1.9G. The largest idle consumer is `🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/rust-build` (119G, no writes since 2026-11-21), followed by `🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/w/b` (27G). Both belong to other tickets, and deleting them was denied to this session.
- Resolved: the store-close question (`l18-store-close-audit.md`). The targets were already removed.

## Native batch results (2026-11-23)
- Board-host target (`semio-os-infinite-board-normal-host`): 53 passed, 1 failed. The lasso fix (`l25`) is confirmed. The new failure `installed_naming_owns_brush_ids_and_fill_labels` (board fill admission re-entry) is fixed in source in `l28-board-fill-admission.md`; a rerun is queued (`l23-lasso2.log`).
- 3D: try 1 hit ENOSPC in the 4g tmpfs (only 2 crates were spilled). Try 2 (5g, all 36 `semio-s-artifact-puzzle-3d*` crates spilled to `/dev/shm`) compiled 1286 crates, then hit E0432 `app::ArtifactColdClose` in `semio-plugin`. That came from another agent's concurrent edit of `🔌️plugin/♻️lifecycle` at 09:05–09:07. A rerun is queued last (`3d-r3`).
- 5D: ENOSPC at about 1100 crates even with 5g. The 2D batch failed the same way with 4g at about 1379 crates. Cause: the l12 flags (opt-level 1, debug assertions, strip, `embed-metadata=no`) make the overlay lower cache (`DEV-PUZZLE-5D…/native-target/b`) cold, so the whole dependency closure is rebuilt in RAM. The machine has about 7G of RAM free and swap is nearly full, so a larger tmpfs is not safe.
- Runner fixes: `L12_SIZE` and `L12_SPILL` can be overridden. The request loop reads `l12-req-<dim>.cur` beside the request file instead of `/tmp`, so `quit` works when the tmpfs is full.

## Remaining blocker
The 5D and full 2D native test closures, the editor activations and the en/de time-travel runs all need disk on `/home`. About 15–20G would let the build dir live on disk instead of in RAM. `/home` is at about 6G, and the idle 119G `REMODEL-PLUGIN-END-TO-END/🗑️generated/rust-build` (other ticket, last written 2026-11-21) is the obvious candidate. Deleting it was denied to this session.

## 3D results and follow-up (2026-11-23 afternoon)
- Partial 3D run 3d-r3: `l29-3d-results.md`. The low-memory reaper stopped it during `editor-gestures`. Nothing reruns on its own; a restart is awaiting the user.
- `l30-3d-import-clipboard.md`: import wire budget producer restored, stale `include_str!` paths in the source-string laws fixed, paste assertion aligned with the reserved clipboard job, reserved-mount ownership uses `matches_retirement`, settlement test closes its result page. All unverified.
- `l31-3d-close-ceiling.md`: the three interactive-ceiling tests failed in teardown, fixed via `forfeit_unissued` and `construct_boxed` (unverified). Open: `closes_mid_reserved_commit` (a close step under-reports its release; the owner needs a symbol build to be named), `conflict_history_edits_end_to_end` (release axis probably double-counted, 470 MB spent against 65 MB retained), and mutation latency (`l21`).
- Board-host: the clean rerun is pending. The last failure came from another agent's `[DEBUG] big alloc` probe, not from the fill fix (`l28`).

## 2026-11-24 restart
- `l32-verify.sh` runs the 3D batch (budgets, defects, gestures, panels, surface, …), then the clean board-host target, with the 5g tmpfs and `/dev/shm` spill. Logs: `🗑️generated/l32-<tag>.log` and `l32-seq.log`.
- `l34-plain.sh` starts after it. It uses `L12_PLAIN=1`: no opt-level/strip/`embed-metadata` overrides, so the flags match the lower cache (`DEV-PUZZLE-5D…/native-verification.sh`) and unchanged crates come from the lower instead of rebuilding in RAM. That is the route around the `/home` disk blocker for 5D, hub and 2D. It first runs `editor_budgets context::` with `RUST_BACKTRACE=1` and symbols, to name the owner behind `closes_mid_reserved_commit`.
- `l33-flag-locale.md`: the inspector flag values in 2D, 3D and 5D are localized (yes/no instead of raw `true`/`false`), and the time-travel gap and roster checks are fixed for de.
- 04:03: the full 3D run is in `l32-3d-results.md`: 42 distinct failures, down from about 80 lines in l29. The close-custody and import-budget fixes are confirmed. Source lanes: `l35` (open_vortex fixture, a stock per dispatch), `l36` (clipboard ingress close, topology counter; the mutation latency plan needs a `[DEBUG]` histogram), `l37` (3D/5D migrated to `original_tool_run()`, accept closes the menu, keyed reserved leases with queued-commit slots and latest-wins permit reuse, `windowId` fixture, `[DEBUG] tool-run gate` probe), `l38` (tool-run provisional `project_owned`, `native_retirement_reply` for unordered maps and sets, 3D window Pack receiver, task-resume decoder binding and cancel retire).
- `l34-3d-close.txt` is now the full 3D step set, so L34 re-verifies everything (plain flags, symbols, `RUST_BACKTRACE=1`) before 5D, hub and 2D.
- Separate follow-up: the owned `ToolRunView`/`with_tool_run` path has no production binder but is still used by the WFC, generation, remodeling and energy editors. Removing it needs its own lane.
- Board-host clean rerun (L32 lasso): 54/54 ok. `l25` lasso and `l28` fill admission are confirmed.
- 04:35: the L34 3D step failed on `levels::CHIP_TINT_PERCENT`. Another agent added `chipTintPercent` to the styling `🔣️.json` and projection but did not regenerate. `repo run @semio-tech/ui-styling-tokens:generate` fails in the dependency installer (EACCES on a patch), so `l39-styling-generate.ts` (bun) regenerated `🔤️tokens/🦀️.rs` and the other artifacts. The full 3D rerun is queued after L34 (`l40-3d-rerun.sh`).
- L41 (plain flags, wide spill): 3D partial, see `l41-3d-results.md`. The low-memory reaper stopped it during panels; 5D, hub and 2D did not run. Not restarted. New source fixes since: `l42`, `l43`, `l44`.
- L46 (2026-11-24): fill-run fix in source (tool-run provisional two-step publication, transient publication, ArtifactView tool_run_poll), see `l46-3d-panels-fill.md`. Unverified; picked up by L45 if compiled after the edit.
- L48 (2026-11-24): activation/time-travel blocked by disk (4.4G free, need 15–20G); engine wasm pkg stale (pre-l33). Candidates in `l48-activation-timetravel-prep.md` need owner approval to delete.
- L47 (2026-11-24): backbone close fix (Arc::into_inner in store backbone retirement, new pair test), action-producer test births its window, digest test caller fixed. See `l47-backbone-settlement.md`. Unverified.
- L45 3D (2026-11-25 00:13): 0 results. The build retry compiled, but rustc for the editor-budgets and editor-manifest test crates was SIGKILLed (out of memory: 5g tmpfs + spill in RAM, plus other agents). One combined step meant every 3D target was lost. L50 (`l50-after.sh`) reruns 3D one target per step with `L12_JOBS=1` after L41 (5D, hub, 2D) ends, then `l49-framework.txt`.
- L51 (2026-11-25 00:31): 5D under L41 also lost two stdio dwg rustc processes to SIGKILL. I stopped L41 and L50; `l51-serial.sh` reruns everything with one build job (3D one target per step, 5D, hub, 2D, then the framework). Logs: `🗑️generated/l51-*.log`.
- L52 (2026-11-25): action-producer WorkLimit regression came from MIT-BESTAND's `maximum=fuel-1` cap in `with_original_effect_at`. Reverted, and a note left in their ticket. See `l52-action-producer-worklimit.md`. Unverified.
- L51 3D budgets: 73 passed, 0 failed (`set_active_example` passes now).
- L51 3D defects: 53 passed, 0 failed.
- L51 3D gestures: 48 passed, 1 failed. Clipboard slot fixed (l42 confirmed). Mutation latency still at 132 turns (store 1474 of 1599 units).
- L53 (2026-11-25): store-run turn-loan batching, last-item inverse reservation, chained item close, and a `[DEBUG]` pump histogram. See `l53-mutation-latency-batching.md`. Unverified; the gestures mutation_latency rerun is queued in l49.
- L51 3D manifest: 63 passed, 0 failed (l44 confirmed).
