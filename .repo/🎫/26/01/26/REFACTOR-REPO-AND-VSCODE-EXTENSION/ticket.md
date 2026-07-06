# Ticket

## Todos

# Plan

## Tasks

- [ ] Analyze codebase to locate relevant logic in `./repo/cli` and `js/vscode`.
- [ ] Refactor Ticket Title Validation in `./repo/cli`.
- [ ] Update Author Resolution logic in `./repo/cli` (contributors file).
- [ ] Refactor Plan Handling in `./repo/cli` (move original plan, store in ticket.json, no plan_ITERATION.md).
- [ ] Update Ignore Logic in `./repo/cli`:
  - [ ] Ignore files from `.gitignore` properly.
  - [ ] Ignore `go/server/server`.
  - [ ] Ignore `LICENSE.md`.
  - [ ] Ignore empty folders.
  - [ ] Remove `json` language processor.
  - [ ] Ignore files/folders without language processor or explicitly ignored.
- [ ] Refactor Document/IDs in `./repo/cli`:
  - [ ] Bundle-based document (Folders, Files, Tickets).
  - [ ] Orphan files in `repo/repo`.
  - [ ] Sections/Definitions parsing and IDs.
- [ ] Update VS Code Extension (`js/vscode`):
  - [ ] Extensions ignoring (LICENSE.md, empty folders, .repo, gitignore).
  - [ ] Remove "bundle:" prefix in tree view.
  - [ ] Sort codebase tree.
  - [ ] Bundle click opens `package.json`.
  - [ ] Fix Sections view (not showing sections, correct parsing).
  - [ ] Fix file tree unfolding.
- [ ] Verify changes with existing tests and add new tests if needed.
- [ ] Close ticket.

## Changes

## Log

## Summary

Bulk close
