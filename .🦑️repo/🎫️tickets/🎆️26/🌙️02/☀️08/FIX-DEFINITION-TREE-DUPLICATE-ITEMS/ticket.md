---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-VSCODE-EXTENSION
---

# Ticket

## Summary

Filtered monorepo section tree children to section-typed nodes so definitions are no longer rendered twice as sections.

## Changes

- Updated `repo/vscode/extension.ts`:
  - Added `isSectionNode(...)` guard for GraphQL section-interface nodes.
  - Filtered `section.children` to section nodes only before building section tree items.
  - Updated child collapsible-state detection to consider only section grandchildren.
- Updated `README.md` (`# 📦️ Bundles` / VS Code bundle section) with the section-child filtering mechanism.
- Updated `AGENTS.md`:
  - Added UI/UX requirement for monorepo section tree child typing.
  - Added `# Codebase` entry detail for `repo/vscode/extension.ts`.

## Log

- Listed goal tree via `./repo/cli/cli goal tree`.
- Opened ticket via `./repo/cli/cli ticket open`.
- Traced VS Code extension tree construction and GraphQL file content usage.
- Confirmed GraphQL `Section.children` can contain both section and definition interface nodes.
- Implemented section-type filtering in extension tree rendering.
- Attempted validation:
  - `npm --prefix repo/vscode run preflight` failed due pre-existing `extension.test.ts` `RepoEvent` type errors.
  - Direct `npx tsc` invocation produced unrelated workspace/tsconfig errors.

## Todos

- Manual UI verification in VS Code: expand a section containing definitions and verify each definition appears only once with definition iconography.

## Plan

- [x] Reproduce source path for duplicate section/definition rows.
- [x] Implement fix in VS Code extension section tree builder.
- [x] Update README.md and AGENTS.md documentation layers.
- [x] Run available validation commands and record limitations.
