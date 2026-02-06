---
goal: REPO-VSCODE-EXTENSION
---

# Ticket

Fix data loading for Goals, Tickets, and Contributors in the "Monorepo" tree view of the semio-repo VS Code extension.

## Summary

The URQL client `query()` calls in the VS Code extension were missing `.toPromise()`, causing them to return observables instead of results, which the async tree provider could not handle.

## Changes

### [js/vscode/extension.ts](semio-repo/vscode/extension.ts)

- Added `.toPromise()` to all GraphQL query calls to ensure they return a `Promise<OperationResult>`.

## Log

- Identified that Projects were loading but Goals, Tickets, and Contributors were not.
- Traced the issue to `client.query()` calls in `extension.ts`.
- Verified the Go backend GraphQL API returns data correctly via CLI.
- Applied `.toPromise()` to all relevant fetching functions.

## Todos

- [x] Identify root cause of empty tree nodes
- [x] Fix GraphQL query calls in `extension.ts`
- [x] Verify backend GraphQL responses
- [x] Validate fix in extension

## Plan

- Add `.toPromise()` to all `client.query` calls in `semio-repo/vscode/extension.ts`.
- Verify that the tree view now populates all root nodes correctly.
