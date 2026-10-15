# Parked Original Cancellation Alias Causal Checkpoint

Source-only successor to actual worker-unit Original73469: that native run compiled24laws,19passed/5failed. No new Original, native compiler, public gate, release gate or lock refresh was started for this checkpoint. The correction and new laws below are staged, not runtime-qualified. Manifests, locks, shared runtime/tooling/SDK Source and all Source limits are unchanged.

## Concrete Primary Failure

The first failing harness law was `worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned`, at retained-ownership437. Its own last admitted session is dropped while `dropped_caller` remains live. The test pumps that parked session using this same initiating caller. Therefore this is an actual primary failure, independent of the later tests trying to drain prior parked slots. The transcript records this failure before the four subsequent failures at lines5224–5240.

The complete source path proves the missing transition:

1. `pump_worker_job_retirements` exclusively claims the existing parked slot and calls its original node pump. The pump calls `worker_job_begin_close` and `worker_job_close_step`.
2. `worker_job_close_step` requires `SESSION_CLOSE`→`SESSION_TRANSITION` CAS before taking the same opaque native authority. Non-close live/checked-out/submitted phases do not enter this authority close.
3. At the Parameters frontier, the lower owner transfers its original `params.cancel` into `CancelTokenRetirement::from_token`. Its original caller still owns the matching root: `Arc::ptr_eq` is true and the root has at least two strong owners.
4. The predecessor lower `close_step` forwarded straight to ordinary parameter retirement. That reaches `CancelTokenRetirement::close_step`, whose `Arc::strong_count > 1` branch returns `SharedAlias` without changing the cursor. With the caller retained, repeating the same pump cannot reach `Arc::try_unwrap`. Yielding or undergranting also leaves the same cursor unchanged. This is a fixed point, not a need for a larger budget.
5. Normal `OriginalWorkerFixtureCaller::close_owner` already resolves this exact case through `return_alias_step`, using the currently borrowed original root as witness. The global parked path omitted this transition.

The predecessor lower file had SHA256 `26777ec23a7ae34fe4f9d55dbea01a74b2319bf302e9c7cffdb7591782678f52` and66352bytes. Read-only inspection recorded that its canonical close body contained no witnessed-alias transition.

The later failures are consistent with residue: the failed fixed-slot law unwinds with additional admitted sessions, whose drop parks their nodes; the later laws either fail to drain prior parked work or fail admission. This causal ordering is observed in73469, but elimination of all four residual failures still requires the actual successor. No independent production flaw is inferred from their occupied-slot symptoms.

## Rightful Lower Transition

Only production body changed: `👷️worker/🎟️authority/🦀️.rs`, `OriginalWorkerJobAuthority::close_step`.

The new branch uses the existing native `has_original_cancel_alias_witness` predicate: close stage3, params already absent, original cursor still containing the exact `Arc::ptr_eq` witness. It checks the same caller's `should_yield`, then calls the existing `return_original_cancel_alias_step` with `cx.original_cancel_token()` and `cx.retained_grant()`. It posts the result through `receive_original_worker_close_result`, which charges only the difference from the same caller's existing receipt and consumes the actual item fuel. No context, cancellation token, stock or authority is minted.

The primitive independently admits one metadata item and depth1. Copy/capacity/release currencies are0. Insufficient item/depth leaves the cursor unchanged; a different root takes the unchanged ordinary retirement path and remains shared/blocked. Returning the exact alias drops only that alias while the borrowed caller root stays alive. The later empty-cursor removal, ledger, native backing, session frame and retirement node remain separate paid frontiers.

The upper CLOSE CAS and original parked-slot claim remain byte-for-byte unchanged. Waker, retained wake, shared frame, unique handle, release/other undergrant guards remain unchanged. Existing non-witness paths retain their original `should_yield` placement; no extra clock sampling was introduced there. The correction does not make any public compatibility API or reverse Cargo edge.

Frozen corrected lower file:67022bytes, SHA256 `d7fe03ac6c7fe4a6ab03b8fe7b0e6cc3496f63d64a05cc050d18de057ec49dcc`.

## Staged Independent Laws

- Closed portable fixture `job-owners/📋️parked-alias.json` and schema `job-owners/🧬️schema/🪢️parked.json`:8cases cover metadata-only exact-root success, item/depth denial, different root, parameters not yet transferred, earlier close stage, checked-out and submitted contention. The authored Bun law compares the codebase-side conjunction against an independent SQLite CASE query, with Ajv closed-schema validation and every receipt axis explicit. This law has not yet run.
- Native law `parked_original_alias_uses_its_initiating_caller_without_slot_residue`, in the existing session-return unit owner, first owns exclusive test slots and requires no prior parked node. It admits a real session with one real Original caller, advances only to the actual witnessed Parameters frontier, drops the session and pumps with that same caller. Item/depth denial must preserve phase, heap0 and receipt0. The first funded metadata-only pump must produce exactly1item/heap0 with `before + actual == after`. Subsequent real paid turns must clear the parked slot and finish the same caller. No native execution of this new law is claimed.
- Existing73469 passed the all7 live-phase/true-terminal fence, combined frame denial cases and mounted14-step receipt/physical HeapWitness law. Their assertions remain unchanged and must continue to pass in the next complete worker-unit successor.

## Read-Only Actual Source Census

The actual repository Source parser was called read-only after the final body edit. All10owners' production and unit closures fit unchanged262144bytes/32files/4000active-lines caps. Relevant exact observations:

| Owner | Production bytes/files/lines | Unit bytes/files/lines |
|---|---:|---:|
| Core |223519 /23 /2367|261367 /29 /2647|
| Worker |123533 /8 /1621|257131 /23 /2603|
| Lower worker authority |67148 /2 /514|100912 /8 /820|

The worker unit has5013bytes of remaining byte allowance. The new absolute fixture/schema paths are at most150UTF16code units, below256. Generated read-only census is `🗑️generated/job-parked-alias-source-census.log`.

Production and test Sources are now stable for Root/Nx scheduling. Actual native RED→GREEN, portable/strict qualification, both-mode lower/public/unit completion,20-workspace lock resolution and final Actor recapture remain pending explicit clearance. No successor will start on the strength of this source-level proof alone.

## Authorized Actual Successor

After protected ordinary prototype and release both passed 1/285, Root authorized the staged portable/strict and native sequence. Actual portable Original24049 passed 25 laws / 2519 assertions; release38691 passed the same cohort. Strict71244 exited0. These runs include all10 actual owner production/unit Source closures and the new Ajv/SQLite8case alias law.

The new `job-worker-parked` private command reuses the genuine repository Cargo launcher and actual Framework worker `--lib --features testkit --locked` target, with the exact full native law name. It is registered in the private Nx project and both editor launch catalogs. It selects one law only for causal isolation; the complete worker unit still runs every law separately.

Isolated49449 reached the real compiler and refused E0382: the new test read its caller after consuming `retire`. The surgical test-only correction now asserts the actual bounded retirement result. That method retains its64turn bound and verifies original roots released equal original roots born. Production was unchanged.

Corrected isolated Original80523 passed in23.0seconds: actual1passed /0failed /24filtered. Its console witness confirms exact metadata1/heap0 alias return, item/depth undergrant0, cumulative before+actual receipt, separate authority/frame/node payments, terminal initiating caller and no parked-slot residue. This supplies the genuine causal runtime successor to73469's missing-alias fixed point. It does not establish complete lower/public/unit or final Actor success.

All own handles were then terminal. Root requested an atomic Framework member/registry publication window for independent Replication lower owners; it is open before the next locked Job call. After that coherent checkpoint, one actual Framework refresh precedes current lower/worker public/unit both modes and the affected20workspace refreshes.

## Full Current Native Cohort

Root kept its pending independent Framework publication staged and authorized immediate current-manifest qualification. The same Sources then produced these actual results:

| Gate | Prototype | Release |
|---|---|---|
| Lower authority, all private laws | 51228: 10 / 10, 25.9 seconds | 72006: 10 / 10, 28.8 seconds |
| Worker complete unit | 32074: 25 / 25, 46.6 seconds | 47376: 25 / 25, approximately 61 seconds |
| Worker public contracts, five exact targets | 75613: 8 / 8, 24.3 seconds | 24935: pre-runtime ENOSPC, 29.9 seconds |

Both complete worker harnesses ran with zero ignored and zero filtered tests. The previously failing fixed-slot retirement law and all four later occupied-slot failures now pass. This qualifies the missing witnessed transition causally through both full unit modes, with the original live-phase/wake/shared/frame/undergrant protections retained.

The five public targets are `original-native-preparation` (1 law), `original-normal-failure` (1), `original-payload` (1), `original-staged-metadata` (1), and `original-worker-turn` (4). Prototype passed all eight. Release hit genuine `LLVM ERROR: IO failure on output stream: No space left on device`, also reported by `rust-objcopy`, while writing/linking the staged-metadata binary inside the fixed Rust image. It did not reach public harness execution. This is a physical capacity frontier, not a public semantic failure or a compiler API error, and public release remains unqualified.

All handles are terminal. No artifact was deleted, no image or Source cap was raised, no lease namespace or Original global wiring changed, and no blind release retry was started. Root and Storage received the exact terminal failure and the open Framework metadata window. The remaining public release, workspace lock/fetch sequence and Actor recapture await authenticated Rust-image headroom and coordinated metadata publication.
