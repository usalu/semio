# g1-m07 round 29 report (tries 1-4, 15:34Z-16:50Z)

Machine: Mac mini, 16 GiB, macOS 27.2.1, worktree /Users/ueli/Documents/semio. Ready: **no** (`.cache/f/native-bootstrap.done` absent, `native_rc=75` four times).

The series began at 15:34Z, after the replica of this machine reported `Git state: shared` again (15:31Z, see `round-28.md`). Tries 5 minutes apart.

## Tries
| try | start (UTC) | end | native_rc | plan stage inference grew to | available at the stop | available at plan start | too heavy | run peak resident | largest `waiting for the fleet: x` | hard stop |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 15:34:39 | 16:10:58 | 75 | 8679 MB | 2252 MB | not measured | not measured | 3.47 GB | none printed | none |
| 2 | 16:16:17 | 16:26:43 | 75 | 10219 MB | 2871 MB | 6422 MB | 43 of 703 | 3.49 GB | none printed | none |
| 3 | 16:32:02 | 16:38:01 | 75 | 9884 MB | 2639 MB | 6586 MB | 47 of 703 | 2.18 GB | none printed | none |
| 4 | 16:43:13 | 16:50:12 | 75 | 9948 MB | 2558 MB | 7789 MB | 47 of 703 | 3.69 GB | none printed | none |

Try 1 lasted 36 minutes: the client printed `Reading the project graph slice (N s): flush 1 of 1.` up to 2080 s and `[repo] still working after 2090 s; last phase started: door.script` before the refusal. Tries 2 to 4 took 6 to 10 minutes.

Final client line of every try (numbers as in the table): "The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to N MB, more than the 6553 MB this machine gives a plan, and only N MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine."

`waiting for the fleet: x of N`: the line appears in no client log and in no service log of the four tries. Largest x: not measured, none printed. The service log says `0 taken from the fleet` in every try.

## Lines of the service log (host-c5f9b6744aec.log, tries 2 to 4; try 1 ran on host-e9fb7654b9f7.log)
- `2026-11-15T16:17:28.996Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=19267 ppid=19204 responsible=69226`
- try 2: `graph: memory readings: machine 16384 MB (physical 16384 MB, cgroup limit none), available 6422 MB`
- try 2: `graph: 703 stale projects, 0 taken from the fleet, 43 too heavy for this machine (@semio-tech/s-dev-composition (about 24150 MB), workspace (about 20880 MB), @semio-tech/framework-rs (about 18460 MB), @semio-tech/repo-test-runner-go (about 14876 MB), @semio-tech/browser-actor-import-test (about 14012 MB), @semio-tech/cargo-workspaces (about 13744 MB), and 37 more; it can give about 3512 MB per project), it can infer …`
- try 2: `graph: preparing 660 of the 660 open stale projects (1 compute slots), 0 of 660 already prepared`
- try 2: `2026-11-15T16:26:38.567Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":10219,"limit":6553,"available":2871,"reserve":2048,"reason":"ceiling","worker":{"heapMb":2749.8,"extraMb":2154.3,"objectCount":13034333,…`
- try 3: `2026-11-15T16:38:00.390Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":9884,"limit":6553,"available":2639,"reserve":2048,"reason":"ceiling","worker":{"heapMb":739.8,"extraMb":524.4,"objectCount":4282986,…`
- try 4: `2026-11-15T16:50:11.567Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":9948,"limit":6553,"available":2558,"reserve":2048,"reason":"ceiling","worker":{"heapMb":946.1,"extraMb":635.3,"objectCount":5592014,…`
- Tries 3 and 4: `47 too heavy`, `preparing 656 of the 656 open stale projects (1 compute slots), 0 of 656 already prepared`.
- No `graph-plan-settled`, no `The resident service met an unexpected failure`, no EPERM, no `closure-missing` (driver count 0 in every try).

Against round 28: the estimates of the heavy projects doubled (the largest was `workspace (about 17778 MB)`, 14 too heavy; now `@semio-tech/s-dev-composition (about 24150 MB)`, 43 to 47 too heavy), and the memory left at the stop fell from 4313-6714 MB to 2252-2871 MB.

## Launcher
Unchanged before and after every try: `~/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo`, target inode 3496898, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357` (`round-29.launcher.txt`).

## Replica and preflight at 16:55Z
- `status`: `Git state: shared`, stage tree `bb58893207bab4930c7772b9e165f8a862e67a27`; g1-m07 `ok`, code `203e01b3594f`, view `108/11/205` (equal to g1-server, g1-m01, g1-m03, g1-m04), publish p50/p95 270/942 ms, apply 356/913 ms. `.git/index.lock` absent, `.git/info/exclude` holds 0 rules.
- `preflight`: 23 checks, 1 failed, 3 warnings.
  - `FAIL cache endpoint: http://192.168.178.70:6090 is not reachable: Unable to connect` (unchanged, not a setting of this machine).
  - `ok worktree access: the replica can read /Users/ueli/Documents/semio` (it was `worktree-access-denied (waiting)` at 15:36Z).
  - `ok event visibility`: 356 ms median, 913 ms at the 95th percentile (12476 ms at 15:36Z, while the stage caught up).
  - Warnings: native build pending; time synchronisation needs root; `free disk (worktree): 23.6 GiB free, below the recommended 30.0 GiB`.
  - Every `guards.*` check still prints `skip … not measured: no git, no repository or the guards could not be read`, although the `git` check of the same run says `git found (/usr/bin/git)`.

Files: `round-29.tries.txt`, `round-29.tail.txt`, `round-29*.monitor.txt`, `round-29*.samples.txt`, `round-29*.p12-samples.txt`, `round-29.launcher.txt`, logs `~/native-bootstrap-round-29[-try-k].log`.

## Addendum (written 19:45Z): tries 5 and 6, started under the round 29 label, ran the round 30 code
Round 30 was published at 16:55Z, try 5 had started at 16:52:13Z. Both tries are reported here; the series labelled `round-30` starts at 19:40Z.

| try | start (UTC) | end | native_rc | highest step | peak resident (30 s samples) | hard stop |
|---|---|---|---|---|---|---|
| 5 | 16:52:13 | 18:46:41 | 1 | step 2 of 6 | 6.25 GB | none |
| 6 | 18:52:12 | 18:54:43 | 130 | none printed | 8.05 GB (8440096 KiB) | yes, SIGINT by my monitor |

### Try 5 (114 minutes)
- Only `cut smaller` line: `Reading the project graph slice (0 s): the slice worker grew to 10207 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 2 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.` No step 1 line was printed before it.
- No exit 75. The client then printed `Reading the project graph slice (N s): flush 1 of 1.` and `[repo] still working after N s; last phase started: door.script` up to 6800 s, and ended with: `The resident service met an unexpected failure (reference 0835b24c); try again, the details under that reference are in the service log …/host-cacfd08454f7.log.`
- `grep -a 'request-failed 0835b24c' -A12`:
```
2026-11-15T18:46:40.881Z request-failed 0835b24c RepoError: invalid build.closure.provider-generated-import {"port":"typescript-package-index","targets":"@semio-tech/ui-rs:generate"}
    at repoErrorV1 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts:33:14)
    at toolClosureGroupV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1046:33)
    at processTicksAndRejections (native:7:39)
graph: plan answered, managed 118 MB, resident 700 MB (released 0 MB)
[repository:command] still running after 6819 s
```
- Same log, before it: `graph: memory readings: machine 16384 MB (physical 16384 MB, cgroup limit none), available 9035 MB`; `graph: 703 stale projects, 0 taken from the fleet, 47 too heavy for this machine (@semio-tech/s-dev-composition (about 24004 MB), workspace (about 20754 MB), …`; `graph: preparing 656 of the 656 open stale projects (1 compute slots), 0 of 656 already prepared`; `graph: preparing the stale projects failed (invalid build.closure.provider-generated-import {"port":"typescript-package-index","targets":"@semio-tech/ui-rs:generate"}); the refresh computes them itself`.
- `waiting for the fleet`: no line. `access-proof version=3 … sample=ok reproof=on route=direct pid=47484`.

### Try 6 (2.5 minutes): the round 23 hard stop collides with the round 30 step-down
At 18:54:43Z the graph child (`bun -- …/📚️library/🏗️build/…`, pid 63956) held 8440272 KiB resident, pressure level 1. My monitor applies "stop a try when a process of the run is above 8 GB resident" and sent one SIGINT; `native_rc=130`. Under round 30 the slice worker is expected to grow to about 10 GB before the service cuts the work smaller (10207 MB in try 5, which happened to fall between two 30 s samples). With the rule unchanged, a 16 GB Mac is stopped by its own monitor before step 2 begins. A sentence from the coordinator is needed: raise the limit above the cut point, or exempt the graph child. Until then the rule stays in force here, as round 30 says.

### Other observations
- 19:20:03Z, service log host-b79757bed7da.log (a `repo` call of another session on this machine, not a try): `graph-plan-too-heavy {"stage":"verification","operation":"verification","megabytes":10286,"limit":6553,"available":6244,"reserve":2048,"reason":"ceiling",…`. The stage `verification` is refused the old way.
- The driver of tries 5 to 10 was stopped by me during its pause after try 6 so that the next tries carry the round 30 label.
