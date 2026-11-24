# L35: 3D import stock, open-vortex close custody, settlement ledger

This is source analysis only. Nothing was compiled or run, so every fix below is UNVERIFIED.
The log is `🗑️generated/l32-3d.log`.

Abbreviations:
- E = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`

## 1. 145 KB import and chunked pick: "original fixture recipient exceeds its one issued stock"

### Where it fires
The panic comes from the lent context's scope retirement after the test body, not from a step inside the import:
- `OriginalFixtureScope::retire` calls `OriginalFixtureCaller::retire` (P/🧪️testing/🎟️caller/🦀️.rs:46).
- That runs `retire_decoder` → `within_stock` (:37) or `retire_step` → `remaining_grant`.
- `remaining_grant` calls `fixture_remaining` (🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs:183), which asserts.

### Why the body never refused earlier
Every fixture turn renews the lent context through `fixture_renew_turn` → `testkit::renew_turn` (🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:656-661). That sets `cx.retained = current + grant` and re-bases `original_total_retained_start = current`. The bound Source budget also resets its `spent` per turn (`OriginalReturnBudget::renew_turn`).

So nothing during the body compares the monotone cumulative `spent` against the caller stock. The overrun only shows up at retirement.

### Which stock applies
The stock is `caller.json` `grant`: 16 Mi items, 504 MiB copy, 256 MiB capacity, 512 MiB release. It is one fixture-lifetime cumulative stock. That is about the size of ONE native initiating event (`HostLifetimeStock`: about 28 Mi items, 504 MiB copy, 252 MiB capacity, 256 MiB release), but the whole test spends it.

The test spends it on four Nakagin-scale document operations on one app:
- the boot document,
- `setActiveExample` Nakagin,
- `addObjectKind`,
- `setActiveExample` Nakagin again,
- a 145 KB `importSnapshot`, which decodes, diffs, applies and packs the 181-object document.

### Which axis overflows
Not proven, because the old message printed no numbers. The capacity axis is the most likely:
- The settlement law's ledger shows the boot alone spends about 9.9 MB capacity against 1.4 MB release and 2.8 MB copy (remaining grant in its panic versus `caller.json`).
- Capacity is cumulative allocation that releases never net, and it is the tightest axis (256 MiB).

### Wire budget
The L30 wire fix holds. Both tests no longer fail with "command wire exceeds its declared maximum" (as in l23/l12) and now run for more than 60 s.

### Verdict
The fixture models a per-event budget as a per-app-lifetime budget. That is structurally too small for a test that chains several Nakagin-scale events. It is not shown to be an over-post by the import.

### Fix made
`fixture_remaining` now prints `spent {spent:?} stock {stock:?}`, so the next run names the axis. No stock was raised.

### Recommended clean fix (not done: framework-wide fixture design)
- Have `OriginalFixtureScope` issue one initiating-event stock per dispatch, the way the host does.
- Have the import laws `arm_host_lifetime` around the import so that the import itself is bounded by one `HostLifetimeStock`.

If the next run names release instead, suspect L31's release double-counting family.

## 2. open_vortex_suggestions ceiling law: "original Puzzle work remains in close custody"

### Root cause
It is in the test fixture.
1. `measured_tool_work` moves the ONLY `ArtifactInstanceOperationOwnerHandle` into the work (`.bound(instance)`).
2. The `openVortexSuggestions` dispatch writes into that owner's payload: `ctx.brush_suggestions(|link| link.open_menu(..))` sets `BrushSuggestionsLink.menu`, so `is_empty()` becomes false (E/⏳️precompute/🖌️brush/⏯️suggestions-run/🦀️.rs:78,117).
3. At close, `Puzzle3dWindowCommandWork::begin_close` stages that handle into `WorkClosing` (E/🧵️retained/🎬️action/🪟️window-command-work/🦀️.rs:147).
4. The handle is retired through `RetireOwned` → `ArtifactInstanceAliasRetirement::close_step` → `alias_return_step` → `MountedOwnerAliasRetirementV1::step(grant, terminal_is_empty)`.
5. For the last alias, that step refuses to drop a non-empty payload and returns `Progress(0)` forever (P/🏇️mounted-owner/🔗️alias/🦀️.rs:51; P/📜️declaration/👁️view/🛬️lifted/👷️job/🦀️.rs:392). Only the instance's own cx close (`ArtifactInstanceOperationOwnerHandle::close_step`) may retire the payload.
6. The loop therefore hits 100 000 turns.

### Why the siblings pass
`acceptSuggestion` and `setActiveExample` never fill the owner, so the unique alias payload is empty and it retires.

### Production
Production is unaffected: the mounted instance always keeps its own alias, so the work's alias is never the last one.

### Fix
Both copies of `measured_tool_work` now build the owner handle first, bind a clone, and return the instance's own alias. `measured_cold_runs` holds it until after `original.finish()` and then drops it, as the suggestions-run laws do with their owner. The work's alias returns at once because `Arc::into_inner` gives `None` while the instance alias lives.

Once teardown passes, the per-step ceiling itself is still untested.

## 3. settlement_work_ledger_is_finite_and_progresses: not settled within 200 turns

### Not a stall
- 196 of 200 turns show distinct progress.
- The op moves lookup/ctx (turn 100) → Worker (turn 150) → Retiring (turn 200).

### What the wait cause means
"the pending result page ladder moved nothing" is the note of the LAST unit of a turn. In this turn the ladder had 22 of the 64 `INTERACTIVE_COMPONENT` preparation items left after earlier units spent 42.

`publish_mounted_typed_operation_pump` returns the turn when a unit cannot advance (P/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs:218). So the result ladder advances by at most about 64 items per host turn. The tail moves only 1 to 47 items per turn at about 18 fuel per turn.

### The 200-turn bound is below the real work volume
- The bound has stood unchanged since at least commit a15c8956e6c.
- The rung window `(3000..3012)` shows the author once saw settlements past 3000 turns, so 200 is a deliberate performance target, not a safety bound.
- The sibling `target_volume_action_and_undo_...` settles the same action through `settle_registered_typed_operation` (per-turn `fixture_renew_turn`, 32768-turn bound) and passes.

### Decision
Do not raise the bound. The remaining cost is the item-granular Retiring tail under a 64-item turn grant. It is likely an O(document) retained clone in the result or retirement path (turn 27 copied 500 KB, the whole boot snapshot) for an O(1) edit, the L21 amplification family.

The next step is a measured `[DEBUG]` run that prints, per turn after turn 40, which pump branch moved and the ladder `phase_witness`, to name the rung that pages item by item. No change was made.

## Files changed
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️budgets/🎟️session-and-step-budget-laws/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🍞️geometry-cache-and-step-budget-laws/🦀️.rs`
- `/home/deck/semio/🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs` (assert message only)

## L35b: one initiating-event stock per fixture dispatch (item 1 clean fix, UNVERIFIED, nothing compiled)

### Model
Each `dispatch_fixture_action` is one initiating native event. At the start of each dispatch, the fixture issues one fresh event stock: the authored `caller.json` `grant`, now exposed as `OriginalFixtureCaller::event_stock()`. The host does the same, birthing one stock per event.

This replaces one fixture-lifetime stock with a per-event stock. It is not a raised stock: every event is held to exactly the same grant as before.

### Accounting stays exact
- `StepContext` gains an `issued_start` baseline, default zero at `assemble`.
- `original_total_retained_grant` caps by `issued_ceiling - (current - issued_start)`. With a zero baseline, every context that is never issued an event, including all production contexts, computes exactly what it did before.
- `testkit::issue_initiating_event(cx, grant)`:
  1. checks that the previous event's net spend fit its issued stock, and otherwise refuses with "the previous initiating event exceeded its issued stock";
  2. re-bases `issued_start` to the current net progress and `issued_ceiling.0` to `grant`. The per-turn ceiling therefore keeps the same magnitude for every `original_turn_ceiling()` user.
  3. records the absolute stock (raw progress at issue + grant) under the context's original clock binding.

### Retirement
- `OriginalJobFixtureCaller::settle_lent_fuel` adopts the recorded absolute stock of the last event (`take_issued_event_stock(clock_binding)`). Every lent-context retirement passes through it (`OriginalFixtureCaller::retire` and `retire_decoder`, hence `OriginalFixtureScope::retire`).
- The final stock check (`fixture_remaining`, and `OriginalFixtureCaller::retire`'s `receipt.fits(stock)`) is unchanged in strictness, but now bounds the last event.
- The heap audit (`retained == released`, `roots_released == born`) is untouched.

### Callers checked
- **About 150 `OriginalFixtureScope` files and 531 `OriginalFixtureCaller::admit` sites (2D, 3D, 5D, hub, framework tests):** every in-body issuance goes through `dispatch_fixture_action`.
  - The issued per-turn ceiling equals the stock the context was lent, so every `original_turn_ceiling()` reader (2D/5D/gisterrain testkits, construction quotes, raster ceilings) sees the same number.
  - Contexts that never dispatch behave exactly as before.
- **`OriginalWorkFixture` and per-turn `turn_original` contexts:** these are re-born every turn with a zero baseline and never adopt a stock, so their `spent.fits(stock)` checks are unchanged.
- **Contexts without a clock binding:** nothing is recorded for them.
- **`should_panic` tests on stock messages:** none exist.
- **The test at `P/🧪️testing/🎟️caller/🦀️.rs:131`:** it never dispatches, so `receipt.fits(policy.grant)` still holds.

### Residual risk
- The final `retire_decoder` and root close are funded from what the last event left over.
- A test that dispatches on one thread and retires on another would fall back to the lifetime stock, because the record is thread-local.

### Files changed (L35b)
- `/home/deck/semio/🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`: `issued_start` field, baseline-relative cap, testkit `issue_initiating_event` and `take_issued_event_stock`.
- `/home/deck/semio/🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`: `settle_lent_fuel` adopts the issued event stock.
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🦀️.rs`: `OriginalFixtureCaller::event_stock()`; `retire` checks against the issued stock.
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`: `dispatch_fixture_action` issues one event stock per dispatch.

Note: `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testkit/🦀️.rs` is an orphan duplicate of the inline `testkit` module and is not compiled. It was left untouched.
