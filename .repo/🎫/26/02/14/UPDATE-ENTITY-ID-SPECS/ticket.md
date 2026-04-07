---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Updated all entity ID generation to match new hierarchical requirements. Added PropagateParentIDs, goalArtifactID helper, fixed GetArtifactID for goals/tickets/commits/line/range, excluded bundle root folders from tree, updated 6 stale tests, added 20+ comprehensive entity ID tests.

## Changes

- `repo/cli/main.go`: Added `PropagateParentIDs` function, `goalArtifactID` helper, fixed `GetArtifactID` for goal/ticket/commit/line/range, excluded bundle root from folder map, removed debug log from `FileHeaderId`
- `repo/cli/main_test.go`: Added 20+ entity ID tests (GetArtifactID for all kinds, Flat, PropagateParentIDs, verifyTreeHierarchy, live tree tests for projects/bundles/folders/files/goals/tickets/contributors/commits), fixed TestFileHeaderId/TestFileKindEmoji/TestSectionHeaderIdAndUri/TestDefinitionHeaderIdAndUri/TestPolicyTreeCommand/TestGraphQLTicketsQuery to match new spec

## Log

- Session 1: Added PropagateParentIDs, goalArtifactID, fixed GetArtifactID for tickets/goals/commits, added line/range support, added goalId to ticket data, added authorId to commit data, wrote comprehensive tests
- Session 2: Fixed bundle root folder exclusion, fixed 6 stale tests, removed debug logging, verified all tests pass

## Todos

- [x] Add PropagateParentIDs function and call it after tree building
- [x] Fix GetArtifactID for goal, ticket, commit, add line/range
- [x] Add goalArtifactID helper function
- [x] Add goalId to ticket tree data, authorId to commit tree data
- [x] Fix bundle root folder exclusion from folder map
- [x] Write comprehensive tests for exact IDs on every entity kind
- [x] Fix stale tests (FileHeaderId, FileKindEmoji, SectionHeaderIdAndUri, DefinitionHeaderIdAndUri, PolicyTreeCommand, GraphQLTicketsQuery)
- [x] Remove debug logging
- [x] Verify all tests pass

## Plan

1. ~~Propagate parent IDs through tree building so every node's Data map includes `parentId`~~
2. ~~Update `GetArtifactID` for new entity kinds (line, range) and ensure all formats match spec~~
3. ~~Update ticket data in tree to include `goalId` for proper hierarchical IDs~~
4. ~~Fix bundle root folder exclusion from tree~~
5. ~~Add comprehensive tests for exact IDs on every list and tree command~~
6. ~~Fix stale tests to match new spec~~
7. ~~Run tests and verify all passing~~
