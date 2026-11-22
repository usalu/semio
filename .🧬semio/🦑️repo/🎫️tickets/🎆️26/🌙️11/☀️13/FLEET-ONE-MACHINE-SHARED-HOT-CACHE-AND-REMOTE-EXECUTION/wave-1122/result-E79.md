# Result E79 (ready-stays-ready), 16:40 CET

## 1. What dropped m11 out of Ready (from code and `S/origins/g1-m11/`)
Not the whole-code digest, not the graph generation, not E58/Nx. The readiness digest is already per stage (`readyDigestsV1`: name, platform, the declared input files of that stage). At 16:02 the file `.../⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts` changed (E68 reader closure work). It is a declared input of the `workspace` stage only (checked by mtime of every declared input: all other stages' inputs are untouched since 04:32). `readyDecideV1` runs the first stale stage and `ready.json` said `ready:false, state:running` for the whole re-verification; the workspace stage is a 15-60 min slice read on a 16 GB Mac (m11 log: 920 s into the read at 16:18, `attempts 0`, reason `adopted`, `readyMs` still 11:14). So the rule was: any landing on one declared input of any stage makes the worker not Ready for the whole refresh; at 3-6 such landings a day and 15+ min per refresh Ready could never be kept.

## 2. Rule landed (T/landings.md "Wave 1122 E79", two landings, area remote-execution)
Ready is a state per stage input digest; a worker that was Ready stays Ready while it re-verifies only the stale stages.
- `readyRefreshV1` (pure): stale stage names, `null` when none, when no marker or `repo`, when never ready, when a failure is recorded (a failed re-verification leaves Ready), when a stale stage was never verified here, or when its last verification is older than `refreshMaximumAgeMilliseconds` (24 h, schema Policy: old evidence is no evidence; found live: m03/m08 carried a `readyMs` of 18 Nov and would otherwise have claimed Ready).
- `readyRefreshingPatchV1` + loop wrapper: while the stage runs, waits or observes, `ready.json` says `state ready, ready true, refreshing true, stage <name>`, reason code `refreshing` (EN/DE text from the schema), refusal null; `backoff` (a recorded failure) passes through, every other state change writes `refreshing:false`. `readyMs` is never cleared; completing the last stage sets it to now.
- Schema first: Status `refreshing` (boolean), reason code `refreshing`, Policy `refreshMaximumAgeMilliseconds`.
- Consumers: head adoption still needs `stage === null` so it never runs during a refresh; `readyForegroundBusyV1` counts a refreshing record as busy (the graph server neither serves nor refreshes while a stage holds the 16 GB machine); silence watch owes beats to a refreshing record (beat = running beat, heartbeat written), a stale refreshing record is `silent`; fleet liveness of a fresh refreshing record is `ready`.
- Unchanged and still holding (tests green): a progressing stage is not stopped by new code (code-stop), stalls are still detected (watchdog), stage completion attribution.

## 3. Proof
New `🧪️tests/🔄️refresh/` (TS + Python oracle `🐍️.py`), 9/0: refresh decision grid (4*2*2*2*... = 1920 rows) equal to the Python implementation; law test: a landing on an unrelated path changes no stage digest, a landing on each declared input changes exactly the stages that declare it (digests equal an independent Python sha256); loop tests: a landing on a workspace input runs only `workspace`, status ready+refreshing during, plain ready after, `readyMs` kept, no update ever says ready:false; shared input refreshes only declaring stages in order; failed re-verification -> backoff, ready false, failure recorded; never-ready worker reports running; old or never-verified evidence is not ready; consumers (busy, silence, liveness).
Other files: ready main 57/0 (round 2: 56/1, one CPU-liveness test red at load 55, not in the changed path), stale-stage 14/0, code-stop 4/0, watchdog 9/0, stage-completion 8/0, silence 6/0, process-tree 16/0, workers 2/0 (first landing; main/stale-stage/stage-completion re-run after the age bound, see landings).
Live (16:29-16:33, after rollout by the replica): m11 `ready/true/workspace/refreshing` pid stable; m03 and m08 showed the same BEFORE the age bound landed (their stale records are from 18 Nov, so that was the false-Ready the 24 h bound now removes).

## NOT verified
- A worker staying Ready across several landings is not yet observed end to end: m11 was refreshing workspace (a >15 min stage) at 16:33; it turns plain Ready when that stage completes and the readyMs stays. Check `S/origins/g1-m11/ready.json` for `ready true` through the next landings.
- Real-machine behaviour of the age bound after rollout (m03/m08 should flip to `running/false`).
- No test on Windows; the logic is platform-neutral.

## Others must know
- "all workers Ready" counters now include a refreshing worker; `repo fleet status` shows it as `ready` with `stage` = the refreshing stage (fleet-status schema not changed).
- A worker whose stage fails during a refresh leaves Ready with a recorded failure and only returns after a successful run.
