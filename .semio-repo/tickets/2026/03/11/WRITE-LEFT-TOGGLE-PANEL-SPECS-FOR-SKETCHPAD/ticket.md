---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Added prompt-ready manual formula and diff addon template beneath Left Toggle Panel specs in sketchpad README.
## Changes
- Added `### Manual Formula (Prompt-Ready)` directly under `## Left Toggle Panel`.
- Added `Base Formula` YAML describing the left panel contract as editable fields.
- Added `Diff Addon Formula` YAML for `add`/`update`/`remove` operations and constraints.
- Added a reusable `Prompt Template` for requesting new left-panel features/addons via diff.

## Log
- Reopened existing ticket `2026/03/11/WRITE-LEFT-TOGGLE-PANEL-SPECS-FOR-SKETCHPAD` for follow-up spec format request.
- Queried repository context with `./semio-repo/cli/cli tree "left toggle panel sketchpad"`.
- Updated `semio/js/sketchpad/README.md` to include formula/manual + diff-driven prompt template under the existing left panel specs.

## Todos
- [x] Reopen existing left-toggle-panel specs ticket.
- [x] Add manual/formula version under left panel specs.
- [x] Add diff-style addon request structure for prompt-driven changes.
- [x] Track updates in ticket file.
- [x] Close ticket with touched files.

## Plan
1. Reuse the existing ticket and keep the same scope (left toggle panel specs).
2. Add an operator-friendly manual format under the current left panel section.
3. Add a diff-first template so new features/addons can be requested as deltas.
4. Close the ticket with updated summary.
