# H25: fleet re-check after a long admission wait, and the suites H19/H20 could not finish

## 1. Re-check (landed 02:00, slot, lock caching-cargo)
- Cause: `awaitCompileAdmissionV1` returned a slot and the wrapper compiled, although another machine may have stored the unit while the unit waited (parked claim, lost claim).
- Fix: `⏳️admit/🟦️.ts` takes `recheck` (a cheap fleet lookup) and returns `served`; it asks once when the wait lasted `slot.recheckAfterMilliseconds` (10 000, schema `🧬️schema/🔣️.json`) or the parked claim was `lost`; on a hit it gives the slot back, prints the new en/de message `recheckHit`. `📜️script.ts` `compile(capture,key,recheck)` passes `fetchFleetUnitV1(fleet)==="hit"`; a served outcome releases the claim and installs the unit (`hit`, reason `after-admission-wait`, counted `remoteHit`); an unservable hit counts `hit-unservable` and compiles.
- Tests: new `🧪️tests/🔄️recheck` 5/0 (long wait asks and serves with en/de line and slot returned; short wait asks nothing; miss or throwing lookup keeps the slot; lost claim asks after a short wait; wiring law). Neighbours after the landing: wedge 10, headroom-first 5, fleet pipeline 7 (law updated: two settle sites), main suite 29. repo typecheck 23 files 0 errors, dangling-imports 0.

## 2. Suites (all foreground; host load 55-66, coordinator Mac)
| Suite | Result | Duration |
|---|---|---|
| unit-cache `🧪️tests/🟦️.ts` (29) | first run 27/2 (load-sensitive), after fixes 29 pass | 717 s then 514 s |
| fleet `🧷️session` | 2 pass | 59 s |
| fleet `⚡️pipeline` | 7 pass (1 law updated for the second hit site) | 12 s |
| fleet `🌳️rooted` | 5 pass (was 2-3 red) | 63 s |
| fleet `🧯️fallback`, `🥀️stale`, fleet `🧪️tests/🟦️.ts` | pass, 20 in the main fleet file | 328 s (main fleet file) |
| admission `♻️replay` | 5 pass (was 2 red); `🐕️watchdog` 5 pass | 6 s |
| topology `🧪️tests/🟦️.ts` | 75 pass | 41 s |
| remote-execution `⌨️command` | 7 pass (was a hang, see below) | 54 s |
| remote-execution `🧪️tests/🟦️.ts` | 17 pass (one load flake fixed) | 95 s |
| scheduler 31, worker 17, sandbox 6, actions 10, sidecar 7, wire 8, endpoint 3, offload 41 | all pass | 239 s together |

## 3. Faults found and fixed (all test hermeticity, no product defect)
- unit-cache main: `waiters of a full slot pool` hung to its 60 s timeout when a waiter acquired the slot before its 2.5 s window (it prints nothing then): the holder is now taken in-process and held until every waiter measured. `three sessions ...` asserted a fixed 8 s lock wait; now bound + longest measured unit time (a loaded host stretches both).
- admission `♻️replay`: the broker read the real host (swap full => `admitted:false`); now started with `available/warden/cancels` overrides.
- remote-execution `⌨️command` serve test hung for ever (0 % CPU): the spawned local worker evaluates REAL host capacity, the busy coordinator has no headroom, the worker never polls, the queued action waits for ever. The test now gives the worker `SEMIO_REMOTE_EXECUTION_HOST_VIEW` (calm simulated host, helper `writeHostViewV1`). Product behaviour (a host without headroom takes no work) is intended.
- remote-execution main: cancelled/timed-out process tree must be gone within 8/5 s -> 40 s (returns as soon as true).
- fleet `🌳️rooted`: `settle()` drained the upload spool with the test process CAS root, so the app unit "was gone" (`upload-entry-gone`) and m2 recompiled it; settle now sets `SEMIO_CAS_ROOT` for the drain. The `leafr.dir()` fixture is `#[inline]` so the app really holds the root on every rustc.
- fleet `🔢️count` is a compiler shim, not a test; `repo test-file` on it fails by design (needs arguments), not a suite member.

## Not verified / notes
- Re-check not exercised against the live fleet; proof is the fake `recheck` plus the wiring law and green real-cargo suites (the path only runs after >= 10 s of admission wait).
- Windows paths untouched.

## 4. Follow-up landed 03:00 (slot, lock caching-cargo): upload worker honours its environment
- `🛰️fleet/📤️uploader/🟦️.ts`: `uploadUnitV1(client,root,job,environment=process.env)` and `drainPassV1` now resolve the content store with `casRootV1(environment)` (was the process env); `uploadRunV1` takes the resolved `cas`.
- Test `🛰️fleet/🧪️tests/🗄️cas-root` 1/0: unit entry in a non-default CAS root; upload with another store in the environment ends `gone`, with the named store ends `true` (the process env names neither).
- Typecheck 20 files 0 errors, dangling-imports 0, landing logged. Neighbours: `✅️complete` 10, buildscript `🌐️fleet-run` 2, pipeline 7, fallback 4, `🌳️rooted` 5 (one run beside four other suites failed its deferred-cache test at 25 s under load; alone it passed 5/5 in 111 s: load flake, not reproduced).
