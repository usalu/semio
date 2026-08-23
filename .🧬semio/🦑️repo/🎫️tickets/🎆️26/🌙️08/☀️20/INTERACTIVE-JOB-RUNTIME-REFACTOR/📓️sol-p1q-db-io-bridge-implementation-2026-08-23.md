# Phase 1 DB I/O Bridge Source Implementation — 2026-08-23

## Status

Source packet ready for independent Terra audit. This report does not claim Phase 1 acceptance or ticket closure. Cargo, Nx, Wasm, browser, network, and runtime integration checks were intentionally not run under the packet constraints.

## Caller census and reachability

The authored Fs and SQLite storage implementations contain 60 live `run_blocking_op` submissions: 57 storage-trait operations and 3 backend open/schema authorities. The storage traits flow through `DbBackend`, `Database`, `ArtifactHandle`, CLI, hub, snapshot, WAL, index, compact, and cluster surfaces, so product/UI/plugin-reachable database work can reach this bridge.

Before this packet, `FsStorage` and `SqliteStorage` held an optional pool and `run_blocking_op` executed `work()` inline when it was absent. That branch was live through pool-less facade and CLI construction. Authored production DB source now has zero optional WorkerPool fields, zero `None => work()` or `None => job()` branches, zero `open_inline`, and zero `.with_pool(...)` compatibility paths. All 60 authorities require an injected process WorkerPool.

The generated Compose server/hub Rust output still names the former pool-less `Database::open_at` signature. Compose was explicitly out of scope for this packet and was not edited; regeneration/reconciliation remains a separate source blocker.

## Owned authority

`DbIoOperation` retains generation, progress, cancellation, exact work, completion, terminal work/result, rejected successor job, retry job, and admission credit. It submits only to `Lane::Io`, checks generation before mutable scheduling state, and uses the existing WorkerPool timer wheel for a generation-keyed, coalesced retry after contention or saturation. Shutdown, poisoning, and retry exhaustion expose the exact rejected closure through `take_terminal_job`; terminal work/result have corresponding retrieval, and `close_step` releases at most one terminal owner.

Admission is schema-first through `DbIoRequest::{metadata,read,write,read_transform,list}` and preflights nested item and byte credits before claiming a slot:

- 16 KiB logical pages.
- 64 item slots.
- 64 pages / 1 MiB maximum credit per operation.
- 1024 pages / 16 MiB process aggregate.
- 4096 list items, including eight credit bytes per returned identifier.
- 496 KiB maximum individual storage blob/input owner.

Write inputs use `DbIoPages`, which takes an existing `Vec<u8>` owner without copying, exposes fixed 16 KiB page views, and returns the exact original Vec on cap or range rejection. The nine Fs/SQLite append/snapshot/payload/catalog/index write façades now move that owner into the retained operation; none performs a caller-thread `to_vec()` before admission. WAL suffix submission retains the original snapshot Vec plus a range cursor rather than allocating a suffix copy.

The public testkit replay law accepts an injected shared pool. Its unit entrypoint uses `process_worker_pool`; authored production testkit source contains no `WorkerPool::new` subsystem constructor.

## Terra rejection remediation

The focused Terra audit found synchronous Hub setup before the retained constructors. The Fs arm called `std::fs::create_dir_all(root)`, the SQLite arm created its parent, and Hub `main` created `data_dir` immediately before `connect_db`. All three calls were redundant with `FsStorage::open` and `SqliteStorage::open`, whose retained constructor operations already own recursive directory preparation and its exact result/error/cancellation state.

Those three Hub calls are removed. The live `connect_db` Fs branch now goes directly to `Database::open_at(pool, ...)`; the SQLite branch goes directly to `SqliteStorage::open(pool.clone(), ...)`; and `main` reaches `connect_db` without filesystem or rusqlite setup. A wider production census found no pre-constructor filesystem/rusqlite call in the CLI or authored DB facade/engine setup paths. Hub's identity-directory backend and admin directory-size endpoint are separate subsystems and were not changed by this focused DB-storage remediation.

Stale Fs storage documentation now names the process `WorkerPool`, states that construction requires it, and describes constructor directory creation as part of the retained `Lane::Io` authority rather than an inline or pool-less mkdir.

### Focused verifier-gap remediation

The first re-audit confirmed production setup was clean but rejected the SQLite adversarial case because it injected `rusqlite::Connection::open` rather than the former parent-directory bypass. `hub-sqlite-pre-open` now reinserts the exact guarded shape immediately before `SqliteStorage::open`: `if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }`. The direct-rusqlite mutation remains separately as `hub-sqlite-direct-open`; the existing Fs-root and main-data-dir mutations are retained.

The self-test also requires that this exact SQLite-parent fixture produces the named `Hub DB setup performs synchronous filesystem/SQLite work before the retained pooled constructor authority` failure. This proves rejection by the intended DB pre-open rule rather than by an incidental fixture/parser failure.

## Deterministic fixtures and verifier mutations

Direct Rust source fixtures cover:

- missing-pool construction being unrepresentable;
- 64 item slots plus one and 16 MiB aggregate saturation;
- one-operation bytes plus one and exact rejected input owner;
- cancel before execution with exact work retrieval;
- cancel during execution with exact result retrieval, and stable cancel after completion;
- stale generation before work consumption;
- shutdown rejected-job take/resume and interrupted one-owner close.

The existing root `📜️script.ts` interactivity verifier now reads production-only DB storage, SQLite, engine, testkit, and Hub entrypoint sources. Its adversarial corpus rejects optional/missing pools, inline `work()`, slice write APIs, 1 GiB-scale slots, missing owner handback, stale-after-mutation ordering, quiet retry stranding, terminal-result sinks, optional engine pools, testkit subsystem pool construction, Hub pre-open `std::fs::create_dir_all`, direct rusqlite setup, and main-level directory preparation before `connect_db`.

## Permitted checks

- Scoped `rustfmt --edition 2021 --check --config skip_children=true` over the Hub entrypoint and 13 edited DB Rust sources: passed.
- `bun 📜️script.ts verify interactivity --self-test`: passed in `deny` mode; one total finding is the existing allowlisted renderer process-entry `block_on`, with zero unlisted failures. The DB and Hub adversarial self-tests run as part of this command, including the exact SQLite-parent pattern and its intended-rule assertion.
- Production forbidden scans: zero optional WorkerPool, inline `None => work/job`, `open_inline`, and `.with_pool` matches in authored DB source; zero production slice/Vec signatures on the five owner-taking storage write methods; zero `WorkerPool::new` in DB testkit. The exact Hub `connect_db` and main-to-`connect_db` slices contain zero `std::fs`, `rusqlite`, or `create_dir_all` setup calls.
- P1q-scoped working-tree, staged, and `HEAD` `git diff --check` variants: passed.
- Whole working-tree, staged, and `HEAD` `git diff --check` variants: passed in the final stable snapshot.

No Cargo, Nx, Wasm, browser, root lint, network, or external runtime validation was run.

## Indivisible residuals

This packet does not claim that a filesystem syscall or SQLite call completes within the interactive turn ceiling. `FnOnce` remains an internal implementation seam around one indivisible backend opportunity. Admission, scheduling, cancellation, retry, and ownership are retained and bounded, but a single `std::fs` or `rusqlite` operation can block an I/O worker beyond the ceiling. SQLite remains a Phase 9 owned-event-log residual.

Additional indivisible/materialization residuals:

- Fs whole-file reads and SQLite whole-blob query results are not readiness-driven or page-streamed; their result length is preflighted/capped, but backend allocation/copy happens in one opportunity.
- SQLite length preflight plus blob fetch can contain multiple backend calls in the same retained closure.
- `SharedBuf::snapshot()` still materializes the current WAL buffer before its Vec owner is handed to `DbIoPages`; suffix extraction itself no longer copies.
- `DbIoPages::into_vec` must copy a nonzero-start range for consumers that request a contiguous Vec; the live Fs/SQLite owner paths use page/slice views and do not call it.
- `db_engine::ArtifactHandle` still contains a distinct actor future `db_actor::block_on` inside a WorkerPool job; it is outside this storage bridge packet and remains a Phase 1 source/runtime-matrix item.
- The runtime acceptance matrix, latency/fairness traces, cancellation behavior under real backend stalls, and generated Compose reconciliation remain open.

Phase 1 therefore remains open even if this source packet is independently accepted.
