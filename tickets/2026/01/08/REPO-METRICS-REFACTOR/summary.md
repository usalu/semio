# Summary: REPO-METRICS-REFACTOR

## Changes

1.  **Metric Removal**: Removed all *Metrics types and metrics fields from graphql/repo/schema.graphql and go/repo/repo.go to simplify the schema and resolve query errors in the CLI. SQLite views remain as the source of truth for metrics.
2.  **Schema Fixes**: Updated Range and Position types in graphql/repo/schema.graphql and go/repo/repo.go:
    *   Range now uses start: Position! and nd: Position! (previously implicit or scalar in some contexts).
    *   Position struct in Go updated to use Character field (JSON character) instead of Column.
    *   Position type uses character: Int!.
3.  **Frontend Generation**: Regenerated VS Code extension GraphQL types (js/vscode/generated/graphql.ts) to match the updated schema.
4.  **Reference Implementation**: Range usage in CodebaseQuery (VS Code) is now compatible with the schema, validating start { line } selection.

## Verification

*   **CLI**: go run go/cli/main.go ticket list executes successfully (previously failed on metrics).
*   **Tests**: go test ./... in go/repo passes all tests, encompassing the schema changes.
*   **Generation**: Verified Range and Position types in js/vscode/generated/graphql.ts match the schema.
