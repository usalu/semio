---
slug: TICKET-REOPEN-API
summary: Add reopenTicket function to ticket API
prompt: >-
  Extend ticket api to be able to reopen a ticket. This should remove the total
  files and lines from the ticket (not from the individual iterations) and set
  the status to open.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-17T18:39:01.076Z'
  finished: '2025-12-17T18:39:54.680Z'
commit: 7c4820638369104ae259b238d9240f08e429e67e
model: claude-opus-4-5
iterations:
  - prompt: >-
      Extend ticket api to be able to reopen a ticket. This should remove the
      total files and lines from the ticket (not from the individual iterations)
      and set the status to open.
    date:
      started: '2025-12-17T18:39:35.734Z'
      ended: '2025-12-17T18:39:46.869Z'
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 7c4820638369104ae259b238d9240f08e429e67e
    files:
      updated:
        - scripts/log.ts:
            lines:
              added: 33
              removed: 3
        - README.md:
            lines:
              added: 18
              removed: 0
      created: []
      removed: []
    lines:
      added: 51
      removed: 3
files:
  updated:
    - README.md:
        lines:
          added: 18
          removed: 0
    - scripts/log.ts:
        lines:
          added: 33
          removed: 3
  created: []
  removed: []
lines:
  added: 51
  removed: 3
---
# Previously

# Plan

# Changes
