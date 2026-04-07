# Ticket

## Todos
# Previously

Ticket creation required a summary even though the summary is only needed when the ticket is finished.

# Plan

- Remove summary from `ticket open`.
- Require `--summary=...` only on `ticket close`.
- Store `summary` in ticket frontmatter on finish.
- Update developer docs to match the CLI.

# Changes

- Updated `scripts/log.ts` to remove summary from `ticket open` and require `--summary=...` on `ticket close`.
- Updated `README.md` and `AGENTS.md` ticket usage examples and schema description.

## Changes

## Log

## Summary
# Summary

Require summary only when finishing tickets
