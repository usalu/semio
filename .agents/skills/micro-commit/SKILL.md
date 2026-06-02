---
name: micro-commit
description: >-
  Fast WIP micro-commit messages for semio dev branches. Bumps the 🚩 counter,
  keeps the parent WIP header line, stamps ⏰⌚⏱️ second, and summarizes git diffs.
  Use when the user says g, go, c, commit, +1, bump, or @micro-commit in a
  dedicated session.
disable-model-invocation: true
---

# Micro Commit

Delegate to the **micro-commit** subagent (`.cursor/agents/micro-commit.md`) or follow the same steps inline.

## When

- Dedicated chat kept open for rapid checkpoints while coding
- User sends only: `g`, `go`, `c`, `commit`, `+1`, `bump`, `msg`, `preview`, `new`, `reset`

## Procedure

1. Resolve contributor from `.repo/🧑‍💻/*/contributor.json` via `git config user.email`.
2. Parse last commit body line 1 for `🚩NNN` or start `001` with today's wip date.
3. `git diff` / `git diff --cached` → 1–8 `- {{emoji}} {{summary}}` bullets.
4. Line 2 = local `🎆YY🌙MM☀️DD⏰HH⌚mm⏱️ss`.
5. On commit triggers: `git add -u` (or `-A` if asked), `git commit` with heredoc message, on `⛳wip` or `🏗️dev` branches only.

Full format and emoji table: `.cursor/agents/micro-commit.md`.

Related: `.agents/skills/merging/SKILL.md` for squash merges (🔀), not micro-commits.
