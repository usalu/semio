# 🧾️ Root cause: macOS "out of application memory" pause of Claude on 2026-11-27

Machine: 16 GB macOS 27.2.1, 10 cores. Times are local (CET). Sources: `host-capacity/metrics/t0..t4.ring` read with the repo's own `readSeriesV1`
(`🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts`; scripts `timeline.ts`, `fine.ts`, `bins.ts` in this folder), `warden/warden.log`, `warden/alarms.jsonl`,
`log show` (windows of at most 40 minutes, kept under 2 minutes each), `sysctl vm.*`, `footprint`, `top`.

## 1. What happened (timeline, numbers)

| time | available | compressed | swap used | swap-out | kernel pressure | warden level | warden subjects |
|---|---|---|---|---|---|---|---|
| 16:00 bucket | 3.14 GB min | 7.91 GB | 2.59 GB | 105 MB/s | warn | guard | 6.7 GB |
| 17:20 | 6.5 GB | 6.68 GB | 2.16 GB | 0.5 MB/s | warn | guard | 2.9 GB |
| 17:26 | 4.35 GB | 5.55 GB | 2.26 GB | 0.5 MB/s | normal | calm | 2.7 GB |
| 17:28 | 3.62 GB | 7.01 GB | 2.25 GB | 0.5 MB/s | **warn** | guard | 3.1 GB |
| 17:30:08-17:30:23 | 2.98 GB min | **8.26 GB (50.4 % of RAM)** | **3.06 GB = 95 % of the 3.22 GB allocated** | **705 MB/s** | **warn** | guard | 2.6 GB |
| 17:31 | 5.3 GB | 3.6 GB | 3.03 GB | 1.5 MB/s | normal | calm | 4.1 GB |
| 17:33-18:00 | 5.9-6.6 GB | 1.0-2.8 GB | 2.7 -> 2.67 GB | < 20 MB/s | normal | calm | 0-0.1 GB |
| 18:01:21 | 4.8 GB | 4.1 GB | 2.67 -> 1.55 GB | 101 MB/s | normal | calm | - |

Kernel log (`log show`, kernel/memorystatus):

- 17:28:07 to 17:30:23: the kernel idle-exits 17 daemons one per second (`killing_idle_process ... rf:low`, `memorystatus_available_pages 328166..514676`,
  `compressor_size 355832..534909` pages = 5.8 to 8.8 GB). This is the kernel's own reaction to warn pressure.
- 17:30:20: `apfs_truncate_locked ... swapfile3`: the kernel creates its third swap file while swap is 95 % full.
- 18:01:15: the user connects (RustDesk starts; HID idle was 22196 s at 17:28, 23396 s at 17:48: the machine was unattended).
- 18:01:21.861: `memorystatus: resuming Claude [11702]` and `loginwindow(176) performed out-of-band resume on pid 11702`: the user dismissed the
  loginwindow low-memory panel (`LowMemoryPanel doLowMemorySampleSequence` samples every 2 s, 630 samples between 17:20 and 18:01).

Macos logs the *start* of a pause nowhere (no `pausing` line exists in the unified log). The only window in 16:30-18:05 in which the host was in the
kernel's low-memory regime is 17:28-17:31, so the pause most probably began at about 17:30:20 and lasted about 31 minutes, until the user came back
at 18:01. The coordinator's estimate "17:35 to 17:52" is not supported by any counter: from 17:33 on every counter is calm. Claude itself held only
190-400 MB (pid 11702); it was paused because the panel pauses the largest user applications, and its renderer (1.3 GB footprint) and `git` helpers
(2.1 GB each) are charged to it.

The kernel pressure relation (all samples of 2026-11-25..27 with a bucket width of at most 60 s, `bins.ts`):

| compressor | samples | with pressure >= warn | swap used max | available min |
|---|---|---|---|---|
| 0-1 GB | 59 | 0 % | 3.58 GB | 6.18 GB |
| 1-2 GB | 587 | 0 % | 4.74 GB | 5.71 GB |
| 2-3 GB | 237 | 0 % | 3.68 GB | 5.29 GB |
| 3-4 GB | 131 | 0 % | 2.83 GB | 4.53 GB |
| 4-5 GB | 89 | 0 % | 2.83 GB | 4.27 GB |
| 5-6 GB | 79 | 11 % | 4.17 GB | 4.05 GB |
| 6-7 GB | 40 | 78 % | 4.27 GB | 3.67 GB |
| 7-8 GB | 28 | 100 % | 4.31 GB | 3.30 GB |
| 8-9 GB | 6 | 100 % | 5.22 GB | 3.20 GB |

The kernel's own parameters of this machine (`sysctl vm.compressor.*`): swap files of 256 MB to 1 GB (`swapfile_size_max` 1073741824), a new file is
created only when the free swap falls under `highwater_bytes` = 256 MB, allocated 3.22 GB now with a high-water of 5.37 GB and `used_bytes_hwm`
4.63 GB. The compressor limit (`vm.compressor_segment_pages_compressed_limit` 13.4 M pages = 205 GB) is far away: the compressor does not
fill up, **the swap files are created too slowly for a page-out burst** (705 MB/s peak against 256 MB of reserve = 0.36 s).

## 2. Who held the memory

The repo's warden saw 2.4-3.1 GB of its subjects. The compressor alone held 7.7-8.3 GB, so more than two thirds of the memory was held by processes the warden
neither owned nor measured. The rings keep no per-process history (the warden state keeps only its last `others` and `tree`, overwritten every tick, and
only while not calm), so the owners of the 17:28-17:30 burst cannot be named from counters. What is known from the live machine and the earlier findings:

- the Claude desktop app diff pane: `git ... diff --numstat -z -M --raw --no-abbrev 277881dca4ea` against the whole 315,851-file worktree, 2.06 GB per run, repeated
  (the app log shows 1,173 `getGitDiff ... -> null` after a 30 s timeout and 2,288 diff runs in `main.log`);
- repo-started test workers that no warden kind matched: `bun ... vitest/dist/worke*` (2.5 GB in `state.json` `tree` at 17:5x, 11.3 GB on a 3.6 GB grant earlier today)
  were *not subjects*; the kinds matched `bun test` only, so a vitest run was invisible to every cap, grant check and cancel;
- Claude renderer 1.3-1.5 GB, Firefox, shells.

## 3. Why the warden did not prevent it

1. **Every line was calm at the moment macOS paused** (arithmetic at 17:30:20, policy before this change):
   available 2.98 GB > guard 1.64 GB; headroom = available + min(free swap + growable disk 31 GB, 8 GB) = ~11 GB > 4.1 GB; swap used 3.06 GB = 18.7 % < 35 %;
   swap-share = 3.06 / (3.22 + 31 GB growable) = 9 % < 45 % (the disk growth term makes the share meaningless: macOS does not grow swap that fast);
   compressed 8.26 GB = 50.4 % < 55 % (9.0 GB). Only the kernel pressure `warn` raised the level to *guard*, which holds new admissions but suspends nothing when the
   subjects are few and `keepRunning` keeps the dearest one alive. `alarms.jsonl` holds no alarm after 05:39:44: the warden did nothing in the 12.5 hours before the pause.
2. **Enforcement reached only subjects**, and only 2 of the running trees were subjects (stage and slice-worker). Vitest workers, the desktop app's git and everything else were outside.
3. **Restart gaps.** `warden.log` shows 151 starts, 119 `stopped: its own source changed` and 31 `stopped: no subject for 10 minutes`. A warden ended itself on every edit of its sources (other agents edit the
   repo tool all day) and nobody guarded until the next repo call started a new one; the ring `warden.level` shows a 4 minute hole 17:50-17:54 (stop 17:52, restart 17:52:50).
   `idle` ended the guard when no subject had existed for 10 minutes although grants (`grants/*.json`) and heavy foreign processes were live.
4. **Grants exceeded without consequence.** `shelter` only ordered the victims; a subject 8 GB above its grant was sheltered from nothing and was acted on only after the host reached guard/danger.
5. **Footprint reads.** The log line `the memory footprint of running process trees cannot be read on darwin (Command failed: footprint -f bytes --noCategories -p 40314)`:
   `footprint` exits 66 *without any output* when one of the listed pids is gone (`Unable to find pid for process matching '40314'`) or unreadable
   (`task_read_for_pid`, `Unable to analyze process ... try as root`), so the whole batch failed and the observer fell back to the resident set for the whole tree.
   The warden loop itself already charged rows with the kernel's `phys_footprint` through `proc_pid_rusage` over `bun:ffi` (`🍏️native-footprint`), which reads 457 processes in 1.2 ms
   (175 system processes are unreadable and keep their resident set). Callers outside bun (node test runners, the gate under node) used the CLI and hit the failure. Native and CLI agree
   (905.14 MB against 949,109,792 B for the same Python allocation).
   Note: bun's large `Buffer`s are tagged *reclaimable* by the kernel and do not count in `phys_footprint` (944 MB reclaimable, 8 MB footprint); the footprint is the kernel's own truth,
   the resident set would price them at 947 MB. Synthetic proofs therefore allocate with Python.

## 4. What changed (all in the repo tool, tests included)

| area | change | files |
|---|---|---|
| continuity | a warden whose sources changed starts its successor (`--handover <pid>`) before it ends; the successor waits up to 15 s for the lock and takes over within one tick. The guard no longer ends for idleness while a grant is live or a foreign process holds >= 1 GB. | `🛡️warden/🔁️loop`, `🛡️warden/⌨️main` |
| grants enforced | every live grant (`grants/*.json`) is a subject: a grant on a pid that is a subject books its reservation, a grant on any other process becomes the subject `granted` of its whole tree; new kind `vitest-worker` (cap 8 GB). | `🛡️warden/🟦️.ts` `grantedWardenSubjectsV1` |
| over-grant throttle | a subject whose excess over its grant is >= max(512 MB, 25 %) while the host has no spare memory (available < 15 % or any swap/compressor depth or pressure) raises the level to guard (trigger `grant`) and is suspended first (never the one kept running); overdue it is cancelled; bilingual reason with the excess. While memory is spare the excess is only counted. | `decideHostWardenV1` |
| foreign memory | the warden measures everything outside subjects and grants (kernel footprint) every tick: `state.foreign {megabytes,processes,heavy}`, `others` always filled; admission uses it as a floor of the unmanaged memory (`withUnmanagedHostV1(host,ledger,foreign)`, gate and status). New kind `foreign-git-read` (a `git diff|log|blame|show|status` above 1.5 GB is cancelled with SIGTERM, whoever started it, bilingual alarm). | `🟦️.ts`, `📏️decision`, `🎫️gate` |
| thresholds from compressor and swap | compressed guard 55 % -> 30 % (4.9 GB: the 4-5 GB bin has 0 % warn, 5-6 GB 11 %), danger 70 % -> 36 % (5.9 GB: 6-7 GB has 78 % warn); new line `swap-free`: the free swap against the largest swap file the kernel creates (`vm.compressor.swapper.swapfile_size_max`, read on darwin, default 1024 MB): below 1 file guard, below 0.5 file danger, only when >= 50 % of the allocated swap is used. At 17:30:20 the free swap was 160 MB = danger 3 s before the peak. | schema `Policy`, `swapDepthHostWardenV1`, `readWardenSwapV1` |
| footprint | `footprint` that fails as a whole is asked process by process (<= 32); the warden already charges every row natively. | `📡️sampling` |

Calibration of the margin: the kernel pressure line is the compressor at 5-6.5 GB (32-40 % of RAM); the new guard (4.9 GB) sits under the first sample with warn and the new danger (5.9 GB) under the 78 %
bucket, so the warden acts 1-3 GB of compressor before the kernel does, and the incident peak (8.26 GB) is 2.4 GB beyond the danger line. The swap-free danger (512 MB) leaves 2x the kernel's own
256 MB reserve, i.e. 0.7 s of the observed 705 MB/s burst; the decision cannot be faster than the 0.5 s tight tick, so the margin comes from the compressor line firing first.

## 5. The desktop app diff pane: what it uses as base, and safe mitigations (for the coordinator)

Facts:

- `~/Library/Logs/Claude/main.log`: every `LocalSessions.getGitDiff: cwd=/Users/ueli/Documents/semio, base=⛳wip, head=<tree>` (2,288 lines, all with `base=⛳wip`). The base is the *default branch* of the repo.
- `.git/refs/remotes/origin/HEAD` is `ref: refs/remotes/origin/⛳wip`; `refs/remotes/origin/⛳wip` and the local `refs/heads/⛳wip` both point at `277881dca4ea` (2026-06-04), which is 762 commits behind
  `HEAD` (`refs/heads/🐙ueli/⛳wip`, `fa8a184f7cbd`). There is no `main`. `origin/🐙ueli/⛳wip` is current (HEAD is not ahead of it).
- So the pane diffs the worktree against a 6 month old commit: 315,851 files, and passes `-M` explicitly (`diff.renames=false` and `diff.renameLimit` do not help; git skips renames above 304,427 but still reads all 315,851 entries).
  Each run takes the 30 s timeout of the app, then runs the fallback `git status`, which also times out.

Mitigations that need no change of the working tree (each one is a ref/config write inside `.git`, which is never replicated; **I did not run any of them**):

1. `git update-ref refs/heads/⛳wip HEAD` and `git update-ref refs/remotes/origin/⛳wip HEAD` (fast-forward both stale refs to the current tip; instant; the diff becomes the size of the real working changes). The archivist machine, which already commits to the linear history, should repeat it after
   every commit (or pass the tip of `🐙ueli/⛳wip` instead of `HEAD`).
2. Alternatively `git remote set-head origin 🐙ueli/⛳wip`, which only rewrites `refs/remotes/origin/HEAD`, and the local `⛳wip` ref moved as in 1 (the log says `base=⛳wip`, the local name).
3. Defence in depth, already in place: the warden kind `foreign-git-read` cancels such a git above 1.5 GB (it is re-run by the app every few seconds, so this only caps the damage: fix 1 or 2 removes the cause).

## 6. Not solved / honest limits

- No counter says which process tree grew at 17:28; the guard now records `foreign` and `others` in `state.json` every tick, but the metric rings still keep no per-process history (follow-up: a `foreign.memory` metric in the
  collector catalogue).
- The warden can only suspend or cancel subjects, grants and `foreign-git-read`. Foreign processes of any other kind (a browser, an editor) are counted against the admission floor, never touched.
- A pause already in progress is not undone by this change; only the cause is removed.
