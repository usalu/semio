# g1-m01 Round 17

Rounds 14, 15 and 16 were posted while the round 13 runner was still going and the agent session that hosted it had ended (about 03:35Z); they were first read at 03:37Z. This round therefore carries their steps too: the hard stops of round 15, step 0 of round 16 (`repo tools setup all`, once) and the facts of round 17. No separate round 14, 15 or 16 run exists for this machine.

## Before The Run (2026-11-15T03:38:45Z)

- Replica agent: `state = running`; 0 kickstarts. Replica code digest: `97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea`, the same code `97445096cc0a` as g1-server in the replica `status` of 03:45Z.
- Agreement: `status` says `The fleet is not in agreement (10 origins; …)`; g1-server is `ok`, g1-m01 `differs in view.digest at a common checkpoint: #23, #127, #153` (3 of 256 buckets). The first try was not held back for it: the fleet has not reported agreement at any check since 13 November while files keep moving.
- Memory pressure level 1; free disk 82 GiB of 228 GiB.

## Result

Not ready: twelve tries, none reached the setup tasks, the marker `.cache/f/native-bootstrap.done` does not exist. `native_rc` of the last try: 75. Step 0 (`repo tools setup all`) ended with exit code 0 after 1895 s.

## What Differs From The Round Text

- The runner ran unattended from 03:38Z to 08:39Z with the rules of round 17 as read at 03:37Z. Rounds 18 to 27 were posted meanwhile and were first read at 09:10Z; their extra copies (`access-proof` line per try, `ls -laO@`, `mount | grep -c semio`, `ps … service host` right after a failure) were not taken at the time of the failure. What the service logs still hold is copied per try below; the last 40 bootstrap lines of every `native_rc=1` try are in `round-17.bootstrap-tails.txt`.
- Tries 4 and 5 were stopped by my runner at memory pressure level 4 in two samples in a row (rule of round 15). Round 23 (05:44Z) removed that stop: try 4 was stopped at 05:42:54Z before it, try 5 at 06:10:28Z after it, against the then current rule. The round 27 runner has no pressure stop.
- Sample files with a colon in the name were renamed at 09:20Z (`round-13.p12-try-*.sample-…T014525Z.txt`, `round-17.p12-try-*.sample-…Z.txt`), round 27.

## Observations For The Coordinator

- Resident size and `footprint` of the slice worker disagree in both directions (same pid, same second): try 12 at 08:36:50Z 8175 MB resident with footprint 2682 MB; round 27 try 1 at 09:48:42Z 2206 MB resident with footprint 9589 MB. Which of the two the plan guard (`grew to 10035 MB`) reads: not measured.
- Restores seen in this round: generation 222 from a state 106 and 117 min old with 73 projects stale, generation 229 from a state 28 min old with 3146 projects stale (remote cache, 5104 documents, 14 to 22 s). Tries without a restore line inferred here from the start.
- Try 12: `waiting for memory headroom (4453 of 0 MB reserved, 1 projects running)`: the reserve total prints as 0 MB.
- Tries 8 to 12 took `0 projects … from the fleet` in the sampler's last `graph:` lines and inferred here (50, then 150 in try 12) until the guard stopped the plan; closure entries on this machine grew 42033 to 42177 in try 12, so part of the inference is kept for the next try.
- Access: tries 7, 8, 9, 11 and 12 print that the launchd route was refused for this boot and the service runs under the calling app; try 7 ended there (direct service refused too), the later ones went on.
- Other memory users on this machine during the round (30 s samples, three largest processes) are in `round-17.samples.txt`.

## Tries

| Try | Start | End | Wall s | Result | Where it ended |
|---|---|---|---|---|---|
| 1 | 04:10:29Z | 04:15:36Z | 307 | native_rc=1 | graph service: unexpected failure (reference b72e2264) while planning `workspace:setup` |
| 2 | 04:18:38Z | 04:27:16Z | 518 | native_rc=75 | fleet refusal: asked for 656, delivered 0 within 301 s; can infer only 128 |
| 3 | 04:30:19Z | 05:03:38Z | 1999 | native_rc=75 | fleet refusal: asked for 701, delivered 0 within 300 s; can infer 0 |
| 4 | 05:06:41Z | 05:43:55Z | 2234 | native_rc=1 | my runner's SIGINT (pressure level 4 twice, rule of round 15) at 1800 s of the graph read |
| 5 | 05:47:19Z | 06:11:07Z | 1428 | native_rc=1 | my runner's SIGINT (pressure level 4 twice) at 631 s of the graph read |
| 6 | 06:14:29Z | 06:49:48Z | 2119 | native_rc=1 | slice worker ended 4 times with exit 75: code on this machine kept changing under it |
| 7 | 06:52:51Z | 07:02:59Z | 608 | native_rc=1 | access: launchd route refused, direct service refused too |
| 8 | 07:06:02Z | 07:37:50Z | 1908 | native_rc=75 | plan guard: stage inference 9954 MB, 2854 MB available |
| 9 | 07:40:52Z | 07:49:30Z | 518 | native_rc=75 | plan guard: stage inference 10058 MB, 2743 MB available |
| 10 | 07:52:32Z | 07:55:09Z | 157 | native_rc=75 | plan guard: stage inference 6729 MB, 3352 MB available |
| 11 | 07:58:11Z | 08:04:45Z | 394 | native_rc=75 | plan guard: stage inference 8808 MB, 2294 MB available |
| 12 | 08:07:47Z | 08:39:31Z | 1904 | native_rc=75 | plan guard: stage inference 10035 MB, 4100 MB available |

## Step 0: `repo tools setup all`

```
exit code: 0
wall time: 1895 s
interrupted for the dependency store wait: no
last 15 lines:
INFO: Found 1 target...
[2 / 2] no actions running
[repository:command] still running after 1860 s
Target //:setup up-to-date:
  bazel-bin/ready.txt
INFO: Elapsed time: 17.720s, Critical Path: 1.43s
INFO: 2 processes: 2 internal.
INFO: Build completed successfully, 2 total actions
[bazel:module-setup] still running after 30 s
[bazel] fixed module catalog ready blobs=156
[bazel] ready binary=8.4.2 darwin/arm64 sha256=45e9388abf21d110 modules=156 blobs lock=06c085a17c9611ed
[bazel] readiness {"version":1,"bazel":{"version":"8.4.2","platform":"darwin","arch":"arm64","sha256":"45e9388abf21d1107e146ea366ad080eb93cb6a5f3a4a3b048f78de0bc3faffa","executable":"/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"},"modules":{"lockSha256":"06c085a17c9611edb2348fa8f6eb9726db9d85a5f3a9edebd2d2958c688d2df0","blobs":156}}
[semio] The in-house build engine needs no external tool to provision.
[semio] Die hauseigene Build-Engine benötigt kein externes Werkzeug.
[repository:command] still running after 1870 s
```

## Per Try

### Try 1: native_rc=1

- Start 2026-11-15T04:10:29Z, end 2026-11-15T04:15:36Z, wall time 307 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: b72e2264.
- Bootstrap log `~/native-bootstrap-round-17.log` (911 lines cleaned); service log(s): host-c5f530f58f3a.log from line 2321.
- Pressure samples by level (30 s): {"1": 9, "4": 1}. Largest process of the run in the 30 s samples: 1981 MB resident.
- Slice worker sampler (5 s): peak 3106 MB resident at this line:

```
2026-11-15T04:14:49Z pid=81064 rss=3106MB footprint=[bun [81064]: 64-bit    Footprint: 1216 MB (16384 bytes per page)] last=[graph: 44 derived results asked again after 1277 stale facts: 8 unchanged, 36 changed]
```

- closure entries before: 41859
- closure entries after: 41863
- Last message of the bootstrap:

```
The resident service met an unexpected failure (reference b72e2264); try again, the details under that reference are in the service log /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-2369f6f2a056.log.
```

- Restore line: none
- First plan line (`per project`): none
- `taken from the fleet` / `inferred here` lines: none
- `too heavy` lines: none
- Plan stage lines: none
- `waiting for the fleet` lines: none
- `waiting for memory headroom` lines: none
- `access-proof`, `where:`, EPERM and refusal lines: none
- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2991 MB because of its plan phase
[capacity] waiting 0s for run: not enough free memory (need 2991 MB; 182 MB spare above the 2048 MB floor, load 38.26; 1 running, 0 ahead); behind @teaching/staffage-react:test (quiet, 1 job(s), 2048 MB)
[capacity] waiting 6s for run: not enough free memory (need 2991 MB; 903 MB spare above the 2048 MB floor, load 35.6; 1 running, 0 ahead); behind @teaching/staffage-react:test (quiet, 1 job(s), 2048 MB)
… 10 more distinct lines left out …
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 28 MB spare above the 2048 MB floor, load 12.3; 1 running, 1 ahead); behind @teaching/staffage-react:test (waiting to grow, 1 job(s), 51 MB)
[capacity] admitted run: 1 job(s), 2048 MB reserved (10183 MB available, 2048 MB floor)
```

### Try 2: native_rc=75

- Start 2026-11-15T04:18:38Z, end 2026-11-15T04:27:16Z, wall time 518 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-2.log` (820 lines cleaned); service log(s): host-2369f6f2a056.log from line 1380.
- Pressure samples by level (30 s): {"1": 17}. Largest process of the run in the 30 s samples: 64 MB resident.
- Slice worker sampler (5 s): peak 2627 MB resident at this line:

```
2026-11-15T04:22:11Z pid=88691 rss=2627MB footprint=[bun [88691]: 64-bit    Footprint: 1208 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 656 projects arrived]
```

- closure entries before: 41863
- closure entries after: 41863
- Last message of the bootstrap:

```
The project graph could not be read on this machine: it can infer only 128 of the stale projects, the fleet was asked for 656 and delivered 0 within 301 s, 656 are still missing, among them the too heavy project(s) @semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Restore line: none
- First plan line (`per project`):

```
graph: 656 stale projects, 0 taken from the fleet, 3 too heavy for this machine (@semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB); it can give about 3512 MB per project), it can infer 128 of the others (32 per wave); asking the fleet for 656
graph: nothing to wait for any longer: the fleet was asked for 656 stale projects and delivered 0 within 301 s; 656 remain, among them the too heavy project(s) @semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
… 1 more distinct lines left out …
graph: 3 stale projects, 0 taken from the fleet, 2 too heavy for this machine (@semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB); it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 3
The project graph could not be read on this machine: it can infer only 128 of the stale projects, the fleet was asked for 656 and delivered 0 within 301 s, 656 are still missing, among them the too heavy project(s) @semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- `taken from the fleet` / `inferred here` lines:

```
graph: 656 stale projects, 0 taken from the fleet, 3 too heavy for this machine (@semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB); it can give about 3512 MB per project), it can infer 128 of the others (32 per wave); asking the fleet for 656
graph: 3 stale projects, 0 taken from the fleet, 2 too heavy for this machine (@semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB); it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 3
```

- `too heavy` lines:

```
graph: 656 stale projects, 0 taken from the fleet, 3 too heavy for this machine (@semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB); it can give about 3512 MB per project), it can infer 128 of the others (32 per wave); asking the fleet for 656
graph: nothing to wait for any longer: the fleet was asked for 656 stale projects and delivered 0 within 301 s; 656 remain, among them the too heavy project(s) @semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-15T04:27:06.651Z graph-wanted-expired {"wanted":"656","arrived":"0","seconds":"301","missing":"656","local":"128","heavy":", among them the too heavy project(s) @semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB) (this machine can give about 3512 MB per project)"}
graph: 3 stale projects, 0 taken from the fleet, 2 too heavy for this machine (@semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB); it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 3
```

- Plan stage lines: none
- `waiting for the fleet` lines:

```
graph: waiting for the fleet: 0 of 656 projects arrived
graph: waiting for the fleet: 0 of 3 projects arrived
```

- `waiting for memory headroom` lines: none
- `access-proof`, `where:`, EPERM and refusal lines: none
- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2991 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3642 MB reserved (9741 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
… 1 more distinct lines left out …
[capacity] admitted dependencies: 1 job(s), 4817 MB reserved (8732 MB available, 2048 MB floor)
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6941 MB available, 2048 MB floor)
```

### Try 3: native_rc=75

- Start 2026-11-15T04:30:19Z, end 2026-11-15T05:03:38Z, wall time 1999 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-3.log` (1438 lines cleaned); service log(s): host-2369f6f2a056.log from line 2554, host-ada6c7a0852c.log.
- Pressure samples by level (30 s): {"1": 49, "2": 13, "4": 4}. Largest process of the run in the 30 s samples: 2214 MB resident.
- Slice worker sampler (5 s): peak 1831 MB resident at this line:

```
2026-11-15T04:56:17Z pid=31679 rss=1831MB footprint=[bun [31679]: 64-bit    Footprint: 1586 MB (16384 bytes per page)] last=[]
```

- closure entries before: 41863
- closure entries after: 41863
- Last message of the bootstrap:

```
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 701 and delivered 0 within 300 s, 701 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Restore line: none
- First plan line (`per project`):

```
graph: nothing to wait for any longer: the fleet was asked for 3 stale projects and delivered 0 within 300 s; 3 remain, among them the too heavy project(s) @semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB), which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-15T04:34:53.487Z graph-wanted-expired {"wanted":"3","arrived":"0","seconds":"300","missing":"3","local":"0","heavy":", among them the too heavy project(s) @semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB) (this machine can give about 0 MB per project)"}
… 2 more distinct lines left out …
2026-11-15T04:47:12.421Z graph-wanted-expired {"wanted":"4","arrived":"0","seconds":"300","missing":"4","local":"0","heavy":", among them the too heavy project(s) @semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB) (this machine can give about 0 MB per project)"}
graph: 701 stale projects, 0 taken from the fleet, 149 too heavy for this machine (@semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB), @semio-tech/framework-os-dev (about 2892 MB), @semio-tech/framework-os-kernel (about 2864 MB), @semio-tech/s-dev-composition (about 2830 MB), and 143 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 701
```

- `taken from the fleet` / `inferred here` lines:

```
graph: 4 stale projects, 0 taken from the fleet, 2 too heavy for this machine (@semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB); it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 4
graph: 701 stale projects, 0 taken from the fleet, 149 too heavy for this machine (@semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB), @semio-tech/framework-os-dev (about 2892 MB), @semio-tech/framework-os-kernel (about 2864 MB), @semio-tech/s-dev-composition (about 2830 MB), and 143 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 701
```

- `too heavy` lines:

```
graph: nothing to wait for any longer: the fleet was asked for 3 stale projects and delivered 0 within 300 s; 3 remain, among them the too heavy project(s) @semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB), which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-15T04:34:53.487Z graph-wanted-expired {"wanted":"3","arrived":"0","seconds":"300","missing":"3","local":"0","heavy":", among them the too heavy project(s) @semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB) (this machine can give about 0 MB per project)"}
graph: 4 stale projects, 0 taken from the fleet, 2 too heavy for this machine (@semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB); it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 4
graph: nothing to wait for any longer: the fleet was asked for 4 stale projects and delivered 0 within 300 s; 4 remain, among them the too heavy project(s) @semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB), which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-15T04:47:12.421Z graph-wanted-expired {"wanted":"4","arrived":"0","seconds":"300","missing":"4","local":"0","heavy":", among them the too heavy project(s) @semio-tech/framework-process (about 1788 MB), @semio-tech/repo-script-providers (about 1218 MB) (this machine can give about 0 MB per project)"}
graph: 701 stale projects, 0 taken from the fleet, 149 too heavy for this machine (@semio-tech/framework-surface-paint-rs (about 6950 MB), @semio-tech/framework-surface-node-graph-rs (about 4256 MB), @semio-tech/repo-lib (about 3950 MB), @semio-tech/framework-os-dev (about 2892 MB), @semio-tech/framework-os-kernel (about 2864 MB), @semio-tech/s-dev-composition (about 2830 MB), and 143 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 701
```

- Plan stage lines: none
- `waiting for the fleet` lines:

```
graph: waiting for the fleet: 0 of 3 projects arrived
graph: waiting for the fleet: 0 of 4 projects arrived
graph: waiting for the fleet: 0 of 701 projects arrived
```

- `waiting for memory headroom` lines: none
- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T05:03:24.242Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=11588 ppid=10824
```

- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2992 MB because of its plan phase
[capacity] waiting 0s for run: not enough free memory (need 2992 MB; 2273 MB spare above the 2048 MB floor, load 42.98; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 1160 MB), @teaching/staffage-react:test (quiet, 1 job(s), 47 MB)
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
… 6 more distinct lines left out …
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] waiting 0s for run: memory pressure is raised (need 512 MB; 1490 MB spare above the 2048 MB floor, load 31.46; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 512 MB reserved (4753 MB available, 2048 MB floor)
```

### Try 4: native_rc=1

- Start 2026-11-15T05:06:41Z, end 2026-11-15T05:43:55Z, wall time 2234 s. Hard stop: memory pressure level 4 in two samples in a row. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-4.log` (1729 lines cleaned); service log(s): host-ada6c7a0852c.log from line 2490, host-b814083e21d9.log.
- Pressure samples by level (30 s): {"1": 55, "2": 15, "4": 3}. Largest process of the run in the 30 s samples: 3531 MB resident.
- Slice worker sampler (5 s): peak 1852 MB resident at this line:

```
2026-11-15T05:13:32Z pid=69379 rss=1852MB footprint=[bun [69379]: 64-bit    Footprint: 165 MB (16384 bytes per page)] last=[graph: generation 229 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 19 s, from a state 28 min old with 3146 projects stale]
```

- closure entries before: 41863
- closure entries after: 41863
- Last message of the bootstrap:

```
Internal error: Original Script process interrupted
```

- Restore line:

```
graph: generation 229 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 19 s, from a state 28 min old with 3146 projects stale
```

- First plan line (`per project`):

```
graph: 3146 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/framework-os-dev (about 23928 MB), @semio-tech/repo-lib (about 16690 MB), workspace (about 13912 MB), @semio-tech/framework-plugin (about 7494 MB), repo-schema-commands (about 5744 MB), repo (about 4430 MB); it can give about 3512 MB per project), it can infer 92 of the others (23 per wave); asking the fleet for 3146
```

- `taken from the fleet` / `inferred here` lines:

```
graph: 3146 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/framework-os-dev (about 23928 MB), @semio-tech/repo-lib (about 16690 MB), workspace (about 13912 MB), @semio-tech/framework-plugin (about 7494 MB), repo-schema-commands (about 5744 MB), repo (about 4430 MB); it can give about 3512 MB per project), it can infer 92 of the others (23 per wave); asking the fleet for 3146
```

- `too heavy` lines:

```
graph: 3146 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/framework-os-dev (about 23928 MB), @semio-tech/repo-lib (about 16690 MB), workspace (about 13912 MB), @semio-tech/framework-plugin (about 7494 MB), repo-schema-commands (about 5744 MB), repo (about 4430 MB); it can give about 3512 MB per project), it can infer 92 of the others (23 per wave); asking the fleet for 3146
graph: the fleet did not deliver 6 too heavy project(s) (@semio-tech/framework-os-dev (about 23928 MB), @semio-tech/repo-lib (about 16690 MB), workspace (about 13912 MB), @semio-tech/framework-plugin (about 7494 MB), repo-schema-commands (about 5744 MB), repo (about 4430 MB)); this machine infers them itself, one at a time and alone after the others, each with up to about 3752 MB
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 182 MB, resident 138 MB)
```

- `waiting for the fleet` lines:

```
graph: waiting for the fleet: 0 of 3146 projects arrived
graph: waiting for the fleet: 0 of 3146 projects arrived; the server last progressed 8 s ago, estimated time left: about 4 min
graph: waiting for the fleet: 0 of 3146 projects arrived; the server last progressed 18 s ago, estimated time left: about 4 min
… 77 more distinct lines left out …
graph: waiting for the fleet: 0 of 3146 projects arrived; the server last progressed 566 s ago, estimated time left: about 4 min
graph: waiting for the fleet: 0 of 3146 projects arrived; the server last progressed 576 s ago, estimated time left: about 4 min
graph: waiting for the fleet: 0 of 3146 projects arrived; the server last progressed 586 s ago, estimated time left: about 4 min
```

- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (768 of 262 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 768 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (768 of 0 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 768 MB and has waited 6 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 0 of 4774 projects · 1 running, longest 5 s: @semio-tech/mit-bestand-praesentation-projektetage (project, 5 s, 0 rows) · waiting for memory headroom (768 of 0 MB reserved)
… 37 more distinct lines left out …
[script] build:projects · Inferring projects: 0 of 4774 projects · 2 running, longest 115 s: @semio-tech/mit-bestand-praesentation-projektetage (project, 115 s, 0 rows), @semio-tech/writer-writer-rs (language:rust, 55 s, 0 rows) · waiting for memory headroom (1031 of 1283 MB reserved)
[script] build:projects · Inferring projects: 0 of 4774 projects · 2 running, longest 120 s: @semio-tech/mit-bestand-praesentation-projektetage (project, 120 s, 0 rows), @semio-tech/writer-writer-rs (language:rust, 60 s, 0 rows) · waiting for memory headroom (1031 of 1283 MB reserved)
graph: waiting for memory headroom (1031 of 1539 MB reserved, 2 projects running): @semio-tech/mit-bestand-bericht needs 768 MB and has waited 63 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
```

- `access-proof`, `where:`, EPERM and refusal lines: none
- `closure-missing` lines:

```
[graph] closure-missing 0160357ce062320aa974ddb16dccf1c9666892d01001ee49f3c3879a50f42407 named by 1 projects: re-derived
[graph] closure-missing 128df6cefe6051b6136e1cd3a1ea493876aaed9bf7142e8f342f9e8916c4ebb4 named by 1 projects: re-derived
[graph] closure-missing 135d24b504cd908d498056c666c80d6175c19c9b1f932e48805a51d20f2d569e named by 3 projects: re-derived
[graph] closure-missing 22e36291644aab00d9fcc1e4a4955f804b6bd47da023f17ef269f419eb7073ae named by 1 projects: re-derived
… 43 more distinct lines left out …
[graph] closure-missing ef1ccf20226e6e6230a310cca299cb3c20d3cf7647bed79828b9ce2d35f50f68 named by 1 projects: refetched
[graph] closure-missing f679da6d2eb584fe3ceea8a03a2c60bd6e93c1ec62051f9a09e1317e53b0fcf9 named by 1 projects: refetched
[graph] closure-missing fa5d1c802ed77461d4dc4708137830ec0b0f0481b16fe92f34df429f02c0e3b9 named by 1 projects: refetched
[graph] closure-missing fdbd7c8b8f7654ef5db01e14467b603b2bbd9a47a79f6cd0f3f6dc3d53f6dfa7 named by 1 projects: refetched
```

- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2996 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3689 MB reserved (6996 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
… 1 more distinct lines left out …
[capacity] admitted dependencies with 4133 of 4827 MB: only 4133 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 512 MB reserved (8394 MB available, 2048 MB floor)
```

### Try 5: native_rc=1

- Start 2026-11-15T05:47:19Z, end 2026-11-15T06:11:07Z, wall time 1428 s. Hard stop: memory pressure level 4 in two samples in a row. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-5.log` (1214 lines cleaned); service log(s): host-b814083e21d9.log from line 2214, host-d71e909e3d92.log.
- Pressure samples by level (30 s): {"1": 32, "2": 12, "4": 3}. Largest process of the run in the 30 s samples: 3118 MB resident.
- Slice worker sampler (5 s): peak 2963 MB resident at this line:

```
2026-11-15T06:09:35Z pid=54503 rss=2963MB footprint=[bun [54503]: 64-bit    Footprint: 4566 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (1259 of 0 MB reserved, 1 projects running): @semio-tech/mathematical-js needs 795 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwi]
```

- closure entries before: 41864
- closure entries after: 41882
- Last message of the bootstrap:

```
Internal error: Original Script process interrupted
```

- Restore line:

```
graph: generation 222 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 14 s, from a state 106 min old with 73 projects stale
graph: generation 222 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 22 s, from a state 117 min old with 73 projects stale
```

- First plan line (`per project`):

```
graph: 73 stale projects, 0 taken from the fleet, 1 too heavy for this machine (@semio-tech/repo-lib (about 3838 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 73
```

- `taken from the fleet` / `inferred here` lines:

```
graph: 73 stale projects, 0 taken from the fleet, 1 too heavy for this machine (@semio-tech/repo-lib (about 3838 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 73
```

- `too heavy` lines:

```
graph: 73 stale projects, 0 taken from the fleet, 1 too heavy for this machine (@semio-tech/repo-lib (about 3838 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 73
graph: the fleet did not deliver 1 too heavy project(s) (@semio-tech/repo-lib (about 3838 MB)); this machine infers them itself, one at a time and alone after the others, each with up to about 8428 MB
graph: project @semio-tech/mit-bestand-demonstrator (project) grew by 2265 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 178 MB, resident 959 MB)
```

- `waiting for the fleet` lines:

```
graph: waiting for the fleet: 0 of 73 projects arrived
```

- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (768 of 1024 MB reserved, 1 projects running): @semio-tech/writer-js needs 768 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (768 of 1059 MB reserved, 1 projects running): @semio-tech/writer-js needs 768 MB and has waited 6 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 0 of 4774 projects · 1 running, longest 5 s: @semio-tech/mit-bestand-demonstrator (project, 5 s, 0 rows) · waiting for memory headroom (768 of 1059 MB reserved)
… 7 more distinct lines left out …
[script] build:projects · Inferring projects: 1 of 4774 projects · 1 running, longest 25 s: @semio-tech/mit-bestand-demonstrator (project, 25 s, 66 rows) · waiting for memory headroom (824 of 505 MB reserved)
graph: waiting for memory headroom (1259 of 0 MB reserved, 1 projects running): @semio-tech/mathematical-js needs 795 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 2 of 4774 projects · 1 running, longest 0 s: @semio-tech/writer-writer (language:typescript, 0 s, 0 rows) · waiting for memory headroom (1259 of 0 MB reserved)
```

- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T05:53:24.084Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=69174 ppid=69056
```

- `closure-missing` lines:

```
[graph] closure-missing 1d57e93cd7c7f9bf6d208090c29c5fd39729e025e972a4365acd21db81bfe35f named by 1 projects: asked of the fleet
[graph] closure-missing 3ff861698b3ceb8ef6cd534d0ffd6e42444cd129bf35c5b7895390b9c037229d named by 1 projects: asked of the fleet
[graph] closure-missing 5b277c5ea90843287799ff92475b362358a6c1f7fc85930597142cad5a0a6032 named by 2 projects: asked of the fleet
[graph] closure-missing 64e5e8471fbdbe0988cf801cd93558955fbb7ddabe212d896454816b4545a1e3 named by 1 projects: re-derived
… 3 more distinct lines left out …
[graph] closure-missing a32e8e410630d55255504482814fd86de14118524a0dcbeb5b8c2db68f2a9e5b named by 70 projects: re-derived
[graph] closure-missing a7a185932fb1e5ed293e21667be5dae7f1ec4150b7688c5b06a0d962d1e04037 named by 1 projects: re-derived
[graph] closure-missing c7cdc4f3baf46a70323920cdaf8c04f1278a9ece901f123785d92179a68ebd21 named by 1 projects: asked of the fleet
[graph] closure-missing dff3e6dcfa306547c5e1eba2a2c8c2a58809297e4e9d74e1a4954ebc505462ca named by 1 projects: asked of the fleet
```

- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4314 MB because of its plan phase
[capacity] waiting 0s for run: the host is swapping out too fast (need 4314 MB; -4975 MB spare above the 2048 MB floor, load 20.84; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 6s for run: swap use is above its ceiling (need 4314 MB; -5448 MB spare above the 2048 MB floor, load 20.18; 2 running, 0 ahead); behind the host itself (nothing of ours)
… 67 more distinct lines left out …
[capacity] admitted dependencies with 3196 of 5298 MB: only 3196 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 512 MB reserved (6815 MB available, 2048 MB floor)
```

### Try 6: native_rc=1

- Start 2026-11-15T06:14:29Z, end 2026-11-15T06:49:48Z, wall time 2119 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-6.log` (912 lines cleaned); service log(s): host-d71e909e3d92.log from line 187, host-d7505b87cee1.log.
- Pressure samples by level (30 s): {"1": 34, "2": 35, "4": 1}. Largest process of the run in the 30 s samples: 1007 MB resident.
- Slice worker sampler (5 s): peak 5527 MB resident at this line:

```
2026-11-15T06:25:12Z pid=75042 rss=5527MB footprint=[bun [75042]: 64-bit    Footprint: 5067 MB (16384 bytes per page)] last=[graph: waiting for the graph build of a peer (220 s in the queue, generation -1 was the newest when this command queued)]
```

- closure entries before: 41882
- closure entries after: 41895
- Last message of the bootstrap:

```
The plan cannot continue: the slice worker ended 4 times in a row with exit code 75 because the code on this machine kept changing under it; try again once the replica has settled.
```

- Restore line: none
- First plan line (`per project`):

```
graph: 73 stale projects, 0 taken from the fleet, 1 too heavy for this machine (@semio-tech/repo-lib (about 3830 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 73
```

- `taken from the fleet` / `inferred here` lines:

```
graph: 73 stale projects, 0 taken from the fleet, 1 too heavy for this machine (@semio-tech/repo-lib (about 3830 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 73
```

- `too heavy` lines:

```
graph: 73 stale projects, 0 taken from the fleet, 1 too heavy for this machine (@semio-tech/repo-lib (about 3830 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 73
graph: the fleet did not deliver 1 too heavy project(s) (@semio-tech/repo-lib (about 3830 MB)); this machine infers them itself, one at a time and alone after the others, each with up to about 8428 MB
graph: project @semio-tech/mathematical-equation-rs (language:rust) grew by 3074 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
… 1 more distinct lines left out …
graph: project @semio-tech/wfc-bitmap (language:typescript) grew by 2048 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/wfc-3d-rs (language:rust) grew by 3623 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/wfc-3d (language:typescript) grew by 1943 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 185 MB, resident 407 MB)
graph: plan stage inference settled from 6835 to 6993 MB
```

- `waiting for the fleet` lines:

```
graph: waiting for the fleet: 0 of 73 projects arrived
```

- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (795 of 717 MB reserved, 1 projects running): @semio-tech/writer-writer needs 1259 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (1259 of 827 MB reserved, 1 projects running): @semio-tech/mathematical-js needs 795 MB and has waited 5 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (1846 of 814 MB reserved, 1 projects running): @semio-tech/mathematical-js needs 795 MB and has waited 14 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
… 85 more distinct lines left out …
[script] build:projects · Inferring projects: 16 of 4774 projects · 1 running, longest 10 s: @semio-tech/wfc-grid3d (language:typescript, 10 s, 0 rows) · waiting for memory headroom (1846 of 0 MB reserved)
graph: waiting for memory headroom (1846 of 0 MB reserved, 1 projects running): @semio-tech/procedural-js needs 966 MB and has waited 21 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 16 of 4774 projects · 1 running, longest 20 s: @semio-tech/wfc-grid3d (language:typescript, 20 s, 0 rows) · waiting for memory headroom (1846 of 0 MB reserved)
```

- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T06:21:20.474Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=launchd pid=95045 ppid=94334
```

- `closure-missing` lines:

```
[graph] closure-missing 7005d48ed16b9ab1293c05ca9cfcac2f953d6dc9beef4cfa16a826394e4b3537 named by 1 projects: asked of the fleet
[graph] closure-missing 7143a7deaa3ba919893aefa5354df74795ac63f70f9e90262dc144994f2965bb named by 70 projects: re-derived
[graph] closure-missing a32e8e410630d55255504482814fd86de14118524a0dcbeb5b8c2db68f2a9e5b named by 70 projects: re-derived
[graph] closure-missing a7a185932fb1e5ed293e21667be5dae7f1ec4150b7688c5b06a0d962d1e04037 named by 1 projects: re-derived
[graph] closure-missing c7cdc4f3baf46a70323920cdaf8c04f1278a9ece901f123785d92179a68ebd21 named by 1 projects: asked of the fleet
[graph] closure-missing dff3e6dcfa306547c5e1eba2a2c8c2a58809297e4e9d74e1a4954ebc505462ca named by 1 projects: asked of the fleet
```

- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4311 MB because of its plan phase
[capacity] admitted run: 8 job(s), 5256 MB reserved (8407 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
… 1 more distinct lines left out …
[capacity] admitted dependencies: 1 job(s), 5300 MB reserved (8217 MB available, 2048 MB floor)
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 512 MB reserved (6513 MB available, 2048 MB floor)
```

### Try 7: native_rc=1

- Start 2026-11-15T06:52:51Z, end 2026-11-15T07:02:59Z, wall time 608 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-7.log` (939 lines cleaned); service log(s): host-d7505b87cee1.log from line 2430, host-88aaa68ccf96.log.
- Pressure samples by level (30 s): {"1": 11, "2": 8, "4": 1}. Largest process of the run in the 30 s samples: 3916 MB resident.
- Slice worker sampler (5 s): peak 4230 MB resident at this line:

```
2026-11-15T06:58:43Z pid=67321 rss=4230MB footprint=[bun [67321]: 64-bit    Footprint: 4808 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (1092 of 0 MB reserved, 1 projects running): @semio-tech/mit-bestand-demonstrator needs 966 MB and has waited 5 s; after 60 s it runs alone if it fits the 1756 MB one project is granted]
```

- closure entries before: 41895
- closure entries after: 41898
- Last message of the bootstrap:

```
The graph service was denied access to the checkout although this process can read it, so this is not a missing file access consent of this session; a service that was started for another route or another app is judged against another identity. The service ended and was started once more under the access of this process; the second one was denied as well. macOS does not let `repo` read your Documents folder: /Users/ueli/Documents/semio is refused, so /Users/ueli/Documents/semio cannot be read. Open System Settings, Privacy & Security, Files & Folders, switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) (or give it Full Disk Access) or click "Allow" on the dialog "repo would like to access files in your Documents folder", then run the command again. The consent is asked once per machine; a checkout outside Documents, Desktop and Downloads needs none.
```

- Restore line: none
- First plan line (`per project`): none
- `taken from the fleet` / `inferred here` lines: none
- `too heavy` lines:

```
graph: project @semio-tech/mit-bestand-praesentation-projektetage (project) grew by 2016 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/mit-bestand-bericht (project) grew by 4217 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 133 MB, resident 322 MB)
graph: plan stage inference settled from 6613 to 6990 MB
```

- `waiting for the fleet` lines: none
- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (966 of 729 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 966 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (1457 of 842 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 966 MB and has waited 7 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 0 of 4774 projects · 1 running, longest 5 s: @semio-tech/mit-bestand-praesentation-projektetage (project, 5 s, 51 rows) · waiting for memory headroom (1457 of 842 MB reserved)
… 11 more distinct lines left out …
graph: waiting for memory headroom (4217 of 2964 MB reserved, 1 projects running): @semio-tech/mit-bestand-demonstrator needs 966 MB and has waited 55 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 1 of 4774 projects · 1 running, longest 55 s: @semio-tech/mit-bestand-bericht (project, 55 s, 63 rows) · waiting for memory headroom (4217 of 2964 MB reserved)
graph: waiting for memory headroom (4217 of 1898 MB reserved, 1 projects running): @semio-tech/mit-bestand-demonstrator needs 966 MB and has waited 60 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
```

- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T06:54:04.639Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=95045 ppid=94334
2026-11-15T06:54:14.979Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=62389 ppid=62314
2026-11-15T07:02:40.887Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=76305 ppid=76186
2026-11-15T07:02:44.062Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=direct pid=76305 ppid=76186
2026-11-15T07:02:52.195Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=76582 ppid=76575
```

- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4307 MB because of its plan phase
[capacity] waiting 0s for run: not enough free memory (need 4307 MB; 1832 MB spare above the 2048 MB floor, load 32.42; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 2048 MB), @teaching/staffage-react:test (quiet, 1 job(s), 51 MB)
[capacity] waiting 5s for run: not enough free memory (need 4307 MB; 2364 MB spare above the 2048 MB floor, load 31.5; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 2048 MB), @teaching/staffage-react:test (quiet, 1 job(s), 51 MB)
… 12 more distinct lines left out …
[capacity] admitted dependencies with 3361 of 5306 MB: only 3361 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 512 MB reserved (8242 MB available, 2048 MB floor)
```

### Try 8: native_rc=75

- Start 2026-11-15T07:06:02Z, end 2026-11-15T07:37:50Z, wall time 1908 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-8.log` (1682 lines cleaned); service log(s): host-88aaa68ccf96.log from line 322, host-4739b686d947.log, host-4a3e468900f5.log.
- Pressure samples by level (30 s): {"1": 48, "2": 15}. Largest process of the run in the 30 s samples: 8060 MB resident.
- Slice worker sampler (5 s): peak 8313 MB resident at this line:

```
2026-11-15T07:24:26Z pid=438 rss=8313MB footprint=[bun [438]: 64-bit    Footprint: 3641 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (5158 of 362 MB reserved, 1 projects running): @semio-tech/flow-extension-brep-rust needs 675 MB and has waited 6 s; after 60 s it runs alone if it fits the 1756 MB one project is grant]
```

- closure entries before: 41898
- closure entries after: 41995
- Last message of the bootstrap:

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 9954 MB, more than the 6553 MB this machine gives a plan, and only 2854 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Restore line: none
- First plan line (`per project`): none
- `taken from the fleet` / `inferred here` lines:

```
graph: 19 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 31 inferred here
graph: 19 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 81 inferred here
```

- `too heavy` lines:

```
graph: project @semio-tech/mit-bestand-demonstrator (project) grew by 2890 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/writer-writer-rs (language:rust) grew by 1919 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/mathematical-js (project) grew by 1871 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
… 26 more distinct lines left out …
graph: project @semio-tech/norm-din16798-rs (language:rust) grew by 5074 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/norm-iso16757-rs (language:rust) grew by 2311 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/norm-en1993 (language:typescript) grew by 2852 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 196 MB, resident 278 MB)
graph: plan stage inference settled from 7048 to 6682 MB
graph: plan stage inference starts (managed 200 MB, resident 335 MB)
graph: plan stage inference settled from 6610 to 7288 MB
graph: plan stage inference starts (managed 207 MB, resident 947 MB)
graph: plan stage inference (during inference) grew the plan by 5054 MB, more than the 4915 MB this machine allows for it; it is stopped before the host runs short of memory
graph: plan stage inference settled from 7130 to 5032 MB
```

- `waiting for the fleet` lines: none
- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (966 of 741 MB reserved, 1 projects running): @semio-tech/writer-js needs 966 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (966 of 765 MB reserved, 1 projects running): @semio-tech/writer-js needs 966 MB and has waited 5 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 0 of 4774 projects · 1 running, longest 5 s: @semio-tech/mit-bestand-demonstrator (project, 5 s, 0 rows) · waiting for memory headroom (966 of 765 MB reserved)
… 240 more distinct lines left out …
graph: waiting for memory headroom (3567 of 1137 MB reserved, 1 projects running): @semio-tech/norm-en1993 needs 1575 MB and has waited 5 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 101 of 4774 projects · 2 running, longest 60 s: @semio-tech/norm-en1992-rs (language:rust, 60 s, 0 rows), @semio-tech/norm-iso16757-rs (language:rust, 25 s, 0 rows) · waiting for memory headroom (3567 of 1137 MB reserved)
graph: waiting for memory headroom (5142 of 2162 MB reserved, 2 projects running): @semio-tech/norm-en1993-rs needs 3567 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
```

- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T07:09:20.590Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=85354 ppid=85145
2026-11-15T07:20:18.307Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=328 ppid=297
```

- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4305 MB because of its plan phase
[capacity] waiting 0s for run: not enough free memory (need 4305 MB; 4107 MB spare above the 2048 MB floor, load 33.4; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 5s for run: not enough free memory (need 4305 MB; 3274 MB spare above the 2048 MB floor, load 32.65; 0 running, 0 ahead); behind the host itself (nothing of ours)
… 16 more distinct lines left out …
[capacity] run: work has to run, waiting 23s to grow the reservation from 512 MB to 14336 MB: not enough free memory (1724 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 10365 MB)
[capacity] run: work has to run, waiting 29s to grow the reservation from 512 MB to 14336 MB: not enough free memory (1724 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 10365 MB)
[capacity] run: reservation grown from 512 MB to 5750 MB with 1 of 8 job(s) after 30s: the host cannot carry more right now (5928 MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
```

### Try 9: native_rc=75

- Start 2026-11-15T07:40:52Z, end 2026-11-15T07:49:30Z, wall time 518 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-9.log` (823 lines cleaned); service log(s): host-4a3e468900f5.log from line 3181, host-74145656f35a.log.
- Pressure samples by level (30 s): {"1": 16, "2": 1}. Largest process of the run in the 30 s samples: 7397 MB resident.
- Slice worker sampler (5 s): peak 7328 MB resident at this line:

```
2026-11-15T07:42:53Z pid=19578 rss=7328MB footprint=[bun [19578]: 64-bit    Footprint: 2318 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (3514 of 1043 MB reserved, 1 projects running): @semio-tech/fem-js needs 3392 MB and has waited 13 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise t]
```

- closure entries before: 41995
- closure entries after: 42020
- Last message of the bootstrap:

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 10058 MB, more than the 6553 MB this machine gives a plan, and only 2743 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Restore line: none
- First plan line (`per project`): none
- `taken from the fleet` / `inferred here` lines: none
- `too heavy` lines:

```
graph: project @semio-tech/fixture-generator-equation-1-any (project) grew by 2150 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/norm-en1994 (language:typescript) grew by 2366 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/norm-en1998 (language:typescript) grew by 1970 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/playbook-extension-procedural-rust (language:cargo) grew by 2518 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/imperative-procedure (language:typescript) grew by 1947 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/imperative-procedure-rs (language:rust) grew by 2161 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 202 MB, resident 1010 MB)
graph: plan stage inference settled from 7051 to 7500 MB
graph: plan stage inference (during inference) grew the plan by 5775 MB, more than the 4915 MB this machine allows for it; it is stopped before the host runs short of memory
```

- `waiting for the fleet` lines: none
- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (3614 of 844 MB reserved, 1 projects running): @semio-tech/writer-writer needs 1575 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (3567 of 430 MB reserved, 1 projects running): @semio-tech/fixture-generator-equation-1-any needs 3614 MB and has waited 3 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (3614 of 308 MB reserved, 1 projects running): @semio-tech/wfc-js needs 3614 MB and has waited 3 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
… 73 more distinct lines left out …
[script] build:projects · Inferring projects: 75 of 4774 projects · 6 running, longest 25 s: @semio-tech/imperative-js (project, 25 s, 19 rows), @semio-tech/imperative-procedure (language:typescript, 25 s, 19 rows), @semio-tech/imperative-procedure-rs (language:rust, 20 s, 27 rows) · waiting for memory headroom (15435 of 933 MB reserved)
graph: waiting for memory headroom (15435 of 933 MB reserved, 6 projects running): @semio-tech/imperative-extension-math-rust needs 1708 MB and has waited 23 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 75 of 4774 projects · 6 running, longest 35 s: @semio-tech/imperative-js (project, 35 s, 19 rows), @semio-tech/imperative-procedure (language:typescript, 35 s, 19 rows), @semio-tech/imperative-procedure-rs (language:rust, 30 s, 27 rows) · waiting for memory headroom (15435 of 933 MB reserved)
```

- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T07:41:19.039Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=19473 ppid=19461 responsible=46526
```

- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4301 MB because of its plan phase
[capacity] admitted run: 8 job(s), 5246 MB reserved (9029 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
… 9 more distinct lines left out …
[capacity] run: work has to run, waiting 27s to grow the reservation from 512 MB to 14336 MB: not enough free memory (-127 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 10664 MB)
[capacity] run: work has to run, waiting 33s to grow the reservation from 512 MB to 5750 MB: not enough free memory (812 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 10664 MB)
[capacity] run: reservation grown from 512 MB to 5750 MB with 1 of 8 job(s) after 36s: the host cannot carry more right now (6017 MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
```

### Try 10: native_rc=75

- Start 2026-11-15T07:52:32Z, end 2026-11-15T07:55:09Z, wall time 157 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-10.log` (624 lines cleaned); service log(s): host-74145656f35a.log from line 2395.
- Pressure samples by level (30 s): {"1": 3, "2": 2}. Largest process of the run in the 30 s samples: 121 MB resident.
- Slice worker sampler (5 s): peak 2226 MB resident at this line:

```
2026-11-15T07:54:40Z pid=28706 rss=2226MB footprint=[bun [28706]: 64-bit    Footprint: 6350 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (3747 of 0 MB reserved, 1 projects running): @semio-tech/wfc-grid2d-rs needs 3747 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwis]
```

- closure entries before: 42020
- closure entries after: 42020
- Last message of the bootstrap:

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 6729 MB, more than the 6553 MB this machine gives a plan, and only 3352 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Restore line: none
- First plan line (`per project`): none
- `taken from the fleet` / `inferred here` lines: none
- `too heavy` lines:

```
graph: project @semio-tech/wfc-2d (language:typescript) grew by 2218 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 208 MB, resident 401 MB)
graph: plan stage inference settled from 6563 to 6386 MB
```

- `waiting for the fleet` lines: none
- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (2931 of 963 MB reserved, 1 projects running): @semio-tech/writer-writer needs 3634 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (3747 of 948 MB reserved, 1 projects running): @semio-tech/wfc-2d needs 3634 MB and has waited 1 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (3634 of 626 MB reserved, 1 projects running): @semio-tech/wfc-2d-rs needs 3747 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
… 1 more distinct lines left out …
[script] build:projects · Inferring projects: 3 of 4774 projects · 1 running, longest 5 s: @semio-tech/wfc-2d (language:typescript, 5 s, 0 rows) · waiting for memory headroom (3634 of 623 MB reserved)
graph: waiting for memory headroom (3747 of 0 MB reserved, 1 projects running): @semio-tech/wfc-grid2d-rs needs 3747 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 4 of 4774 projects · 1 running, longest 0 s: @semio-tech/wfc-2d-rs (language:rust, 0 s, 0 rows) · waiting for memory headroom (3747 of 0 MB reserved)
```

- `access-proof`, `where:`, EPERM and refusal lines: none
- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4300 MB because of its plan phase
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 33% of the default from 15 earlier run(s) of its class, at least 4300 MB because of its plan phase
… 2 more distinct lines left out …
[capacity] admitted dependencies with 4839 of 5314 MB: only 4839 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 102% of the default from 5 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 512 MB reserved (6876 MB available, 2048 MB floor)
```

### Try 11: native_rc=75

- Start 2026-11-15T07:58:11Z, end 2026-11-15T08:04:45Z, wall time 394 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-11.log` (767 lines cleaned); service log(s): host-74145656f35a.log from line 3912, host-25cab2808032.log.
- Pressure samples by level (30 s): {"1": 13}. Largest process of the run in the 30 s samples: 7651 MB resident.
- Slice worker sampler (5 s): peak 8106 MB resident at this line:

```
2026-11-15T08:03:05Z pid=32106 rss=8106MB footprint=[bun [32106]: 64-bit    Footprint: 3085 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (3762 of 0 MB reserved, 1 projects running): @semio-tech/energy-model-state needs 3762 MB and has waited 11 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, ot]
```

- closure entries before: 42020
- closure entries after: 42033
- Last message of the bootstrap:

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 8808 MB, more than the 6553 MB this machine gives a plan, and only 2294 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Restore line: none
- First plan line (`per project`): none
- `taken from the fleet` / `inferred here` lines: none
- `too heavy` lines:

```
graph: project @semio-tech/imperative-extension-control-rust (language:cargo) grew by 1926 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/energy-js (project) grew by 2595 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/trinity-js (project) grew by 2580 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/trinity-rewriting (language:typescript) grew by 2592 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 202 MB, resident 990 MB)
graph: plan stage inference (during inference) grew the plan by 4966 MB, more than the 4915 MB this machine allows for it; it is stopped before the host runs short of memory
graph: plan stage inference settled from 6588 to 6861 MB
```

- `waiting for the fleet` lines: none
- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (2931 of 270 MB reserved, 1 projects running): @semio-tech/writer-writer needs 3634 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (3747 of 426 MB reserved, 1 projects running): @semio-tech/wfc-2d-rs needs 3747 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 2 of 4774 projects · 1 running, longest 0 s: @semio-tech/writer-writer-rs (language:rust, 0 s, 0 rows) · waiting for memory headroom (3747 of 426 MB reserved)
… 55 more distinct lines left out …
graph: waiting for memory headroom (2653 of 0 MB reserved, 1 projects running): @semio-tech/trinity-rewriting-rs needs 737 MB and has waited 4 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (1450 of 0 MB reserved, 1 projects running): @semio-tech/trinity-jack-shell needs 3884 MB and has waited 5 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 76 of 4774 projects · 1 running, longest 5 s: @semio-tech/trinity-rewriting-rs (language:rust, 5 s, 0 rows) · waiting for memory headroom (1450 of 0 MB reserved)
```

- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T07:58:35.870Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=32017 ppid=31989 responsible=46526
```

- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4299 MB because of its plan phase
[capacity] admitted run: 8 job(s), 5244 MB reserved (9829 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
… 11 more distinct lines left out …
[capacity] run: work has to run, waiting 38s to grow the reservation from 512 MB to 5750 MB: not enough free memory (4313 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 4015 MB)
[capacity] run: work has to run, waiting 44s to grow the reservation from 512 MB to 5750 MB: not enough free memory (3984 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 4015 MB)
[capacity] run: reservation grown from 512 MB to 7841 MB with 2 of 8 job(s) after 46s: the host cannot carry more right now (5960 MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
```

### Try 12: native_rc=75

- Start 2026-11-15T08:07:47Z, end 2026-11-15T08:39:31Z, wall time 1904 s. Hard stop: none. Failed setup tasks: none (the setup tasks were not reached). Reference: none.
- Bootstrap log `~/native-bootstrap-round-17-try-12.log` (1560 lines cleaned); service log(s): host-25cab2808032.log from line 2300, host-8dabac9314c5.log.
- Pressure samples by level (30 s): {"1": 63}. Largest process of the run in the 30 s samples: 8037 MB resident.
- Slice worker sampler (5 s): peak 8175 MB resident at this line:

```
2026-11-15T08:36:50Z pid=39433 rss=8175MB footprint=[bun [39433]: 64-bit    Footprint: 2682 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (4453 of 0 MB reserved, 1 projects running): @semio-tech/sourcing-curation-rs needs 5237 MB and has waited 17 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, ]
```

- closure entries before: 42033
- closure entries after: 42177
- Last message of the bootstrap:

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 10035 MB, more than the 6553 MB this machine gives a plan, and only 4100 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Restore line: none
- First plan line (`per project`): none
- `taken from the fleet` / `inferred here` lines:

```
graph: 0 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 50 inferred here
graph: 0 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 100 inferred here
graph: 0 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 150 inferred here
```

- `too heavy` lines:

```
graph: project @semio-tech/wfc-grid2d-rs (language:rust) grew by 2721 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/trinity-rewriting-rs (language:rust) grew by 2201 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/trinity-jack (language:typescript) grew by 3391 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
… 33 more distinct lines left out …
graph: project @semio-tech/s-flow-composition-rs (language:cargo) grew by 3009 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/s-fem-composition-tests (project) grew by 2624 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/s-forms-composition-tests (project) grew by 1906 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
```

- Plan stage lines:

```
graph: plan stage inference starts (managed 208 MB, resident 1003 MB)
graph: plan stage inference (during inference) grew the plan by 4988 MB, more than the 4915 MB this machine allows for it; it is stopped before the host runs short of memory
graph: plan stage inference settled from 7357 to 7989 MB
graph: plan stage inference starts (managed 204 MB, resident 1018 MB)
graph: plan stage inference (during inference) grew the plan by 4956 MB, more than the 4915 MB this machine allows for it; it is stopped before the host runs short of memory
graph: plan stage inference settled from 6910 to 7939 MB
```

- `waiting for the fleet` lines: none
- `waiting for memory headroom` lines:

```
graph: waiting for memory headroom (4806 of 684 MB reserved, 1 projects running): @semio-tech/writer-writer needs 1152 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (828 of 513 MB reserved, 1 projects running): @semio-tech/wfc-grid2d-rs needs 1430 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: waiting for memory headroom (2002 of 305 MB reserved, 1 projects running): @semio-tech/wfc-bitmap-rs needs 737 MB and has waited 3 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
… 347 more distinct lines left out …
[script] build:projects · Inferring projects: 146 of 4774 projects · 1 running, longest 5 s: @semio-tech/s-forms-composition-tests (project, 5 s, 0 rows) · waiting for memory headroom (1561 of 0 MB reserved)
graph: waiting for memory headroom (911 of 0 MB reserved, 1 projects running): @semio-tech/cad-js needs 169 MB and has waited 2 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
[script] build:projects · Inferring projects: 147 of 4774 projects · 1 running, longest 0 s: @semio-tech/s-dev-composition (project, 0 s, 0 rows) · waiting for memory headroom (911 of 0 MB reserved)
```

- `access-proof`, `where:`, EPERM and refusal lines:

```
2026-11-15T08:08:10.817Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=39352 ppid=39317 responsible=46526
```

- `closure-missing` lines: none
- `[capacity]` lines of the bootstrap log:

```
[capacity] run: sized at 7% of the default from 16 earlier run(s) of its class, at least 4298 MB because of its plan phase
[capacity] admitted run: 8 job(s), 5243 MB reserved (9442 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
… 9 more distinct lines left out …
[capacity] run: work has to run, waiting 28s to grow the reservation from 512 MB to 14336 MB: not enough free memory (3358 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 2048 MB)
[capacity] run: work has to run, waiting 33s to grow the reservation from 512 MB to 5750 MB: not enough free memory (2687 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 2048 MB)
[capacity] run: reservation grown from 512 MB to 5750 MB with 1 of 8 job(s) after 36s: the host cannot carry more right now (6212 MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
```

