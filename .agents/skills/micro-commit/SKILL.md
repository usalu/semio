---
name: micro-commit
description: >-
  Instant WIP micro-commit. Triggers g, go, c, commit, +1, bump — execute immediately, no reasoning.
  Default level from micro-commit.json (prepare-only). Reply with commit message only in one ``` fence; preserve every newline from stdout.
---

# Micro Commit

**Not bundle commit** — no bundle scope lines, no `🎆YY🌙MM☀️DD` day sections, no per-bundle or per-day `📊uloc` on headers. One timestamp line (`🎆…⏰…`) plus bullets; script adds a single **`📊uloc` footer** at the end. For squashed bundles with per-area and per-day uloc, use `.agents/skills/commit/SKILL.md`.

## Rule (always)

**Any trigger (`g`, `go`, `c`, `commit`, `+1`, `bump`, `@micro-commit`) = do it now.**

- **No** planning, preamble, recap, or postamble.
- **No** visible reasoning, todos, or questions.
- **No** describing what you are about to do.
- **One** assistant turn: run the commands below, then reply with **only** the final `prepare` stdout inside a single ` ``` ` fenced block (or stderr on failure, same fence).

## Level (set once, then repeat)

1. **First trigger in this chat** (or if the user named a mode): read `.repo/🧑‍💻/{alias}/micro-commit.json` → `level` (default `prepare-only`). Remember it for this conversation.
2. **Every later trigger** (`g`, `go`, `c`, …): use **the same level** again. Do not re-read config or re-ask.
3. **Override only** when the **current** user message contains `gc` / `commit!`, `gp` / `push!`, or `g.` / `prepare!` — pass that token on the `prepare` line.

## Execute (every trigger, no variation)

Run these **two** shell steps in the same turn (no text between them):

```bash
bun ./script.ts micro-commit stage && bun ./script.ts micro-commit diff
```

Read the diff **silently** (do not paste it to the user). Write 1–8 bullets from **this** diff only — not chat memory. **Cover every staged area** that matters (see below)—the script rejects commits that skip an entire area. **Sort by impact**: highest-impact bullet first, then descending (never alphabetical or file-path order). For each line, choose **whatever single emoji best fits that specific change** (no fixed palette).

```bash
bun ./script.ts micro-commit prepare [gc|gp|g.] <<'EOF'
{emoji}{highest-impact change from the diff}
{emoji}{next change}
EOF
```

(`prepare` already runs `git add -A`; `stage` keeps the index explicit. Success prints **only** the commit message on stdout.)

## Your reply

| Result | Reply |
|--------|-------|
| `prepare` exit 0 | **Only** one fenced block with **stdout alone** (subject, timestamp, bullets, blank lines, **📊uloc** block, Signed-off-by). **No** title, **no** `##` headers, **no** `[micro-commit]` lines, **no** staged path lists, **no** prose outside the fence. |
| exit non-zero | Re-run `prepare` **without** `2>/dev/null` if needed; reply with **only** one ` ``` ` fence around the error text. No headers or commentary. |

### Newlines (required — GitKraken / `git commit` need them)

- **One stdout line = one line in the fence.** Never collapse the message into a single paragraph or one long wrapped line.
- Paste **verbatim**: same order, same `\n`, same **blank lines** stdout prints (e.g. empty line before `📊uloc`, empty line before `Signed-off-by:`).
- **Forbidden:** joining lines with spaces; bullet lists with `-` prefixes; hard line breaks only at 80 cols; “cleaning up” spacing; omitting empty lines stdout included.
- Opening ` ``` ` on its own line → **raw multiline body** → closing ` ``` ` on its own line.

Example shape (each line is its own line in the fence):

```
🐙ueli🎆26🌙06☀️02🚩081
🎆26🌙06☀️04⏰00⌚02⏱️50
🪣First bullet from stdout
📊uloc➕12✏️3➖1🟰16
🟦803k

Signed-off-by: Name <email@example.com>
```

## Bullets (you)

- From **current** `diff` output only; precise; not path-only (`Update foo.ts` forbidden).
- **Cover every staged area (required):** one bullet minimum per area present in the diff. Areas the script checks:
  - **`.cursor/plans/`** — Cursor plans are **high impact**; summarize what the plan decides or enables (read the diff, not the filename alone).
  - **`framework/` / `puzzle/` / `compose/` / `cad/` / `ui/` / …** — product/runtime behavior.
  - **`repo/`** — hooks, CLI, MCP, micro-commit, tooling.
  - **`.agents/`** — skills/rules (except pure `SKILL.md` tweaks bundled with larger work—still mention the skill change if it is part of the story).
  - **`.devcontainer/`** — environment/bootstrap.
  - **Anything else staged** — include it; do not cherry-pick only “code” and drop plans, config, or infra.
- **Order by impact (required):** line 1 = biggest user or system effect; each next line = strictly lower impact. A new **`.cursor/plans/*.plan.md`** often belongs in the top 1–2 bullets when it defines the approach for the rest of the diff.
- New tickets: script adds `{emoji}{title}` from `ticket.json` — do not duplicate.
- Format: `{emoji}{summary}` — each line **starts with the emoji** (no leading `-`), **no space** after the emoji (words inside the summary may use spaces). Example: `🗺️Fix north-up tile affine`, not `- 🗺️ Fix …` or `🗺️ Fix …`.
- **Emoji choice:** one leading emoji per bullet — the **most accurate** for *that* description (read the diff; vary emojis across lines). There is **no** approved list.
- **Reserved (never on bullets):** `🎆` (calendar timestamp line only), `📊` / `🔢` (uloc block), `🚩` (subject counter). Do not paste `🎆YY🌙MM☀️DD…` date patterns into bullets.

## Script (deterministic)

Counter, timestamp line, bullets, **📊uloc** metrics (first line is **`📊uloc➕…✏️…➖…🟰…`** — staged git deltas for the whole repo, **🟰** = ➕+✏️+➖; then **every** language with repo `{loc}` plus the same delta shape, largest first), Signed-off-by, validation, GitKraken files. You only author bullets; do not hand-write the metrics block.

After commit, hooks reset to an **empty** `.git/gkcommittemplate.txt` and point `commit.template` at it (GitKraken reuses the last message if that file is missing). Run `g` / `prepare` for the next change set. `bun ./script.ts setup git` installs hooks (post-commit, post-checkout, post-merge, post-rewrite, prepare-commit-msg).

## GitKraken (do not mention unless commit fails)

Line 1 = `…🚩NNN`; rest = timestamp, bullets, Signed-off-by. Reopen Commit panel or **WIP** if stale.

## Branch

Only `⛳wip` or `🏗️dev`.

## Forbidden

Reasoning in the open, codebase search, tickets MCP, goals, `gh pr create`, hand-editing counter/timestamp/Signed-off-by, inventing bullets without reading this turn's diff, bullets sorted by file name or diff hunk order instead of impact, **omitting whole staged areas** (especially `.cursor/plans/` or repo infra) while only describing unrelated tweaks, leading `-` on bullets, a space after the emoji (`🗺️ Fix …`), reusing one emoji for every bullet when the changes differ, **reserved lead emojis** (`🎆` `📊` `🔢` `🚩`) or copying the `🎆YY🌙MM☀️DD` date pattern into bullets, **stripping or merging newlines** in the fenced commit message reply.

Related: `.agents/skills/merging/SKILL.md`, `.agents/skills/commit/SKILL.md`.
