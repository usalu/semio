# Neutral Advisory Instructions

Changed exactly two current advisory lines from `bun nx run` to canonical `bun repo run`, preserving all target spellings, options, surrounding Markdown/backticks and Dockerfile comment syntax:

- `🌎️hub/Dockerfile:18`: `bun repo run os-hub:dev`.
- `.codex/plans/declarative-actions.md:52`: `bun repo run @spatial/js-core:test` and `bun repo run @spatial/js-kernel-brepjs:test`.

Read-only validation confirmed `os-hub` and its `dev` target in the actual authored `🌎️hub/📦️packages/🦀️rust/📋️project.json`. The two `@spatial/*` plan targets are not current authored package/project names; current exact-name searches find them only in this older plan. Their names were deliberately preserved within the requested two-line scope, so they are not qualified as runnable targets and no replacement target was guessed.

Python shlex independently parsed all three neutral instructions into exactly `bun`, `repo`, `run`, and their unchanged target. Dockerfile remains a comment-only change, the plan remains a single Markdown list item, and all source before/after SHA-256 values and exact lines are recorded in `🗑️generated/neutral-advisory-instructions-receipt.json`. Both files are in the owned edited inventory. No AGENTS file, runtime Source, SDK/Native/Tools/store or task execution was changed or run.

The explicitly historical Dockerfile comment at8 and explanatory `nx run` discussion at71 remain historical/descriptive text, not current instructions or executable bypasses. Aggregate execution remains held pending Root release.
