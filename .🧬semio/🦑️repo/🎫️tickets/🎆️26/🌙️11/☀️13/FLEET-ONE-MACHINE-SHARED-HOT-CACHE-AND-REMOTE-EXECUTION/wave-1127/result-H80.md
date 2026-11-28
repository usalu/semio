# H80: why workers did not converge onto new code, and the reload rule

## Findings (evidence)
1. H79's "digest" is not a code digest. `ready.json.digest` and the run `digest` of `ready-timings.json` hash the declared stage INPUT files (workspace: nine files of `caching/bootstrap`; tools: two). They change only when one of those files changes, so "a579a1413f46 since 10:00" only says the stage was launched before 12:04. At 12:26 all ready workers already showed `46f9c5cbfd67` (m04 `bd282afe`, m05 `1bf75313` differ). No per-role code digest existed in the store, hence the new record below.
2. Replica lag is not the cause for 13 of 15 workers: at 12:28 every attest has the same manifest (526 501 files) and cursors, apply p50 0.2-2 s, p95 1.3-14 s. Exceptions: m14 (`queue.incoming` 19 084, tree `lagging` 10 627 files), m05 (`lagging`, Windows apply p95 110 s).
3. The real lag is the common restart policy (`🛰️sidecar/🧬️schema`): quiet 5 min, maximum deferral 30 min, minimum interval 30 min from process start, applied to agent, scheduler, spool (and graph supervisor). The closure of those roles is edited continuously by 20 lanes, so it is never quiet 5 min and the maximum deferral decides. `build-agent.log` (UTC): agent edit-to-restart 08:00->08:30, 08:32->09:02, 09:04->09:34, 09:34->10:09, 10:13->10:41 (all 1710-1800 s), 11:00->11:13 (810 s); spool 08:39->10:41 (2 h); cache 3 h by H14's design; pool 2-10 min and ready 2-5.5 min were fine. H14's numbers apply only to the cache role (`sidecarRestartPolicyForV1`), H10/H65 guards only touch stages waiting for a holder (never a sidecar role).
4. Stages pin code by design: the ready role restarts in minutes and the next child ADOPTS the stage child; the stage CLI (`repo run-many`, graph reads) keeps the code it loaded for the whole attempt (m03 11:07-12:15, m07 10:40-12:00, m08 10:01-11:33). A stage is stopped for code only when a `semanticsSources` value is bumped (H10: a landing must never cost an hour of reading). Not changed: restarting reads on every landing was the wave-1125 failure, and graph reads resume only partly.

## Rule (schema `Restart.reloadWithinMilliseconds` 900000)
Every role runs the code of its own closure at most 15 min after the first edit of that closure: quiet 2 min coalesces a slot, maximum deferral 9 min cuts a closure edited without a pause, grace 3 min for work in flight, the agent drains served actions for at most 6 min (counted from drain start, was 2 h), minimum interval 5 min, cache 5/10 min with 15 min interval (hand-over, no downtime), pool/ready unchanged. Exempt: the graph server while a held read reports advancing progress (restarts when progress stalls). Stages: next attempt, or an explicit `semanticsSources` bump when a landing must reach running stages. A test asserts maximum deferral + drain <= 15 min for every role.

## Implemented (landed 13:03, locks ready / remote-execution / sidecar, typecheck 0 errors on 50 files)
- Restart numbers and rule in the schema; tests restart 25/0.
- `🛰️sidecar/📇️code-record`: every role writes `.cache/f/sidecars/code-<role>.json` (running vs disk closure digest, `staleSinceMs`); the ready role (and the group server loop) publishes `S/origins/<origin>/sidecar-code.json`. Tests 3/0.
- F8: `sender` and `stopBy` in `ready-timings.json` run records (schema, `readyTimingSenderV1`, stop-sender test asserts the row); tests 2/0 + 10/0.
- Scripts: `T/🧰️scripts/H80/land.py`, `code-table.py` (reads the store).

## Proof: convergence after the 13:00 slot (`code-table.py`, 13:20)
Landings of the slot ended ~13:10. Running digest (12 hex) of the closure of each role per origin; same digest = same code. Source: `sidecar-code.json` records (13:19-13:20).

| origin | agent | ready | pool | graph | spool | current |
|---|---|---|---|---|---|---|
| m01 m02 m03 m06 m07 m08 m10 m11 m12 m13 m14 m15 | d4ebff | 92932d | e74265 | 461a71 | c0b409 | running = disk, 12 of 12 |
| g1-server | d4ebff | 92932d | e74265 | 461a71 | c0b409 | running = disk (cache 399479, scheduler e331b5) |
| m04 (Linux) | 094eb0 | d5a145 | e63b53 | e8bc49 | 6ebe46 | running = disk (other platform closure) |
| m05 (Windows) | 8c0ff1 | 0548fd | 411577 | d80d94 | ebb5a0 | running = disk |
| m09 | absent | | | | | |

Times: first records 13:05 (m01 m02 m08 m10 g1-server, ready 5 min after the landing); at 13:16 12 of 15 origins ran d4ebff/92932d/e74265/461a71/c0b409 (agent, graph, pool, spool restarts at 13:08-13:14 against moving disks, other lanes landing until 13:10); m07 m14 m15 followed by 13:20 (restart 2-4 min after quiet). Convergence of all origins: <= 10 min after the last edit of the slot, 20 min after the slot started. Snapshots: `W/🗑️generated/H80/snap.txt` (deleted when done).

## Proof 2: after the 14:00 slot (14:12, fix landed 14:00, records now rewritten every 60 s)
Running digest of each role, `=` means running = disk (`code-table.py --json`, records 14:11-14:12, 12 min after the slot start).

| origin | record | agent | ready | pool | graph | spool |
|---|---|---|---|---|---|---|
| m01 m06 m10 m15 g1-server | 14:11-14:12 | f1283b= | 6298fa= | c072c8= | 3ae566= | 838f4e= |
| m02 m11 | 14:12 | f1283b= | 6298fa= | 16ab19 (disk c072c8, restarting) | 73a54b (disk 3ae566, restarting) | 838f4e= |
| m03 m07 m08 m12 m14 | 14:12 | f1283b= | 6298fa= | 16ab19 (disk c072c8, restarting) | 3ae566= | 838f4e= |
| m13 | 14:12 | 0ce460 (disk f1283b, restarting) | 6298fa= | c072c8= | 3ae566= | 838f4e= |
| m04 Linux | 14:12 | f529d1= | ad1fcd= | 837b9d= | d90685= | 89004a= |
| m05 Windows | 14:12 | 161341= | 4898f0= | a80067= | 62ad50= | c5bdb6= |

Every origin runs the post-slot closure digest of agent, ready and spool 12 min after the slot started; the pool edits of the slot (edited until ~14:10, quiet 2 min, pool restarts within seconds) were mid-restart on 7 origins. Same story at 13:20 (table above). Last edit to all-current: 13:10 -> 13:20 and 14:10 -> ~14:15.

## Found while proving, fixed at 14:00 (landed, code-record 4/0, typecheck 0 errors)
The recorder rewrote an unchanged record every 5 min but the table calls a record older than 2 min stale, so `current` showed false between 2 and 5 min (rows displayed `x!=x`). Rewrite every 60 s (staged `scratchpad/H80/code-record-fix.ts`).

## Not verified / open
- Cache role not yet observed handing over under the new numbers (needs a closure edit after 13:20; g1-server cache `399479` = disk).
- m04 and m05 digests differ from the macOS set (platform closure); cross-platform equality not claimed.
- Stages still pin code for one attempt (finding 4); a stage-level age bound needs the coordinator's call now that the seal window is lifted.
- m14 replica queue (19 084 incoming) and m05 lag are replica topics, not reload topics.
