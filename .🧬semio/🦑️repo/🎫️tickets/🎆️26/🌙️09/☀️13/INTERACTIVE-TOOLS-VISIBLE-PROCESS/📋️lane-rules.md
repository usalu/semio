# 📏️ Lane rules — phase 4 implementation agents

Ticket folder `T` = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`.
Binding spec: `T/📋️tool-run-contract.md`. Repo rules: `/Users/ueli/Documents/semio/CLAUDE.md` (read it fully).

1. Edit only the files your lane owns (contract §5). If you must touch a file another lane owns, make the smallest
   edit, and record it under "Foreign edits" in your report.
2. Other developers and agents edit the repo concurrently. Never run modifying git commands (commit, stash,
   checkout, reset, restore, worktree). Ignore unrelated churn; if a repo-wide build breaks outside your files,
   check whether it is a peer's in-flight edit, wait and retry rather than "fixing" foreign code.
3. Every Bash call starts with `cd /Users/ueli/Documents/semio` (cwd drifts). The Grep tool is broken; use `rg`.
4. Builds and tests run in the FOREGROUND only; background processes die with your turn. Never spawn background
   agents. macOS has no `timeout`. Do not set a private `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`; the shared build dir is
   intentional. A cargo run waiting on a lock is normal under load.
5. TDD: fixture + failing test first, then implementation. Every feature gets a language-agnostic fixture test and a
   third-party oracle where the contract names one.
6. Emoji-named test files need an explicit ASCII `[[test]] name` + `path` in Cargo.toml. New taxonomy members must be
   registered in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` if the folder is new.
7. Concise code, no comments inside definitions, docstrings start with a unique emoji, no compatibility layers,
   no deprecations, no legacy support. Temporary logs carry the `[DEBUG] ` prefix and are removed before you finish.
8. Proof of type-check = the compiler reached warnings for your crate. Zero errors with no warnings may mean an
   aborted expansion. Check wasm32 targets explicitly where the contract says so (cfg-gated code never compiles
   natively).
9. Outputs (build logs, test output) go to `T/🗑️generated/<lane>/` as `.txt`. Never delete or sweep anything else in
   `T/🗑️generated`. Never delete reports or scripts in `T`.
10. Do NOT open, close or reopen the ticket, and do not edit `T/📓️status.md` or `.vscode/launch.json`/`.claude/launch.json`
    (the coordinator registers commands from your report).
11. Never end your turn with work in flight. Finish by writing `T/📓️wave-<lane>.md`: what changed (file:line),
    public API as landed (exact signatures), tests run with exact commands and pass/fail counts, commands to register
    in launch.json, deviations from the contract with reasons, foreign edits, open items. Then reply with only the
    report path and a 5-line summary.
