---
slug: PROMPTS-MD-FORMAT-EXCLUDE
summary: Exclude prompts.md from formatting
prompt: >-
  prompts.md should be excluded from all kinds of formatting (such as by
  preflight script or format on save in vscode, etc)
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.959Z'
commit: '0000000000000000000000000000000000000000'
iterations:
  - prompt: >-
      prompts.md should be excluded from all kinds of formatting (such as by
      preflight script or format on save in vscode, etc)
    date:
      started: '2025-12-15T12:52:39.541Z'
    model: gpt-5-2
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 76900221ecf5cfb30a37d69fbb66abb3e0a0e45a
    files:
      updated:
        - path: .prettierignore
          lines:
            added: 10
            removed: 1
        - path: .vscode/settings.json
          lines:
            added: 10
            removed: 1
        - path: AGENTS.md
          lines:
            added: 10
            removed: 1
        - path: README.md
          lines:
            added: 10
            removed: 1
        - path: hooks/prettier.ts
          lines:
            added: 10
            removed: 1
      created:
        - log/tickets/2025/12/15/PROMPTS-MD-FORMAT-EXCLUDE.md
      removed: []
    lines:
      added: 50
      removed: 5
---

# Previously

`log/prompts.md` was formatted by Prettier (preflight + VS Code format on save).

# Plan

Exclude `**/prompts.md` via `.prettierignore` and ensure VS Code Prettier uses the same ignore file.

# Changes

- Updated `.prettierignore` to ignore `**/prompts.md`.
- Updated `.vscode/settings.json` to set `prettier.ignorePath` to `.prettierignore`.
- Updated `hooks/prettier.ts` to pass `--ignore-path .prettierignore`.
