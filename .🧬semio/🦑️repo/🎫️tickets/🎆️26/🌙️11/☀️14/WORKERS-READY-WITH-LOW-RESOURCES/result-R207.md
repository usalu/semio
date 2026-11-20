# R207: what the farm children of the refresh read wait for (living report, 2026-11-18 13:15 to 13:35 UTC)

Continues R204. Scratch: `scratchpad/R207/` (`s*.txt`, `i*.txt` = the stack samples, `grp.py` = per-group CPU sampler through `proc_pidinfo`, `usys.py`, `prof/` = one cpuprofile). No landing, no interrupt, no kill, no download; four 3-second `sample` runs (three farm children, then one idle child and the parent), one profiled owner through R200's harness.

## 0. Answer (read this first)

**The serial-helper hypothesis is false for this read, and the children do not wait for a lock, a socket, a heavy-question child or the cache server. They wait for a CPU.** The machine (M1 Max, 8 P + 2 E cores) has 0 percent idle (top: 45 to 80 percent system, 20 to 45 percent user) and a load average of 35 to 65; the read gets about 1 core of 10. Every process of the read (parent and children) is runnable but gets 15 to 25 percent of a core. A request that crosses processes (parent to child and back) needs both sides to be scheduled, so a "busy" child alternates between "descheduled" and "idle, waiting for the equally descheduled parent". That is the picture behind "0.2 to 5 percent CPU, 15 to 27 owners running for 1000 s, 0 waiting leases".

Because the cause is not a code fault, **I wrote no fix and no red-first law**; inventing a helper pool for a helper that is not in the path would be a change without evidence. What I can state with numbers is below; the decision it needs is for the coordinator (section 5).

## 1. Process tree of the read at 13:15 to 13:30 UTC

Runner 38180 (parent pid 1, 0 percent) > read 38186 (0.0 %, S) > graph process 38258 (the parent of the farm, 13 % of a core, main thread in JS 76 % of 2313 samples, `__read_nocancel` 5 %) > nine farm children (`--semio-role=graph-farm-worker`, pids change: 39521/39522/39523 (1 h old) and 61894 left the tree between 13:16 and 13:22 and were replaced by 10006, 15296; 1983, 7173, 7174, 7175, 79112 stay) with 0 to 41 % each, state `R` (runnable) or `S`, priority 20 (the decayed priority of a long-running process; a fresh process gets 31 to 46). Eight defunct children of 38258 (`Z`, pids 15559 to 15566) stayed unreaped for more than 2 minutes: the parent's main thread is too starved to collect them. No heavy-question child, no walk-farm child and no cache-server process existed in the tree during any of the samples (`ps` ten times, 13:16 to 13:30); `footprint` ran once in 30 s (1 s).

Stack samples (macOS `sample <pid> 3 -mayDie`, 1 ms interval, ~2300 samples per thread; files `s39521.txt`, `s61894.txt`, `s79112.txt`):

| process | main thread | `Worker` thread | other 12 threads |
|---|---|---|---|
| child 39521 | 2211 of 2311 in bun user code, 83 `memmove`, 5 `madvise`; 0 in any wait call | `kevent64` (idle loop) | idle (`__ulock_wait2`, `kevent64`, `cvwait`) |
| child 61894 | 1595 + 352 (JIT frame) of 2300 in user code; 0 in a wait call | `kevent64` 2279 | idle |
| child 79112 | 2115 of 2249 in user code, 13 `read` | `kevent64` | idle |
| child 7174 at 13:30 (idle moment, 5 % CPU) | **`kevent64` 2458 of 2580** = waiting for a message | `kevent64` | idle |
| parent 38258 | 1763 of 2313 in user code, 123 `read` | n/a | idle |

So no child was blocked in a socket or pipe read, `flock`/`fcntl`, `waitpid` or synchronous filesystem call in the three busy samples (a thread that is runnable but not scheduled shows its user-space program counter, which is exactly what 96 % user-code frames with 20 % CPU mean). When a child is idle it sits in `kevent` waiting for the parent. `iostat`: disk 80 to 256 MB/s at 10 000 to 37 000 transfers/s during the first sample (page-ins of file cache, 4000 to 13 000 pages per 3 s; swap 1.6 GB used, no swap-in/out in the window); compression small. `repo leases status`: 0 waiting.

## 2. Who has the CPU (30 s and 20 s windows, `grp.py`, per process user + sys)

| group | cores (window 13:17) | cores (13:27) |
|---|---|---|
| the read (parent + 8 children) | 1.25 (user 1.02, sys 0.22) | 0.70 |
| this machine's own fleet pool role (`sidecar ... pool`, 5 to 6 owner-pool children) | 1.18 (sys 0.53) | |
| slice lab (R197, 2 + 3 processes) | 0.51 + 0.13 | 0.72 |
| replica daemon | 0.24 | |
| Claude.app and its `git` polling | 0.18 averaged, but single `git` processes at 40 to 540 % | |
| other readable processes (477) | 0.74 | 0.69 |
| visible total | 4.2 of 10 | 2.8 of 10 |

The rest of the 10 cores is root-owned kernel time that `proc_pidinfo` cannot attribute without sudo (`kernel_task` 52 to 79 %, `WindowServer` 20 %, `mediaanalysisd` 13 to 20 %, `nfsd` 10 %, `fseventsd` 25 %, `mds` 8 %). Proof of the scheduling disadvantage of long-running processes: a fresh bun busy loop for 12 s got **45 % of a core** at load 35; the profiled owner run (fresh process, section 3) got 74 %; the farm children, one hour old with priority 20, get 15 to 25 %.

## 3. Cost of one owner on the real farm-child path (R200's harness `prof2.ts`, `LATE=1`, `--cpu-prof`, fresh process, load 52)

`@semio-tech/norm-din4108-rs`, first owner of the process: produce 41.8 s wall, **31.1 s CPU**, RSS 137 to 2507 MB, heavy child peak 2131 MB (R200 measured 40.2 s / 27.5 s; reproduced). Self time of the 46.7 s profile: file reads 5.4 s (layer reads for the pack bundle 2.0 s), brotli (shown as `writeSync` under `zlibBufferSync`) 3.8 s in the closure-store `attempt` plus 5.3 s zlib inclusive, `statSync` 3.1 s, `JSON.stringify` 2.5 s, `existsSync` 2.1 s, `lstatSync` 1.0 s, `hash` 0.4 s, GC 0.9 s. `packBundleV1` 7.3 s inclusive, `walkImportsV1` 9.0 s, `observedSourceAnalysisV1` 6.7 s, `projectObservationHoldsV1` 5.4 s. No single function dominates after R200's fix; the cost is spread over reading, hashing and compressing the same layers (R200 section 1 "late-mode profile" list stays valid and unfixed).

Rate of the running read through the real path: 88 of 4785 done at 68 minutes (42 present, 46 inferred here), the sampler's 10-minute window 0.28 to 0.34 per minute; the read's children together used about 28 CPU-minutes in the first hour for those 88 owners, and 0.7 to 1.25 cores now. That is **150 to 220 CPU-seconds per finished owner** (75 CPU-s per minute divided by 0.34 to 0.5 owners per minute) and about **0.04 owners per minute per child** over the whole read (9 children, 0.34 per minute).

## 4. Projection

- Now: 4697 owners left x 150 to 220 CPU-s = 195 to 290 CPU-hours; at the read's 0.7 to 1.25 cores that is 160 to 410 hours (the sampler's own eta: 229 to 289 h).
- If this machine were idle and the nine children had a core each: 13 to 32 hours.
- If eight such machines shared the owners and adopted each other's records (the pool publishes about 40 records per minute fleet-wide): 2 to 4 hours, but only if the records hold on the taking tree (35 of 35 looked at here were stale, 34 at 13:30; R204 section 4, the row is printed only by the next read).
- No read of the whole workspace can finish on this machine at the current load; with the load at today's level it cannot finish in a week.

## 5. What stands between this machine and a finished read

1. CPU share: the read is one of ~30 CPU-hungry processes. The three big ones that are not the read: the machine's own fleet pool role (1.2 cores, infers owners for the fleet while the read infers for itself), the slice lab (0.5 to 4 cores, R197) and Claude.app's `git` polling (bursts to 5 cores of `git` on the 1 GB worktree). The coordinator can decide to run the whole-read on a machine without the lab and without the pool role (one way: the pool role stopped by its supported command, not by kill) or to stop the lab while a read runs. I did none of it.
2. CPU per owner: 150 to 220 CPU-s in late mode (R200 measured the pieces; closure-store pack/unpack/hash/brotli of the same layers, `walkImportsV1` repeats). A fix needs its own ticket: memoize `listDigest` per layer content across owners of a process and avoid re-packing layers whose digest is in the store (R200 section 1), measured on this harness with the numbers of section 3 as base.
3. Stale fleet records (35 of 35): cause not yet named; needs a read that carries R204's landings 2 and 3. I did **not** use the authorised interrupt: it is only allowed after my first landing and I made none. A coordinator interrupt (`kill -INT` of the runner whose command line holds `graph-server/read-record.json`) would cost the 88 owners nothing (records persist) and one prime phase (about 5 to 8 minutes).
4. Parent starved: with the whole system at 15 to 25 % per process, the parent (13 % of a core, 76 % runnable) is slower to dispatch, adopt and reap than the children need; an unreaped-defunct count of 8 for two minutes shows it. Raising the priority of the parent or children needs root (negative nice) and is not available; fewer simultaneous competitors is the only lever.

## 6. Not done / not verified

No code change, no law, no landing, no interrupt (task items 2 and 4 not performed, reasons above). The kernel share of the CPU (root processes) is not attributable without sudo; I did not run `fs_usage`. The cpuprofile owner is a Rust owner; the many TypeScript and `verify the stored record` looks of the read were not profiled separately. The 3-second samples show only user-space program counters without symbols (bun has no symbol table), so what JavaScript the busy child ran at that moment is not known.
