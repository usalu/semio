---
slug: TICKET-SCRIPT-SEMANTICS
prompt: Improve log script semantics. Rename logs to tickets. create log becomes ticket create. Then a new command is ticket iteration start Then there should be ticket iteration finish finish becomes ticket finish Throw an error if an iteration is unifinished for a ticket (e.g. when another iteration start or ticket finish is called) Force files to be a necessary parameter to call for iteration start and iteration finish. Update the file list on finish for the iteration and compute stats (lines).
summary: Rename log workflow to ticket+iterations
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-16T15:26:35.793Z"
    finished: "2025-12-16T15:59:31.839Z"
commit: "0000000000000000000000000000000000000000"
iterations:
    - prompt: Improve log script semantics. Rename logs to tickets. create log becomes ticket create. Then a new command is ticket iteration start Then there should be ticket iteration finish finish becomes ticket finish Throw an error if an iteration is unifinished for a ticket (e.g. when another iteration start or ticket finish is called) Force files to be a necessary parameter to call for iteration start and iteration finish. Update the file list on finish for the iteration and compute stats (lines).
      model: gpt-5-2
      date:
        started: "2025-12-16T15:26:35.793Z"
        ended: "2025-12-16T15:59:24.768Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: c44e5e38193be007ca56cc649aa2f58238c1ec40
      bundles:
        '@semio':
            files:
                AGENTS.md:
                    sections:
                        _root:
                            lines:
                                added: 107
                                removed: 59
                README.md:
                    sections:
                        _root:
                            lines:
                                added: 16
                                removed: 7
                log/tickets/2025/12/16/TICKET-SCRIPT-SEMANTICS.md:
                    sections:
                        _root:
                            lines:
                                added: 35
                                removed: 0
                scripts/log.ts:
                    sections:
                        _root:
                            lines:
                                added: 693
                                removed: 249
      files:
        updated:
            - path: scripts/log.ts
              lines:
                added: 693
                removed: 249
            - path: README.md
              lines:
                added: 16
                removed: 7
            - path: AGENTS.md
              lines:
                added: 107
                removed: 59
            - path: log/tickets/2025/12/16/TICKET-SCRIPT-SEMANTICS.md
              lines:
                added: 35
                removed: 0
      lines:
        added: 851
        removed: 315
---


# Previously

- `scripts/log.ts` exposed a log-centric workflow (`create`, `update`, `finish`) where new iterations could be appended before finishing the previous one.
- The latest-iteration finish step accepted optional file lists and computed per-file line stats from git diffs.

# Plan

- Add ticket command hierarchy and ticket finish semantics.
- Enforce iteration lifecycle (no overlapping iterations).
- Require file lists for iteration start/finish and persist them on start/finish.
- Update dev docs to ticket terminology and workflow.

# Changes

- `scripts/log.ts` now exposes `ticket create`, `ticket iteration start`, `ticket iteration finish`, and `ticket finish` while keeping legacy aliases.
- `updateLog` now represents iteration start, requires file lists, and refuses to start if the latest iteration is unfinished.
- `finishIteration` now requires file lists, overwrites the iteration file lists on finish, records `finished`, and computes per-file line stats.
- Tickets can be finished via `finishTicket` only when the latest iteration is finished; ticket frontmatter stores `status`, `created`, and `finished`.
- `README.md` and `AGENTS.md` document the ticket workflow and constraints.
