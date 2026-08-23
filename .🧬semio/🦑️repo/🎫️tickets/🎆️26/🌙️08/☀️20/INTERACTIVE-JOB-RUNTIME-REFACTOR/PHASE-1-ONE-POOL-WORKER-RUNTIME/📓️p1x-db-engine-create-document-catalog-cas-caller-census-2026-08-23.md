# P1x DB Engine Create-Document Catalog CAS Caller Census

Date: 2026-08-23  
Status: **PRE-EDIT SOURCE CENSUS.** P1x is not implemented or accepted by this report.

## Selected Production Wait

The P1x cut is the `Database::create_document` catalog mutation block. The current path holds the catalog mutex while it:

1. scans for a duplicate document;
2. clones the complete catalog entry vector and every document-id `String` backing;
3. appends the new entry;
4. clones document ids again into a temporary serialization vector;
5. serializes the complete catalog root into a fresh byte vector;
6. transfers that backing into `DbIoPages`;
7. synchronously drives `cas_root(epoch, pages)` on native targets (or awaits it on Wasm);
8. replaces the authoritative in-memory vector after CAS success.

Only step 7 is the literal remaining production `db_actor::block_on` bridge, but P1x cannot claim an 8 ms bounded cut if it merely moves the wait and leaves the full catalog scan/clone/encode inside one caller or worker grant.

## Caller Reachability

`Database::create_document` is one public method used by the DB CLI's open-or-create flow, the DB facade, DB engine tests, and the DB testkit/replay harness. All implementations and targets share this one catalog mutation route, with only the native/Wasm wait syntax currently split by `cfg`.

## Required Transaction Boundary

The implementation must preserve one linearizable catalog mutation without retaining a synchronous mutex guard across suspension. A viable owned state machine must carry:

- the requested `ArtifactId` and its exact `String` capacity;
- the selected base catalog epoch and immutable revision/snapshot identity;
- bounded cursors for duplicate detection and catalog-root encoding;
- every allocated catalog entry/id/page backing in an exact byte/item ledger;
- the storage owner and backend CAS future/result;
- a commit/revalidate step that publishes the new epoch/entries only if the in-memory base still matches the selected revision;
- a deterministic retry or explicit `Fenced`/conflict result when either the backend epoch or in-memory base has advanced.

The implementation must not clone the whole dynamic catalog before admission. Catalog sharing, incremental clone/encode, or ownership transfer must make every allocation and copy observable and bounded. It must also avoid spawning an authority or emitting `document_created` until durable catalog publication is accepted.

## Cleanup And Freshness Obligations

Admission failure, queue saturation, delayed callback retry, cancellation, stale generation, worker panic, CAS fencing, and in-memory revalidation failure must preserve or incrementally retire every owned entry/id/page/storage/result. Close must release at most one dynamic backing or fixed bounded unit per grant and expose terminal-empty. No dropped retry intent, silent command loss, nested executor, unbounded loop, recursive owner drop, or len-as-capacity accounting is acceptable.

## Verification Obligations

Permanent fixtures and the verifier must prove:

- after P1w and P1x the production wait census is exactly two: compaction and sync hello;
- native and Wasm use the same retained state machine rather than separate semantic paths;
- concurrent create requests cannot publish duplicate documents, lose a winning catalog, or overwrite a newer epoch;
- success publishes the exact returned epoch and only then spawns/registers the authority;
- stale/fenced/cancel/panic/admission/saturation paths preserve exact owners and reach terminal-empty incrementally;
- low fuel/deadline forces repeated yields during scan, copy, encode, publication, and close;
- item and byte ledgers use allocated capacity/backing ownership for every nested dynamic value;
- completion publication follows check-register-recheck and slot generations reject ABA-stale callbacks.

The backend `cas_root` poll remains one explicit Phase 9 indivisible-latency residual. Native, Wasm, browser, stress, and timing validation remains deferred to the serialized build matrix after overlapping Rust source packets are quiescent.
