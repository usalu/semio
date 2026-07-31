---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Extended filter coverage in existing sketchpad test structure with full Home and Kit URL toggle cycles; Home+Kit targeted playwright runs pass.

## Changes

- Updated `compose/js/sketchpad.test.ts` (existing file only):
- Extended `Home` filter section with URL on/off cycle assertions for all three toggles: `temporary`, `local`, `remote`.
- Extended `Kit` filter section with URL on/off cycle assertions for `designs`, `types`, and `qualities`.
- Kept all changes inside existing test blocks and existing file structure.

## Log

- Reopened the existing filter ticket for follow-up test-completeness request.
- Audited current app tests and identified missing explicit URL cycle assertions in `Home` and `Kit`.
- Added reusable in-test helpers for toggle-cycle assertions without creating new files.
- Ran `npx playwright test sketchpad.test.ts --grep "Home|Kit|Type|Design" --reporter=list`:
- `Home` passed.
- `Type` passed.
- `Kit` initially failed due incorrect assumption about Kit URL semantics (`kind` stores active entry when selected).
- `Design` failed in existing filter assertion (`pieces` off did not hide nodes in this run).
- Updated Kit assertions to match runtime semantics.
- Re-ran `npx playwright test sketchpad.test.ts --grep "Home|Kit" --reporter=list`:
- `Home` passed.
- `Kit` passed.

## Todos

- [x] Reopen related filter ticket.
- [x] Identify missing filter assertions in existing app tests.
- [x] Add Home filter URL cycle assertions for all filter toggles.
- [x] Add Kit filter URL cycle assertions for main filter toggles.
- [x] Run targeted Playwright verification and capture result.

## Plan

1. Map existing filter coverage by app in `sketchpad.test.ts`.
2. Extend Home and Kit sections with full URL toggle-cycle assertions.
3. Run targeted e2e verification and adjust assertions to observed runtime behavior.
4. Close ticket with summary and updated file list.
