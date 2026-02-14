# Ticket

## Todos

- [x] Consolidate sidebar view registration to remove duplicate filter provider declarations.
- [x] Preserve monorepo view registration in the unified sidebar entrypoint.
- [x] Update dev docs for sidebar registration and filter provider behavior.

## Changes

- Unified sidebar registration in `semio-repo/vscode/extension.ts` and restored monorepo view wiring alongside shared filter provider usage.
- Documented sidebar registration expectations in `README.md` and `AGENTS.md`.

## Log

- Located duplicate `filterProvider` declarations and duplicate `registerSidebarViews` in `semio-repo/vscode/extension.ts`.
- Consolidated sidebar view registration and removed obsolete filter provider block.
- Updated dev documentation for sidebar registration constraints.
- Verified `npm run build:codegen` in `semio-repo/vscode`.

## Summary

Resolved codegen failure by consolidating sidebar view registration to a single filter provider, restoring monorepo view wiring, and documenting the sidebar registration expectations.
