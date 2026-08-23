# P1v DB-engine Retained Catalog-root Read — 2026-08-23

Date: 2026-08-23

## Verdict

**SOURCE-AUDIT-READY, NOT ACCEPTED.** The single catalog-root read bridge in `Database::open_with`
is replaced by a persistent I/O-lane authority. The production DB-engine wait census is four:
initial catalog CAS, create-document CAS, compaction, and sync hello. No Cargo, Nx, Wasm, browser,
runtime, network, or native test ran. The one backend `read_root` poll is an explicit Phase 9 latency
residual, and Phase 1 remains RED.

## Pre-edit evidence

The exact caller/reachability and five-to-four wait census was written before production edits in
`📓️p1v-db-engine-catalog-root-read-caller-census-2026-08-23.md`.

## Retained authority

`DatabaseCatalogReadFuture` owns one non-cloneable `DatabaseCatalogRootKey` and the exact
`Arc<DbBackend>` before constructing the backend future. Its dedicated admission provides:

- 64 generation-keyed slots;
- 8 items and 64 KiB per operation;
- 512 aggregate items and 4 MiB aggregate bytes; and
- exact checked claim/release with stale-generation ABA rejection.

The backend future reacquires the borrowed `CatalogRef` only inside its one owned future and always
returns storage, root key, and `Result<Option<(Vec<u8>, EpochFence)>, DbError>` together. A result
whose retained byte capacity exceeds 64 KiB faults while preserving that exact output authority.

Each process-pool opportunity advances one of `Handoff`, `Poll`, `RetainWork`, `DrainWork`,
`ReleaseWork`, `RetainResult`, `Publish`, or `Terminal`. The real generation waker coalesces Pending
wakes; submission saturation retains the exact job for a delayed retry; cancellation, stale
generation, panic, and output-cap faults publish terminal state before polling release. Normal and
terminal work close one future/root owner per opportunity.

Public APIs expose rejected retry/one-owner close, progress/cancellation, abandoned terminal take,
terminal work/retry resume, terminal-result checkout/take/resume/close, checked-out Drop handback,
and terminal emptiness. The DB facade reexports the complete owned surface.

## Live open cutover

`Database::open_with` now performs:

1. the accepted retained capability probe;
2. `open_catalog_read_retained(pool.clone(), storage)`;
3. awaited storage/key/root handback;
4. existing catalog decode when a root exists; or
5. the untouched initial catalog CAS when no root exists.

No compatibility or synchronous catalog-read route remains. The adjacent CAS is intentionally still
one of the four Phase 1 residual waits.

## Fixtures and verifier

Permanent source fixtures cover:

- item +1, byte +1, 64/+1 saturation, exact aggregate credit, slot reuse, and ABA;
- a live memory-backend success with exact storage pointer, root-key handback, and empty-root result;
- controlled Pending/Ready/panic futures with the production scheduled callback, real during/late
  wakes, one exact successor, no terminal repoll, and retained work/result owners;
- cancellation/staleness plus exact rejected storage/key close; and
- terminal-result checkout Drop handback and exact result storage identity.

The root verifier checks the four-wait census, retained live call, fixed credits, all eight phases,
one backend await/poll, panic safety, result cap, exact storage/key return, waker/retry ownership,
cancel/stale publication, public terminal APIs, and meaningful fixtures. Seventeen mutations restore
the block, add a fifth wait, remove caps/phases/freshness/owner handback, clone the key, drop a
saturated job, mask wake scheduling, or bypass successor execution.

## Permitted gates

- Edition-2021 scoped Rustfmt: **PASS**.
- Interactivity self-test: **PASS**; DENY clean and all catalog mutations reject.
- Plain interactivity verifier: **PASS**; DENY clean with the one expected global
  `blocking-bridge` finding.
- Exact production census: **PASS**; four `db_actor::block_on` calls remain before the test region,
  the sole catalog backend await is inside `DatabaseCatalogReadWork`, and the removed direct
  catalog-read bridge has zero matches.
- Scoped working/staged/HEAD diff checks: **PASS**.
- Whole-tree working/staged/HEAD diff checks: **PASS**.

## Residuals

- The backend catalog read is an indivisible Phase 9 platform/backend call.
- Catalog initialization CAS, create-document CAS, compaction, and hello remain synchronous.
- Builds, runtime behavior, timing, backend integration, and public ABI typechecking are unproven by
  policy.
- Phase 1 remains RED.

## Final-audit repair — 2026-08-23

The two blockers in
`📓️coordinator-independent-p1v-catalog-read-final-audit-2026-08-23.md` are repaired in source:

1. `DatabaseCatalogReadFuture::poll` installs the consumer waker and then rechecks `completion`
   before it may return `Pending`. `publish_public_completion` is the single worker publication
   seam. The controlled fixture publishes an exact result owner after the first completion check
   and before waker installation, then proves the same public poll observes the exact owner without
   requiring a later event.
2. `DatabaseCatalogReadRejected::close_step` retires storage and root key on distinct grants. The
   live `Database::open_with` rejection path now transfers unfinished rejection ownership into a
   retained process-pool I/O close state before returning the error. Submission saturation retains
   the exact close job for callback retry. Its controlled fixture proves storage is released on the
   first job, the key remains mounted, and only the second job reaches terminal empty.

The permanent verifier now rejects a check-before-register public poll, a combined storage/key
close branch, and bypassing the mounted rejection close state. The fixture census grows by two and
the P1v mutation census grows from 17 to 20.

### Repair gates

- Edition-2021 scoped Rustfmt and parser: **PASS**.
- P1v self-test section including all 20 mutations: **PASS**; the workspace self-test advanced
  beyond P1v and then stopped on a concurrent unrelated Puzzle-fill baseline mismatch in
  `📜️script.ts`.
- Plain interactivity verification: same unrelated Puzzle-fill baseline stop; no P1v failure was
  emitted.
- Exact production DB-engine wait census: **PASS**, four ordered groups; direct catalog-root
  `block_on` remains absent and the retained backend read remains the sole `read_root` await.
- Scoped and whole working/staged/HEAD diff hygiene: **PASS**.
- Cargo, Nx, Wasm, browser, runtime, and network: **not run**.

The repair is source-audit-ready, not independently accepted. The unrelated concurrent verifier
baseline prevents an honest repository-wide green claim; Phase 1 and the Phase 9 backend-call
latency residual remain red.
