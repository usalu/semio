# P7b Puzzle2d Fourth Independent Source/Static Acceptance Re-audit

Date: 2026-08-25  
Auditor: Codex independent source/static audit  
Verdict: **RED — P7b must not be accepted.**

## Scope and method

I reread the P7b repair contract, Sol's implementation report, and Terra's three prior RED audits. I independently inspected the live editor branch, config/schema, mounted fill action/session, Board ingress/search/terminal code, and the current diff. This report does not trust prior green claims.

No Cargo, Nx, Wasm, browser, runtime, or shared verifier command was run. No production source was edited. The only new file is this audit.

## Decision

The earlier whole-config branch P0 is closed: the fill branch makes a `Puzzle2dFillRuntime::from_config(config)` scalar projection and emits `Puzzle2dConfigMutation::Fill`, before the ordinary document clone/rebuild path. Its current branch has zero `config.clone()`, `Puzzle2dConfigMutation::Snapshot`, `Vec<Value>`, `BTreeMap`, `BoardHost`, `RefCell`, or document clone.

P7b is nevertheless RED for two independent current P0 violations:

1. The live fill action/session owns dynamic `String` values while constructing the typed placement mutations. This is prohibited by the requested action-ownership boundary and leaves the action verifier with a material blind spot.
2. `StepOutcome::Complete(CommitCandidate)` contains only a 13-byte summary (`accepted_count`, `stalled`, `search_count`). The terminal consumer updates runtime status and queues adoption; it emits no typed node-and-edge event-sourced mutations from that candidate, and therefore cannot meet the required same-turn destination-credit/source-kind terminal path.

Either P0 blocks acceptance.

## P0-A — Dynamic action-owned strings remain live

`FillPlacementApplyCursor` is retained by `FillSessionNode` in the mounted action module. It declares `FillPlacementEdgeOwner` with `id`, `source`, `target`, and optional `edge_kind` `String` owners at `set-fill-count/🦀️component.rs:778-794`. The same cursor creates and copies dynamic strings one field at a time, including node/handle values at lines 847-945 and edge values at lines 947-965. It has seven direct dynamic String declarations/initialisations and eleven `.to_string()` constructions in the production action file.

Its close path is incremental in places, but that does not make these fixed-only action state. In particular, `self.handle.take()` at lines 990-993 can ordinary-drop a populated `Puzzle2dHandle` (which owns strings) in one action-close opportunity. The P7b requirement here is zero action-owned `String`, not merely a staged dynamic allocation.

The local `mounted_fill_source_contract` only checks the textual `Puzzle2dFillActionCtx` slice for `String`/`Vec<Value>`/`BTreeMap` at lines 1954-1977. It never rejects a dynamic owner elsewhere in the action module; the actual dynamic owners above therefore coexist with the passing local predicate. This is not a verifier that enforces the stated full action boundary.

## P0-B — Terminal CommitCandidate has no typed node/edge payload or terminal apply

The Board producer defines `BoardFillResult` solely as `accepted_count`, `stalled`, and `search_count` at `board/🦀️component.rs:1025-1058`; its committed payload is exactly 13 bytes. `BoardFillJob::complete` writes only that encoded result and publishes `CommitCandidate { state: empty, output }` at lines 6482-6506. The completion body has zero placement/node/edge references.

The mounted consumer correctly matches `StepOutcome::Complete(candidate)` and avoids `BoardFillJob::take_result`, but it only decodes that summary, writes runtime counters, records `FillTerminal::Completed(result)`, and queues adoption at `set-fill-count/🦀️component.rs:1628-1658`. It does not reserve mutation destinations or emit `create_node`/`connect_handles` there. Those mutations are instead emitted by the checkpoint-owned `FillPlacementApplyCursor` at lines 967-976. Thus the required exact terminal candidate → same-turn credited typed node+edge mutation path is absent, including source edge kind carried by the terminal candidate.

## Confirmed retained properties

- The early fill branch is before the ordinary `doc.snapshot.0.clone()` branch and is guarded by a source predicate at `editor/🦀️component.rs:1526-1575`; it now rejects injected config clone and Snapshot mutations.
- `Puzzle2dFillRuntime` is `Copy` and contains only fixed scalars/enums/fixed 64-byte text slots at `config/🦀️component.rs:147-190`; `Puzzle2dConfigMutation::Fill` exists at lines 342-368.
- `ArtifactBoardFillJob` owns the generation-qualified `SnapshotRead`, performs worker-only capture, checks cancel/operation/generation/snapshot authority before and after one `capture_one`, and consumes one fuel unit at `set-fill-count/🦀️component.rs:154-220, 670-759`.
- Capture is cursorized by node/handle/kind/rule field enums and byte cursor at lines 36-152, with scalar/byte ingress opportunities rather than whole-record ingress.
- The Board source handback census is GREEN: the range from `impl BoardFillSnapshotIngress` to `impl Drop for BoardFillJob` has exactly 11 `try_push_owned` sites; every one binds `if let Err(...)`, and no compatibility `.is_err()` discard remains. The local hostile mutation is at Board lines 12541-12568.
- No production action `BoardFillJob::take_result` call is present. The registry remains fixed-eight-slot and generation-qualified; SnapshotRead registration/reconciliation and shared InteractiveNative worker session remain live.

## Executed static gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on Board plus scoped P7b Puzzle2d Rust leaves | PASS |
| Bun `JSON.parse` of the Puzzle2d fill config schema | PASS |
| Scoped unstaged and staged `git diff --check` over Board/Puzzle2d | PASS |
| In-memory editor predicate: injected `config.clone()` in fill branch | PASS: rejected |
| In-memory editor predicate: replace typed `Fill` with `Snapshot { config: config.clone() }` | PASS: rejected |
| In-memory config predicate: inject `Vec<Value>`/`String` into fixed runtime | PASS: rejected |
| In-memory action-context predicate: inject `Vec<Value>` into fill context | PASS: rejected |
| Full action ownership / terminal candidate census | **FAIL**: live dynamic Strings and summary-only terminal candidate |

The mutation checks above faithfully reproduce the live textual contract logic without writing source. They prove the new config-projection regressions are rejected; they cannot prove P0-A because the current source already violates it and the action predicate intentionally scopes only its context slice.

## Required closure

1. Remove `String` and `Puzzle2dHandle`/node/edge dynamic ownership from the action/session application path. Move a fixed typed placement representation through a bounded authority, or make every action-side field owner fixed capacity; then extend the verifier to inspect the complete production fill action/session region and reject any `String`, `Vec<Value>`, `BTreeMap`, `BoardHost`, `RefCell`, full config snapshot, or document clone.
2. Make `CommitCandidate` carry the exact final typed placement/page authority. Its `Complete` consumer must validate freshness, reserve destination node+edge capacity in that same bounded turn, emit the typed mutations including source edge kind, and ACK/close the exact candidate with all refusal paths returning the owner.
3. Add hostile source mutations for an action-wide dynamic string owner and for replacement of the terminal typed candidate with the current summary-only result; both must fail before a new acceptance audit.

Deferred compiler/runtime, WorkerPool 1/2/4/default parity, saturation/panic/cancel/close stress, allocation, native/Wasm, and watchdog evidence remain unexecuted and are not acceptance claims.
