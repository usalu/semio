# Round 5 on g1-m07

## Machine
- Mac mini, 17179869184 bytes memory (16 GiB), macOS 27.2.1 (Darwin 27.2.0), 108 GiB free of 228 GiB on the data volume (measured 18:09 UTC).

## Before the run (2026-11-14T18:09:10Z)
- Round 3 had ended (hard stop in its try 4 at 18:06:49Z); nothing was running, no SIGINT was needed. Round 4 was not for this machine.
- Replica code digest (`.cache/f/health.json`, written 18:09:07Z): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`, `store: ok`, sidecar `build-agent` running (pid 43797), `build-scheduler` and `build-cache` off.
- `repo show projects` process: none.
- `repo service host` processes (pid, ppid, age, rss KiB):
  - 15153, 1, 01:11:54, 58976 (`repo service host run`)
  - 15166, 15153, 01:11:52, 89088 (its loader child)
- Last 8 lines of `.cache/f/sidecars/build-agent.log`:

```
2026-11-14T17:53:38.441Z agent code changed, restarting
2026-11-14T17:53:38.444Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-02-lan.fritz.box-43798
2026-11-14T17:53:40.158Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:53:40.215Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:53:40.221Z worker registered: 8915f5a6-bd81-4c1c-972f-2fa7106fe322 cache http://192.168.178.70:6094
2026-11-14T17:58:38.217Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:03:44.565Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:08:51.148Z graph-server: alive, 0 machines seen, 0 requests held, read none
```

- Memory pressure level before the run: 1.

## Try 1 (`~/native-bootstrap-round-5.log`)
- Start 2026-11-14T18:09:23Z, end 2026-11-14T18:10:24Z (61 s), `native_rc=1`. No fleet refusal, no hard stop. 3 samples, all `pressure_level=1`; largest process of the run 1092512 KiB (largest on the machine 2166992 KiB, the `git` of the Claude desktop app).
- New failure, not seen in rounds 1 to 3: macOS denied the newly started graph service access to the worktree. No fleet step was reached: wanted, arrived, missing, seconds: not measured.

End of the bootstrap log, verbatim:

```
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (10053 MB available, 2048 MB floor)
Starting the repo graph service for this host.
Waiting for the repo graph service (3 ms).
Waiting for the repo graph service (2017 ms).
Waiting for the repo graph service (4067 ms).
[repo] still working after 5 s; last phase started: door.script
Waiting for the repo graph service (6096 ms).
Loading the project graph of 440cffa5563d.
Planning 1 targets.
Reading the project graph slice (2 s).
macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow `repo` under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run `repo service restart`, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-6feebb05efe3.log.
end 2026-11-14T18:10:24Z
native_rc=1
```

Graph service log `host-6feebb05efe3.log` (service pid 55157, child 55175, started 18:10:14Z by this try, implementation `00f097676be003fe51b0308f7dc9aaefca31fca3b502f72c4d3f2b0b37320f70`): no line with `waiting for the fleet`, `taken from the fleet`, `inferred here`, `refreshing N projects`, `asking the host for` or `stale facts`. Its last lines before the failure, verbatim:

```
[script] build:source-admission · Admitting the sources of the build
[script] build:implementation-source · Reading the sources of the build implementation
[repo] still working after 5 s; last phase started: source.served
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (8096 MB available, 2048 MB floor)
[script] build:source-admission · Admitting the sources of the build
2026-11-14T18:10:24.048Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
```

- After the try two graph services exist: the old one of round 3 (pid 15153, parent 1, 1 h 14 min old, 59536 KiB, log `host-f74179f31e9f.log`) and the new one (pid 55157, parent 1, 487504 KiB). The services of rounds 2 and 3 were started the same way from the same session and had access to the worktree.
- This is neither `native_rc=75` nor a refusal text. I run one more try all the same (within the 10 tries of the round) to see whether the denial repeats; nothing else was changed.

## Tries 2 to 6
| try | start (UTC) | end (UTC) | native_rc | how it ended | wanted | arrived | missing | seconds | peak RSS of one process | samples |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 18:09:23 | 18:10:24 | 1 | macOS denied the new service access (EPERM) | not measured | not measured | not measured | not measured | 1092512 KiB (run) | 3, level 1 |
| 2 | 18:12:17 | 18:38:35 | 75 | fleet refusal | 454 | 78 | 376 | 1483 | 2914080 KiB | 53, level 1 |
| 3 | 18:40:55 | 18:54:00 | 7 | hard stop, pressure level 4 | 573 | 389 (last count seen) | 184 | not measured (no refusal) | 3418720 KiB | 25 level 1, 1 level 2, 1 level 4 |
| 4 | 18:58:00 | 19:05:34 | 7 | hard stop, pressure level 4 | no fleet line | - | - | - | 5143008 KiB | 14 level 1, 1 level 2, 1 level 4 |
| 5 | 19:09:35 | 19:17:37 | 7 | hard stop, pressure level 4 | no fleet line | - | - | - | 6111072 KiB | 16 level 1, 1 level 4 |
| 6 | 19:21:37 | 19:25:29 | 1 | service failure `ed6d3ee0` (ENOENT of an input closure record) | no fleet line | - | - | - | 3684656 KiB | 8, level 1 |

- Pauses: 120 s after tries 1, 2; 180 s after the hard stops of tries 3, 4, 5; 120 s after try 6.
- The access denial of try 1 did not repeat: try 2 used the same service (pid 55157) and read the worktree.
- No process of the run was above 8 GB resident in any sample. Level 2 never lasted five samples; all three stops were level 4, one SIGINT each, the run was gone one minute later each time.

### Try 2: fleet refusal
Bootstrap log, verbatim: `The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 454 and delivered 78 within 1483 s, 376 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.`

Service log `host-6feebb05efe3.log`, verbatim, repeat counts in brackets:
```
graph: 651 stale projects, 197 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 454
graph: waiting for the fleet: 0 of 454 projects arrived      [12]
graph: waiting for the fleet: 30 of 454 projects arrived     [63]
graph: waiting for the fleet: 78 of 454 projects arrived     [74]
graph: nothing to wait for any longer: asked the fleet for 454 stale projects and received 78 within 1483 s, 376 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T18:38:35.736Z graph-wanted-expired {"wanted":"454","arrived":"78","seconds":"1483","missing":"376","local":"192"}
2026-11-14T18:42:57.581Z shutdown superseded
```
No `refreshing`, `asking the host for`, `stale facts` or `inferred here` line in this try.

### Try 3: fleet delivered 389 of 573, then the worker refreshed all 651 itself; hard stop at 18:53:59Z
Service log `host-a561d8014168.log` (new service pid 72817, child 72824, started 18:41:47Z), verbatim:
```
graph: 651 stale projects, 78 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 573
graph: waiting for the fleet: 0 of 573 projects arrived      [10]
graph: waiting for the fleet: 245 of 573 projects arrived    [19]
graph: waiting for the fleet: 293 of 573 projects arrived    [11]
graph: waiting for the fleet: 341 of 573 projects arrived    [5]
graph: waiting for the fleet: 389 of 573 projects arrived    [1]
graph: refreshing 651 projects (generation 74)
graph: 509 stale facts (file 245, analysis 223, field 37, digest 4) leave 651 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
graph: inferring projects with 1 worker processes (1536 MB each)
graph: waiting for memory headroom (752 of 805 MB reserved, 7 projects running)
graph: inferring projects with 1 worker processes (1536 MB each)
graph: waiting for memory headroom (1137 of 1019 MB reserved, 6 projects running)
graph: waiting for memory headroom (887 of 1005 MB reserved, 3 projects running)
graph: waiting for memory headroom (1061 of 1180 MB reserved, 3 projects running)
graph: waiting for memory headroom (471 of 375 MB reserved, 1 projects running)
graph: waiting for memory headroom (480 of 448 MB reserved, 1 projects running)
```
- Note the first line: "78 taken from the fleet" although try 2 had 197 taken plus 78 arrived (275). The count taken went down between tries, and the refresh again names all 651 projects, not the rest.
- Samples before the stop: 18:52:58Z level 1 (largest 3418720 KiB), 18:53:29Z level 2, 18:53:59Z level 4.
- Last `[capacity]` lines of the bootstrap log, verbatim:

```
[capacity] run: work has to run, waiting 26s to grow the reservation from 2048 MB to 8190 MB: memory pressure is critical (-23630 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] run: work has to run, waiting 31s to grow the reservation from 2048 MB to 2574 MB: memory pressure is critical (-26102 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] run: work has to run, waiting 37s to grow the reservation from 2048 MB to 2574 MB: memory pressure is critical (-29537 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] run: work has to run, waiting 42s to grow the reservation from 2048 MB to 2574 MB: memory pressure is critical (-29537 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind the host itself (nothing of ours)
```

### Try 4: no fleet step, refresh of 651 at once; hard stop at 19:05:34Z
```
graph: refreshing 651 projects (generation 74)
graph: 509 stale facts (file 245, analysis 223, field 37, digest 4) leave 651 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
graph: inferring projects with 1 worker processes (1536 MB each)
graph: waiting for memory headroom (882 of 1024 MB reserved, 2 projects running)
graph: waiting for memory headroom (441 of 992 MB reserved, 1 projects running)
graph: waiting for memory headroom (803 of 966 MB reserved, 1 projects running)
graph: waiting for memory headroom (803 of 965 MB reserved, 1 projects running)
graph: waiting for memory headroom (840 of 1194 MB reserved, 7 projects running)
graph: waiting for memory headroom (668 of 935 MB reserved, 2 projects running)
graph: inferring projects with 1 worker processes (1536 MB each)      [2]
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
graph: 1 of 48 projects inferred by 3 worker processes in 273 s (0 failed, 0 lost, 0 restarts)
```
- Progress seen in this try: `Inferring projects: 45 of 48 projects` (first wave).
- Samples before the stop: 19:04:32Z level 1 (slice process 3334976 KiB, a child of it 3291232 KiB, the app's `git` 2451616 KiB), 19:05:02Z level 2 (5143008 KiB), 19:05:34Z level 4.

### Try 5: same shape; hard stop at 19:17:37Z
```
graph: refreshing 651 projects (generation 74)
graph: 509 stale facts (file 245, analysis 223, field 37, digest 4) leave 651 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
graph: inferring projects with 1 worker processes (1536 MB each)
graph: waiting for memory headroom (15947 of 1024 MB reserved, 1 projects running)
graph: waiting for memory headroom (15947 of 1102 MB reserved, 1 projects running)
graph: waiting for memory headroom (15947 of 784 MB reserved, 1 projects running)
graph: waiting for memory headroom (486 of 752 MB reserved, 1 projects running)
graph: waiting for memory headroom (15947 of 971 MB reserved, 1 projects running)
graph: waiting for memory headroom (555 of 913 MB reserved, 6 projects running)
graph: waiting for memory headroom (184 of 773 MB reserved, 1 projects running)
graph: waiting for memory headroom (750 of 769 MB reserved, 1 projects running)
graph: inferring projects with 1 worker processes (1536 MB each)      [3]
```
- "15947 of 1024 MB reserved": one project is booked with 15947 MB.
- Samples before the stop: 19:16:36Z level 1 (3365200 KiB), 19:17:06Z level 1 (6100336 KiB, plus the app's `git` 1702304 KiB), 19:17:37Z level 4. The pressure went from 1 to 4 within one 30 s sample.
- `[capacity]` lines of the service log near the stops, verbatim:

```
[capacity] waiting 82s for exec: not enough free memory (need 1536 MB; -6777 MB spare above the 2048 MB floor, load 13.89; 6 running, 0 ahead); behind graph-plan (running, 1 job(s), 21063 MB), workspace:setup (quiet, 1 job(s), 50 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] waiting 87s for exec: memory pressure is critical (need 1536 MB; -6777 MB spare above the 2048 MB floor, load 16.22; 6 running, 0 ahead); behind the host itself (nothing of ours)
```

### Try 6: the graph step got to its end, then a missing record
```
graph: refreshing 1 projects (generation 75)
graph: 55 stale facts (analysis 36, readdir 17, exists 1, readdirNames 1) leave 1 projects stale
graph: 3 derived results asked again after 55 stale facts: 3 unchanged, 0 changed
2026-11-14T19:25:29.267Z request-failed ed6d3ee0 Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/9a4568e5a997d393e37763151f3aaca7b06212ff1bbf361b10aa3051b7893b8e.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (…/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (…/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (…/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
```
- So generation 74 was completed somewhere between try 5 and try 6 (only 1 project stale at generation 75). Whether this machine finished it after the SIGINT of try 5 or took it from the fleet: not measured.
- Bootstrap log, verbatim: `The resident service met an unexpected failure (reference ed6d3ee0); try again, the details under that reference are in the service log …/host-a561d8014168.log.`
- Not a refusal and not a hard stop; I continue with try 7 within the 10 tries of the round, because the graph step is now past.

### The `git` of the Claude desktop app
- Still among the three largest in most samples of this round, largest 2536928 KiB. Same command and parent chain as reported in round 3 (`Claude Helper` → `Claude.app` → launchd). It was 2451616 KiB in the sample before the level 2 of try 4.

## Try 7 (`…-round-5-try-7.log`)
- Start 2026-11-14T19:27:38Z, end 2026-11-14T19:31:19Z (221 s), `native_rc=1`. No fleet line, no hard stop; 8 samples, all level 1; largest process 3211120 KiB.
- Same failure as try 6, same reference. Bootstrap log, verbatim:

```
[semio] workspace:setup…
[capacity] run: sized at 102% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8711 MB available, 2048 MB floor)
Planning 1 targets.
The resident service met an unexpected failure (reference ed6d3ee0); try again, the details under that reference are in the service log /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-a561d8014168.log.
end 2026-11-14T19:31:19Z
native_rc=1
```
- Service log lines of this try: see "After try 6" below.
- Tries 8 to 10 were not run: the failure repeats unchanged, it is neither `native_rc=75`, a refusal text, a hard stop nor a code change, so more tries would only repeat it.

## Result of round 5
- Not ready: last `native_rc=1`, marker `.cache/f/native-bootstrap.done` absent. Failed step: `workspace:setup`, now behind the graph refresh, at reading an input closure record. No nx task ran in any try, so there is no failed nx task to list.
- What works now on this 16 GiB worker: the request to the fleet and its answer in steps (30, 78; 245, 293, 341, 389).
- What stopped it: (1) after a partial answer the service refreshes all 651 projects itself instead of only the rest, which drove the machine to pressure level 4 three times; (2) once the graph was complete, the missing record `.cache/b/g/m/build-input-closures/9a4568e5….json`.
- Wall time per phase: not measured one by one. Whole tries: 61 s, 1578 s, 785 s, 454 s, 482 s, 232 s, 221 s.
- Files: `round-5*.samples.txt`, `round-5*.monitor.txt`, `round-5.tries.txt`, `round-5.tail.txt` (last 200 lines of try 7).

## After try 6: service log lines of try 7 (`host-a561d8014168.log`), verbatim
```
graph: waiting for the graph build of a peer (23 s in the queue, generation -1 was the newest when this command queued)
graph: waiting for the graph build of a peer (58 s in the queue, generation -1 was the newest when this command queued)
graph: generation 60 (5099 projects) restored from the remote cache, 5103 documents (48 MB) downloaded in 11 s
graph: No verified change journal cursor is recorded for generation 60; verifying the tree once.
graph: refreshing 1 projects (generation 61)
graph: 47 stale facts (analysis 36, readdir 11) leave 1 projects stale
graph: workspace is stale: the workspace file holder was removed
graph: workspace is stale: the workspace file holder/claims was removed
graph: os-hub is stale: the workspace file 🌎️hub/📦️packages/🦀️rust/.semio was removed
(13 more "is stale … was removed" lines of the projects workspace and os-hub)
graph: 47 more stale inputs are not listed
graph: headroom of 1024 MB is below one project (1527 MB), inferring one project at a time
graph: generation 61 published, 1 projects refreshed in 100 s (0 kept their edges)
graph: fact index of generation 61: 748846 facts of 5100 projects, 86.2 MB expanded (16.8 % of the 512 MB limit), 14.0 MB packed
graph: 1 derived results asked again after 47 stale facts: 0 unchanged, 0 changed
2026-11-14T19:31:19.488Z request-failed ed6d3ee0 Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/9a4568e5a997d393e37763151f3aaca7b06212ff1bbf361b10aa3051b7893b8e.json'
```
- The local generation numbers went back: try 6 refreshed to generation 75, try 7 found "generation -1" as the newest, restored generation 60 (5099 projects) from the remote cache and published generation 61. The graph step itself now ends on this machine (100 s for the one stale project, one project at a time).
- The record that is missing does not exist at 19:32Z; the folder `.cache/b/g/m/build-input-closures` holds 16758 entries.
- Graph service after the round: pid 72817 (parent 1, started 18:41:47Z), child 72824.
