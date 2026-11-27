# Work-limit (Koordinator, Bearbeiten), 2026-11-26

Status: root cause narrowed by source reading, NOT proven in a browser. The native measurement is queued (see "Measurement").

## Findings

1. The ledger arithmetic is not the defect. `work_conserved` is `work + held_fuel <= fuel`. A return claim holds 2 fuel (`claim()` in `🪙️budget/🦀️.rs`), a closed receiving scope holds 0 (`idle_unquoted_receiving_scopes_leave_no_held_share_on_the_source`). So `held_fuel` is tiny and `work` itself must reach `fuel`. This matches the third branch of the `first_work_refusal` decision table in `🔬️work-limit-root-cause-1125.md`: the operation spent its whole fuel.
2. The fuel is a lifetime stock. `succeed_step` (actor retained-turn context) is the only place the Source stock is renewed, and it requires a quiescent actor (no pending input). Ordinary dispatched epochs only shrink `input.work.fuel` (`update` in `bearbeiten-parser-law.rs`; `borrow_context` checks `budget.fuel <= retained.work.fuel - fresh_work`). A command whose ingress stays `command-pending` for thousands of epochs therefore draws on one 50,000,000 stock (the `big work quote 49999995` line). When the stock is gone every later poll overruns and the bridge re-throws `authority (work-limit)` (an overrun is deliberately not held as a native refusal).
3. The consumer of that stock is per-turn overhead of the contributions ingress, not one oversized effect. Evidence in this ticket: `🔬️bearbeiten-ingress-throughput.md` (26,538-byte Process pack needed 54,913 host-owner calls before the same-turn burst, 2,550 after; the Koordinator trace shows 21,570 replies at epoch 43,313 spending Work continuously). 50,000,000 / 21,570 is about 2,300 work units per reply, which exhausts the stock in the observed time frame.
4. Both panes now run the owners' own guests (`cad`, `process3d`), so the old Demonstrator guest's refusal text is historical until a fresh build is probed. The source fixes that target the consumption are present: the 32-step same-turn burst in `OriginalHostConfiguration::advance`, and the armed command lease (`Lease.armed`).
5. The catalogue `addObject` command-authority rejection is fixed in source by the retained producer (`🔬️cad-add-object-1125.md`), still pending native typecheck and a browser run.

## Test added (language-agnostic fixture plus native law)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🧪️tests/🦀️.rs`: `a_large_pack_settles_within_its_operation_lifetime_fuel` drives every case that declares `maximumWorkUnits` under `renew_each_turn(turnFuel, lifetimeFuel)` (the fixture caller's finite lifetime stock) and asserts the lifetime fuel consumed to reach `ready`.
- Fixture: `🧪️tests/🔣️.json` gains `turnFuel` (65,536), `lifetimeFuel` (50,000,000) and `maximumWorkUnits` on `process-output-cap-skips-and-continues`; schema `🧪️tests/📐️.json` updated. The bound is language-agnostic data any other implementation can replay.
- Third-party oracle: the existing neutral expected outputs (Python JSON projection, `bearbeiten-json-oracle.log` in the throughput note) remain the oracle for the output; the work bound is an own-implementation budget.

## Measurement

Run: `repo cargo --store=shared --configuration=prototype -- test -j 2 -p semio-plugin-host-configuration --features testkit --lib lifetime_fuel -- --nocapture` (log `🗑️generated/work-limit-lifetime-red.log`, queued behind other builds). The `[DEBUG]` line prints turns and work. After it runs, tighten `maximumWorkUnits` to about 1.5x the measured value (the placeholder is the full 50,000,000, so it can only fail if the stock is exhausted).

## Rebuild set

Guests that link `semio-plugin-host-configuration` and the job context: `cad` (Koordinator), `process3d` (Bearbeiten), `sourcing`, and any hub embedding them (`demonstrator`). No host TypeScript change.

## Not done / open

- No fix beyond what is already in source: the cause of the remaining per-turn cost (about 2,300 units per reply) in Koordinator's typed-operation ingress is not isolated; a fresh browser run with the `first_work_refusal` probe is still the decider.

## Structural fix (host-side only, TS), 2026-11-26

Defect: `OriginalOperationSource.issue()` handed every poll the whole remaining stock as the context budget fuel, and `requireStock` only gates the floor before a turn starts, so one poll could drain the stock to zero and overrun the guest ledger (`authority (work-limit)`, re-thrown on every later poll).

Why not a smaller epoch `work.fuel`: the Rust actor context refuses any later epoch whose `work.fuel` exceeds the previous epoch's fuel minus its spend (`validate_currency`), so the issued epoch work must stay equal to the stock. The bounded slice is therefore the context budget fuel (`StepBudget.fuel`, which the guest only requires to be `<= work.fuel`). No Rust invariant is touched.

Change:
- `🔁️succession/📋️policy.json`: `reserveDivisor` 32 (schema updated).
- `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`: `OriginalOperationTurn.budgetFuel`; a turn issued after `requireStock` gets `budgetFuel = fuel - ceil(tenure.fuel/32)`; `spendReserve()` (cleanup, succession crossings, `originalSourceJob(cleanup)`) lifts it; `originalContextBudgetWit` and `OriginalJobFunds` use `budgetFuel`; the emitted worker source carries `ORIGINAL_STOCK_RESERVE`. Result: a gated poll always leaves at least the reserve, the next gate throws the typed `plugin.original-stock-exhausted` (axis `fuel`) once, and cleanup/succession stay funded.
- `🌉️component-bridge-source/🟦️.ts`: the cleanup branch of `exclusive` calls `spendReserve()`.
- Tests: `🔁️succession/🧪️tests/🟦️.ts` (two new laws, SQL oracle, fixture `reserve` rows in `🧫️fixtures/🔣️.json`, schema `reserve`); `🧪️tests/🧬️wit-lowering/🟦️.ts` poll budget fuel 1000 -> 968 (old value encoded the defect).
- Results (repo test-file): retained-turn succession 14/14, retained-turn tests 2/2, job-funds 17/17, bootstrap 3/3, materialization succession 4/4, bootstrap 3/3, lifetime 2/2, wit-lowering 7/7.
- Rebuild: shard worker / component bridge bundle (`repo run @semio-tech/framework-plugin-web:support`) and the site; no Rust guest.
- Honest note: the new tests were written after the edit, so no red run exists; the old wit-lowering assertion failing at 1000n vs 968n was the red.

## Browser measurement (staged Nov 23 demonstrator guest, Bearbeiten, headless Chromium, 2026-11-26)

Probe: `🗑️generated/work-limit-probe-agg.cjs` (the ticket's `bearbeiten-ingress-probe.cjs` with per-receipt work aggregation), served by `overlay-serve.mjs` over the staged site `5dc1b077e86de2aa/out` and the staged modules. The staged cad/process guests are from Nov 21 and the demonstrator from Nov 23, so this measures OLD guest code, not current source. The `first_work_refusal` print is not in any staged core.

Per second, steady state after about 10 s: 620k to 730k work units, 33 to 41 receipts above 100 work (about 17,000 work, 60 copied items, 18 to 25 KB copied each) plus about 3,300 tiny receipts (about 5 work each). Cumulative work 37.4M at 61 s (rising about 650k/s), so the 50M lifetime stock is gone at about 80 s. Meanwhile the command never completes: 4,335 `command-pending` replies, the same 4 pages accepted, 8 `command-complete`. Items processed per second (about 2,300) are about 60 times what one parse of the 20 KB pack needs, so the guest repeats the same ingress work instead of progressing.

Conclusion so far: the drain is repeated ingress work on an operation that is not progressing, not legitimate per-reply cost, and not a quadratic cost curve (work per big receipt is flat at about 17k). Renewing stock per settled reply would only hide it. Whether current source still repeats is decided by the native `lifetime_fuel` test (queued) and the lease fix noted in `🔬️koordinator-wc3-command-ingress.md`; both need guests rebuilt from current source.

## Support build and overlay finding (2026-11-26 06:35)

`repo run @semio-tech/framework-plugin-web:support` completed on the second attempt (the first was cancelled by the memory warden at about 35 min). Its output is only the `🧵️shard` worker and the preview2 shim under `.cache/b/p/m/browser/prototype/modules`. Neither contains the original-source / succession / reserve code (0 matches for `ORIGINAL_STOCK_FLOOR` and `requireStock`). That code is emitted into each guest's `🌉️bridge.js` by `component-bridge-source` through the materialization step that also writes `🛂️.descriptor.semio` and `🔣️.json` for the module (`🏗️materialization/🚀️commands/🟦️.ts`).

Consequences:
- The reserve fix reaches the browser only through regenerated `🌉️bridge.js` files, one per guest module. It needs a jco transpile / materialization of each already-compiled guest, not a Rust recompile. `support` and the shard worker do not carry it.
- The staged guests (cad and process Nov 21, demonstrator Nov 23) have bridges without any stock gate at all, so an overlay probe on them cannot show the reserve behaviour; the browser measurement above is old-guest behaviour only.
- Sequencing: any component already materialized from the earlier bridge source keeps the old behaviour until its bridge is regenerated. Components materialized after 2026-11-26 06:00 pick it up.

Still open: the native `lifetime_fuel` test (stopped on request to free memory, not restarted). It decides whether current guest source still repeats the ingress work; the root-cause drain fix waits on it and on a fresh guest build.

## Native measurement of current source (2026-11-26 08:05)

`a_large_pack_settles_within_its_operation_lifetime_fuel` (isolated store, `-j 2`): the 26,538-byte Process pack settles in 4,378 host turns and 176,563 work units, 0.35% of the 50,000,000 stock, with the exact expected output. The staged Nov 23 guest spent 37,447,596 work in 61 s on the same kind of pack and never completed (4,335 `command-pending` replies). Fixture bound tightened to `maximumWorkUnits` 400,000.

Decision: current source's host-configuration ingress does not drain the stock, so the browser drain on the old guest comes from guest code that predates the fixes already in source (armed command lease, pending-holder retention, the 32-step burst); it is not a per-reply cost that grows with the document and not a case for per-reply stock renewal. No Rust drain fix is made. Limits: this covers the Bearbeiten host-configuration path only. Koordinator's typed-operation ingress (`semio-plugin-instance` lease path) has native lease laws but no native work-budget measurement, and neither pane was verified in a browser against guests built from current source.

Wrapper issue: a shared-store test build waited 30 min on claims held by five `components describe ... --facet browser` bun processes without naming them; `--store=isolated` ran immediately.

## Koordinator setContributions host ingress (2026-11-26 08:50)

Generated fixture case `cad-raw-contributions-lane-at-the-wire-ceiling` (CAD's own declaration shape: unbounded `RawText`, identity mutation; 1,890 x 26-byte units = 49,140 bytes, the largest cad pack the wire notes allow): settles in 386 host turns and 6,164 work units with byte-identical output, bound `maximumWorkUnits` 20,000 (fixture `🔣️.json`, schema `📐️.json`, test `a_large_pack_settles_within_its_operation_lifetime_fuel`, run `--store=isolated -j 2`). Work 0.012% of the 50M stock, so Koordinator's host-side ingress cannot drain it.

Not covered natively: the typed-operation publication after ingress and Set Active Example (mounted owner / covering-grant path). Its native law `a_document_archive_load_and_its_typed_publication_complete_under_the_browser_mounted_grant` lives in the part-4 plugin test host, which did not compile on 2026-11-25 (115 errors from other included tests) and was not retried here. The old-guest Koordinator drain (21,570 replies, repeated `typed_operation`/`command_ingress` turns) is therefore attributed to the lease/pending-holder defect already fixed in source, by analogy with the Bearbeiten measurement, and remains unverified until a guest built from current source runs in the browser.
