# Plan: Fix Ticket GitHub Comments and File Exclusions

## Goals

1. Ensure GitHub reopen comments always include `# 🤖 Prompt` heading.
2. Ensure GitHub close comments include `#🔍 Summary` and `# ✍️ Changes` headings with line metrics.
3. Ensure ticket workspace files never appear in close metrics or GitHub comments.
4. Update tests and documentation to lock in behavior.

## Steps

1. Inspect ticket reopen/close GitHub comment generation and identify why headings are missing.
2. Fix GitHub comment formatting and ensure metrics are included.
3. Filter ticket workspace files before metrics/comment generation.
4. Extend repo tests for reopen and close comment formatting and exclusions.
5. Update README.md and AGENTS.md, then close ticket.
