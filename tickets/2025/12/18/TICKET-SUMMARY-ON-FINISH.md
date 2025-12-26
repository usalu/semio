---
slug: TICKET-SUMMARY-ON-FINISH
prompt: The summary flag should be only available and forced when closing a ticket.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-18T15:51:06.030Z"
  finished: "2025-12-18T15:52:32.230Z"
summary: Require summary only when finishing tickets
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: claude-opus-4-5
iterations:
  - prompt: >-
      Move ticket summary capture to ticket finish and require --summary only
      when closing.
    date:
      started: "2025-12-18T15:52:10.403Z"
      ended: "2025-12-18T15:52:21.179Z"
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - scripts/log.ts:
            lines:
              added: 133
              removed: 62
        - README.md:
            lines:
              added: 16
              removed: 10
        - AGENTS.md:
            lines:
              added: 28
              removed: 18
        - log/tickets/2025/12/18/TICKET-SUMMARY-ON-FINISH.md:
            lines:
              added: 55
              removed: 0
      created: []
      removed: []
    lines:
      added: 232
      removed: 90
files:
  updated:
    - AGENTS.md:
        lines:
          added: 28
          removed: 18
    - README.md:
        lines:
          added: 16
          removed: 10
    - log/tickets/2025/12/18/TICKET-SUMMARY-ON-FINISH.md:
        lines:
          added: 57
          removed: 0
    - scripts/log.ts:
        lines:
          added: 133
          removed: 62
  created: []
  removed: []
lines:
  added: 234
  removed: 90
---

# Previously

Ticket creation required a summary even though the summary is only needed when the ticket is finished.

# Plan

- Remove summary from `ticket create`.
- Require `--summary=...` only on `ticket finish`.
- Store `summary` in ticket frontmatter on finish.
- Update developer docs to match the CLI.

# Changes

- Updated `scripts/log.ts` to remove summary from `ticket create` and require `--summary=...` on `ticket finish`.
- Updated `README.md` and `AGENTS.md` ticket usage examples and schema description.
