# Brief: Monolith Split Executor (shared by all split slices)

Repo: `/Users/ueli/Documents/semio` (macOS, zsh). `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.
Coordinator ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN` (`C`).

## Hard rules
- Follow `AGENTS.md`. Never run a modifying git command (commit, stash, checkout, reset, restore, add). No worktrees. No sub-agents; run every command in the foreground (background children die with you).
- Do NOT run `cargo`, `bun repo`, `nx`, `bazel` or any build/serve. The coordinator compiles centrally. Static verification only: `rustfmt --edition 2024 --check <file>` parse checks, the python/ts oracles in `F/c5b-tools/` and descendants (`modsplit.py lossless`, `vmod.py`, `defscan.py`, `ambig.py`, `readers.py`, `namecheck.py`, `modclash.py`), and the union-sandbox `--emit=metadata` differential if your predecessor tools set it up (sandboxes under `/Users/ueli/.cache/<prefix>/`, never inside the repo).
- `cd /Users/ueli/Documents/semio` explicitly in every Bash call. The shell `grep` is aliased to ugrep and silently misses matches on emoji paths: use `command grep` or `/usr/bin/grep`. Emoji path segments carry U+FE0F variation selectors; discover names with `ls`.
- Never delete a `🗑️generated` folder or other agents' files. Put logs under `F/🗑️generated/<prefix>-*`, tools under `F/<prefix>-tools/`.
- Other agents edit the repo concurrently. Re-read a file right before editing it. Use small anchored edits in shared parent/wiring files (`📦️lib.rs`, `🦀️.rs` parents, `Cargo.toml`); never rewrite a shared file wholesale.
- File paths must stay under 256 characters.

## Split rules (lessons from earlier slices)
- Goal: every file in your package under 1,000 lines, one file per semantic component, in the domain taxonomy tree (emoji-prefixed directory per child, `🦀️.rs` leaf, wired via `#[path]` from the parent like the existing children). Follow exactly how `RESULT-CENSUS5-A.md` / `RESULT-CENSUS5-B.md` split their files.
- Lossless: no code change beyond moving items, adding `mod`/`use` wiring and minimal visibility (`pub(crate)`/`pub(super)`). No compatibility re-exports, no legacy aliases, no behaviour change.
- Keep `// region`/doc comments, attributes (`#[cfg]`, `#[test]`, `#[allow]`) and item order with their items. Struct fields read across the new module boundary need `pub(crate)` (or `pub(super)`).
- Test roots split via `include!` keep the exact same test names (count before/after must match).
- Text readers: before moving code, find tests/scripts that read the file as text (`include_str!`, `readFileSync`, source-law assertions, sha pins) and re-point them in the same slice.
- Each new crate closure must stay within the source-boundary gate (4,000 active lines, 32 files, 256 KiB per crate closure). If a split would push a crate over 32 files, stop and report instead of creating a crate.
- A function longer than 1,000 lines cannot be split losslessly: report it, do not rewrite it.

## Report
Write/extend `F/RESULT-<SLICE>.md` with: verdict table per file (before/after lines, children), oracle results, text readers re-pointed, open risks, and the exact crate names the coordinator must compile (`cargo check -p <crate> --lib --tests`). Final message: 8 lines max with verdict and the crate list.
