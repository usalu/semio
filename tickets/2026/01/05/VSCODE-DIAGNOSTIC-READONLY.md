---
slug: VSCODE-DIAGNOSTIC-READONLY
prompt: Fix VSCode diagnostics opening files as read-only preview instead of editable files when clicking on semio violation diagnostics.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-05T11:12:30Z"
commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
model: claude-opus-4-5
iterations:
    - prompt: Fix VSCode diagnostics opening files as read-only preview instead of editable files when clicking on semio violation diagnostics.
      model: claude-opus-4-5
      date:
        started: "2026-01-05T11:12:30Z"
        ended: "2026-01-05T11:19:30Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
      files:
        updated:
            - path: contributors/usalu/contributor.json
              lines:
                added: 3
                removed: 2
            - path: go/repo/main.go
              lines:
                added: 177
                removed: 11
            - path: js/vscode/extension.ts
              lines:
                added: 10
                removed: 4
            - path: prompts/ueli.md
              lines:
                added: 3
                removed: 1
            - path: reports/playwright.json
              lines:
                added: 23
                removed: 549
      lines:
        added: 216
        removed: 567
    - prompt: Continue investigating - the read-only preview issue persists. Need to find the root cause.
      model: claude-opus-4-5
      date:
        started: "2026-01-05T11:56:37Z"
        ended: "2026-01-05T12:00:03Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
      declared:
        updated:
            - path: js/vscode/extension.ts
      files:
        updated:
            - path: js/vscode/extension.ts
              lines:
                added: 17
                removed: 7
      lines:
        added: 17
        removed: 7
---
# Previously

# Plan

# Changes