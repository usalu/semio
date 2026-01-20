- Started investigation into VS Code extension issues.

## 10:41 - Fix Diagnostics and Autofixes

I analyzed the `repo analyze` output and finding that the `kind` field is an object, not a string as expected by the VS Code extension. I also found that the `summary` field was missing from the returned data.

To fix this:
1.  **Updated Backend**: Modified `go/repo/main.go` to include the `summary` field in the analyze and fix queries.
2.  **Updated Frontend Interface**: Refactored the `Violation` interface in `js/vscode/extension.ts` to align with the backend object shape.
3.  **Refactored Logic**:
    *   Updated `updateFileDiagnostics` to correctly parse the violation kind ID by stripping the prefix from the `violation.kind.id` object property.
    *   Updated `createRepoCodeAction` to check `violation.kind.autofixable` boolean instead of using a hardcoded list or string check.
    *   Fixed TypeScript compilation errors in `resolveTicketData` and other utility functions caused by the interface changes.

Verified that the TypeScript code now compiles and matches the backend schema.

Next: Investigate and fix Tree Views.

## Tree Views Fix
Investigated the issue where tree views were not loading.
Found that `js/vscode/extension.ts` was sending a JSON payload `{ "query": "...", "variables": ... }` to the `repo graphql` command.
Verified that the `repo graphql` command in `go/repo/main.go` only accepted a raw query string as the first argument, causing it to fail with a syntax error.
Updated `go/repo/main.go` to detect and unmarshal JSON payloads in the first argument, supporting the protocol used by the VS Code extension's `urql` client.
Rebuilt the `repo` binary.
Verified that `repo graphql` now accepts the JSON payload and returns the correct data structure, fixing the tree views.

## Completion
Fixed all identified issues:
1. Diagnostics: Fixed `Violation` interface in `extension.ts` to handle the object structure correcty (specifically the `kind` object).
2. Autofixes: Fixed `summary` property access in `RepoCodeActionProvider`.
3. Tree Views: Fixed backend (`repo`) to support frontend's GraphQL request format.
4. Verified compilation of `extension.ts`.
