# P1z DB Sync Hello Retained Job Caller Census

Date: 2026-08-23  
Status: **PRE-EDIT SOURCE CENSUS.** P1z is not implemented or accepted by this report.

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
