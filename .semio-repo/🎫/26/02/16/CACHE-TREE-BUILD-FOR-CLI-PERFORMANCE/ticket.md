---
goal: R26-03
---

# Ticket

## Summary

Implemented tree caching with gzip JSON disk cache, in-memory fuzzy search replacing Bleve, git command timeouts, and ToolGoalList/ToolTicketList/ToolPolicyList parity fixes. Tree returns in under 5s on warm cache. All test regressions fixed.
## Changes

- semio-repo/cli/main.go: Added BuildMonorepoTreeCached with gzip JSON disk cache + git fingerprint invalidation
- semio-repo/cli/main.go: Added levenshtein and fuzzyContains functions for fuzzy search matching
- semio-repo/cli/main.go: Replaced Bleve-based search with in-memory searchTreeInMemory using fuzzyContains
- semio-repo/cli/main.go: Added execGitWithTimeout (60s context timeout per git command)
- semio-repo/cli/main.go: Added in-memory fingerprint caching with 10s TTL
- semio-repo/cli/main.go: BuildMonorepoTreeCached always builds with IncludeSections: true (superset tree)
- semio-repo/cli/main.go: Fixed ToolGoalList to use toolResultFromTreeList for CLI parity
- semio-repo/cli/main.go: Fixed ToolTicketList to use toolResultFromTreeList for nil filter case
- semio-repo/cli/main.go: Fixed ToolPolicyList to use toolResultFromTreeList for CLI parity
- semio-repo/cli/main.go: Fixed Interaction struct literal to use Files instead of removed Diff field
- semio-repo/cli/main.go: Fixed GetFiles caller to use GetInteractionFiles for contributor code
- semio-repo/cli/main_test.go: Refactored TestBuildMonorepoTree to pre-build trees once

## Todos

- [x] Analyze BuildMonorepoTree bottleneck
- [x] Design tree caching strategy
- [x] Implement tree cache (save/load gzipped JSON)
- [x] Modify BuildMonorepoTree to use cache on fingerprint match
- [x] Replace Bleve search with in-memory fuzzy search
- [x] Add git command timeouts and fingerprint caching
- [x] Fix ToolGoalList/ToolTicketList/ToolPolicyList parity
- [x] Fix concurrent edit breakages (Interaction.Diff, GetFiles)
- [x] Fix TestQueryFlag section/definition query failures
- [x] Build, test, validate <5s on cache hit
- [x] Run full test suite - only pre-existing failures remain
- [x] Close ticket
