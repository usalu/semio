---
slug: TICKET-FILES-ONLY
summary: Restrict ticket files and aggregate stats
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T16:09:53.578Z'
  finished: '2025-12-16T16:25:36.733Z'
commit: c44e5e38193be007ca56cc649aa2f58238c1ec40
iterations:
  - prompt: >-
      Only allow files to be created, updated and deleted files. Create ticket
      shouldnt create an iteration. Iteration need files. Add author and date to
      ticket from git. Once finished, combine all the files from all iterations
      and add it as extra field to the ticket. Use git one last time to compute
      the lines.
    date:
      started: '2025-12-16T16:09:53.578Z'
      ended: '2025-12-16T16:25:23.282Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: c44e5e38193be007ca56cc649aa2f58238c1ec40
    files:
      updated:
        - path: scripts/log.ts
          lines:
            added: 701
            removed: 253
        - path: README.md
          lines:
            added: 18
            removed: 9
        - path: AGENTS.md
          lines:
            added: 113
            removed: 59
        - path: log/2025/12/16/TICKET-FILES-ONLY.md
          lines:
            added: 34
            removed: 0
      created: []
      removed: []
    lines:
      added: 866
      removed: 321
files:
  updated:
    - AGENTS.md
    - README.md
    - log/2025/12/16/TICKET-FILES-ONLY.md
    - scripts/log.ts
  created: []
  removed: []
lines:
  added: 888
  removed: 321
---
# Previously

- Ticket creation wrote an initial iteration and allowed `read` file tracking.
- Ticket finish only toggled status without aggregating files or computing ticket-level line totals.

# Plan

- Restrict iteration file tracking to `updated`, `created`, and `removed` only.
- Make ticket creation create a ticket only (no iteration) and store `author`, `created`, and `base` from git.
- On ticket finish, aggregate all iteration files into ticket-level `files` and compute ticket-level `lines` from git diff against `base`.
- Update dev docs to reflect the new ticket workflow and schema.

# Changes

- `scripts/log.ts` no longer supports `read` file tracking; only `updated`, `created`, and `removed` are accepted and persisted.
- `ticket create` now creates a ticket without an iteration and records ticket-level `author`, `created`, and `base` from git.
- `ticket finish` now aggregates all iteration files into ticket-level `files` and recomputes ticket-level `lines` from git diff against the ticket `base` commit.
- `README.md` and `AGENTS.md` document the updated ticket workflow and fields.
