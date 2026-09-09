# Order-dependent test failures — root-cause audit (A2)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Read-only audit for the three failure families blocking close gates (`📓️2026-09-09-runtime-verification.md` §00:05→00:40, `📓️2026-09-09-wave-F-fill-oom.md` §2). Confirmed with source reading and targeted `cargo test` runs (private `target-p3d`, `RUSTC_WRAPPER=""`).

---

## Executive summary

| # | family | root cause in one line | fix shape |
|---|--------|------------------------|-----------|
| 1 | plugin-host `patches::tests` | Process-global surface output pool (`REGISTRY`) + resident ledger leak across tests; 64 slots exhaust before later laws run | Shared test-guard mutex + explicit registry drain seam (same pattern as `fill_envelope_test_guard`) |
| 2 | `semio-framework-ui-runtime` lib suite | Same global registries, plus `UI_VALUE_ARENA` / handback registry contention; 18 laws fail in suite, pass alone | One ui-runtime test guard wrapping all reconcile/output/transaction laws + isolate poison/deadlock tests |
| 3 | 2 MiB test-thread stack | App-fixture dispatch/render futures need ~2.6 MiB peak; boxing fill carriers (W-F) fixed the worker half, not the fixture frame | Set `RUST_MIN_STACK` centrally in `runCargoTestBudgeted`; keep per-heavy-crate overrides |

---

## Family 1 — plugin-host `patches::tests`

### Root cause (shared state)

Three process-wide authorities back every `PatchTracker::reserve_mounted_owned` admission. None are reset between libtest cases.

| static | file:line | role |
|--------|-----------|------|
| `REGISTRY` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🦀️.rs:21` | Fixed 64-entry surface **output pool** (`SurfaceReconcileOutputs::try_reserve` → `registry()` line 25–27) |
| `ENTRY_RETURNS` / `QUEUE_RETURNS` | same file `:22–23` | Deferred release flags; slots stay occupied until `SurfaceReconcileOutputs::drain_one` (`:224–247`) runs |
| `RESIDENT_LEDGER` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs:87` | Aggregate resident credit (`UiResidentPermit::try_reserve`, registered once via `register_surface_reconcile_backing` at `♻️reconcile.rs:2227–2228`) |
| `SURFACE_RECONCILE_HANDBACKS` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs:2348` | Reconcile handback slots (384 = 64×6) tied to each reservation |

Admission path (where the leak originates):

```393:418:🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs
    fn reserve_mounted_owned(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey) -> Result<MountedReconcileGrant, ui_contract::SurfaceId> {
        // ...
        let mut outputs = SurfaceReconcileOutputs::default();
        let output_reservation = match outputs.try_reserve(generation, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES) {
            Ok(Some(owner)) => owner,
            Ok(None) => return Err(surface),   // ← suite failure: SurfaceId("75:direct") returned here
```

Each successful `begin()` / `reserve()` consumes one global output entry **and** resident backing. When a `PatchTracker` drops without incremental close of every `ReadySlot.outputs`, `SurfaceReconcileOutputs::drop` (`output/🦀️.rs:251–254`) only sets `QUEUE_RETURNS` — it does **not** free the entry. Production turns pump `drain_one` via `close_surface_reconcile_handback_one` (`♻️reconcile.rs:3333–3336`); tests do not.

**Measured (2026-09-10):** full module, single-threaded:

```
cargo test -p semio-framework-plugin --lib -- --test-threads=1 'component::reactor::patches::tests::'
→ 18 passed; 16 failed
```

Every failing case passes alone. Typical failure: `reserve(...).unwrap()` → `Err(SurfaceId("75:direct"))` because the pool is already full.

### Leaking tests (enumerated)

Tests that **pass in the suite but leave global pool entries occupied** (no `close_instance_to_empty` and/or no global drain):

| test | instance / surfaces | reservations leaked (min.) |
|------|---------------------|----------------------------|
| `effects_publish_in_admission_order_even_when_later_tree_finishes_first` | `1:first`, `1:second` | 2 output entries |
| `generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation` | `61:first`, `61:maximum`, refused attempts | 2+ |
| `mounted_document_tree_publishes_nested_interactive_rows` | fixture surface | 1 |
| `abandoned_reconcile_job_closes_its_ready_output_so_the_tracker_can_idle` | `5:main` | 1 (ready output may close locally; global queue return still pending) |
| `a_closing_terminal_does_not_wait_behind_sixty_three_empty_slots_per_unit` | `6:main` | 1+ |
| `a_deferred_surface_awaiting_the_hosts_acknowledgement_does_not_hold_more_work` | `9:main` | 1+ |
| `cap_plus_one_returns_the_exact_tree_owner` | 64 manual slot fills + refused `65:main` | 0 new reservations (manual slots bypass pool) but resident pressure |
| `terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation` | `62:idle` + 64 terminals | partial |

Tests that **call `close_instance_to_empty` but still fail in suite** (pool already saturated by earlier cases):

All `mounted_output_admission_*` (instances 71–76, fixture instance for `direct`), plus `mounted_path_advances_one_reconcile_opportunity_per_grant`, `mounted_settings_controls_publish_with_authored_fields`, `mounted_sources_publish_every_window_and_panel_tree`, `mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner`, `one_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps`, `published_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss`, `resize_storm_coalesces_to_one_deferred_surface_owner`, `stale_generation_fault_is_publicly_retrievable`, `terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed`, `terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion`, `terminal_full_plus_matching_rejected_advances_capacity_before_conversion`.

Documented upstream leakers from runtime verification: `effects_publish_in_admission_order…`, `generation_max…`, `issued_obsolete…` (last one also hits stack limit — family 3).

### Proposed fix (repo-idiomatic, long-term)

Mirror `fill_envelope_test_guard()` (`✏️editor/⏳️precompute/🦀️.rs:413`) and `PANEL_PAGE_GUARD` (wave O §6.1):

1. **`surface_reconcile_registry_test_guard()`** in `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🦀️.rs` (ui-runtime owns the pool):
   - `static TEST_GUARD: Mutex<()>`
   - On acquire: `drain_surface_reconcile_registry_until_idle()` — bounded loop calling `SurfaceReconcileOutputs::drain_one(1, SURFACE_RECONCILE_PAGE_BYTES)`, `UiResidentPermit::drain_one()`, and `close_surface_reconcile_handback_one()` until `!UiResidentPermit::has_pending_returns()` and output registry has no occupied entries (expose a `#[cfg(test)] fn output_registry_occupied_count() -> usize` reading `REGISTRY`).
   - On drop: same drain + `debug_assert_eq!(UiResidentPermit::snapshot().unwrap(), baseline)` (baseline captured at first guard init).

2. **Re-export for plugin-host tests:** `pub(crate) use …::surface_reconcile_registry_test_guard` from `semio_framework_ui_runtime` (or a thin `#[cfg(test)]` helper in `patches/🧪️tests` that calls the ui-runtime function).

3. **Every `patches::tests` law** takes `let _guard = surface_reconcile_registry_test_guard();` as first statement (34 laws).

4. **RAII `PatchTracker` test fixture** (optional tightening, same file): wrapper whose `Drop` calls `close_instance_to_empty` for every instance id used in the test, then relies on guard drop for global drain. Replace ad-hoc `PatchTracker::new()` in laws that currently omit close.

5. **Do not** raise `SURFACE_RECONCILE_ADMISSION_SLOTS` / `UI_RESIDENT_SLOTS` — the 64-cap is an authored law (`surface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth` in ui-runtime).

6. **plugin-host `📜️script.ts` `TestScript`:** pass test env with `RUST_MIN_STACK` (family 3) — today it calls bare `runCargo([...test...])` with no stack override (`🖥️host/📦️packages/🦀️rust/📜️script.ts:18–21`).

### Verification command

```bash
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d
export RUSTC_WRAPPER=""
export RUST_MIN_STACK=134217728
cargo test -p semio-framework-plugin --lib -j 4 -- --test-threads=1 'component::reactor::patches::tests::'
# expect: 34 passed; 0 failed
```

Spot-check isolation vs suite:

```bash
# must pass alone (already does)
cargo test -p semio-framework-plugin --lib -- --test-threads=1 mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots
# must pass after fix when run after effects_publish + generation_max
cargo test -p semio-framework-plugin --lib -- --test-threads=1 effects_publish_in_admission_order generation_max mounted_output_admission_direct_receiver
```

---

## Family 2 — `semio-framework-ui-runtime` lib suite

### Root cause (shared state)

Same global authorities as family 1, plus:

| static | file:line | role |
|--------|-----------|------|
| `UI_VALUE_ARENA` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:908` | Process-global value arena (`LazyLock<Mutex<UiValueArena>>`); panel page laws use `PANEL_PAGE_GUARD` but ui-runtime laws do not |
| `RESIDENT_TURN` | `♻️reconcile.rs:3334` | Alternates resident drain turns; stale state if prior test skipped drain |

**Measured (2026-09-10):**

```
cargo test -p semio-framework-ui-runtime --lib -j 4 -- --test-threads=1
→ 106 passed; 18 failed
```

Matches wave O baseline (`📓️2026-09-09-wave-O-outliner-and-arena.md` §3.1, §6.2): failures are `Credits { … }` aggregate refusals and leaked reservations, not arena-size regressions. Every listed law passes in isolation.

### Failing tests (enumerated — full suite, 2026-09-10)

**output / reconcile**

- `reconcile::tests::output_pool_tests::surface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot`
- `reconcile::tests::ownership::surface_ownership_resident_return_maintenance_preserves_contended_credit` (deadlocks in suite — holds registry lock, re-entrant `release_surface_reconcile_handback`; skip or isolate)
- `reconcile::tests::semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged`
- `reconcile::tests::stale_cancel_and_drop_handoff_preserve_public_terminal_ownership`
- `reconcile::tree_retirement::tests::runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission`
- `reconcile::tree_retirement::tests::runtime_tree_retirement_rejected_close_preserves_source_until_handback_admission`

**transaction**

- `transaction::tests::a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch`
- `transaction::tests::a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction`
- `transaction::tests::a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command`
- `transaction::tests::an_entity_notified_but_not_read_by_any_surface_produces_no_patch`
- `transaction::tests::an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch`
- `transaction::tests::cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision`
- `transaction::tests::deterministic_surface_order_is_independent_of_hash_map_insertion_order`
- `transaction::tests::one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output`
- `transaction::tests::presence_flushes_on_its_own_channel_and_never_appears_in_a_patch`
- `transaction::tests::repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command`
- `transaction::tests::transaction_canonical_job_preserves_independent_node_credit`
- `transaction::tests::two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other`

### Leaking tests (typical sources)

Earlier passing laws in the same process reserve output/handback/resident credit without reaching the production drain pump. High-volume modules: `📤️output/🧪️tests/📤️output/🦀️.rs` (7 laws, each admits up to 64 entries), `📏️ownership/🧪️tests/📏️ownership/🦀️.rs`, `🧪️tests/🔬️reconcile-unit/🦀️.rs`, `🧪️tests/🔬️present-unit/🦀️.rs`. None take a process guard today (grep: no `test_guard` in ui-runtime).

Contract-side `poisoned_arena_lock_recovers_without_losing_fixed_authority` poisons `UI_VALUE_ARENA` by design; any value-retirement test scheduled after it fails closed (`📓️2026-09-09-wave-O-outliner-and-arena.md` §7.7).

### Proposed fix

1. **Same `surface_reconcile_registry_test_guard()`** as family 1 — ui-runtime is the owner crate; export `#[cfg(test)] pub fn surface_reconcile_registry_test_guard()`.

2. **Extend guard for value arena** when tests materialise `UiValue`: either
   - sub-guard `ui_value_arena_test_guard()` beside `PANEL_PAGE_GUARD` in puzzle panels (already exists for panel laws), **or**
   - fold arena reset into the reconcile guard by calling existing `with_ui_value_arena` recovery path after drain (preferred single guard for ui-runtime crate tests).

3. **Apply guard to every law** under `reconcile::tests`, `transaction::tests`, `output_pool_tests`, ownership tests, present-unit, reconcile-unit, tree_retirement tests (~120+ laws). Macro or test-module `setup` hook acceptable if every law inherits.

4. **Isolate special cases** (not guard-fixable):
   - `surface_ownership_resident_return_maintenance_preserves_contended_credit` → `#[ignore]` + run only via `runExactCargoLaws` single-process filter (wave O already skips in measurements).
   - `poisoned_arena_lock_recovers_without_losing_fixed_authority` → own process (nextest) or fix retirement pump to recover poison like `with_ui_value_arena` (contract owner).

5. **`📦️packages/🦀️rust/📜️script.ts` `TestScript`:** today calls `runCargoTestBudgeted([], packageRoot, …)` with **no** `RUST_MIN_STACK` in env (`:38`). Inherit central policy (family 3).

### Verification command

```bash
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d
export RUSTC_WRAPPER=""
export RUST_MIN_STACK=134217728
cargo test -p semio-framework-ui-runtime --lib -j 4 -- \
  --test-threads=1 \
  --skip surface_ownership_resident_return_maintenance_preserves_contended_credit
# expect after fix: 123 passed (or 124 if deadlock fixed); 0 failed
```

---

## Family 3 — 2 MiB default test-thread stack policy

### Root cause

libtest spawns each test on a worker thread with **2 MiB** stack unless `RUST_MIN_STACK` is set. Measured in `📓️2026-09-09-wave-F-fill-oom.md` §2.1:

| body | peak stack |
|------|------------|
| `testkit::new_app_with_registry` | 1 334 KB |
| `app().await` (full fixture) | 1 966 KB |
| `app().await` + one `dispatch` | 2 575 KB |
| failing law body (`fill_and_brush_params…`) | 2 651 KB |

W-F boxed the oversized **fill worker** carriers (`OwnedFillWorker = Box<MountedFillWorker>`, `OwnedFillOutcome = Box<StepOutcome>` at `⏳️precompute/🦀️.rs:211–224`) — that removed ~29 KiB from collision paths but **not** the ~1.4 MiB `VcsArtifactApp` fixture frame built on every dispatch/render.

**Confirmed (2026-09-10):** `issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner` aborts at `RUST_MIN_STACK=2097152` (2 MiB); needs ≥ 64 MiB per runtime verification. Stack overflow is from `PendingPatchAuthority` + full reconcile close loops on the test thread, not from the output pool.

### Current invocation map (inconsistent)

| path | `RUST_MIN_STACK` for `cargo test` |
|------|-----------------------------------|
| `runCargoTestBudgeted` (repo library) | **none** — passes `env` through unchanged (`🟦️.ts:1747`) |
| `runExactCargoLaws` | `nativeEnv: 268435456` (256 MiB) for isolated law binaries |
| `semio-framework-os` exact stages | build 32 MiB / native 256 MiB (`💻️os/📦️packages/🦀️rust/📜️script.ts:32–36`) |
| `semio-s-artifact-puzzle-3d` | `process.env.RUST_MIN_STACK ??= 128 MiB` in crate script (`📦️packages/🦀️rust/📜️script.ts:13`) |
| `semio-framework-plugin-host` `TestScript` | **none** |
| `semio-framework-ui-runtime` `TestScript` | **none** |
| Direct `cargo test` / launch.json | ad hoc (gates use 128–134 MiB manually) |

**Should fixtures shrink instead?** Partial W-F boxing helps workers only. Shrink the fixture frame (box `Puzzle3dPlayApp` at dispatch boundary, shrink `StepOutcome` inline pages) is valid long-term production work but does **not** replace gate policy: plugin-host `patches` and ui-runtime reconcile laws also need headroom without puzzle3d fixtures.

### Recommended policy (one zero-touch rule)

**Primary:** extend `runCargoTestBudgeted` in  
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`:

```typescript
const testEnv = {
  ...env,
  RUST_MIN_STACK: env.RUST_MIN_STACK ?? process.env.SEMIO_TEST_NATIVE_RUST_MIN_STACK ?? "134217728", // 128 MiB
};
// use testEnv for cargo test / nextest run (not necessarily for cargo build --tests)
```

- **`SEMIO_TEST_NATIVE_RUST_MIN_STACK`** — override for CI/devcontainer (document in devcontainer env, Windows/macOS/Linux parity).
- **`SEMIO_BUILD_RUST_MIN_STACK`** — stays 32 MiB for **rustc** worker threads only (existing os-kernel pattern).
- **`runExactCargoLaws`** — keep 256 MiB nativeEnv for the heaviest isolated laws (plugin-host guest-fault, ui-patch marshalling).
- **Per-crate `??=` overrides** — retain puzzle3d's 128 MiB and gis/norm/hub 256 MiB where already declared; they win over the default when explicitly set before calling shared router.

**Secondary:** plugin-host + ui-runtime `📜️script.ts` `TestScript` should call `runCargoTestBudgeted` with explicit env merge (or rely on central default after library change).

**Not recommended:** `.cargo/config.toml` `[env] RUST_MIN_STACK` — hides per-target needs and fights `SEMIO_BUILD_RUST_MIN_STACK` split.

### Verification commands

```bash
# 2 MiB must fail issued_obsolete (plugin-host patches)
export RUST_MIN_STACK=2097152
cargo test -p semio-framework-plugin --lib -- --test-threads=1 issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner
# → stack overflow (expected before policy)

# 128 MiB gate default must pass
export RUST_MIN_STACK=134217728
cargo test -p semio-framework-plugin --lib -- --test-threads=1 issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -- --test-threads=1 fill_and_brush_params_are_tagged_utility_options_not_engagement_controls

# nx zero-touch (after library script change)
bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts test
bun ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts test
```

---

## Cross-family dependency

Fix family **1** and **2** together in ui-runtime (guard + drain live in `📤️output/🦀️.rs` / `♻️reconcile.rs`). Family **3** is orthogonal but must land in the same gate pass so `issued_obsolete…` and puzzle3d app laws stop aborting the process.

---

## Files referenced

| path | relevance |
|------|-----------|
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🦀️.rs:21–23,251–254` | Output pool statics + Drop deferral |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs:87–88,188–190,263+` | Resident ledger + drain_one |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs:2227–2228,2348,3333–3336` | Backing registration + handback drain pump |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:393–418` | Admission consumes global pool |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | 34 laws, many without global drain |
| `✏️s/…/⏳️precompute/🦀️.rs:211–224,413` | W-F boxing + fill guard precedent |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1659–1747` | Central test runner (missing stack default) |
