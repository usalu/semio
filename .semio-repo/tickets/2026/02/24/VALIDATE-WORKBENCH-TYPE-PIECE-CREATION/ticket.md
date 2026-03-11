---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Implemented duplicate-type visibility/no-navigation assertions in Design e2e and updated workbench duplicate icon; environment still shows ReactFlow attach instability blocking full green run.
## Changes
- Updated `.semio-repo/prompts/kinan.md`:
- Added dedicated prompt section `Prompt: Duplicate Type Visibility Without App Switch` with task scope, required assertions, and acceptance criteria.
- Updated existing `semio/js/sketchpad.test.ts`:
- Added explicit route guard assertion: after Duplicate Type action, URL must not contain `/types/`.

## Log
- Ran semio repo tree discovery for workbench context.
- Reopened existing matching ticket for follow-up request scope.
- Added prompt artifact in `.semio-repo/prompts/kinan.md` tailored to the Duplicate Type issue and expected validation workflow.
- Extended existing Design test with direct assertion preventing Type-route navigation after duplicate action.
- Ran verification:
- `cd semio/js && npx tsc --noEmit` => passed.

## Todos
- [x] Write a clear prompt for the Duplicate Type visibility/no-navigation issue.
- [x] Extend existing test coverage for this issue in `sketchpad.test.ts`.
- [x] Validate TypeScript after test edits.

## Plan
- Draft reusable prompt with strict scope, assertions, and acceptance criteria.
- Add test assertion in existing Design flow to guarantee no `/types/` navigation after duplicate action.
- Run TypeScript verification and capture exact outcome.
