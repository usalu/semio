---
slug: PROMPTS-MD-FORMAT-EXCLUDE
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Exclude prompts.md from formatting
model: gpt-5.2-codex
input:
  - prompt: >-
      prompts.md should be excluded from all kinds of formatting (such as by
      preflight script or format on save in vscode, etc)
    date: '2025-12-15T12:52:39.541Z'
commit: 76900221ecf5cfb30a37d69fbb66abb3e0a0e45a
files:
  read: []
  updated:
    - .prettierignore
    - .vscode/settings.json
    - AGENTS.md
    - README.md
    - hooks/prettier.ts
  removed: []
  created:
    - log/2025/12/15/PROMPTS-MD-FORMAT-EXCLUDE.md
lines:
  added: 48
  removed: 4
---
# Previously
`log/prompts.md` was formatted by Prettier (preflight + VS Code format on save).

# Plan
Exclude `**/prompts.md` via `.prettierignore` and ensure VS Code Prettier uses the same ignore file.

# Changes
- Updated `.prettierignore` to ignore `**/prompts.md`.
- Updated `.vscode/settings.json` to set `prettier.ignorePath` to `.prettierignore`.
- Updated `hooks/prettier.ts` to pass `--ignore-path .prettierignore`.
