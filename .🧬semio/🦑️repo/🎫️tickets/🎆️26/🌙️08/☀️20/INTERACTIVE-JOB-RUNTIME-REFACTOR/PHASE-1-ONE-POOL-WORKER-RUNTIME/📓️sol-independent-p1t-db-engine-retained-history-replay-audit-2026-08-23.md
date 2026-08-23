# Sol Independent P1t DB Engine Retained History Replay Audit — 2026-08-23

## Audit Admission

The coordinator requested an independent Sol High audit of the P1t retained history replay packet.
Terra admission remained scheduler-limited. This audit has no P1t implementation authorship, made
no production edits, and treated the P1t/P1s implementation reports and prior P1s audits as claims
to verify rather than acceptance evidence.

Reviewed evidence:

- `📓️p1t-db-engine-retained-history-replay-2026-08-23.md`;
- `📓️p1s-db-engine-vcs-retained-bridge-2026-08-23.md` and all three independent P1s
  audit/re-audit reports;
- the current replication CRC codec, DB artifact actor/replay, DB engine outer authority, and root
  interactivity verifier sources and diffs; and
- working, staged, and `HEAD` whitespace diffs.

## Verdict

**REJECT — source-only P1t retained history replay.** The selected live history `block_on` is
removed, the production engine census is exactly six, and the normal replay route contains a useful
retained page/token/item cursor foundation. The packet is not source-acceptable because terminal,
fault, and actor-close paths can synchronously destroy the entire retained owner graph, a
completion/abandon race can strand ownership and admission forever, and claimed fixed result/scratch
ownership still uses fallible dynamic allocation after retained work has begun.

Phase 1 remains RED. In addition to these P1t defects, the six named engine bridges, backend syscall
latency, compilation/runtime evidence, and native/Wasm/browser/platform timing matrix remain open.

## Reachability and Engine Census

The live route is structurally present and contains no replay blocking bridge:

1. `ArtifactHandle::history()` returns `HistoryFuture::submit(self)`;
2. the outer history authority advances `Request` to
   `authority.history_retained(generation, cancelled)`;
3. `ArtifactAuthority::history_retained` sends `ArtifactMessage::History`;
4. the artifact actor starts the turn and awaits
   `engine.history_replay(operation_generation, cancelled)`; and
5. `ArtifactRunner::run_turn` polls the retained turn future once per accepted actor grant, while
   the outer authority polls its retained ask future once per I/O-lane grant.

The production-only DB-engine source contains exactly six `db_actor::block_on` sites:

1. storage capabilities during open;
2. catalog-root read during open;
3. initial catalog-root CAS during open;
4. create-document catalog CAS;
5. compact-document; and
6. sync hello.

The former `replay_history` definition and direct
`db_wal::replay_document(&storage.wal, ...)` history bridge are absent. Later `block_on` matches in
the same file are under tests. The six remaining production groups are named Phase 1 residuals, not
part of this verdict.

## Normal-Path Retained Structure

The following source claims are verified for the non-faulting path:

- the outer authority has eight generation-keyed slots, reserves 2,048 16-KiB pages (32 MiB) and
  20,481 items per operation, and caps aggregate admission at 256 MiB;
- authority generation is revalidated before the history mailbox handoff;
- weak generation wakers and the `scheduled` compare/exchange coalesce readiness;
- a rejected WorkerPool job is recovered through `error.into_job()`, retained in `retry_job`, and
  retried through the process timer callback with a retry generation;
- dense WAL probing retains one `segment_len` future and increments the segment only after page
  retirement;
- segment length is checked before source-page allocation, with at most 1,024 pages and 16 MiB of
  source bytes;
- each WAL read requests `ByteRange { offset, len: requested }` with `requested <= 16 KiB`, and the
  returned owner must have exact length and capacity no greater than 16 KiB;
- the simultaneous ledger checks segment bytes, page-owner backing, one 16-KiB scratch allowance,
  and retained results against 32 MiB;
- `Crc32cCursor` retains CRC state and consumes at most one admitted page per frame-cursor poll;
- the frame tokenizer advances one length byte, scalar token, CRC page, trailer byte, or finish
  check; commands are numeric page ranges rather than cloned raw frames;
- the envelope/frontier cursors advance one field, dependency, clock item, or scalar opportunity per
  poll; and
- successful segment retirement pops one page per actor grant, successful mapping takes one source
  entry per outer grant, and dense ordering is segment/frame/FIFO.

These positives do not establish safe termination because the same owners have bypass paths below.

## Rejection Findings

### 1. Outer terminal close bulk-drops retained work and results

`ArtifactHistoryState::close_one` takes and immediately drops a whole terminal job, retry job,
`ArtifactHistoryWorkOwner`, or `ArtifactHistoryOutcome`. `ArtifactHistoryTerminalWork::close`
likewise drops its whole owner, and `HistoryFuture::Drop` invokes `close_one` directly in the caller's
destructor.

An `ArtifactHistoryWorkOwner::Actor` can retain the actor ask/turn and its complete
`HistoryReplayFuture`. A `Map` retains the remaining source `IntoIter` plus destination `Vec`, where
each source entry can own an operation-ID vector and Strings. A result can retain the complete
`HistoryView`. Taking one enum owner is not one nested page/item retirement grant. These paths
therefore bypass the cursorized normal/cancel retirement contract.

### 2. Completion followed by abandonment strands ownership

Normal completion is placed in `completion` while `abandoned == false`. If the consumer drops the
future after completion was produced but before it polls, `HistoryFuture::Drop` sets `abandoned` and
`cancelled`, schedules another turn, and calls `close_one`; it does not transfer the already present
`completion` into `terminal_result`.

The cancellation turn may add a second terminal result, but the original completion remains. The
future that could poll it no longer exists, `close_one` never consumes `completion`, and
`terminal_is_empty` therefore never witnesses empty or releases the admission. Eventual ordinary
state destruction is not an exact terminal handback.

### 3. Replay errors ordinary-drop up to a whole segment and result graph

`HistoryReplayFuture::next` first replaces its phase with `Complete`, then uses `?` and direct error
returns from phases that locally own the retained graph. Examples include:

- a `PageRead` backend error or returned-page size/capacity fault while `pages` owns every prior
  page;
- `cursor.step(&pages)?` in `Frame` while the page-set `Arc` owns up to 1,024 pages;
- `cursor.step()?` in `Envelope` and `Frontier` while their page-set owns the segment; and
- result item/byte/reserve faults after partial `pending_operation_ids` or `entries` construction.

Those exits do not transfer ownership into `CancelRetire` or a fault-retirement phase. The local
phase and then the completed future are ordinarily dropped, which can release up to the admitted
16-MiB source graph plus retained result owners in one actor callback. The cancellation conversion
also returns an error from `Arc::try_unwrap` after removing the prior phase, so an unexpected page
lease can take the same deep-drop route.

### 4. Actor close and panic paths can destroy the active replay turn

`ArtifactRunner::finish` takes the active `turn`; the local owner is then dropped. `run_turn` also
takes and drops the turn on panic. A history turn contains the async frame that awaits
`HistoryReplayFuture`, so actor cancellation, address close, or panic can bypass page/result
retirement. The outer actor-terminal-job API does not transform the inner history future into a
page cursor before this destruction.

### 5. Result and scratch ownership is not fixed or pre-admitted as claimed

`HistoryPageSet::copy_small` allocates a new `Vec::with_capacity(len)` for each copied text field.
The 16-KiB cap bounds one allocation but does not provide a retained fixed scratch owner or exact
terminal handback.

The replay dynamically grows `pending_operation_ids` and `entries` with `try_reserve(1)`, then
accounts newly observed capacity. The engine outer mapper calls `entries.try_reserve_exact` before
checking combined derived item/byte credit. Thus allocation can occur before the claimed derived
preflight. The aggregate caps may reject subsequent publication, but they do not prove fixed owner
backing, rejection before allocation, or cursorized disposal after failure.

Rejected outer admission also clones `handle.document` while constructing `ArtifactHistoryState`
after `try_claim` has failed. This is another pre-admission ownership/materialization gap unless the
handle's String backing is separately proved and credited at this seam.

### 6. Fixtures and verifier do not discriminate these defects

The empty/two-batch async fixture is the only meaningful replay integration fixture. The cap,
generation, wake, cancellation, ordering, and terminal fixtures predominantly inspect
`include_str!` substrings. They establish the presence of names and selected ordering, but do not
exercise:

- malformed frame/envelope/frontier faults with a full page set;
- backend failure after partial page acquisition;
- abandonment after completion but before poll;
- actor close/panic while the history turn owns pages;
- terminal work/result closing with maximum nested owners;
- allocation failure after partial replay result construction; or
- exact admission release and terminal-empty witnesses for each path.

The root verifier similarly requires `CancelRetire`, `pages.pop`, terminal API names, and selected
source fragments. It does not reject `drop(work)`, `drop(result)`, actor-turn destruction, error
returns that bypass retirement, dynamic scratch/result allocation, or the stranded-completion race.
Its authorized self-test passing therefore does not repair the source ownership failures.

## Gates Run

| Gate | Independent result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on replication codec, DB artifact, and DB engine | PASS; no diagnostic |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS; exit 0, DENY clean, existing allowlisted findings only |
| `bun ./📜️script.ts verify interactivity --format json` | PASS; same DENY baseline |
| production engine blocking census | PASS; exactly six named `db_actor::block_on` sites |
| isolated history-route forbidden scan | PASS for `block_on`, blocking ask/submit, thread/pool creation, polling/drain loops, whole-segment reads, whole-frame CRC, and raw-frame command cloning |
| scoped working/staged/`HEAD` whitespace checks | PASS |
| whole working/staged/`HEAD` whitespace checks | PASS |
| source diff inspection | Completed; the four packet sources span 2,271 insertions and 75 deletions relative to `HEAD`, alongside unrelated shared-worktree edits |
| Cargo/Nx/Wasm/browser/network/root lint/runtime/timing | Not run; prohibited for this audit |

Rust fixtures and verifier mutations were inspected but not compiled or executed. The permitted
interactivity self-test executed the root source verifier mutations; those mutations do not cover
the rejection cases above.

## Required Remediation Boundary

Source acceptance requires:

1. a fault/cancel/close phase that receives every active replay phase before returning any error and
   retires one page/result/nested owner per accepted grant;
2. an actor-close protocol that cancels, polls, and cursor-retires an active history turn before the
   runner becomes terminal;
3. terminal work/result APIs whose close operations advance retained cursors rather than dropping
   whole owner enums;
4. atomic completion-to-terminal handback on abandonment, including the completion-before-Drop
   race, with exact admission release;
5. fixed or exactly pre-admitted scratch/result backing, with checks preceding allocation and exact
   rejected-owner retention; and
6. adversarial fixtures and verifier mutations for every fault/abandon/actor-close path at maximum
   admitted ownership.

## Residual Status

P1t is source-REJECTED. Phase 1 also remains RED for the six production DB-engine wait groups, the
already documented backend/platform `segment_len` and at-most-16-KiB read syscall duration,
compiler-generated future step duration, full compilation and runtime behavior, saturation/fairness
timing, cancellation/terminal timing, and the native/Wasm/browser/platform matrix.
