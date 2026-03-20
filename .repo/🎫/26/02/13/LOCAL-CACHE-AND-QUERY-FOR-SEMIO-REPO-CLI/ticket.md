---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS
---

# Ticket

## Summary

Extended tests for cache index (TestCacheIndexAndTreeQuery). Granular cache: when file changes, only file/folder/bundle/project docs updated via getChangedPathsFromGit+expandPathsWithAncestors. Parent docs include child names. Incremental update when index exists and dirty paths present.

## Changes

- repo/cli/main.go: Query Cache region (computeCompositeFingerprint, cacheMeta, ensureCacheIndexed, queryCacheIndex), searchMonorepoTreeWithCache, query command, crypto/sha256+encoding/hex imports
- repo/cli/main.go: Fixed pre-existing typo `if langName == "python" {performs` → `if langName == "python" {`
- repo/cli/README.md: Query cache spec in Monorepo Tree section
- repo/cli/main.go: Fix ticket ID 0000/00/00: add year/month/day to GraphQL ticketOpen query and ToolTicketOpen output; unwrap GraphQL data envelope in formatResult/formatMarkdownResult; GetArtifactID/GetArtifactURI fallback to id/uri when year/month/day missing; ticketNodeToData extracts year/month/day from URI

## Log

## Todos

## Plan

1. Add cache region: composite git fingerprint, meta.json, Bleve disk index under `.repo/cache/<repoID>/` ✓
2. Implement `computeCompositeFingerprint`: superHead, superDirtyHash, submodule pointers hash, submodule working hash ✓
3. Implement incremental index updates (git diff for clean, status for dirty) – deferred
4. Wire SearchMonorepoTree to use cache when fingerprint matches; fallback to in-memory ✓
5. Add `query` subcommand returning matching resource IDs/URIs ✓
