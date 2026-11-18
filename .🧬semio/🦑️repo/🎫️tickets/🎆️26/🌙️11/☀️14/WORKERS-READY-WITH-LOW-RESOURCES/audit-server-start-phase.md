# Audit: graph read start phase on g1-server (2026-11-16, read-only)

Read-only audit. No sub-agents, no builds, nothing killed, no git command run. Samples are in `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/audit-start/`. Times are UTC (CET = UTC+1). Snapshot taken 03:23 to 03:25Z.

## 1. Process tree (live)

- 74362 `bun` runner (parent 74312), 27 MB.
- 74363 `bun` read, door `script.ts` (`repo show projects`), 815 MB, state S, 0.1% CPU. Phase `source.served` since 02:50:36Z, still open.
- 74396 `bun --eval` graph child, 5.1 GB RSS, 0.3% CPU. Does the graph work.
- 83881 `bun` heavy-questions child (`graph/inference/script-inputs/heavy-questions/script.ts`), 1.6 GB, spawned about 03:12Z. Policy: 900 s timeout, 2560 MB RSS cap.
- 87955 `bun`, child of 83881, 49% CPU, short-lived (gone at the next check).

## 2. Phase table (this read, pid 74363 / 74396)

| Phase | Start (UTC) | Duration | Dominant |
|---|---|---|---|
| gate, source.capture | 02:50:30 | 6 s | light |
| source.served (parent waits on graph child) | 02:50:36 | 33+ min, open | waiting on child |
| graph.plan.doubt | 02:50:55 | about 30 s | not sampled |
| graph.plan.facts #1 | 02:51:25 | 2.0 min | not sampled |
| settle #1 (verification and rederivation) | 02:53:26 | 10.4 min (wall 625 s) | ask 426 s (68%), support 56 s, seal 38 s |
| flush and nx body-recheck | 03:03:25 | 1.3 min | I/O |
| membership index (132,729 entries) | 03:04:43 | 0.5 min | filesystem walk |
| graph.plan.facts #2 | 03:05:15 | 2.0 min | not sampled |
| settle #2 | 03:07:16 | 4.9 min (wall 292 s) | ask 124 s, seal 49 s |
| graph.build.prime (heavy-questions child 83881) | 03:12:13 | 11.2+ min, still `flush 1:1` | waiting; 83881 threads blocked |
| owners wave | not reached | | |

Settles reported `slow=7404` and `slow=7313` of 7470 closures, so almost every closure takes the slow path on every read. This is the main cost in the planning phases.

## 3. Where the time goes: CPU user vs sys vs waiting

- Machine (top, one snapshot): 15% user, 79% sys, 6% idle. Load average 37 on 10 cores. Swap in use 3.08 GB of 4 GB. Sys time is spread over git, bun and NFS/kernel work.
- Read tree itself: under 1% CPU in every snapshot. The read is waiting, not computing.
- Samples (1 ms, 5 s): 74396 has about 85% of its top-of-stack in `__ulock_wait2`, `kevent64` and `__psynch_cvwait`. 74363 and 83881 are about 90% lock or condvar waits. bun has no symbols, so JS functions cannot be named.

## 4. Top 5 hot stacks (from samples)

1. 74396 main thread: `__ulock_wait2` (42,680 samples), lock wait.
2. 83881: `__ulock_wait2` (25,950) and `__psynch_cvwait` (15,570), waiting on its own children or a lock.
3. 74363: `__ulock_wait2` (22,651) and `__psynch_cvwait` (17,627), waiting on the graph child.
4. 74396: `kevent64` (5,705), event-loop wait on child pipes.
5. 74396 CPU frames: `_platform_memcmp` (280 samples) under bun addresses 0x1010fbd40 and 0x102ff1b68. Consistent with 4.5M live strings at flush.

## 5. Non-fleet share of machine CPU (top, core-%, 1000% = whole machine)

- git from Claude Helper (pid 87483) and replica (8967): about 390%. Claude git poll runs `git status` and `git diff --numstat` on a 278k-entry index (see `host-load-audit.md`).
- kernel_task: 78%.
- WindowServer 30%, mediaanalysisd 24%, nfsd 14%, TextEdit 8%, Windows App 7%, RustDesk 6%, other Claude app about 40% (earlier snapshot).
- Total non-fleet: about 590% (about 60% of the machine).
- Repo bun processes (70769, 85162, 86596, 87955, replica 8967): about 100%. The read tree: under 5%.

## 6. Suggestions, ranked by expected minutes saved (estimates, not measured)

1. Make closures stay on the fast path (about 10 to 15 min per read). Settles show 7300+ slow closures; the R53 fix is not visibly active in this run. Check whether the running process loaded old modules and why the cutoff misses.
2. Shorten or overlap the prime stall (about 10 min). `graph.build.prime` blocks the owners wave for up to the 900 s child timeout with no progress beyond `flush 1:1`. Add progress, or let owners start before prime ends. Note: the child is due to hit its 900 s timeout at about 03:27Z. What the read does on that kill is not verified.
3. Stop the Claude git poller on this repo, or lengthen it (unknown, several minutes across phases; removes about 3 to 4 cores of sys load).
4. Cap concurrent repo builds and walk-farm children by memory: swap is 3.1 GB and the eval child is 5 GB. Reduces sys time.
5. Spotlight exclusion (`.metadata_never_index`) for the repo and `.cache` (minor).

## Not verified

- Which JS function the waits sit under (no bun symbols).
- The effect of each suggestion; the minute figures are estimates.
- The read's behavior after the 900 s kill.
