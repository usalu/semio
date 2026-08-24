# Sol High P1w DB Engine Initial Catalog CAS Implementation

Date: 2026-08-24  
Disposition: **SOURCE IMPLEMENTED — ready for independent audit; executable matrix remains deferred.**

## Exact Caller Cutover

The fresh-storage `None` branch of `Database::open_with` now writes the empty catalog directly into retained `DbIoPages`, mounts `DatabaseCatalogBootstrapFuture` on the supplied shared `WorkerPool`, awaits its public check-register-recheck future, and consumes the exact storage/key/expected/actual result. `Database::open`, `open_at`, `open_with_emit`, and `open_with_authz` continue to converge on this one private path. The selected production `db_actor::block_on` census is three: create-document CAS, compaction, and sync hello.

## Counterexample-to-Fix Map

| Counterexample | Implemented production closure |
| --- | --- |
| Submit before credit or malformed input destroys owners | A checked 64-slot admission ledger claims page/result item and byte credit before state allocation, registry publication, or scheduling. Maximum pages, aggregate totals, and generation increments are checked. Refusal returns the identical storage/pages/key/fence inside a prepared close authority. |
| Backend CAS runs on the constructor/facade executor | Handoff and polling are distinct persisted phases. Every drive and retirement callback is submitted only through the supplied pool's typed `Lane::Io`; the caller only polls the publication future. |
| Cancel between handoff and first backend poll drops raw pages | `DatabaseCatalogBootstrapWork::new` retains storage and pages explicitly with no future. Its first `Lane::Io` poll transfers them into the backend future. Before that transfer, close advances exactly one retained page or control owner per opportunity. |
| Pending/Ready/panic loses the polled future or terminal value | The polling gate takes one work owner, performs one caught poll, stores Pending work or Ready work plus actual result, then releases the gate. Cancellation during polling records one wake request instead of scheduling a concurrent poll. |
| CAS mismatch retries `EpochFence::INITIAL` | Validation uses checked `expected.epoch + 1`; a mismatch publishes the exact `DbError::Fenced` once and never retries the semantic CAS. |
| Queue saturation drops the callback | `WorkerSubmitError::into_job` is retained with a checked bounded retry-pressure witness and resubmitted by the shared pool callback path. |
| Lost future/result has no recovery authority | A generation-qualified fixed registry exposes exact take/resume/result/witness/close authorities. Result Drop allocates nothing: it hands its exact owner back to the already-admitted state. Success releases admission only when `into_parts` transfers every owner. |
| Rejection/result Drop allocates or destructures leaf owners | Rejection construction prepares its fixed close authority around the intact storage/pages/key ledger. Retry/handback takes and restores that ledger transactionally. Drop only schedules the prepared authority. Result Drop stores `state: None` ownership into the same terminal slot and schedules retained retirement. |
| Terminal cleanup drains an owner graph in one caller call | Public close only schedules. `RetainWork`, `CloseWork`, `RetireInput`, validation, publication, and terminal retirement persist their cursors; each worker opportunity polls once or releases one page/future/result/control owner. |
| ABA/stale callback releases a replacement | Slot, generation, and byte identity are rechecked. Stale callbacks publish a qualified stale-generation result and cannot release a replacement slot. |

## Production Authority Ledger

- fixed 64 operation slots;
- at most eight retained input pages per operation;
- eight fixed semantic/control items per operation;
- actual retained page backing plus a fixed 16 KiB result/retirement reserve;
- fixed `[u8; 16]` catalog key, expected fence, checked actual fence/error, storage owner, backend-work owner, exact retry job, waker, terminal result, and admission witness;
- explicit prepared rejection close and generation-qualified terminal registry; no `unwrap`, `expect`, `String`, `Vec`, loop, nested pool, thread spawn, or blocking bridge in the P1w production region.

## Hostile Law Evidence

The live Rust fixture bodies cover:

1. aggregate maximum, MAX+1, page MAX+1, and ABA replacement identity;
2. real refusal storage/page-operation/key/fence identity and terminal close;
3. exact shared-worker thread/lane success and epoch;
4. exact fenced mismatch with no retry;
5. Ready, Fenced, panic, and Pending interruption;
6. cancellation after Handoff but before first poll with explicit unpolled storage/pages retention;
7. lost handle take, resume, bounded close, and terminal witness;
8. backend no-service close on the I/O worker;
9. stale-generation storage identity and replacement-slot preservation;
10. real queue saturation, retained-job identity, and recovery;
11. deterministic replay and initial-fence single-winner behavior;
12. publication-before-waker and successor-pressure race closure;
13. public result Drop handback, terminal recovery, exact owner identity, and admission release.

The isolated P1w verifier inspects the production caller and state machine, all thirteen law bodies, prohibited patterns, exact pre-admission ordering, one-poll ownership publication, check-register-recheck, retirement placement, prepared rejection/result Drop behavior, and unpolled-work ownership. Its 37 hostile mutations restore or remove each prohibited shape/law dependency; every mutation is rejected before the faithful source fixture and live tree are accepted.

## Evidence Run

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1w` | PASS — `live-source and hostile mutations clean` |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS — accepted P1q source/mutations preserved |
| `rustfmt --edition 2021 --check --config skip_children=true` on DB engine | PASS |
| P1w production prohibited-pattern sweep | PASS — no matches |
| Scoped unstaged `git diff --check` on engine/script | PASS |

Cargo, Nx, Wasm, browser, native/release, runtime, worker-count replay, allocation pressure, and timing tests were not run, as required while overlapping source packets remain active. This report claims source/static readiness only; the serialized final matrix remains mandatory.
