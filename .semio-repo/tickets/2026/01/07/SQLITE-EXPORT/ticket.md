# Ticket

## Todos

# SQLITE-EXPORT Plan

## Goal

Add an `export` command to the CLI that exports all repo data to a SQLite database file.

## Tasks

1. Create SQLite schema in `sql/sqlite/repo/schema.sql`
2. Add SQLite dependency to `./semio-repo/cli/go.mod`
3. Add export functions to `./semio-repo/cli/cli.go`
4. Add `export` command to `go/cli/main.go`

## Schema Design

Tables needed:

- `repo` - Single row with repo metadata
- `bundle` - Nx bundles
- `folder` - Filesystem folders
- `file` - Source files
- `section` - Code sections/regions
- `definition` - Code definitions (functions, classes, etc.)
- `contributor` - Contributors
- `contributor_email` - Contributor emails (many-to-one)
- `contributor_link` - Contributor links (many-to-one)
- `ticket` - Tickets
- `ticket_checkpoint` - Ticket checkpoints
- `ticket_checkpoint_file` - Checkpoint file contributions
- `policy` - Policies
- `violation_kind` - Violation kinds
- `violation` - Individual violations
- Metrics computed via SQL views/queries

## Implementation Notes

- Use `github.com/mattn/go-sqlite3` for SQLite driver
- Export all data in a single transaction for consistency
- Include line/section/definition counts via queries

## Changes

## Log

# SQLITE-EXPORT Log

## 2026-01-07

### Implementation Complete

1. Created comprehensive SQLite schema at `sql/sqlite/repo/schema.sql`:
   - Tables: repo, bundle, bundle_tag, folder, file, section, definition, contributor, contributor_email, contributor_link, commit_record, ticket, ticket_checkpoint, checkpoint_file, checkpoint_section, checkpoint_definition, policy, policy_scope, violation_kind, violation, contribution tables
   - Views: repo_metrics, bundle_metrics_view, folder_metrics_view, file_metrics_view, section_metrics_view, violation_priority_counts, contributor_metrics_view, ticket_metrics_view

2. Added pure Go SQLite driver (`modernc.org/sqlite v1.34.5`) to `./semio-repo/cli/go.mod`:
   - Initially tried `github.com/mattn/go-sqlite3` but it requires CGO/C compiler
   - Switched to `modernc.org/sqlite` which is pure Go and works on Windows without CGO

3. Added export functions to `./semio-repo/cli/cli.go`:
   - `ExportResult` struct to hold export counts
   - `ExportToSQLite()` main function
   - Helper functions: `exportRepo()`, `exportBundles()`, `exportFolders()`, `exportFiles()`, `exportSectionsRecursive()`, `exportContributors()`, `exportTickets()`, `exportPolicies()`, `exportViolations()`
   - `ToolExport()` CLI wrapper

4. Added `export` command to `go/cli/main.go`:
   - Usage: `repo export [output]`
   - Defaults to `temp/repo.db` if no output specified
   - Returns JSON with counts of exported entities

### Issues Fixed

1. Changed driver name from `"sqlite3"` to `"sqlite"` for modernc.org/sqlite compatibility
2. Updated schema CHECK constraint to allow `'finished'` status in addition to `'open'` and `'closed'`

### Test Results

Export command successfully created database with:

- 15 bundles
- 37 folders
- 135 files
- 769 sections
- 7 contributors
- 262 tickets
- 3 policies
- 30 violation kinds
- 1,356 violations

Database file size: 1.6 MB

## Summary

# Summary

Added `export` CLI command that exports all repo data to a SQLite database file. Uses pure Go SQLite driver (modernc.org/sqlite). Exports repo metadata, bundles, folders, files, sections, contributors, tickets, policies, violation kinds, and violations. Schema includes views for metrics computation.
