---
goal: ENTITYIDS/TESTCOVERAGE
---

# Ticket

## Summary

Refactored entity IDs and extended test coverage: Added EntityKinds/ResourceKinds/DiffableKinds/RelatedToFileKinds constants, renamed TreeNodeStatuteNode to TreeNodeStatute, added TreeNodeTodo/TreeNodeBreach, updated treeNodeKindToEntityKind mapping, fixed TestGoalListIDs type assertion, added 12 new test functions covering all spec ID patterns. All 79+ tests pass with 0 failures.
## Changes

- `semio-repo/cli/main.go`: Added EntityKinds, ResourceKinds, DiffableKinds, RelatedToFileKinds constant slices (~line 4285). Renamed TreeNodeStatuteNode→TreeNodeStatute, added TreeNodeTodo/TreeNodeBreach. Updated treeNodeKindToEntityKind: statute→"", category→"", todo→"todo", breach→"breach".
- `semio-repo/cli/main_test.go`: Added 12 new test functions (TestEntityKinds, TestResourceKinds, TestDiffableKinds, TestRelatedToFileKinds, TestProjectListIDs, TestBundleListIDs, TestSectionListIDs, TestContributorListIDs, TestGoalListIDs, TestTicketListIDs, TestDraftListIDs, TestAllSpecIDExamples). Fixed TestGoalListIDs type assertion []Goal→[]*Goal. Fixed TestTreeNodeKindToEntityKindCoversAll expected values for statute and category. Renamed TreeNodeStatuteNode→TreeNodeStatute globally.

## Todos

- [x] Understand current Go implementation
- [x] Run existing tests
- [x] Add EntityKinds/ResourceKinds/DiffableKinds/RelatedToFileKinds constants
- [x] Rename TreeNodeStatuteNode→TreeNodeStatute, add TreeNodeTodo/TreeNodeBreach
- [x] Update treeNodeKindToEntityKind mapping
- [x] Insert 12 new comprehensive test functions
- [x] Fix TestGoalListIDs type assertion
- [x] Fix TestTreeNodeKindToEntityKindCoversAll expected values
- [x] Run final comprehensive test suite (79+ tests, 0 failures)
