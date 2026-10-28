# Course Scheduled Quiz Access

Implemented scheduled UI access for topics, quizzes and individual questions. **Energy is available; every other current topic and its quizzes remain locked.** No dates were invented for unreleased material.

The course and its solution-free view carry a schema-first `unlocks` map, mirrored in TypeScript and Rust. Keys are topic IDs, quiz IDs or `quiz-id/task-id`; numeric values are Unix milliseconds, `null` means awaiting scheduling, and missing keys add no restriction. Every ancestor restriction applies. The next release wakes the UI automatically; returning to a sleeping tab also rechecks it.

Locked previews stay rendered behind a theme-aware tint with English/German status. Links, card actions, question controls, saved-run actions, ordinary deep links and history destinations obey the lock. Session guards also prevent a stray UI command from starting a locked run or opening/answering a locked question. Restored and followed content is gated as well.

The hidden author-preview URL is **`?preview=1#<page-id>`**, for example `?preview=1#parametric`. There is no UI switch. It is an explicit UI override, not authentication.

[Verification, passing tests, browser evidence and remaining repository gate limits](verification.md).

No dependencies, migration scripts, Git commits, worktrees or `AGENTS.md` edits were introduced. Concurrent content and framework changes were preserved. Generated ticket output is removed; authored scripts and Markdown audits remain.
