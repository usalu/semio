---
slug: COMMENT-STRING-DETECT
prompt: inside strings are wrongly identified as comments.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-03T02:11:20Z"
commit: 757a4d5aa0cf14f288561eed3253d5195a96e75e
iterations:
    - prompt: inside strings are wrongly identified as comments.
      model: ""
      date:
        started: "2026-01-03T02:11:20Z"
        ended: "2026-01-03T02:18:00Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 757a4d5aa0cf14f288561eed3253d5195a96e75e
      files:
        updated:
            - path: AGENTS.md
              lines:
                added: 34
                removed: 8
            - path: README.md
              lines:
                added: 7
                removed: 4
            - path: go/repo/main.go
              lines:
                added: 490
                removed: 188
            - path: js/vscode/extension.ts
              lines:
                added: 78
                removed: 22
            - path: prompts/ueli.md
              lines:
                added: 31
                removed: 0
        created:
            - path: tickets/2026/01/03/ITERATION-LINES.md
              lines:
                added: 61
                removed: 0
            - path: tickets/2026/01/03/VIOLATION-TREE-STRUCTURE.md
              lines:
                added: 15
                removed: 0
            - path: tickets/2026/01/03/VSCODE-TICKET-TOGGLE.md
              lines:
                added: 33
                removed: 0
        removed: []
      lines:
        added: 749
        removed: 222
---
# Previously

# Plan

# Changes