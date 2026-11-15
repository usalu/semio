# g1-m02 Rounds 14, 15 and 16

round: 16

One report for rounds 14, 15 and 16, under the file names `round-14…` as ROUND.md says. Result: not ready. Four tries, no nx task ran in any (succeeded: none, failed: none, not run: all). Step 0 of round 16 (`repo tools setup all`) ended with exit code 1 after 1554 s. `.cache/f/native-bootstrap.done` does not exist. Round 17 was published while step 0 ran; its fresh series is reported in `round-17.md`.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk: not measured in these rounds.

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Largest process on the machine | P12 peak of the slice worker | How it ended |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-14.log` | 02:12:39 | 02:21:12 | 7 | 3815504 KiB | rss 3763 MB, footprint 195 MB | my runner's SIGINT when round 15 appeared (against round 15, see below) |
| 2 | `…-round-14-try-2.log` | 02:22:18 | 02:38:52 | 75 | 3316240 KiB | rss 3238 MB, footprint 427 MB | fleet refusal: asked for 521, delivered 0 within 901 s |
| 3 | `…-round-14-try-3.log` | 02:42:04 | 02:59:44 | 75 | 2731520 KiB | rss 2667 MB, footprint 418 MB | fleet refusal: asked for 530, delivered 0 within 902 s |
| 4 | `…-round-14-try-4.log` | 03:03:20 | 03:20:29 | 7 | 3080080 KiB | rss 3007 MB, footprint 293 MB | `waiting for the fleet: 0 of 530 projects arrived` unchanged for 908 s; one SIGINT (stall rule) |

- Pressure level 1 in all 122 samples (30 s apart); no hard stop; level 2 never occurred.
- Try 1, restore line: A = 4 min, S = 0 (`from a state 4 min old with 0 projects stale`), then `refreshing 569 projects (generation 217)`. All ten plan stages have a `took` line: `inference` 27.7 s (+523 MB), `environment` 7.9 s (+57 MB), `edges` 1 s (+38 MB), the others 0 s. Then `0 projects taken from the fleet, 50 inferred here` and a second `inference` stage started.
- Tries 2 to 4: no restore line and no plan stage line in the try's part of the service log. First plan line says `it can give about 0 MB per project` with 94 and 96 projects too heavy (verbatim in the tries below); `vm_stat` was not taken then (round 17 asks for it; see `round-17.md`).
- Try 4: my stall SIGINT came at 908 s, a few seconds after the service's own limit of about 901 s would have ended the wait with a refusal; the fleet had delivered 0 either way.
- Closure entries and the full sampler extracts are in the tries below. Launcher unchanged in all tries (inode 3679486, `CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357`).

## Before the Run (2026-11-15T02:13Z)

- Replica agent: `state = running`, `runs = 24` (23 in round 13); started by me 0 times in this round.
- Replica code digest (`.cache/f/health.json`): `d399114215f574cd93e191090e959fc4f901cfe0f2cd44963ab8d2b8d4046284`; every origin in the replica `status` is on `d399114215f5`.
- `bootVersion`: 6
- Agreement: the replica `status` did NOT report agreement at 02:10:50Z and again at 02:12:24Z. This machine: `0 events (0 B) behind`, origins behind/queued `0/0`, but verdict `diverged`, verbatim: `g1-m02 differs in view.digest at a common checkpoint: #153=5d46 instead of #153=9aed (#n = bucket n of 256 of the paths, value = its digest)`. The same bucket at both readings, 93 s after the last event, so it is not a file in flight. g1-m03 and g1-m04 were also `diverged`, five origins `lagging`; last line: `The fleet is not in agreement (10 origins; …)`. I cannot repair a divergence (no change outside my folder), so I started try 1 on the same code digest as the fleet and report this instead of waiting further.
- Try 1 was interrupted against round 15's instruction: my runner still had the rule of the earlier rounds ("a newer round and no task started: one SIGINT") and sent it at 02:21:12Z, before I had read "Do NOT interrupt a try that is running". The try was healthy then (plan stage `environment` took 7.9 s, 50 projects inferred here, waiting for the fleet). From try 2 on (start 02:22:18Z) the runner follows round 15: level 2 is recorded and no stop, and a newer round never interrupts a running try.

## Step 0 of Round 16: `repo tools setup all`

- Start 2026-11-15T03:21:32Z, end 2026-11-15T03:47:26Z, wall time 1554 s, exit code 1
- Log `~/tools-setup-round-16.log` (1316 lines without `[script] {` rows)
- Memory pressure, samples 10 s apart: level 1 in 155

No `Another process holds the dependency store` wait of more than 2 minutes; no SIGINT. Last 15 lines (without `[script]` rows and `still running` lines):

```
[repo] still working after 1160 s; last phase started: source.served
[repo] still working after 1190 s; last phase started: source.served
[repo] still working after 1220 s; last phase started: source.served
[repo] still working after 1250 s; last phase started: source.served
[repo] still working after 1280 s; last phase started: source.served
[repo] still working after 1310 s; last phase started: source.served
[repo] still working after 1340 s; last phase started: source.served
[repo] still working after 1370 s; last phase started: source.served
[repo] still working after 1400 s; last phase started: source.served
[repo] still working after 1430 s; last phase started: source.served
[repo] still working after 1460 s; last phase started: source.served
[repo] still working after 1490 s; last phase started: source.served
[repo] still working after 1520 s; last phase started: source.served
Internal error: Complete acquired dependency retirement plan required
repository:command failed with exit code 1
```

## Try 1

- Log `/Users/ueli/native-bootstrap-round-14.log`: `start 2026-11-15T02:12:39Z`, `end 2026-11-15T02:21:12Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=3815504 run_peak_rss_kib=460144 stop=round 15 was published and no task has started fleet_refusal=False closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T02:16:58Z pid=66234 rss=3763MB footprint=[bun [66234]: 64-bit    Footprint: 195 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 46 of 703 projects arrived]`

Key lines of the graph service log, verbatim, in order:

```
graph: generation 216 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 11 s, from a state 4 min old with 0 projects stale
graph: refreshing 569 projects (generation 217)
graph: plan stage inference starts (managed 1116 MB, resident 3425 MB)
graph: plan stage inference took 27.7 s (managed 1639 MB, 523 MB more than at its start, resident 3761 MB)
graph: plan stage environment starts (managed 1639 MB, resident 3761 MB)
graph: plan stage environment took 7.9 s (managed 1696 MB, 57 MB more than at its start, resident 1869 MB)
graph: plan stage tests starts (managed 1696 MB, resident 1869 MB)
graph: plan stage tests took 0 s (managed 1696 MB, 0 MB more than at its start, resident 1870 MB)
graph: plan stage policies starts (managed 1696 MB, resident 1870 MB)
graph: plan stage policies took 0 s (managed 1696 MB, 0 MB more than at its start, resident 1873 MB)
graph: plan stage dangling starts (managed 1696 MB, resident 1873 MB)
graph: plan stage dangling took 0 s (managed 1696 MB, 0 MB more than at its start, resident 1875 MB)
graph: plan stage files starts (managed 1696 MB, resident 1875 MB)
graph: plan stage files took 0 s (managed 1696 MB, 0 MB more than at its start, resident 1880 MB)
graph: plan stage test-edges starts (managed 1696 MB, resident 1880 MB)
graph: plan stage test-edges took 0 s (managed 1696 MB, 0 MB more than at its start, resident 1880 MB)
graph: plan stage edges starts (managed 1696 MB, resident 1880 MB)
graph: plan stage edges took 1 s (managed 1734 MB, 38 MB more than at its start, resident 1930 MB)
graph: plan stage metadata starts (managed 1734 MB, resident 1930 MB)
graph: plan stage metadata took 0 s (managed 1734 MB, 0 MB more than at its start, resident 1934 MB)
graph: plan stage result starts (managed 1734 MB, resident 1934 MB)
graph: plan stage result took 0 s (managed 1734 MB, 0 MB more than at its start, resident 1935 MB)
graph: plan stage inference starts (managed 1900 MB, resident 3290 MB)
graph: 0 projects taken from the fleet, 50 inferred here
```

P12 sampler `round-14.p12-try-1.txt` (94 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-007c265f8fa9.log
closure entries before: 42537
2026-11-15T02:12:44Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T02:21:38Z no slice worker
closure entries after: 42537
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
Reading the project graph slice (403 s): owners 108 of 569.
Reading the project graph slice (405 s): owners 110 of 569.
Reading the project graph slice (407 s): owners 110 of 569.
Reading the project graph slice (409 s): owners 113 of 569.
Reading the project graph slice (411 s): owners 114 of 569; running (name, kind, seconds, rows): @semio-tech/framework-sqlite-snapshot-rs (language:cargo, 3, 0); waiting for memory headroom (931 of 648 MB reserved).
Reading the project graph slice (413 s): owners 114 of 569; running (name, kind, seconds, rows): @semio-tech/framework-sqlite-snapshot-rs (language:cargo, 5, 0), @semio-tech/manifest-media-rs (language:rust, 2, 0), @semio-tech/manifest-mutation-inputs-rs (language:rust, 2, 0).
Reading the project graph slice (415 s): owners 118 of 569.
Reading the project graph slice (417 s): owners 119 of 569.
Reading the project graph slice (419 s): owners 122 of 569; running (name, kind, seconds, rows): @semio-tech/manifest-app-surface-rs (language:rust, 6, 0), @semio-tech/manifest-plugin-rs (language:rust, 6, 0), @semio-tech/framework-tool-machine-rs (language:cargo, 6, 0).
Reading the project graph slice (421 s): owners 126 of 569.
Reading the project graph slice (423 s): owners 128 of 569.
Reading the project graph slice (425 s): owners 132 of 569.
Reading the project graph slice (427 s): owners 135 of 569.
Reading the project graph slice (429 s): owners 138 of 569; running (name, kind, seconds, rows): @semio-tech/job-return-escrow-publication-rs (language:rust, 1, 0), @semio-tech/job-return-budget-rs (language:rust, 0, 0), @semio-tech/job-clock-rs (language:rust, 0, 0).
Reading the project graph slice (431 s): owners 140 of 569.
[repo] still working after 440 s; last phase started: door.script
Reading the project graph slice (433 s): owners 140 of 569.
Reading the project graph slice (435 s): owners 140 of 569.
Reading the project graph slice (437 s): owners 144 of 569.
Reading the project graph slice (439 s): owners 144 of 569.
Reading the project graph slice (441 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 5, 0); waiting for memory headroom (931 of -181 MB reserved).
Reading the project graph slice (443 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 5, 0); waiting for memory headroom (931 of -181 MB reserved).
Reading the project graph slice (445 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 5, 0); waiting for memory headroom (931 of -181 MB reserved).
Reading the project graph slice (447 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 10, 0); waiting for memory headroom (931 of -181 MB reserved).
Reading the project graph slice (449 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 10, 0); waiting for memory headroom (931 of -181 MB reserved).
Reading the project graph slice (451 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 15, 0); waiting for memory headroom (931 of -166 MB reserved).
Reading the project graph slice (453 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 15, 0); waiting for memory headroom (931 of -166 MB reserved).
Reading the project graph slice (455 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 15, 0); waiting for memory headroom (931 of -166 MB reserved).
Reading the project graph slice (457 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 20, 0); waiting for memory headroom (931 of -182 MB reserved).
Reading the project graph slice (459 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 20, 0); waiting for memory headroom (931 of -182 MB reserved).
Reading the project graph slice (461 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 20, 0); waiting for memory headroom (931 of -182 MB reserved).
[repo] still working after 470 s; last phase started: door.script
Reading the project graph slice (463 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 25, 0); waiting for memory headroom (931 of -182 MB reserved).
Reading the project graph slice (465 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 28, 0), @semio-tech/os-infinite-world-bridge-rs (language:rust, 0, 0).
Reading the project graph slice (467 s): owners 144 of 569; running (name, kind, seconds, rows): @semio-tech/framework-os-kernel (language:cargo, 30, 0), @semio-tech/os-infinite-world-bridge-rs (language:rust, 2, 0), @semio-tech/os-infinite-world-host-rs (language:rust, 2, 0).
Reading the project graph slice (469 s): owners 145 of 569.
Reading the project graph slice (471 s): owners 145 of 569.
Reading the project graph slice (473 s): owners 149 of 569.
end 2026-11-15T02:21:12Z
native_rc=7
```

## Try 2

- Log `/Users/ueli/native-bootstrap-round-14-try-2.log`: `start 2026-11-15T02:22:18Z`, `end 2026-11-15T02:38:52Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3316240 run_peak_rss_kib=474672 stop=None fleet_refusal=True closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T02:27:43Z pid=73926 rss=3238MB footprint=[bun [73926]: 64-bit    Footprint: 427 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 521 projects arrived]`

Key lines of the graph service log, verbatim, in order:

```
graph: 521 stale projects, 0 taken from the fleet, 94 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 88 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 521
graph: nothing to wait for any longer: the fleet was asked for 521 stale projects and delivered 0 within 901 s; 521 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 88 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-15T02:38:52.626Z graph-wanted-expired {"wanted":"521","arrived":"0","seconds":"901","missing":"521","local":"0","heavy":", among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 88 more (this machine can give about 0 MB per project)"}
```

P12 sampler `round-14.p12-try-2.txt` (188 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-3af1e67bb957.log
closure entries before: 42537
2026-11-15T02:22:22Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T02:38:52Z no slice worker
closure entries after: 42537
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
[repo] still working after 890 s; last phase started: door.script
Reading the project graph slice (891 s): flush 1 of 1.
Reading the project graph slice (893 s): flush 1 of 1.
Reading the project graph slice (895 s): flush 1 of 1.
Reading the project graph slice (897 s): flush 1 of 1.
Reading the project graph slice (899 s): flush 1 of 1.
Reading the project graph slice (901 s): flush 1 of 1.
Reading the project graph slice (903 s): flush 1 of 1.
Reading the project graph slice (905 s): flush 1 of 1.
Reading the project graph slice (907 s): flush 1 of 1.
Reading the project graph slice (909 s): flush 1 of 1.
Reading the project graph slice (911 s): flush 1 of 1.
Reading the project graph slice (913 s): flush 1 of 1.
Reading the project graph slice (915 s): flush 1 of 1.
Reading the project graph slice (917 s): flush 1 of 1.
Reading the project graph slice (919 s): flush 1 of 1.
[repo] still working after 920 s; last phase started: door.script
Reading the project graph slice (921 s): flush 1 of 1.
Reading the project graph slice (923 s): flush 1 of 1.
Reading the project graph slice (925 s): flush 1 of 1.
Reading the project graph slice (927 s): flush 1 of 1.
Reading the project graph slice (929 s): flush 1 of 1.
Reading the project graph slice (931 s): flush 1 of 1.
Reading the project graph slice (933 s): flush 1 of 1.
Reading the project graph slice (935 s): flush 1 of 1.
Reading the project graph slice (937 s): flush 1 of 1.
Reading the project graph slice (939 s): flush 1 of 1.
Reading the project graph slice (941 s): flush 1 of 1.
Reading the project graph slice (943 s): flush 1 of 1.
Reading the project graph slice (945 s): flush 1 of 1.
Reading the project graph slice (947 s): flush 1 of 1.
Reading the project graph slice (949 s): flush 1 of 1.
[repo] still working after 950 s; last phase started: door.script
Reading the project graph slice (951 s): flush 1 of 1.
Reading the project graph slice (953 s): flush 1 of 1.
Reading the project graph slice (955 s): flush 1 of 1.
Reading the project graph slice (957 s): flush 1 of 1.
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 521 and delivered 0 within 901 s, 521 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 88 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T02:38:52Z
native_rc=75
```

## Try 3

- Log `/Users/ueli/native-bootstrap-round-14-try-3.log`: `start 2026-11-15T02:42:04Z`, `end 2026-11-15T02:59:44Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=2731520 run_peak_rss_kib=145104 stop=None fleet_refusal=True closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T02:44:44Z pid=86478 rss=2667MB footprint=[bun [86478]: 64-bit    Footprint: 418 MB (16384 bytes per page)] last=[graph: nothing to wait for any longer: the fleet was asked for 521 stale projects and delivered 0 within 901 s; 521 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/r]`

Key lines of the graph service log, verbatim, in order:

```
graph: 530 stale projects, 0 taken from the fleet, 96 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 90 more; it can give about 0 MB per project), it can infer 48 of the others (12 per wave); asking the fleet for 530
graph: nothing to wait for any longer: the fleet was asked for 530 stale projects and delivered 0 within 902 s; 530 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 90 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-15T02:59:44.444Z graph-wanted-expired {"wanted":"530","arrived":"0","seconds":"902","missing":"530","local":"48","heavy":", among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 90 more (this machine can give about 0 MB per project)"}
```

P12 sampler `round-14.p12-try-3.txt` (203 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-3af1e67bb957.log
closure entries before: 42537
2026-11-15T02:42:07Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T03:00:09Z no slice worker
closure entries after: 42537
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
Reading the project graph slice (964 s): flush 1 of 1.
Reading the project graph slice (966 s): flush 1 of 1.
Reading the project graph slice (968 s): flush 1 of 1.
Reading the project graph slice (970 s): flush 1 of 1.
Reading the project graph slice (972 s): flush 1 of 1.
Reading the project graph slice (974 s): flush 1 of 1.
Reading the project graph slice (976 s): flush 1 of 1.
[repo] still working after 980 s; last phase started: door.script
Reading the project graph slice (978 s): flush 1 of 1.
Reading the project graph slice (980 s): flush 1 of 1.
Reading the project graph slice (982 s): flush 1 of 1.
Reading the project graph slice (984 s): flush 1 of 1.
Reading the project graph slice (986 s): flush 1 of 1.
Reading the project graph slice (988 s): flush 1 of 1.
Reading the project graph slice (990 s): flush 1 of 1.
Reading the project graph slice (992 s): flush 1 of 1.
Reading the project graph slice (994 s): flush 1 of 1.
Reading the project graph slice (996 s): flush 1 of 1.
Reading the project graph slice (998 s): flush 1 of 1.
Reading the project graph slice (1000 s): flush 1 of 1.
Reading the project graph slice (1002 s): flush 1 of 1.
Reading the project graph slice (1004 s): flush 1 of 1.
Reading the project graph slice (1006 s): flush 1 of 1.
[repo] still working after 1010 s; last phase started: door.script
Reading the project graph slice (1008 s): flush 1 of 1.
Reading the project graph slice (1010 s): flush 1 of 1.
Reading the project graph slice (1012 s): flush 1 of 1.
Reading the project graph slice (1014 s): flush 1 of 1.
Reading the project graph slice (1016 s): flush 1 of 1.
Reading the project graph slice (1018 s): flush 1 of 1.
Reading the project graph slice (1020 s): flush 1 of 1.
Reading the project graph slice (1022 s): flush 1 of 1.
Reading the project graph slice (1024 s): flush 1 of 1.
Reading the project graph slice (1026 s): flush 1 of 1.
Reading the project graph slice (1028 s): flush 1 of 1.
Reading the project graph slice (1030 s): flush 1 of 1.
Reading the project graph slice (1032 s): flush 1 of 1.
The project graph could not be read on this machine: it can infer only 48 of the stale projects, the fleet was asked for 530 and delivered 0 within 902 s, 530 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 90 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T02:59:44Z
native_rc=75
```

## Try 4

- Log `/Users/ueli/native-bootstrap-round-14-try-4.log`: `start 2026-11-15T03:03:20Z`, `end 2026-11-15T03:20:29Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=3080080 run_peak_rss_kib=474816 stop=no new progress line for 908 s fleet_refusal=False closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T03:04:58Z pid=1281 rss=3007MB footprint=[bun [1281]: 64-bit    Footprint: 293 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 530 projects arrived]`

Key lines of the graph service log, verbatim, in order:

```
graph: 530 stale projects, 0 taken from the fleet, 96 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), @semio-tech/framework-pack-json-rs (about 15422 MB), repo (about 14264 MB), @semio-tech/framework-actor-retained-turn-rs (about 11518 MB), @semio-tech/framework-plugin (about 8626 MB), and 90 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 530
```

P12 sampler `round-14.p12-try-4.txt` (196 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-79a74804a430.log
closure entries before: 42537
2026-11-15T03:03:26Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T03:20:59Z no slice worker
closure entries after: 42537
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
Reading the project graph slice (925 s): flush 1 of 1.
Reading the project graph slice (927 s): flush 1 of 1.
Reading the project graph slice (929 s): flush 1 of 1.
Reading the project graph slice (931 s): flush 1 of 1.
Reading the project graph slice (933 s): flush 1 of 1.
Reading the project graph slice (935 s): flush 1 of 1.
Reading the project graph slice (937 s): flush 1 of 1.
Reading the project graph slice (939 s): flush 1 of 1.
Reading the project graph slice (941 s): flush 1 of 1.
Reading the project graph slice (943 s): flush 1 of 1.
Reading the project graph slice (945 s): flush 1 of 1.
Reading the project graph slice (947 s): flush 1 of 1.
Reading the project graph slice (949 s): flush 1 of 1.
[repo] still working after 950 s; last phase started: door.script
Reading the project graph slice (951 s): flush 1 of 1.
Reading the project graph slice (953 s): flush 1 of 1.
Reading the project graph slice (955 s): flush 1 of 1.
Reading the project graph slice (957 s): flush 1 of 1.
Reading the project graph slice (959 s): flush 1 of 1.
Reading the project graph slice (961 s): flush 1 of 1.
Reading the project graph slice (963 s): flush 1 of 1.
Reading the project graph slice (965 s): flush 1 of 1.
Reading the project graph slice (967 s): flush 1 of 1.
Reading the project graph slice (969 s): flush 1 of 1.
Reading the project graph slice (971 s): flush 1 of 1.
Reading the project graph slice (973 s): flush 1 of 1.
Reading the project graph slice (975 s): flush 1 of 1.
Reading the project graph slice (977 s): flush 1 of 1.
Reading the project graph slice (979 s): flush 1 of 1.
[repo] still working after 980 s; last phase started: door.script
Reading the project graph slice (981 s): flush 1 of 1.
Reading the project graph slice (983 s): flush 1 of 1.
Reading the project graph slice (985 s): flush 1 of 1.
Reading the project graph slice (987 s): flush 1 of 1.
Reading the project graph slice (989 s): flush 1 of 1.
Reading the project graph slice (991 s): flush 1 of 1.
Reading the project graph slice (993 s): flush 1 of 1.
Reading the project graph slice (995 s): flush 1 of 1.
end 2026-11-15T03:20:29Z
native_rc=7
```

## Stall 1: Last 60 Lines of the Service Log

```
graph: waiting for the fleet: 133 of 530 projects arrived
[repo] still working after 2030 s; last phase started: source.served
[repository:command] still running after 2030 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2040 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2050 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repo] still working after 2060 s; last phase started: source.served
[repository:command] still running after 2060 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2070 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2080 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repo] still working after 2090 s; last phase started: source.served
[repository:command] still running after 2090 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2100 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2110 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repo] still working after 2120 s; last phase started: source.served
[repository:command] still running after 2120 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2130 s
graph: waiting for the fleet: 133 of 530 projects arrived
[repository:command] still running after 2140 s
graph: waiting for the fleet: 274 of 530 projects arrived
[repo] still working after 2150 s; last phase started: source.served
[repository:command] still running after 2150 s
graph: waiting for the fleet: 274 of 530 projects arrived
[repository:command] still running after 2160 s
graph: waiting for the fleet: 274 of 530 projects arrived
[repository:command] still running after 2170 s
graph: waiting for the fleet: 274 of 530 projects arrived
[repo] still working after 2180 s; last phase started: source.served
[repository:command] still running after 2180 s
graph: waiting for the fleet: 274 of 530 projects arrived
[repository:command] still running after 2190 s
graph: waiting for the fleet: 274 of 530 projects arrived
[repository:command] still running after 2200 s
graph: waiting for the fleet: 321 of 530 projects arrived
[repo] still working after 2210 s; last phase started: source.served
[repository:command] still running after 2210 s
graph: waiting for the fleet: 321 of 530 projects arrived
[repository:command] still running after 2220 s
graph: waiting for the fleet: 321 of 530 projects arrived
[repository:command] still running after 2230 s
graph: waiting for the fleet: 321 of 530 projects arrived
[repo] still working after 2240 s; last phase started: source.served
[repository:command] still running after 2240 s
graph: waiting for the fleet: 321 of 530 projects arrived
[repository:command] still running after 2250 s
graph: waiting for the fleet: 321 of 530 projects arrived
[repository:command] still running after 2260 s
graph: waiting for the fleet: 321 of 530 projects arrived
[repo] still working after 2270 s; last phase started: source.served
[repository:command] still running after 2270 s
graph: waiting for the fleet: 321 of 530 projects arrived
```

## Stall 1: `ps` of the Service and the Run

```
  747 73331   747       17:09   2304   0.0 bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/native-bootstrap.sh /Users/ueli/native-bootstrap-round-14-try-4.log
  760   747   747       17:09   2960   0.0 bash /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh setup
  761   747   747       17:09    960   0.0 tee /Users/ueli/Documents/semio/.cache/f/native-bootstrap.log
35202 35172 35172 02-02:47:12 705024   0.5 /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Contents/MacOS/Claude Helper (Renderer) --type=renderer --user-data-dir=/Users/ueli/Library/Application Support/Claude --standard-schemes=cowor
86375     1 86375       38:00  68080   0.1 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.
86380 86375 86380       37:59 387104   0.9 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'
```
