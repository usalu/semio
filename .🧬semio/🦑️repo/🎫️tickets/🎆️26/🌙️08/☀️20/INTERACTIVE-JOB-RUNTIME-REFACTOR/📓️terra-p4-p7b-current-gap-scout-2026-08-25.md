# Phase 4 and P7b Current Gap Scout

Date: 2026-08-25  
Method: read-only current-source and ticket/verifier census. No Cargo, Nx, Bun verifier, Wasm, browser, runtime, or production-source command was run.

## Decision

**Phase 4 is source/static accepted in its later P4d/P4e scope, but remains runtime-matrix pending. P7b is source RED and has not started its required mounted-session repair.** The two packets must not be conflated: the Puzzle 3D route is now enqueue/poll-only; the Puzzle 2D route still performs whole job construction, JSON checkpointing, and one direct batch step in an action callback.

## Phase 4 — Puzzle 3D

### Current source/static status: GREEN, scoped only

The live `fill-build-tick` action now calls `poll_fill_job` and `enqueue_fill_job`, then emits one isolated `Effect::SpawnJob`; it contains no `precompute_step_lane` call. `enqueue_fill_job` retains and measures the registered `FillBuilder` owner before exposing its token. The current sources also contain adversarial/resume timing fixtures that measure individual steps against 8 ms and first candidate/preview against 50 ms.

The historical P4d/P4e acceptances still hold absent contrary source evidence in this scout:

- P4d R7-R11 GREEN: registry-exclusive admitted `FillBuilder`, resume-safe terminal reclamation, exact raw/decoded/live worker binding, and checked nonzero semantic identity allocation. See `PUZZLE-3D-RESUMABLE-VERTICAL-SLICE/📓️terra-p4d-r9-r11-independent-acceptance-audit-2026-08-24.md`.
- P4e B1/B2 GREEN: cooperative ten-root preflight including both weight maps; cap+1 retains the exact root item; the first refusal grant publishes a generation-qualified no-ghost diagnostic before the following fault; fixed spatial ownership and canonical renderer transport remain defended. See `PUZZLE-3D-RESUMABLE-VERTICAL-SLICE/📓️terra-final-p4e-b1-weight-acceptance-audit-2026-08-24.md`.
- The permanent verifier has P4e baseline/mutation predicates at `📜️script.ts:7754-8177` (including 21 P4e mutations). The historical P4e self-test reached its Puzzle predicates; the final global denial was unrelated P1q DB work.

The earlier 2026-08-22 P4 closure audit is historical only. Its direct action-drive, missing adversarial timing, missing first-preview, and cap/diagnostic claims are contradicted by the current P4d/P4e source and subsequent acceptance packets; it does still correctly identify that a phase ticket cannot close merely on source/static evidence.

### Remaining Phase 4 gates

No source repair is presently evidenced as required. The still-open gates are final-tree execution gates:

1. run the permanent P4 predicate as part of final static hygiene after the unrelated global verifier blockers are resolved;
2. compile/test native debug and release, strict warnings, `wasm32-unknown-unknown`, and `wasm32-wasip2` on the quiescent tree;
3. execute the mounted Puzzle 3D fixture through real WorkerPool counts 1, 2, 4, and default, requiring byte-identical accepted sequence/final commit and replay;
4. capture runtime instrumentation for all adversarial stages: each worker step below 8 ms, cancellation p99 below 8 ms, first substantive preview below 50 ms, and active preview cadence at or below 33 ms; and
5. exercise the browser route with recorded locale, renderer, worker count, callback/step maxima, cancellation, stale replacement, and preview rendering.

These must be run only by the serialized final-matrix owner; historical transcript results cannot certify the final shared tree.

### Bounded next packets

1. **P4 final static re-audit (Terra):** after source quiescence, read the six P4d/P4e leaves plus `📜️script.ts`, rerun the permanent static command as part of matrix stage 1, and verify the 21 P4e mutations and action enqueue-only route.
2. **P4 final runtime matrix (single serialized owner):** own P4 compiler/replay/timing/Wasm/browser execution only after all source packets settle. It must stop at the first failure and return a narrowly described source repair; it is not parallel-safe with other Cargo/Wasm work.

## P7b — Puzzle 2D Mounted Fill

### Current source/static status: RED

The full current route still matches the unrepaired contract:

- `set-fill-count/🦀️component.rs:35-42` calls `BoardHost::board_fill_snapshot`, creates `BoardFillJob`, writes `job.checkpoint_bytes`, and advances generation with `saturating_add`.
- `:53-90` clones the stored checkpoint, reconstructs a job with `BoardFillJob::restore`, builds a fresh `root_cancel_token`, directly admits a `BatchJobSession`, and calls `session.step()` in the action path. It hardcodes base revision zero.
- `:79-85` applies the job's placement JSON directly to the fixture and serializes a whole next checkpoint. `:113-115` drains terminal output in a loop, discards close errors, and begins close; terminal `CommitCandidate` is not handed to a live-authority consumer.
- The shared board implementation remains dynamic and whole-state: `BoardFillJobState` owns `Vec<serde_json::Value>` placements; `BoardFillJob::new` serializes the snapshot, `restore` `serde_json`-decodes snapshot plus state, and `checkpoint_bytes`, preview, and complete serialize full dynamic values (`board …/normal/🦀️component.rs:435-500,4468-4547,4789-4913`).
- A full repository caller census finds no production `WorkerJobSession<BoardFillJob>` or WorkerPool-owned Board fill session. The existing batch/test `drive_step` occurrences do not repair the mounted action.

Therefore the historical P7b contract has **no acceptance to carry forward**. The older WFC P7a/P7g/P7h static acceptances cover Assembly/WFC and guest relay/checkpoint cleanup, not Puzzle 2D; they do not reduce this P7b scope.

### Exact missing gates

1. fixed generation-tagged retained session on the process WorkerPool, with one worker step per admitted opportunity and retained cancel/operation/base revision/live generation;
2. persistent, credit-preflighted capture of nodes, handles, kinds, templates, source pairs, compatibility, and weights; stale capture must close without mutating published state;
3. owned fixed/page authorities and exact cap/+1 same-owner handback for every capture/search/placement/checkpoint/channel owner;
4. cursorized compatibility, template/ID/string/typed-placement construction, bounded prefix application, latest-wins fixed preview, and lossless bounded checkpoint/commit publication;
5. schema-first incremental checkpoint/restore with identity/page/count validation and no mounted whole serde/JSON snapshot/state/placement payload;
6. live revision/generation validation immediately before every prefix and terminal commit, with ACK/take/retry semantics rather than ignored terminal candidate;
7. one resumable terminal disposer for replacement, cancel, stale, saturation, producer rejection, panic/fault, unclaimed output, and document/window/app close, proving terminal empty rather than ordinary deep Drop;
8. checked generation exhaustion (no saturation/ABA); and
9. source fixtures/mutations and final debug/release/native/Wasm/WorkerPool/replay/allocation/watchdog/browser evidence specified by the P7b contract.

### Bounded implementation and audit order

P7b is intentionally serial at its shared Board/action seam; splitting the following into concurrent source edits would create ownership races.

1. **P7b-1 capture and authority (Sol High):** introduce the fixed/page-owned board-fill capture/session authority in the Board module; one source field/slot per grant, exact preflight/rejection return, checked operation/revision/generation allocator, and terminal retirement cursor. Do not touch action dispatch yet beyond compiling interfaces.
2. **P7b-2 retained worker route (Sol High):** replace the `set-fill-count` action continuation with enqueue/poll-only WorkerPool session ownership; wire bounded preview/checkpoint/commit channels, live authority validation, replacement and document/window/app close pumping. Delete the checkpoint-in-runtime action path rather than retaining it as compatibility behavior.
3. **P7b-3 bounded job payloads (Sol High):** convert compatibility/placement/preview/checkpoint/restore/commit to fixed/page cursors and schema-owned typed payloads; eliminate mounted `serde_json` whole snapshot/state/placement construction. Add the hostile source fixtures and faithful permanent verifier predicates.
4. **P7b independent source audit (Terra):** fresh constructor/caller/Drop census, admission/ownership/replay/UI route audit, cap/+1 and mutation review, rustfmt/diff plus permanent verifier after its supporting global scope is green. A RED finding returns one narrowly bounded remediation packet.
5. **P7 final execution (single serialized owner):** after P7b and P7c source acceptance, execute debug/release, strict warnings, real 1/2/4/default replay, cancellation/stale/close/saturation/panic/allocation stress, 8 ms/50 ms/33 ms metrics, native and both Wasm targets, then browser parity.

## Verification Boundaries

The final matrix contract at `📓️coordinator-serialized-final-verification-matrix-contract-2026-08-24.md` is authoritative: no historical build/runtime output is reusable on the final tree, timing requires instrumentation rather than source constants, and Cargo/Nx/Wasm/browser stay serialized until all overlapping Rust source packets are quiescent.
