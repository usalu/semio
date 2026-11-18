# Audit: Real Commands Last 24 Hours

Read-only audit. No source files, builds, `repo` commands or network calls were used. Only this file was written.

Window: 2026-11-17 07:32 CET (06:32 UTC) to 2026-11-18 07:32 CET (06:32 UTC). Local machine is `g1-server` (see `serve-state.json` owner). Times below are CET unless marked Z (UTC). Durations marked "at least" come from the longest "still running / still working after N s" line, so the true start is earlier or equal.

Important caveat: no log records who started a command. The initiator column is "not logged" unless the file names an agent or person. Commands are identified by the `repo` top-level command and the `repository:command` owner, not by full argument lines, except where a holder line names the command (g1-m05).

## 1. Distinct real commands

### 1a. Commands started by a person or agent (or not attributable to the build system)

| # | Machine | Start (CET) | End / state | Duration | Outcome | Source |
|---|---|---|---|---|---|---|
| 1 | g1-server | about 15:06 | ended 16:58:45 | at least 1 h 53 min (6773 s) | cancelled: "Cancelled (SIGINT); leases released." | `.cache/f/graph-server/read.1794931125984.log` |
| 2 | g1-server | about 17:02 | ended 18:16:08 | at least 74 min (4433 s) | failed, exit 1 (see 2a) | `read.1794935768501.log` |
| 3 | g1-server | about 18:26 | ended 18:27:52 | about 90 s | failed, exit 1 (see 2a) | `read.1794936472671.log` |
| 4 | g1-server | about 18:38 | ended 23:38:46 | at least 5 h (18000 s) | cancelled: "SIGTERM received: the running work is cancelled and its leases are released" | `read.1794955126322.log` |
| 5 | g1-server | about 23:49 | ended 00:15:55 (18 Nov) | at least 27 min (1611 s) | failed, exit 1 (see 2a) | `read.1794957355419.log` |
| 6 | g1-server | 00:26 (startedMs 1794957962397) | still running at 07:32 | at least 7 h 08 min (25664 s) | no end recorded; `read-record.json` status null | `read.log`, `read-record.json` |
| 7 | g1-server | about 05:41 (host pid 18219 started 05:42:24) | still running at 07:32 | at least 1 h 51 min (6661 s) | no end recorded; last phase `source.served` | `semio-repo/501/host-80f5fdab18bf.log` |
| 8 | g1-m10 | about 06:27 | completed 07:08:54 | at least 41 min (2480 s) | succeeded: "command repo stage complete" | `origins/g1-m10/ready-log-workspace.txt` |
| 9 | g1-m05 (Windows) | holder since 04:55 (holds dependency store since 03:55:14Z) | no end in any file | at least 1 h 17 min at last sighting | unknown | `origins/g1-m05/ready-log-tools-stalled.txt` |

Row 9 detail: `Process 36528 (00945ece... run @semio-tech/fem-3d-rs:check --output=stream --parallel=8) holds the dependency store since 2026-11-18T03:55:14.747Z; waiting for it (1074 s of at most 21600 s)`. This is the only foreign command text found in the window. The waiter was the g1-m05 tools probe, not a person.

Note on row 6 and row 7: row 6 is the refresh read (`read-record.json` reason "refresh"), so it is background work. Row 7 is named only as `repo`; initiator not logged.

### 1b. Build-system background work (not a person or agent)

- Refresh read, row 6 above.
- Pool children and owner farm: `owners n=50` lines, `fleet owner pool ...: 4774 owners published` (repeated).
- Readiness probes per origin: `ready-log-workspace*.txt`, `ready-log-tools*.txt`, `service-failure*.txt`. Summarized in section 4 and 5.

## 2. Long or failed commands: phase and exact text

### 2a. Failed graph reads (rows 2, 3, 5)

Exact failure text (identical in all three):
- `A heavy derived question was stopped: its child process wrote nothing for 60 s (stuck on a lock, a dead peer or a blocked loop): -`
- `repository:command failed with exit code 1`

Phase at failure: `last phase started: source.served` (row 2 with build:source-admission, 22,895 repository:command lines in its log).

### 2b. Cancelled reads (rows 1, 4)

- Row 1: `Process 9513 received SIGTERM from owned execution repository:command (killing process 6881): cancelled`. Phase: build:discover (1541 lines) and build:playground-components (772 lines).
- Row 4 (5 h): `SIGTERM received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.` followed by `Cancelled (SIGINT); leases released.` Phase: build:projects. The log records 1502 `waiting for a farm child to look at the owner: assembly authority, stored record and closure references (the parent computes nothing)`, 1141 `waiting for a farm child to infer the owner, or for one to start`, 573 `waiting for memory headroom`, 63 `waiting for the memory governor to let the owner back into the process`. No cause for the cancel is logged.

### 2c. Running reads (rows 6, 7)

Row 6 (refresh, still running): last phase source.served; `build:projects` at `Inferring projects: 2603 of 4775 projects · 8 running, longest 225 s: semio-s-artifact-norm-vdi3805-mutations ... waiting for memory headroom (2036 of 7453 MB reserved)`. Owner `semio-s-artifact-bim-model-mut-any-N` is repeatedly `parked: waiting for a farm child to infer the owner ... (the host capacity governor holds a child while the machine lacks free memory) [no children]`.

Row 6 also shows owners set aside: at least 187 lines of `[owners] owner ... is set aside, the read goes on and ends naming it: farm children could not infer ... in 3 tries (The TypeScript compiler of the tooling at /Users/ueli/Documents/semio/.cache/b/d/m/j/d22a22b6385877c52eb2/node_modules/typescript (version 5.9.3, wanted 5.9.3) lacks the API the tooling uses: createPr)`. Example owner: `semio-s-artifact-bim-model-mut-any-138`.

Row 7 (host repo): last phase source.served; no end line.

Row 9 (g1-m05 holder): probe waited 1074 s on the holder; the probe stage later failed (see section 4).

### 2d. Owner wall times in the window

From footprint `28979.log` (2026-11-17 20:29 to 20:34 Z, i.e. 21:29 to 21:34 CET):
- `@semio-tech/stdio-mp3` wall=9449769 ms, queue=385213 ms, other=8999562 ms.
- `@semio-tech/fixture-generator-stdio-bcf-2-1-any` wall=9378963 ms, queue=5337332 ms.
- `@semio-tech/norm-js` wall=9317587 ms, refs=8810876 ms.

## 3. Whole-workspace reads (4775 projects)

- Whole-workspace project inference ran in 2 graph sessions: row 4 (cancelled after 5 h) and row 6 (still running at 07:32, 2603 of 4775 done).
- Finished in the last 24 h: none.
- Refresh reads: row 6 is the one refresh read; it started 00:26 CET and has run 7 h 08 min.
- `show` admissions: `[capacity] admitted show: 1 job(s), 4096 MB reserved (16667 MB available, 4096 MB floor)`. No outcome is logged, and no line names `show projects` without a selection on this machine.
- Peak memory in window (footprint heartbeats, lines stamped 2026-11-17 06:32 Z onwards): RSS 11818 MB at 2026-11-17T07:07:37Z; footprint 10592 MB at 2026-11-17T07:53:42Z.
- Current live processes: about 118 `semio/` processes, mostly bun worker children; largest is pid 42303 at 2.9 GB RSS (refresh read child).

## 4. Service failures, last 12 hours (since 2026-11-17 19:32 CET)

15 service-failure files were modified in the window. The `service-failure.txt` copies duplicate the per-reference files, so counts below are unique (origin, reference).

Top failure texts:

| Rank | Failure text (first line of "en") | Count | Newest occurrence (UTC) | Origins |
|---|---|---|---|---|
| 1 | `Error: Original project inference observations required` | 3 | 2026-11-17T22:04:01Z (g1-m08, ref 08a57583) | g1-m07 (x2), g1-m08 |
| 2 | `Error: Original Script process terminated` (4 times in a row on g1-m02) | 2 | 2026-11-18T05:52:36Z (g1-m02, ref b1c96dc7) | g1-m03, g1-m02 |
| 3 | `Error: Build source authority keeps changing during inference` | 2 | 2026-11-18T06:11:20Z (g1-m02, ref f0be4e1a) | g1-m03, g1-m02 |
| 4 | `Error [SEMIO_LEASE_ORDER]: Refusing to wait: native-storage-mapping:5874ac7a... (shared) has been unavailable for 17 s while this process holds repository-graph-fact...` | 1 | 2026-11-18T00:59:59Z (g1-m02, ref 17306fe5) | g1-m02 |
| 5 | `Error: Resource lease timed out: resource-directory-admission:v1` | 1 | 2026-11-18T06:14:04Z (g1-m02, ref 4883957b) | g1-m02 |
| 6 | `Error: ts.preProcessFile is not a function` | 1 | 2026-11-17T20:45:07Z (g1-m01, ref 061a43de) | g1-m01 |

Ranks 4 to 6 are tied at count 1 each; ranks 4 to 6 are listed as the remainder. Total unique failure events: 10.

## 5. Walls a developer would hit today

1. Disk full on g1-m01 root partition: `Partition / has only 5.54 GiB free (97.6% used, limit 97%), so no build or cache write starts before space is freed. Free space with repo cache cas gc, then run the command again.` (`origins/g1-m01/ready-log-workspace-failed.txt`, 2026-11-18T01:41:45Z).
2. Long whole-workspace reads do not finish: row 6 has run 7 h 08 min, still at 2603 of 4775 projects; row 4 was cancelled after 5 h (`read.1794955126322.log`).
3. Heavy derived questions are killed by a 60 s no-output watchdog: `A heavy derived question was stopped: its child process wrote nothing for 60 s` leads to `repository:command failed with exit code 1` (3 graph sessions in window).
4. Foreign holder blocks other runs: g1-m05 `run @semio-tech/fem-3d-rs:check --output=stream --parallel=8` holds the dependency store since 2026-11-18T03:55:14Z; tools probe waited 1074 s (`origins/g1-m05/ready-log-tools-stalled.txt`).
5. Workspace readiness keeps failing on g1-m02 (attempts 54, previous-failed): `Original Script process terminated` (x4 in a row, 05:52:36Z), `Resource lease timed out: resource-directory-admission:v1` (06:14:04Z) (`origins/g1-m02/service-failure-*.txt`, `ready-log-workspace-failed.txt`).
6. Readiness on g1-m01 stuck: attempts 68, exit 1, `waiting for the fleet to infer @semio-tech/s-dev-composition (beyond the expected 1800 s)`, 4472 s at last sample (`origins/g1-m01/ready.json`).
7. Windows g1-m05 tools stage fails on file rename: `Windows relative preservation rename failed (status 0xc0000022; STATUS_ACCESS_DENIED ...)` (`origins/g1-m05/ready-log-tools-failed.txt`, 05:25:10Z). Memory of process trees cannot be observed on win32: `(Command failed: powershell.exe ... Get-CimInstance Win32_Process ...)` (`ready-log-tools.txt`, 06:32:27Z).
8. Nx output rule on g1-m08: `Original process admission closed - first failure: Nx output escapes declared cache partitions` (`origins/g1-m08/ready-log-workspace-failed.txt`, 02:30:58Z).
9. Owned Nx invocation failure on g1-m10: `Internal error: Owned Nx invocation failed (1): nx run-many --projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp --targets=build --parallel=4` (`origins/g1-m10/ready-log-workspace-failed.txt`, 05:07:31Z). Later run on g1-m10 completed at 07:08:54 CET.
10. TypeScript API mismatch in farm inference: `ts.preProcessFile is not a function` (g1-m01, 20:45:07Z), and `typescript (version 5.9.3, wanted 5.9.3) lacks the API the tooling uses: createPr`, which set aside at least 187 owners in the refresh read.
11. Stale refresh stuck on owner waits: `waiting for a farm child to look at the owner ... (the parent computes nothing) [no children]` appears 1502 times in row 4 alone.

Ready state now (`origins/*/ready.json`): g1-m03, g1-m07, g1-m08, g1-m10 ready; g1-m01 (workspace, attempts 68), g1-m02 (workspace, attempts 54) and g1-m05 (tools, attempts 14) still running or retrying.
