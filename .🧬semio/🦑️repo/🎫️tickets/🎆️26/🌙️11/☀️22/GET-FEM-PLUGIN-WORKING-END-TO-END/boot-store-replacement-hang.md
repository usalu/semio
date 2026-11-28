# Boot store-replacement hang (RetiringCommittedStore never leaves)

## Verdict
Two separate causes, one already fixed in the tree but not in the served wasm, one live in native tests and owned by a peer.

### 1. Browser hang (served build 2026-11-26 18:15): displaced store waits on a read lease nobody returns
Re-probed the served build with the console line limit raised (`console-all-probe-long.mjs`, output `🗑️generated/hang/h1.txt`). The full diagnostic at the stall:
`replacement op=128 state=RetiringCommittedStore committed=true ... store_close=ReturnedReads/started=false/active=false leases_empty=false`.
So the committed replacement is complete; the displaced old store sits in the `ReturnedReads` phase because a snapshot read lease taken from it is still outstanding (the live visual job of the old document holds one per render). Nothing told the holder its store was displaced, so it never returned it; stage 14 reads empty forever and the archive stays in `AwaitReplacement`. The grant of 1 item is not the cause (demand is funded=Some(true)).
The 120 s ceiling never fired in that build because the ceiling lived in the archive driver, which the ladder skips while stock < 16 items.

Fix (already in the tree since 18:24, after the 18:15 wasm; the coordinator rebuild at 00:15 includes it):
- `SnapshotReadLeaseRegistry::revoke_authority()` + `ArtifactStore::revoke_snapshot_read_authority()`, called on the displaced store in `commit_store_replacement_candidate` (`P🪜️maintenance-ladder/🦀️.rs`).
- FEM 3D and 2D mounted job step cancels itself when its lease no longer matches the store authority (`✏️editor/🧵️session/💼️mounted-state`), which drops the lease.
- Deadline moved into the ladder (`MaintenanceCursors.archive_wait`, `expire_stalled_document_archive_loads`): 120 s of only waiting readings fault every live load as `plugin.document-load.failed`.
- This pass: that fault now also names every store replacement with its state, `store_close` phase, outstanding and returned read counts (`P🪜️maintenance-ladder/🦀️.rs`), and `ArtifactStoreCursorDisposer::close_phase_witness` additionally prints the active retirement demand (`🏪️store/🏗️initialization/🏠️owners/📥️lifted/🏪️store/🦀️.rs`). New rate-limited `[DEBUG] retained store close` line in `drive_retained_store`.

Law: `editor_live_cleanup::fem3d_replacement_revokes_the_read_leases_of_the_displaced_store` (holder that ignores revocation keeps the load blocked for 200 turns, then returns, load must reach Ready).

### 2. Native hang today: store initializer cannot acknowledge its worker outcome
Not the same as (1), seen when running the law above and reported by the coordinator for `editor_app_2` mounted laws (`remove_selection_*`):
- Test binary built 03:17: replacement `Initializing`, `worker=CheckedOut outcome_pending=true`, initializer cycles forever with the identical step demand (copy 4138, capacity 2384). The peer `[DEBUG] plan ready=true wanted={items 8, copy 3600, work 4097} quantum={items 1, copy 2368, depth 1}` repeats 1100+ times in `🧵️job/⏱️context/🛬️owned`: the original-return escrow scope for the outcome receipt is quoted ready, but the acknowledgement never completes with the 1-item live grant. An earlier binary (01:19) got through initializer, commit and into stage-14 with `store_close=Complete/active=true` (envelope retirement), so the escrow/scope edits made by a peer between 01:12 and 03:17 (`🧵️job/⏱️context/🛬️owned`, `📬️outcome/🪙️budget/📦️stock/🎟️scope`, new `scope-quote` test target) changed behaviour. That peer is actively debugging it (their `[DEBUG] outcome false` / `plan ready` lines are in the tree); I did not touch those files.
- `fem3d_live_cleanup_never_reaches_the_zero_progress_verdict` passes (5 of 5 runs on the 03:17 binary); it failed twice (final_stall 20) on earlier builds and cleared without any change of mine.

## What the next browser run must show
- Stage `[DEBUG] archive/replacement wait` lines show `store_close` leaving `ReturnedReads` within a few readings after the Demo load commits, replacement reaches Complete, archive `Ready`, both windows fill.
- If it still stalls: the new `[DEBUG] retained store close` and the `plugin.document-load.failed` fault after 120 s name `outstanding_reads` and the phase; `outstanding_reads>0` means a lease holder other than the mounted visual job exists (candidates: render projection `🖼️render/📥️staging/🪟️projections`, live interaction query `🕹️ephemeral-interaction`, typed-command request read), which must then be taught the same revocation.

## Open
- Native law (2) needs the peer's escrow-scope fix before `fem3d_replacement_revokes_*` and the `editor_app_2` mounted laws can settle; rerun both then.
- Remove the `[DEBUG]` lines (ladder, initializer, outcome scope) once the browser confirms.

Scripts: `hang-law.sh`, `console-all-probe-long.mjs` in this folder. Private build dir `C:\t\hang` deleted.
