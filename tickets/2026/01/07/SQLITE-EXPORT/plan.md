# SQLITE-EXPORT Plan

## Goal
Add an `export` command to the CLI that exports all repo data to a SQLite database file.

## Tasks
1. Create SQLite schema in `sql/sqlite/repo/schema.sql`
2. Add SQLite dependency to `go/repo/go.mod`
3. Add export functions to `go/repo/repo.go`
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
