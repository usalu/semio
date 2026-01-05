---
slug: TICKET-SUMMARY-ON-FINISH
prompt: The summary flag should be only available and forced when closing a ticket.
summary: Require summary only when finishing tickets
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-18T15:51:06.030Z"
    finished: "2025-12-18T15:52:32.230Z"
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: claude-opus-4-5
iterations:
    - prompt: Move ticket summary capture to ticket finish and require --summary only when closing.
      model: claude-opus-4-5
      date:
        started: "2025-12-18T15:52:10.403Z"
        ended: "2025-12-18T15:52:21.179Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
      bundles:
        '@semio':
            files:
                "":
                    sections: {}
      files:
        updated:
            - path: ""
            - path: ""
            - path: ""
            - path: ""
      lines:
        added: 232
        removed: 90
bundles:
    '@semio':
        files:
            "":
                sections: {}
files:
    updated:
        - path: ""
        - path: ""
        - path: ""
        - path: ""
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
