# 🧹 P1y Retained Database Compaction Implementation

Date: 2026-08-24  
Executor: Sol High  
Status: SOURCE/STATIC AUDIT READY — NOT SELF-ACCEPTED  
Scope: P1y production compaction authority, selected engine facade, P1y root verifier, and the exact post-P1y P1w/P1x wait-census preservation adjustment.

## 🎯 Caller census and cutover

The selected production path was confirmed as:

`db_cli::cmd_compact` → `Database::compact_document` → `Compactor::run_from_latest_snapshot`.

`Database::compact_document` now preflights its fixed `DbIoText` holder, converts the document identity, observes the real async epoch, admits `DatabaseCompactionFuture`, explicitly closes a rejected authority, awaits only the mounted terminal witness, and closes/takes its report. The CLI therefore uses the same retained authority without a compatibility route. Production `db_actor::block_on` census in `db_engine` is now one: the independently scheduled `db_sync::handle_hello` residual. `Compactor::run_from_latest_snapshot` and the other eager helpers are `#[cfg(test)]` only.

Evidence:

- `db_engine/component.rs:7373` — retained facade.
- `db_engine/component.rs:7378` — selected async facade cutover.
- `db_engine/component.rs:7395` — sole remaining production wait.

## 🧵 Counterexample-to-fix mapping

| Counterexample | Implemented production fix | Evidence |
| --- | --- | --- |
| A facade poll can synchronously drive the complete multi-subsystem compaction pass. | One generation-qualified mounted future owns execution, terminal result, retry, wake, cancellation, deadline, and retirement state. Only its typed `Lane::Io` job polls backend work. | `db_compact/component.rs:824`, `:1372`, `:1467`, `:1481`, `:1533` |
| Admission refusal can lose storage/document/holder identity or charge logical length instead of backing. | Fixed 32-slot admission charges fixed item/byte credits and checks the admitted `String` capacity. `DatabaseCompactionRejected` retains exact storage, document, holder, error, and mounted incremental close. | `db_compact/component.rs:932`, `:1882` |
| Backend-returned and cloned allocations can exceed the credited working set despite small logical lengths. | Actual `String`/descriptor `Vec` capacities are observed against the per-operation credit. Over-limit snapshot descriptors are consumed by an explicit one-backing-per-poll retirement path; index document clones are observed after allocation. | `db_compact/component.rs:835`, `:858`, `:1100`, `:1229` |
| WAL replay, horizon selection, payload trace/delete, index merge, snapshot chain/page collection, publication, generation pruning, or cleanup can run to completion in one opportunity. | Fixed segment/hash/page/report owners and retained async cursors yield at each record, scalar, hash comparison, delete, index kind, descriptor/page, generation, and close transition. Owner close uses `poll_fn` with one close step per poll, not a bulk close loop. | `db_compact/component.rs:1100`, `:1229` |
| Cancellation/fault after lease acquisition can strand the lease or let release failure overwrite the original failure. | The execution authority retains resource/holder/fence and always enters lease release after the governed run. `(run, release)` preserves the earlier run error and reports a release error only after successful work. The admitted epoch is used for acquisition. | `db_compact/component.rs:1312` |
| Concurrent snapshot publication can make a collected chain stale. | Publication revalidates the exact latest generation immediately before publishing, overflow-checks the successor, and validates the returned generation before pruning. | `db_compact/component.rs:1229` |
| Lane saturation can drop the exact closure, create a lost wake, or let callback/drop maintenance poll backend work. | Atomic `Idle → Queued → Driving/Retry` ownership publishes the exact refused job before claim transition, retries through the shared `WorkerPool` typed `Lane::Io`, and uses check-register-recheck completion. Callback close is gated on terminal execution and never calls `poll_one`. | `db_compact/component.rs:1467`, `:1481`, `:1495`, `:1533` |
| Ready/Pending/panic/Drop can release the driver claim before publishing the retained owner. | The driver takes the unique claim, polls once, publishes the future/output/quarantined future under the core owner, consumes the wake, then releases the claim. Result/Future Drop mounts terminal cleanup instead of recursively draining. | `db_compact/component.rs:1533`, `:1654`, `:1705` |
| Terminal reports retain a dynamic `Vec` and cleanup drains all owners. | `CompactionIndexReports` is a fixed array/cursor. Each terminal close opportunity retires at most one report entry, holder backing, document, storage, or result owner. | `db_compact/component.rs:767`, `:1387` |

## 🧪 Hostile production-path laws

- `retained_compaction_handoff_to_first_poll_cancel_uses_real_io_lane_and_releases_exact_owners_under_eight_ms` holds the sole worker, cancels in Handoff, proves sub-8 ms admission, exact storage/document/holder identity, typed cancellation, registry drain, and the production `Lane::Io` driver.
- `retained_compaction_actual_deadline_callback_lost_wake_and_drop_close_release_lease_once` exercises the mounted deadline callback plus the real wake path and proves typed timeout, exact identity, idle driver, and released admission.
- `retained_compaction_max_plus_one_capacity_refusal_preserves_storage_document_holder_and_hash_authority` attacks slot MAX+1, externally overallocated `String`, fixed hash MAX+1, and a descriptor whose observed `Vec` capacity exceeds the credited item ceiling; exact rejected identities are recovered and descriptor refusal uses mounted incremental retirement.
- `retained_compaction_stale_aba_drop_and_partial_terminal_close_keep_one_generation_owner_per_opportunity` installs an ABA replacement generation, proves typed stale rejection and registry discoverability, then proves exactly one fixed report owner is removed by one close opportunity.

Law bodies: `db_compact/component.rs:2031`, `:2059`, `:2081`, `:2140`.

## 🛡️ Permanent verifier

The exact `//#region 🧹P1yDatabaseCompaction` source verifier checks the facade wait census/cutover, fixed admission and working owners, actual capacity calls, overflow-checked admission/publication generations, atomic driver transitions, exact saturation retry, owner publication ordering, one-opportunity yield/close, admitted epoch, snapshot revalidation, lease-release precedence, mounted deadline, check-register-recheck, terminal close, test-only eager helpers, and concrete law-body evidence.

Its live-source mutation matrix removes or corrupts each obligation, including logical-length substitution for document/descriptor/index backings, descriptor retirement removal, unchecked generations, dynamic working set, yield removal, wrong worker lane, refused-job loss, retry callback/limit removal, driver claim removal, callback polling of the live backend, terminal-guard bypass, Pending/Ready/panic owner loss, lease release/admitted epoch removal, both publication-generation checks, deadline registration, lost-wake recheck, dynamic reports, eager compactor resurrection, facade blocking, every hostile law, and the selected-wait contract statement. Every mutation must independently make the P1y gate fail.

Evidence: `📜️script.ts:10728-10872`.

## ✅ Permitted source/static gates

- `bun ./📜️script.ts verify interactivity p1y` — GREEN: `live-source and hostile mutations clean`.
- `bun ./📜️script.ts verify interactivity p1x` — GREEN: `live-source and hostile mutations clean`.
- `bun ./📜️script.ts verify interactivity p1w` — GREEN: `live-source and hostile mutations clean`.
- `bun ./📜️script.ts verify interactivity p1q-b1-b6` — GREEN: `live-source and hostile mutations clean`.
- `rustfmt --edition 2021 --check` for the scoped `db_compact` and `db_engine` files — GREEN.
- `git diff --check` for scoped `db_compact`, `db_engine`, and root verifier files — GREEN.
- Caller/eager-helper `rg` census — one production engine wait; selected facade and CLI route use the retained authority; enumerated eager compaction helpers are test-only.

Cargo, Nx, Wasm, browser, and runtime/build gates were not run, exactly as required while overlapping source packets are active. Phase 9 indivisible backend-poll latency and the serialized native/Wasm/runtime matrix remain deferred to coordinator verification. This report makes no runtime acceptance claim.

