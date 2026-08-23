# P1t DB Engine Retained History Replay — 2026-08-23

## Status

The independently rejected P1t source packet has been remediated and is ready for focused source re-audit. This report does not claim P1t or Phase 1 acceptance; the runtime matrix and six other production DB-engine wait groups remain open.

## Caller and reachability census

- Removed definition: `db_engine::replay_history`, formerly the one production `db_actor::block_on(db_wal::replay_document(...))` bridge selected from the seven-call P1s residual census.
- Live API: `ArtifactHandle::history`. It is public and therefore UI/product/plugin reachable even though the only authored production call is the DB CLI history command.
- Authored production caller: `db/⌨️cli/🦀️component.rs`, `handle.history().await`.
- Test callers: the DB-engine round-trip/history fixtures and the DB facade fixture.
- Production DB-engine `db_actor::block_on` census is now exactly six:
  1. storage capabilities during open;
  2. catalog-root read during open;
  3. initial catalog-root CAS during open;
  4. create-document catalog CAS;
  5. compact-document;
  6. sync hello.
- The selected history group is absent, so the accepted P1s seven-call census is reduced by exactly one. The six named groups remain independent residuals.

## Retained architecture

- `ArtifactHandle::history()` returns `HistoryFuture`; no synchronous compatibility wrapper remains.
- A fixed eight-slot, generation-keyed admission authority reserves 16 KiB pages, 32 MiB per operation, 256 MiB process aggregate, and 20,481 maximum simultaneous source/derived item owners.
- The operation moves through exactly one request handoff or one actor ask poll per process-pool I/O-lane grant. The former outer result mapper and document/operation-ID clone graph are deleted: the fixed artifact result moves directly into `HistoryView` with its admission lease.
- Stale handle generation and operation generation are checked before mailbox mutation. A weak generation waker coalesces wake storms, and exact rejected WorkerPool closures use the process timer wheel with bounded retry.
- Terminal job, work, result, and actor-terminal-job owners have public take/resume/close paths. `ArtifactHistoryTerminalHandle` and the fixed process registry retain abandoned completion/work ownership after a `HistoryFuture` or returned `HistoryView` is dropped; a later `ArtifactHandle::history_terminal(generation)` can resume, take, or close it.
- `ArtifactMessage::History` carries the operation generation, shared cancellation bit, and exact preadmitted replay reservation into the document actor. `ArtifactTurn::History` retains the engine, replay cursor, and reply separately, polls the replay once per accepted UserVisible grant, and refuses runner finish while replay ownership is nonempty.

## Bounded WAL replay cursor

- Dense segment discovery uses one retained `segment_len` future at a time; it does not allocate an unbounded segment-index list.
- `HistoryReplayReservation` preallocates fixed backing before request transfer: 1,024 source owner slots, 960 result pages of 16 KiB, 8,192 operation-range slots, 4,096 entry slots, and one 16 KiB scratch page. Result text is stored as admitted page ranges; live replay performs no `String` construction, `Vec` growth, or reserve call.
- A segment is admitted as at most 1,024 fixed 16 KiB pages. Every storage read requests one `ByteRange { offset, len: requested }` with `requested <= 16 KiB`; no whole-segment read exists on the history route. Exact fixed reservation backing plus the current segment length is checked against the 32 MiB operation grant before the first read.
- `protocol::codec::Crc32cCursor` preserves CRC-32C state across pages. The retained frame cursor advances one body-length byte, scalar token, CRC page, trailer byte, or finish check per poll; no full-frame CRC call remains.
- Raw command frames stay as numeric ranges over admitted page owners. The envelope cursor consumes one text/range/dependency/clock field per poll; an operation ID is UTF-8 validated in the fixed scratch page, then copied by a retained source/destination page fragment into the preadmitted result pages. Opaque diff/inverse byte ranges are validated without copying.
- Frontier decoding is a retained document/scalar/hash cursor. Segment/frame/result order remains stable FIFO; entry metadata points to stable operation-range spans in the result page set.
- Every tokenizer, CRC, envelope, frontier, range, backend, cancellation, and close error transitions to `HistoryReplayPhase::FaultRetire`. It releases one rejected page, source page, range, entry, result page, or scratch owner per grant before returning the terminal error. `HistoryReplayFuture::Drop` asserts terminal-empty instead of providing a bulk destruction bypass.
- Successful views retain the exact outer admission lease. `HistoryView::close_step` releases one nested owner per call, while ordinary view Drop atomically transfers the exact view and lease back to the public terminal registry rather than destroying pages.

## Independent rejection remediation

The 2026-08-23 independent audit rejected six paths. Their source remediation is now explicit:

1. Outer `close_one` no longer drops a whole work/result graph. Actor work is returned to the live slot under cancellation and polled to completion; result close advances its nested page/range/entry cursor.
2. `complete` and `HistoryFuture::Drop` serialize on the completion mutex. Drop sets abandonment while holding that mutex and atomically transfers an already completed result to terminal ownership, closing the completion-before-Drop race.
3. Replay phases no longer own source/result page vectors or `Arc<HistoryPageSet>` leases. Page/result graphs are future-owned, phase variants carry only scalar cursors or one backend future, and all live `next` errors are matched into `FaultRetire`; no `?` exists in `next`.
4. `ArtifactRunner::finish` cannot take a nonempty history turn. Direct terminal close uses a retained close-driving grant with wake suppression, preserving the exact rejected job until active history reaches terminal-empty; panic requests replay fault retirement instead of taking the turn.
5. `copy_small`, per-ID `String`, pending-ID growth, entry growth, and outer Map allocation are removed. Allocation occurs only in the fixed reservation constructor before mailbox ownership transfer.
6. New semantic fixtures and verifier mutations cover completed-result Drop handback, future/terminal handle handback, 1,024-page backend/token/CRC fault retirement, scratch/result boundary and +1 rejection, and runner close during an active history turn.

## Honest indivisible residuals

- Each requested `segment_len` and at-most-16-KiB storage `read` remains one backend/platform operation. Fs/SQLite latency inside that bounded request is the already documented P1q/Phase 9 syscall residual and is not described as an 8 ms guarantee.
- `ArtifactEngine::open_retained` still reaches the separate `db_wal::replay_document` materialization route. It belongs to a remaining DB-engine open group, not this removed `ArtifactHandle::history` bridge.
- Cargo/build/type checking was prohibited for this source packet; source-only type/borrow reconciliation is therefore subject to independent audit and the later permitted runtime matrix.

## Fixtures and verifier mutations

Direct fixtures cover empty and deterministic two-batch replay, operation slot cap/+1 and ABA, fixed reservation capacities, exact result cap/+1 handback, segment/page cap, page-bounded reads, incremental CRC/tokenization, 1,024-page fault retirement, late/quiet wake, retry saturation, cancel before/during/after, stale generation, FIFO publication, completed-result Drop handback, public terminal take/resume/close, and runner close during an active history turn.

The existing root `📜️script.ts` interactivity verifier now rejects:

- a reintroduced history `block_on` or seventh production DB-engine wait;
- poll/drain loops;
- missing fixed source/result/range/entry backing or result-byte preflight;
- raw command cloning;
- whole-segment reads;
- full-frame CRC;
- stale-after-handoff;
- uncoalesced wakes;
- stranded saturation jobs;
- dynamic live-cursor reserve/String construction, `?` fault escape, bulk result/page retirement, or active history turn Drop;
- missing terminal work/fixture authorities.

## Permitted validation

- `rustfmt --edition 2021` on the five scoped Rust sources: clean.
- `rustfmt --edition 2021 --check`: clean.
- `bun 📜️script.ts verify interactivity --self-test`: exit 0; one allowlisted blocking finding, zero unlisted; DENY clean.
- `bun 📜️script.ts verify interactivity`: exit 0; one allowlisted blocking finding, zero unlisted; DENY clean.
- Production scans: selected history route has zero `block_on`, `ask_blocking`, `submit_blocking`, runtime/thread/pool creation, whole-segment read, full-frame CRC, raw-command decode clone, spin, or drain matches.
- Scoped and whole working/staged/HEAD `git diff --check`: exit 0 with no whitespace errors.
- Scoped working diff contains the shared root verifier, protocol CRC cursor, DB engine/artifact, the DB CLI mechanical history-view access, and the DB facade terminal-handle reexport. The shared worktree has substantial unrelated staged/working P3, P8, test-refactor, dependency, renderer, and stdio changes; they were neither edited nor reverted by this remediation.
- No Cargo, Nx, Wasm, browser, network, or root lint command was run.

## Files

- `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs`
- `📜️script.ts`
- this report

## Remaining Phase 1 blockers

- Six named production DB-engine wait groups remain.
- The broader runtime matrix and behavior/build validation remain RED/unrun.
- Existing accepted P1n/P1o/P1p/P1q/P1r/P1s packets require preservation during subsequent work.

## Latest terminal/transition re-audit remediation

The follow-up independent re-audit rejected three source paths: ordinary destruction of a full
reservation before actor handoff, publication of `Complete` before panic-prone phase work, and
hidden full-capacity accounting scans. The focused remediation changes only the retained history
authority, its root verifier region, fixtures, and this report.

- `ArtifactHistoryAdmission` cannot release its generation slot while it still owns a replay
  reservation. Cancel or future Drop atomically recognizes an unhanded `Request`, moves the exact
  reservation into `HistoryReplayReservationCloseCursor`, publishes the cursor through
  `ArtifactHistoryTerminalReservation`, and retains the aggregate grant until the cursor is
  terminal-empty. The public cursor supports exact take, pre-retirement resume, one-owner
  `close_step`, and abandoned-cursor return to the terminal slot. `finish_if_terminal_empty` only
  releases an admission whose reservation has already moved or completed.
- Replay phase ownership is now `Option<HistoryReplayPhase>` guarded by the separate
  `HistoryReplayTransition::{InProgress, FaultRetire, Complete}` state. Normal work borrows the
  active phase in place. A panic therefore leaves that exact phase retained; the runner catch moves
  the guard to `FaultRetire`, closes the actor address, and reschedules one cleanup opportunity at a
  time. `Complete` is written only after fault retirement is empty or a successful result has moved
  out.
- Source-page occupancy, operation backing bytes, and result backing bytes are retained scalar
  counters. Page retirement decrements the occupancy index directly. Segment preflight and final
  view publication use cached exact backing credits and O(1) lengths; production replay contains no
  `rposition` or result/source fixed-capacity terminal/accounting traversal.

New discriminating fixtures exercise a full pre-handoff reservation while all eight operation
grants are occupied, prove that the cancelled grant cannot be reused before one-owner retirement is
terminal, then prove a fresh generation can reuse it. A transition-panic fixture injects a panic
before commit for every replay phase variant, verifies that the active phase remains owned, requests
fault retirement, and drives all reservation owners to terminal-empty. The 1,024-page fault fixture
checks the retained source-page count after every grant, and an exact source predicate rejects the
former capacity scans. Existing boundary, ABA, runner-close, terminal take/resume, and result
handback fixtures remain.

The root interactivity self-test corpus now has adversarial mutations for unhanded admission Drop,
pre-work `Complete`, `rposition`, removal of the retained source counter, and bulk page retirement.
Both `bun 📜️script.ts verify interactivity --self-test --format json` and the corresponding plain
DENY invocation exited 0 with one allowlisted blocking finding and DENY clean. Scoped rustfmt and
rustfmt check on the DB artifact/engine sources, the production wait/replay forbidden scans, and
scoped plus whole working/staged/`HEAD` whitespace checks are clean. Cargo, Nx, Wasm, browser,
network, root lint, and runtime tests remain prohibited and were not run.

This is a source-only packet for independent audit, not Phase 1 acceptance. The six named
production DB-engine wait groups, page-bounded backend syscall latency, compilation/runtime
validation, fairness/timing evidence, and the native/Wasm/browser/platform matrix remain open.

## Final registry-backed construction-fault remediation

The final independent audit rejected the remaining raw reservation-construction escape. The
focused repair makes reservation construction private and registry-owned before its first fallible
allocation:

- `HistoryReplayReservation::try_new` and
  `HistoryReplayReservationConstructionFault` are crate-private. The former public raw
  construction cursor and `into_parts` escape no longer exist.
- A fixed 64-slot, generation-keyed construction registry is claimed before source/result slot,
  result-page, range, entry, or scratch allocation. A private builder owns that token and every
  partial root. Builder unwind transfers the exact error and partial close cursor to its reserved
  slot; it cannot ordinary-drop those owners.
- A checked-out construction-fault handle returns its exact error and cursor to the same generation
  on Drop. A later registry take resumes that owner. Its close path releases one nested page/root
  per grant, then the error scalar, and only then its registry token.
- Registry release validates the exact slot generation, checked-out state, and absence of retained
  error/cursor roots. It clears only generation/occupancy scalars; it does not assign or Drop a
  whole slot. If successful construction cannot perform that release, the fully built reservation
  is converted back into the retained fault cursor instead of escaping.
- `HistoryFuture::submit` registers the construction fault in the public history state before
  taking and publishing its exact error. The admission, byte/item grant, and all partial roots
  therefore remain publicly retrievable while the error is observable.

Discriminating fixtures now discard an unchecked injected construction error, retrieve its exact
three-page owner from the registry, Drop the checked-out handle, retrieve it again, and close it one
page/root per grant. A caught builder unwind with an allocated page proves later retrieval rather
than merely proving absence of panic. A 64/+1 registry test proves rejection before partial
allocation, exact cleanup, fresh-generation reuse, and stale-generation rejection. The existing
zero/one/mid/cap construction-boundary and public admission-final-grant fixtures remain.

The verifier corpus rejects a reintroduced public raw constructor, public construction cursor,
`into_parts` escape, missing builder or checked-out-handle handback, assert-only construction
Drop, infallible scratch allocation, allocation before registry claim, missing registry retrieval,
and removal of the unchecked-error, unwind, or saturation/ABA fixtures.

Permitted final gates:

- Rust-2021 `rustfmt --check` and parser output on replication codec, DB artifact, DB engine, DB
  CLI, and DB facade: clean.
- Interactivity self-test and plain DENY: exit 0; one approved test-only blocking finding and zero
  unlisted findings.
- Production census: exactly six `db_actor::block_on` calls remain in the six named independent
  groups; the retained history route has zero blocking bridge matches.
- Construction scans: zero public raw constructor/cursor/`into_parts` or assert-only construction
  Drop matches; required builder/handle registry handback, take, capacity, and semantic fixtures are
  present.
- Scoped working, staged, and `HEAD` whitespace checks: clean. Whole working/staged/`HEAD`
  checks remain concurrently RED only for unrelated trailing whitespace in
  `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:459` and a blank line at EOF in the Phase 3 raster audit
  report; neither file was edited here.

No Cargo, Nx, Wasm, browser, network, runtime, or root-lint command was run. This packet is
audit-ready source only and is not accepted. Six DB-engine wait groups, bounded backend syscall
latency, compilation/runtime/timing evidence, and the full platform matrix remain open.

## Latest public-terminal and construction-fault remediation

The second independent re-audit isolated two remaining ownership gaps. The focused repair keeps the
accepted replay phase, panic-to-`FaultRetire`, O(1) accounting, one-page backend read, and six-wait
census semantics unchanged.

- `ArtifactHistoryState` now separates `terminal_roots_are_empty` from the public
  `terminal_is_empty` witness. The public witness additionally requires `finished`, an empty
  admission slot, no scheduled closure, and no armed retry. `finish_if_terminal_empty` locks the
  admission transition, drops the exact admission and therefore releases its generation/byte slot,
  unregisters the terminal-registry generation, and only then publishes `finished`.
- Both `HistoryFuture::close_step` and `ArtifactHistoryTerminalHandle::close_step` return immediately
  after one nested owner or actor-authority close opportunity. Admission release is a separate final
  grant after those paths report no progress. A blocked actor or checked-out terminal owner therefore
  leaves `close_step` pending and the public terminal witness false.
- Reservation construction no longer propagates page/range/entry allocation failure through `?`.
  Each failure returns `HistoryReplayReservationConstructionFault` with every source/result/root
  allocated so far moved into `HistoryReplayReservationConstructionFaultCursor`. The matching
  `ArtifactHistoryAdmission` remains generation- and byte-slot-owned in the public history registry;
  its exact `DbError` moves to normal completion/terminal-result ownership.
- `ArtifactHistoryTerminalConstructionFault` provides public take, explicit retained `resume`, and
  one-owner `close_step`. Dropping the checked-out handle returns the exact cursor to the same generation. Result pages are
  popped one at a time, followed by operation-range, entry, source-slot, result-slot, and scratch/root
  backings one per grant. Only the subsequent final admission grant releases capacity. The public
  type is reexported by the DB facade.

The new semantic source fixtures cover all eight admission slots, a dropped/cancelled public future,
false terminality while the eighth slot is retained, the distinct final admission-release grant,
generation unregister/reuse, construction failure at zero/one/mid/cap-minus-one/cap result pages,
and a synthetic exact-owner sweep for every partial page count from zero through the 960-page cap.
The cap-plus-one injection cannot construct a 961st owner and returns an ordinary fixed 960-page
reservation for retained close. The public wiring fixture verifies the construction cursor,
checked-out return path, finished/admission witness, and close ordering.

The root verifier now rejects removal of the public final finish call, replacement of the
admission/finished witness with a constant, restoration of `?`-driven construction unwind, removal
of the retained resume API, and removal of the cap/every-page fixture. The prior 22 mutations remain. The self-test and plain DENY
invocations both exit zero with one approved test-only blocking finding and no unlisted finding.
Scoped Rust-2021 rustfmt and parser checks are clean; selected history source scans contain no
blocking bridge, loop/drain, whole-segment read, full-frame CRC, fixed-capacity scan, construction
bulk-clear, or result-page construction `?` path. Scoped and whole working/staged/`HEAD` whitespace
checks are clean.

No Cargo, Nx, Wasm, browser, network, root lint, or runtime command was run. This remains an
audit-ready source packet, not P1t or Phase 1 acceptance. The six named production DB-engine waits,
backend syscall latency, compilation/runtime validation, and the full platform matrix remain open.

## Construction-registry linear ABA remediation

The independent construction-registry re-audit accepted the pre-allocation fixed registry, private
fault API, retained public delivery, and one-owner close ordering but rejected its unchecked
handback. The former construction token was copyable, and handback unconditionally rewrote the
destination generation/error/cursor without comparing the live slot. A stale duplicate could
therefore displace a newer retained graph.

The focused remediation makes the partial owner graph registry-resident from construction birth:

- `HistoryReplayReservationConstructionToken` is linear and no longer implements `Clone` or
  `Copy`. It contains only the fixed slot index and generation.
- Claim installs the empty source/result/range/entry/scratch owner roots in the selected fixed slot
  before the first `try_reserve_exact`. The builder and checked-out fault carry only the linear
  token; neither can become the final owner of the partial graph.
- Source/result slot reservation, each result-page publication, operation/entry reservation,
  scratch reservation, accounting, and successful reservation extraction operate against that
  exact occupied/checked-out generation. The fallible scratch remains builder-governed.
- Handback uses `get_mut` and compares bounds, occupied state, checkout state, generation, and the
  resident cursor witness before changing only `checked_out`. It never assigns generation, error,
  or cursor owners. A stale, duplicate, out-of-bounds, or otherwise mismatched handback returns the
  exact rejected slot/generation scalar while the registry graph remains untouched and resumable.
- Fault error take and close operate on the exact registry generation. Close still advances one
  page/root/error opportunity per grant. Final release still requires occupied + checked-out +
  exact generation and empty error/cursor before clearing the slot scalar.
- Builder unwind installs its error only after exact slot comparison, then performs checked scalar
  handback. Fault Drop performs the same checked scalar return. Neither Drop moves a page/root or
  overwrites a destination owner.

The new semantic fixture closes an old generation, reuses the same slot, forges the old scalar
identity, and proves stale handback rejection while the replacement page and error pointers remain
exact. It separately rejects an out-of-bounds token, performs a normal handback followed by a
duplicate handback, resumes the current generation, rechecks both pointers, and closes it to
terminal-empty. The existing 64/+1 saturation, unchecked error, checked-out Drop/resume, builder
unwind, every-page boundary, public terminal, panic/`FaultRetire`, and six-wait fixtures remain.

The permanent verifier now rejects a copyable token; non-resident claim roots; removal of bounds,
occupied, checkout, generation, or cursor comparison; unconditional error/cursor assignment;
missing builder/fault handback; and removal of the stale/duplicate semantic fixture. Its focused
mutation matrix contains an independent mutation for every comparison plus restored `Clone + Copy`
and unconditional owner publication.

Permitted source gates for this remediation:

- Rust-2021 `rustfmt --check` on replication codec, DB artifact, DB engine, DB CLI, and DB facade:
  clean.
- `bun ./📜️script.ts verify interactivity --self-test --format json`: exit 0; all focused mutations
  rejected and DENY clean with the one approved test-only blocking finding.
- `bun ./📜️script.ts verify interactivity --format json`: exit 0 with the same clean baseline.
- Exact source predicate: linear token, fixed registry-resident roots, checked non-overwriting
  handback, semantic ABA/pointer fixture, and zero `mem::forget` in the replay region: clean.
- Production DB-engine census: exactly six `db_actor::block_on` calls, all outside retained history.
- Scoped working/staged/`HEAD` diff checks: clean. Whole checks retain only concurrent out-of-scope
  whitespace findings recorded by the final gate output.

No Cargo, Nx, Wasm, browser, network, runtime, or root-lint gate was run. This is audit-ready source,
not acceptance. The six DB-engine waits, bounded backend syscall latency, compilation/runtime/timing
evidence, and full platform matrix remain RED.
