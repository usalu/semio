# actionbus3 lane (1108): took over from actionbus2
Rerun action-bus lib: `SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-actionbus3-NN "🧰️framework/🔨️modules/🎯️action-bus/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-action-bus --lib`
Queue note: every run waits 30-60+ min in the cargo capacity queue (load 16-26); poll `$G/W-actionbus3-N.rc`.

## Task 1 DONE: original_command_wire_law (FIXED-verified W-actionbus3-14; whole lib 32/32 W-actionbus3-15)
- Root cause (instrumented W-7/9): the fixture return table was never born. `OriginalJobReturnBirth::advance_inner` funds `grant_admits(table = state.table != Live, publisher)`, but `original_return_refresh_quote` always priced the chain with table=false/publisher=false, so a grant sized from it was Narrow every turn (`admit_original_return_table` returned false forever).
- Production fix: `original_return_refresh_quote` (native-turn/🌱️birth) prices the chain from the birth state (table/publisher births still owed). Verified quote == actual (50 items, 15858 copy, 5984 capacity both sides).
- Test fix (wire/🌱️original/🧪️tests): a denied close turn still pays the per-turn birth-service refresh chain (9 items/6064 copy, legit); the law now expects that quote when the grant admits it, else zero. Instrumentation (`LAST`, all [DEBUG]) removed everywhere.
- Lesson: never eprintln inside `observe_heap_allocations` regions (it perturbs the heap-vs-receipt assertion).

## Task 2 PARTIAL: quoted receives
- Done (retirement path): `with_native_retirement_receiving{,_value}(out, quote, op)` now require a quote and claim exactly it; `StepContext::native_retirement_quote(demand, work)` = 1 item + the cleanup effect's physical demand + max work. Callers ported: plugin tool-run tick/🎮️operations (close_native: quote(d, maximum)), plugin composition publication/🤝️group (quote = its granted close grant, fuel), job construction test, job-receiving tests.
- NOT done: public unquoted `with_native_receiving` / `with_native_receiving_ready` still take `None` -> half-turn rule in `original_receiving_outcome`. 67 call sites (store, plugin, ifc, flow, tests) whose receivers (arbitrary closures/futures) do not expose a quote; a real fix needs each receiver family to publish its quote (e.g. `HistoryFoldJob::receive_quote()`), a campaign across lanes. Debt, not hidden.

## Task 3: fixture equality (edited, verification queued W-actionbus3-20)
- `assert_eq!(roots_released, born)` restored in job testing/🎟️caller (non-audit). W-actionbus3-16 failed 21496 vs 19936: the 1560 B are birth-service allocations (authorities, loans, publisher) never counted as born. Fixture now adds the capacity the birth service receipts per turn (`served`) to `born` (replaces the table-only `table_born`).

## Task 4: pending
- receiving-context target `original-native-receiving-context` (job-context --features testkit): was 3 compile errors after the quote port (ported, run W-actionbus3-21 queued). Then contracts target, then wasm32-wasip2 check of action-bus, job-native-turn, job-receiving.

## Task 5
- Claims log: posted 21:45 that construct (W-construct-217) and toolrun3 typing-edit can rerun. Typing-edit rerun queued as W-actionbus3-19.

Reruns queued: W-actionbus3-19 (typing-edit), -20 (action-bus lib), -21 (receiving-context).
STATUS: PARTIAL W-actionbus3-15 (action-bus lib 32/32 native; quoted retirement receives done, unquoted receive + job-context targets + wasm pending)
