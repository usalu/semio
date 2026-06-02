---
name: micro-commit
description: >-
  Fast WIP checkpoint prep on semio dev branches. Stages all changes, writes the
  next signed commit message to commit.template (GitKraken) and COMMIT_EDITMSG,
  bumps 🚩 counter. Never commits or pushes. Use when the user sends g, go, c,
  commit, +1, bump, or @micro-commit in a dedicated session.
disable-model-invocation: true
---

# Micro Commit

Dedicated-session workflow only. Every trigger runs the **same** pipeline: read diffs → build message → `git add -A` → write template files. **Never** commit or push.

## Forbidden

Never run `git commit`, `git push`, `git push-*`, `gh pr create` (if it would push), or any command that records a commit or updates a remote.

## Speed rules

- **Shell only** (`git`, `date`). No codebase search, tickets MCP, or goals.
- One turn: parallel git reads, stage, write message, reply with message block + `git status -sb`.
- No prose unless something failed. End with: click **WIP** in GitKraken or run `git commit -S`.

## Triggers

`g`, `go`, `c`, `commit`, `+1`, `bump` (case-insensitive, optional trailing `!` / whitespace).

## Contributor

From `.repo/🧑‍💻/*/contributor.json` where `email` or `emails` matches `git config user.email`. Line 1 uses `emoji` + `alias`. Footer required:

```
Signed-off-by: {{name}} <{{email}}>
```

## Message shape

```text
{{devEmoji}}{{devAlias}}🎆{{wipYY}}🌙{{wipMM}}☀️{{wipDD}}🚩{{NNN}}
🎆{{nowYY}}🌙{{nowMM}}☀️{{nowDD}}⏰{{HH}}⌚{{mm}}⏱️{{ss}}
- {{bulletEmoji}} {{short summary from diff}}
Signed-off-by: {{name}} <{{email}}>
```

- **Line 1:** stable wip date for the series; **3-digit** `🚩` counter for the next commit (+1 from last commit line 1, or `001`).
- **Line 2:** local time now, zero-padded.
- **Bullets:** 1–8 lines; `- …` only if more than 8 logical changes.

## Counter

1. `git log -1 --format=%B` — if line 1 has `🚩[0-9]+`, reuse wip date, counter = previous + 1 (pad 3 digits).
2. Else wip date = today (local), counter `001`.

## Diff bullets

```bash
git diff --stat
git diff
git diff --name-only --diff-filter=A
git ls-files --others --exclude-standard
```

### New tickets

Added `.repo/🎫/**/ticket.json` → one bullet each, **exactly** `- {{emoji}}{{title}}` from JSON (no space after emoji). Do not use `description`/`summary` or guess from slug/diff.

### Other changes

Content-fitting emoji per change (📜 bootstrap, 🧪 tests, 🔧 fix, ♻️ refactor, 🧹 cleanup, …), not path-based. No duplicate bullet when a ticket bullet already exists.

## Branch guard

On `⛳wip` or `🏗️dev` only: stage and write template. Otherwise output message and say to switch branch; do not `git add`.

## Pipeline

```bash
git add -A
GIT_DIR="$(git rev-parse --git-dir)"
TEMPLATE="$(git config --local --get commit.template 2>/dev/null || true)"
if [ -z "$TEMPLATE" ]; then
  TEMPLATE="$GIT_DIR/gkcommittemplate.txt"
  git config --local commit.template "$TEMPLATE"
fi
MSG="$(cat <<'EOF'
{{full message including Signed-off-by}}
EOF
)"
printf '%s\n' "$MSG" > "$TEMPLATE"
printf '%s\n' "$MSG" > "$GIT_DIR/COMMIT_EDITMSG"
```

GitKraken reads `commit.template`, not `COMMIT_EDITMSG` alone. Once per machine: Preferences → Commit Template → **Apply this template to commit messages**. After run, click **WIP** to refresh the panel.

## Example

```text
🐙ueli🎆26🌙05☀️30🚩002
🎆26🌙06☀️02⏰18⌚02⏱️30
- 📜 Land Bun/Nx zero-touch monorepo bootstrap with root `script.ts`, `project.json`, and cross-platform shell entrypoints
- 🧮 Add Elements geometry play package with WASM topologic-kernel bridge and React renderer
Signed-off-by: Ueli Saluz <ueli@semio-tech.com>
```

Related: `.agents/skills/merging/SKILL.md` for 🔀 merges.
