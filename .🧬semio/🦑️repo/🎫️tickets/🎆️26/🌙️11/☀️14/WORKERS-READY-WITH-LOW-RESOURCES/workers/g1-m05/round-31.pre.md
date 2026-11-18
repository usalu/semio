# g1-m05 (Windows): Round 31, State Found

Written 2026-11-16T00:50Z by an agent session on the machine itself. This is the first file of this origin in the coordinator's ticket; the earlier history is `26/11/09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m05-join.md`.

## What This Session Does

- Follows `ROUND.md` round 31: it runs no setup command (`native-bootstrap`, `repo tools setup all`, `repo dependencies sync`), touches nothing under `.cache/f/ready`, stops no process and changes no build code.
- `round-31-observe.ps1` (this folder) appends one observation every 10 minutes to `round-31.md`. It reads files and process counters only.
- This session never started a setup try, so nothing had to be stopped.

## Machine

| Item | Value |
|---|---|
| Host, origin, group | `DESKTOP-6NSLEUO`, `g1-m05`, `g1` |
| System | Windows 11 Pro 10.0.22631, AMD Ryzen 7 5800X (8 cores, 16 threads), 65 447 MB RAM |
| Worktree | `C:\semio` on NVMe (NTFS), 99 GB free of 784 GB |
| Store | `S:\` (NFS 3, `\\192.168.178.70\Users\Shared\semio-store`), mounted and readable |
| Clock | synchronised by `time.windows.com`, last sync 2026-11-15T21:28Z |
| Long paths | `LongPathsEnabled` = 1 |
| Defender | real-time protection on; the exclusion list cannot be read without elevation |
| Last boot | 2026-11-13T12:39Z |

## Replica

- Scheduled task `semio-fleet-replica` is `Running`; supervisor pid 46616 since 2026-11-15T17:53Z, phase `run`, source `worktree`, code `b4c6513b87ef`.
- `health.json` (00:45Z): store `ok`, trust `shared-source` (verified 00:14Z), sidecar `build-agent` running (pid 57428, 0 restarts), `build-scheduler` and `build-cache` off.
- Failure counters in `health.json`: `blocked` 11 148 (last 00:45Z, still counting), `loop-blocked-restart` 18 (last 2026-11-15T02:41Z), `git-broken-config-unreadable` 1 (2026-11-15T17:53Z).
- Incoming changes arrive (`ROUND.md`, `pool-failures-and-windows-timeout.md` of 00:33Z are here). `S:\v1\origins\g1-m05\head.json` was last replaced 2026-11-15T15:41Z, so nothing was published from here for 9 hours before this file. Checked after writing it: the head was replaced at 2026-11-16T00:50:51Z (3 134 to 3 137 events), so this machine publishes and these files are in the store.
- `.cache\f\native-bootstrap.done` is absent: the worker is not ready.

## Automatic Ready Runner

- `ready` child pid 33364. Stages `native` (5 runs, 45 to 49 s, exit 0) and `repo-mcp` (22 s, exit 0) are done.
- Stage `dependencies` (3 of 5): tries 1 to 6 exit 1 after about 950 s, tries 7 and 8 `stage-timeout` at 3 600 s. This matches section Q2 of `pool-failures-and-windows-timeout.md`.
- Try 9 started by itself at 00:39:29Z (wrapper pid 56052, `repo.exe dependencies sync --all`) and is running.

## One Measurement for Q2 (Why the Stage Is Slow Here)

Process counters of the try 9 process tree over 30 s (00:47Z, Windows `Win32_Process`, read-only):

| Process | Resident | CPU in 30 s | Bytes read | Bytes written | File operations in 30 s |
|---|---|---|---|---|---|
| `bun.exe` pid 68716 (the worker of `dependencies sync --all`) | 1 351 MB | 10.5 s | 0.2 MB | 0 MB | 203 787 |
| `bun.exe` pid 53056 (door) | 59 MB | 0 s | 0 MB | 0 MB | 2 075 |
| `repo.exe` pid 92448, wrapper pid 56052 | 10 MB, 43 MB | 0 s | 0 | 0 | 1, 0 |
| `MsMpEng.exe` (Defender) | | 0 s | | | |

Reading (the first three points are measured, the last is my inference):

- The stage is not waiting for a lock holder outside its own process tree: the one worker does about 6 800 file operations per second.
- It moves almost no bytes (0.2 MB read, nothing written in 30 s) and uses a third of one core. The operations are therefore metadata calls (open, stat, close, directory reads), not content.
- Defender used no CPU in that window, so real-time scanning is not what limits it at that moment.
- Inference: the `store-ownership` frontier walk is bound by the number of single-threaded metadata calls on NTFS. At 1 560 index units per second (section 2.2) against 6 800 operations per second, one unit costs about 4 file operations. `round-31.md` splits the operations into read, write and other every 10 minutes, so the rate over a whole try can be read from it.

## Not Done

- `repo preflight --lang en` and the replica `status`: not run, so that no `repo` command runs beside the stage. Say so in `next.md` if you want them now.
- No elevated command was run. The Defender exclusion list and the `verify` run of the join report stay open.
