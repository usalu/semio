# 🧯️ Memory Incident on This Worker (2026-11-24, measured 23:00)

The dev reported that this worker ran out of application memory and macOS halted it. This file holds what was measured afterwards, the cause, what was changed at once and what stays open.

## Measured at 23:00

- Machine: 16 GB RAM, 10 cores. Swap 14.7 of 15.4 GB in use (17 swap files), 7.7 GB compressed, load 37 to 39, pressure `warn`.
- Six bun processes held 28.6 GB between them (`top`, column MEM, nearly all of it compressed or swapped):

| MB | Process | Started by |
| --- | --- | --- |
| 6736 | `📊️matrix.ts` worker, one asset, running 86 min | multiview fixer |
| 5259 | `📊️matrix.ts` worker, running 52 min | multiview fixer |
| 4998 | check CLI `--asset multi-stem-birch` | fixer MV-2 |
| 4838 | `📊️matrix.ts` worker, running 32 min | multiview fixer |
| 3679 | check CLI `--asset european-beech` | plant fixer |
| 3089 | `📊️matrix.ts` worker, running 4 min | multiview fixer |

- Four idle `📊️matrix.ts` parent processes held a further 0.83 GB each.

## Cause

1. **The proof runs bypassed the memory queue.** The fixers of this ticket start their runs with the pinned bun directly, because `repo run` hangs for staffage. So none of the runs held a capacity grant. `repo capacity status` showed 2 grants with 2.8 GB reserved while 28.6 GB were in use: to the capacity system the fixers' runs are foreign consumers.
2. **Eight fixers ran at once**, each free to start a run of several GB. Nothing limited the sum.
3. **One matrix run of one asset grows to 5 to 7 GB.** The footprint rises with the run time (3.1 GB after 4 min, 6.7 GB after 86 min), so the worker keeps what it has measured instead of writing it out cell by cell. The check CLI needs 3.7 to 5 GB for one large tree. Both are far too large for one asset: this is the carving fault.
4. **The warden did not stop it.** It had suspended 8, resumed 7 and cancelled 2 processes by 23:00, all of them repo-owned ones. The six processes above are outside what it acts on.

The coordinator of this ticket is responsible for 1 and 2: the brief told the fixers to use the pinned bun and set no limit on parallel runs.

## Changed at once

- All eight running fixers were told (23:02) to start every proof run only through `repo capacity run --memory=<MB> -- …` (7000 for the matrix, 5000 for the check CLI, 2500 for vitest suites, 1000 for scans), one heavy run per fixer. Runs in flight were not killed. `repo capacity run` was tested first: a 64 MB request waited 8 s behind the crowded swap and was then admitted.
- `older/suite.sh` and `check-categories.sh` of this ticket now go through the same queue.
- `📋️multiview-brief.md` and `🤝️handoff.md` carry the rule for every later fixer.

## Open, not done here

- **Matrix worker memory** (`S/🧪️tests/🔭️multiview-coverage/📊️matrix.ts` and its worker): write cells out as they are measured and release them, so one asset stays below a fixed budget. Belongs to the multiview ticket.
- **Check CLI memory** on large plants (`european-beech`, `multi-stem-birch`, `weeping-willow` with its 9.5 MB file): find what holds 4 to 5 GB.
- **Warden and foreign consumers**: a process tree started from the workspace without a grant should be treated as a subject (suspend first, cancel last) once the guard line is crossed, or the start itself should be refused. The host-capacity module was being edited by someone else while this was written (`🛡️warden` at 21:58, `📏️decision` at 23:00; for a few seconds `repo capacity status` failed with `hostSessionCapMegabytesV1 is not defined`), so nothing was changed there from this ticket.
- **`repo run` for staffage** hangs; that is the reason the fixers left the queue in the first place. With it repaired, the direct bun calls can go.
