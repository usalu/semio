# R147: one budget for the machine's own read and the pool (living report)

Scripts: `T/🧰️scripts/R147/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `patch-pool.py`, `patch-wire.py`, `gen-tests.py` (staged edits and tests), `overlay.ts`, `mkovl.mjs`, `stage-tests.mjs`, `run-staged.sh`, `battery.sh` (run any test against the staged files), `land.py`, `window.sh`, `land-machine.sh` (landing). Landing entry: `landings.md` "R147 landing 1" (lock held 31 s; staged first, lock taken before any live file was touched).

## 1. What was wrong (from R142 cause (a), code read again)

- The pool sizing already read the gate (grants and queue), but the read's own memory governor (`createMemoryGovernorV1`) never appeared in it: a heavy or solo owner of the read waits (or runs) against the measured headroom of the system, and the pool's 7 to 10 children are what holds that memory. Nothing told the pool.
- `ownerPoolSizingV1` stopped yielding to a gate request after `yieldUntilMilliseconds` (10 min): a time-based "stop yielding", the one the task forbids.
- One child was forced whenever `children < 1` (even with the read's reservation leaving no memory): the pool exceeded the spare.
- Swap: only a cliff at 64 MB/s (a literal duplicating the gate's `swap.outMegabytesPerSecond`).

## 2. The mechanism (landed 20:45 UTC)

One budget = the capacity gate of the machine. The read claims from it; the pool gets what it leaves.

1. **Asks** (new member `host-capacity/🙋️asks`, schema first; Ajv cross-check): one file per read process in `<gate directory>/asks/`: `{pid, key, needMegabytes, alone, sinceMs, updatedMs, heartbeatMilliseconds}`. Alive while the process lives and was rewritten within 3 heartbeats of its own heartbeat (no machine constant). A broken ask counts for nothing; writers swallow every failure.
2. **The governor publishes its claim** (`ask` port of `createMemoryGovernorV1`, wired by `graphHostAskV1` in `project-nodes`): the first waiting owner with its cost (`alone` when the cost table gave it a solo limit) while it waits, an alone owner while it runs, nothing otherwise. The same poll that admits owners is the heartbeat.
3. **The host view carries the asks** (`🌡️load`) and the pool reads them (`ownerPoolHostOfV1` -> `foreground.ask`, summed, `alone` if any).
4. **Sizing** (`ownerPoolSizingV1`, pure, Python oracle with 11 new cases):
   - `ask.alone`: pool to 0 (reason `waiting`, children end, SIGKILL after `yieldKillMilliseconds`) for the whole life of the alone owner. This is the "whole machine for that owner"; R145 decides what cannot fit at all.
   - A claim (the ask or what gate requests ask for) counts as held only if it fits once the children are gone (`megabytes <= available + children * childMegabytes - floor`): then the memory arithmetic drains exactly as many children as the claim needs; a claim that never fits drains none (the pool is not the blocker).
   - A gate request that waited `yieldAfterMilliseconds` stops the pool only while the children are what holds it up (memory fits only once they are gone, or no core is idle although they would free one; unmeasured idle counts as held). There is **no time after which the pool comes back** (`yieldUntilMilliseconds` is removed from policy, schema, oracle, fixture).
   - The forced single child stays only where the machine has no work of its own or memory is not the limit; otherwise the pool is 0 ("nothing-fits"), so it never exceeds the spare.
   - Swap: from `swapActiveMegabytesPerSecond` (1) to `swapOutMegabytesPerSecond` (64) the children are cut in proportion, growth stops; both numbers are the gate's own (`🔣️policy.json` `hostCapacity.swap`), pinned by a law. The pool shrinks before the gate stops the farm.
5. The read's farm already gets memory when the pool is idle (R125/R135 `farm-width` laws); the pool takes only idle cores and what the read leaves, so the two share cores in both directions (existing `one-budget` laws kept: pool never has fewer children than the read's bookings leave idle).

Derived windows: the pool polls every `targetPollMilliseconds` (2 s) and kills after `yieldKillMilliseconds` (3 s) measured from the first poll that saw the ask, so the worst case ask-to-memory-free is `targetPoll * (1 + ceil(yieldKill / targetPoll))` = 6 s, plus the governor's admit spacing and poll (the law test asserts this window, not a literal).

## 3. Proof (red first, then green)

Red before: the new laws were written against the old sizing (time bound, forced child, no ask): the old tree fails `yieldUntilMilliseconds` absent, "pool never exceeds spare" (forced child), no alone stop, swap cliff. The control inside the simulation is the old behaviour itself: without the ask the alone owner waits for the governor's own bound (`waitBoundMilliseconds`, 60 s).

New law tests (hosts declared 8 GB / 4 cores and 16 GB / 8 cores, model: R135 host matrix), `owner-pool/🧪️tests/🧮️shared-budget` (11 tests):
1. read asks for an alone grant while the pool holds N children (4 / 8): an in-process simulation with the real governor, real ask files, real sizing and a clocked pool (poll, kill limit) frees the memory and the governor admits the owner within the derived window; control without the ask waits at least `waitBoundMilliseconds`.
2. pool never exceeds the spare: sweep of available memory (256 MB steps), children, claims (0 to the whole host), alone/not.
3. no starvation with small owners: the read's owner still fits after the pool grew, and the pool runs at least one child.
4. swap: monotone, cut before the gate stops anything, 0 at the gate's number, no growth while it swaps; policy numbers equal the gate's.
5. no time makes the pool stop yielding to a request its children hold up (1 minute to 48 hours); a wait the children did not cause does not stop them.
Others: `🙋️asks` 4 (schema vs Ajv, heartbeat, stale/dead/misshapen, never throws), memory `🙋️ask` 3 (governor publishes alone/not alone/clears, a throwing port never fails a run), owner-pool main (fixture + Python oracle, 45 cases) and `one-budget` (3 laws moved on purpose: forced child, never-both-idle formula, unknown-size waiter), sidecar pool test (the "pool comes back after 10 min" law now says it never does by time).

Measured: staged battery of 14 existing files (memory, sidecar, host-matrix, take, governed-dispatch, too-heavy, parent-budget) base vs new: identical except the sidecar pool file where the old 10-minute law moved; the same files fail before and after (memory overrun 6, footprint 1, take/governed-dispatch/too-heavy/parent-budget 1 each, sidecar 3). Under the lock (harness, parallel): all new files, owner-pool sizing, memory main (30) and both standing preflights green in 31 s.

## 4. Slice lab (20:50 to 20:59 UTC, `--slice=repo-mcp --machines=2 --memory=16384 --cores=8 --timeout=540 --keep`, scratch `R147/lab1`)

Lab extension (landed 20:47, `land-machine.sh`, lab tests 16 pass): with `SEMIO_LAB_POOL=1` the machine driver (`slice-lab/👷️machine`) starts the real pool sidecar (`runPool`) beside the readiness loop and prints every 5 s one `budget:` line: the asks in the machine's gate directory and the state of its own pool. Run: the read took 542 s and ended at `taken 52 of 53` on both machines (timeout), 25 duplicate owners.
What the logs show (`lab1/m1-driver.log`, `m2-driver.log`, 109 lines each): the real governor of the read in the real process publishes its claim and the pool of the same machine reads it from the one gate directory, e.g. `budget: asks repo-cache-reset 256 MB; pool idle, 0 children, 0 running, no-wave` (4 samples), `asks repo-task-scope 64 MB`, `asks @semio-tech/print 64 MB` (m1), `asks repo-next-storage 64 MB`, `@semio-tech/print 102 MB` (m2), then `asks none` once the owners ran. **The pool never had a wave in this lab** (`no-wave` in every sample: the lab's reads did not publish an owner-pool wave, so there were no pool children), so the contest "read's heavy owner against 7 to 10 pool children" was not reproduced in the lab. The lab proves the plumbing (ask file written by the governor, read by the pool's host view, heartbeat and withdrawal across real processes, nothing else broke); the contest itself is proven by the simulation law (section 3).

## 5. Not verified / open

- No real contest with pool children in the lab (see above): needs a lab slice whose read publishes a wave (>= `announceMinimumOwners` missing owners with a second machine pulling) and an alone owner; not tried.
- The sim models the runner's drain as immediate for a non-alone claim (the real runner finishes the owner of a drained child first; only `waiting` kills within `yieldKillMilliseconds`). The alone path is the one with the proven window.
- The ask is written by the parent governor only. The farm's children and the alone owners that run inside farm workers are covered because the parent's governor runs the owner slot (`away` gives the slot back while a worker infers: a long alone owner in a farm worker keeps its ask only while its slot is held; `resume` re-announces). Not exercised by a test.
- `alone` comes from the solo limit (`limitBytes`) the take loop passes for owners the cost table lists as `alone`; owners that a 16 GB read refuses to infer at all (`never`) are R145's, and they publish no ask (they wait for the fleet).
- Windows/Linux: the ask directory and pid liveness use `process.kill(pid, 0)` (EPERM counts as alive); not run on Linux or Windows.
- Running pool and read processes keep the old code until restarted; the pool's status text for `waiting` still says "has waited {seconds} s" (the number is the gate request's wait; for an alone ask it is 0).
- Known failing before and after (not mine): sidecar pool tests 3, memory `🛑️overrun` 6, footprint 1, `📥️take` 1, `🚦️governed-dispatch`, `🛑️too-heavy`, `🧹️parent-budget`.

## 5. Not verified / open

(filled in below)
