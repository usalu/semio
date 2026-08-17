---
name: commit
description: >-
 Bundle micro-commits into one squash commit. Triggers C, bundle, squash — execute immediately.
 Agent must analyze last bundle state and the full diff, decide bundle attribution (not automatic), then write new summaries.
 Prepare stdout: four git fences, tag name, full commit message (six blocks total).
---

# Commit (bundle)

**Bundle squash only** — not `micro-commit`. Micro-commits have no bundle scopes or per-day sections; they keep one `📊️metric` block at the end of each message. Per-bundle and per-day inline `📊️metric📃uloc…📊️metric💾size…` apply only here (`bun ./📜️script.ts commit prepare`).

## Rule (always)

**Any trigger (`C`, `bundle`, `squash`, `@commit`) = do it now.**

- **No** planning, preamble, recap, or postamble.
- **No** visible reasoning, todos, or questions.
- **One** assistant turn: analyze, run `prepare`, reply with **only** full `prepare` stdout (six fenced blocks).

## What the script does vs you

| Script (automatic)                                                                                                                     | You (from analysis)                                                                                  |
| -------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| Subject `🐙️…🔀️`                                                                                                                        | —                                                                                                    |
| Per-bundle and per-day inline `📊️metric📃uloc…📊️metric💾size…` from git **after** you name bundles (`🟰️` = ➕️+✏️+➖️); **bundles sorted highest 🟰️ first** | **Which bundles exist**, bundle scope lines (`🏘️compose✍️sketchpad`), **which changes belong where** |
| Footer `📊️metric` block (`📃uloc` + `💾size` totals, then per-language pairs) + `Signed-off-by`                                                                    | Date lines `🎆️YY🌙️MM☀️DD` (no metrics on stdin)                                                         |
| Four `git` commands on prepare stdout                                                                                                  | **New** bullets `{emoji}{description}`                                                               |

**Nothing** in the script reads prior commit bullets and auto-fills the bundle body. Prior messages may be wrong or another format — **never** copy them.

## Bundle attribution (you only — not automatic)

**Bundles, folder membership, and bullets cannot be computed automatically.** Bundle folders and repo layout **change** between WIPs (moves, renames, new areas, split/merged bundles). The script only maps **git numstat** to bundle labels **you already wrote** on stdin — it does **not** decide which bundles exist or which files belong to which bundle.

**You must:**

1. **`commit log`** — read the **last** bundle/WIP squash and micro-commits since then (what was shipped before, rough dates). Context only; do not copy bullets.
2. **`commit diff`** — read **`--stat` and the full diff** for every changed path (files and folders). This is what actually changed.
3. **Decide attribution** — from that evidence, choose emoji bundle scopes, group work by `🎆️YY🌙️MM☀️DD`, and write **new** impact bullets per bundle/day. If a path could fit multiple bundles, pick the one that matches **this** diff’s intent, not an old folder map from memory.

Do not assume yesterday’s bundle boundaries still apply. Do not let path-token heuristics replace reading the tree.

**Metrics constraints (enforced on `check` and `prepare`):**

| Level                    | Must sum (➕️ ✏️ ➖️ 🟰️)              |
| ------------------------ | ----------------------------------- |
| Each bundle’s `🎆️` days  | That bundle’s scope header metrics (`📃uloc` and `💾size`) |
| All bundle headers       | Footer `📊️metric` totals (full WIP range) |
| Each footer language row | Same footer totals per kind (`📃uloc`, `💾size`)     |

Every changed path must belong to **exactly one** bundle. If days, bundles, or languages do not add up, attribution is wrong — fix scopes/dates and re-run check.

## Level (set once, then repeat)

1. Read `.repo/🧑️‍💻️/{alias}/commit.json` → `level` (default `prepare-only`).
2. **Prepare-only reply:** paste **all** of stdout verbatim — **six** fenced blocks in order (see below).

## Execute (prepare-only) — four steps

### 1. Prior commits (context only)

```bash
bun ./📜️script.ts commit log
```

Read **stderr**: every micro-commit since the last `…🔀️` WIP. Use only to see **what happened when** (dates, rough themes). **Do not** reuse bullet wording — messages may be mistaken or obsolete.

### 2. Git diff (source of truth)

```bash
bun ./📜️script.ts commit diff
```

Read **stderr**: `git diff --stat` and full `git diff` for `lastWIP..HEAD`. **All bundle bullets must come from this patch** — what files and behavior actually changed.

Optional single command: `bun ./📜️script.ts commit analyze` (= log + diff).

### 3. Check (validate attribution)

```bash
bun ./📜️script.ts commit check <<'EOF'
…bundle body…
EOF
```

Exit **0** on stderr: `commit check: OK`. Exit **1** with which constraint failed. Run after drafting the body, before `prepare`.

### 4. Prepare (your new summary)

```bash
bun ./📜️script.ts commit prepare <<'EOF'
🏘️compose✍️sketchpad
🎆️26🌙️06☀️04
🗺️Summarize highest-impact change you see in the diff that day
🎆️26🌙️06☀️03
🧪️Next change from diff for that date

🖱️ui⚛️react
🎆️26🌙️06☀️02
🖥️Another area from diff
EOF
```

**Do not** pass `ct` / `cs` / `cp` unless the user asked to run git steps.

## Bundle body rules

- **Scope:** emoji + area name (`🏘️compose✍️sketchpad`, `🥅️framework`, `🖱️ui⚛️react`) — no paths, no `🔀️` / `📊️metric`. You choose scopes after analyzing log + diff; stdin order does not matter — the script reorders bundles by **highest 🟰️** once scopes exist.
- **Dates:** group bullets by calendar day (`🎆️YY🌙️MM☀️DD`), **newest first** within each bundle. Script appends inline `📊️metric…` to each date line from micro-commits that day (same bundle paths).
- **Bullets:** impact order; one leading emoji; written **after** reading the diff.
- **Never** paste subject, metrics block, or `Signed-off-by` on stdin.
- **Rejected:** bullets that verbatim-match a line from a prior commit in the range.

## Your reply (prepare-only)

Paste **entire** `prepare` stdout verbatim — **six** fenced blocks, in order:

1. `git tag -s -m '…🚩️' '…🚩️' HEAD`
2. `git reset --soft <sha> && git commit -S -F '.git/compose-commit-message'`
3. `git push --follow-tags`
4. One chained line (tag + squash + push)
5. **Tag name only** — e.g. `🐙️ueli🎆️26🌙️06☀️04🚩️` (single line in the fence)
6. **Full commit message** — subject `…🔀️`, bundles, `📊️metric`, `Signed-off-by` (exact script output)

- Blocks 1–4 are shell commands to copy-run; blocks 5–6 are for review / GitKraken.
- Do not merge, reorder, omit, or re-wrap; do not add prose outside the fences.

## Optional git steps

`ct` / `cs` / `cp` on `prepare` only when the user requests it; clean working tree required.

## Branch

Only `⛳️wip` or `🏗️dev`.

## Forbidden

Copying bullets from `commit log`, skipping `commit diff`, inventing changes not in the patch, **guessing bundles without reading all changed paths**, trusting stale folder maps instead of the current diff, omitting the tag-name or commit-message block at the end, merging six blocks into one.

Related: `.agents/skills/micro-commit/SKILL.md`.
