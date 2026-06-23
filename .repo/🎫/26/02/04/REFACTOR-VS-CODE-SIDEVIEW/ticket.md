# Ticket

## Summary

Refactored VS Code sidebar into two views (Monorepo + Filter), fixed repo binary resolution so root nodes expand, moved filter options into menu actions, wired filter state across all monorepo branches, and updated tests/docs accordingly.

## Changes

- Refactor sidebar view contributions to only include Monorepo and Filter
- Fix repo CLI binary resolution so GraphQL-backed tree branches populate
- Consolidate Filter UI actions into menu actions per filter-kind
- Ensure Monorepo view shows at least one child per root kind (Projects/Goals/Tickets/Policies/Contributors/Commits)

## Log

- 2026-02-04: Opened ticket.
- 2026-02-04: Consolidated repo activitybar views to only `compose.monorepo` and `compose.filter` (removed `compose.todos`).
- 2026-02-04: Fixed repo binary resolution for the VS Code extension to use `repo/cli/cli` when `./repo/cli/cli` is missing, unblocking GraphQL-backed tree expansion.
- 2026-02-04: Refactored Filter view to expose one tree item per filter kind, with option toggles exposed via item context menus (menu button actions) instead of per-option tree nodes.
- 2026-02-04: Implemented Monorepo tree population for Projects/Goals/Tickets/Policies/Contributors/Commits and ensured expanding root nodes yields children.
- 2026-02-04: Implemented file -> section -> definition expansion and click navigation for sections/definitions.
- 2026-02-04: Wired Filter search and toggles to filter all Monorepo branches.
- 2026-02-04: Updated VS Code extension tests and verified `npm test` passes.

## Todos

- Refactor sidebar view contributions to only include Monorepo and Filter
- Fix repo binary resolution so tree providers can load children
- Implement full Monorepo navigation + section/definition expansion
- Convert filter actions from inline icons/individual toggles to menu actions per filter-kind
- Update/extend tests to ensure at least one item per kind renders
- Update README.md + AGENTS.md with the new sideview/filter mechanism

## Plan

1. Consolidate views into Monorepo + Filter.
2. Fix data loading + filtering + navigation in Monorepo tree.
3. Refactor Filter tree to menu actions and apply filters across Monorepo.
4. Add/adjust tests and update dev docs.
