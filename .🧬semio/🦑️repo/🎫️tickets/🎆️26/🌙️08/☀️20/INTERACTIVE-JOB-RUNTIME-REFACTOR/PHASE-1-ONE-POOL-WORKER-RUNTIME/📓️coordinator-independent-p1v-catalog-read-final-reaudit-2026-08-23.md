# Coordinator Independent P1v Catalog Read Final Reaudit

Date: 2026-08-23  
Verdict: **SOURCE ACCEPTED.** Both blockers from the first independent audit are repaired in the current source.

## Scope

- `DatabaseCatalogReadFuture` public completion/waker protocol
- rejected storage/root-key ownership and live `Database::open_with` rejection cleanup
- controlled fixtures and permanent verifier mutations
- exact post-cut production wait census

No Cargo, Nx, Wasm, browser, network, native runtime, or timing gate was run while overlapping Rust source packets remain active.

## Accepted Repair 1: Publication/registration race

The public future now follows a correct check-register-recheck protocol:

1. take/check `completion`;
2. install the current consumer waker;
3. recheck `completion` before returning `Pending`;
4. clear the transient waker when the recheck returns `Ready`.

Worker publication stores completion before taking/waking the registered waker. Every legal interleaving either returns the result from one of the two checks or leaves a registered waker for the publication. No completion-before-registration window can strand the future.

The controlled fixture publishes an exact result owner after the first check and before registration, then proves the same public poll returns that owner without a later wake. The verifier rejects restoring check-before-register behavior.

## Accepted Repair 2: Rejected owner retirement

`DatabaseCatalogReadRejected::close_step` now releases storage and root key in separate observable grants. The live open rejection route does not perform those grants inline: it mounts the unfinished owner onto a retained process `WorkerPool` I/O close state, returns the error, and keeps a rejected submission job for delayed callback retry.

The controlled fixture proves the first governed close releases only the storage owner, the key remains mounted, and the second governed close reaches terminal-empty. The verifier rejects a combined branch and bypassing the mounted close owner.

## Supporting Source Evidence

- The direct catalog-root `db_actor::block_on` bridge remains absent.
- The production DB-engine wait census remains exactly four, in the expected order: initial catalog CAS, create-document catalog CAS, compaction, and sync hello.
- Admission, backend-poll, cancellation/staleness, result handoff, terminal checkout, ABA, and saturation mechanisms accepted by the first audit are unchanged in the repaired packet.
- Scoped formatting and diff hygiene were reported PASS. The repository-wide verifier currently stops later on concurrent Puzzle-fill source drift and emits no P1v finding; no broader green claim is made.

## Residuals

- The backend `read_root` poll remains one explicitly tracked Phase 9 indivisible-latency residual.
- P1w initial catalog CAS, P1x create-document CAS, P1y compaction, and P1z sync hello remain open.
- Executable native/Wasm/browser/runtime/timing validation remains pending for the serialized build matrix.

P1v is accepted only as a source packet. Phase 1 remains RED and its ticket must remain open.
