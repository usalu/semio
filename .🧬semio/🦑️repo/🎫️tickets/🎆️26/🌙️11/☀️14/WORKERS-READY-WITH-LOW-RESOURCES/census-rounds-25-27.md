# Census of rounds 25 to 27 (read-only audit)

Scope: `workers/g1-m01` ... `g1-m10` as on disk at about 10:24Z on 2026-11-15. Files modified since 2026-11-15 08:00Z. Times are UTC. "not in the files" means the data is not there. Quoted lines are verbatim unless marked (cut). `g1-m05` has no folder (Windows machine, only seen in the sidecar logs). Method: `find -newermt`, grep and tail on the files named; no whole-file reads, no writes outside this file.

Naming note: several worker files for g1-m04 and g1-m07 carry the round-24 label although the worker's report calls them round 27. The table follows the file names and says so.

---

## Part A

### A1. Per machine: last report, tries, exit codes, end points

| Machine | Last report (state.json / last write) | Tries in rounds 25 to 27 (exit code `native_rc` each) | Where each try ended |
|---|---|---|---|
| g1-m01 (Mac, 16 GB) | state 10:18:18Z (try 4); try 5 started 10:21:18Z, still running at 10:24 | r27 t1 75, t2 75, t3 75, t4 75; t5 running | all four in `graph` plan stage `inference`: `graph-plan-too-heavy` at 09:48:43, 09:56:24, 10:07:35, 10:17:50 (try 3 reason `pressure`, the others `ceiling`). Round-17-labelled tries 11 (rc 75, ended 08:04:45) and 12 (rc 75, ended 08:39:31) also fall in the window, but not under the round 25-27 label |
| g1-m02 (Mac, 16 GB) | state 10:18:58Z (r27 t3); r27 t4 started 10:21:58Z, running at 10:24 | r26 t1 75, t2 75, t3 75, t4 75 (08:31 to 08:53); r27 t1 75, t2 **1**, t3 75 (08:53 to 10:18) | r26: plan stage inference, `fleet_refusal=True` each (ends 08:33:58, 08:39:35, 08:44:43, 08:53:19). r27 t1 ends 08:56:38 in plan stage inference. r27 t2 ends 10:09:44 with `The resident service met an unexpected failure (reference fb106d37)` after about 70 min in `waiting for memory headroom`. r27 t3 ends 10:18:58 in plan stage inference (`graph-plan-too-heavy` at 10:18:25, 10092 MB, reason `ceiling`) |
| g1-m03 (Mac, 16 GB) | state 07:59:56Z (round 14 try 12, rc 1); nothing written since 07:59:57 | none in the window (round 14 only) | try 12 (ended 07:59:57, before the window) ended with `Internal error: Relative filesystem fstatat failed (EPERM)` then `repository:command failed with exit code 1` |
| g1-m04 (Linux) | state 10:04:25Z (round-24-labelled try 12, rc 1); round-27 step 0 started 10:05:36Z, no end line in the folder | round-24-labelled tries 2 to 12 in the window (try 1 ended 08:32 and is outside): all rc 1. No try labelled 25 to 27 | every try 2 to 12 ends in the service: `The plan cannot continue: the service holds N MB of its N MB memory limit (safety margin 256 MB)`, N from 5340 to 6384 |
| g1-m06 (Mac) | state 04:43:49Z (round 18 try 9, rc 75) | none | nothing written since 04:43:50Z |
| g1-m07 (Mac) | state 09:24:29Z (round 27 try 4, rc 75, failed `workspace:setup`) | 4 tries (files named round-24, report says round 27): t1 75 (ended 08:42:15), t2 75 (08:59:37), t3 75 (09:07:18), t4 75 (09:14:29) | all four in plan stage inference: `The project graph could not be read on this machine yet: ... grew to 9859 to 10003 MB, more than the 6553 MB this machine gives a plan` (worker report, cut) |
| g1-m08 (Mac) | state 04:13:44Z (round 18, rc 75); `next.md` 08:47:48Z says "round: 27" | none (no try since 04:13) | none |
| g1-m10 (Mac, 16 GB) | state 09:25:32Z (round 27 try 2, rc 7) | r27 t1 **75** (08:57:23 to 09:05:19); r27 t2 **7** (09:08:59 to 09:25:32) | t1: plan stage inference, `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 10076 MB, more than the 6553 MB this machine gives a plan` (verbatim from `round-27-try-1.tail.txt`, cut). t2: rc 7 is the worker's own observer stop (`memory-pressure-level-4-two-samples`, a rule that round 23 retired); not a repo result |

Tries with exit 1 on g1-m02 and g1-m04 are listed above; no try in the window ended with 0.

### A2. Expected lines

| # | Expected line | Result | Verbatim example |
|---|---|---|---|
| 1 | `fstatat failed (EPERM)` or `where: root=` after 08:17Z (should be gone) | **Absent from every log** modified after 08:17Z. Matches only in scripts that hold the literal pattern (`g1-m02/run-round-25.py`, `run-round-26.py`, `run-round-27.py`, `g1-m04/run-tries-27.sh`) and in reports that quote history (`g1-m01/round-17.md`, `g1-m04/round-24.md`). The last real occurrence is g1-m03 try 12 at 07:59:57 (before the cut) | none in logs |
| 2 | Linux g1-m04 first plan line `machine 15848 MB … per project … infer` | **Seen in tries 2 to 12** (08:39 onwards). Try 1 (07:21 to 08:32, outside the window) printed `0 MB per project` in `round-24.service.txt` | `graph: memory readings: machine 15848 MB (physical 15848 MB, cgroup limit none), available 8227 MB` then `graph: 703 stale projects, 0 taken from the fleet, 7 too heavy for this machine (workspace (about 12212 MB), ...; it can give about 3298 MB per project), it can infer 192 of the others` (try 2, 08:39, cut after `others`) |
| 3 | `access-proof version=3` | **Seen** on g1-m01 r27 t1, g1-m10 r27 t1, g1-m02 r26 t1 and t4, g1-m04 tries 2 to 12. **Absent** from g1-m01 r27 t2, t3, t4, g1-m10 r27 t2, g1-m02 r26 t2, t3, and g1-m02 r27 t1, t2, t3. The sampler's `ACCESS LINES TRY n: 0` on g1-m02 r26/r27 is the same absence | `2026-11-15T09:41:45.358Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=91892 ppid=91891 responsible=46526` (g1-m01 r27 t1); `2026-11-15T08:57:29.486Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=95517 ppid=95437 responsible=39644` (g1-m10 r27 t1, from its report) |
| 4 | `repo tools setup all`: exit code, wall time, per-scope `restored` / `installed` lines | **g1-m02, round 25 (08:10:29 to 08:31:21): exit 0, 1252 s.** Only one matching line, the header `Funding the dependency store: the dependencies of every scope are installed into it now...`. No per-scope line. **g1-m07, attempt 3 (08:03:44 to 08:31:30): exit 1, 1666 s**, same error as attempt 2: `The dependency store returned no complete retirement plan (... /value/roles/names must contain at most 16 item(s))`. **g1-m10, attempts 4 (07:55 to 08:51, 3369 s) and 5 (08:51 to 08:57, 338 s): exit 1**, `Resource lease timed out: native-storage`. **g1-m04, round 27 step 0: started 10:05:36Z, no result in the folder.** g1-m01's step 0 (exit 0, 1895 s) is from before 08:00 (`round-17.notes.md`) | g1-m02: `Funding the dependency store: the dependencies of every scope are installed into it now. A first funding takes many minutes on a small machine; ...` |
| 5 | `graph: waiting for the fleet: x of N projects arrived` (largest x and final outcome) | **Largest x seen since 08:00Z: 0** (every line, all machines). Lines: g1-m04 `0 of 703` (tries 2 to 12); g1-m10 `0 of 1682` and `0 of 1686` (r27 t1 and t2); g1-m07 `0 of 676`. Final outcomes: g1-m04 ends in the memory-limit refusal (above); g1-m10 t1 ends in plan stage inference; g1-m07 never got past its plan | `graph: waiting for the fleet: 0 of 703 projects arrived; the server last progressed 353 s ago, estimated time left: about 1 min` (g1-m04 try 1 file, outside the window; the same wording recurs in later tries) |
| 6 | `ready: true` or `native-bootstrap.done` | **None.** Every machine's `state.json` has `"ready":false`. The only matches are scripts that test for the marker and reports that say it is absent (g1-m07 report, g1-m10 report) | none |
| 7 | Every distinct error line that ended a try (group, count, machines) | See A3 | see A3 |

### A3. Distinct error lines that ended tries (since 08:00Z, rounds 25 to 27 labels)

| Error (class) | Count | Machines |
|---|---|---|
| `graph-plan-too-heavy` at plan stage inference (`ceiling` or `pressure`), then rc 75 with `could not be read on this machine yet` | 15 (g1-m01 x4, g1-m02 r26 x4 and r27 x2, g1-m07 x4, g1-m10 x1) | g1-m01, g1-m02, g1-m07, g1-m10 |
| `The plan cannot continue: the service holds N MB of its N MB memory limit (safety margin 256 MB)` (rc 1) | 11 (g1-m04 tries 2 to 12) | g1-m04 |
| `The resident service met an unexpected failure (reference fb106d37); try again` (rc 1) | 1 | g1-m02 r27 t2 |
| `Internal error: Relative filesystem fstatat failed (EPERM)` (rc 1) | 0 in the window; 1 at 07:59:57 | g1-m03 (before the window) |
| `Resource lease timed out: native-storage` (setup, rc 1) | 2 | g1-m10 (attempts 4 and 5) |
| `dependency store returned no complete retirement plan (... at most 16 item(s))` (setup, rc 1) | 1 | g1-m07 attempt 3 |

Note: the g1-m02 r26 tries ended with `fleet_refusal=True`; the sampler's `PLAN TOO HEAVY` line is the end cause, so they are counted under the first row. g1-m07 counts are the four tries in its report.

### A4. Progress detail (plan numbers, for the coordinator)

| Machine, try | Plan stage inference settled from / to | Too-heavy line (megabytes, limit, available, reason) |
|---|---|---|
| g1-m01 r27 t1 | 8071 to 10051 MB | 10051, 6553, 2729, ceiling (09:48:43) |
| g1-m01 r27 t2 | 7492 to 9563 MB | 10202, 6553, 2852, ceiling (09:56:24) |
| g1-m01 r27 t3 | 6659 to 8675 MB | 8675, 6553, 1774, pressure (10:07:35) |
| g1-m01 r27 t4 | not settled | 9965, 6553, 5701, ceiling (10:17:50) |
| g1-m02 r26 t1 | 7936 to 7561 MB | not found in the grep (end 08:33:58) |
| g1-m02 r26 t2 | 7013 to 9316 MB | not found in the grep (end 08:39:35) |
| g1-m02 r26 t3, t4 | no settle line found | not found in the grep (ends 08:44:43, 08:53:19) |
| g1-m02 r27 t1 | no settle line found | not found in the grep (end 08:56:38) |
| g1-m02 r27 t3 | 7377 to 8354 MB | 10092, 6553, 4352, ceiling (10:18:25) |
| g1-m10 r27 t1 | not in files | 10076 MB, 6553, 3452 MB available |

Grant lines since 08:00Z: g1-m02 r26 t1 to t4 and r27 t1 to t3, g1-m10 r27 t1, all show `it can give about 3512 MB per project), it can infer 192 of the others (48 per wave)`. The too-heavy counts were 12, 14, 16, 5 (r26) and 8, 10, 12 (r27 t1 to t3), 17 (r27 t4, running). Mac memory readings: `machine 16384 MB (physical 16384 MB, cgroup limit none)`.

g1-m01 r27 shows no grant line. Its files say `grew by 3441 MB, more than the 1756 MB this machine gives one project`, so it works from the per-project limit, not from the 3512 MB grant. Ask the coordinator which grant applies to g1-m01.

---

## Part B

### B1. Claim-held lines in `build-agent.log`

Total lines `the claim for .* is held by` since 2026-11-15T08:00Z: **2**. Since 05:00Z: 7.

| Time (Z) | Requester | Holder | Text |
|---|---|---|---|
| 05:32:44 | g1-m02 | another machine (unnamed) | `held by another machine; this machine leaves the request to it` |
| 05:43:03 | g1-m04 | another machine (unnamed) | same |
| 05:43:13 | g1-m01 | another machine (unnamed) | same |
| 05:51:21 | g1-m02 | another machine (unnamed) | same |
| 06:52:41 | g1-m10 | g1-m05 | `held by g1-m05` |
| 09:37:21 | g1-m10 | g1-m05 | `held by g1-m05` |
| 10:01:28 | g1-m02 | g1-m05 | `held by g1-m05` |

Per holder since 08:00Z: g1-m05 2. Per requester since 08:00Z: g1-m10 1, g1-m02 1.

### B2. Did the holder deliver? (`g1-m05` lines)

- `build-agent.log` contains **no line written by or about g1-m05 except the three held lines above**.
- `build-scheduler.log` and `build-cache.log` contain **no line naming g1-m05, g1-m10 or g1-m02 after 06:52Z**. The scheduler and cache logs do not name requesters at all.
- **g1-m10, claim held at 06:52:41:** this machine went on serving g1-m10 (`serving 1910 stale projects for g1-m10` at 06:59:23, `serving 1653` at 08:02:25, `serving 1682` at 08:29:23). Then `the claim for g1-m10 had lapsed; this machine holds it again` at 08:59:36. No delivery from g1-m05 is visible; the claim lapsed.
- **g1-m10, claim held at 09:37:21:** no later line for g1-m10 in any log. Neither a delivery nor a lapse is recorded. **Undetermined.**
- **g1-m02, claim held at 10:01:28:** `serving 52 stale projects for g1-m02` at 10:23:55. No lapse line. The answer came from this machine about 22 min later. Whether g1-m05 released, expired or delivered is **not determinable** from the logs.

### B3. Counts since 08:00Z in `build-agent.log`

| Phrase | Count | Notes |
|---|---|---|
| `graph-server: continuing after a restart` | 5 | 08:35:35, 09:00:41, 09:22:57, 09:41:02, 10:17:01; each with "0 projects already reported fresh" |
| `made no progress` | 28 | first 08:04:35.719Z (`the read (pid 86463) made no progress for 120 s`), last 10:25:39.120Z (`the read (pid 29617) ...`). Not in scheduler or cache logs |
| `times its grant` | 3 | 09:22:16 (`the read holds 25807 MB, 1.8 times its grant of 14336 MB`), 09:40:01 (25838 MB, 1.8 times), 10:14:19 (33992 MB, 2.4 times) |
| `are fresh` | 8 | all are `joined the held request, 0 are fresh already` lines: 08:37:56 (g1-m02, 49), 09:00:41 (g1-m10 1620, g1-m07 605), 09:05:19 (g1-m07 703), 09:07:04 (g1-m04 703), 09:07:09 (g1-m10 1686), 09:12:44 (g1-m07 703), 09:22:57 (g1-m04 703). **No complete answer (N of N) in this window.** The only non-zero `are fresh` lines are from 2026-11-14 17:27 to 17:28, for example `262 of 703 projects of g1-m03 are fresh` |

---

## Facts the coordinator must act on

- No try in rounds 25 to 27 reached setup tasks or `ready`. Tries ended in the plan stage (g1-m01, g1-m02, g1-m07, g1-m10), in the service memory limit (g1-m04), or in one unexpected service failure (g1-m02 r27 t2). `native-bootstrap.done` exists nowhere.
- The fleet delivered nothing since 08:00Z: the largest `x` in `waiting for the fleet: x of N` is 0 on every machine.
- The Linux fix works in the grant (`3298 MB per project`), but every g1-m04 try (2 to 12) still ends on `The plan cannot continue: ... memory limit`.
- g1-m02 round 27 try 2 ended with `The resident service met an unexpected failure (reference fb106d37)`. The reference is not in the worker folder. The host log named in that line is needed.
- g1-m07 attempt 3 of `repo tools setup all` still fails with the 16-role retirement error, though round 25 says it is fixed. g1-m10 has three attempts with `Resource lease timed out` and no restored/installed line. g1-m04 round 27 step 0 has no recorded result.
- The `access-proof version=3` line is missing from most tries (g1-m02 r27 all, g1-m01 r27 t2 to t4). Its absence means the try was not logged with its access state, so its absence cannot prove the access route worked.
- g1-m03, g1-m06 and g1-m08 have had no try since before the window (last writes 07:59, 04:43 and 04:13). g1-m08 has only a pointer file at 08:47.
- g1-m05 holds claims (2 since 08:00Z, for g1-m10 and g1-m02). No line from g1-m05 exists in any sidecar log, so whether it delivered or let its claims expire cannot be determined from the files.
