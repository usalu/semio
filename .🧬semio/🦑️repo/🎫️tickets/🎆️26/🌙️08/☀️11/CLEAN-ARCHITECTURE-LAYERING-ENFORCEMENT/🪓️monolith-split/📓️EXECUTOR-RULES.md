# 📏 Executor Rules for the Monolith-Split Fleet

Every executor in this fleet follows these rules. They come on top of `/Users/ueli/Documents/semio/AGENTS.md`.

## Process

- Work in the foreground and spawn no sub-agents. Do not end your turn until your deliverable file exists; confirm it with `ls -l`.
- Use `/usr/bin/grep` for code search. Plain `grep` is a ugrep alias that silently misses matches in emoji paths, and the Grep tool is broken. Quote glob patterns in zsh.
- Never run a git command that modifies anything: no commit, stash, checkout, restore, add or reset, and no worktrees. Read-only git commands (diff, status, show, log) are fine.
- Humans, Codex peers and other fleet agents edit the same tree at the same time. Never revert or "clean up" a change you did not make. Re-read a file right before every edit.
- Never delete any `🗑️generated` folder. Put logs only in `🪓️monolith-split/🗑️generated/<your-prefix>-*.log`.
- Debug logs carry the prefix `[DEBUG] ` and must be removed before you finish.

## Editing

- Scripted cuts use content anchors you have verified are unique (`assert text.count(anchor) == 1`). Run `git diff --stat -- <files>` right after each one. If the counts are out of proportion, restore your own change at once.
- Each new file starts with a docstring whose first character is a unique emoji. Write no comments inside definitions.
- Name new folders `<emoji>️<kebab>`, with the emoji followed by U+FE0F. The emoji must be an RGI emoji; reuse one already used as a folder prefix in the repo, and never repeat an emoji among siblings. Keep full paths at most 240 UTF-8 bytes.
- Rust child modules are wired with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus a `pub(crate) use` or `pub use` re-export. Resolve glob ambiguity with an explicit `use super::name;`, never with a qualified path, because a peer sweep strips qualifiers.
- The repo is greenfield. Do not add compatibility layers, aliases, deprecations or migration shims. Update every caller and fixture at once.

## Verification

- Raw cargo is refused. Run `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --message-format=short`, with a `--tests` variant and a `--target` variant for wasm code where it applies.
- macOS has no `timeout`. Cap long commands with `perl -e 'alarm N; exec @ARGV' -- <cmd>`.
- Record a baseline before your first edit, and attribute every error: only errors caused by your own change are yours. If a peer's crate breaks the workspace load, retry in an `until` loop with `sleep 60`, for at most about 40 minutes.
- Check right after each multi-file edit (compile-atomic edits), so the workspace never stays broken.
- The compiler-ingress source-boundary gate (4000 active lines, 32 files, 262144 bytes per crate, dependencies included) currently refuses nearly every framework and os crate. For Rust splits the gate you must pass is the full-resolution differential in `📓️RESOLUTION-ORACLE.md` (`--emit=metadata -o /dev/null`, run from the real crate lib root against a sandbox holding the original). `-Zunpretty=expanded` alone is insufficient. Also check losslessness (line multiset), rustfmt parsing and path resolution. Keep every child at or below about 2500 active lines, because each child will later become its own crate.
- Never compile around the gate. Do not build scratch cargo workspaces or manifests outside the repo, do not use private target dirs, and do not set RUSTC by hand to type-check or test repo crates. That sidesteps the peer fleet's compiler storage custody. Also do not chain `rustc --emit=metadata` through `.rmeta` files written anywhere, and do not borrow third-party `.rmeta`/`.rlib` from other checkouts or target dirs. Both amount to a type-check around the gate. Allowed: the analysis-only rustc oracles in `📓️RESOLUTION-ORACLE.md` (output to `/dev/null`) and the sanctioned `bun repo cargo` launcher.
- Never claim a check passed unless you ran it. Report each command you ran and its actual result.
