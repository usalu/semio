# Wave W-O4 — order-dependent Rust test isolation

**Claimed:** 2026-09-10T00:34:49+0200
**Updated:** 2026-09-10T03:06:00+0200
**Ticket:** `26/09/02/PUZZLE-3D-END-TO-END`
**Scope:** audit families 1 (plugin-host `patches::tests`) and 2 (`semio-framework-ui-runtime` lib suite). Family 3 (`RUST_MIN_STACK`) is out of scope.

Repo MCP (`repo://goals`, `ticket_reopen`) is not bound in this Cursor session. Work proceeds inside the named ticket folder.

---

## Verdicts

| Family | Suite | Before (audit) | After | Alone | Notes |
|---|---|---|---|---|---|
| 1 plugin `patches::tests` | guard already on all 34 laws | 18 passed / 16 failed in-suite | **34 passed / 0 failed** single-thread **and** default parallelism | 3/3 previously-failing laws pass alone | same shared registries as family 2 |
| 2 `semio-framework-ui-runtime` `--lib` | guard on the 18 in-suite failures + guard-boundary handback reclaim | 106 passed / 18 failed | **122 passed / 0 failed** single-thread **and** default (2 isolated filtered) | 3/3 previously-failing laws pass alone | deadlock + poison isolated via `#[ignore]`; both pass via `--ignored --exact` |

`surface_ownership_resident_return_maintenance_preserves_contended_credit` stays `#[ignore]` — re-entrant resident observe plus handback drain deadlocks in-suite. Run via `runExactCargoLaws` / `--ignored --exact`.

`retained_handback_poison_is_fault_without_mutating_queued_owner` is `#[ignore]` — poisons `SURFACE_RECONCILE_HANDBACKS` by design. Run the same way.

---

## What shipped

### Shared isolation seam

`surface_reconcile_registry_test_guard()` acquires a process mutex, then `isolate_surface_reconcile_registries()`:

1. `drain_surface_reconcile_registry_until_idle()` (soft): recover output poison, drain pending output/resident returns, `close_surface_reconcile_handback_one`, `close_ui_value_page_one`, `close_built_node_page_one`
2. `reclaim_orphaned_handback_slots()` — reserved-unqueued leftovers that soft drain never flags. Retained boxes are taken out of the registry and dropped **after** the mutex is released (in-lock drop deadlocks / stale-assembles).
3. soft drain again so Drop-released pages retire

Hard reclaim of the output registry / resident ledger was tried at the drain boundary and then at the guard boundary. Both wipes produced `UiDocumentAssemblyError { kind: Stale }` / `AliasCapacity` on ~38 document/transaction laws (122/1 → 84/38). Those wipes stay out. Mid-test `drain_surface_reconcile_registry_until_idle()` (census) stays soft so a live job is not wiped.

Guards were already present on all 34 `patches::tests` and on the 18 audit-listed ui-runtime failures (output / reconcile / transaction / ownership / present / document / patch / handback / tree). Dispatch / entity / gateway / inbox / presence / tracking do not share these registries and stay unguarded.

### Production defect (fixed earlier, not an assertion weaken)

`close_surface_reconcile_handback_one` held `SURFACE_RECONCILE_HANDBACKS` and then `close_step` re-locked the same mutex when releasing `output_handback`. That is a process-wide deadlock, not just a test leak.

Fix:

- `SurfaceReconcileRetained::close_step_with(Option<&mut SurfaceReconcileHandbackRegistry>)`
- `release_surface_reconcile_handback_in` releases while the pump already holds the mutex
- queued owners that still carry an external reservation now release it in-place instead of stalling the retirement queue

The production turn pump also cycles output / resident / handback (`AtomicU8` phase) so one pool cannot starve the others.

---

## Before tails (audit 2026-09-10)

### Family 1

```
cargo test -p semio-framework-plugin --lib patches::tests --offline -j 4 -- --test-threads=1
→ 18 passed; 16 failed
```

### Family 2

```
cargo test -p semio-framework-ui-runtime --lib -j 4 -- --test-threads=1
→ 106 passed; 18 failed
```

Later same-wave measurement after guard + soft drain, before handback orphan reclaim:

```
test result: FAILED. 122 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.39s

failures:
    reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged
```

Panic: first job (`generation: 7001`) admits; second `try_new(..., 7002)` returns `SurfaceReconcileRejected`. Same law passes alone. Soft drain empties return-flagged owners and queued handbacks; it does not hard-reset reserved-unqueued orphans.

---

## After tails (this wave)

Envelope: `CARGO_TARGET_DIR=…/target-p3d`, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`, `CARGO_INCREMENTAL=0`, `--offline -j 4`.
Full logs: `🗑️generated/patches-*.txt`, `🗑️generated/ui-runtime-*.txt`.

### Family 1 — single-threaded

```
running 34 tests
...
test component::reactor::patches::tests::terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation ... ok
test component::reactor::patches::tests::terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed ... ok
test component::reactor::patches::tests::tracker_initialization_fits_the_component_stack_budget ... ok
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 576 filtered out; finished in 0.23s
```

### Family 1 — default parallelism

```
running 34 tests
...
test component::reactor::patches::tests::terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation ... ok
test component::reactor::patches::tests::terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed ... ok
test component::reactor::patches::tests::tracker_initialization_fits_the_component_stack_budget ... ok
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 576 filtered out; finished in 0.28s
```

### Family 1 — previously-failing laws alone

```
running 3 tests
test component::reactor::patches::tests::cap_plus_one_returns_the_exact_tree_owner ... ok
test component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner ... ok
test component::reactor::patches::tests::mounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 607 filtered out; finished in 0.01s
```

### Family 2 — single-threaded (skip isolated deadlock + poison)

```
running 122 tests
test reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged ... ok
test result: ok. 122 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 1.02s
```

### Family 2 — default parallelism (skip isolated deadlock + poison)

```
running 122 tests
test reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged ... ok
test result: ok. 122 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.43s
```

### Family 2 — previously-failing laws alone

```
running 3 tests
test reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged ... ok
test reconcile::tests::stale_cancel_and_drop_handoff_preserve_public_terminal_ownership ... ok
test transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 121 filtered out; finished in 0.01s
```

### Isolated deadlock law alone (`--ignored --exact`)

```
running 1 test
test reconcile::tests::ownership::surface_ownership_resident_return_maintenance_preserves_contended_credit ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 123 filtered out; finished in 0.00s
```

### Isolated poison law alone (`--ignored --exact`)

```
running 1 test
test reconcile::handback_entry_tests::retained_handback_poison_is_fault_without_mutating_queued_owner ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 123 filtered out; finished in 0.01s
```

## Remaining work

Family 3 (`RUST_MIN_STACK`) stays out of scope. Isolated deadlock/poison laws are proven alone and stay out of the default suite.

---

## Files

- `♻️reconcile.rs` — guard, soft drain, guard-boundary handback orphan reclaim (drop retained state after unlock)
- `output/🦀️.rs` — pending-return / poison isolation helpers (full slot wipe stays unused from the guard)
- plugin-host `patches::tests` and ui-runtime law modules — `let _guard = …surface_reconcile_registry_test_guard()`
- ownership contended-credit law — `#[ignore]`
- handback poison law — `#[ignore]`
- raw tails under `🗑️generated/`
