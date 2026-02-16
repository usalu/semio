---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI/REPO-CLI-FILTERS
---

# Ticket

## Summary

Implemented the monorepo `tree` command with fuzzy full-text search via bleve and comprehensive filtering. Added `TreeNodeKind` enum and `TreeNode` struct modeling the complete monorepo hierarchy (projects, bundles, folders, files, sections, definitions, goals, tickets, drafts, policies, contributors, commits). Added `TreeFilter` with kind-level, sub-kind-level, date, status, and contributor filtering with case-insensitive matching. `BuildMonorepoTree` streams all data sources concurrently with a single filesystem walk, assigns folders/files to bundles by path prefix. `FilterMonorepoTree` recursively filters and `collapseFilteredKinds` promotes children of excluded kinds. `SearchMonorepoTree` uses bleve in-memory index for fuzzy matching with parent chain preservation. Section/definition parsing is opt-in via `TreeBuildOptions.IncludeSections`. Added `bindTreeFlags` with 60+ CLI flags. 46 tests added covering all units (TreeNodeKind, TreeFilter methods, FilterMonorepoTree, SearchMonorepoTree, RenderMonorepoTree, BuildMonorepoTree, collapseFilteredKinds, sortTreeChildren, flag binding). Updated README.md and AGENTS.md documentation.
## Changes

## Log

## Todos

## Plan
