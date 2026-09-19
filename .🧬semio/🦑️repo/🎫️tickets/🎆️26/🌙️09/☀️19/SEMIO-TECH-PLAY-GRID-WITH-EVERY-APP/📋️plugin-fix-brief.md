# 📋️ Plugin Fix Brief (shared by every fix agent)

Repository: /Users/ueli/Documents/semio (macOS, bun + nx + Rust nightly). Read `AGENTS.md` first and follow it,
with these overriding constraints:

- NEVER run modifying git commands (commit, stash, checkout, reset, restore) and NEVER use git worktrees. Other
  developers and other agents edit the tree concurrently — ignore unrelated changes, never revert them.
- Do NOT open/close/reopen tickets or goals. Do NOT delete or sweep any `🗑️generated` folder. Do NOT spawn
  sub-agents. Run every build/test in the FOREGROUND. macOS has no `timeout`. `cd` explicitly in every Bash call.
- Scratch files/logs only under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/🗑️generated/<your-topic>/`.
- Use `CARGO_INCREMENTAL=0` for test runs (disk is shared and fills up); run `df -g /` before big builds and, below
  20 GB free, prune only stale cargo incremental session dirs older than 2 h
  (`find <⚡️cache>/cargo/build/*/incremental <⚡️cache>/cargo/build/*/*/incremental -mindepth 2 -maxdepth 2 -type d -mmin +120`).
- A watchdog kills native test binaries running longer than 10 minutes; a hanging test is a defect to fix.

## Goal

Every native test of the crates assigned to you passes:
`CARGO_INCREMENTAL=0 cargo test -p <crate> --lib --tests --no-fail-fast [--features component-app-assembly when the crate declares it]`.

Baseline logs (before your work) per crate: `🗑️generated/baseline/<crate>.txt`; summary `🗑️generated/baseline/summary.tsv`.

## Rules for fixing

- Fix ROOT causes. Never delete, `#[ignore]`, weaken or loosen a test, and never special-case test inputs.
- Committed fixtures are contracts. When a fixture and the code disagree, decide which side is wrong from the
  surrounding convention (sibling crates that pass, the schema, the enum attributes, doc comments, git history via
  `git log`/`git show` — read-only). Regenerate a fixture only when the code is right and the new output is correct,
  and then regenerate it with the crate's own generator/printer, not by hand-editing to match.
- A test exercising a removed/renamed API is updated to the current contract, preserving what it proved.
- Known repo convention example: mutation enums whose fixtures are `{"mutation": "setSnapshot", ...}` carry
  `#[value(tag = "mutation", rename_all = "camelCase")]` (see `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/…/🧬️mutations/🦀️.rs`).
- Framework code under `🧰️framework` may be changed when the defect is there, but keep the change general and run the
  affected framework crate's tests too.
- If you touch a crate other agents also depend on, run a `cargo check` of a couple of dependents.

## Report

Concisely: per crate, baseline pass/fail → final pass/fail (from runs you actually made), root causes grouped,
files changed (absolute paths), and anything left failing with the precise reason. Do not claim anything you did not run.
