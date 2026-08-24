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
| Backend-returned and cloned allocations can exceed the credited working set despite small logical lengths. | A debit/return ledger cumulatively charges actual `String`, descriptor `Vec`, page and body capacity across every simultaneously live owner. Refusal and all error exits mount explicit incremental retirement before returning credit. | `db_compact/component.rs:838`, `:884`, `:1300` |
| WAL replay, horizon selection, payload trace/delete, index merge, snapshot chain/page collection, publication, generation pruning, or cleanup can run to completion in one opportunity. | Fixed segment/hash/page/report owners and retained async cursors yield at each record, scalar, hash comparison, delete, index kind, descriptor/page, generation, and close transition. Owner close uses `poll_fn` with one close step per poll, not a bulk close loop. | `db_compact/component.rs:1100`, `:1229` |
| Cancellation/fault/panic after lease acquisition can strand the lease or let release failure overwrite the original failure. | `DatabaseCompactionLeaseRecovery` retains resource/holder/fence immediately after acquire. Ordinary failure and caught panic poll the typed release future on `Lane::Io`; public panic completion waits for release, release/execution quarantine retirement, admission release and registry drain. `(run, release)` preserves earlier-error precedence. | `db_compact/component.rs:1479`, `:1755`, `:1855` |
| Concurrent snapshot publication can durably write a stale full baseline between check and write. | `publish_retained_expected` holds the fixed per-document publication claim across latest-generation observation and write, rejects a mismatch before building/writing, overflow-checks the successor, uses the fixed retained page source without a hidden hash `Vec`, and returns the exact body owner on success or refusal. | `db_snapshot/component.rs:613`, `:783`, `:1235`; `db_compact/component.rs:1415` |
| Lane saturation can drop the exact closure, create a lost wake, or let callback/drop maintenance poll backend work. | Atomic `Idle → Queued → Driving/Retry` ownership publishes the exact refused job before claim transition, retries through the shared `WorkerPool` typed `Lane::Io`, and uses check-register-recheck completion. Callback close is gated on terminal execution and never calls `poll_one`. | `db_compact/component.rs:1467`, `:1481`, `:1495`, `:1533` |
| Ready/Pending/panic/Drop can release the driver claim before publishing the retained owner. | The driver takes the unique claim, polls once, publishes the future/output/quarantined future under the core owner, consumes the wake, then releases the claim. Result/Future Drop mounts terminal cleanup instead of recursively draining. | `db_compact/component.rs:1533`, `:1654`, `:1705` |
| Terminal reports retain a dynamic `Vec` and cleanup drains all owners. | `CompactionIndexReports` is a fixed array/cursor. Each terminal close opportunity retires at most one report entry, holder backing, document, storage, or result owner. | `db_compact/component.rs:767`, `:1387` |

## 🔴 Independent RED A1–A4 Closure

- A1: the production index child now receives the exact parent cancellation `Arc`, an eight-millisecond deadline, and bounded fuel through `IndexHandle::retained_operation_control`. Fuel/deadline retries retain the handle and its charged document; cancellation becomes an error value, then the handle is dropped and its ledger debit is returned before propagation.
- A2: the former post-write comparison is gone from the live path. A fixed nonblocking atomic publication claim covers latest-generation observation through storage write, mismatch is refused before construction/write, and the allocation-free retained encoder reads page hashes directly from the fixed admitted source.
- A3: lease recovery is installed immediately after acquire. Both normal and panicking release polls use the same typed I/O driver; panicked execution/release futures are quarantined, release is retried from retained state, and public completion waits for exact quarantine, lease, admission, and registry retirement.
- A4: the backing ledger uses checked cumulative add/sub across descriptor, index-document, page, body, and list owners. Snapshot collection keeps every dynamic owner in outer retained slots so all `?`, cancellation, capacity overflow, stale, and publication-fault paths enter one-owner-at-a-time close before debit return.

The governing census now states these narrowed obligations explicitly; the root verifier requires them and mutates each statement independently.

## 🔁 Post-RED Lease Release Error Remediation

The independent re-audit found one remaining A3 counterexample: `release_future` consumed the fence and set `released` after both `Ok` and `Err`. The recovery authority now branches on the actual backend result. Only `Ok(())` removes the fence and publishes `released`; `Err` resets only the in-flight release claim and preserves the exact storage/resource/holder/fence owners.

The first release fault remains a stable discoverable owner. Later retry faults occupy a separate retained slot and are retired one backing per callback opportunity. `callback_at` supplies real worker-loop backoff, then republishes the typed release future before `schedule` submits its next poll to `Lane::Io`. A normal execution output whose release failed is parked in `release_waiting`; a panic remains in execution/release quarantine. Neither route can enter terminal close, release admission/registry, or complete publicly while the fence or release fault remains. After release succeeds, retained fault owners are incrementally retired before the parked output or panic fault is published.

Two new hostile laws exercise the production state: `retained_compaction_release_error_retries_through_real_worker_loop_until_success_before_public_fault` injects one release failure then proves the real callback/I/O retry succeeds and drains exactly; `retained_compaction_perpetual_release_error_keeps_fence_fault_admission_and_registry_discoverable` sustains release failure, proves the fence/fault/lease/admission/registry remain discoverable with no main compaction repoll, then removes the fault and proves exact recovery.

Permanent mutations now force unconditional fence consumption, incorrectly set `released` on `Err`, drop the retained error, delete the actual release-retry callback registration, remove the `release_waiting` terminal guard, and delete either hostile law. Each independently fails the P1y verifier.

## 🧪 Hostile production-path laws

- `retained_compaction_handoff_to_first_poll_cancel_uses_real_io_lane_and_releases_exact_owners_under_eight_ms` holds the sole worker, cancels in Handoff, proves sub-8 ms admission, exact storage/document/holder identity, typed cancellation, registry drain, and the production `Lane::Io` driver.
- `retained_compaction_actual_deadline_callback_lost_wake_and_drop_close_release_lease_once` exercises the mounted deadline callback plus the real wake path and proves typed timeout, exact identity, idle driver, and released admission.
- `retained_compaction_max_plus_one_capacity_refusal_preserves_storage_document_holder_and_hash_authority` attacks slot MAX+1, externally overallocated `String`, fixed hash MAX+1, and a descriptor whose observed `Vec` capacity exceeds the credited item ceiling; exact rejected identities are recovered and descriptor refusal uses mounted incremental retirement.
- `retained_compaction_stale_aba_drop_and_partial_terminal_close_keep_one_generation_owner_per_opportunity` installs an ABA replacement generation, proves typed stale rejection and registry discoverability, then proves exactly one fixed report owner is removed by one close opportunity.
- `retained_compaction_index_child_uses_exact_parent_cancel_and_eight_ms_control` passes the production parent cancellation identity and eight-millisecond deadline through `IndexHandle::retained_operation_control`, cancels it, and proves the child reports the exact cancellation rather than using the former private 30-second/65,536-fuel authority.
- `retained_compaction_expected_snapshot_publication_never_persists_stale_baseline` captures generation zero, advances the same document, attacks expected generation zero, recovers the exact rejected body owner, and proves only generations zero and one exist.
- `retained_compaction_panic_after_lease_acquire_releases_once_before_public_fault_and_registry_drain` installs an actual lease fence into the production state, injects the caught execution panic, and proves release precedes typed panic completion, quarantine retirement, admission release and registry drain.
- `retained_compaction_cumulative_observed_backing_rejects_individually_valid_combined_max_plus_one` retains two actual external allocations that each fit alone, proves their combined observed capacity is refused at MAX+1 without identity loss, retires one backing per opportunity, and proves full ledger recovery.

Independent-RED law bodies: `db_compact/component.rs:2469`, `:2483`, `:2503`, `:2525`.

## 🛡️ Permanent verifier

The exact `//#region 🧹P1yDatabaseCompaction` source verifier checks the facade wait census/cutover, fixed admission and working owners, actual capacity calls, overflow-checked admission/publication generations, atomic driver transitions, exact saturation retry, owner publication ordering, one-opportunity yield/close, admitted epoch, snapshot revalidation, lease-release precedence, mounted deadline, check-register-recheck, terminal close, test-only eager helpers, and concrete law-body evidence.

Its live-source mutation matrix removes or corrupts each obligation, including logical-length substitution for document/descriptor/index backings, exact descriptor retirement, cumulative checked-add/debit return, all six page-ledger returns, shared index child control, expected-generation claim/pre-write refusal/exact body recovery/allocation-free hash construction, panic/release quarantine, admitted lease epoch, Pending/Ready/panic owner publication, deadline registration, lost-wake closure, facade blocking, every hostile law, and each narrowed A1–A4 contract statement. Every mutation binds live source and independently makes the P1y gate fail.

Evidence: `📜️script.ts:10730-10921`.

## ✅ Permitted source/static gates

- `bun ./📜️script.ts verify interactivity p1y` — GREEN: `live-source and hostile mutations clean`.
- `bun ./📜️script.ts verify interactivity p1x` — GREEN: `live-source and hostile mutations clean`.
- `bun ./📜️script.ts verify interactivity p1w` — GREEN: `live-source and hostile mutations clean`.
- `bun ./📜️script.ts verify interactivity p1q-b1-b6` — GREEN: `live-source and hostile mutations clean`.
- `rustfmt --edition 2021 --check` for scoped `db_compact`, `db_index`, and `db_snapshot` files — GREEN.
- `git diff --check` for scoped compact/snapshot/root-verifier/census/report files — GREEN.
- Caller/eager-helper `rg` census — one production engine wait; selected facade and CLI route use the retained authority; enumerated eager compaction helpers are test-only.

Cargo, Nx, Wasm, browser, and runtime/build gates were not run, exactly as required while overlapping source packets are active. Phase 9 indivisible backend-poll latency and the serialized native/Wasm/runtime matrix remain deferred to coordinator verification. This report makes no runtime acceptance claim.
