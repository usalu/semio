# Fleet status audit 2026-11-19 00:30 UTC (read-only)

Snapshot taken at 2026-11-19 00:28 to 00:30 UTC. Ages are relative to 00:28:45 UTC. Local clock is UTC+1 (the `ls` output of the origins folder shows local times). No file was changed except this report. No sub-agents were used.

## 1. Origins: status per machine

Source: `/Users/Shared/semio-store/v1/origins/<origin>/` (`ready.json`, `attest.json`, `head.json`, `pool.json`, newest `ready-log-*.txt` and `service-failure*.txt`).

| Origin | Host / platform | ready | state, stage | attempts / exit | ready.json updated (UTC, age) | attest.json age | head.json age | pool.json (state, units, claimed, published, stored, held, failed, running, slots) | Verdict |
|---|---|---|---|---|---|---|---|---|---|
| g1-m01 | mac-mini-m6 / macos | true | ready, digest 70ffaf34 | 0 / null | 23:11:57, **77 min** | 75 min (00:13 local) | 80 min | pulling, 23594, 159, 88, 133, 1, 8, 2, 5 | **Ready but silent** (no status update > 60 min). Pool still lists pulling, last error "Native dependency has no build project owner" |
| g1-m02 | mac-mini-m4-01 / macos | true | ready, digest 70ffaf34 | 0 / null | 00:27:42, 1 min | 0 min | 6 min | pulling, 18941, 2, 1, 29, 0, 0, 0, 2 (reason: yielding, 0 of 10 cores idle) | Ready, progressing |
| g1-m03 | mac-mini-m4-03 / macos | true | ready, digest 70ffaf34 | 0 / null | 00:27:40, 1 min | 0 min | 0 min | pulling, 23530, 58, 41, 709, 4, 7, 6, 9 (last error: record not stored, fleet cache did not answer) | Ready, progressing |
| g1-m04 | ueli-workstation-lin-1 / linux | false | running, stage tools 4/5, digest 304d5d98 | 48 / 1 (previous-failed, retrying after back-off) | 00:28:43, 0 min | 0 min | **68 min** | pulling, 23530, 23, 6, 413, 0, 2, 4, 1 | **Progressing but not ready**; head.json stale but ready/pool fresh. Different digest from the ready majority |
| g1-m05 | DESKTOP-6NSLEUO / windows | false | running, stage tools 4/5, digest ce0ca918 | 25 / 75 (stalled 10 min, restarted at once) | 00:28:43, 0 min | 6 min | 14 min | pulling, 23530, 25, 12, 528, 0, 2, 6, 16 | **Not ready**: tools stage restarts after stalls (exit 75). Different digest from the ready majority |
| g1-m06 | mac-mini-m4-04 / macos | false | running, stage workspace 5/5, digest 70ffaf34 | 1 / 1 (adopted, not restarted) | 00:27:46, 1 min | 0 min | 2 min | pulling, 18941, 13, 6, 182, 0, 0, 0, 3 | **Progressing, but the graph slice looks stuck**: last lines "Reading the project graph slice (885 s to 891 s): taken 4 of 591" repeat without advancing |
| g1-m07 | mac-mini-m4-02 / macos | true | ready, digest 70ffaf34 | 0 / null | 00:27:02, 2 min | 0 min | 7 min | pulling, 18941, 12, 5, 280, 2, 0, 3, 5 | Ready, progressing |
| g1-m08 | mac-mini-m4-05 / macos | true | ready, digest 70ffaf34 | 0 / null | 00:26:56, 2 min | 0 min | 7 min | pulling, 18941, 14, 8, 547, 8, 0, 3, 8 (reason: starting, 5 of 8 children run) | Ready, progressing |
| g1-m09 | (no status) | n/a | n/a | n/a | none | none | none | none | **Silent**: only `fleet-auth.json` (2026-11-14 02:13) in the folder, no status file ever since (about 6 days) |
| g1-m10 | Mac / macos | true | ready, digest 70ffaf34 | 0 / null | 00:22:36, 6 min | **22 min** | 3 min | pulling, 32730, 914, 847, 1212, 29, 22, 0, 5 (last error: owner pool child exit 137) | Ready, progressing; attest 22 min old (watch) |
| g1-server | ueli-mac / macos (server) | no ready.json | n/a | n/a | none | 0 min | 12 min | pulling, 23530, 72, 62, 648, 2, 4, 4, 7 (last error: fleet cache did not answer in time) | Progressing (server puller, no ready.json by design) |

Note on the pool wave: m01, m02, m06, m07, m08 pull wave `a9596a40`; m03, m04, m05, m10, g1-server pull wave `f9e3ed6c`. Two waves are active at once.

### Newest logs (last 5 meaningful lines; heartbeat lines "still running after N s" are dropped unless they are the last line)

- **g1-m01**: newest service-failure (20:45 UTC previous day, about 28 h old) ends with `nxBuildEngineExternalInputsV1` stack frames. Newest ready-log-workspace (10:19 local, about 14 h old) ends with `repo` `stage: complete`.
- **g1-m02**: service-failure 08:43 local (about 16 h old): `Slice worker exited with code 124`. ready-log-workspace ends with `Reading the project graph slice (250 s): owners done 591 of 591.`, `tasks: 0 stale, 3 up to date`, `[script] ... stage: complete`.
- **g1-m03**: service-failure 00:27 local (about 24 h old): `Build source authority keeps changing during inference`. ready-log-workspace ends with `[fleet] Published the nx-graph of state 30ed064446df ... (3 documents, 4 MB)` and `stage: complete`.
- **g1-m04**: ready-log-tools (00:28 local, fresh) ends with `javascript-installed-paths:65996 ... (shared, 38 s)`, `dependencies:bootstrap-current-ready` (exclusive), `javascript-installed-paths:30497` (shared, 5 s). No service-failure file.
- **g1-m05**: ready-log-tools (fresh) ends with `[repo] still working after 384 s ... 504 s; last phase started: source.served` (repeated). No service-failure file.
- **g1-m06**: ready-log-workspace (fresh) ends with `Reading the project graph slice (885 s, 887 s, 889 s, 891 s): taken 4 of 591.` and `[repo] still working after 1310 s; last phase started: door.script`. No service-failure file.
- **g1-m07**: service-failure 22:02 local (about 26 h old): `observeProjectInferenceV1` / `tagProjectFailureV1` stack. ready-log-workspace (about 20 h old) ends with `stage: complete`.
- **g1-m08**: service-failure 22:04 local (about 26 h old): same `observeProjectInferenceV1` stack as m07. ready-log-workspace (about 20 h old) ends with `stage: complete`.
- **g1-m09**: no logs.
- **g1-m10**: service-failure 11:04 local (about 37 h old): `RepoError: unserved build.graph.no-progress {"minutes":5,"done":17,"total":24,...}`. ready-log-workspace (about 18 h old) ends with `[repo] still working after 2480 s; last phase started: source.served`, `stage: complete`.
- **g1-server**: no ready-log, no service-failure. Only pool and attest files.

### Notes

- **Ready now (ready.json true, fresh):** g1-m02, g1-m03, g1-m07, g1-m08, g1-m10 (m10 attest is 22 min old).
- **Ready but silent (> 60 min):** g1-m01 (ready.json 77 min, attest 75 min, pool 75 min, head 80 min). Its pool still reports pulling, so the machine may be down or its daemon stopped.
- **Silent (no status ever since 2026-11-14):** g1-m09.
- **Progressing, not ready:** g1-m04 (linux, tools retrying after 48 attempts), g1-m05 (windows, tools stalls and restarts, 25 attempts), g1-m06 (macos, workspace stage, graph slice seems stuck at 4 of 591).
- **Digest mismatch:** ready majority digest `70ffaf34`. g1-m04 uses `304d5d98` and g1-m05 uses `ce0ca918`. Their tools-stage failures may come from a different build input, which should be checked before waiting for them.
- **Recurring pool errors:** "the record was not stored: an earlier request got no answer / the fleet cache did not answer in time" appears on m03, m04, m05, m07 and g1-server. This points at the fleet cache endpoint, not at individual machines.

## 2. Rust probe timings (handover step 4)

Searched every file under `/Users/Shared/semio-store/v1/origins/` (ready-log-*, ready-timings.json, `*timings*`, view.txt, service-failure, pool, slice-memory, segments) for `[phase]`, `[cargo-timings]` and `[units]`.

- **Result: 0 lines in all origins** for each of the three markers.
- `ready-timings.json` exists on g1-m01 only. It holds stage run records (`stage`, `seconds`, `outcome`, `peakMegabytes`, `attempts`), no phase or cargo rows. Example: g1-m01 workspace run of 14602 s with outcome `stage-timeout`, followed by repeated `stalled` and `stage-failed` runs. Not a per-phase breakdown, so nothing to sum.
- Reason: the R206 phase ledger (`result-R206.md` in the WORKERS folder) is only staged. Its step 2 (one real run) is blocked by an unrelated bug: `Unknown closure reference publication: inference/selections/...` raised in `retireBuild`, reported at 13:30 and 13:32 UTC on 2026-11-18. No worker runs the ledger yet.
- Conclusion: no phase timings exist yet. The next usable source is the first `ready-log-workspace.txt` written after the R206 code lands on a worker, read with `grep '^\[phase\]\|^\[cargo-timings\]\|^\[units\]'`.

## 3. Whole-workspace read on this Mac (pid 18668)

Source: `/Users/ueli/Documents/semio/.cache/f/graph-server/read.log` (30090 lines, last write 01:29 local = 00:29 UTC). `read-record.json`: runner 18668, read 18669, started 2026-11-18 about 18:01 UTC, beat 00:29 UTC. The read is still running, and was not touched.

### Last 30 lines, summarized

- Progress: `build:projects · Inferring projects: 4218 of 4785 projects · 12 running`. The count has been at 4218 for the whole visible tail. Owners line: `owners 4184 of 4785 done: 65 present, 122 fetched from the fleet, 3997 inferred here (27 running), 601 wanted from the fleet, 0 stale in the fleet`.
- Running owners: `semio-hub-vcs-component` (language:cargo) 965 s with 0 rows, parked, "waiting for a farm child to verify the stored record"; `semio-hub-shooting-component` 635 s with 0 rows, parked the same way; `semio-hub-sequence-component` 545 s with 0 rows. Others parked, waiting for a farm child or for memory: `semio-hub-reasoning-component` (428 s), `semio-hub-layout-component` (124 s), `semio-hub-cad-component` (123 s), `semio-hub-norm-component` (121 s).
- Memory waits: `waiting for memory headroom (6472 of 7563 MB reserved, 8 projects running)`. `semio-hub-forms-component` needs 754 MB and has waited 82 s (the rule is "after 60 s runs beside the others as soon as it fits, otherwise refused with exit 75"). The log does not show an actual refusal.
- Errors: none. Searching for `Error`, `unexpected failure` and `refused` finds only the boilerplate "otherwise the run is refused with exit 75" sentence and no real error lines.
- Elapsed: `[repository:command] still running after 23212 s` (about 6.4 h).

### Progress rate (from the `still running after N s` clock and the `Inferring projects` lines, 6532 samples)

| Elapsed (s) | Done of 4785 | Rate over the window |
|---|---|---|
| 19632 | 3768 | |
| 20232 | 3849 | 135 per 10 min (about 13.5/min) |
| 20832 | 3931 | 82 per 10 min (about 8/min) |
| 21432 | 4012 | 81 per 10 min (about 8/min) |
| 22032 | 4170 | 158 per 10 min (about 16/min) |
| 22632 | 4196 | 26 per 10 min (about 2.6/min) |
| 23232 | 4218 | 22 per 10 min (about 2.2/min) |

- Last hour average: 450 projects in 3600 s, about 7.5 per minute.
- Last 10 minutes: about 2.2 per minute, and 4218 has not moved for about 2.5 min at the end of the log.
- Remaining: 567 projects. At the last-10-minute rate that is about 4.3 h. At the last-hour average it is about 1.3 h.

## Findings

1. Only 5 of 11 origins are ready and fresh (m02, m03, m07, m08, m10). g1-m01 is ready but silent for 77 min, g1-m09 has been silent since 2026-11-14, and g1-m04 and g1-m05 are stuck in the tools stage on different digests (304d5d98, ce0ca918) than the ready majority (70ffaf34).
2. There are no `[phase]`, `[cargo-timings]` or `[units]` lines in any origin. R206 is staged only, and its step-2 real run is blocked by an `Unknown closure reference` bug in another package.
3. The whole-workspace read is alive, at 4218 of 4785 (88%), with no real errors. The rate fell from about 8 per minute to about 2 per minute in the last 10 minutes, with the longest owners parked on farm-child verification and memory headroom.
