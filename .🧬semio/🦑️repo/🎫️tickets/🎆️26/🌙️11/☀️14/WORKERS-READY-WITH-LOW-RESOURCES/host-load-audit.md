# Host Load Audit: g1-server (10 cores, 32 GiB)

Read-only audit, 2026-11-15 17:37 to 17:43 CET. No sub-agents, no builds, no sudo, nothing killed, no state-changing git command. All numbers are from plain tools (ps, top, iostat, vm_stat, nfsstat, stat, sample, mdutil).

Load averages: 34.7 (1 min) / 51 (5 min) / 51 (15 min) at 17:37; 55 at 17:42. Load is a 1-minute average of runnable plus uninterruptible threads, so it reads about 3.5x to 5.5x the 10 cores.

## 1. CPU split and owners

| Share | Samples (top, 3 snapshots 1 s apart, plus iostat 6x5 s) |
|---|---|
| user | 21% / 34% / 56% (top); about 29% (iostat average) |
| sys | 80% / 65% / 41% (top); about 61% (iostat average) |
| idle | 0% / 1% / 3% (top); about 10% (iostat average) |

Most of the CPU is kernel time. kernel_task alone measured 79% and 42% of a core in two snapshots.

Owners, from the third top snapshot (sum of listed processes is about 700% of 1000%):

| Group | CPU (approx, %=1 core) | Notes |
|---|---|---|
| git spawned by Claude desktop (parent Claude Helper, pid 87483) | about 260% (three git processes at 87 to 95% each) | Confirmed parent for pids 95366, 95367, 95539 |
| git spawned by a bun build (parent 80320) | a few % | Confirmed for pid 12620 |
| Graph read `bun --eval` (pid 32013) | 20% to 80%, about 50% average | Cumulative CPU time about 59 min |
| Fleet framework build `bun` (pid 80320, child of fleet `boot.mjs run` pid 80262) | 61% to 200% | State U (uninterruptible) once; 10 min of CPU time |
| Other bun (pids 88344, 93182) | 66% and 20% | Parent not identified; short-lived |
| kernel_task | 42% to 79% | Kernel work (VFS, NFS, I/O) |
| WindowServer | 27% | Display |
| Spotlight (mds_stores 25%, mds 6%, mdworkers) | about 31% | Low now; mdworkers under 3% total |
| nfsd | 8% to 14% | NFS server for the fleet |
| Claude app (claude, Claude Helper x2, BackgroundShortcuts) | about 40% | Excluding the git poller |
| Remote desktop (RustDesk --server, Windows App) | about 18% | 47 and 41 threads |
| GitKraken | 0% | Not running |
| Replica supervisor | not found | No process has both `shared-worktree` and `supervisor` in its args. Closest is fleet `boot.mjs run` (pid 80262, idle, 0% CPU) with busy child 80320 |

## 2. Runnable and blocked threads (what drives load)

- Total threads: about 3,215 to 3,236 across 593 to 601 processes.
- Per-process `ps -M` snapshots show one to three runnable threads per process (the STAT column is a point-in-time value), so the load comes from many processes each briefly runnable, not one hot process.
- Processes with the most threads: RustDesk 47, Windows App 41, bun 80320 24, WindowServer 25 to 26, nfsd 19, fseventsd 17 to 20, Claude Helper 22 to 23.
- Load-carrying processes: about 17 bun processes (fleet and graph builds), 3 to 4 git processes, nfsd (3 runnable), mds_stores (2 runnable), and WindowServer.
- Caveat: an earlier per-thread count was miscounted (my match hit capital R in command names), so I did not use it.

## 3. Disk and NFS

Disk (iostat, disk0, 6 samples of 5 s):
- Reads 124 to 162 MB/s at 19k to 25k transfers/s, average transfer about 7 KB.
- Bandwidth is not saturated for an internal SSD, but the IOPS are high and the workload is small random reads.
- Writes are negligible on disk0 (about 400 to 600 write ops/s in the top counters).

NFS server (nfsstat -s, 30 s window, 17:38:06 to about 17:38:37):

| Op | Ops/s (approx) |
|---|---|
| Getattr | 920 |
| Lookup | 222 |
| Access | 47 |
| Read | 31 |
| Write | 11 |
| Create | 10 |
| Commit | 9 |
| Setattr | 7 |
| Rename | 8 |
| Remove | 1 |
| Total | about 1,270 ops/s |

Server RPC errors: about 87/s (2,611 in 30 s). Cause not identified; the counter is cumulative and unexplained.

The NFS traffic is dominated by attribute checks (Getattr plus Lookup, about 90%), which is what a tree-wide `git status` produces from remote clients.

## 4. Memory and swap

- Physical memory 34 GB total (32 GiB), 26 to 27 GB used, free 3.4 to 10 GB across samples (fluctuates).
- Swap: 572 MB used of 2 GB. swapins and swapouts counters did not change over 30 s (0 swap I/O).
- Pageouts: 0 in the window.
- Pageins: about 1,800 to 6,400 per second (file-backed, roughly 30 to 100 MB/s).
- Page faults: about 70,000 to 150,000 per second.
- Compressor activity: negligible.

Memory is not the constraint. The load comes from CPU and kernel work, plus file page-ins.

## 5. Git polling

- Polling source: Claude desktop Helper (pid 87483) spawns `git status --ignore-submodules=dirty --porcelain=v2 --branch -z` and `git diff --ignore-submodules=dirty --submodule=short --no-textconv --numstat -z -M 277881dca4ea640e85beffcc5527b3d7e3feae94`.
- Cadence: about one git process spawned per second across all parents (30 distinct git pids in 30 s).
- Duration (from elapsed times, not measured by me): `git status` about 2 to 3 s. `git diff --numstat -M <base>` at least 27 s (two observed processes were still running at 26 to 27 s elapsed).
- `.git/index`: 79,649,249 bytes (about 76 MiB), 278,056 tracked entries. Repo `.git` is 8.5 GB.
- Index rewrite frequency: mtime changed every 3 to 5 s (17:39:02, :06, :09, :12; then 17:41:48, :53, :59, :04). Size stayed the same. `git status` runs without `--no-optional-locks`, so it refreshes and rewrites the index on each poll. The `ls-files` calls do use it.

Rough cost: the 76 MB index rewrite every 4 s is about 19 MB/s of writes plus a full lstat walk of 278k paths per poll. That walk is the likely source of the kernel time and Getattr/Lookup load.

## 6. Spotlight

- `mdutil -s /`: "Indexing enabled."
- mds_stores is at 6% to 25% of a core. mdworker processes total under 3% at the moment.
- Spotlight is not in a heavy indexing burst but is active. It contributes about 30% of a core in the busiest snapshot.

## Ranked options

Can be changed without sudo and without closing apps (ranked by expected gain):

1. Reduce the git poll load. The Claude desktop git poller is the largest single consumer (about 2.6 cores in peak snapshot). Options:
   - Lengthen or disable its status poll for this repo (app setting; the app is the owner, so the user should do this).
   - Use `git diff` with `--stat` or a cached base only on demand.
2. Exclude the tree from Spotlight. Add a `.metadata_never_index` file at the repo root and under `.cache` (no sudo, reversible). This removes the mds_stores and mdworker work on large trees.
3. Cap fleet and graph build concurrency. The fleet build child (pid 80320) and the graph read bun (pid 32013) together take 1.5 to 3 cores. A lower worker cap in the repo build config would protect the 10 cores. This is a config change for the team to decide.
4. Shrink the tracked index. A 278k-entry index is the root cause of the stat storm. Moving generated or vendored trees out of tracking would cut both the git walk and the NFS Getattr/Lookup load. This is a repo change and needs the owners' approval.
5. Quit or idle remote desktop servers (RustDesk --server, Windows App) if not in use. About 18% of a core plus 88 threads. Only the user can decide this.

Only the user can do (these need sudo, a settings change in an app, or a repo change that I must not make):

- Set the Claude desktop app's git polling interval or disable it for this repo.
- Add repo git config `core.untrackedCache=true` and `core.fsmonitor` (a repo modification; the user should run `git config` deliberately). The Claude git calls currently pass `core.fsmonitor=`, which disables fsmonitor, so the app must allow it too.
- Adjust NFS server export options or thread count in `/etc/exports` (needs root).
- Adjust NFS client attribute caching (acregmin, acdirmin) on the 50 fleet machines (needs root on the client machines).
- Change Spotlight privacy under System Settings (UI, user only).
- Stop or quit RustDesk and Windows App if not in use.

## Not verified

- Replica supervisor: not located by name (see section 1).
- Exact git polling parents for all observed git pids: only pids 95366, 95367, and 95539 were confirmed as children of pid 87483, and pid 12620 as a child of bun 80320.
- `sample` failed for nfsd and mds_stores (system daemons), so their stacks are not included.
- The per-thread R count is a point-in-time value, so it does not fully reconcile with a 1-minute load of 35 to 55.
