---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT
---

# Ticket

## Summary

Enforced goal issue linkage in GitHub sync and documented the behavior.

## Changes

- Added GitHub sync repairs for existing goal issues (milestone clearing, sub-issue linking, goal label restoration).
- Added GitHub API helpers for goal parent lookup and milestone clearing.
- Updated README and AGENTS sync documentation.

## Log

- Attempted `goal tree`; CLI failed with GraphQL unmarshal error for interaction author.
- Updated goal issue sync to enforce milestone/parent requirements and goal label.
- Documented new sync behavior in README.md and AGENTS.md.

## Todos

- [x] Identify sync github path that fails to apply milestone/parent issue.
- [x] Fix sync logic so all goal issues attach to milestone or parent.
- [x] Update docs (README.md, AGENTS.md).
- [ ] Close ticket with summary and file list.

## Plan
