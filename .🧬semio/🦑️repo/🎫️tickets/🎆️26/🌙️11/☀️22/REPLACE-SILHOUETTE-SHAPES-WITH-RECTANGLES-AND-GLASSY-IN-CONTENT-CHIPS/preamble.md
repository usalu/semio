# 📜 Lane Preamble

Read `design.md` in this folder first. It is the contract.

Rules for every lane:

- Repo root: `/Users/ueli/Documents/semio`. Paths contain emoji; quote them.
- The worktree is shared live with 50+ machines and other agents edit the same tree. Re-read a file right before you edit it. Ignore unrelated changes. Never stop because someone else touches the area.
- No git command that changes the repository or working tree (no commit, stash, checkout, switch, reset, clean, restore, rebase, merge, worktree). Read-only git is fine.
- Do not edit `AGENTS.md` / `CLAUDE.md`. Do not use `repo dashboard` or IDEs.
- Greenfield: no backwards compatibility, no deprecated aliases, no compatibility layers, no migration scripts. Rename and rewrite by hand, everywhere at once, including fixtures and cases. Old names must not survive anywhere in your lane.
- Schema first, test driven: update schema and cases before code; keep TypeScript and Rust on the same cases; validate geometry against a third-party or independent implementation in tests where one exists already.
- Concise code. No comments inside definitions. Docstrings start with a unique fitting emoji. Temporary logs carry the `[DEBUG] ` prefix and are removed before you finish.
- UI text supports multiple languages with no default; add no hardcoded user-facing strings.
- No new runtime dependencies.
- Stay inside the files your lane owns. If another lane's file needs a change, write the need into your handoff file instead of editing, unless it is a one-line import/name fix that unblocks compilation.
- Build and test through the `repo` binary (`repo run <project>:<target>`, `repo test ...`) when it works within a few minutes; if it hangs or fails for infrastructure reasons, fall back to the native tool (`bun`, `vitest`, `cargo check -p ...`) scoped to your files. Never block on infrastructure.
- Put every generated output (logs, command output) under `🗑️generated/` in this ticket folder and delete it when done. Keep scripts and markdown.
- Finish with a handoff markdown in this ticket folder named `<lane>-handoff.md`: what changed (file list), what was verified with exact commands and results, what is open. Your final chat answer is three lines at most and points to that file.
