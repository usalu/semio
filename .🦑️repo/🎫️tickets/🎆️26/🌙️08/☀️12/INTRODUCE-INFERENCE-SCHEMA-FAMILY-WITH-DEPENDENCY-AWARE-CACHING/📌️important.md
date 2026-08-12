# Important — read first, every agent

Ticket: `26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`, issue #2546, goal `🎯aioptimizedrepo`.
Approved plan: `/Users/ueli/.claude/plans/finish-introduce-inference-schema-family-iridescent-sprout.md`. Design source: `/Users/ueli/.claude/plans/introduce-inferences-to-every-elegant-reddy.md`.

## The thesis (one paragraph)

Add a fourth schema family — `💡️inferences` (plural, slug-dir shape mirroring `🧬️mutations`) — to every owning artifact subset (~107), with real dependency-aware merkle caching (`InferredField`, `InferenceCache`, content-addressed dep-hash chains). Framework spine + puzzle3d pilot already shipped session 1. 72/107 subsets already got the fan-out from an earlier session today; this session finishes the remaining ~35 subsets, two anti-pattern removals (puzzle5d snapshot slimming, trinity `recompute_derived` deletion), the script.ts policy cluster, and the taxonomy/discovery flip.

## Hard rules (non-negotiable, repo-wide CLAUDE.md + local precedent)

1. **No git-modifying commands, ever** — no `commit`, `stash`, `checkout`, `reset`, `rebase`, no worktrees. The tree is shared with live human and agent sessions. `isolation: "worktree"` is forbidden on every Agent/Workflow call.
2. **The repo AUTO-COMMITS** (commits look like `🐙️ueli🎆️26🌙️06☀️04🚩️<n>`, incrementing). `git status` is therefore **not** a churn detector: work that landed minutes ago shows clean, and a vanished edit was committed, not lost. Detect churn with `git log --oneline -5 -- <path>`, `stat -f '%Sm' <path>`, and `git log --oneline -3`. Never run a git command to "recover" anything.
3. **All temp files, logs, scratch, reports go inside this ticket folder.** Scratch logs are `.txt` or `.md`, **never `.log`** (`*.log` is repo-gitignored and silently drops out of `ticket_close`'s file list).
4. **Never call `ticket_close` or `ticket_reopen`.** Only the coordinator closes this ticket, with an explicit path.
5. **Never run bare `cargo check` / `cargo build` / `cargo test`.** Always scope `-p <crate>`, always with `CARGO_TARGET_DIR=<this-ticket>/🎯️target`. "Blocking waiting for file lock on build directory" is normal; wait, do not kill it. A red workspace is not a red crate.
6. **`bun` + `nx` only.** Permanent scripts live in `📜️script.ts` at the relevant directory; `project.json`/`package.json` only call it. Never create another script file.
7. **Greenfield.** No compatibility layers, no legacy support, no deprecations, no migration scripts. Delete and handcraft the replacement.
8. **Regions** (`//#region`/`//#endregion`, `pub mod` in Rust) for structure. Extend existing files; no new test files (extend existing); no example files (implement in dependent parts).
9. **Docstrings start with a unique fitting emoji.** No comments inside definitions.
10. **Never claim a test passed without running it.** Paste real command + real output into your report.
11. **Derive dual-copy mirroring**: `✨️derive/🦀️component.rs` and `✨️derive/📦️packages/🦀️rust/📦️glue.rs` must stay byte-identical — cargo compiles the glue copy.
12. **Grammar-honesty**: no placeholder/lorem text in 📝️text or 💾️binary grammar leaves — real, artifact-specific vocabulary only.
13. **No empty inference families** — every `💡️inferences/` ships ≥1 slug dir with a real derivation: either an `InferredField` impl (honest `dep_input` covering everything `compute` reads) **or** a pure-fn leaf reading the snapshot directly (architect `🧭topology` is the sanctioned exemplar). **`InferredField` is required only where the derivation is genuinely per-entity and DAG-shaped** (a merkle dep-chain over a flat whole-snapshot record costs more than the fold it would cache) — see the P0 ruling in `📓️status.md` for the resolved 8-vs-72 question; this line originally overstated the requirement and is corrected to match the approved plan's P2 checklist wording ("real `InferredField`/pure-fn leaf").

## Live concurrency — THREE other sessions are in this tree

| Session | Ticket | Owns |
|---|---|---|
| SMO | SEMANTIC-MUTATIONS-OVERHAUL #2545 | ~21 plugin lanes: app `🦀️component.rs`, `🎮️commands/**`, panels, engines, per-plugin `📦️glue.rs`. Currently editing **trinity** — our W-B surface. |
| UCAS | UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM #2548 | **stdio** (transiently red mid-rename) + framework `🔌️plugin/🦀️component.rs`, `📡️spr`, `🧬️schema`, kernel `💡️inference` module. |
| APA | ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE #2549 | `🔣️taxonomy.json`, `📜️script.ts` write order (APA → UCAS-W6 → SMO → **us, last**), and is mid-work on **puzzle** — our W-B ◻2d/5d surface. |

Contact peers via `ListAgents` (peer sessions show as "Peer sessions"); use `SendMessage` to their session names. Coordinate before entering trinity, puzzle, or stdio. Never "fix" another session's file — treat red plugin crates as churn: retry `-p <crate>` up to 3× at 60s intervals, then record under `## Concurrent-churn observations`.

> ⚠️ **This table is stale in two ways — see `📓️status.md`, which is the live record.** There are **five** peer sessions, not three (add DKM #2550 and SUBSET-CONFORMANCE). Session names rotate whenever a session hits a usage limit, so any name written here will go stale; re-discover them rather than trusting a written name. Ownership has also moved: trinity and puzzle were released by SMO and migrated by APA; APA is done with `📜️script.ts`.

## ⚖️ Two rules learned the hard way today — apply them before amending any record

1. **Content evidence attributes; timing evidence does not.** The repo auto-commits the **whole tree on a timer**, so every flag bundles every session's in-flight work. Use `R100` rename records, diff contents, which symbols a change touches, a peer's own claim of authorship. **Never** use which flag a change landed in, what else shares that commit, or mtime proximity. *(Violated twice in one session before it stuck.)*

2. **Verify a live predicate before acting on any report — including your own agents'.** Reports, audits, `📓️status.md` files and commits are all **derived artifacts** and go stale within minutes in this tree. Four separate times today a report was already obsolete when it arrived; once, acting on it would have written a false blocker into this file and stalled a finished lane. Re-run the check yourself, then amend.

**Corollary — `cargo check` is not a gate.** It does not compile `#[cfg(test)]` code. Five separate issues today hid behind that blind spot, including breakage that landed unverified. **Always `--all-targets`**, and note that a lib-only check can pass while `--all-targets` fails on a *dependency* rebuilt with dev-dependency feature unification.

## Repo gotchas that cost other sessions real time

- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` is dual-mounted into two crates via `#[path]` — one edit, two crates.
- Additive struct fields still break struct literals (`grep -rln "TypeName {"` after adding a field).
- Adding an enum variant is expensive where matched exhaustively (`grep -rln "EnumName::"` first).
- Puzzle glue.rs / stdio glue.rs churn from concurrent inliner passes — re-read immediately before editing.

## Report shape (every agent, every wave)

Append to your assigned report file in this ticket folder. Sections, in order: what changed (file:line + grep anchors) · files touched (created/updated/removed) · verification commands run **with real output pasted** · `## Concurrent-churn observations` · honest pass/fail. A wave is not done until its report exists.
