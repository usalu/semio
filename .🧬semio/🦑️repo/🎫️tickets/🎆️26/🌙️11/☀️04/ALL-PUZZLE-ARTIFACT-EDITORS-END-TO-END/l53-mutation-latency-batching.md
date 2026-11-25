# L53: 3D Mutation Latency, Batching Store Units per Host Turn

This is source analysis and edits only. Nothing was compiled or run, because the brief forbids builds while the native batch runs. All changes are UNVERIFIED.

Input: `🗑️generated/l51-3d-mutation-latency.txt`, which records translate = 132 turns and 1599 units, with 800 store-unit runs that stopped on `run.stop.items` 798 times.

Paths are shortened:
- `FW` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `PUB` = `FW/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication`

## 1. Why each store unit moves one item

The `[DEBUG] receiving quote` line in the log already shows the cause: `retained=RetainedCloneGrant { maximum_items: 1, … } phase=RetainedCloneGrant { maximum_items: 44, … }`.

The chain is:
1. The pump (`PUB/🏃️mounted-run-and-pump`) hands every unit `original_lifecycle_grant(cx, remaining)`.
2. `original_lifecycle_grant` (`FW/🔌️plugin/🏛️ownership/⚠️failure`) clamps items to `.min(1)`. This is the framework's "one phase step = one item" rule.
3. `publish_mounted_typed_operation_unit` quotes the receiving scope as `retained.maximum_items.min(phase − scope)`, which is 1.
4. `store_unit_run` (l43) gets `body.maximum_items = 1`. After the first unit, `prior.copied_items >= body.maximum_items` is true, so it stops with `run.stop.items`.

Result: the l43 run never chained a second unit. Every store step paid a full receiving scope, whose headroom from `original_receiving_scope_headroom` is about 10 items.

The cost per turn follows:
- The mounted policy `INTERACTIVE_COMPONENT` grants 64 items per host turn.
- Each step costs 1 body item plus about 10 scope items, so a turn fits about 6 steps.
- 800 steps × about 11 items ≈ 8.8k items ≈ 132 turns. This matches the observed count.

The store itself is not the limiter. `advance_apply_batch` deliberately does one item per call (`item_grant = min(1)`), and the run is the batching mechanism.

## 2. Why translate folds 508 times and delete 18

`apply.fold` counts `fold_batch_item` calls made while a candidate is prepared and not yet folding. These are the stage setup steps, and above all the inverse backing reservation.

The reservation looked like this:
- `fold_batch_item` (`FW/🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/📬️store-owners-batch-fold-and-history`) reserved the staged inverse `PagedList` up to `publication.footprint.work_items`, the gesture's declared rows. It did so one page per unit, and each page costs one item.
- The 3D drag-selection schema declares `"x-semio-inverse-rows": { "bounded": 4096 }`. The bound is honest, because the attraction re-solve can touch any object. So `work_items = 4097`, and the reservation took about 500 page units for an inverse of a few rows.
- delete-object declares `bounded: 65`. That gives a few pages, plus setup, for 18 units.

## 3. Why item close retires 155 items one at a time

The `apply.item_close.retiring` branch (`FW/🏪️store/⚙️engine/🧳️bundle`) closes `stage.retiring`, the folded cursor's boxed retirement tree:
- Each unit ran ONE `artifact_retirement_box_close_step` in one `STEP_WORK` effect, and each step reports one item.
- The 155 steps are a constant shape (the same for translate, delete, 1 object and 180 objects). Each one cost a host-turn item, and before fix 4.1 each also cost a receiving scope.

`apply.item_close.preparation=43` (`spend_item_close_grant`) has the same shape. It was left unbatched; see section 6.

## 4. Fixes applied

### 4.1 The store lane's receiving run gets the whole turn loan

Pump (`PUB/🏃️mounted-run-and-pump/🦀️.rs`):
- The pump now passes `cx.retained_grant()`, which is already bounded by `with_retained_loan(remaining)`.
- It no longer passes the one-item `original_lifecycle_grant`.

Unit (`PUB/📬️typed-operation-unit/🦀️.rs`):
- `publish_mounted_typed_operation_unit(…, loan, …)` narrows `retained = loan` with `items.min(1)`, and every ladder phase keeps using it.
  - Production behaviour of all non-store branches is therefore byte-identical: they received 1 item before.
  - The direct test callers already pass `grant == with_retained_loan(grant)`.
- Only the durable lanes (Artifact, Config, Draft, flagged as `durable`) quote the receiving scope from `loan.maximum_items.min(phase − scope)`. `store_unit_run` then chains units inside ONE scope until the funded items, the copy bytes or the work quote run out, or until a non-progress result.

Accounting is unchanged:
- The summed receipt is checked against the full body grant.
- It is settled once through `consume_receiving_retained`.
- The pump's `spent` is the actual delta, and `remaining.maximum_items == 0` ends the turn.

### 4.2 The last item reserves exactly its prepared inverse

In `fold_batch_item`, the gesture's last item (`stage.folded + 1 >= admitted_items`) already holds its prepared candidate. It now reserves `staged inverse + candidate inverse` rows, never more than the declaration, instead of the whole declared bound:
- Earlier items of a multi-item gesture still reserve the declared bound, because an exact partial page cannot grow later.
- While the candidate is folding (taken), no reservation is made.
- The `TooLarge` gate and the `PreflightingCommit` rows gate are unchanged.
- The source law (`🏙️bin-01/…one-item-publication…`) counts of `try_reserve_exact(` and `reserve_original!(&mut` are unchanged.

Expected effect: translate `apply.fold` drops from 508 to about 10.

### 4.3 One item-close unit chains box-close steps

In the bundle's retiring branch, a new `ITEM_CLOSE_RUN_STEPS = 256` bounds a loop. Each loop step does the following:
- It runs its own `receiving.with_effect(left, STEP_WORK, …)`. The quote rule holds: each effect quotes exactly `STEP_WORK`, and performed work is at most the quote, as before.
- Its grant `left` is the unit grant minus the bytes already spent, so the summed bytes fit the unit grant.
- The loop stops on any of: retirement done, an effect the scope cannot fund (`None`), `Pending`, or an empty step.

The unit reports ONE item with the summed bytes. This is the same aggregation the bundle's own preparation loop already uses (`copied_items: usize::from(…>0)`) and the same "one item per call" convention as the l43 seal window. If the first step yields nothing, the branch returns exactly what it returned before (Blocked or an empty progress).

Expected effect: `apply.item_close.retiring` drops from 155 to 1–2.

### 4.4 Relation to l52 (WorkLimit ceiling)

None of these changes touches `StepContext` ceilings or `with_original_effect`:
- Every new effect runs through `NativeDecodeControl::with_effect`, with its own exact quote.
- The receiving scope refuses an unfunded quote (`None`), and the loop treats that as a stop, never as an overrun.

## 5. Expected count (estimate)

Translate store-run items after the fixes:
- census 2, item close about 1 + 43, folding 18, fold setup about 10, cursor and flips about 5, digest about 65, preflight and publishing about 5.
- Total: about 150 body items, down from about 800.

At about 54 body items per 64-item turn, with one scope per turn, that is about 3 store turns. Delete is about the same.

Unknown that remains: Store units outside the run. The census has `store − run.scope` ≈ 674 (translate) and 769 (delete) pump units that are not receiving runs (staging, returned-snapshot-read returns, accepted receipt, …). Each costs at least one item when it moves. If they are 1-item units, they alone are about 10 turns. To pin them down, the next run carries a new `[DEBUG]` pump histogram (section 7). If they dominate, they need the same "chain inside one unit" treatment.

## 6. Deliberately not done

- **`apply.item_close.preparation` (43).** Its error path (`receiving_fault` → `PreparationRefused` with `preparation_refusal_progress`) cannot also return earlier aggregated progress. Batching it needs a refusal receipt that carries the chained progress.
- **`preflight.digest.seal.takeOwner` (35) and `admission` (13).** These are l43 §3 step 2 (seal the staged edit in place) and are still open.
- **Raising `INTERACTIVE_COMPONENT.maximum_items` from 64 to the host's 256.** This would be framework-wide. It is not needed if section 5 holds.

## 7. Temporary `[DEBUG]` additions (remove with the l43 ones)

- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`: 23 new labels `pump.*`. The array grows from 67 to 90.
- `PUB/🏃️mounted-run-and-pump/🦀️.rs` counts:
  - `pump.call` once per host turn;
  - `pump.unit.{owned_child,child,interaction,inline,page,accepted,pending,staging,other}` per unit;
  - `pump.items.{page,accepted,pending,staging,other}`, the items each kind spent;
  - `pump.stop.{yield,phase_items,stalled,items,stage,done,wall,pumps}`, why each turn ended.

The test's `translate-histogram=(…)` prints them.

## 8. Next run

`-p semio-s-artifact-puzzle-3d-editor --test editor-gestures one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`. Expect `run.scope` ≈ the number of store turns, `run.stop.items` per turn, and `apply.fold` ≈ 10. Then read `pump.items.*` for the remaining constant.

Also run:
- the store laws `semio-os-store*` (bin-01 source law, fold and close laws);
- the plugin `publication-retirement-authority` and `app-typed-command-full-operation` laws (direct unit callers);
- 2D/5D `editor-gestures` (shared pump).

## Files changed

- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs` (unit loan = turn loan; `[DEBUG]` pump histogram)
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs` (`loan` param; one-item `retained` for ladder phases; durable receiving quote from `loan`; docstring)
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/📬️store-owners-batch-fold-and-history/🦀️.rs` (last item reserves its actual inverse; docstring)
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` (`ITEM_CLOSE_RUN_STEPS`; chained retiring close)
- `/home/deck/semio/🧰️framework/🔨️modules/🌱️value/🦀️.rs` (`[DEBUG]` labels)
- `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/ALL-PUZZLE-ARTIFACT-EDITORS-END-TO-END/l53-mutation-latency-batching.md` (this report)
