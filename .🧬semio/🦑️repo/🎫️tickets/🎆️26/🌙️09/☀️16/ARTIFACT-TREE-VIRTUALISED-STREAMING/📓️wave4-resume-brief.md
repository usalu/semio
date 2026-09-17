# 📓️ Wave 4 — resume brief (2026-09-17, new coordinator session)

The previous fleet died with its session. Code from every packet is on disk; what is missing is verification, the
unfinished packets (A6, A8, F1, F2) and their reports. Ticket folder (all 📓️ reports live here):
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING`

## Read first
1. `/Users/ueli/Documents/semio/AGENTS.md`.
2. `📓️design-virtualised-tree.md` (normative design) and `📓️p3-sdk.md` (the SDK as built — it wins where it differs).
3. `📓️wave2-app-brief.md` §"What migrated means" (app packets) and the previous report for your area if one exists
   (`📓️a1…a8`, `📓️p1…p5`, `📓️w3-browser-verification.md`). Previous logs under `🗑️generated/<packet>/` are STALE
   (peers have churned the tree since) — use them as hints only, re-run everything you claim.

## The goal, in one line
No `+N` / `…more` continuation row and no silent truncation in any panel tree of any app: every container stamps its
full `total`, children materialise lazily on expand, only the viewport's rows are materialised, scrolling streams more
from the guest. One framework mechanism (`TreeWindow`, `ViewModel.tree_windows`, SDK `TreeWindows` /
`window_section` / `tree_window_item`, host observer), used by every app. No app-local paging survives.

## Hard rules
- `cd /Users/ueli/Documents/semio` explicitly in EVERY Bash call. Quote emoji paths. `grep` here is ugrep: always pass
  `-a` or it silently returns zero matches on these files.
- Never `git stash / commit / checkout / reset / restore / worktree`. Never sweep or delete any `🗑️generated` folder.
  Never close/reopen tickets. Never start dev/preview servers (the coordinator does that).
- Other agents (this fleet and unrelated peers, incl. Codex) edit the same files concurrently. Re-read a file right
  before editing it, keep edits surgical, never revert or "clean up" changes you did not make, never rewrite a whole
  file. If a build breaks in code that is not yours, check whether a peer is mid-refactor (mtime, `git log --date=iso`),
  wait 2–3 min and retry; only fix it if it is caused by this ticket's changes.
- Builds run in the FOREGROUND of your own Bash calls (background children die when your turn ends). macOS has no
  `timeout` command. Use Bash `timeout` parameter up to 600000 ms and re-invoke/poll if a build needs longer.
- Shared cargo build dir with fine-grain locking: never set `CARGO_TARGET_DIR` or `RUSTC_WRAPPER`. An idle-looking
  cargo with a rustc child is working; a cargo waiting on a lock is normal under a fleet — wait, don't kill others' builds.
- Env for every cargo call: `export DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0
  CARGO_PROFILE_WASM_DEV_DEBUG=false`.
- Artifact crates: `cargo test -p <crate> --features component-app-assembly` (check the crate's Cargo.toml), then
  `cargo check -p <crate> --target wasm32-wasip2 [--features …]` — native check never compiles cfg(wasm32) code.
  `cargo test` rc=101 means compile error OR failing test: read the log. Zero errors with zero warnings and no
  `Finished` line is not proof. Prefer running the window-law tests by name filter first, then the whole suite.
- A failing test is yours if it touches trees/panels/paging/windows/`setPanelPage`/`UI_BUILT_CHILDREN_MAX`/
  `UI_VALUE_PAGE_ROWS`/retained publication of the deleted action. If it fails identically without this ticket's change
  (prove it: read the assertion + the code it exercises, or `git show HEAD~N:<path>` for read-only comparison), record
  it as pre-existing with the evidence and move on.
- Capture logs as `.txt` under `🗑️generated/<packet>/` (create the dir). Write your report to `📓️<packet>.md` in the
  ticket folder: what you changed (paths), what you ran (commands + pass/fail counts), what is NOT finished and why.
  Final chat reply: report path + at most 8 lines. Do not paste logs into chat.
- Never claim "written, not run". If you could not run something, say so explicitly in the report.
