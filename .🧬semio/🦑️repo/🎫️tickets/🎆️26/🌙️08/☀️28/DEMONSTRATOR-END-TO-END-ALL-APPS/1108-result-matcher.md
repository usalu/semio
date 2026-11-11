# Lane matcher: record matcher KeyAcknowledgement zero step

## Diagnosis (W-matcherack-5 and -13, allocation-free probe on fem 3d)
`advance_nested` at selection KeyAcknowledgement (grant admits the demand) completes the acknowledgement as `Ready{work:0}` (`claim_child_work` is the UNREPORTED delta; the work was reported or retained earlier) and maps it to `idle(0)` = zero progress, zero work. The snapshot cursor stall check (`io/control/snapshot/rs`: `progress==default && turn.work==0`) refuses that. The next selection after that step is KeySettlement (a Local action), so it cannot be run in the same nested call.
Probe caveat: probes that allocate (`format!`/`eprintln!`) perturb the heap-receipt assert at `original-receiving-tests.rs:27` (+64..271 B) and in one run hid the stall (W-matcherack-11). Use allocation-free probes (stack buffer + `stderr().write_all`).

## Changes (all [DEBUG] probes removed; drive.rs grep clean)
- `record/schema/drive.rs`: KeyAcknowledgement arm calls new `advance_acknowledgement`: polls the acknowledgement inside one call while it reports `Pending{work>0}` (summing), returns `Pending{total}` on `Pending{0}` (state kept), `idle(total+ready_work)` on `Ready`. This merges a Pending(work)+Ready(0) pair into one charged step, but does NOT cover a first-poll `Ready{0}`.
- `record/schema/work/tests/rs` (`record_matcher_driver`): nested turns now run under an exact quote: grant = `demands()` (items 1, copy/capacity/release/depth) and work ceiling = `nested_work_demand()`; an admitted turn starting at KeyAcknowledgement must report work>0 or progress. (The exact-work-ceiling edit is the last edit and has not been run yet.)

## Verified
- native-owned lib + 16 targets: W-matcherack-8 rc0 (17 "test result: ok"), taken with the first version of the fix.
- wasm32-wasip2 native-owned lib: W-matcherack-9 rc0.
- record_matcher_driver exact-grant law: W-matcherack-6 passes with the fix; the owned fixture acknowledges with work in one call, so it does not yet reproduce the fem zero step.

## Round 2 (coordinator rulings: exact acknowledgement quote, transitions are progress, no literal bound) - implemented, verification queued
- `decode/rs`: `NativeDecodeControl::record_transition()` / `transitions()` (thread-local epoch, allocation-free).
- `record/schema/drive.rs`: the acknowledgement quote and the step share one predicate, `acknowledgement_started()`: quote = 1 when the acknowledgement starts (its resident producer performs exactly one unit), 0 when only settlement remains. `advance_acknowledgement` keeps driving while a poll changes that predicate (state moved), returns `Pending{total}` when a `Pending{0}` changes nothing, and on `Ready` demands `total == quote` (invariant error otherwise); a zero total records one transition.
- `io/control/snapshot/rs` `drive_cursor`: no bound literal. A turn is a stall only when progress, work and the transition epoch are all unchanged (`SnapshotAdmission::transitions`; decode = epoch, encode = 0).
- Law (`record_matcher_driver`): actual work == exact quote at KeyAcknowledgement; zero quote records exactly one transition and changes selection; charged acknowledgement records none; denied (starved) acknowledgement records none.
- Source changed after queuing W-matcherack-14 (fem 3d) and -15 (native-owned lib + 16 targets); neither had started compiling (0 `Compiling` lines), so they build the final tree. wasm: W-matcherack-16 (native-owned + os-io lib). Results: `$G/W-matcherack-{14,15,16}.rc/.txt`.

STATUS: BLOCKED verification pending: final tree implemented (predicate-exact quote, epoch progress, no literal), W-matcherack-14/15/16 queued behind the host capacity queue
