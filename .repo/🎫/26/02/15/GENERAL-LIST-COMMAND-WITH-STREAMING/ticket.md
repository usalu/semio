---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary
Introduce a general `list` command that streams a flat list of monorepo items using the same tree infrastructure (BuildMonorepoTree + FilterMonorepoTree). Remove entity-level `list` and `tree` subcommands. Update tests.

## Changes
- Added `listCommand` function and `flattenTreeNodes` helper
- Registered `listCommand` in `NewRootWithConfig`
- Removed entity-level list/tree from sectionCommand (code + AddCommand)
- Updated all ~20 test functions with new args
- Fixed buildOpts.IncludeSections optimization
- Fixed bleve cache invalidation: swapped filter/search order so cache indexes full unfiltered tree

## Log
- Implemented listCommand with --sorted flag
- Implemented flattenTreeNodes helper
- Removed sectionCommand list/tree (code + registration)
- Updated all test args across ~20 test functions
- Fixed buildOpts: IncludeSections only when targeting sections/definitions or no specific filter
- Discovered bleve cache bug: cache fingerprint ignores filter, so different --only-* filters cause stale hits
- Fix: swap to search-first, filter-second order in list/tree/mcpTree commands
- Clear stale cache; all query tests pass individually after fix

## Todos
- [x] Implement listCommand function
- [x] Register listCommand in root
- [x] Remove sectionCommand list/tree
- [x] Update all test args
- [x] Fix buildOpts optimization
- [ ] Fix bleve cache invalidation (swap filter/search order)
- [ ] Run full query test suite
- [ ] Run all remaining test groups
- [ ] Fix any failures
- [ ] Close ticket

## Plan
1. Remove list/tree subcommands from each entity command
2. Remove entire empty entity commands (interaction, statute, commit, project, bundle, definition)
3. Remove their registration in NewRootWithConfig
4. Add "list" to canonical commands in repoPolicy
5. Refactor all test functions that use entity list/tree
6. Run tests, fix failures, repeat until green
