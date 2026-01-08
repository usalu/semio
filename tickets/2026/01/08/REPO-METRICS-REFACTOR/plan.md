# Plan: REPO-METRICS-REFACTOR

## Objective

Remove all metrics from the GraphQL layer and move computation purely into SQLite database views. Extend GraphQL tests to cover all nodes and edges.

## Analysis

### Current State

1. **GraphQL Schema** (`graphql/repo/schema.graphql`):
   - Contains 9 metrics types: `RepoMetrics`, `BundleMetrics`, `FolderMetrics`, `FileMetrics`, `SectionMetrics`, `DefinitionMetrics`, `ContributorMetrics`, `TicketMetrics`, `CheckpointMetrics`
   - Additional helper types: `LineMetrics`, `CountMetrics`, `AnalyzeMetrics`, `PriorityCount`
   - All entity types have `metrics` fields

2. **Go Repository** (`go/repo/repo.go`):
   - Contains corresponding Go struct types for all metrics
   - Schema builder creates GraphQL types for metrics
   - Resolver logic returns stub/computed metrics data

3. **SQLite Schema** (`sql/sqlite/repo/schema.sql`):
   - Already has views for metrics: `repo_metrics`, `bundle_metrics_view`, `folder_metrics_view`, `file_metrics_view`, `section_metrics_view`, `contributor_metrics_view`, `ticket_metrics_view`, `violation_priority_counts`

4. **Tests** (`go/repo/repo_test.go`):
   - Tests individual collections but no comprehensive edge testing
   - No nodes and edges query test

### Target State

1. **GraphQL Schema**: Remove all `*Metrics` types and `metrics` fields from entities
2. **SQLite Views**: Keep existing views (metrics can be queried directly from database)
3. **Tests**: Add comprehensive nodes/edges test covering all node types and degree-one relationships

## Changes

### 1. GraphQL Schema (`graphql/repo/schema.graphql`)

Remove types:
- `RepoMetrics`
- `BundleMetrics`
- `FolderMetrics`
- `FileMetrics`
- `SectionMetrics`
- `DefinitionMetrics`
- `ContributorMetrics`
- `CheckpointMetrics`
- `TicketMetrics`
- `CountMetrics` (only used by contributions, keep if used elsewhere)
- `AnalyzeMetrics` and `PriorityCount` (keep for analyze query)

Remove `metrics` fields from:
- `Repo`
- `Bundle`
- `Folder`
- `File`
- `Section`
- `Definition`
- `Contributor`
- `Ticket`
- `TicketCheckpoint`

### 2. Go Repository (`go/repo/repo.go`)

Remove struct types:
- `RepoMetrics`
- `BundleMetrics`
- `FolderMetrics`
- `FileMetrics`
- `SectionMetrics`
- `DefinitionMetrics`
- `ContributorMetrics`
- `CheckpointMetrics`
- `TicketMetrics`

Keep:
- `LineMetrics` (used for ticket/checkpoint line stats)
- `CountMetrics` (used for contributions)
- `AnalyzeMetrics` and `PriorityCount` (used for analyze query)

Update `buildSchema()` to remove metrics field resolvers.

### 3. Tests (`go/repo/repo_test.go`)

Add comprehensive test:
```go
func TestNodesAndEdges(t *testing.T) {
    // Query all node types and verify IDs are non-empty
    // Query degree-one edges (e.g., file.folder, section.file, violation.kind)
}
```

### 4. MCP Handlers (`go/mcp/main.go`)

Remove metrics references from handler return values if any.

### 5. VS Code Extension (`js/vscode/extension.ts`)

Update GraphQL documents to remove metrics fields.

### 6. gqlgen.yml

Remove model bindings for removed metrics types.

## Execution Order

1. Update GraphQL schema
2. Update Go types and schema builder
3. Update gqlgen.yml
4. Run tests to verify no breakage
5. Add comprehensive nodes/edges tests
6. Update MCP handlers
7. Update VS Code extension
8. Run full test suite
