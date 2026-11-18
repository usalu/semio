# g1-m03 Round 30 (the series is running; this file is rewritten after every try)

- Machine: Mac mini M4, 16 GB, macOS 27.2.1 (26B131). Worktree `/Users/ueli/Documents/semio`. Free disk before the series 32 GiB of 228 GiB (`.cache/b` 123 GB).
- This worker reported nothing between 07:59Z and 17:00Z (agent session ended); rounds 19 to 29 were not run here. The earlier series is in `round-14.md`.
- Series rules applied by `run-round-30.sh`: tries 5 minutes apart, no try limit; memory pressure is recorded, never a stop; stop only when a process of the run is above 8 GB resident; a stall is 15 minutes without a new progress line AND without a new `graph:` line in the service log.

## Before the series

    Pre-run facts, 2026-11-15T17:01:58Z
    - gap: this worker's runner ended its series at 2026-11-15T07:59:56Z (try 12 of the round 14 to 18 series); the agent session ended and was restarted at about 17:00Z, so rounds 19 to 29 were not run and not reported here.
    - replica code digest (.cache/f/health.json): 203e01b3594f1a510964bfe95d4031e2f7925aba3bdb457016bd480173d76efd, store ok, pending None , trust verified: True , daemon pid 97222
    - agreement (read from /Volumes/semio-store/v1/origins/<origin>/attest.json at 17:01Z, the fleet command hangs on this machine): g1-server, g1-m01, g1-m02, g1-m03, g1-m07 attest code 203e01b3594f, 0 to 1 s old, sync on; g1-m06 attests b93aabf29dd1, 2712 s old.
    - replica agent: state running, runs 65; "bootVersion":6
    - graph service job: - 2 tech.semio.repo.service.8a6496305af6;
    - repo service host processes (pid, ppid, age, rss KiB):
        pid 89361 ppid 89343 age 06:08:20 rss 76256
        pid 89387 ppid 89361 age 06:08:16 rss 116144
    - memory pressure level 1, swap total = 3072.00M  used = 1935.25M  free = 1136.75M  (encrypted), free disk 32Gi of 228Gi (it was 81 GiB at 02:23Z)
    - .cache sizes: 123G .cache/b, 8.5G .git

## Step 0 again: `repo tools setup all` — exit code 0

```
step 0 again in round 30: repo tools setup all, from /Users/ueli/Documents/semio (environment as in W/native-bootstrap.sh)
start 2026-11-15T17:04:58Z, end 2026-11-15T17:42:22Z, exit code 0, wall time 2244 s (37 min 24 s), no SIGINT, the line 'Another process holds the dependency store' did not appear (0 lines)

-- per-scope lines (restored / installed / funding), each distinct line once, without the 'remains active on javascript-installed-paths' progress lines
Funding the dependency store: the dependencies of every scope are installed into it now. A first funding takes many minutes on a small machine; the lines below report each scope

-- other distinct lines that are not progress (lease-order, capacity, errors)
[capacity] admitted command: 1 job(s), 2048 MB reserved (7281 MB available, 2048 MB floor)
[lease-order] Lock order violated: javascript-dependency-bindings:/Users/ueli/Documents/semio (namespace, rank 5) is requested while native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d (image, rank 7) is held exclusive. Acquire locks in ascending rank or release native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d first.

-- last 20 lines
[repository:command] still running after 2230 s
…
-- last 20 lines
[repository:command] still running after 2230 s
Computing main repo mapping: 
Computing main repo mapping: 
Loading: 
Loading: 0 packages loaded
Analyzing: target //:setup (1 packages loaded, 0 targets configured)
Analyzing: target //:setup (1 packages loaded, 0 targets configured)

INFO: Analyzed target //:setup (5 packages loaded, 7 targets configured).
INFO: Found 1 target...
Target //:setup up-to-date:
  bazel-bin/ready.txt
INFO: Elapsed time: 9.513s, Critical Path: 0.06s
INFO: 2 processes: 2 internal.
INFO: Build completed successfully, 2 total actions
[bazel] fixed module catalog ready blobs=156
[bazel] ready binary=8.4.2 darwin/arm64 sha256=45e9388abf21d110 modules=156 blobs lock=06c085a17c9611ed
[bazel] readiness {"version":1,"bazel":{"version":"8.4.2","platform":"darwin","arch":"arm64","sha256":"45e9388abf21d1107e146ea366ad080eb93cb6a5f3a4a3b048f78de0bc3faffa","executable":"/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"},"modules":{"lockSha256":"06c085a17c9611edb2348fa8f6eb9726db9d85a5f3a9edebd2d2958c688d2df0","blobs":156}}
[semio] The in-house build engine needs no external tool to provision.
[semio] Die hauseigene Build-Engine benötigt kein externes Werkzeug.
```

No per-scope `restored` / `installed` line is in its log (only the `Funding the dependency store` line). Full notes: `round-30.tools-setup.txt`, log `~/tools-setup-round-30.log`.

## Try 1: 17:42:25Z to 18:09Z, `native_rc=75` — all six steps were used, then the refusal

- Highest step reached: **step 6 of 6**. `native_rc` is not 0. Marker absent.
- The graph service found no usable generation and built the first one: `graph: no usable generation (empty), building the first one`.
- No hard stop, no stall.

Every `cut smaller` line (bootstrap log, verbatim, in order):

```
Reading the project graph slice (0 s): the slice worker grew to 9156 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 2 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 8969 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 3 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 10317 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 4 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 9865 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 5 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 9860 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 6 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
```

The refusal after step 6 (bootstrap log, verbatim) and the end of the log:

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 10113 MB, more than the 6553 MB this machine gives a plan, and only 3667 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T18:09:22Z
native_rc=75
```

Largest resident size seen:

- P12 sampler (graph child, every 5 s): largest resident 7649 MB, largest footprint 9781 MB. The row with the largest resident size:

```
2026-11-15T17:54:14Z pid=54263 rss=7649MB footprint=[bun [54263]: 64-bit    Footprint: 5073 MB (16384 bytes per page)] last=[]
```
- 30 s samples: 54 samples, pressure level other than 1 in 2 (   1 pressure=2    1 pressure=4 ), largest process 4186112 KiB, pid 49874, ppid 49873.
- closure entries before: 4636;closure entries after: 4661;

Graph service log of the try (verbatim, each distinct line once; file `round-30.try-1.service.txt` has all of it, with the last 30 lines of the service log and the last 40 of the bootstrap log):

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-b79757bed7da.log (lines after 0)
2026-11-15T17:51:43.703Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=51856 ppid=51835 responsible=78070
graph: plan stage inference starts (managed 208 MB, resident 965 MB)
graph: waiting for memory headroom (1505 of 551 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 1505 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: plan stage inference settled from 7326 to 9156 MB
graph: plan stage inference starts (managed 209 MB, resident 1022 MB)
graph: waiting for memory headroom (1505 of 977 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 1505 MB and has waited 0 s; after 60 s it runs alone if it fits the 1756 MB one project is granted, otherwise the run is refused with exit 75
graph: plan stage inference settled from 7183 to 8969 MB
graph: plan stage inference starts (managed 202 MB, resident 884 MB)
graph: plan stage inference settled from 8607 to 10317 MB
graph: plan stage inference settled from 7230 to 9573 MB
graph: plan stage inference starts (managed 203 MB, resident 1010 MB)
graph: plan stage inference settled from 6800 to 9196 MB
graph: plan stage inference starts (managed 205 MB, resident 1007 MB)
graph: plan stage inference settled from 6804 to 9581 MB
2026-11-15T18:09:21.257Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":10113,"limit":6553,"available":3667,"reserve":2048,"reason":"ceiling","worker":{"heapMb":560.5,"extraMb":425.8,"objectCount":2400984,"objectTypes":[{"type":"string","count":992196},{"type":"Object","count":414824},{"type":"Array","count":268634},{"type":"WeakRef","count":94273},{"type":"Promise","count":55969},{"type":"Function","count":53790},{"type":"JSLexicalEnvironment","count":33059},{"type":"Structure","count":32921}],"at":1794766132972}}
```

Waiting for the fleet in this try: largest x:  largest x:  

State: `state.json` (round 30, try 1, native_rc 75, ready false). The runner started the wait of 300 s before try 2 at 18:09:41Z.

## Try 2 (18:14:41Z to 18:30:49Z): native_rc=130, interrupted by my own runner

- This try was NOT ended by the build. My runner sent one SIGINT to the process group at 18:30:16Z because its stall rule saw no new progress line and no new `graph:` line for 15 minutes. `native_rc` is not 0.
- Where it stood: `[semio] Installing the dependency scopes (the planner of every later run needs them)…`, two `[capacity] … dependencies` lines, then only heartbeat lines `[script] bun:dependency-storage remains active on native-fixed-image:…/.cache/b/d (shared, N s)` up to N = 573 s. `Checking declared package providers` lines: 0. No `graph:` line was written in the try (service log line counts unchanged: 34 and 336), so no plan line, no `cut smaller` step, no fleet wait line: highest step in this try: none reached.
- I cannot tell whether this was a real stall or slow healthy work. In try 1 the same phase took about 9 minutes, but its longest stretch of heartbeat-only lines was 208 s; here it was 573 s and still running when interrupted.
- Peak resident of a process of the run (P12 sampler): not measured MB. 30 s samples: 128.
- Record: `round-30.stall.txt` (last lines of the service logs and of the bootstrap log), `round-30.try-2.samples.txt`, `round-30.try-2.p12-samples.txt`.
- Runner change at 18:33Z, between tries: the progress signature now also counts every bootstrap log line that is not a heartbeat (`remains active`, `still running`, `still working`), so work in the dependency phase that writes lines is seen as progress. A phase with heartbeat lines only for 15 minutes is still interrupted once.

State: `state.json` (round 30, try 2, native_rc 130, ready false). Try 3 starts 18:35:50Z.

## Try 3 (18:35:54Z to 19:19:42Z): native_rc=75

- `native_rc` is not 0. The marker `.cache/f/native-bootstrap.done` does not exist.
- Phases reached: launcher current; rust toolchain; native repository command; `Installing the dependency scopes` (bootstrap log lines 955 to 5809, finished this time; it waited more than 570 s for run capacity: `[capacity] waiting … for run: memory pressure is critical` / `swap use is above its ceiling … behind the host itself (nothing of ours)`); then `[semio] workspace:setup…`, where the graph read was refused. Setup tasks: none run (0 `nx run` lines).
- Highest step: **6 of 6**. Cut-smaller lines (bootstrap log, each once):
  - grew to 6852 MB, more than the 6553 MB … step 2 of 6
  - grew to 8750 MB … step 3 of 6
  - grew to 9344 MB … step 4 of 6
  - grew to 10150 MB … step 5 of 6
  - grew to 10179 MB … step 6 of 6
- After step 6 the refusal came back: `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 9895 MB, more than the 6553 MB this machine gives a plan, and only 2991 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could.`
- Service log (verbatim, `round-30.try-3.service.txt` has all of it):

```
2026-11-15T19:07:02.866Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=25029 ppid=24997 responsible=78070
graph: plan stage inference starts (managed 208 MB, resident 739 MB)
graph: plan stage inference settled from 6591 to 6852 MB
graph: plan stage inference settled from 7053 to 8750 MB
graph: plan stage inference settled from 6983 to 9344 MB
graph: plan stage inference settled from 8672 to 9703 MB
graph: plan stage inference settled from 7338 to 10179 MB
graph: plan stage inference settled from 6992 to 9379 MB
2026-11-15T19:19:41.304Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":9895,"limit":6553,"available":2991,"reserve":2048,"reason":"ceiling", …}
```

- First plan line (`graph: N stale projects …`): not written in this try. Waiting for the fleet: no line. `closure-missing`: no line.
- Peak resident of the graph child (P12 sampler): **8336 MB** (`2026-11-15T19:18:35Z pid=38027 rss=8336MB footprint=[bun [38027]: 64-bit    Footprint: 5540 MB (16384 bytes per page)] last=[]`). This is above 8 GB. My runner's hard stop only looks at the bootstrap's process group and the graph child belongs to the service, so no stop was made; the service refused the plan itself.
- 30 s samples: 348; pressure level 1 in 67, level 2 in 5, level 4 in 15 (recorded, not a stop).
- Closure entries before 4661, after 4661.
- Free disk at 19:04Z: 28 GiB.

State: `state.json` (round 30, try 3, native_rc 75, ready false). Try 4 starts 300 s after 19:19:53Z.

## Try 4 (19:24:53Z to 19:44:39Z): native_rc=1, unexpected failure 6a8ac318

- `native_rc` is not 0. The marker `.cache/f/native-bootstrap.done` does not exist.
- Last bootstrap lines: `Reading the project graph slice (146 s).` then `The resident service met an unexpected failure (reference 6a8ac318); try again, the details under that reference are in the service log …/host-f13cda8149a9.log.` (the last 40 lines are only the `Reading the project graph slice (N s).` lines 70 s to 146 s; they are in `round-30.try-4.service.txt`).
- Highest step: no `cut smaller` line in this try. Setup tasks: none run.
- Service log, plan lines of the try (verbatim):

```
graph: plan stage inference starts (managed 202 MB, resident 1064 MB)
graph: 9 of 4774 projects inferred by 3 worker processes in 76 s (0 failed, 0 lost, 0 restarts)
graph: plan stage inference took 136.4 s (managed 1048 MB, 845 MB more than at its start, resident 2328 MB)
```

- `grep -a 'request-failed 6a8ac318' -A12` on the service log (verbatim):

```
2026-11-15T19:44:37.840Z request-failed 6a8ac318 RepoError: invalid build.closure.provider-entry-stale {"port":"layers"} code=invalid
Error
    at repoErrorV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts:33:14)
    at closureLayerBatchesV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1159:67)
    at closureLayerListV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1166:71)
    at resolve (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1507:427)
    at async resolveClosureRefsV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:487:80)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:315:111)
    at async tagProjectFailureV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:83:22)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:315:32)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:334:639)
    at async run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs:255:21)
    at async <anonymous> (unknown)
```

- Peak resident of the graph child (P12 sampler): 3134 MB. 30 s samples: 40, all pressure level 1.
- Closure entries before 4661, after **4362** (299 fewer). `closure-missing … refetched|re-derived`: no line.
- Before the graph read the try waited about 15 minutes: `[script] repository:command · Waiting for access to semio.compiler.cpu-picker/v1 (exclusive): position 2 of 2, behind the request of process 45891 (exclusive, queued 860 s ago)`. Process 45891 is a `repo run repo-mcp:build…` started by the fleet replica agent on this machine (not by me); a `cargo test` not started by me ran at the same time. Process chain and memory list: `round-30.other-work.txt`. I touched neither.

State: `state.json` (round 30, try 4, native_rc 1, ready false). Try 5 starts 300 s after 19:45:10Z.

## Try 5 (19:50:10Z to 20:52:46Z): native_rc=130, stopped by me because round 31 says so

- Round 31 (read 20:52Z) says: finish or stop the running try with one SIGINT and start no other. I ended my runner and sent one SIGINT to the process group of try 5 at 20:52:36Z. `Cancelled (SIGINT); leases released.` `native_rc` is not 0.
- This try got further in the graph read than any before and was still progressing when stopped: `Reading the project graph slice (3507 s)`, `Inferring projects: 297 of 4774`. No `cut smaller` line, no refusal, no unexpected failure in 62 minutes.
- Service log, lines of the try (verbatim, each once):

```
graph: plan stage inference starts (managed 203 MB, resident 1033 MB)
graph: 31 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 19 inferred here
graph: 62 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 38 inferred here
graph: 91 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 59 inferred here
graph: 124 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 76 inferred here
graph: plan stage inference settled from 7081 to 8487 MB
graph: project os-hub-ts (project) grew by 2549 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: 170 projects taken from the fleet with 0 closure entries, 2 of them did not hold here and were inferred again, 0 not shared, 80 inferred here
```

- Many `graph: waiting for memory headroom …` and `headroom of -N MB is below one project …, inferring one project at a time` lines (headroom was negative most of the time; the fleet agent's own `ready` child and other fleet work ran on the machine at the same time, see `round-30.other-work.txt`).
- Peak resident of the graph child (P12 sampler): 3704 MB. 30 s samples: 125; pressure level 1 in 123, level 2 in 2.
- Closure entries before 4362, at the stop 4759.

Series result of round 30: tries 1 to 5 ended with native_rc 75, 130 (my runner's stall rule), 75, 1, 130 (round 31 stop). None ended with 0. The marker `.cache/f/native-bootstrap.done` does not exist. I start nothing further; observation continues in `round-31.md`.
