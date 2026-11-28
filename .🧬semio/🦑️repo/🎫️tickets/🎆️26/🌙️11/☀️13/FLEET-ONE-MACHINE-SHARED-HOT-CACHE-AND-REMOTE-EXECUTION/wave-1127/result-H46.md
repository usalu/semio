# H46: graph service lost during source admission

## Cause (evidence)
- 05:01 CET m01, m07, m10, m11, m14 failed their workspace stage with the same text (`S/origins/*/ready-log-workspace-failed.txt`). Not the warden (memory.json: no kill of a graph service) and not launchd.
- Service log tail: `[script] build:source-admission ... ENOENT lstat .../⚡️caching/💾️storage/🚀️bootstrap/...`. The 05:00 landing moved `💾️storage/🚀️bootstrap` to `⚡️caching/🚀️bootstrap`; the front door walks the compiler closure, the path vanished, `read` layer gave up after 6 walks (3.4 s of pauses) and the front door died before writing an exit row, so the client reported `lost`. The client did not restart it (`connectHostV1` lost was thrown outside the retry loop).
- The "launchd route refused since this machine started" line is the consent memory (24 h, whole boot); it only explains why the service ran under the calling app, not the death.
- m07 now: no failure since 05:01; attempt 56 waits in the capacity queue (`run-many` behind graph-plan jobs of 5957/4329 MB, 1100+ s). Separate issue, see findings below.

## Fix (landed 06:00 slot, lock graph, script `T/🧰️scripts/H46/apply.py`)
1. `🏗️compiler/🟦️.ts`: source capture in the churn `build` layer (membership change -> typed concurrent edit; 4 rounds, 2/8/30 s pauses, EN/DE notices). `🔏️authorizers/🟦️.ts` exports `membershipChangeV1`.
2. `🚪️client/🛰️plan/🟨️.mjs` (+ `.d.mts`): a service lost while starting is started again, `HOST_LOST_START_RESTARTS_V1` = 3 restarts, 5/10/15 s, EN/DE progress line; `lostStarts` option.
3. `🔐️consent/🟨️.mjs`, schema, host composition: refusal memory bounded to `LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1` = 10 min (then launchd is probed again; refused again = remembered again); reason line says the time and minutes until the next probe in EN/DE.

## Verification
typecheck 0 errors, drift 0; tests: lost-start 3/0 (new), consent 16/0 (window assertions updated), consent-service 31/0; dangling imports clean.
Unverified: the build-layer retry against a real moving tree (compiler test needs the private repository workspace; not run); no live replay of a landing burst.

## Findings
- m07 capacity wait: "an earlier request is still waiting" with spare 1018-1673 MB >= 512 MB needed: head-of-line blocking behind graph-plan requests of 4-6 GB that cannot fit a 16 GB host. Fixed in round 2 below.

# Round 2: capacity backfill (landed 07:00 slot, lock capacity, script `T/🧰️scripts/H46/apply2.py`)
## Cause
`decideHostCapacityV1` makes a waiter that waited `bypassMilliseconds` a barrier: `blocked` (it drains) or `reserved` (it cannot fit even drained); every later request then got `queue-order` ("an earlier request is still waiting") although it fitted. A 4-6 GB graph-plan that cannot fit a 16 GB host beside unmanaged memory kept 512 MB requests waiting for 1100+ s.
## Change (`🌡️host-capacity/📏️decision/🟦️.ts`, gate, schema texts)
- Backfill window: while the holder has waited less than `hostBackfillBoundMillisecondsV1` = clamp(2 x median measured wait of live grants, `bypassMilliseconds`, `agingMilliseconds`), fewer than 3 measurements = `bypassMilliseconds` (identical to the old strict behaviour), a request that fits now runs ahead if its reservation <= `extra` = room once everything ended - holder need - every grant admitted since the holder arrived - admissions of the round (so cumulative backfill can never exceed what the holder leaves). After the bound the holder reserves for good (starvation ticket: aging head drains the rest). The `maximumBypasses` overtaking limit before patience is untouched (barrier only opens backfill from `bypassMilliseconds`). Deterministic: waiter order unchanged.
- Gate stamps `arrivedAtMs` on every grant (source of the measured waits). No row schema change (field existed, optional).
- Waiting text `queue-order` EN/DE now says it holds the next reservation and that smaller requests that fit beside it run first (schema + 5 stored fixtures/tests changed together).
## Verification
New `🧪️tests/🪂️backfill` (5/0) against the independent Python oracle `🐍️.py`: exact admission second and ran-ahead count for backfill (large admitted 120 s, 5 ran ahead) and strict (100 s, 0), no overcommit (most used 7000 <= 8192), no starvation (admitted <= end of what ran at arrival + step), bound arithmetic. Regression: starvation 7/0, capacity main 30/0, gate 34/0, overcommit 4/0, upgrade 10/0, resident booking 7/0, fleet-status 32/0. The pre-existing red `🧾️blockers` (upgrade-8 vector) fails identically on the untouched tree. Typecheck: 0 errors in the touched files; 8 pre-existing errors in warden/retained/scaling test typings (not touched).
## Trade-off / unverified
- Backfill delays a draining holder by at most the end of the grants that already ran when it arrived (oracle: 120 s instead of 100 s), never by later arrivals.
- Not observed live: m07 was ended by SIGTERM by the landing restart and is back in the queue; its next capacity wait is the first live evidence.
