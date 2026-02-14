---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI/REPO-CLI-FILTERS
---

# Ticket

## Summary

Added --query bleve full-text search (fuzziness=2) to all list/tree commands. Created statute list/tree and commit list commands with StreamStatutes/StreamCommits functions. Converted goal tree from GraphQL to streaming with query support. Added --query flag to top-level tree command via bindTreeFlags. All 27 query subtests pass, 4 fuzzy match tests pass, 6 empty-query tests pass, statute/commit command tests pass. Pre-existing test failures (TestInteractionUnmarshalAuthorShapes, TestFixtureBreachsByLanguage, TestUriToId) are unrelated.

## Changes

## Log

## Todos

## Plan
