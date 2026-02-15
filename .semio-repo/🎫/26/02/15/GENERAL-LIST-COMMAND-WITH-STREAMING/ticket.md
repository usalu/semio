---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary
Introduce a general `list` command that streams a flat list of monorepo items using the same tree infrastructure (BuildMonorepoTree + FilterMonorepoTree). Remove entity-level `list` and `tree` subcommands. Update tests.

## Changes
- Added `listCommand` function and `flattenTreeNodes` helper
- Registered `listCommand` in `NewRootWithConfig`
- Removing entity-level list/tree subcommands from: policy, draft, todo, ticket, goal, interaction, statute, commit, contributor, project, bundle, folder, file, section, definition
- Removing empty entity commands from root: interaction, statute, commit, project, bundle, definition
- Updating canonical commands in repoPolicy
- Refactoring tests to use general `list` command

## Log
- Implemented listCommand with --sorted flag
- Implemented flattenTreeNodes helper
- Build passes

## Todos
- [x] Implement listCommand function
- [x] Register listCommand in root
- [ ] Remove entity list/tree subcommands
- [ ] Remove empty entity commands from root
- [ ] Update canonical commands
- [ ] Refactor tests
- [ ] Run tests and fix all failures

## Plan
1. Remove list/tree subcommands from each entity command
2. Remove entire empty entity commands (interaction, statute, commit, project, bundle, definition)
3. Remove their registration in NewRootWithConfig
4. Add "list" to canonical commands in repoPolicy
5. Refactor all test functions that use entity list/tree
6. Run tests, fix failures, repeat until green
