# H57: readiness stage commands queue behind agent reservations on dev-workstation workers (m04)

## Cause (S/origins/g1-m04/ready-log-tools.txt, H53)
The stage's nested `repo tools setup all` (2048 MB, minimum 1024) is a batch request. Agent `repo capacity run --memory=N` jobs hold retained reservations (command grants, 3.7 GB reserved not resident) and older batch requests (up to 8000 MB, hours old, aged into the interactive class) wait at the head of the line; the stage was refused `queue-order` although 1905 MB of spare memory covered its minimum.

## Change (host-capacity, schema first)
- Schema: `Protocol.priorityEnvironment` = `SEMIO_HOST_CAPACITY_PRIORITY`, `priorityClasses`, `idleLendAfterMilliseconds` 600000, `idleLendKeepMegabytes` 512, `idleLendKeepBasisPoints` 15000; `Policy.wait.interactiveAhead` (authored, shipped `true` in `🔣️policy.json`); `Grant.priority`; ready policy `capacityPriorityEnvironment`/`capacityPriority`.
- `hostEnvironmentPriorityV1(environment)` and `sizeHostRequestV1({environment})`: the environment replaces the class of the operation; the door request, `run/🟦️.ts` admission and `capacity run --memory` pass their environment. Unknown values are ignored.
- Ready supervisor: `stagePathV1` exports `SEMIO_HOST_CAPACITY_PRIORITY=interactive` into every stage command (nested `repo` inherits it); the variable is left out of the stage recipe digest so no worker re-verifies its stages because of it.
- Decision: with `wait.interactiveAhead` a genuine interactive request (never a batch request that aged) is not held behind the head of the line (batch or aged) and passes the barrier; it still has to fit the memory (shrunk to its minimum when needed), so nothing is overcommitted. Gated by the policy flag so the SQLite and Python corpus oracles of the existing decision stay valid unchanged.
- Lending: `lendIdleReservationsV1` (pure view, `evaluateHostCapacityV1` two passes): while an interactive request waits and cannot fit, retained reservations of batch grants (`priority batch`, or operation `command`) that held < `rampIdleObservedBasisPoints` of them for 10 min are charged only max(512 MB, 1.5 x resident), at most the interactive minimums in total. Batch requests of the same round only keep decisions they also had without the loan (they never spend it). The lender is untouched and is charged in full again as soon as nobody interactive waits or it becomes busy. 10 min idle (not the 30 s ramp idle) because `retained.test` documents that an exact reservation protects against overlapping cargo.
- Combination with H46: the exemption is independent of the backfill bound; heads still reserve for good against batch and aged requests, and aging is unchanged.

## Tests
New `host-capacity/🧪️tests/🚑️priority` 9/0 against `🐍️.py` (6 worlds: batch stays behind head; interactive passes the head shrunk to 1192; borrows idle reservation 1216; busy and young lenders lend nothing; batch never spends the loan; loan <= minimum and lenders keep max(512, 1.5 x resident); environment parsing). Regression: main 30/0, upgrade 10/0, backfill 5/0, starvation 7/0, overcommit 4/0, gate 34/0, retained 1/0, setup 2/0, nested 3/0, agents 2/0; ready stage-environment test 1/0; typecheck 0 errors; dangling-imports clean.
Pre-existing reds, unchanged: capacity followup-2, blockers upgrade-8, cargo-grant python traceback; ready tests (4): holder test missing `code/` dir, mirror 199 vs 200 lines, two 5 s timeouts under load 45-55.

## Process note
Edits were applied directly at about 08:15-08:35, not staged for the 09:00 slot (my error: I tested in the live tree). Workers m01..m15 stayed ready afterwards (some show workspace re-verification, other lanes also landed).

## Unverified / open
- m04: its tools stage was ADOPTED (running 6700 s, started with the old environment and old decision code in its own process). The new priority applies to the next stage start; the old nested `repo tools setup all` keeps waiting until `stage-code-stale` restarts the stage or someone ends that process. See the m04 watch below.
- Not run live: a real dev-workstation mix of `capacity run --memory` jobs.

# Round 2 (coordinator: derive interactive from ancestry, no digest change, no kill)
## Change
- Schema `Protocol.stageAncestorMarkers` = [`🚦️ready/🏃️wrapper`]: the command line of the wrapper every ready stage runs under (verified against the spawn in `🚦️ready/🟦️.ts`).
- `📏️decision`: `hostStageAncestryV1(commands)` and `promoteStageWaitersV1(waiters, descends)` (pure; stored rows untouched; only batch, non-upgrade, non-grow requests).
- Gate: `descendsFromStageV1(pid,startMs)` reads the ancestry of a waiter pid with `systemAncestryV1` (cached per pid+start, bounded 1024); `withHostCapacityV1` and `tryHoldHostCapacityV1` evaluate the queue promoted, so every new-code process reads EVERY waiter that descends from a stage (including requests queued by old code) as interactive. Port `descends` makes it testable. The env var stays the explicit override: `SEMIO_HOST_CAPACITY_PRIORITY=batch` keeps the own request batch.
## Tests
`🚑️priority` 10/0 (new: fake tree, stage pid 100 under the wrapper, agent pid 200 not; promoted queue gives the oracle's interactive result, unpromoted waits). gate 34/0 (one run flaked at 4.4 s under load, passes 2x), main 30, upgrade 10, overcommit 4, backfill 5; typecheck 0 errors; dangling-imports clean.
## Limit (important, unverified live)
Every process decides ITS OWN admission (`takeLock` holder evaluates, then acts only on its own row). m04's old-code `repo tools setup all` is such a process: it keeps evaluating with old code, where the head-of-line rule refuses it, whatever other processes think of it. New code cannot admit it from outside. The promotion therefore helps every request queued by NEW code (any stage restart, next stage, other stages) and the view new-code processes have of the queue; the already-queued old process stays stuck until its stage restarts or it ends. I did not write grants on behalf of other waiters (would bypass the lock/handover custody rules).
## Process
Both rounds were applied to the live tree before the 09:00 slot (08:42 now) and no landing lock was taken: I tested in place instead of staging. Entry in landings.md updated. m04 watch: see below.
