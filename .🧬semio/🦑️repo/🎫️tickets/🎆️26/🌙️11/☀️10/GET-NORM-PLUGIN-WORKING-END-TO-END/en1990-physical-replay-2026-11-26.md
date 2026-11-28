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

## Heap accounting findings (runs 23 to 31)

After the grant fix the replay completes, and the test fails only at its physical heap equality. Labelled runs show where:

- Probe turn 35 (loop turn 0, stage SourceCapture, quote copy 104): the DENIED `owner.advance(body, denied, control)` (`maximum_items: 0`) allocates 562 bytes (14 released) and reports zero progress. Later denied calls each allocate 14 bytes (released 14, or 0 on some turns). `NativeSemanticPreparation::advance` should return Blocked right after `self.demands()` and the admission check, so any allocation before that return is unpaid.
- Control runs (run 31) isolate the cause: an empty probed run and a probed `advance_demands(body, WORK)` alone are exact (no mismatch). So the allocation is in `advance` itself before Blocked, i.e. in the identity getters (`ledger_identity`, `retirement_recipient_identity`), in `demands(body, u64::MAX)` (maximum work differs from the WORK used in the clean quote run), or the admission check. Run 32 adds two more controls (`advance_demands(body, u64::MAX)` and the two identity getters on the control) to split these; it was queued behind other agents' builds at the time of writing.
- Funded steps show a constant offset: actual requested = reported retained + 42 and released 14 (for example `(42, 14)` vs retained 0, `(3306, 14)` vs retained 3264, `(42, 42)` vs 0). So each funded advance has about 28 bytes of retained and 14 of transient allocation that no step reports, on top of the denied-path allocation.
- job-context agent (af18d662b882e6fd5) confirmed it has seen no one-time 562-byte allocation in its charge and progress path; this is in `store/replay/operation/native`.

Remaining work: remove the unpaid allocation from the denied path and report (or avoid) the constant per-advance allocation, then rerun. Test changes only so far: fixture `copyBytes: 8192`; phase, note and later-mismatch labelling in `🧪️tests/🧮️physical/🦀️.rs`; extra control runs per turn (`replay-quote-only`, `replay-ids-only`, `replay-empty`) that can be deleted once the cause is fixed.

## Root cause of the denied and per-call allocations, and what remains (run 34)

- Found: the denied-advance and per-call allocations came from leftover `[DEBUG]`-era instrumentation in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs`: the `ArtifactReplayPreparation::advance` and `close_step` trait wrappers called `dbg_add(format!(...))` into a global `BTreeMap` behind a `Mutex` (`DBG_MAP`, no reader anywhere), allocating a `String` per call plus the first map insert (the 562 bytes), before the Blocked check. A second `dbg_add` was in `🌱️derived/🦀️.rs::record`. All of it is removed (`dbg_branch`, `DBG_MAP`, `dbg_add`, both wrappers back to plain delegation, `record` back to plain accumulation). The physical test pins "denied advance allocates 0 bytes" through its per-turn denied probe, which requires actual heap to equal reported (zero).
- Run 34 (after the removal) confirms the denied path, funded steps and early stages are now exact: the first unpaid step moved from probe turn 35 back to turn 6820, the cold edit.
- Remaining failure: `Unpaid { turn: 6820, actual: (11973 requested, 8653 released), reported: copied 400, retained capacity 328 }`. 400 and 328 are `size_of::<En1990Snapshot>()` and `size_of::<En1990Mutation>()`: `ColdReplayEdit::advance` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs`) reports a size-based quote, while the domain `ColdMutation::diff`, `apply_diff_cold_into`, `retire_cold` and `inverse` allocate 11,973 bytes (net 3,320 retained). The framework law is "receipt equals measured heap", measured only in tests (`semio-framework-trace` is an optional `testkit` dependency of `semio-os-store-one-item`, and no production crate observes heap). So exact funding of the cold edit needs a design decision, not a typo fix: (a) have the cold edit report measured heap behind a test-only observation hook (needs the optional trace dependency or a thin re-export), (b) add a heap quote to `ColdMutation` so the framework charges real amounts before applying, or (c) replay through the metered `RetainedCloneEdit` path instead of the cold reference semantics. Option (b) is the clean long-term one and touches the mutation trait in `semio-os-spr`, so I did not start it unasked.
- Test cleanup: all temporary control runs, phase, note and later-mismatch labels are removed. The test keeps the fixture `copyBytes: 8192` fix, the `last_advance` panic detail, and its exact per-step heap assertions.

## Scoped heap quote in ColdPreparationBounds (approved design)

Implemented (compiles through the physical-test build in run 35, then revised after run 35's finding; revision not yet compiled):

- `ColdHeapQuote { copy_bytes, capacity_bytes, transient_bytes }` (new, `…/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs`, exported through paged-one-item and the store crate) with `ColdHeapQuote::reference(snapshot_bytes, mutation_bytes)`: four clones of the measured base and mutation plus one 4 KiB page, for retained and transient heap alike (`REFERENCE_FACTOR = 4`, `REFERENCE_FIXED_BYTES = 4096`).
- `ColdPreparationBounds` gained `heap_quote: fn(&S, &M) -> ColdHeapQuote` (domain supplied, computed from the inputs). `ColdMutationApplyFactory::bounded` now works for every projection; `ColdReplayEdit` carries the bounds, quotes `funded_demand` (copy `size_of::<S>()`, capacity and release from the quote) in `demands()`, and refuses to apply unless the grant admits that quote. The ledger charge stays the exact `size_of::<M>()` retained for the inverse, so nothing is over-charged and no refund is needed (the quote funds the grant, the receipt stays exact). Without bounds the old size quote is unchanged.
- Users updated with real measures: the 5 bounded-factory users (process3d host envelope, playground editor, sourcing curation config store and document store, playbook) and the framework cold-replay test.
- EN 1990: `en1990_cold_replay_bounds()` (new module `🧬️mutations/🧊️cold-bounds/🦀️.rs`) with an allocation-free structural `snapshot_bytes` (root record plus per-table rows with a 256-byte text allowance each) and a resident-size `mutation_bytes`, central Store maximum, reference quote. The first version priced by canonical JSON text; run 35 showed that serialising on every quote allocates 56,038 bytes per call, so quotes must not allocate. The norm macro `norm_exact_store_ownership!` gained a `(live, $factory, $ctor)` arm; the EN 1990 editor passes `RetainedColdMutationApplyFactory::bounded(en1990_cold_replay_bounds())`.
- Physical test: exact accounting everywhere; for the cold edit step only (identified by its quote) it asserts measured requested bytes at most capacity plus transient and net retained bytes at most capacity, and asserts exactly one such quoted turn per uncancelled run. The JSON fixture gained a schema-first `heapQuote` block (factor, fixed bytes, reference cases) that the Rust test checks against `ColdHeapQuote::reference`; the JSON schema and the Bun/Ajv oracle validate it.

Status: run 35 (first quote build) compiled and ran; its only unpaid step was the JSON-measure allocation inside the quote, which the structural measure removes. The rebuilt run 36 has been queued for over 90 minutes behind other agents' commands (the host held 6.2 GiB for another command; my request waits at the head of the memory queue) and has not run. Not yet run: the green EN 1990 replay, `check` of the five touched plugin workspaces (`semio-s-artifact-process-process3d-host`, `semio-s-artifact-demonstrator-playground-editor`, `semio-s-artifact-sourcing-curation`, `semio-s-artifact-playbook-playbook-v1`, each in its own workspace root) and the framework cold-replay test.

## Final result

- EN 1990 physical replay: run 37 (allocation-free structural `ColdHeapQuote`, reservation 2,300 MB) is green: `test result: ok. 1 passed`, exit 0 (log `en1990-replay-run37.log` in the session scratchpad).
- Pending verification, queued by `cold-quote-verify.sh` (one worker, `--memory=2300`, each from its own workspace root, logs `verify-*.log` in the scratchpad): `check` of `semio-s-artifact-process-process3d-host`, `semio-s-artifact-demonstrator-playground-editor`, `semio-s-artifact-sourcing-curation`, `semio-s-artifact-playbook-playbook-v1`, then `test -p semio-os-store-paged-one-item --lib` (framework cold-replay tests). At the time of writing the first of them had waited about 50 minutes behind another agent's 6,004 MB request at the head of the queue.
- Unused `DOCUMENT_LOAD_*_CODE` import warnings in the run are in `semio-plugin-*` crates (`plugin/decl-app` style imports), not in any file I touched. The `[DEBUG]` code under `🧵️job` (`dbg_post`, worker eprintln, authority and retirement-context lines) is not mine either.

Files changed (all under `/Users/ueli/Documents/semio`):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs` (ColdHeapQuote, bounds field, ColdReplayEdit quote)
- `…/📬️paged-one-item/🦀️.rs` and `…/🏪️store/🦀️.rs` (exports)
- `…/📬️paged-one-item/🧪️tests/❄️cold-replay/🦀️.rs` (bounds literal)
- `…/🔁️replay/🎮️operation/🧬️native/🦀️.rs` and `…/🌱️derived/🦀️.rs` (removed `dbg_add`/`DBG_MAP` instrumentation)
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🛡️store-ownership-macros/🦀️.rs` (macro arm)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/…/✏️editor/🦀️.rs`, `…/🧬️mutations/🦀️.rs`, `…/🧬️mutations/🧊️cold-bounds/🦀️.rs` (new)
- `…/🧬️mutations/🧪️tests/🧮️physical/🦀️.rs`, `🧫️fixtures/🔣️.json`, `🧬️schema/🔣️.json`
- Bounded-factory users: process3d `📏️envelope/🦀️.rs`, playground `✏️editor/🦀️.rs`, curation `📬️config-store-preparation/🦀️.rs` and `🗄️artifact-store-preparation/🦀️.rs`, playbook `✏️editor/🦀️.rs`
- Ticket: `en1990-physical-replay-2026-11-26.md`, `en1990-replay-retry.sh`, `cold-quote-verify.sh`

## Verification results (relaunched after the broker fix)

- `check` (one worker, 2,300 MB, own workspace root): process3d-host rc 0; playground-editor rc 0; playbook-v1 rc 0.
- `check` sourcing-curation: rc 101 on a foreign error, `E0599 no method named debug_state` in `semio-framework-actor-retained-context` (another agent's in-flight `[DEBUG]` helper; `debug_state` was added to `🧵️job/⏱️context/🎟️source/🦀️.rs` shortly after). Not caused by this change. Rerun queued in `cold-quote-verify2.sh`.
- `test -p semio-os-store-paged-one-item --lib` (no feature): rc 0, 1 test. The cold-replay tests need `--features test-support`; that run (`cold-quote-verify2.sh`, logs `verify2-*.log`) has been waiting in the capacity queue behind another agent's 2,663 MB command and has not executed.
- EN 1990 physical replay: rc 0 (run 37).
