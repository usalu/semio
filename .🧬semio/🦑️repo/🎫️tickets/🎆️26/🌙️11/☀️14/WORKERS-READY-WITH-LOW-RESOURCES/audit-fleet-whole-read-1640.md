# Fleet Whole-Read Audit (read-only)

Scope: `/Users/Shared/semio-store/v1` and this machine (g1-server, `.cache/f/...`). Local time CET (UTC+1). Pool samples: S1 17:34:26, S2 17:39:31, S3 17:44:48 (5 min apart). No source edited, no repo commands, no git writes.

## Answer in short

- The six 16 GB machines are not idle. g1-m03, g1-m07, g1-m08 and g1-m10 are actively pulling owner children of the current wave `fc7a04bc` (24234 units). g1-m01 and g1-m04 are idle, held by memory pressure. g1-m02 has stopped (pool.json not written since 17:33). g1-m06 is dead (pool.json frozen since 2026-11-16). g1-m09 has no pool.
- The whole-workspace read (4785 projects) is run by g1-server (reader). Its owner records are fed by the pool waves on the other machines. No sealed whole-workspace generation exists in the store (fleet `pending.json` generation is -1).

## 1. Pool state per origin (S1 / S2 / S3)

Fields: state, claimed/published/running, children/target, reason, ownersPerMinute.

| Origin | S1 17:34:26 | S2 17:39:31 | S3 17:44:48 | Notes |
|---|---|---|---|---|
| g1-m01 (16 GB) | idle, 1/1/0, 0/0, pressure, 0.2 | pulling, 19/4/0, 1/4, starting, 0.6 | idle, 38/13/0, 0/1, starting, 1.8 | Pressure-gated; reason "starting" with 1 child pending |
| g1-m02 (16 GB) | idle, 0/0/0, no wave | same | same, pool.json age 685 s | Not writing pool.json since 17:33:23; likely stopped |
| g1-m03 (16 GB) | pulling, 12/3/3, 4/6, starting, 0.6 | pulling, 40/30/5, 9/9, yielding, 5.8 | pulling, 66/57/4, 10/10, null, 5.2 | Active; recycles children every ~30 s to return memory |
| g1-m04 (linux) | idle, 0/0/0, 0/0, pressure (24 MB/s swap) | idle, same (64 MB/s) | idle, same (78 MB/s) | ready.json stage tools "still running after 8863 s" |
| g1-m05 (windows) | pulling, old wave 5ecd9c21, 16/4/6, 7/13, starting | pulling, wave fc7a04, 0/0/0, 1/13, starting | pulling, 2/0/1, 2/11, starting | Counters reset on wave change; ready.json stage tools stalled and restarted |
| g1-m06 (16 GB) | pulling, wave 1dd2d90c, 2/2/0, 1/1, nothing-fits, 0.2 | identical | identical | pool.json frozen since 2026-11-16 10:41; dead |
| g1-m07 (16 GB) | pulling, 6/2/4, 5/5, nothing-fits, 0.4 | pulling, 18/15/2, 6/6, nothing-fits, 2.6 | pulling, 28/23/3, 8/7, nothing-fits, 1.6 | Only 0 MB free above 2048 MB floor at S3 |
| g1-m08 (16 GB) | pulling, 17/13/1, 5/5, yielding, 2.6 | pulling, 34/25/5, 6/7, starting, 3.4 | pulling, 56/45/6, 7/7, yielding, 4.0 | No idle core |
| g1-m09 | no pool.json (only fleet-auth.json, Nov 14) | - | - | Not running |
| g1-m10 (16 GB) | pulling, 5/3/2, 3/4, starting, 0.6 | pulling, 18/13/2, 6/7, starting, 2.0 | pulling, 50/44/4, 8/9, starting, 6.6 | Fastest growth in window |
| g1-server (reader) | pulling, 7/0/4, 5/5, yielding, 0 | pulling, 9/0/5, 5/5, yielding, 0 | pulling, 19/9/4, 6/6, yielding, 1.8 | Reader; owners/min rising |

Movement over 10 minutes (S1 to S3), published counter: m01 +12, m03 +54, m07 +21, m08 +32, m10 +41, g1-server +9. m05 excluded (wave change reset its counter). m02, m04, m06, m09 zero.

Owner-rate sum at S3 across active pools: m01 1.8, m03 5.2, m07 1.6, m08 4.0, m10 6.6, g1-server 1.8 (ownersPerMinute, self-reported).

## 2. Whole refresh read per origin

- g1-server: yes, runs a whole read. `read-record.json`: reason "request", startedMs 17:35:34, last beat 17:43:02, endedMs null (in progress). read.log: "graph: asking the host for up to 9 worker processes for 4785 projects" and "[owners] owner records present: 183 of 4785, 4602 to infer or fetch". Pool children are the owner wave.
- Other origins: no whole-read ledger in their current files. Their ready-log-workspace stages end in "complete" (m03 Nov 18 04:58, m10 Nov 18 07:08, m07 Nov 18 05:06, m08 Nov 18 05:04). m06 is the only one still in a graph-slice read: "Reading the project graph slice (1083 s)", then stalled and stopped at 10:23.
- Failures on 16 GB machines (service-failure files, newest first):
  - g1-m02, Nov 18 00:50: "graph: project @semio-tech/procedural-generation3d-rs needed 5250 MB in a worker process even when it ran alone, above the cap of 4200 MB" (code graph.farm-overrun). Also Nov 18 09:43: "Slice worker exited with code 124".
  - g1-m03, Nov 18 01:27: "Build source authority keeps changing during inference".
  - g1-m06, Nov 17 06:40: ready-log-workspace-failed; Nov 18 10:23 stalled.
  - g1-m07, Nov 17 23:02: "Original project inference observations required"; g1-m08 Nov 17 23:04 same.
  - g1-m10, Nov 17 12:04: "unserved build.graph.no-progress" (done 17 of 24, 5 minutes idle).
  - g1-m01, Nov 17 21:45: "ts.preProcessFile is not a function" (tooling error, not a graph limit).
  Evidence of a failed whole read: yes on m02 (worker cap 4200 MB vs 5250 MB needed) and m10 (no progress).

## 3. Fleet store

- Sealed whole-workspace generation: none found. `pending.json` (g1-server, atMs 17:37:12) shows generation -1 and 5042 projects. read.log: "graph: no usable generation (empty), building the first one". `priority.json` 5042 projects (atMs 17:35:40). Note 5042 is not the 4785 the read uses.
- `fleet/cache.json` generation 7 (publisher g1-server, 11:32 Nov 18): the cache endpoint config, not a graph seal.
- Only sealed graph publications in the log: g1-m03 "Published the nx-graph of state 30ed064446df ... (3 documents, 4 MB)" Nov 18 04:58; g1-m10 "state c219d6bd9055" Nov 18 07:08. Scope not stated; not a whole-workspace seal.
- Heads (zero-byte, named segment_bytes_events_hlc), newest first: g1-m07 17:34, g1-server 17:32 (segment 154, 1048451 bytes, 3289 events), g1-m04 17:30, g1-m02 17:29, g1-m01 17:18, g1-m05 17:15, g1-m08 17:13, g1-m10 16:32, g1-m03 13:33, g1-m06 Nov 16. No scope-named heads exist.
- Owner-pool wave heads: `fc7a04bc` created 17:13:37 by g1-m04 (linux); `e6019ec0` created 17:12:38 by g1-server.
- Store top level (du -sk, counts): blobs 256 entries 63.2 GB (newest 11-11 17:27); bootstrap 5 entries 10.2 GB (tar, 11-18 11:23); fleet 4 entries 12.8 GB (11-18 11:32); snapshots 9 entries 3.2 GB (11-15); origins 11 dirs 84 MB (11-18 17:33); heads 10 entries 20 KB (11-18 17:34); tmp 0; trash 0; compaction.json (11-14).

## 4. This machine's read (g1-server)

- Phase: owner records (inference). Read started 17:35:34, not ended. Owner-phase start: "owner records present: 183 of 4785, 4602 to infer or fetch". Shared closures primed in 291 s (49 recipes, 0 stale).
- Ledger, current read: present 183 of 4785. Fetched, here, stale: not logged yet in read.log (rotated at 17:32:30). Last complete ledger (read.1795019550058.log, owners 110 of 4785): 3 present, 47 fetched from the fleet, 60 inferred here (26 running), 0 wanted from fleet, 62 stale in the fleet.
- `; reason:` lines: none exist (0 in read logs and in all origin ready-logs). Read record reason is "request".

## 5. Totals for the 10-minute window (17:34:26 to 17:44:48)

- Records published by all pools: 169 (sum of published-counter deltas on wave fc7a04bc: m01 +12, m03 +54, m07 +21, m08 +32, m10 +41, g1-server +9). Counter-based; m05 reset excluded. S3 absolute sum over the six pools: 191.
- Records fetched by the reading machine (g1-server) in the same window: 0 logged. No owners ledger line with a fetched count was written in the window; the last one (47 fetched) predates it at 17:32:30.

Key numbers: published by pools in window = 169; fetched by reader in window = 0 (not logged).

## Scratch

Sampler script and sample outputs: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R213/` (sample.py, s1.txt, s2.txt, s3.txt).
