# P1y DB Compaction Retained Job Caller Census

Date: 2026-08-23  
Status: **PRE-EDIT SOURCE CENSUS.** P1y is not implemented or accepted by this report.

## Selected Production Wait

The P1y facade cut is `Database::compact_document`, which currently synchronously drives one complete `Compactor::run_from_latest_snapshot` pass. That future is not a single bounded I/O wait. It performs a multi-subsystem operation:

1. load the latest snapshot and derive the WAL floor;
2. acquire the per-document compaction lease;
3. replay the complete document WAL;
4. build segment groups/horizons and retention selection;
5. delete selected WAL segments;
6. build payload candidate/live hash sets and delete orphaned payloads;
7. compact every index kind;
8. optionally materialize the snapshot chain, deduplicate and read pages, publish a full baseline, enumerate generations, retain the new floor, and enumerate again;
9. release the compaction lease and publish the report or precedence-correct error.

The existing numeric `CompactionBudget` caps total counts for some collections, but most helpers still execute their entire allowed count within one async poll/grant, allocate dynamic maps/vectors eagerly, and await many backend operations without an explicit resumable state owner.

## Caller Reachability

The direct live facade is `Database::compact_document`. Repository tests also invoke `Compactor::run`, `run_from_latest_snapshot`, `SnapshotConsolidator`, and the lower-level helpers directly. P1y must preserve those batch/test surfaces as adapters over the same job rather than leave a second run-to-completion implementation beside the retained route.

## Required Job Graph

The production compaction owner must be one admitted, cancellable state machine with explicit phases and persistent cursors for every list, record, segment, hash, index kind, generation, descriptor, page, and backend request. Its retained boundary includes:

- exact document and holder `String` capacities;
- the storage owner, lease resource/holder/fence, and release obligation;
- budget/timestamp/consolidation flags;
- replay records and every nested record payload backing;
- segment groups/horizons/selection and their allocated capacities;
- candidate/live hash-table buckets and entries;
- index reports and any retained index merge owners;
- snapshot descriptors, combined archives, page vectors/page byte backings, dedupe tables, generation lists, and publication pages;
- report/error/completion owners and retry intent.

The state machine must prioritize lease release after every partial failure, cancellation, stale generation, or panic. A failed release must not overwrite an earlier run error, matching the existing error-precedence contract; a release error becomes terminal only after successful work.

## Boundedness Gaps In Current Helpers

The following current helpers are whole-operation or eager-allocation boundaries and therefore cannot remain on the live path unchanged: `replay_document`, `group_records_by_segment`, `segment_horizons`, `plan_wal_retention`, `apply_wal_retention`, `sweep_payloads`, `compact_all_indexes`, `materialize_chain`, `collect_chain_pages`, `SnapshotConsolidator::consolidate`, and the generation-list/retain/list sequence. Batch adapters may drive their resumable replacements to completion at entry/test boundaries.

## Verification Obligations

Permanent fixtures and the verifier must prove:

- after P1w, P1x, and P1y the only remaining selected production wait is sync hello;
- low fuel/deadline forces repeated yields through every phase, including map construction, sorting/selection, each delete, each index kind, page union/publication, and close;
- cancellation/staleness is observed within one grant and still releases an acquired lease;
- lease contention, renewal/release faults, storage faults, worker panic, queue/admission saturation, and delayed callback retry preserve exact owners and deterministic error precedence;
- allocated capacity/backing, not logical length or semantic estimates, feeds every item/byte ledger;
- deep terminal cleanup is iterative and releases at most one dynamic backing or fixed bounded unit per governed grant;
- deterministic ordering is stable across worker counts, especially hash-derived payload candidates and page unions;
- no nested executor, recursive dynamic drop, unbounded loop, or full helper invocation remains reachable from the live facade;
- public completion uses check-register-recheck and generation-tagged slots reject ABA-stale callbacks.

Each backend operation may remain one explicit Phase 9 indivisible-latency residual poll. Native, Wasm, browser, stress, and timing validation remains deferred to the serialized build matrix after overlapping Rust source packets are quiescent.
