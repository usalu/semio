# Terra Independent P7b Puzzle2d Mounted Retained Session Audit

Date: 2026-08-25  
Auditor: Terra independent read-only source/static audit  
Verdict: **RED — do not accept P7b.**

## Scope and method

I read the master plan; P7b packet and mounted-retained repair contract; the fresh P4/P7b scout; the current Sol report; accepted P2a1 material; the current P2c and P1 source audits; current Board fill, Puzzle2d editor/action/schema/UI/terminology, and brush fixture sources. I inspected callee bodies, not just the new API names.

No production source, shared verifier, build, Cargo, Nx, Wasm, browser, or runtime test was run. The static checks listed below were run on the current shared tree.

## Decision

The inner BoardFillJob is materially improved: its job state uses bounded page authorities and one-stage worker steps, the mounted session genuinely submits through the shared process worker pool, the eight-slot registry has reservation and guard recovery, and the EN/DE lifecycle controls are real. Those properties do not repair the production ArtifactView route. Every fill continuation still reconstructs and parses the whole document before it reaches its supposedly bounded capture or worker opportunity. In addition, capture itself is called directly by the action path, terminal CommitCandidate is ignored, and one of the eleven owned page insertion branches discards the returned item.

## P0 — Every continuation performs whole document cloning, JSON decoding, and BoardHost rebuild

The real mounted action route is Puzzle2dPlayApp::handle, not the isolated BoardFillJob fixture.

1. handle receives the real ArtifactView, but at editor component lines 989-1008 it clones doc.snapshot.0 twice, constructs a fresh scene, constructs a fresh BoardHost, synchronizes the fixture, and only then dispatches brushFillSessionStep and the other fill arms.
2. The active dispatch maps brushFillSessionStep through the live set-fill-count step function at editor component lines 1037-1044.
3. sync_host_fixture_content calls BoardHost::parse_fixture_v1 plus JSON catalog serializations at editor component lines 551-562. parse_fixture_v1 immediately clones the whole serde_json::Value, clears every live board collection, and iterates all nodes and handles at board component lines 8694-8765.
4. After the action, the same route derives a full document delta at editor component lines 1051-1063. The callee compares whole Values, clones both Values, serde-decodes both snapshots, and computes their aggregate mutation sequence at schema mutations component lines 326-335.

Thus even a queued, capture, apply, adoption, cancel, or discard continuation has unbounded whole-document copy/decode/rebuild/diff work on its production action path. This directly falsifies P7b's one admitted semantic/allocation/copy unit per grant and its ban on whole mounted snapshot work. The hostile brush tests bypass this route: their test-only capture_fill_snapshot drives a direct BoardHost capture to completion at brush component lines 35-45, and their mounted session helpers construct standalone headless pools at lines 47-182. They cannot prove the live ArtifactView route bounded.

## P0 — Capture remains action-thread work, outside the worker-owned session

begin_fill_job creates a BoardFillSnapshotCapture from ctx.host at set-fill-count component lines 702-755. step_fill_job then directly calls capture.step(&ctx.host.borrow()) at lines 1002-1045; only after capture completes does it build BoardFillJob and MountedWorkerJobSession at lines 1009-1037.

The inner session does use pump_one with process_worker_pool at lines 827-830. MountedWorkerJobSession::pump_one itself submits one worker task and later takes the ticket-qualified outcome at framework job component lines 1749-1777. But that does not cover capture. Capture consumes the reconstructed dynamic BoardHost and does not receive StepContext fuel, deadline, cancellation, worker capability, or worker ownership. This violates the P7b requirement that the action only admits/enqueues retained worker work and that node/handle/kind/template/rule capture is governed one worker opportunity at a time.

## P0 — Terminal CommitCandidate is discarded rather than consumed and acknowledged

BoardFillJob::complete returns StepOutcome::Complete(CommitCandidate) at board component lines 5979-5987. The live session consumer matches that outcome as StepOutcome::Complete(_) and instead reads BoardFillJob::take_result at set-fill-count component lines 869-879. It neither takes, validates, nor acknowledges the CommitCandidate. adopt_fill_job later accepts only the out-of-band FillTerminal::Completed(result) scalar at lines 1060-1092.

The contract requires terminal CommitCandidate consumption and acknowledgement, not rereading mutable job state. The candidate's current payload happens to be empty; that makes the missed ownership observable only as a structural source violation today, not acceptable. A future payload would be silently ignored on this unchanged route.

## P1 — The eleventh owned page insertion does not restore the returned item

Ten BoardFillFixedPages::try_push_owned calls explicitly bind Err(item) and restore it to a retained cursor. The remaining compatibility branch is:

- BoardFillJob::scan_compatibility at board component lines 5643-5662 copies the retained candidate, calls state.candidates.try_push_owned(candidate).is_err(), and returns the capacity fault.
- BoardFillFixedPages::try_push_owned returns the exact T on capacity or page-admission refusal at lines 431-447.

The returned Err(candidate) is discarded by is_err instead of being explicitly restored through the same-owner handback path required for all eleven sites. BoardFillCandidate is currently Copy, so its retained pre-call copy remains in compatibility_candidate and no heap backing is lost in this instance. Nevertheless this is not the required exact returned-owner transfer and a faithful mutation can restore the discard without a local source-law failure. The source fixture census does not test this branch's MAX+1 handback.

## Positive findings preserved

- The legacy mounted-action census found no production board_fill_snapshot call, BoardFillJob::new, checkpoint_bytes, BatchJobSession, direct drive_step, or RevisionId(0) in the P7b fill action/core. begin_board_fill_snapshot is the new staged capture constructor, not the retired whole snapshot method.
- BoardFillFixedPages uses a fixed table with fallible four-item page admission and observed capacity bytes at board component lines 402-495. The other ten try_push_owned sites retain their returned item explicitly.
- The fixed eight-slot registry reserves before backing allocation, releases a pre-publication reservation in Drop, republishs a checked-out node through FillSessionGuard::drop, and rejects mismatched operation/generation in set-fill-count component lines 480-583. Its hostile MAX/MAX+1, pre-publication panic, lost-guard, and terminal-generation tests are present at lines 1187-1237.
- The actual inner job is staged, fuel-one, cancellation/deadline/operation-generation checked, and produces a fixed preview after each worker unit at board component lines 5995-6091. Its close walks scalar/page owners incrementally at lines 6093-6191.
- The mounted UI control supplies lifecycle status and localized cancel/retry actions at fill tool component lines 20-85; EN/DE terminology exists at terminology component lines 45-52, with an explicit German accessibility law at fill tool lines 114-140.
- The brush fixture has concrete direct-job coverage for cancellation/stale, deadline, submit refusal/saturation, unclaimed terminal, 1/2/4/default chronology, MAX+1 capture, and stage visibility at brush component lines 554-920. Those are useful inner-job laws but do not cover the P0 ArtifactView/action route.

## Scoped static gates

| Gate | Result |
| --- | --- |
| rustfmt --edition 2021 --check --config skip_children=true on Board fill, Puzzle2d editor/action/UI/brush leaves | PASS |
| Bun JSON.parse of the Puzzle2d fill config schema | PASS |
| Scoped git diff --check across Board fill and Puzzle2d editor sources | PASS |
| Legacy mounted fill census | PASS for the retired names listed above |
| All eleven try_push_owned bodies inspected | FAIL: lines 5655-5657 discard Err(candidate) |

No shared verifier command was run because it is explicitly outside this read-only P7b audit and no faithful P7b permanent mutation gate exists in the root verifier. No compiler, runtime, parity, allocation, watchdog, native, or Wasm conclusion follows from these static checks.

## Required closure

1. Make the real ArtifactView continuation route retain/admit document input once and remove whole Value cloning, parse_fixture_v1 rebuild, JSON catalog serialization, and whole delta diff from every P7b grant. The action must not reconstruct the live board around a retained session.
2. Move BoardFillSnapshotCapture under the mounted worker authority with fuel/deadline/cancel/stale checks, rather than calling capture.step directly from brushFillSessionStep.
3. Transfer, freshness-validate, and ACK the terminal CommitCandidate at the live consumer; do not substitute BoardFillJob::take_result.
4. Bind and restore the Err(candidate) from the compatibility insertion, and add a production-shaped MAX+1 mutation/law for all eleven insertion sites.
5. Add mounted ArtifactView route laws that execute the actual dispatch/action path for capture, candidate, paged apply, stale document revision, cancellation, abandonment, terminal, and close. Then rerun this independent static audit before the deferred compiler/runtime matrix.

