# Census of rounds 28 and 29 (read-only audit)

Scope: `workers/g1-m01` ... `g1-m10` as on disk at about 16:28Z on 2026-11-15. Files read: every `round-28*` and `round-29*` file in each worker folder, plus `state.json` and `active-try.json`. Times are UTC. Times in `ls` output of the Macs are local (CET, UTC+1); those were converted. "not in the files" means the data is not there. Quoted lines are verbatim unless marked (cut). Method: `ls`, `stat`, grep and tail; no writes outside this file.

Round 29 says (ROUND.md) that a try which stood still in the 14:50 to 15:01Z server outage is not a result. Round 28 tries (15:22Z and before) are reported as tries. Machines named in round 29 but with no round-29 files: g1-m01, g1-m03, g1-m06, g1-m08 (and g1-m10 wrote nothing after 15:37Z).

---

## Part A. Per machine

Peak figures: `peak_rss_kib` and `run_peak_rss_kib` are the monitor fields as written (KiB). `slice_peak_mb` is the slice worker peak from the `.tries.txt` files. Mac sampler peaks are in GB where the report gives them.

| Machine | Last round reported (state.json) | Tries (rounds 28/29 files) | Time of last report (UTC) | Setup exited 0? | Verdict / refusal of the last try (verbatim, short) | Peak memory figures | `ready: true` anywhere? |
|---|---|---|---|---|---|---|---|
| g1-m01 (Mac) | round 27, try 12, native_rc 75 | 0 files for rounds 28/29 (state says try 12 of round 27) | 2026-11-15T11:27:00Z (state.json 11:26:59Z) | not in the files | not in the files (state only: native_rc 75, failed []) | none in the files | no |
| g1-m02 (Mac) | round 28, native_rc 75 (try 8) | 9: tries 1 to 8 ended (all native_rc 75); try 9 running (started 15:20:44Z, last sample 16:27:51Z) | state 15:15:44Z; newest file 16:26:51Z | no: setup not reached (native_rc 75, failed []) | try 8 end line: `TRY 8 END native_rc=75 ... fleet_refusal=True`; last plan line before it: `graph-plan-too-heavy {"stage":"inference",...,"megabytes":10103,"limit":6553,"available":2883,"reserve":2048,"reason":"ceiling"` (15:15:07Z) | try 1 `peak_rss_kib=9164768` (run peak 145328); try 8 `peak_rss_kib=4789408` (run peak 139968); try 7 `peak_rss_kib=4007568` | no |
| g1-m03 (Mac) | round 14, try 12, native_rc 1 | 0 for rounds 28/29 | 2026-11-15T07:59:56Z | not in the files | not in the files for rounds 28/29 (state only: native_rc 1) | none in the files | no |
| g1-m04 (Linux) | round 29, try 4, native_rc 1, failed `workspace:setup` | round 28: 14 tries, all ended; round 29: tries 1 to 4 ended, try 5 running (pid 990145, started 16:12:46Z) | state 16:07:46Z; newest file 16:26:51Z | no: round-28.md says step 0 was not run again ("exit 1 twice, see round-27.md", not in these files); state failed `workspace:setup` | try 4 (round 29), verbatim: `The plan cannot continue: the service holds 9764 MB of its 9764 MB memory limit (safety margin 256 MB), so it was cancelled to keep the service up; try again later or ask for a smaller selection.` (native_rc 1) | round 28 slice peaks 8217 to 9624 MB (try 3 9624; try 1 9035; try 12 8506); round 29 slice peaks try 1 9243, try 2 9064, try 3 8473, try 4 7404 MB; `peak_rss_kib` round 29 try 1 9482796, try 4 6168928 | no |
| g1-m06 (Mac) | round 18, native_rc 130, tries 11 (state) | 0 for rounds 28/29 | 2026-11-15T11:58:27Z | not in the files | not in the files (state only) | none in the files | no |
| g1-m07 (Mac) | round 29, try 1, native_rc 75, failed `workspace:setup` (state at 16:11:17Z) | round 28: 4 tries (11:36Z to 11:55Z), all native_rc 75; round 29: try 1 (15:34Z to 16:11Z) native_rc 75, try 2 (16:16:17Z to 16:26:53Z) ended, verdict line not in the files | state 16:11:17Z; monitor end of try 2 at 16:26:53Z | yes for round 28: round-28.md reports step 0 attempt 4 `exit_code=0`, 1162 s (11:17:31Z to 11:36:53Z). Not run again. Round 29 setup not reported | round 29 try 1, from round-29.tries.txt: `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to N MB, more than the N MB this machine gives a plan, and only N MB of memory were available` (numbers not printed in the tries file). Round 28 try 4, verbatim from round-28.md: `...grew to 9966 MB, more than the 6553 MB this machine gives a plan, and only 4313 MB of memory were available (at least 2048 MB are kept in reserve)...` | run peak resident round 28: 7.49 GB (try 1), 3.37, 3.32, 3.09 GB; slice worker peaks round 28: 7706, 5094, 5425, 7648 MB; monitor `peak_rss_kib` round 28 try 1 7858976; round 29 try 1 3639728, try 2 3858320 | no |
| g1-m08 (Mac) | round 18, native_rc 75, failed `workspace:setup` (state) | 0 for rounds 28/29 | state 2026-11-15T04:13:44Z (file 04:16:32Z); `next.md` 08:47:48Z says `round: 27` | not in the files | not in the files (state only) | none in the files | no |
| g1-m10 (Mac) | round 27, native_rc 75, failed `native-bootstrap setup` (state at 15:36:57Z) | 0 round-28/29 tries; `active-try.json` names round 27 try 3 (pid 57179, started 15:25:16Z) | state 15:36:57Z; newest file 15:37:41Z | no: state native_rc 75, failed `native-bootstrap setup` | not in the files for rounds 28/29 | none in the files | no |

Other findings in the files for these machines:
- g1-m02, round 28: `The resident service met an unexpected failure` does not appear in any round-28 file. `access-proof version=3` appears once in tries 1, 3, 5 and 8 (route=launchd in the samples file).
- g1-m04: `access-proof version=3` appears once per try in the files that carry it (round 28 try 2, round 29 try 4).
- g1-m10: `round-29.agreement-wait.txt` reads `15:37:32 m10=32b23f76b8e7 inc=0 srv=32b23f76b8e7 inc=0` then `AGREE`. The replica agrees with the server on code digest 32b23f76…

### A1. g1-m04 memory limit (asked for explicitly)

- The Linux memory limit does NOT read about 9508 MB in any round-28/29 file. The refusal line reads `the service holds 9764 MB of its 9764 MB memory limit` (round 28 try 8, native_rc 1; round 29 try 4, native_rc 1).
- The readings line reads `graph: memory readings: machine 15848 MB (physical 15848 MB, cgroup limit none), available 8701 MB` (round 28) and `available 8481 MB` / `8535 MB` (round 29). It prints no limit.
- The only 9508 figure is `## 2026-11-15T12:43:54Z MemAvailable=9508844kB SwapFree=14588148kB` in `round-28.samples.txt`. That is free memory, not a limit.
- Inferred, not in the files: 9764 MB is about 61.6 % of 15848 MB; the round-28 instruction expected 60 % (9508 MB). The file does not show why the value differs.

### A2. g1-m10 sqlite lease diagnostics (`round-28.sqlite-diagnostics.txt`, taken 15:37Z)

- File: `/Users/ueli/Documents/semio/.cache/b/l/m/data/20f6e20c7921c319df990fd41f8a8cfeea31b906359fe0358c1e8000cc6bf173.sqlite`. Local listing: `-rw-r--r--@ ... 8192 Nov 15 16:25` (local CET, so 15:25Z, inferred from the time zone; matches the start of round-27 try 3 at 15:25:16Z).
- `lsof` on the file printed nothing: no process had it open at 15:37Z.
- The full `.dump` is one table and one row: `CREATE TABLE semio_resource_lease (id INTEGER PRIMARY KEY CHECK (id = 1), version INTEGER NOT NULL CHECK (version = 1), resource TEXT NOT NULL);` and `INSERT INTO semio_resource_lease VALUES(1,1,'native-storage:[16777233,1112611,38653698048,16777255]');`.
- Who holds the `native-storage` lease: the files do NOT say. The table has no holder, pid or time column; the row is only a resource key. Inferred: the lease holder cannot be read from this database. The `ps` list in the same file shows `repo mcp` processes (pids 39171 and 39689, about 11.5 h elapsed) and several `bun` processes under the worktree (pids 50959 to 51086, 57741, 57772). None is linked to the lease by the files. Not verified.
- Nothing was deleted (the protocol forbids it).

---

## Part B. Silent machines

Silent = no file written after 15:01Z on 2026-11-15.

- g1-m01: silent. Last write 11:27:00Z (state.json).
- g1-m03: silent. Last write 07:59:57Z.
- g1-m06: silent. Last write 11:58:27Z.
- g1-m08: silent. Last write 04:16:32Z (`next.md` at 08:47:48Z still says `round: 27`).
- Not silent by the rule, but idle: g1-m10 (last write 15:37:41Z, about 50 minutes before this census).

Machines that wrote after 15:01Z: g1-m02 (9 files), g1-m04 (49 files), g1-m07 (12 files), g1-m10 (16 files; newest 15:37:41Z).

---

## Part C. Distinct failure reasons (rounds 28 and 29 files)

| # | Reason (class) | Count | Machines | One exact line |
|---|---|---|---|---|
| 1 | Agent's own sampler stop: `stop=process of the run above 8 GB resident (...)`, native_rc 7 | 14 | g1-m04 (round 28 tries 1, 2, 4, 5, 6, 7, 9, 10, 11, 13, 14 = 11; round 29 tries 1, 2, 3 = 3) | `try 1: start 2026-11-15T12:40:53Z ... native_rc=7 ... stop=process of the run above 8 GB resident (9272008 KiB)` (round-28.tries.txt) |
| 2 | Plan refusal, exit 75: `The project graph could not be read on this machine yet: ...` (or the same with `fleet_refusal=True`) | 15 | g1-m02 round 28 (tries 1 to 8 = 8), g1-m07 round 28 (tries 1 to 4 = 4), g1-m07 round 29 (try 1 = 1), g1-m04 round 28 (tries 3 and 12 = 2; try 3 printed no rc line) | `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 9602 MB, more than the 6339 MB this machine gives a plan, and only 2676 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could.` (g1-m04 round 28 try 12, round-28.md) |
| 3 | Service memory limit: `The plan cannot continue: ...`, native_rc 1 | 2 | g1-m04 (round 28 try 8; round 29 try 4) | `The plan cannot continue: the service holds 9764 MB of its 9764 MB memory limit (safety margin 256 MB), so it was cancelled to keep the service up; try again later or ask for a smaller selection.` |
| 4 | No end line yet (still running, or ended with no verdict in the files) | 3 | g1-m02 round 28 try 9 (running); g1-m04 round 29 try 5 (running); g1-m07 round 29 try 2 (ended 16:26:53Z, no verdict in the files) | `[repository:command] still running after 1221 s` (g1-m02 round 28 try 8 service log, cut context; try 9 has no service file yet) |
| 5 | `unexpected failure`, `fstatat ... EPERM`, setup `Resource lease timed out` | 0 in rounds 28/29 files | none | not in the files |

Setup state (from state.json only): `workspace:setup` failed on g1-m04 (round 29, rc 1) and g1-m07 (round 29, rc 75). `native-bootstrap setup` failed on g1-m10 (round 27 state, rc 75).

Grant lines: g1-m02 (Mac) prints `it can give about 3512 MB per project` (18 times). g1-m04 (Linux) prints `it can give about 3298 MB per project` (round 28 and round 29) and also `1024 MB this machine gives one project` (round 29, 3 to 7 times per try). The 1024 MB line is new in these files.

Largest `waiting for the fleet` value (x): 0 (g1-m02 `0 of 78`, g1-m04 `0 of 703`). g1-m07 round 29 sampler line at 15:52:35Z: `graph: 5 projects taken from the fleet with 0 closure entries, 1 of them did not hold here and were inferred again, 0 not shared, 45 inferred here` (and again at 16:06:06Z with 95 inferred here). This is the only non-zero fleet take in the files.

`ready: true`: none in any round-28/29 or state.json file. `native-bootstrap.done`: not present (g1-m07 round-28.md: `Ready: **no** (.cache/f/native-bootstrap.done absent)`).

---

## Part D. Compared with the previous census (rounds 25 to 27)

1. g1-m02 moved to round 28 (9 tries, first started 10:41Z). The round-27 `unexpected failure` did not recur. All 8 finished tries ended with plan refusals (exit 75).
2. g1-m04 ran 14 round-28 tries and 4 round-29 tries. The old `plan cannot continue` refusal is now in 2 tries only. The rest end on the agent's own 8 GB stop (native_rc 7).
3. g1-m07 step 0 is reported as exit 0 (round-28.md, 11:17:31Z to 11:36:53Z, 1162 s). The previous census had attempt 3 at exit 1; that was before this fix. Also new: a zero-byte `.git/index.lock` in the worktree (11:17:11Z to 15:30Z) stopped the replica for about 4 hours. round-28.md says g1-m02 "shows the same numbers" and may have the same stale lock (not verified here).
4. g1-m10 now shows state 15:36:57Z with native_rc 75 (previously rc 7 in round 27). Round-29 agreement is reached (AGREE at 15:37:32Z). The sqlite diagnostics were added (A2).
5. g1-m01 has been silent since 11:27Z (previous census: try 5 running at 10:24Z). State shows 12 tries under round 27.
6. g1-m06 state changed to `round 18, native_rc 130, tries 11` at 11:58:27Z (previous census: 04:43:49Z). rc 130 is probably an interrupt (inferred, not in the files).
7. g1-m08 unchanged: state 04:13:44Z; `next.md` still points to round 27. It has not read ROUND.md (round 29).
8. g1-m03 unchanged (07:59:56Z).
9. Linux: a per-project inference line `1024 MB this machine gives one project` appears alongside the 3298 MB grant (new in round 29).

---

## Inferred (not stated by the files)

- g1-m04: 9764 MB limit differs from the 60 %-of-RAM value (9508 MB) expected by round 28; the cause is not in the files.
- g1-m10: the lease holder is not identified by the diagnostics; the process list is only a candidate list.
- g1-m10 sqlite file modified 15:25Z (local 16:25 CET converted).
- g1-m06 rc 130 is an interrupt (exit-code convention, not stated).
