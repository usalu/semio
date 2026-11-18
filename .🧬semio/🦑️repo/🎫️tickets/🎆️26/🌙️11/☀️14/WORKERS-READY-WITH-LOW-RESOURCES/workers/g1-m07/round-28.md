# g1-m07 round 28 report (tries 1-4, 11:36Z-11:55Z; written 15:45Z by the next session)

Machine: Mac mini, 16 GiB, macOS 27.2.1, worktree /Users/ueli/Documents/semio. Ready: **no** (`.cache/f/native-bootstrap.done` absent).

## Why nothing arrived from g1-m07 between 11:17Z and 15:30Z
A zero-byte `.git/index.lock` was left in this worktree at 11:17:11Z (12:17:11 CET) by a git process that no longer exists (no holder in `lsof`). From then on the replica reported `Git state: degraded` (`write-tree-failed`, later `index-lock-stale`) and `status` said "nothing is published or applied until it works"; 5181 incoming entries were queued, the view of this machine was `967/10/4330` against `43/11/121` on the others. The session that drove round 28 ended at 11:55Z without noticing. At 15:30Z the lock was removed (the index was copied first); 45 seconds later: `Git state: shared`, stage tree `d4f57b999b0e43f416cc7fdc7987626bc133b3e9`, queue 10. `verify` at 15:33Z: drift on 5 of 288223 paths, all sample files that g1-m02 and g1-m10 were writing at that moment. g1-m02 shows the same numbers (`1/5171`, view `967/10/4331`): look for a stale `.git/index.lock` there.
Also removed here: 10 rules of `.git/info/exclude` (written 03:56Z under the heading `# claude-code-runtime`, all `**/.claude/...` runtime paths, none of which exists on disk); `fleet` listed `g1-m07 differs in guards.infoExclude: 10 instead of 0`.

## Step 0 (repo tools setup all)
Attempt 4 (11:17:31Z-11:36:53Z): **exit_code=0**, 1162 s (`round-17.tools-setup.txt`, log `~/tools-setup-round-16.log`). Not rerun since.

## Tries
| try | start (UTC) | end | native_rc | plan stage inference grew to | available | slice worker peak (P12 sampler) | run peak resident | hard stop |
|---|---|---|---|---|---|---|---|---|
| 1 | 11:36:53 | 11:40:15 | 75 | 9866 MB | 4504 MB | 7706 MB | 7.49 GB | none |
| 2 | 11:43:29 | 11:45:28 | 75 | 9945 MB | 6156 MB | 5094 MB | 3.37 GB | none |
| 3 | 11:48:31 | 11:50:24 | 75 | 9978 MB | 6714 MB | 5425 MB | 3.32 GB | none |
| 4 | 11:53:33 | 11:55:29 | 75 | 9966 MB | 4313 MB | 7648 MB | 3.09 GB | none |

Final client line of every try (numbers as in the table): "The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to N MB, more than the 6553 MB this machine gives a plan, and only N MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine."

`waiting for the fleet: x of N`: no such line in the client logs of these four tries (largest x: not measured, none printed).

## Lines of the service log (host-82a7a2b56cae.log, tries 3 and 4)
- `graph: memory readings: machine 16384 MB (physical 16384 MB, cgroup limit none), available 8780 MB` (try 3), `available 8899 MB` (try 4)
- `graph: 703 stale projects, 0 taken from the fleet, 14 too heavy for this machine (workspace (about 17778 MB), @semio-tech/energy-oracle-py (about 11128 MB), @semio-tech/repo-lib (about 9652 MB), repo-vite-storage (about 9244 MB), @semio-tech/2d-trace-rs (about 8876 MB), @semio-tech/framework-os-dev (about 7676 MB), and 8 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave)`
- `2026-11-15T11:50:23.863Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":9978,"limit":6553,"available":6714,"reserve":2048,"reason":"ceiling","worker":{"heapMb":670.6,"extraMb":477.8,"objectCount":3856668,…`
- `2026-11-15T11:55:28.133Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":9966,"limit":6553,"available":4313,"reserve":2048,"reason":"ceiling","worker":{"heapMb":653.9,"extraMb":461.8,"objectCount":3846239,…`
- No `The resident service met an unexpected failure` and no EPERM in these tries.

## Launcher
Unchanged in all four tries: `~/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo`, target inode 3496898, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357` (`round-28.launcher.txt`). Receipts at 15:40Z: `client.json` ok, `service.json` ok (route direct), `launcher.json` **waiting** for pid 59558 (the replica supervisor started 15:07Z); `preflight` therefore prints `FAIL worktree access: worktree-access-denied (waiting): a consent prompt for launcher waits`, while that daemon reads and writes the tree (it applied the 5171 queued entries).

## Preflight at 15:36Z (23 checks, 2 failed, 3 warnings)
- `FAIL cache endpoint: http://192.168.178.70:6090 is not reachable: Unable to connect` (the host answers ping in 0.4 ms; port 6090 refuses). Not a setting of this machine.
- `FAIL worktree access` as above.
- `skip` on every `guards.*` check: "not measured: no git, no repository or the guards could not be read".
- Warnings: native build pending, time synchronisation needs root, event visibility 12476 ms at the 95th percentile (median 3367 ms).

Round 29 series starts next (5 minutes apart).
