---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Bulk close

## Changes

- `.prettierrc.json`: Added `overrides` array with `*.md` files scoped to `tabWidth: 1`

## Log

- Tested `tabWidth: 1` vs `tabWidth: 2` — both produce 2-space list indentation (minimum due to `- ` marker width)
- Tested `tabWidth: 3` and `tabWidth: 4` — these do produce wider indentation (3 and 4 spaces respectively)
- Conclusion: Prettier's minimum markdown list indent is 2 spaces; `tabWidth: 1` keeps it at that minimum

## Todos

- [x] Add markdown override to `.prettierrc.json`
- [x] Verify behavior

## Plan

- Add markdown override with `tabWidth: 1` to `.prettierrc.json` so nested lists use minimum indentation
