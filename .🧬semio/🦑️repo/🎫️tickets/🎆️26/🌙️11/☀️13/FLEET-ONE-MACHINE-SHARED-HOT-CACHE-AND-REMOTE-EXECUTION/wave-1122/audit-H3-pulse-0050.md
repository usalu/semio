# 🩺 Audit H3 Pulse: Worker Progress Versus Census (wave 1122)

Read-only audit of `S/origins/*` (`ready.json`, `ready-log-*.txt`, `service-failure*.txt`; log tails of the last 80 lines). Snapshot 1 at 23:42:50 UTC, snapshot 2 at 23:55:20 UTC (12.5 min apart; the wait ran short of the nominal 10 minutes by the tool's background handoff). Baseline is the census at 22:51 UTC (`wave-1121/census-workers-ready-blockers.md`), about 64 min before snapshot 1. Owners per minute = change of `taken N of M` divided by the snapshot interval. Scratch files are in the scratchpad `H3/` folder (outside the worktree). No files in the tree were changed.

RAM is known only where the hostname states it (16 GB). Other hosts show `?`.

| Origin | Platform / RAM | Stage (snap 2) | State (snap 2) | Attempts | Receipt age (s, snap 2) | Peak MB | Last progress line (verbatim, latest log) | Last error line (verbatim, non-containment log) | Movement vs 22:51 census | Owners/min (snap 1 to 2) |
|---|---|---|---|---|---|---|---|---|---|---|
| g1-m01 | macOS 16 GB | workspace | running | 0 | 1 | 9392 | `Reading the project graph slice (19966 s): taken (pass 2) 443 of 600.` | `the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet` (stalled log, about 5 h old) | 249 to 443 (+194, about 3.1/min over 64 min) | 5.5 (374 to 443) |
| g1-m02 | macOS 16 GB | workspace | running | 8 | 5 | 5492 | `Reading the project graph slice (4289 s): taken 267 of 600.` | `repository:command failed with exit code 1` (workspace-failed log). Census ENOSPC (22:23) not seen in the last 80 lines | 113 to 267 (+154, about 2.4/min) | 2.9 (231 to 267) |
| g1-m03 | macOS 16 GB | workspace | running | 3 | 9 | 10473 | `Reading the project graph slice (9923 s): taken 396 of 600.` | `the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet` (workspace-failed, about 2.6 h old) | 297 to 396 (+99, about 1.5/min) | 0.2 (394 to 396), flagged |
| g1-m04 | Linux (`?`) | tools | running (receipt stale) | 9 | 86531 | 1876 | none in the last 80 lines; last `[repository:command] still running after 6671 s` | `Internal error: Original Script process terminated (reference 557d4d71)` (tools-failed, about 26 h old) | unchanged, stale since Nov 20 | 0 (no counter), flagged |
| g1-m05 | Windows (`?`) | dependencies | running (receipt gap) | 13 | 417 | 7398 | none (dependency stage; `bun:dependency-storage` activity line) | `repository:command failed with exit code 1` (dependencies-failed, about 4 h old) | census workspace/refusal state gone; now dependencies running. Receipt was 6 s old at snap 1, 417 s at snap 2 | n/a, flagged (no write for about 7 min) |
| g1-m06 | macOS (`?`) | dependencies | waiting | 17 | 416 | 7539 | workspace counter frozen: `Reading the project graph slice (3205 s): taken 327 of 596.` | `the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet` (stalled log, about 31 h old) | 327 unchanged; receipt was 29 h stale, now writing again; stage moved to dependencies, attempt 16 to 17 | 0 on workspace; stage moved on |
| g1-m07 | macOS 16 GB | workspace | running | 0 | 2 | 10662 | `Reading the project graph slice (284 s): taken 600 of 601.` | `the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet` (stalled, about 2.6 h old) | was waiting for leader load at census; now a new slice at 284 s elapsed (counter reset) | n/a (reset), flagged |
| g1-m08 | macOS (`?`) | workspace | running | 2 | 5 | 10208 | `Reading the project graph slice (14573 s): taken (pass 2) 76 of 600.` | `the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet` (stalled, about 7.7 h old) | 503 (pass 1) to 76 (pass 2): pass restart, not comparable | 3.4 (34 to 76, pass 2) |
| g1-m09 | not provisioned | absent | absent | n/a | n/a | n/a | none (no `ready.json`) | none | unchanged, absent | n/a, flagged |
| g1-m10 | macOS (`?`) | workspace | running | 1 | 5 | 5120 | `Reading the project graph slice (14441 s): taken (pass 2) 156 of 600.` | `repository:command failed with exit code 1` (dependencies-failed, about 5.7 h old) | 518 (pass 1) to 156 (pass 2): pass restart, not comparable | 1.7 (135 to 156) |
| g1-m11 | macOS 16 GB | workspace | running | 19 | 5 | 5726 | `Reading the project graph slice (13672 s): taken 349 of 600.` | `the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet` (stalled, about 4.4 h old) | 240 to 349 (+109, about 1.7/min) | 3.4 (306 to 349) |
| g1-m12 | macOS 16 GB | dependencies | running | 27 | 1 | 6163 | workspace counter frozen: `Reading the project graph slice (9369 s): taken 461 of 600.` | `the stage process was ended by signal SIGTERM / Der Stufenprozess wurde durch das Signal SIGTERM beendet` (workspace-failed, about 11 min old) | 358 to 461 (+103, about 1.6/min); stage moved to dependencies | 0 on workspace; stage moved on |
| g1-m13 | macOS (`?`) | workspace | running | 0 | 2 | 3513 | `Reading the project graph slice (1173 s): waiting for the load of this checkout that is running ahead of this one (1 load(s) of 2 workers busy; leader: …)` | `Internal error: Original Script process terminated (reference 557d4d71)` (workspace-stalled, about 12 min old) | 463 (census) to waiting on leader load; slice restarted at 1173 s | n/a (waiting), flagged |
| g1-server | coordinator (32 GB, per brief) | n/a | no `ready.json` | n/a | n/a | n/a | n/a | n/a | n/a | n/a |

## Flags (no progress or regression)

- **g1-m04** (Linux): receipt 86531 s (about 24 h) old, no progress line. Stale. Needs a manual check, no restart from this audit.
- **g1-m09**: still absent.
- **g1-m05** (Windows): receipt gap of about 7 min at snapshot 2, no progress counter. Possibly stuck in `bun:dependency-storage`.
- **g1-m03**: near zero, 2 owners in 12.5 min.
- **g1-m06** and **g1-m12**: workspace counters frozen since the census, but both moved stage (m06 to dependencies, waiting; m12 to dependencies, running). Not a stall in the stage sense.
- **g1-m07** and **g1-m13**: the slice counters were reset or are waiting on leader load. m07 restarted its slice at 284 s elapsed. m13 is waiting for the leader's load.
- **g1-m08** and **g1-m10**: pass 2 restarted after pass 1 near 500 owners. Rates are per pass 2 and are not comparable with the census.

## Summary of movement

- Owners per minute on the active graph-slice workers: m01 5.5, m08 3.4, m11 3.4, m02 2.9, m10 1.7, m03 0.2.
- Four workers now in other stages or on a slice restart: m05 and m12 running dependencies, m06 waiting in dependencies, m07 and m13 on a fresh or waiting slice.
- Receipts were fresh (under 10 s) for 9 of 12 known workers at snapshot 2. Not fresh: m04 (stale), m05 (about 7 min), m06 (about 7 min, but written at snapshot 2).

## Not verified

- The per-minute rates come from the last `taken` line in the workspace log. Pass changes and slice resets break comparability, so rates for m08, m10 and m07 are indicative only.
- RAM for hosts without `16gb` in the name is unknown. No receipt field carries it.
- Census ENOSPC on m02 (22:23) is not in the last 80 lines of its current logs, so its current state is unknown beyond the logged `exit code 1`.
- The 10-minute wait ran 12.5 minutes between snapshots (the wait was moved to background by the tool; no second snapshot was taken after snapshot 2).
