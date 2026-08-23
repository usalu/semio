# Sol Independent P1t Terminal and Fixed-Owner Remediation Re-audit — 2026-08-23

## Audit admission

The coordinator requested an independent Sol High source re-audit of the P1t terminal/fixed-owner
remediation. Terra admission was scheduler-limited. This audit has no P1t implementation authorship,
made no production edits, and treated the updated P1t report and the prior independent rejection as
claims to verify against the current source and diff.

Reviewed scope:

- the prior `sol-independent-p1t-db-engine-retained-history-replay-audit-2026-08-23.md` rejection;
- the updated `p1t-db-engine-retained-history-replay-2026-08-23.md` report;
- replication CRC, DB artifact replay/actor, DB engine outer authority, CLI/facade, and root
  interactivity-verifier source and diffs; and
- scoped and whole working, staged, and `HEAD` whitespace diffs.

## Verdict

**REJECT — source-only P1t terminal/fixed-owner remediation.** The normal replay path now has the
claimed fixed backing, page-sized reads, incremental CRC/token cursors, and a useful retained
fault-retirement phase. It is not source-acceptable because cancel-before-handoff still destroys the
entire preallocated reservation through ordinary `ArtifactHistoryAdmission` destruction, and a
caught replay panic can leave the phase at `Complete`, bypass `FaultRetire`, retain all pages, and
reschedule forever. The adversarial fixtures and verifier mutations are structural name checks and
do not discriminate either live failure.

Phase 1 remains **RED**. The six named DB-engine wait groups, backend syscall duration, build/type
validation, and native/Wasm/browser/platform runtime matrix also remain open.

## Verified remediation foundation

- `HistoryReplayReservation::try_new` preallocates a 1,024-slot source-page vector, 960 individually
  backed 16-KiB result pages, capacity for 8,192 operation ranges, capacity for 4,096 entries, and a
  fixed 16-KiB scratch owner. Live replay `next` contains no `?`, `try_reserve`, `reserve`,
  `Vec::with_capacity`, or `String::from_utf8` allocation path.
- Length/item checks precede live `operation_ids.push` and `entries.push`, so those two vectors do
  not grow past their admitted backing on the intended path. Result writes target already allocated
  pages.
- WAL reads use `ByteRange { offset, len: requested }` with `requested <= 16 KiB`; returned length,
  capacity, page count, and checked offset are validated before ownership publication.
- `Crc32cCursor` retains CRC state, frame payload work consumes at most one page, and the tokenizer
  advances one length byte, scalar field, CRC page, trailer byte, or finish opportunity. Raw command
  payloads remain numeric ranges over source pages.
- Backend, tokenizer, envelope, frontier, range, result-capacity, and ordinary cancellation errors
  reached through `next` are matched into `begin_fault`/`HistoryReplayPhase::FaultRetire`; that phase
  retires one rejected/source/result/range/entry/scratch owner per poll on the non-panic replay path.
- Completion and `HistoryFuture::Drop` serialize on the completion mutex. A completed result is
  transferred into the fixed public terminal registry, and `HistoryView::Drop` similarly returns an
  unfinished view rather than ordinarily destroying its nested page graph.
- Terminal job/work/result and actor-terminal-job take/resume/close surfaces exist. An active actor
  ask is returned to live work under cancellation rather than being dropped by the outer close API.
- The production engine census is exactly six executable `db_actor::block_on` calls; the selected
  history bridge and old direct WAL replay call remain absent.

## Blocking findings

### 1. Cancel before request handoff bulk-drops the full preallocated reservation

`ArtifactHistoryAdmission` owns `Option<HistoryReplayReservation>`. The full reservation is created
before `ArtifactHistoryWorkOwner::Request` reaches the actor. Cancel, stale generation, rejected
worker scheduling, or future abandonment can terminalize that still-unhanded `Request`.

`ArtifactHistoryState::close_one` consumes the zero-sized terminal `Request` but never transfers the
admission's reservation to a retirement cursor. Once the shallow terminal result is removed,
`finish_if_terminal_empty` calls `admission.take()`. The local option then ordinarily drops
`ArtifactHistoryAdmission`, whose `Drop` releases only aggregate slot counters; its still-populated
reservation is implicitly destroyed afterward. That can synchronously free 960 16-KiB result pages,
the two fixed vector backings, the range/entry backing, and scratch allocation in one caller turn.

This violates exact cancellation/close handback and the claimed one-owner-per-grant terminal
contract. There is no admission-reservation close cursor, terminal witness, or fail-safe shallow
shell. The ordinary reservation constructor has the same issue on a partial allocation failure:
after allocating up to 959 result pages, an allocation error propagates and releases all partial
owners in one stack unwind.

### 2. A caught replay panic can permanently bypass `FaultRetire`

`HistoryReplayFuture::next` first replaces `self.phase` with `HistoryReplayPhase::Complete` and then
steps the moved local phase. A panic anywhere after that replacement can therefore leave the stored
phase as `Complete`. `ArtifactRunner::run_turn` catches the panic and calls
`replay.request_close(...)`, but `request_close` only records `terminal_error`; it does not force the
phase to `FaultRetire`.

The next poll matches `HistoryReplayPhase::Complete` and returns `Err(DbError::Closed)` immediately,
without calling `close_step`. `replay.terminal_is_empty()` remains false while source/result owners
remain, so the runner retains the turn and schedules it again. Every subsequent poll repeats the
same `Complete` branch. The actor neither returns ownership nor reaches terminal-empty, and the
retained page graph is stranded indefinitely. This contradicts the report's claim that panic
requests retained fault retirement before runner finish.

### 3. Close/accounting helpers still perform hidden full-capacity traversals

Even where one owner is released, `HistoryReplayReservation::close_step` locates it with
`source_pages.iter().rposition`, so repeated retirement of a full 1,024-page reservation performs a
cumulative quadratic scan over trailing empty slots. `retained_bytes` traverses all 960 result pages
whenever segment admission is checked, `finish_view` traverses the complete result-page set again,
and terminal predicates scan all fixed source slots. These iterator traversals evade the verifier's
literal `while`/`loop` check. They are finite, but they are not the claimed retained one-owner/item
cursor and have no deadline evidence. This independently keeps the hard worker-turn claim RED.

## Fixture and mutation audit

- The deterministic empty/two-batch fixture is the only semantic end-to-end replay fixture.
- `artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each` does not produce a
  backend, token, or CRC fault in `HistoryReplayFuture`. It manually fills
  `HistoryReplayReservation::source_pages`, calls the reservation helper directly, then checks fault
  message substrings. It therefore cannot expose the panic/`Complete` state or actor handoff.
- The cancel-before/during/after, terminal-work, runner-close, and future-handoff fixtures are
  `include_str!` predicates. They do not instantiate cancellation before `Request` transfer, inspect
  reservation pointers across terminal close, inject a replay panic after phase replacement, or
  prove terminal-empty.
- The boundary/+1 fixture meaningfully checks fixed capacities and preserves the first result-page
  pointer across result-byte rejection, but it neither closes the full rejected reservation nor
  tests allocation failure.
- The verifier's `runner-history-drop` mutation only replaces a source substring; the accepted
  synthetic source requires `replay.request_close` and `!replay.terminal_is_empty()` but contains no
  phase-transition semantics. The bulk-page mutation changes `pop` to `clear` but does not cover the
  outer admission Drop. Consequently both permitted verifier runs pass while accepting the two
  blocking paths above.

## Gates run

| Gate | Independent result |
| --- | --- |
| Rust-2021 scoped rustfmt check on replication codec, DB artifact, DB engine, DB CLI, and DB facade | PASS; no diagnostic |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS; exit 0; DENY clean; one allowlisted finding |
| `bun ./📜️script.ts verify interactivity --format json` | PASS; exit 0; same DENY baseline |
| fixed-owner/read/fault source predicate | PASS for fixed backing, no live `?`/reserve/while, <=16-KiB reads, and `FaultRetire`; independently also detected both blocking paths |
| production engine wait census | PASS; exactly six executable production calls |
| scoped and whole working/staged/`HEAD` whitespace checks | PASS |
| source diff inspection | Completed; the six scoped sources total 3,149 insertions and 282 deletions relative to `HEAD`, with unrelated concurrent changes preserved |
| Rust semantic fixtures, Cargo, Nx, Wasm, browser, network, root lint, runtime/timing | Not run; prohibited |

## Required remediation boundary

1. Move the pre-handoff `HistoryReplayReservation` into a public retained admission-retirement
   owner. Cancel/stale/saturation/abandon/construction-fault paths must detach and retire exactly one
   page/range/entry/scratch/backing scalar per grant before admission credit release; ordinary
   `ArtifactHistoryAdmission::Drop` must be shallow and terminal-witnessed.
2. Make runner panic atomically force a publication-ineligible `FaultRetire` state regardless of
   where the panic occurred. Retain the moved phase owner safely, then poll cancellation until exact
   replay terminal-empty before replying, restoring the engine, or finishing the actor.
3. Replace `rposition` and full result-page accounting/terminal scans with retained indices and
   incrementally maintained exact byte/item counters. Add a deadline/fuel witness to each close and
   finalization grant.
4. Add live hostile fixtures for cancel before first actor handoff with all 960 result pages,
   partial reservation-construction failure, panic in each replay phase after stored-phase
   replacement, backend/token/CRC faults with 1,024 real page owners, terminal resume/close, exact
   pointer handback, and terminal-empty/admission-release ordering. Mutations must remove the actual
   state transitions rather than only their names.

## Residual status

P1t remains source-**REJECTED**. Phase 1 also remains RED for the six named production DB-engine
wait groups, backend `segment_len`/read syscall duration, compiler-generated future step duration,
compilation and runtime behavior, saturation/fairness timing, cancellation/terminal timing, and the
native/Wasm/browser/platform matrix.
