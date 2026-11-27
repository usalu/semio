# EN 1990 physical replay, 2026-11-26

## Result

The test target now compiles and reaches the test body (28 min cold build, one-worker, 2,663 MiB grant). The single test `en1990_public_factory_pays_nested_projection_and_semantics` FAILS at operation 0, no cancellation, loop turn 3402:

- Replay demand `RetirementDemand { copy_bytes: 4832, capacity_bytes: 2800, release_bytes: 0, depth: 1 }` exceeds the fixture grant `maximum_copy_bytes: 4096` (fixture `physical-public-factory.v1`, `copyBytes: 4096`, `bodyBytes: 4096`).
- A physical heap mismatch precedes it: `Unpaid { turn: 6820, actual: (11973 requested, 8653 released), reported: copied 400, retained capacity 328, released 0 }`. The probe turn counter is about 2 per loop turn, so this is the funded advance of loop turn 3401. A step allocated (and mostly freed) far more than it reported.

The Bun/SQLite oracle, schema check and 73-byte heap witness pass.

## Root causes found

1. Wrong working directory. `semio-s-artifact-norm-en1990-mutations` belongs to the nested EN 1990 workspace, not the root workspace. The route in `retained-publication-cargo-2026-11-26.md` fails with "package ID specification did not match any packages" unless launched from `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990`. The MCP `capacity_run` and `cargo` tools have no working-directory argument, so they cannot reach nested workspaces; use the `repo` CLI from that directory. Adding a `--directory` flag needs the shared flag schema (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🖼️assets/🏗️build-surface.json`) plus executor and CLI changes, so I did not do it here.
2. Quote/grant mismatch and under-reported heap in the replay preparation chain (`Stage::Projection`/`Edit`/`Share` quotes in `NativeSemanticPreparation::demands`, cursor of the derived `RetainedClone` for `En1990Snapshot`). Which stage produced 4832/2800 is not yet identified.

## Change made

`🧪️tests/🧮️physical/🦀️.rs`: the demand-exceeds-grant assertion now also prints `last_advance` (turn, quote, step, work, next quote, receiving limits) so the offending stage is visible. Not yet executed.

## Not done

- No run after the diagnostic edit completed. Reruns were cancelled by the host warden (critical memory, swap at 8.7 of 9.2 GiB) at about 90 compile units, three times, or never admitted. The coordinator then paused heavy runs for the Base publication.
- Someone cleaned `🗑️generated` logs mid-session; later logs were written to the session scratchpad.
- Next step: one admitted run, read `last_advance`, then either (a) fix the stage's quote/report so copy stays within `body` plus overhead and heap reporting matches physical allocation, or (b) size the fixture grant from the framework's quoted maximum if the quote is correct and only the fixture is too small. The mismatch (11973 vs 328) says (a) is needed at least for the clone step.

Retry helper: `en1990-replay-retry.sh` (loops on warden cancellation).

## Static trace during the Base pause

- The test uses `RetainedColdMutationApplyFactory` (ClonedProjection plus `ColdReplayEdit`). Stage order in `NativeSemanticPreparation`: Projection (about 3,400 cursor turns), TakeProjection, EditBirth, PostCapture, Edit, Share.
- The oracle (Bun/Ajv/SQLite) checks only the policy schema and the semantic result. The schema sets no upper bound on `copyBytes`, so the 4,096 grant is a fixture knob and the oracle cannot say which side is wrong.
- `ColdReplayEdit::demands` quotes `copy = size_of::<S>()` and `capacity = size_of::<M>()`; `Stage::Edit` uses it once the clone is complete, which fits "turn 3402 after 3,400 clone turns". The `2800` capacity is therefore most likely `size_of::<En1990Mutation>()`, and `4832` is a whole-struct move quote, not a splittable page. That would make the quote truthful and the 4,096 fixture grant too small for a one-shot in-place edit.
- Contradiction not resolved statically: `PostCapture` quotes `2*size_of::<Option<S>>()+...` and would then exceed 4,096 too, yet the loop reached turn 3402. So the exact stage is unconfirmed.
- The heap mismatch (actual 11,973 requested / 8,653 released vs reported 400 copied / 328 retained) sits at the funded step right before Edit (TakeProjection, EditBirth or PostCapture). Its net retained heap (3,320 B) is about ten times what the step reports, so at least one step under-reports real heap; raising the grant alone cannot fix `heapMatchesReceipts`.
- Added to the test panic: `size_of` of snapshot, mutation and `Option<snapshot>`, so one run settles which quote is which and whether the fix is the grant (fixture) or the step accounting (framework).

## Status

No fix is committed. A fix without the run's `last_advance` and sizes would be a guess. The single admitted run (`en1990-replay-retry.sh`) was held back because Base publication (4.7 GiB) was still waiting for memory.

## Backfill attempts (19:00 to 20:00)

- Run 8 was admitted, compiled 446 units, then failed with E0433 in `semio-plugin-instance-stores` (missing `semio_plugin_retained_command_operation`). The manifest gained that dependency while the run was in flight, so this was shared-tree drift, not my change.
- Runs 9 to 13 were each admitted (or queued behind a 5,250 MB command) and stopped before the compiler: `Rust crate semio-framework-job-context exceeds its source boundary before the compiler: 17 of 32 files, 262450 of 262144 bytes`. Another agent's in-flight edit to `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` (a `[DEBUG]` eprintln guarded by `SEMIO_DEBUG_BORN`, file now about 92 KB and still changing) pushes the crate over the 256 KiB boundary. Every cargo run in the repo fails the same way until that crate is carved or the debug line removed. I did not touch it, as it is another agent's open edit.
- `en1990-replay-retry.sh` now also retries on E0433, E0432 and the source-boundary refusal. Logs: session scratchpad `en1990-replay-run*.log`.
- Still no run has reached the test body since the diagnostic edit, so `last_advance` and the size output are not available and no fix is made.

## Later attempts (20:00 to 21:40)

- Run 14 compiled 262 units then failed with E0599 in `semio-os-store-engine`: another agent's in-flight `[DEBUG]` method `owned_roots_witness` (on `ArtifactStore`, called on `StorePartsRef`) in `…/🏪️store/⚙️engine/📥️lifted/🏪️store/🧯️close-host-drop-and-pair-merge/🦀️.rs`. Not mine; shared-tree drift. The `job-context` boundary overrun had cleared by then.
- Run 15 (script retries now include E0599, E0433, E0432, E0425, E0308 and boundary refusals) sat in the capacity queue for 45+ minutes behind other agents' 2,663 MB and 5,250 MB commands without admission.
- Still no run has reached the test body after the diagnostic edit.

## Evidence from run 16 (first run to execute the diagnostic)

- `size_of::<En1990Snapshot>() = 400`, `size_of::<En1990Mutation>() = 328`, `size_of::<Option<En1990Snapshot>>() = 400`. So the failing quote (copy 4832, capacity 2800) is not a whole-struct move quote, which rules out `ColdReplayEdit::demands` and `PostCapture`.
- The previous turn (3401) ended with quote `copy 1` (a stage header) and `Pending` progress; the next stage quotes 4832 / 2800. 4832 equals the 4,096-byte page body plus 736 bytes of overhead, so the quote is the post-read Edit-stage page quote, and the fixture's `copyBytes: 4096` left no room for the overhead.
- Change: fixture `copyBytes` raised from 4096 to 8192 (body plus overhead, still bounded and still asserted against every quote).
- A heap mismatch is still reported at probe turn 35 (actual requested 562, released 14, reported all zeros), in a different place than in run 1 (turn 6820), so the shared source decoder moved between runs. This may be a genuine under-report in the framework mutation source and is the next thing to check once the grant no longer stops the replay.

## Run 17: grant fixed, heap accounting remains

With `copyBytes: 8192` the replay for operation 0 (no cancellation) completes: the demand-exceeds-grant panic is gone. The test now fails at its final assertion (line 354): `Unpaid { turn: 35, actual: (562 requested, 14 released), reported: all zero }`. The step at probe turn 35 allocates 562 bytes and reports no progress, so the physical accounting of one step is wrong (framework side), not the fixture.

The test now labels the phase of each probed run (`source-admit`, `source-advance`, `source-take`, `replay-birth`, `replay-denied`, `replay-funded`, `retire`, `retire-snapshot`, `finish`) and prints it in `Unpaid`. Run 18 (queued behind 4.6 GiB and 2.6 GiB commands, not yet admitted) will show the phase. Prime suspect from source reading: `ArtifactMutationSource::advance_chained` over a `HoldCustody::drive` that returns a Pending reply without reporting its allocation (`🔗️input/🦀️.rs`).
