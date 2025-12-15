---
date:
  created: '2025-12-14T21:46:55.100Z'
  updated: '2025-12-14T21:49:44.428Z'
slug: LOG-PROMPTS-STATS
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Extend log.ts prompts/model/stats
model: gpt-5.2-codex
prompts:
  - >-
    Extend the log.ts functionality. Enhance the script and documentation.
    Include in the frontmatter: prompts an array of all the prompts provided by
    the user. Whenever the user sends a new prompt append it to the array.
    log.ts must take model name (of the llm) as forced input (enum). Include
    stats in the frontmatter and add a command to update stats (affected files,
    total added/removed lines) recomputed across multiple prompts.
commit: 687798cd73468e4a7e60722f49f21a32c9a0a44f
affectedFiles:
  - AGENTS.md
  - README.md
  - log/2025/12/14/LOG-PROMPTS-STATS.md
  - scripts/log.ts
lines:
  added: 419
  removed: 20
---
# Previously
`scripts/log.ts` created logs with minimal frontmatter (date/slug/author/summary/model), with model defaulting from env and no support for tracking user prompts or task-scoped git stats.

# Plan
Extend log frontmatter with `prompts` and `stats`.
Make `--model` and an initial `--prompt` required on `create` and validate models via an enum.
Add CLI commands to append prompts and to maintain task-scoped affected files and git line stats.
Update `README.md` and `AGENTS.md` to describe the mechanism and CLI usage.

# Changes
Extended `scripts/log.ts` frontmatter with `prompts` and `stats` (base commit, affected files, +/− lines, updatedAt).
Added CLI commands: `prompt`, `files`, `stats`, and `models` and made `create` require `--model` + `--prompt`.
Documented the updated log workflow in `README.md` and `AGENTS.md`.
