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
