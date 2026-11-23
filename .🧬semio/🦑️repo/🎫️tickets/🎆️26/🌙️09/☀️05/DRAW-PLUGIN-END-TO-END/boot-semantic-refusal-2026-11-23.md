# Draw guest boot: `native-return-refused: semantic` (2026-11-23)

Status: root cause narrowed to one owner and one wait condition; NOT fixed and NOT proven. No behavior change was shipped. One diagnostic was added so the next boot names the exact blocker.

## What the failure line proves

`[DEBUG] FEM cleanup semantic failure` is the live maintenance verdict `RuntimeCleanupFault::ZeroProgress`: `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` (256) consecutive full maintenance passes returned `Progress(0)` or `Blocked` (`runtime_live_cleanup_nonterminal_status`, cleanup-turns `🦀️.rs`). `AwaitingInput` resets the counter. The log line reads:

- archive op 10 is in `AwaitReplacement`; it only leaves that phase when `poll_artifact_store_replacement` is `Ready`, i.e. the replacement reached `Complete`.
- replacement op 128 is `RetiringCommittedStore` (committed = true): the displaced old Store is being closed. Its quote is `{copy 0, capacity 0, release 0, depth 2}` and is funded.
- `stage=16` is NOT the stalled stage. `LAST_MAINTENANCE_STAGE` is the last stage that executed in a full pass, and stage 16 (`instance_operation_owner`) always runs last after the cursor rotation. Stage 14 (replacement) and 24 (archive) both ran and returned zero progress too.
- The grant is fully sufficient (items 64, depth 4096), so this is not a funding or work-loan starvation: `with_original_work_loan` is not involved, and the ladder visits every stage each pass (no fuel starvation).

## Where the zero progress comes from

`drive_retained_store` -> `ArtifactDocumentStoreDisposer::close_step` -> `ArtifactStore::close_owned_store_step` -> `ArtifactStoreCursorDisposer::close_inner`. The quote `{0,0,0,depth 2}` is the store depth +1 over `close_cursor_demands(...) = structural {depth 1}`. In `close_inner` the only phase that can return a zero receipt for 256 passes is `ReturnedReads` (store engine `🏗️initialization/🏠️owners/.../🏪️store/🦀️.rs`): when the displaced Store's snapshot-read lease registry is not terminal-empty but no lease has been returned (`!has_returned()`), `advance_returned_snapshot_read` answers `Complete(default)` which is mapped to `Progress(0)`. (`PrefixSnapshots` and `TailSnapshot` with a shared Arc return `copied_items: 1`, so they cannot stall.) In other words: the displaced Store is waiting for an unreturned read lease held outside the retirement pump, and this wait is reported as zero progress rather than as an external wait.

Draw-specific holder candidate: `scene_mounted::reconcile` (`✳️any/🧬️schema/🎬️scene/🧵️mounted/🦀️.rs`) takes the store `SnapshotRead` from the pending view and keeps it inside the scene job state (`DocumentVectorJob`/`DocumentVectorRetirement`, later `returned`). It is only returned by `GeometryOwner::close_step`. Draw does not implement `mounted_job_maintenance_phase` (default `None`, unlike FEM 3d/2d which return `Some(Maintenance)` while `maintenance_pending`), so ladder stage 15 never runs the scene cleanup in normal operation; it runs only through `mounted_job_close_step` while `document_blocks_pending_effects()` holds (an archive load or replacement in flight). I could not show statically that this close path always returns the lease, nor rule out another holder (reserved preparation read in `🏷️reserved-jobs`, interaction live query read, window projections).

## Native reproduction attempt

The nearby law is `demo_example_load_settles_through_the_host_document_archive_door` in `editor/🧪️tests/🔬️archive-load/🦀️.rs` (crate `semio-s-artifact-draw-drawing-editor-tests`; run `repo cargo --store=isolated --configuration=prototype -- test -p semio-s-artifact-draw-drawing-editor-tests archive_load_tests -- --nocapture` from the drawing workspace). It FAILS natively before reaching the archive door: `setActiveExample` never publishes `Effect::LoadDocument` (the `published_load` loop runs 100000 iterations with the typed operation still pending). With temporary debug output the maintenance step after round 2 was `Progress(0)` or `AwaitingInput { "cancellation cleanup awaits its original lease custody" }` forever, ladder demand `{copy 1, depth 1}`, no archive or replacement job yet (so this native stall is earlier than, and not identical to, the browser stall; it is the same class: a retained owner waiting on a lease/custody handle that the pump cannot release). Test file is restored; debug output removed. Full native boot of the archive path therefore still needs that earlier stall fixed first (cancellation registry handle `ToolCancellationHandle` strong count != 1, `live_cleanup_awaits_custody`, in `♻️cancellation/🎟️issuer/🦀️.rs`; a typed-operation handle outlives the cleanup it blocks).

## Change made

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs` (`document_load_diagnostic`): each replacement line now also prints `store_close=<close phase witness> leases_empty=<bool>` of the retained (displaced) Store. On the next boot the `[DEBUG] FEM cleanup semantic failure` line will say `ReturnedReads/started=true/active=false leases_empty=false` if the unreturned-lease reading is right, or another phase if not.

## Verification

- The edited crate compiled and the archive-load test ran (it fails as described above, independent of the edit).
- No fix was verified; the Draw guest was not rebuilt (per instruction).

## Recommended next steps (in order)

1. Rebuild Draw, boot once, read `store_close=` in the failure line to confirm the phase.
2. If `ReturnedReads` + `leases_empty=false`: find the holder. Cheapest check: make Draw implement `mounted_job_maintenance_phase` to return `Some(MountedOwnerPhaseV1::Maintenance)` while `scene_mounted` has any shell with `cleanup_pending()` (as FEM does), so finished scene jobs return their lease before a replacement ever commits, and confirm `GeometryOwner::close_step` returns the `returned` lease.
3. Independently of the holder, `ReturnedReads` waiting on an unreturned lease should surface as `AwaitingInput` (external wait) like the close ladder's `ExternalWait`, not `Progress(0)`; implement it in `drive_retained_store` once the holder is understood so a legitimate wait cannot fault the instance.
4. Fix the earlier native stall (cancellation custody) so the archive-load law can run to the replacement and gate step 2 natively.

## Update: fixes implemented (coordinator request)

1. Draw `mounted_job_maintenance_phase` (editor `🖥️play-app` and `👁️viewer`) returns `Some(Maintenance)` while `scene_mounted::maintenance_pending(instance)` holds (new fn: any scene shell of the instance with `cleanup_pending()`, which includes the shell that owns the store read lease). Scene cleanup (`GeometryOwner::close_step`) therefore runs in normal ladder stage 15 and returns leases before any replacement commits.
2. `drive_retained_store` (store replacement, `🗃️document/🧰️store-replacement-initializer-and-retirement`) returns `AwaitingInput { "displaced document store awaits its outstanding read leases" }` when the displaced Store is in the `ReturnedReads` close phase with an outstanding lease and nothing returned yet. New accessors `StoreOwners::snapshot_read_leases_await_return` and its `ArtifactStore` delegate. Real stalls stay visible: the wait only exists while a lease holder is outstanding, the holder is now driven by fix 1, and the `store_close`/`leases_empty` diagnostic remains. Caveat: if a holder never returns, the instance now waits instead of faulting.
3. NOT fixed: `demo_example_load_settles_through_the_host_document_archive_door` still fails at "the example publishes a document load". Findings: it is not fuel exhaustion (turn renewal added to `published_load`, as `settle_archive` does, did not change it). The typed operation `setActiveExample` sits in ladder stage 0 (`drive_worker_step`) with `session_rejected = Some` (pending worker admission), `session = None`; `RefusedWorkerSource::admit_pending` only advances its first four events (binding, cancel alias, params, then waits) and the worker session never becomes available; every pass is `Progress(0)` and the cancellation registry keeps a second handle (`cancellation cleanup awaits its original lease custody`). Grant (items 1, depth 4096, capacity 16 MiB) funds the quote; no deadline or yield. Suspected: the native worker-pool session admission for the reserved route never completes in this fixture (other agents are concurrently changing receiving/worker code and print `[DEBUG] recv refuse` in the same run), so this needs the owner of `job/🚪️session` and worker admission.

## Test results

- Only `archive_load_tests` was runnable; it fails as in item 3. Other editor-tests laws were not run: the crate repeatedly failed to build because of other agents' live edits (pdf model/mut crates, `native-decoding`, `guest-contract`), and it SIGTRAPs in canvas laws per the earlier note.
- Fixes 1 and 2 compiled (the test binary built with them) but are unverified at runtime.

## Changed files (this update)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧵️mounted/🦀️.rs`
- `.../✏️editor/🖥️play-app/🦀️.rs`, `.../👁️viewer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs`
- store engine `🧱️store-components/.../🏗️store-owners-retained-reads-and-retirements/🦀️.rs` and `📥️lifted/.../🪞️delegating-accessors-and-reads/🦀️.rs`
- `.../✏️editor/🧪️tests/🔬️archive-load/🦀️.rs` (turn renewal)
- helper script `retry-draw-editor-tests.sh` (ticket folder)
