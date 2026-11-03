# 📋️ Session 2 Brief (2026-11-02, late evening)

A fresh coordinator session resumes this ticket with a new 20-worker fleet. Workers from session 1 are gone; their reports (`T/📝️<agent>.md`) and the coordinator board (`T/🗂️work-board.md`, decisions D1–D47) are the state of record.

Read, in this order: `AGENTS.md` (repo root), `T/📋️brief.md` (rules, ownership — still in force), `T/🗂️work-board.md` (decisions D1–D47 are binding), `T/🔗️interfaces.md`, `T/📐️conventions.md`, your own `T/📝️<agent>.md`.

## Rules added in session 2

- **Verify before you claim.** Everything in a session-1 report is "worker-reported". Re-run the check that proves it before you build on it, and cite the command and its result.
- **Report.** Append a `## Session 2` section to your own `T/📝️<agent>.md` (read-only auditors: return the report text in your final message; the coordinator files it). Keep it current as you go: what changed (files), what you verified (command + result), what is still open.
- **Never kill global processes** (`taskkill /IM`, `pkill bun`, …). Only stop processes you started, by PID.
- **No `rm -rf` outside your own `T/🗑️generated/<agent>/` and `T/🛠️scripts/<agent>/`.**
- **Shared files**: reread right before every edit, edit with Edit (minimal hunks), never rewrite a whole shared file. Owners are in the brief. Schema/registry changes go through their owner; cross-system decisions go to `main` via SendMessage, then continue with independent work.
- **Running checks**: `repo run @teaching/staffage:test --lang=en` (or the ticket-local vitest configs in `T/🛠️scripts/*` if repo tooling is broken — say so). Validator: the catalog check of `🔨️modules/🗂️catalog` / `🔨️modules/🔭️multiview`. Matrix: `T/🗑️generated/validation/` (validation owns it).
- **Big renders**: one at a time, `--jobs 1`; check free disk before rendering thousands of frames.
- **Finishing**: when your assignment is done, your final message lists changed files, verification evidence, and open issues. The coordinator then assigns your next batch.
