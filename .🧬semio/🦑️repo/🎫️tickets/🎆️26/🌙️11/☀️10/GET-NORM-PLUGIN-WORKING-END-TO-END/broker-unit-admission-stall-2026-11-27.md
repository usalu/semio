# Broker unit admission stall (2026-11-27)

## Cause
In `🎟️admission/🟦️.ts` the queue law (`admitWaitersV1`) already picks a unit that fits when nothing runs. The unit was then held by the second gate, `availableAdmissionV1`, which compares the unit with measured host spare and the warden level. A host spare of -158 MB (other grants hold the memory) puts the warden at guard or danger, and guard/danger holds every start with no bound (the `waited-out` exception needs a calm warden). Because the held units never started, the output said `waits in the broker queue ... although nothing runs`. The `forced` stall rescue (after 90 s) passed through the same gate, so it did not help either.

The budget itself was not reduced by host spare: `budget() = min(physical - reserve, grant) - nonCompiler`, where `nonCompiler` is the driver's own RSS (about 1772 MB of the 2300 MB grant for memchr). That is "grant minus what is in use" and stays as is.

The lost-scheduler line comes from the running-silence watchdog (1200 s). A SIGSTOPped wrapper sends no rss report, so it was taken for lost.

## Fix (`🎟️admission/🟦️.ts`)
- `availableAdmissionV1` has a new input `granted` and reason `granted`. A unit that fits its grant (or is forced or serial) while nothing of the broker runs starts at once regardless of host spare and warden level. Active swap still holds it. Units with a running sibling, units without a grant and cdylib finals needing physical room keep the old host gate.
- `schedule()` computes `granted` from `parentAllowance`, `running.size === 0` and `predicted <= parentBudget()`, or `forced`/`serial`. The oversize head therefore still waits for growth (`rescueStalled`) and, after the stall bound, starts alone. Smaller units behind a stuck head pass, because `admitWaitersV1` skips a head bigger than the budget.
- New `processStoppedV1(pid)` (Linux `/proc/<pid>/stat`, macOS/BSD `ps -o stat=`, Windows false) and broker option `stopped`. The silence watchdog resets the clock of a stopped process instead of dropping it. No warden change is needed; the warden agent a4821c12e9df56893 was told.

## Tests
- Corpus `🧪️tests/🧯️available/🔣️.json` (6 new cases, schema `granted` and reason `granted`, Python oracle `🐍️.py` updated).
- `🧯️available/🎫️broker/🟦️.ts`: 3 new broker tests (admit at once with spare 0 and warden danger, budget = 528 MB; second unit and swap still hold; oversize head 615 MB does not block 367/339 MB units and starts alone later).
- `🐕️watchdog/🟦️.ts`: stopped unit is not dropped until it runs again; a real SIGSTOP/SIGCONT child reads stopped/running.
- `⏳️waits/🟦️.ts`: the growth-rescue test relied on warden guard holding an in-grant unit; it now holds with swap.

Results via `repo test-file`: available corpus 4/4, available broker 16/16, watchdog 9/9, waits 6/6, parent 5/5, growth 10/10, admission main 18/18, sibling 2/2, tokens 4/4, charged-peak 2/2, deadlines 6/6, all passing.

## Open
- Running processes 92345 and 35455 hold the old broker code in memory and need a restart by their owners.
- A cdylib final with a grant still waits for physical room with no bound (deliberate earlier rule); not changed.
- A waiting (not running) wrapper that the warden stops is still bounded by the queue expiry.
