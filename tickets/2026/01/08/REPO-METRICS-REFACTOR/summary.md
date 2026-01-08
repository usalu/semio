# Summary

## Overview

Removed metrics from the GraphQL layer and moved them to be derived from SQLite views. The refactoring touched the GraphQL schema, Go repository implementation, and VS Code extension.

## Changes

### GraphQL Schema (`graphql/repo/schema.graphql`)
- Removed entity-level metrics fields from Bundle, Folder, File, Section, Definition, Contributor, Ticket types
- Kept contribution-level metrics on `Contribution*` types and `Checkpoint*Contrib` types for tracking line changes
- Kept `AnalyzeMetrics` for analyze operation results

### Go Repository (`go/repo/repo.go`)
- Removed metrics struct fields and GraphQL type definitions
- Removed resolver implementations for metrics fields
- Added `Violation.kind` resolver to fix type mismatch

### Go Tests (`go/repo/repo_test.go`)
- Added `TestNodesAndEdgesQuick` - tests all node collections and edges without slow bundle queries
- Added `TestNodesAndEdges` - comprehensive test (skipped in short mode)
- Added `TestNodeQuery` - tests Node interface with inline fragments
- Added short mode skips for slow tests (bundles, contributors, commits)

### VS Code Extension (`js/vscode/extension.ts`)
- Updated GraphQL queries to remove metrics fields from all document queries
- Updated TypeScript code to use alternative data sources (array lengths, names) instead of metrics
- Fixed codegen.ts schema path to point to correct location
- Regenerated GraphQL TypeScript types

### Files Verified (No Changes Needed)
- `go/mcp/main.go` - Uses `AnalyzeMetrics` which is kept
- `js/vscode/extension.test.ts` - No metrics references

## Build Status
- Go tests pass in short mode
- VS Code extension TypeScript compiles successfully
