# Capacity: hold-and-wait deadlock and head-of-line starvation (2026-11-27)

## What failed

Four `cargo-orchestration` and `command` jobs held 1,536-2,600 MB grants for 4.5-5.5 h, zero `rustc` ran, and each job's next unit waited for a wider memory grant (`unit-growth`, operation `cargo-growth`). Spare was 100-150 MB, nobody released, and a 6,004 MB `command` waited 4.4 h at the head of the queue.

Root causes found in the code:

1. **The growth request did not know its parent.** `createGrantGrowthPortV1` queued `cargo-growth` as an ordinary batch request, and deleted `SEMIO_HOST_CAPACITY_GRANT` from the environment. The gate could not tell "work of a running grant needs more" from "new work wants to start".
2. **Growth was blocked by the head of the line.** A waiting request that is patient and drains becomes a barrier (`blocked`/`reserved` in `decideHostCapacityV1`) and holds back every later request, including the growths of the running grants whose memory it is waiting for. The head waits for the grants to finish, the grants wait for their growth behind the head.
3. **Nothing detected the cycle.** The existing `ancestorHolds` rule only sees one job with nothing else running. With 4 jobs `otherRunning > 0`, so no cycle was recognised.
4. **A request with a declared minimum (`repo capacity run --memory 6004`) that the host could never give** (other applications held ~10 GB) was neither "never fits" (that test only compares against total memory) nor "reduced start" (needs `room >= minimum`), so it queued for ever.

## Design

All of it is pure decision logic plus a thin gate integration; the schema comes first.

### Growth ranks before new work (`decideHostCapacityV1`)

`hostGrowthOfV1(ledger, waiter)` returns the live, non-lent grant a request grows the work of (the parent named by its label `unit-growth:<grant id>`). Such a growth:

- is ordered before every request that starts new work (stable partition after `orderHostWaitersV1`),
- is exempt from the head-of-line barrier, the `queue-order` head rule and the session cap,
- never becomes the head itself.

The memory a parent holds only comes back when the parent finishes, so growth first is the deadlock-avoiding order. Requests without `parentGrant` decide exactly as before.

### Deadlock detection and resolution (`resolveHostHoldAndWaitV1`)

New module `host-capacity/⛓️hold-and-wait/🟦️.ts`. Deadlock holds when:

- the round's decision admitted nothing,
- every running root grant (no `parentGrant`, not lent) has a growth request that waited at least `wait.deadlockMilliseconds` (a running grant without a waiting growth still makes progress, so no deadlock).

Resolution:

- **Shrink.** A growth that declares `floorMegabytes` (the unit runs under a lower parallelism or memory budget) is not deadlocked: the existing reduced start of `decideHostCapacityV1` runs it with what the room carries once it waited 60 s. `GrantGrowthPortV1.acquire` now accepts `floorMegabytes` and passes it on as `minimumMegabytes`+`floorMegabytes`. (The admission broker does not supply a floor yet: see follow-ups.)
- **Preempt.** Victims are chosen lowest priority first (batch before interactive), then the youngest grant (least running time lost), then by id, and only as many as it takes so that a waiting request of a grant that stays fits `spare + released`. Released memory is what the victim and its growth bookings are charged (observed, or the reservation while ramping/retained). If nothing would ever fit it answers `none` and the unreachable rule refuses.

Preemption mechanics in the gate:

- The gate that resolves the deadlock (under the ledger lock) writes `grants/<id>.preempt` (`hostPreemptPathV1`) and prints a bilingual `preempting` line.
- The holder's new watcher sees the marker, prints `preempted`, and aborts `handle.preempted` (new optional `AbortSignal` on `HostCapacityHandleV1`). After `Protocol.preemptGraceMilliseconds` (30 s) it stops the process via the existing orphan-watch stop (SIGTERM to itself, the same cleanup path as a broken pipe). The grant file and marker are removed on cleanup.
- `repo capacity run` terminates its child gracefully when `handle.preempted` fires and exits with 75 (`EX_TEMPFAIL`, `PREEMPTED_EXIT_V1`); the error `capacityPreempted` says "run the same command again".
- Finished units stay in the unit cache, so a re-run loses no compile work. The requeue itself is the caller's retry (see follow-ups).

### Starvation: refuse the unreachable fast, age the rest

- `hostPossibleMegabytesV1` = what the host could give if every grant of ours ended (available + ours - floor, within the budget share), plus the memory other applications hold.
- `hostLeastMegabytesV1` = the request's reduced floor, else its declared minimum, never below `estimates.minimumMegabytes`.
- `hostUnreachableSinceV1` / `hostRequestUnreachableV1`: if the least need exceeds what is possible for `wait.unreachableMilliseconds` continuously, the gate throws `memoryUnreachable` (en/de) naming minimum, possible, floor, total, how much other applications hold, and what to do (close applications until N MB are available, or use a larger machine). Memory returned for a moment restarts the count.
- Requests that can fit keep the existing guarantees: interactive-before-batch, aging (`wait.agingMilliseconds` promotes a batch request and it heads its class), bypass barrier, head of line.

### Schema and policy

- **No on-disk record format changed.** An earlier iteration added `parentGrant` to the Request and Grant schemas; live readers on the old closed schema then refused such grants ("written by a newer version"), which stalled the Base publication. That field is removed: the parent travels in the existing `label` (`unit-growth:<parent id>`, `hostGrowthLabelV1`/`hostGrowthParentV1`). The only new file is `grants/<id>.preempt` (not `.json`, so every reader ignores it). A test asserts that `Request`, `Grant` and `Waiter` have no new property. Grant fcBRq6LyB3RGrYXHrNgOPg is no longer in the ledger (`repo capacity status`); all gate tests use `mkdtemp` ledger directories.
- `Policy.wait.deadlockMilliseconds` and `unreachableMilliseconds` (min 10 ms; absent = off), `Protocol.preemptSuffix|preemptGraceMilliseconds|maximumPreemptBytes`, messages `preempting`, `preempted`, errors `memoryUnreachable`, `capacityPreempted` (en and de).
- Shipped policy (`build/🔣️policy.json`): `deadlockMilliseconds` 120000, `unreachableMilliseconds` 180000.
- `createGrantGrowthPortV1` now labels the request with the parent from `SEMIO_HOST_CAPACITY_GRANT` before it drops that variable from the request environment.

## Files

Changed:

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts` (types, `hostGrowthOfV1`, growth ordering/exemptions, message names)
- `.../🎫️gate/🟦️.ts` (resolution, unreachable refusal, marker, holder watcher, `handle.preempted`)
- `.../🧬️schema/🔣️.json`, `.../🟦️.ts` (barrel export)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts` (growth port), `.../⌨️command/🟦️.ts` (exit 75)
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts` (`floorMegabytes` in the acquire ask type)
- `.../host-capacity/🧪️tests/🟦️.ts` (message placeholder values for the new messages)

New:

- `.../host-capacity/⛓️hold-and-wait/🟦️.ts`
- `.../host-capacity/🧪️tests/⛓️hold-and-wait/🟦️.ts`, `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`, `🐍️oracle/🐍️.py`, `🚪️gate/🟦️.ts`

## Tests

JSON scenarios (schema `🧬️schema/🔣️.json`, one-second discrete simulation driving the real `evaluateHostCapacityV1`, `resolveHostHoldAndWaitV1`, `hostUnreachableSinceV1`); the expected event timeline is computed by the independent standard-library Python oracle `🐍️oracle/🐍️.py`, which restates the room arithmetic and ordering rules and shares no code. All six timelines matched the oracle on the first run:

| Scenario | Result |
| --- | --- |
| `four-job-hold-and-wait` (4 x 1536 MB grants, growth 2200 MB each, 2048 MB spare, 6004 MB command waiting) | at t=133 s (last ask 13 s + 120 s) the youngest batch grant `job-3` is preempted; `job-1` grows at 135 s, `job-2`/`job-4` at 195 s, `job-3` and the command at 255 s; every job finishes |
| `four-job-without-resolution` (no deadlock time) | no job ever finishes: the observed failure |
| `four-job-reduced-growth` (one unit accepts 1800 MB floor) | grows reduced (2048 MB) at 71 s, no preemption |
| `unsatisfiable-request` (other applications hold 9500 MB, 6004 MB minimum) | refused at 180 s; smaller requests run one at a time beside it |
| `large-request-aging` (6000 MB batch beside an interactive 1000 MB stream) | admitted at 430 s, 30 s after aging promoted it and drained the stream |
| `large-request-without-aging` | starved for the whole 900 s horizon |

Plus unit tests of the pure laws (victim order, minimal victim set, no resolution when something is admitted/stalled too short/policy off, unreachable timer restart) and gate tests over the real ledger: unreachable refusal in English and German, memory that frees up admits instead of refusing, and two real `withHostCapacityV1` jobs where the youngest receives `handle.preempted`, releases, and the other's growth is admitted (marker removed).

Results:

- `⛓️hold-and-wait/🟦️.ts`: 16 pass. `⛓️hold-and-wait/🚪️gate/🟦️.ts`: 3 pass.
- `repo typecheck` on all changed sources and their 100 importers: 0 errors.
- Capacity suites (`🪟️partial`, `🪜️phases`, `🛟️unsatisfiable` and the rest of the 127 test files of the host-capacity, product host-capacity and admission folders): see the section below.

## Capacity suite status (second round, after the format fix)

Full run of 108 files (all host-capacity, product host-capacity and admission tests plus my new ones): **105 passed, 3 failed** (`repo test-file`, 605 s). Files 107-127 ran separately in 96 s: 18 of 20 passed.

**The `🧵️frontend-threads` stall was lease contention, not a hang.** Alone it passes in 17 s (4 tests, one builds a cdylib serially for 13 s). In the long run it sat behind `native-build remains active on .../bun (shared, 350 s)`: the shared bun lane was held by other agents' long test runs. Running the tail as its own invocation avoids the queue.

Triage of the six earlier failures. Method: I extracted `HEAD` into a scratch directory (outside the repo, `git archive`), and ran the same tests with HEAD, HEAD plus single files of mine, and the current tree; I also ran the decision with and without my growth-ranking rule.

| Failure | Caused by my growth rule? | Cause and action |
| --- | --- | --- |
| followup-2 and 16 other stored worlds (`memory` became `queue-order`) | No. HEAD, current and "rule disabled" decide identically for these worlds | Commit 9c8a98ec234 (the starvation ticket) added the `reserved` barrier for requests that cannot drain; the stored worlds predate it. The independent Python oracle `🌤️followup/🐍️oracle/🐍️.py` already models that barrier and writes the fixture: I re-ran it, so the stored worlds are now the oracle's output (34 reasons changed, all `memory`/`queue-order`, and in two worlds the opposite way plus one `reduced` admission). TS equals oracle on all 220 worlds. |
| `🧾️blockers` upgrade-8 | No | Same cause: the vectors come from `🧾️blockers/🐍️oracle/🐍️.py`; re-ran it (839 vectors), test passes. The upgrade oracle's output equals its stored fixture (unchanged). |
| `🧮️graph-plan` "fair queue ordering even when a plan fits" | No | Passes at HEAD and with all my process files; fails only with another agent's uncommitted `interactiveAhead: true` in the shipped policy. The policy text says an interactive request is not held behind the barrier of **batch** requests, but the code let it bypass an **interactive** barrier holder too. Minimal fix in `decideHostCapacityV1`: the barrier records whether a holder is interactive (`barrier.interactive`) and `ahead` applies only when none is. Graph-plan test (15), the priority oracle test and the decision suite (30) pass. |
| g13/g29 replays (product `🌤️followup`) | No | They fail at pristine HEAD already (10 failures at HEAD, 2 now). Not touched. |
| carving law (`🧢️unit-ceiling`) | No | Reads this machine's real telemetry ledger: bim-model crates peak at 1.6-2.5 GB against the 1.5 GB library budget and are not on the carving list; one listed entry is stale. Needs carving work in those crates, not scheduling work. |
| New this round: `🔬️footprint` (2), image-slack registry priority, `🧯️available/🎫️broker` | No | Footprint: macOS `footprint` binary on this host (one fails at HEAD as well; the other follows another agent's uncommitted sampling edit). Image-slack: disk-writer priorities (91 vs 85) in the disk-recovery registry. Broker: passed alone in 3.5 s, failed once at 40 s under contention. |
| `admission/🧪️tests/🟦️.ts` | n/a | Hit the 420 s limit under contention; not rerun alone. |

Safety note on the regenerated fixtures: I only re-ran the oracles that own those fixtures; nothing was edited by hand. The `git diff` of the followup fixture is the reason changes listed above, no other field.

## Preempted jobs run again (item 3)

- `withBuildCapacityV1` (the invocation wrapper of `repo run/test/cargo`) is now a loop around the old function (renamed `admitBuildCapacityV1`): when the gate preempted the grant (`handle.preempted` aborted), the attempt fails with code `capacity-preempted` and is admitted anew, up to `PREEMPTED_RERUNS_V1` (5) times, with a bilingual line; units finished before stay in the unit cache. Helper `rerunPreemptedV1` is exported.
- The gate now acts on a body that ignores the signal: after `preemptGraceMilliseconds` it terminates its **child processes** (graceful tree termination), not itself, so the failing body returns and the wrapper reruns; only after twice the grace does it fall back to SIGTERM of itself.
- Test `🧪️tests/🔁️preempt-rerun/🟦️.ts` (3 pass): reruns until success with the German and English line, bounded reruns, other errors and successes pass through. Not covered by a test: the child-termination stage of the gate (the grace is a 30 s protocol constant) and a full `withBuildCapacityV1` round trip.

## Broker floor (item 4)

Not supplied. `floorMegabytes` must be the memory a unit needs when it runs under a lower parallelism or budget. The admission broker only knows a recorded peak per unit key (quantile of observed peaks, cold-start model) and the final-link serial rule; the front-end thread bound lives in the Rust scheduler and no model of memory per thread count exists anywhere in the broker or the history. Deriving a floor from, say, the median recorded peak would be a guess that the warden may cancel. The gate, the decision, the growth port (`ask.floorMegabytes`) and the scenario `four-job-reduced-growth` are ready, so a measured memory-per-thread model can feed it later.

## Component driver waiting inside an exact parent grant (2026-11-27 17:xx)

Observed: `prototype-00 waits for a local slot: it needs 3409 MB here, 2650 MB are free above the 4096 MB floor ...; no fleet slot` under `repo capacity run --memory=3400`, nothing else running, for ever.

Root causes (`component-pass/🧵️job-slots/🟦️.ts`, `component-pass/🟦️.ts`):

1. **Growth that nobody serves.** The pass needs 3409 MB beside a 750 MB controller (`baseline`); the grant carries 3400 - 750 = 2650 MB. `ParentComponentMemoryV1.growable()` was true because the growth ceiling was `memory - admissionReserve` (12288 MB), so the pass waited up to `growth.waitMilliseconds` (15 min, and again after each refusal) for the holder to enlarge the grant in place. The holder of an exact `--memory` grant (and of a cargo orchestration grant) never does that; `declare` only teaches the estimate store. Nothing could ever end the wait.
2. **Wrong floor.** The "4096 MB floor" is `admissionReserveBytesV1` (25 % of 16 GiB), the compile-unit admission margin of a host **without** a grant. A grant is already carved from the memory above the capacity gate's floor (2048 MB on this desktop, `hostFloorMegabytesV1`), so inside a grant the floor was counted twice, and the waiting line mixed the grant's remaining 2650 MB with the host floor. The desktop floor of 2,048 MB is unchanged.

Fix:

- Inside a parent grant the local floor and the growth ceiling use the capacity gate's floor (`capacityFloorMegabytesV1`), not the admission reserve: ceiling = machine memory - 2048 MB (14336 MB on 16 GiB), local floor 2048 MB. Without a grant nothing changes.
- A requirement above the grant is carried by a **supplementary reservation of the difference** through the growth path of this ticket (`createGrantGrowthPortV1`, request labelled `unit-growth:<parent grant>`, so the deadlock resolver and growth-first ordering apply): `ParentComponentMemoryV1.acquire` asks `baseline + need - grant` once, the allowance grows by it, the pass starts, and the reservation is released when the component build ends (`releaseGrowth`). If the gate refuses it (for example unreachable beside other applications) the pass is refused at once.
- An **exact** grant (`parentCompilerGrantExactV1`: retained reservation) without an `acquire` path is not growable: a pass that can never fit refuses immediately with the existing message "Component build needs N MiB for the local pass, but the parent granted M MiB ..." (en / de), so callers resize. Non-exact holders that grow in place keep waiting as before (existing growth tests pass: 5 + 21 + 5).
- A need beyond what the machine could ever give (`baseline + need > ceiling`) is refused at once.
- Fixture test `component-pass/🧵️job-slots/🧪️tests/🎁️parent-grant/` (schema, JSON cases, independent Python oracle, 11 pass): fits in grant, the incident (exact grant, no growth path: refused at once; grown by 759 MB supplementary reservation: starts, allowance 4159 MB; reservation refused: refused), no growth at all, footprint counted, beyond the machine, the 2650/2651 MB boundary, and the bilingual message. Typecheck of the three changed files and 52 importers: 0 errors.
- Other component-pass tests: `📥️inputs` (2), `🔏️seal` and `🦀️real` fail at pristine HEAD too (checked in the HEAD extract), unrelated to this change. The existing `🎟️parent` driver tests ("a first local final needs the verified parent grant", "strict fleet fallback refuses local compilation above the verified parent grant") expect a refusal when a grant of 2048 MB cannot carry a 4096 MB pass. With the growth feature already in HEAD they waited 15 minutes for a growth nobody served (the same bug) and now, with supplementary growth, would wait for the real gate; the driver got an injectable `parentGrowth` port and those two tests pass `async()=>undefined` (the gate refuses the supplementary reservation), so they keep their meaning and pass again (2 pass).
Full component-pass run (33 files, 900 s limit, under contention): 20 files finished; open failures are `📥️inputs` (2), `🔏️seal`, `🦀️real` (all fail at pristine HEAD too) and `neutral actual Native driver retires its plan...` / `selected component libraries share one Cargo type identity` (120 s timeout), which I did not diagnose; the last 13 files did not finish in the limit.

## Component-pass tests, one invocation per file (final)

Every one of the 33 files ran alone with a 400 s timeout (`repo test-file`). Result: 27 passed, 4 failed, 2 timed out.

| File (below native-build/) | Result | Time |
| --- | --- | --- |
| 🎟️admission/🪆️parent/🟦️.ts | passed, 5 tests | 3s |
| 🟦️.ts | passed, 16 tests | 13s |
| 🩺️failure-diagnosis/🟦️.ts | passed, 8 tests | 12s |
| 📋️contract/🟦️.ts | passed, 1 tests | 9s |
| 🔁️failure-churn/🟦️.ts | passed, 40 tests | 14s |
| 💾️storage-gate/🟦️.ts | passed, 12 tests | 5s |
| 🔏️seal/🟦️.ts | FAILED | 74s |
| 📥️inputs/🟦️.ts | FAILED | 4s |
| 🦀️real/🟦️.ts | timeout 400 s | 400s |
| 🎭️stub/🟦️.ts | passed, 0 tests | 5s |
| 🧩️closure/🟦️.ts | passed, 4 tests | 32s |
| 🚪️upfront/🟦️.ts | passed, 1 tests | 5s |
| 📸️snapshot/🟦️.ts | FAILED | 262s |
| 🎟️parent/🟦️.ts | passed, 2 tests | 12s |
| ♻️reclaim/🟦️.ts | passed, 2 tests | 39s |
| 🧠️memo-wiring/🟦️.ts | passed, 1 tests | 19s |
| 📸️scheduler/🟦️.ts | passed, 1 tests | 36s |
| 🧶️feature-unification/🟦️.ts | FAILED | 9s |
| 🛑️cancellation/🟦️.ts | timeout 400 s | 400s |
| 🏃️jobs/🟦️.ts | passed, 15 tests | 85s |
| 🔁️retry/🟦️.ts | passed, 10 tests | 74s |
| 💥️failures/🟦️.ts | passed, 8 tests | 67s |
| 🌊️streaming/🟦️.ts | passed, 6 tests | 77s |
| 🧵️job-slots/🟦️.ts | passed, 5 tests | 7s |
| ⏳️job-wait/🟦️.ts | passed, 5 tests | 18s |
| 🎟️stage/🟦️.ts | passed, 6 tests | 48s |
| 🧵️job-slots/🏁️finals/🟦️.ts | passed, 2 tests | 55s |
| 🧵️job-slots/🎁️parent-grant/🟦️.ts | passed, 11 tests | 23s |
| 🧵️job-slots/🌐️fleet/🟦️.ts | passed, 7 tests | 1s |
| 🧵️job-slots/🎟️parent/🟦️.ts | passed, 21 tests | 1s |
| 🧵️job-slots/🧵️plan/🟦️.ts | passed, 4 tests | 0s |
| 🧵️job-slots/🧵️local/🟦️.ts | passed, 11 tests | 2s |
| 🧵️job-slots/🌱️growth/🟦️.ts | passed, 5 tests | 0s |

(The `🧵️job-slots/🎁️parent-grant` row is the new fixture test; `🎟️parent` is the existing driver test now passing again; the `🎟️stage` row ran before its test repair, which passes: 6 tests.)

Diagnosis of the two tests you named:

- **"neutral actual Native driver retires its plan..." (`🎟️stage/🧪️tests`): fixed, test-only.** It is a source-text test that looked for a one-line `if(!options.consume&&...){await inputContinuations.close();await planContinuations.close();}`. The driver source is pretty-printed over several lines at HEAD as well (the one-liner occurs 0 times in the HEAD extract), so it failed at HEAD and on the current tree, not because of my changes. The code it guards is intact (both owners closed, nothing between that and `return completed;`, the plan start before it). The test now compares the source with whitespace removed.
- **"selected component libraries share one Cargo type identity" (`🧶️feature-unification`): not caused by my changes; the 120 s timeout is removed.** The test runs `repo cargo ... --unit-graph` through the real capacity gate against the live ledger of this shared machine, where it queued for the whole 120 s behind a 5,400 MB `command` head ("an earlier request is still waiting and holds the next reservation"; spare 1.0-2.4 GB against its 1,536 MB). It now uses an isolated ledger directory (`SEMIO_HOST_CAPACITY_DIRECTORY` under its temp root), is admitted at once and fails in 2.6 s with the next, independent problem: `Internal error: Undeclared original Repo build selection` from `build/📥️caller` (the nested `repo cargo` of the test does not match the launch the outer `repo test-file` declared). That is a repo-CLI contract question outside this ticket; dropping the inherited `SEMIO_SCRIPT_CAPABILITIES` and related variables did not help, so I reverted that attempt. HEAD cannot be run for this test (the extract has no tool registry), but nothing I changed is on its path.
- Pre-existing at HEAD (checked in the git-archive extract): `📥️inputs` (2 tests: "component Cargo continuation consumes each exact live command view", "component batch effects receive the exact consumed view"), `🔏️seal` ("the driver seals compiler provenance..." and 2 unnamed), `🦀️real` ("a real build over wasm32-wasip2 components...": fails after 39 s at HEAD; times out at 400 s alone here).
- Not diagnosed: `🛑️cancellation` (suite `native-queued-cancellation`) ran silent for 400 s and again for 110 s; `📸️snapshot` fails inside Cargo (`failed to load manifest for workspace member .../base`) in the test's synthetic workspace after 257 s. Neither touches capacity growth, but I could not run them at HEAD (they need the repo test environment), so I cannot say they are pre-existing.

## Follow-ups (not done)

- `repo capacity run` still exits 75 on preemption instead of rerunning (an arbitrary command may not be safe to repeat); callers decide.
- Carve the bim-model crates (carving law) and rerun `admission/🧪️tests/🟦️.ts` alone.
- Waiters of a killed holder are cleaned by the existing stale-waiter logic; a marker of a holder that crashed before seeing it stays until its grant id is swept (it is keyed by a random id and harmless).
