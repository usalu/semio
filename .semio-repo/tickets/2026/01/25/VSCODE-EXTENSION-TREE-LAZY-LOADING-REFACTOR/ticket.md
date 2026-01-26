# Ticket

## Todos

- [x] Fix --format jsonl to --json flag in urql client
- [x] Fix runRepoCommandJson to also use --json flag
- [x] Refactor CodebaseProvider for lazy loading
- [x] Fix tickets, policies, and contributors providers to not depend on getCodebase()
- [x] Update contributor avatar path to new location

## Changes

### Critical Fix: CLI Flags
- Changed `["--format", "jsonl", "graphql", query]` to `["--json", "graphql", query]` in the urql client fetch function
- Changed `runRepoCommandJson` to use `--json` flag and parse JSONL output via `parseRepoEvents` and `extractRepoResult`
- The `--format jsonl` flag didn't exist in the repo binary, causing it to output human-readable format with emoji characters that couldn't be parsed as JSON

### CodebaseProvider Lazy Loading
- Root level now fetches only bundles via `fetchBundlesViaGraphQL()` instead of loading entire codebase
- Bundle/folder expansion uses `fetchFolderContent(path)` for incremental loading
- File expansion loads sections/definitions via `runRepoCommandJson` on demand
- Added local bundle cache to avoid repeated fetches

### Provider Independence
- TicketsProvider: Now calls `fetchTicketsViaGraphQL()` directly instead of depending on `getCodebase()`
- PoliciesProvider: Now calls `fetchPoliciesViaGraphQL()` directly
- ContributorsProvider: Now calls `fetchContributorsViaGraphQL()` directly
- Fixed contributor avatar path from `contributors/` to `.semio-repo/contributors/`

### GraphQL Range Selection Fix
- Updated section and definition GraphQL queries to request line/column subfields for start/end positions
- Aligned CLI and MCP section/definition queries with Position selection requirements so tooling can parse ranges

### Section List Query Repair
- Rewrote the section list GraphQL query to use valid nested selections
- Rebuilt the repo CLI binary so tooling runs the corrected query

## Log

- Fixed additional issue: `runRepoCommandJson` was also missing the `--json` flag
- Rebuilt and reinstalled extension
- Updated repo GraphQL queries to include Position subfields for section/definition ranges
- Rebuilt repo CLI binary after correcting the section list query

## Summary

Repair section list query syntax, rebuild the repo CLI, and document nested section selection for the VS Code Sections view.
