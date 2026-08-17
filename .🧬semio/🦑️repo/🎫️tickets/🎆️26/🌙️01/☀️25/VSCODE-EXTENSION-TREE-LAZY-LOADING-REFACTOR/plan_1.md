# VSCode Extension Tree Lazy Loading Refactor

## Problem Analysis

### Root Cause: Wrong CLI Flag

The extension uses `--format jsonl` flag but the repo binary expects `--json`:

- Current: `["--format", "jsonl", "graphql", query]`
- Expected: `["--json", "graphql", query]`

This causes the repo binary to output human-readable format with emoji characters like "→ found 33 bundles" which cannot be parsed as JSON.

### Secondary Issue: No Lazy Loading

The CodebaseProvider currently tries to load the entire codebase tree at once via the `CodebaseDocument` GraphQL query, which fetches all bundles, folders, files, sections, and definitions in a single request.

## Solution

### 1. Fix the CLI Flag (Critical)

Change `["--format", "jsonl", "graphql", query]` to `["--json", "graphql", query]` in the urql client fetch function at line 120.

### 2. Implement Lazy Loading Architecture

The codebase tree should load incrementally:

```
Root Level (no element)
└️─️ Bundles (fetch bundles query)
    └️─️ Bundle expanded → fetch folder(path: bundle.root)
        └️─️ Folder expanded → fetch folder(path: folder.path)
            └️─️ File expanded → fetch file(path: file.path) for sections/definitions
                └️─️ Section expanded → show child sections and definitions
```

### GraphQL Queries for Lazy Loading

1. **Root Level** - `query { bundles { id name root uri } }`
2. **Bundle/Folder Content** - `query { folder(path: $path) { children { path name uri } files { path name uri } } }`
3. **File Content** - `query { file(path: $path) { sections { id name path range { start end } } definitions { id name kind range { start end } } } }`

### 3. Provider Updates

#### CodebaseProvider Changes

- Remove dependency on `getCodebase()` for the entire tree
- Root `getChildren()` calls `fetchBundlesViaGraphQL()`
- Bundle/Folder expansion calls `fetchFolderContent(path)`
- File expansion calls new `fetchFileContent(path)` for sections/definitions

#### Other Providers

- TicketsProvider, PoliciesProvider, ContributorsProvider should use their dedicated queries directly instead of relying on the codebase cache
- This ensures they work independently even if codebase loading fails

## Implementation Steps

1. Fix CLI flag: `--format jsonl` → `--json` in urql fetch
2. Add `fetchFileContent(path)` GraphQL helper
3. Update `CodebaseProvider.getChildren()` for lazy loading
4. Update tickets/policies/contributors providers to not depend on codebase cache
5. Add proper loading states and error handling

## Files to Modify

- `js/vscode/extension.ts` - Main extension file with all providers and urql client
