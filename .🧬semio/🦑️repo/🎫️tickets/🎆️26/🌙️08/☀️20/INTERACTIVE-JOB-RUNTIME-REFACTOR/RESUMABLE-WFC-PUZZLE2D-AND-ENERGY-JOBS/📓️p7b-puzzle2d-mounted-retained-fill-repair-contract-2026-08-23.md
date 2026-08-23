# P7b — Puzzle2d Mounted Retained Fill Repair Contract

Date: 2026-08-23  
Owner: `/root` coordinator  
Verdict: bounded source packet prepared; implementation and acceptance remain pending.

## Exact live route

The sole mounted Puzzle2d fill action is
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs`.
It calls `BoardHost::board_fill_snapshot`, constructs `BoardFillJob`, serializes a complete
checkpoint, restores that complete checkpoint on every `brushFillSessionStep`, calls `drive_step`
directly on the action thread with a fresh root cancellation token, applies placement JSON to the
fixture, then serializes the job again.

The shared Board implementation is in
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs`:

- `BoardFillJobState` begins near line 435;
- `BoardFillJob` begins near line 485;
- `BoardHost::board_fill_snapshot` begins near line 4384;
- `BoardFillJob` constructors/checkpoint/restore begin near line 4461;
- `accept_candidate` begins near line 4749; and
- preview, complete, and `InteractiveJob::step` begin near lines 4878–4900.

No production `WorkerJobSession<BoardFillJob>` or mounted WorkerPool owner is present. The current
job's `snapshot_checkpoint`, sources, rejected sets, candidates, connected set, virtual graph,
dynamic JSON placements, and previews are all ungoverned dynamic owners.

## Required repair

### Persistent mounted session

Replace the action continuation loop with one generation-tagged, worker-owned session retained for
the document/window lifetime. `set-fill-count` may admit and enqueue work, but it may not capture a
whole snapshot, construct/restore a job, drive a step, encode a checkpoint, or apply an unvalidated
candidate on the UI/action thread.

The mounted route must use the process WorkerPool and the universal job-session authority. It must
retain one cancellation source, exact operation/generation/base-revision identity, bounded
preview/checkpoint/commit channels, explicit take/retry/ACK semantics, and a document/window/app
close disposer. Starting a replacement operation must cancel and drain the displaced session
without dropping its live owners.

### Retained snapshot capture

Replace `BoardHost::board_fill_snapshot` on the mounted route with a persistent capture job. Capture
one admitted field or fixed collection slot per grant across:

- nodes, IDs, and world-bound computation;
- handles, IDs, node/handle kinds, anchor/slot calculation, visibility, and connectivity;
- node kinds, icons, geometric scalars, and handle templates;
- source-pair construction;
- compatibility traversal over source pair × kind × template; and
- node/handle weight entries.

The capture must validate the live board revision/generation before taking each page and again
immediately before snapshot publication. A mutation during capture makes the candidate stale and
routes it through retained close. Last-valid content remains visible while capture/search is
pending.

Use fixed/page-owned collections or explicit owned container authorities. Do not infer standard
map/set node bytes from layout, treat requested capacities as actual capacities, or omit Box/Arc
control backing. Preflight item/page/operation/aggregate byte credits before copying. Every maximum
has exact +1 same-owner handback.

### Retained search and construction

Preserve the existing valuable one-source, one-candidate, and one-collision cursors, but remove the
remaining monolithic work:

- compatibility key construction and every dynamic ID/string copy need retained field cursors;
- set lookup/insert and vector push need owned fixed/page authority;
- candidate acceptance must build at most one template/handle/string/JSON field per grant;
- placement output must be a schema-owned typed candidate, not a growing `serde_json::Value` tree;
- placement prefix application must be one bounded, freshness-validated mutation per grant;
- preview construction/encoding must be fixed-size and latest-wins;
- checkpoint and commit construction must be fixed-page and lossless; and
- completion must never serialize all placements in one turn.

The source board is immutable until a bounded prefix or final candidate is validated against the
current document revision and operation generation immediately before atomic publication.
The terminal `CommitCandidate` must be consumed and acknowledged; it may not be ignored in favor of
re-reading mutable job state.

### Resumable checkpoint and restore

Remove whole `serde_json::to_vec`/`from_slice`, whole snapshot copies, and `checkpoint_bytes` from
the production path. Build a schema-first versioned checkpoint through persistent field/page
cursors and restore it through a matching retained parser. Validate all length/count fields before
allocation. Checkpoint state must include exact operation, generation, base revision, RNG,
search/capture cursors, candidate owner identities, and channel sequences without cloning the
authoritative snapshot.

Restoring with a wrong operation/generation/revision, truncated page set, duplicate/missing page,
or saturated authority must return the exact owner for retry/close and must not mutate the live
session. Deterministic uninterrupted/resumed results are required for every worker count.

### Terminal ownership

Cancellation, stale continuation, replacement, channel saturation, producer rejection, panic,
fault, completed-but-unclaimed output, document close, window close, and app close all drain through
the same retained disposer. Close advances at most one scalar/fixed page/fixed slot/control owner
per grant and ends only with terminal-empty evidence. Ordinary Drop must fail closed while any
snapshot, checkpoint, placement, preview, candidate, channel, session, or WorkerPool child owner is
live; it must never deep-drop after a fixed retry count.

Generation advancement uses checked arithmetic and refuses exhaustion. `u64::MAX` must not alias a
new session through saturation or wrapping.

## Permanent fixtures and verifier mutations

Add source fixtures and self-mutating verifier predicates for:

- nodes, handles, kinds, templates, compatibility, placements, pages, and total bytes at maximum
  and maximum +1 with exact pointer/page identity on rejection;
- zero fuel, insufficient fuel, expired deadline, and cancellation at capture/search/construct/
  preview/checkpoint/commit/restore/close boundaries;
- one template per accept turn, one compatibility pair per turn, one placement field per turn, and
  one close owner per turn;
- stale revision before capture publication, prefix publication, and final commit;
- wrong/duplicate/stale/wrapped/ABA operation-generation handles;
- allocator over-capacity and formerly-populated map/set backing retirement;
- panic and producer rejection before and after ownership transfer;
- latest-wins preview saturation without checkpoint/commit loss;
- checkpoint/commit saturation with exact retry/ACK and no silent eviction;
- cancel after `Complete` before output takeover;
- interrupted close followed by exact resume to terminal-empty;
- deterministic uninterrupted/restored output across WorkerPool sizes 1, 2, 4, and default; and
- UI callback and worker-step watchdog probes with substantive first preview and active cadence.

The permanent verifier must reject reintroduction of `board_fill_snapshot` in the action,
`BoardFillJob::new/restore/checkpoint_bytes` in the action, direct `drive_step`, fresh root cancel
tokens, hard-coded revision zero, ignored terminal candidates, dynamic JSON placement authority,
whole serde checkpoints/previews/commits, bulk capture/compatibility/template loops, missing exact
preflight, unchecked generation, or cancel-by-drop.

## Acceptance gates

Source handoff requires scoped edition-2021 `rustfmt --check`, focused permanent verifier self-tests,
live source verifier with no P7b-specific failure, deterministic ledgers, exact constructor/caller
census, and scoped/whole `git diff --check`. The serialized final lane must then execute debug and
release Rust gates, strict warnings, native and both Wasm targets, real WorkerPool 1/2/4/default,
saturation/panic/cancel/close stress, deterministic replay, allocation evidence, and watchdog
evidence for the 8 ms ceiling and preview cadence.

This contract is not an implementation or acceptance claim. Phase 7 remains open.
