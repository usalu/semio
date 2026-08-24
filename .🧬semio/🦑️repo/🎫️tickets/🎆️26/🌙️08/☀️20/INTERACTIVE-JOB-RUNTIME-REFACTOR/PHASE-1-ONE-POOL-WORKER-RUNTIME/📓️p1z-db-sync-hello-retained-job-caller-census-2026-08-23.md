# P1z DB Sync Hello Retained Job Caller Census

Date: 2026-08-23  
Status: **IMPLEMENTED; INDEPENDENT SOURCE ACCEPTANCE PENDING.**

## Selected Production Wait

The final selected DB-engine bridge is `Database::hello`, which synchronously drives `db_sync::handle_hello`. The callee currently performs a complete reconnect/bootstrap operation in one future:

1. replay the entire retained document WAL;
2. decode every command and allocate all mutation envelopes;
3. retain a digest vector and concatenate all digests again to derive the chain hash;
4. clone the advertised frontier and decide tail/snapshot/none bootstrap;
5. clone all missing envelopes for a tail response, or read/hash a complete snapshot generation;
6. encode a resume token;
7. clone tail envelopes or snapshot bytes again while lowering the response;
8. eagerly allocate every snapshot chunk and the complete follow-up frame vector;
9. assemble the welcome frame and return all output owners at once.

Removing only the outer `block_on` would leave multiple whole-input scans, complete clones, eager output materialization, and backend calls in indivisible live grants.

The selected P1z production wait is `Database::hello`. Its production path now pre-admits an owned retained authority and has no blocking bridge. The batch `WelcomeResponse` shape is test-only and must drive that same authority; it is not a compatibility path.

## Retained Job Contract

One shared WorkerPool and typed `Lane::Io` own every backend poll, retry, frame-production opportunity, and close opportunity. Each follow-up worker opportunity constructs a fresh monotonic `Instant::now() + 8 ms` grant and checks it before allocation, page copy/transfer, and publication. Snapshot returned-frame output treats the caller chunk size only as a preference: every frame allocation, copy, and transfer is capped at the fixed 4 KiB frame unit even when the caller requests the full 256 MiB budget. Only that fixed unit is pre-debited before frame allocation. Its wire owner is a boxed `[u8; 4096]` plus initialized length, so neither production allocation nor bounded direct decode can create a larger backing or fall back to allocator-dependent `Vec::try_reserve_exact`; the exact observed 4 KiB debit transfers into the returned-frame lease until acknowledgement or retained close. Before snapshot-generation acquisition, a separate fixed reservation pre-debits exactly the maximum `DbIoPages` owner: `DB_IO_OPERATION_PAGES` items and `DB_IO_OPERATION_PAGES * DB_IO_PAGE_BYTES` bytes. `read_generation` cannot own more; the reservation settles to observed `page_count * DB_IO_PAGE_BYTES`, never logical length, and page close/error returns that exact debit. Neither snapshot reservation reaches generic whole-remaining `reserve_allocation`. Each other grant owns one page, record, dependency, hash fragment, frame, or owner opportunity before yielding. Pre-admission reserves fixed item and byte authority before replay or output allocation. Every non-snapshot WAL text, dependency shell, payload, page, envelope, frontier, token, and frame allocation pre-debits all remaining admissible capacity before allocation, then settles to observed actual capacity or incrementally retires the refused owner.

Snapshot output is a backpressured retained stream. Every returned frame is a generation-qualified lease; its exact item and observed-capacity debit stays live until the consumer explicitly acknowledges the frame and the mounted close cursor retires its backing. A later request cannot release or replace that lease. A request may copy one retained 4 KiB unit and may publish at most one `SnapshotChunk`, `SnapshotDone`, tail command frame, or terminal witness. Generation-qualified registry publication rejects stale and ABA callbacks before any frame becomes visible. Cancellation and deadline are checked both before and after every cooperative yield and immediately before every stream demand, allocation, transfer, and next WAL/backend operation. The controlling helper returns timeout whenever `expired` is set, independently of cancellation; the pool deadline callback publishes both expired and cancelled before scheduling the real driver. Panic retains the boxed execution future in a named one-backing quarantine cursor. Page-close errors remain in a typed retry fault authority; registry and admission release are forbidden until follow-up terminal ownership, quarantine, execution owners, item credit, and byte credit are all zero. Refusal close retry is bounded by attempt, deadline, and cancellation; exhaustion retains the exact rejected job and owners behind a generation-qualified discoverable typed terminal witness without recursive callback registration.

The governing one-pool physical boundary is exact. With one permanently non-returning sole OS worker, P1z guarantees discoverable retained ownership and no owner loss only; it makes no cancellation-latency or completion claim. After a finite stall returns, the real `worker_loop` services `callback_at` recovery through typed `Lane::Io`; at two or more workers the reserved service capacity permits timers and close work to progress while one violating worker remains held. No timer thread, second pool, or caller-driven completion is permitted.

## Caller Migration

The engine facade transfers owned document, advertised frontier, session, origin, and chunk-credit identities into `DatabaseSyncHelloFuture`. The hub takes `Welcome` once, awaits `next_frame` once per socket send, explicitly acknowledges each returned-frame lease only after the corresponding send completes, and never holds an eager follow-up vector. Native and Wasm use the same authority and state machine.

## Caller Reachability

`Database::hello` is the direct DB-engine facade. The shared `db_sync::handle_hello`, `replay_sync_state`, `decide_bootstrap`, `build_welcome`, and related helpers are also used directly by sync tests and adjacent frontier-advertise paths. P1z must make batch/test helpers drive the same resumable machinery instead of preserving a second run-to-completion implementation.

## Required Job Graph

The retained hello job must expose persistent phases/cursors for input validation, WAL replay, record decode, incremental chain-hash folding, bootstrap decision, snapshot load/hash, tail selection, resume-token encoding, response lowering, chunk production, and terminal handback/close. Its exact ledger includes:

- storage, document id, optional advertised frontier and every nested `String` capacity;
- session id, origin id, chunk size, operation/generation metadata;
- replay records and every nested command/payload backing;
- decoded mutation envelopes and all nested protocol allocations;
- incremental digest/hash state without a second full concatenation buffer;
- snapshot generation bytes/pages and hash state;
- bootstrap/welcome/follow-up frames and all nested vector/string/byte capacities;
- backend future/result, completion, retry, rejection, and close owners.

Large snapshot output must be a credit-governed stream of retained chunks rather than an eagerly materialized `Vec<ServerFrame>`. Tail commands likewise require item+byte credits and deterministic ordered publication. If the frozen `WelcomeResponse` batch shape remains for entry/test consumers, it must be assembled only by a batch adapter driving the chunk stream outside the interactive route.

## Verification Obligations

Permanent fixtures and the verifier must prove:

- after P1w/P1x/P1y/P1z the selected DB-engine production `db_actor::block_on` census is zero;
- every replay/decode/hash/tail/snapshot/chunk/close phase yields under low fuel/deadline;
- huge WAL, command payload, tail, snapshot, and tiny-chunk cases remain within exact item+byte credits without eager whole-output duplication;
- zero chunk size and other malformed input are rejected only after ownership guards are installed;
- cancellation/staleness, backend fault, worker panic, queue/admission saturation, and delayed callback retry preserve exact owners and publish one terminal result;
- newer generation cannot receive stale tail/snapshot chunks or a stale commit-like terminal frame;
- deterministic byte/frame order and hashes are identical at worker counts 1..N;
- capacity/backing ownership, not logical length or semantic estimates, drives accounting for every nested protocol value;
- terminal cleanup is iterative and releases at most one dynamic backing or fixed bounded unit per governed grant;
- no nested executor, unbounded loop, recursive dynamic drop, eager all-chunk materialization, or whole clone helper remains reachable from the live facade;
- completion publication follows check-register-recheck and generation-tagged slots reject ABA-stale callbacks.

Individual backend polls remain explicit Phase 9 indivisible-latency residuals. Native, Wasm, browser, stress, and timing validation remains deferred to the serialized build matrix after overlapping Rust source packets are quiescent.
