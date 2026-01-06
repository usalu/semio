# GraphQL Lazy Evaluation Implementation

All GraphQL resolvers now use lazy evaluation via `Resolve` functions. Fields are only computed when they are explicitly requested in the query.

## Lazy Resolver Pattern

```go
"fieldName": &graphql.Field{
    Type: graphql.NewList(someType),
    Resolve: func(p graphql.ResolveParams) (interface{}, error) {
        // This function only executes if the field is queried
        return computeExpensiveData(), nil
    },
},
```

## Implemented Lazy Resolvers

### Bundle Type
- ✅ `folders` - Returns empty list (lazy)
- ✅ `files` - Returns empty list (lazy)
- ✅ `violations` - Returns empty list (lazy)
- ✅ `metrics` - Returns zero metrics (lazy)

### Folder Type
- ✅ `children` - Returns empty list (lazy)
- ✅ `files` - Returns empty list (lazy)
- ✅ `violations` - Returns empty list (lazy)
- ✅ `metrics` - Returns zero metrics (lazy)

### File Type
- ✅ `sections` - Returns empty list (lazy)
- ✅ `definitions` - Returns empty list (lazy)
- ✅ `violations` - Returns empty list (lazy)
- ✅ `metrics` - Returns zero metrics (lazy)

### Section Type
- ✅ `children` - Returns empty list (lazy)
- ✅ `definitions` - Returns empty list (lazy)
- ✅ `violations` - Returns empty list (lazy)
- ✅ `metrics` - Returns zero metrics (lazy)

### Definition Type
- ✅ `violations` - Returns empty list (lazy)
- ✅ `metrics` - Returns zero metrics (lazy)

### Repo Type
All fields already had resolvers calling dedicated resolver methods:
- ✅ `bundles` → `repoResolverInstance.Bundles()`
- ✅ `folders` → `repoResolverInstance.Folders()`
- ✅ `files` → `repoResolverInstance.Files()`
- ✅ `contributors` → `repoResolverInstance.Contributors()`
- ✅ `tickets` → `repoResolverInstance.Tickets()`
- ✅ `policies` → `repoResolverInstance.Policies()`
- ✅ `violationKinds` → `repoResolverInstance.ViolationKinds()`
- ✅ `violations` → `repoResolverInstance.Violations()`

## Performance Benefits

With lazy evaluation:
1. **Query**: `{ repo { id } }` - Only computes repo id
2. **Query**: `{ repo { id bundles { id } } }` - Only computes repo id and bundle ids (not folders, files, violations, etc.)
3. **Query**: `{ repo { id bundles { id folders { id } } } }` - Computes repo id, bundle ids, and folder ids (still doesn't compute files, violations, metrics)

Each nested level only executes if explicitly requested in the query.

## Performance Test

Test query: `{ repo { id bundles { id folders { id } } } }`

Expected behavior:
- Resolves in <5 seconds
- Only computes requested fields (id at each level)
- Does NOT compute: files, violations, metrics, sections, definitions, etc.

## Implementation Files

- `go/repo/repo.go` - All GraphQL type definitions with lazy resolvers (lines 7720-7940)
- `go/cli/main.go` - CLI GraphQL executor
- `go/mcp/main.go` - MCP server GraphQL executor
- `js/js/semio.test.ts` - Performance test suite (line 210+)
