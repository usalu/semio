---
slug: TICKET-ITERATION-FILE-FILTER
prompt: Fix ticket iteration end to only calculate lines for files that were declared when the iteration started, not all files from git.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-05T11:41:39Z"
commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
model: claude-opus-4-5
iterations:
    - prompt: Fix ticket iteration end to only calculate lines for files that were declared when the iteration started, not all files from git.
      model: claude-opus-4-5
      date:
        started: "2026-01-05T11:41:39Z"
        ended: "2026-01-05T11:50:30Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
      files:
        updated:
            - path: go/repo/main.go
              lines:
                added: 201
                removed: 35
      lines:
        added: 201
        removed: 35
---
# Previously

# Plan

# Changes