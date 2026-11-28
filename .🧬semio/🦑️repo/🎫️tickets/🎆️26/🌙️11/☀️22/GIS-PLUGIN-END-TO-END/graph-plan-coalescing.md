# Graph-plan coalescing: four lanes, one slice read

Scope: `repo run` / `repo cargo` of four lanes (browser, gismap cargo, terrain cargo, TS `repo run`) started together on this 16 GB Mac. Logs: `🗑️generated/ts-final-*.log`, `browser-s-react-3.log`, `gismap4-*.log`, `terrain-all3.log`, `coalesce-dry-*.log`; service logs `/var/folders/*/T/semio-repo/501/host-*.log`; ledger read with `repo capacity status --json` and `~/Library/Caches/semio/host-capacity/grants`.

## Answers

### (a) Can concurrent slice reads of one tree state be coalesced?

Yes, and the pool was the only place that could do it, but did not:

- `shareKeyV1` keyed a slice load on `[root, arguments (verbatim), tool, graphPolicy, grant]`. `arguments` carries `--lang`, `--dry-run`, agent flags; `grant` is the per-invocation capacity grant (`SEMIO_HOST_CAPACITY_GRANT`). Two invocations of the very same task therefore never shared a load, and the service's own `jobKeyV1` also contains `frame.grant`.
- Different tasks never shared either. But the selection of a `run` is `{kind:"tasks", tasks:[...]}` (schema `🕸️graph/🧬️schema/🚪️selection.json`), a list; the closure of a union of tasks is the union of their closures, so one read of the union holds each asker's own tasks. The planner picks each client's targets out of the returned graph (`createGraphPlannerPortV1.plan`: `indexOf(slice.graph)`, then `visit(targets)`), and nothing on the client side compares `graph.admission.selection` with its own (only `rosterTaskIdsV1` for `*:*`).
- A sealed graph cannot serve them: a `repo run` plan reads a selected slice (`loadSlice`, owners of the closure only); `SEMIO_BUILD_GRAPH_POLICY=current` serves only generation reads (`show`, `graph`), see addendum 16 of `planner-slice-latency.md`. The scope memo is keyed by the selection (`buildGraphScopeKeyV1`), so a different selection is cold by design.
- Net effect before: 4 lanes = 4 loads, 2 pool slots, 2 workers each booking 5 to 6.5 GB (the measured peak x 1.15) and 2 more loads queued, every one verifying and discovering 1,000+ owners of overlapping closures.

### (b) Why does a waiting request "hold" reserved-but-not-resident memory?

It does not; a running planner does, in two ways (ledger at 03:06, `repo capacity status --json`):

1. The service held six `graph-plan` grants (reserved 2781, 393, 551, 478, 349, 708 MB, `retainReservation`). Only the first has an attached process (worker 97143, `observedMegabytes` 5123); the other five have `observed 0`. They are the growth reservations of that one worker booked *beside* the first grant by the service generation that started at 01:00 (addendum 6; the current code grows in place, `growInPlace`). 393+551+478+349+708 = **2479 MB**, exactly the "2479 MB reserved but not yet resident" of the gate line, while the worker holds 5123 MB resident inside the first grant: the same memory is counted once as observed and once more as unrealized reservation (`unrealizedOf` = reservation minus observed, per grant). A worker is in no way "waiting" there.
2. By design a `graph-plan` grant is `retainReservation: true`: its whole reservation stays charged for the grant's life (`reservationOf` in the decision does not ramp it down), and the pool books the full measured peak up front (addendum 12). While a worker boots, waits for the graph state lease, verifies the memo (low residency) the unrealized part is charged in full. This is the price of "no kills": on 16 GB with a 2048 MB floor only about two such grants fit, so every further planner (and every `cargo-orchestration`, 1536 MB, batch, behind them in the fair queue) waits for them.
- Waiting requests themselves hold nothing: `waiters` have no grant; "N MB reserved but not yet resident" is the arithmetic of the grants that are admitted.

Not changed in the gate: ramping a retained planner reservation down would trade the no-kill guarantee for admission; the superseded-generation double count disappears with the in-place growth already in the code. The fix that matters is to book once.

### (c) Does a cargo-only `repo cargo` need a graph slice?

No, and it does not take one. `PLANNED_COMMANDS_V1` (`🌡️host-capacity/🎯️planned-commands`) is `run, run-many, affected, exec, watch, inputs`; `invocationPlanRequestV1` returns `undefined` for anything else, so `repo cargo` sends no plan request to the service, and `PHASELESS_COMMANDS_V1` (`cargo`, `command`) keeps its capacity estimate free of planner phases. Evidence: `gismap4-config.log` (1,310 s), `gismap4-lock.log`, `terrain-all3.log` contain no "Reading the project graph slice" line, only `cargo-orchestration` waits. The cargo lanes suffer only as gate neighbours: their 1536 MB `batch` request queues behind `interactive` graph-plan requests ("behind graph-plan (running, 1 job(s), 5123 MB) ..."), so fewer planner bookings free them too.

## Root cause

The slice pool shares loads only on the exact (arguments, grant) identity, so concurrent lanes each pay a complete read and a complete 5 to 6.5 GB booking; with a 16 GB host, 2 slots, a 2048 MB floor and `retainReservation` grants, the first two bookings leave 0 MB "above the floor" and every other request (planners and cargo) starves while those two read overlapping closures, for 15 to 30 minutes each.

## Fix

Schema-first, language-agnostic rows, no shims.

1. `🕸️graph/🚪️admission/🧺️coalesce/🟦️.ts` (new): `coalesceBuildGraphSelectionsV1` (union of two `tasks` selections that agree on `excludeDependencies`, bounded by `maximumTasks`, canonical order; `all`, `owners` and mixed dependency rules stay separate) and `buildGraphSelectionCoversV1` (a `tasks` selection contains the `tasks` selections whose tasks it lists, a following read contains a direct one; everything else contains only itself, so a scoped task never waits for a whole-workspace read).
2. `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` `share`:
   - identity is `(root, tool, graphPolicy, selection)`; flags and grant no longer split a load (`selectionOf` parses `request.arguments` with the same `buildInvocationGraphSelectionV1` the worker uses; requests without a parsable selection keep the exact old key);
   - a client joins a queued or running load whose selection covers its own;
   - a client with other tasks joins a load whose worker has **not spawned yet** and the load then reads the union (`group.open` closes at spawn, so a started worker and its checkpoints are never changed under it);
   - a `tasks` load gathers `sliceCoalesce.gatherMilliseconds` (1500 ms) before it asks for a slot, so lanes started together arrive together (a lone request pays 1.5 s once on a 15+ minute read);
   - the group's `grant` is kept only while all members agree, else dropped (the plan then books at the machine gate instead of inside one lane's grant that may end first);
   - a merged load that fails is read again per client (`counters.split`), so one bad task refuses only its own lane;
   - `stats()` reports `joined`, `merged`, `split`.
3. `🏡️resident/🧬️schema/🔣️.json`: `sliceCoalesce: {gatherMilliseconds: 1500, maximumTasks: 16}`.
4. `slice-pool/👷️worker.ts` passes `workerData.selection` and `🧭️invocation/🕸️graph/🟦️.ts` `loadBuildInvocationGraphV1` takes it as an optional last parameter (`selected ?? buildInvocationGraphSelectionV1(args)`).

Files changed:
- new `🕸️graph/🚪️admission/🧺️coalesce/🟦️.ts`, `🧪️tests/🟦️.ts`, `🧪️tests/🔣️.json`
- new `🏡️resident/🧪️tests/🧺️slice-coalesce/🟦️.ts`, `🔣️.json`
- edited `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`, `👷️worker.ts`; `🏡️resident/🧬️schema/🔣️.json`; `🧭️invocation/🕸️graph/🟦️.ts`
- tests that modelled "different slices" with arbitrary strings (`["1"]`, `["leader"]`: all whole-workspace selections, hence one load now) given realistic selections: `🏡️resident/🧪️tests/⏳️queue-progress/🟦️.ts` + `🔖️checkout.json` (owners selections), `🔁️restart/🔣️slot.json` + `🟦️.ts` (tasks, no gather).

## Tests (`repo test-file --lang=en`)

- coalesce: 3 pass (9 union rows, 8 cover rows, commutativity/idempotence checked against plain set arithmetic).
- slice-coalesce: 6 pass (four lanes with four grants = 1 worker on the union selection, no grant; same task twice with two grants = 1 worker without grant; with one grant keeps it; late member joins the started worker and a task outside it gets its own; direct vs following never merge; a failing merged read is re-read per lane).
- Regression: slice-priority, booking, grant-growth, cut-load, reservation, lease-wait, restart, plan-guard, restart-cause, restart-quiet, cut-stops, cut-order, pass-notice, plan-limit, queue-progress, cancel, load-outlives-idle, host (39) all pass.

## Second finding while measuring: growth of a running planner starves behind every new request

With the four lanes served by one worker the run still stalled twice (466 s and more than 8 min) in "waiting for a host capacity decision on the 523 MB / 1097 MB request": the pool stops the worker while the gate answers its growth. Ledger at 03:46: 4231 MB available (2183 MB above the 2048 MB floor), the growth waiter (`upgrade`, minimum 256 MB) last in the queue behind three `cargo-orchestration` 1536 MB requests and a 480 MB plan, none of which fit (the head blocks, `queue-order`). `orderHostWaitersV1` ranked every `grow` waiter after every other request.
Change: `🌡️host-capacity/📏️decision/🟦️.ts` `orderHostWaitersV1`: the growth of a grant that retains its reservation (a graph plan, whose stopped worker holds gigabytes nobody else can use) is ordered like any request of its priority and arrival; growth of other grants and elastic requests stay last. Test `🌡️host-capacity/🧪️tests/🟦️.ts` (30 pass, row "growth of a grant that retains its reservation"). Suites upgrade 10, starvation 7, gate 34, grow 1, graph-plan 15 pass. `🧾️blockers` fails on vector `upgrade-8 z7l5nahnio08` with and without this change (another lane's head-of-line edit of the same decision file changes the queue the oracle models); not touched.
This lands in the resident service when it reloads; the running dry runs below used the old order.

## Measurements

Before (logs in `🗑️generated`, one lane each, same host class, 6 to 8 active grants):
- 4 lanes = 4 graph-plan requests of 5 to 6.5 GB (6313 / 6553 MB, minimum 2526 / 6553), `repo capacity status`: 6 grants + 3 queued graph-plan requests, gate "0 MB available above the 2048 MB floor", 2479 MB "reserved but not yet resident".
- `ts-final-s-plugin-composition`: never left planning, killed by the 3000 s timeout (`rc=124`); `ts-final-gis-gisterrain`: queued 536 s at the last line; `browser-s-react-3`: queued 557 s for a 682 MB request, then 1588 s in the inventory at 457 of 600 owner checks, 192 cut/restart rows.
After (`🗑️generated/coalesce-dry-1..4.log`, `repo run <task> --lang=en --dry-run`, started together at 03:15 on a fresh service with the new pool, targets `gis-gisterrain:test`, `s-plugin-composition:test`, `gis-js:test`, `framework-os-dev:dev-s-react-prototype`):
- One graph-plan request in the gate for all four (`6553 MB requested, minimum 2622 MB`, identical line in the four logs), one slice worker (pid 66203, one `slice-pool` child for the four clients), one growth sequence (3180 granted, then +523, +660, +581, +798, +1097 MB in place into the one grant: one ledger row, 5742 MB reserved against 5479 MB observed, no row with `observed 0`).
- The four clients show the same progress at every second (admitted after 418 s in the gate queue, behind other agents' grants; then "waiting for the graph state lease" 8 s; inventory 169 of 634 admitted owners at 1526 s). Booking: 1 x 6.5 GB instead of 4 x 6.5 GB; workers: 1 instead of 4 (2 slots, the others queued).
- The dry runs had not finished when this note was written (the union closure includes the 1,259-owner `dev-s-react-prototype` closure, cold on this selection, plus two growth stalls of 466 s and 8+ min before the gate change above); no completed wall time exists. They were left running; their logs end with `exit 0` when done.
