# G1 result: admission always makes progress on a low-resource machine

Landing record: `landings.md` (entry "G1 admission always makes progress"). Census inputs: `wave-1121/census-graph-memory-gate.md`, `wave-1121/census-low-resource-thresholds.md`.

## What changed (all under `🧰️framework/`, abbreviations HC = `🔨️modules/🏃️process/🏗️build/🌡️host-capacity`, BUILD = `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`)

1. Shrinkable rebuild claim. `HC/🪜️phases/🟦️.ts` resizer returns `minimumMegabytes` (`estimates.rebuildMinimumMegabytes` 2304 in `BUILD/🔣️policy.json`, the same minimum as the `graph` operation) and books `max(minimum, min(rebuild share, learned))`. `HC/🎫️gate/🟦️.ts` carries it into the rebuild upgrade request. `HC/📏️decision/🟦️.ts` admits a rebuild upgrade with the largest amount that fits now (reason `shrunk`, never under the minimum, at least what the own grant already holds, so an upgrade never waits on itself) and refuses `never` at once when the minimum exceeds the capacity. The graph farm follows the grant: the booked megabytes are exported as `SEMIO_HOST_CAPACITY_REBUILD_MEGABYTES` (`BUILD/🌡️host-capacity/🚏️marks/🟦️.ts`) and `BUILD/🧠️memory/🪜️plan-stage/🟨️.mjs` (`grantedHostMegabytesV1`, `planFarmWorkersV1`) plans the farm width against it.
2. Learned estimates. The rebuild phase learns by the median (`rebuildQuantileBasisPoints` 5000), peaks are clamped at the rebuild share (`rebuildCeilingMegabytes` from `BUILD/🌡️host-capacity/🟦️.ts`), the declared class ceiling (e.g. 4096 for show/graph) now applies to the rebuild as well as to the floor. Live store, `phase:rebuild` (15 samples, one at 280000 bp): old rule 28549 MB (clamped to the 14336 share), new rule 3742 MB.
3. Bounded wait, every operation. `wait.deadlineInteractiveMilliseconds` 120000 / `deadlineBatchMilliseconds` 300000. Past the deadline a soft stop or a hard stop (swap, swap-activity, pressure-critical) admits the shrinkable minimum when the room above the floor covers it (`shrunkStopped` line, en/de); below it the stop keeps holding. Waiting lines past the deadline append the holders (`pastDeadline`, upgrade waits also name the run's own grant, `blockerSelf`). `hostRequestNeverFitsV1` refuses at the door (`memoryNeverFits`, en/de) a request whose least need exceeds what the host may give out; the existing `unsatisfiableHostRequestV1` path (it was already wired as `lone` in the gate, the census was stale on this) stays for requests that fit the machine but not the foreign load.
4. One memory authority. `BUILD/🗓️scheduler/💾️store/🟦️.ts` budget = total minus `hostFloorMegabytesV1` (policy fields `hostBudgetMegabytes`, `daemonReserveMegabytes`, `systemReserveMegabytes` removed from the scheduler schema).
5. Doctor. `capacity.wait-long` now carries `blocker` (largest holder) and `holder` (`self`, `other`, `none`), texts en/de (`BUILD/🩺️doctor`).

## Tests (TDD, new cases)
- `BUILD/🌡️host-capacity/🧪️tests/🛟️unsatisfiable`: 16 GB booking >= minimum, self-upgrade, minimum above capacity -> never, door refusal en/de, deadline with soft/hard/swap stops, shrunkStopped and pastDeadline lines en/de.
- `HC/🧪️tests/🪜️phases` + Python oracle `🐍️oracle/🐍️.py` (equal, `--check` passes): one cold run does not pin, share clamp, resize with minimum, class ceiling. `📐️estimates` oracle policy gained the two rebuild fields.
- `BUILD/🌡️host-capacity/🧪️tests/🧮️graph-plan` (16 GB grant), `BUILD/🧠️memory/🪜️plan-stage/🧪️tests/📐️farm-width` (Python model with grants), `BUILD/🗓️scheduler/🧪️tests` (budget equals independent Python).
- Adapted: `🔼️upgrade/🌱️grow` waiter request size 31000 -> 26000 (the old 31000 never fits the host, it is now refused instead of waiting).

## Real checks
`repo capacity status --json` (exit 0): {"host": {"totalMegabytes": 32768, "availableMegabytes": 15227, "pressure": "normal", "swapUsedMegabytes": 20352, "load": 38.72}, "grants": 0, "waiters": 0, "floorMegabytes": 4096, "headroomMegabytes": 11131, "unmanagedMegabytes": 17541}
`repo doctor --only=capacity --json` (exit 0): {"verdict": "ok", "counts": {"error": 0, "warn": 0, "info": 0}, "findings": [], "waits": []}
The 11.6 h incident wait itself is gone from the ledger (no grants or waiters now), so the new rules were proven by the oracle-backed tests, not by reproducing the live wait.

## Known failures not from G1
`HC/🧪️tests/👻️orphan` and `HC/🧪️tests/🤲️lend/🚪️gate` fail identically on HEAD (checked on a `git archive HEAD` copy). Some timing tests (`🔼️upgrade`, `🤫️quiet`, scheduler) flake under machine load and pass on rerun. `BUILD/🧪️tests/🧮️arguments` needs the managed python binding.
