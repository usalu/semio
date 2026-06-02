---
name: micro-commit
description: >-
  Fast WIP checkpoint prep on semio dev branches. Stages all changes, writes the
  next signed commit message to commit.template (GitKraken) and COMMIT_EDITMSG,
  bumps 🚩 counter. Never
  commits or pushes. Use when the user sends g, go, c, commit, +1, bump, or
  @micro-commit in a dedicated session.
disable-model-invocation: true
---

# Micro Commit

Use subagent `.cursor/agents/micro-commit.md` or run the same pipeline inline.

## Triggers

`g`, `go`, `c`, `commit`, `+1`, `bump` — always the same steps.

## Pipeline

1. Contributor from `.repo/🧑‍💻/*/contributor.json` + `git config user.email`.
2. Counter from last commit line 1 or `🚩001` today.
3. New `.repo/🎫/**/ticket.json` → bullets `- {{emoji}}{{title}}` from JSON only.
4. Other diff → content-fitting emoji bullets; line 2 = local `🎆…⏰…⌚…⏱️…`.
5. On `⛳wip` or `🏗️dev`: `git add -A`, write message to `commit.template` (default `.git/gkcommittemplate.txt`) and `.git/COMMIT_EDITMSG`.
6. Dev commits in GitKraken (WIP panel) or `git commit -S` locally. GK needs **Apply this template to commit messages** in Preferences.

**Never** `git commit`, **never** `git push` or any remote update.

Related: `.agents/skills/merging/SKILL.md` for 🔀 merges.
