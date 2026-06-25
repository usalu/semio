# Ticket

## Summary

Extended VS Code extension tests and stabilized headless test runner; aligned ticket interactions query; ensured build+tests pass.

## Changes

- repo/vscode/.vscode-test.mjs
- repo/vscode/package.json
- repo/vscode/extension.ts
- repo/vscode/queries.ts
- repo/vscode/extension.test.ts

## Log

- Adjusted VS Code test runner to run under Xvfb when `DISPLAY` is missing.
- Fixed test fixtures to load from `assets`.
- Updated extension GraphQL tickets query to use `interactions` (schema-aligned).
- Refactored Monorepo provider to accept filter provider injection for testability.
- Registered contributed commands in activation so command availability checks pass.
- Extended `extension.test.ts` to cover filter toggles, monorepo root categories, and ticket interaction structures.

## Todos

- [x] Make `npm test` stable in headless Linux.
- [x] Ensure `npm run build` succeeds.
- [x] Extend existing test suite for new Monorepo/Filter features.
- [ ] Update root `README.md` and `AGENTS.md` documentation.
- [ ] Close ticket.

## Plan

- Stabilize VS Code extension tests across headless environments.
- Ensure GraphQL schema changes are reflected in VS Code extension queries and types.
- Expand `extension.test.ts` coverage for new Monorepo/Filter behavior.
- Update developer docs and close the ticket.
