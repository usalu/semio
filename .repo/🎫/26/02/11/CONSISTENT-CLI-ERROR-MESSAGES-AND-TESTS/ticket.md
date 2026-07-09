---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Fixed inconsistent CLI error messages: added status guards to FinishTicket, GoalClose, GoalReopen; extended wrong-args tests for all commands (todo, section delete/extract/integrate, goal change, top-level move/extract/integrate); added TestCliWrongArgs_ErrorMessages with 36 error message verification cases; added wrong-lifecycle checks to E2E ticket/goal lifecycle tests; extended TestCliJsonErrorsToStderr from 4 to 22 cases

## Changes

### main.go

- Added status guard to `FinishTicket`: rejects closing non-open tickets with "ticket is not open" (before file/summary validation)
- Added status guard to `GoalClose`: rejects closing already-closed goals with "goal is already closed"
- Added status guard to `GoalReopen`: rejects reopening already-open goals with "goal is already open"

### main_test.go

- Extended `TestCliWrongArgs_SectionOperations`: added delete, extract, integrate missing-args cases
- Added `TestCliWrongArgs_GoalChange`: validates cobra.ExactArgs(1) enforcement
- Added `TestCliWrongArgs_TodoOperations`: create (missing parent/name), change (missing id), delete (missing id)
- Added `TestCliWrongArgs_TopLevelOperations`: move, extract, integrate missing-args cases
- Added `TestCliWrongArgs_ErrorMessages`: comprehensive error message verification for 36 cases across all commands (ticket, goal, todo, folder, file, section, definition, contributor, graphql, top-level extract/integrate)
- Extended `TestCliE2E_TicketLifecycle_Syntaxes_NoGithub`: added wrong-lifecycle checks (reopen open ticket, close closed ticket)
- Extended `TestCliE2E_GoalLifecycle_Syntaxes_NoGithub`: added wrong-lifecycle checks (reopen open goal, close closed goal)
- Extended `TestCliJsonErrorsToStderr`: added 18 new error cases covering all commands that return errors on missing arguments

## Log

1. Identified 3 missing status guards: FinishTicket, GoalClose, GoalReopen
2. Added status guards to all 3 functions
3. Cataloged all 30+ command validations across the CLI
4. Extended wrong-args tests for section (delete/extract/integrate), todo, goal change, top-level operations
5. Added TestCliWrongArgs_ErrorMessages with 36 error message verification cases
6. Extended lifecycle E2E tests with wrong-state error checks
7. Extended TestCliJsonErrorsToStderr from 4 to 22 cases
8. All tests pass

## Todos

- [x] Add status guards to FinishTicket (reject closing non-open tickets)
- [x] Add status guards to GoalClose (reject closing non-open goals)
- [x] Add status guards to GoalReopen (reject reopening non-closed goals)
- [x] Fix ticket close error message order (status check before file validation)
- [x] Extend tests for wrong args on all commands
- [x] Extend tests for wrong lifecycle state (close non-open, reopen non-closed)
- [x] Run all tests and verify they pass
- [ ] Update docs

## Plan

1. Add status guards to FinishTicket, GoalClose, GoalReopen
2. Fix error message priority in ticket close (check status before requiring files)
3. Extend existing test functions with wrong-argument and wrong-lifecycle tests
4. Run tests, fix any failures
5. Update README.md and AGENTS.md docs
