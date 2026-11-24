# W4: why no worker converges to Ready, and the convergence design (2026-11-23, 22:40 to 23:05)

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Evidence: `/Users/Shared/semio-store/v1/origins/g1-m*/{ready.json,ready-log-workspace*.txt,slice-memory.json,ready-timings.json}` (read-only), `repo fleet status --json`. Lock `lock.ready` (released). Landings: `T/landings.md` "Wave 1122-dist W4".

## Verdict in six lines
1. The read of a worker is not slow in its take code (GR15: 190 ms/owner on the coordinator). It is slow in five places that multiply: (a) the passes over the owners are not incremental on a 16 GB machine, (b) every inventory round walks the whole tree and the bound on rounds is 24 (about 100 min) and then ENDS the read, (c) the stage competes with up to 3 to 6 remote units on the same machine, (d) the sealed head of the selection keeps vanishing from the fleet cache, so each restart is cold, (e) the 10-minute stall rule kills the read (14 to 40 min) before any of this converges.
2. Fixed at the root (landed, tested): (a) the taken memo was empty under memory pressure; (b) a lenient last attempt guarantees that the inventory seals on a tree that never stops moving, and the round bound is 4 instead of 12; (c) a worker whose stage runs lends at most 1 remote unit.
3. Not fixed (needs a restart or a server decision, preamble forbids): the head still has no durable retention class on the cache server; the stall rule; the mounted-image `mappingGrant` waits.
4. Design for guaranteed convergence: Ready on a CONSISTENT snapshot, then follow changes: section 4. Most of it already exists (head, per-owner acceptance, volatile paths, repair rounds); what was missing were the three guarantees above.
5. Live effect is not observed yet: the new code reaches a worker only when its graph child restarts (the last observations at the end of this file). Count of Ready workers: 0 of 13 at every look from 22:40 to 23:0x.
6. Prior belief refuted: the "same code everywhere" question (d) is true for 6 of 8 machines (graph children started after 21:45); only g1-m08 (child started 20:35 until it was restarted by the stall rule at about 22:50) and g1-m13 (child started 17:54, still running at 18,115 s of its read) ran code older than GR14/GR15.

## 1. Measured per worker (all numbers from the files above, 22:40 to 22:56)
| worker | graph child started | state at 22:40-22:56 | what the numbers say |
|---|---|---|---|
| g1-m01 | 21:51 (after GR15, W3) | pass 3, `taken 238 of 605` from 2,930 s on (190 s without change); at 22:50 back in the file inventory (361, later 2,548 completed owner checks) | pass 1 to 2 took 2,300 s, pass 2 reached 603 in 8 s (memo replay), pass 3 restarted from 0 at 2,900 s after "owners done 604 of 605" (a moved path); presence session `postVerify` had ONE verification in flight for 716 s, `mappingGrant` 674 s of it (a mapping lease held by someone else, e.g. disk collection while 10.9 GB free) |
| g1-m02 | 22:07 | file inventory, 2,735 s of the stage, 2,549 owner checks, 94,542 observations, `608 owners admitted` (head adopted at start) | 4 rounds of 636 owner checks in about 2,000 s |
| g1-m06 | 22:27, restarted again 22:55 | `taken 233` at 348 s, `taken 251` at 999 s, `336 of 606` on m12 at 1,414 s | the 233 owners stored locally take 1.5 s each, the 372 that come from the fleet advance 18 in 650 s (0.03/s); `reuse: entries 0, remembered 0` (memo empty) |
| g1-m07 | 22:34 | "reading the owners (round 1) 5 of 3600", then "waiting for host capacity (not enough free memory); 3 active grants, 1 queued, 1,311 MB above the 2,048 MB floor" | the STAGE itself queues behind 3 grants (remote units) on the machine |
| g1-m08 | 20:35 until about 22:50 | `taken (pass 2) 503 of 605` from 7,374 s to 7,662 s; then restarted by the stall rule | 503 looks, average 2.0 s, i.e. 1,005 s of look time in 7,662 s; memo `entries 17, remembered 17, reused 1`; presence body 5,371 s over 68 sessions (79 s each, of which the real checks: 11 s in all), postVerify 907 s (13.5 s per session, `reservationOperation`/`mappingGrant` 678 s); fleet `gateTimings` metadata 6 requests 52.6 s (8.8 s each), blob 10 requests 210 s (21 s each); lookups `unavailable 0`, `transport 0` (the breaker refusals GR15 saw are gone) |
| g1-m10 | 21:45 | claimant of the selection; file inventory at 3,489 s with 2,642, at 4,261 s with 10,829 completed owner checks | 10,829 / 636 owners = 17 whole rounds in 4,261 s = 250 s per round; the bound ends the read at round 24 (about 100 min) |
| g1-m12 | 22:32 | "waiting for g1-m10 (then g1-m13) to seal this selection 7 of 1500", later `taken 336 of 606` | waits 8 to 42 poll lines for a holder that needs hours; reads itself after the wait |
| g1-m13 | 17:54 (!) | still `checking the file inventory` at 18,115 s, 2,594 owner checks, 608 admitted | 4.9 hours inside the inventory with code older than GR14/GR15 |
| g1-m03 | n/a | stage `dependencies` in backoff, exit 1 (attempt 11) | does not reach `workspace` at all |
| g1-m04, m05, m09, m11 | n/a | disk (m04, 4.9 of 6.1 GB), windows (m05, dependencies attempt 32), no record (m09), offline 3.9 h (m11) | out of this lane |

Stall kills (the stage process ended by SIGTERM, `ready-log-workspace-stalled.txt`): m01 19:37 to 19:51 (14 min), m07 21:19 to 21:33 (14 min), m06 20:46 to 21:26 (40 min), m08 19:21 to 19:35 (14 min), m10 17:05 to 17:36 (31 min). `ready-timings` of m03 shows 10 attempts of 121 s at the dependencies stage.

Wait chain ("waiting for X to seal"): m03 waited for m07 21 poll lines, m12 for m10 and m13 56 poll lines. The claim moves from machine to machine as holders are killed by the stall rule; nobody holds it long enough to publish.

Passes: the pass counter on m01 is 3, on m08 2. What triggers a pass: a path that moved under the read. On m07, m12, m13 the line "a new pass over the owners starts (round 1): 1 path(s) changed under the read, 0 owner(s) forgotten" is shown: a membership path that no owner read restarts the pass; the memo is the only thing that keeps the old pass's work (see 2a).

## 2. Causes, each checked against the code

### 2a. The taken memo is empty on a loaded machine (landed)
- `B🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` (inferProject, end of an owner): the retained observations of the owner were released when the memory governor reports pressure (`headroom() < hostAvailableFloorMegabytes`, `B🧠️memory/🟨️.mjs:284`), and only AFTER that the memo asked for them (`projectInferenceObservationsV1(cachePath)`, now undefined) so `remember` was never called. Every 16 GB worker is under that pressure all the time: m08 `reuse.entries 17` of 503 taken, m06 `0`; the 32 GB coordinator and idle machines remember (m01 replayed 74 owners in 8 s).
- Consequence: after each moved path every pass looked all owners up again. Local records cost 1.5 s each, fleet records arrive at 0.03 to 0.3 owners/s on a loaded worker, so a pass restarts the slow part.
- Fix: `keepTakenThenReleaseV1` (`B🕸️graph/📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs`): remember first, release second. The rows are references into the shared row pool (1.19 M held references to 284 k distinct rows, about 10 MB on m08), the project text is already kept as the answer.
- Tests (`repo test-file`): `♻️taken-memo/🧪️tests/🧷️pressure` 3 pass (the old order remembers 0 of 60 owners and takes all 60 again, the new order remembers 60 and takes 0; a path that moves in each of 18 passes re-takes exactly the owners that read it, 60 x 3 in all, and the read ends; a path that moved without hint is found by the verification of the memo rows), `♻️taken-memo/🧪️tests` 5/0 (the source-order assertion now names the new call), `🧪️tests/♻️pass-retention` 1/0.

### 2b. Every inventory round walks the whole tree, and the round bound ends the read (landed)
- Code: `B🕸️graph/📥️load/🟦️.ts:~262` `settledInventory` calls `inventory()` per round; `workspaceBuildFilesV1` (readdir of every walked directory) and `buildGraphMembershipV1` (lstat of every member and directory) run in EVERY round (`B🕸️graph/📁️workspace/🟦️.ts:23,135`); only the owner observations are incremental (memo, `forgetOwnerFactsReferencingV1`).
- Measured: m10 17 rounds in 4,261 s (250 s each); `inventoryRounds` 12 (volatile sealing) and 24 (the read ENDS with the moved-path error: `🔁️inventory-rounds/🟦️.ts:29`). With 50 machines editing, a round without a vanished member is rare; m10 would have been refused at round 24 (about 6,000 s) and the stage restarted from nothing, as m01 and m07 were.
- Fix (guarantee): after twice `inventoryRounds` the next attempt is LENIENT: a member or directory that vanished or changed type meanwhile is named to the tolerated set and left out of the walk and the hash instead of throwing (`vanished` callback of `workspaceBuildFilesV1` and `buildGraphMembershipV1`), so the read always ends sealed with the volatile paths named (`[graph-volatile]`, the next read observes them again; this is the existing semantics of volatile, now unconditional). `inventoryRounds` 12 to 4 (schema `B🕸️graph/🧬️schema/🔣️.json`, EN and DE texts). The blank record of a vanished entry never enters the hash.
- Tests: `🔁️inventory-rounds/🧪️tests` 9 pass (new: a fresh path in every round now seals lenient at round 9 with the 8 earlier paths volatile and checks bounded by owners + rounds; the real walk and hash on a tree where a directory vanishes name `flicker` and `flicker/package.json`, do not fail, and equal the hash of the re-walked tree), settle-churn 4, inventory 10, excluded-membership 1, membership-follow 1, source-discovery 1, observed-facts 1.
- Failing and not caused here (before/after identical for my files): `🧪️tests/🌊️vanished-directory` (1 of 2, an owner observation law, `observation/🟨️.mjs` was edited at 15:14 by another lane), and the four harness tests `🩹️repair-rounds`, `🌊️seal-under-churn`, `🧊️stale-facts`, `🔄️candidate` that extract `loadBuildGraphAttemptV1` from the source text and do not define `settleInventoryRoundsV1` (ReferenceError at their line 266).

### 2c. The stage competes with remote units (landed, conservative)
- m07 22:50: the stage waits in the capacity queue "3 active grants, 1 queued". GR15 saw 3 to 6 units beside the stage and lookups of 9 s. `B🌐️remote-execution/🌡️load/🟦️.ts` (`evaluateRemoteExecutionAdmissionV1`): while the view carries a fresh stage claim that does not wait, the worker has at most `stageConcurrency` (schema Policy, 1) slots, refusing with `slots`; the load report asks the same function, so the scheduler stops placing beyond 1. A waiting stage or a stale claim limits nothing.
- Tests: `🧪️tests/🏋️unit-admission` 10/0 (new law over running 0..3: no stage 4 slots, waiting stage 4, running stage 1, stale claim 4), `🔎️why`, `🪑️stage-claim`, `🔎️refusal-reason`, `🧮️single-charge` each 1/0, `🧪️tests/🟦️` 17/0.
- Trade-off, stated: it lowers the offload capacity of every worker that has not reached Ready (all 11 now) from up to 4 units to 1 while its stage runs; the scheduler places on the machines it can and falls back local (`offload.fellBack` 2,280 already). Raise `stageConcurrency` in the schema to relax; it is one number.

### 2d. The head vanishes (operational, partly open)
- Lookup of `2f245baf...` at 22:44: miss, although the coordinator published it at 21:54 and workers m02, m06, m07 adopted it between 22:07 and 22:34 ("608 owners admitted"). The cache server evicts it again under 34 GB of churn (W3). Republished at about 22:50 with `GR14/probe-publish.ts` (620 owners, 26.7 s, with the retention manifest of W3; lookup answers in 2.1 s).
- Open: the server judges the manifest only in enforce mode (retention runs in `dry-run`: `retention.mode`), so the declared tier is not applied yet. A head that must survive needs an explicit class in the store (item 3 of the open list of W3).
- (c) of the task: a published head already ends a wait at once. Test (new, `selected-exchange/🧪️tests` 23/0): a renewing holder that never publishes, a third machine publishes after 150 ms, the small machine returns `waited` after under 1.5 s although its wait slice is 5 s, with zero reads. What does not end the wait is the HOLDER's lifetime: nobody publishes before the whole read ends, and the holders are killed every 14 to 40 minutes.

### 2e. The stall rule and the mapping lease (not changed)
- The stall rule ends the stage after 10 minutes without progress; the progress sign (`takeProgressSignV1`) moves only on a taken owner or an observation; a fleet fetch of 12 s per request with 24 in flight can hold the sign for minutes. It restarts the graph child, losing the process memo (the first restart after landing is when the new code arrives, which is why nothing was forced).
- Presence sessions: `postVerify` waits on `mappingGrant` (a shared lease on the mounted-image mapping, `B..//native-build/💾️storage/🔒️mapping`); a writer of the mapping (disk collection, image maintenance) blocks every presence batch and with it every look; m01 had one verification open for 716 s. A bounded wait with a re-ask belongs to the storage owner.

## 3. What I verified is NOT the limiter
- Fleet lookup refusals by the breaker: 0 on m08 at 7,372 s (GR15 saw 37 of 81 earlier; its fixes work). The cache server answers a lookup in 2 ms (`curl`) and a head miss in 50 ms at 22:44.
- The take code itself (GR15) and the head adoption (the workers with a head show "608 owners admitted" at the first line).
- Disk: m01 10.9 GB free of 233 GB is tight but not the cause of the pace; m04 is (6.1 GB need).

## 4. The design that guarantees convergence (schema first)
Principle: a worker is Ready on a CONSISTENT snapshot (a tree state that existed, minus named volatile paths), and later reads follow the moved paths incrementally; a read never waits for the whole tree to hold still.
1. Snapshot = the head of the scope (owner digests + admission facts + sealed graph, `selected-exchange`); every owner is accepted on its own digest; only differing owners are read (`acceptSelectedOwnersV1`). Already in place; its fragility (2d) is the remaining hole.
2. Inventory = rounds that forget only dirty owners; after `inventoryRounds` every path that still moves is volatile; after twice that the last attempt is lenient (new). The read cannot be refused for moving paths any more; its only failures are real ones (owner faults, memory).
3. Owner passes = the taken memo answers an owner again while its job and rows hold; evicted only by moved paths that its rows name (`evictReferencing`); filled regardless of pressure (new). A pass after one moved path costs the readers of that path plus one verification of the others.
4. Machine = the stage does not compete with lent work (new: `stageConcurrency`).
5. Schema places: `B🕸️graph/🧬️schema/🔣️.json` (`inventoryRounds`, descriptions EN/DE), `B🌐️remote-execution/🌡️load/🧬️schema/🔣️.json` (`stageConcurrency`), taken-memo contract (`♻️taken-memo/🧪️tests`).
Justification against the model: the exchange already is "snapshot then repair"; the volatile mechanism already is "follow later"; they were guarded by hard bounds that turned a moving tree into a refusal. Removing the refusal is the minimal change; a second read model would duplicate both.

## 5. Open items, in order of effect
1. A durable class for the head on the cache server (enforce mode for declared stage outputs, or a class `selected-head` pinned like the live units); until then a coordinator keeper (`probe-publish.ts`) is the only way to have a head after 22:34.
2. The stall rule should restart a read only when the progress sign is flat AND the fleet requests are idle; a 12 s fetch is progress.
3. Mapping-lease waits in presence sessions (bounded, re-asked) and remembering adopted leaves for the same look (GR15 item 3).
4. A coordinator-side head publisher after every complete coordinator read (the coordinator read takes 11 min; workers take 60 to 300 min).
5. g1-m13 runs a graph child from 17:54 (4.9 h) that predates GR14/GR15; it restarts only through the stall rule.
6. Incremental follow-up WALK (stat memo per directory by mtime) would remove the 250 s per round; not done: the lenient attempt bounds the number of rounds, which is the guarantee; the per-round cost stays.

## 6. Observation after landing (Ready count over time)
See the lines appended below.

Workers Ready (`repo fleet status`, "Workers: N ready, 11 working"): 22:40 0, 22:56 0, 22:57 0, 22:58 0, 22:59 0. State at 22:59: m06 restarted at 22:55 and is in the file inventory with the head's memo ("217,203 observations, 46 owner checks"), m12 `taken 353 of 606` (was 336 at 22:52), m02 2,579 owner checks, m10 10,860 (round 17 still, bound not yet reached), m13 3,186, m07 mid-restart, m08 restarting (new run admitted). No worker restarted its graph child because of my landing (nothing was forced); the first restart after 22:55 (m06, m08, m07) runs the new code; the effect on the memo (`slice-memory.json` `owners.reuse.remembered` versus `taken`) and on the inventory rounds (`[graph-volatile]`) is the first thing to read next. Not Ready: 0 of 13 at the end of this lane.
