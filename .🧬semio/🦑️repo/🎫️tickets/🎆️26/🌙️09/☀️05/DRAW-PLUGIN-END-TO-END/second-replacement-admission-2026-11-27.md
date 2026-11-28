# Second replacement's refused-worker admission makes zero progress (Demo -> Blank), 2026-11-27

Status: NOT root-caused, NOT fixed. The native law is written but has not run yet (host capacity and peer build breakage; see "Blockers").

## What the guest log proves
- `session_rejected` is not a refusal. `RefusedWorkerSource::pending(job, op)` is the normal birth state of every store-replacement initializer (`store-replacement-admission-and-member-drive` line 33). `admission_pending()` is true until the session is admitted, so "starts from a refused worker session" is the wrong frame: the replacement is stuck in its ordinary first admission.
- Phases `("authority","source")` mean: `mounted` is at stage 1 (`authority`), `batch` is untouched (`source`, stage 0). Stage 1 calls `OriginalWorkerJobAdmission::advance`, which is the only place that can return zero progress here once `prepare_pending_params` passed (binding, cancel alias, params all post progress).
- The grant is NOT the problem: capacity 16777204, copy about 1 MiB, items 13 to 61, depth 64, fuel about 50M, `should_yield=false`, not cancelled. `admission_demands()` (birth capacity only) is satisfied. This is not the clamp fixed by the FEM peer (`quote_floor`); the grant is fully funded.
- The ladder quote is zero (`step_demand=RetirementDemand{0}`) because the initializer has no session yet; that is expected.

## Where a funded turn can still return zero (static)
Inside `OriginalWorkerJobAdmission::advance` -> `prepared.admit_step` -> `OriginalJobReturnOrigin::admit` -> `OriginalJobReturnReceiver::prepare` (`🧵️job/📬️outcome/⏸️continuation/🎟️origin`):
1. `cx.admit_original_return_origin()` returns false, with no progress posted, when the Source's return budget has a table but `admit_return_origin` finds `origin().source_turn != cx.original_source_turn_identity()` (`🧵️job/⏱️context/🎟️source` line 73), or when the table is absent and only `birth_requested` was set (needs `OriginalJobReturnBirth::service` before the business turn).
2. `claim_original_sized_return` answers false via `claim_why` (`table_full_nothing_reclaimable`, `table_reserve_none`, `stock_*_below`, ...). The 32-row escrow table is per Source; the first replacement's displaced-store retirement may still hold rows (`table_full_nothing_reclaimable` would be permanent while it retires).
3. `RetainedPayloadBuilder` page admission for the fault payload (`JOB_PAYLOAD_PROCESS_OWNED_BYTES` ceiling 64 MiB; unlikely).

Hypotheses 1 and 2 both fit "first replacement fine, second never". Hypothesis 1 is the same class as the 01:13 FEM fix (`PostProducer::validate_context` comparing a per-turn dispatch counter instead of the issuer): a resident owner compares a per-turn `OriginalSourceTurnIdentity` and never matches after the first turn changes. Neither is proven.

## Temporary instrumentation (kept, `[DEBUG]` prefixed, remove with the peers' lines)
`store-replacement-initializer-and-retirement/🦀️.rs` `drive_initializer`: the peers' `[DEBUG] store initializer admission zero` line now also prints `claim={cx.last_claim_reason()}` (the existing `claim_why` thread-local), which names hypothesis 2 directly. The peers' `dbg_prepare` in the origin file already prints the `admit_origin` flag (hypothesis 1).

## Native law added
`✏️s/🔌️plugins/🖍️draw/.../✏️editor/🧪️tests/🔬️archive-load/🦀️.rs`: `demo_then_blank_example_load_settles_two_consecutive_replacements` (Demo, settle, Blank, settle; asserts `Ready` twice, snapshot id `blank`, no layers, terminal close). It reuses `published_load` / `settle_archive` (20 000 rounds bound) so a stalled second admission fails with "archive load reaches a terminal state" instead of hanging. NOT yet executed.

## Unrelated build break repaired on the way (peer-introduced)
A peer added `impl Drop for SnapshotReadLease` (23:49), which made `store/read/retirement/🦀️.rs` line 49 fail with E0509 (move out of a Drop type). Fixed with the exact existing owner API: `read.return_original_lease()` instead of `lease.registry` moved out of the lease. File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/♻️retirement/🦀️.rs`. Re-read before touching; if the peer already fixed it differently, keep theirs.

## Blockers hit (for the coordinator)
- Capacity: `repo capacity run --memory=3500` waited 10 to 25 minutes per attempt behind peer `graph-plan` (3 to 6 GB), build (5.3 GB) and 2600 MB commands; compressor occupancy about 70k to 219k pages (1.1 to 3.4 GB), swap 0.9 to 1.1 GB, pressure normal.
- Disk: one attempt failed with `No space left on device` (rlib archive); 45 GiB free afterwards.
- Warden: two attempts cancelled (SIGTERM from `repository:command`); one run failed because the final link of `semio_s_artifact_draw_drawing_editor_tests` peaked at 4136 MB, above the 4096 MB ceiling, "not on the carving list". This test crate sits at the ceiling on its own; the coordinator should carve it or rerun when calm.
- Peer breakage: the `host-capacity/gate` schema rejected `/format` for one attempt (a peer mid-edit); transient.
- Logs: `🗑️generated/sr-native-1..7.log`, `sr-run.sh` (delete when done).

## Next step once a native run completes
1. If the new law hangs: read the `claim=` and `admit_origin` fields of the `[DEBUG]` line; fix the named owner (hypothesis 1: compare the issuer, not the dispatch; hypothesis 2: reclaim or lend rows to the second operation before its admission, or retire the displaced store's rows first).
2. If the law passes natively, the stall is wasm- or host-specific (clock or turn identity under jco): then add the same law at the framework level (`🎟️scope-quote` style) with a second Source turn.

## Files changed
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️archive-load/🦀️.rs` (new law)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs` (`[DEBUG]` claim reason)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/♻️retirement/🦀️.rs` (E0509 repair)
