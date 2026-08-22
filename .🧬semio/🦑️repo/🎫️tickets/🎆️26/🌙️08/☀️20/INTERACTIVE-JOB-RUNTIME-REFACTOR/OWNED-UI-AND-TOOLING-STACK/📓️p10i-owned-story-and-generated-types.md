# P10i Owned Story and Generated Types

## Scope

This packet removes five undeclared source imports and one stale direct type row without adding declarations, allowlists, or suppressions:

- The Tree interaction story now uses the owned `@semio-tech/ui-react/test` DOM-event/query boundary and the package-owned test runner instead of `storybook/test`.
- The repository TSX fixture relies on JSX return-type inference instead of importing the external React JSX namespace.
- The demonstrator footer owns its small structural logo-style contract instead of importing React solely for `CSSProperties`.
- The VS Code generated-query section owns its typed document artifact contract. Both generated imports of `@graphql-typed-document-node/core` are gone while result/variable phantom typing and `DocumentType` inference remain.
- The renderer removed `@types/katex`; the installed KaTeX package already publishes its own `types/katex.d.ts` declaration entry.

## Verification

- `bun ./📜️script.ts nx run @semio-tech/ui-react:typecheck --skip-nx-cache`: passed after the owned story interaction migration.
- `bun install --ignore-scripts`: passed and updated the Bun lockfile.
- `bun ./📜️script.ts verify dependencies`: passed at 180 third-party identities, 58 below the frozen 238 baseline, with no additions.
- `bun ./📜️script.ts verify dependencies parity js`: expected Phase 10 red exit at 83 manifests, 303 external rows, 142 evidenced rows, 161 unowned rows, and 32 undeclared imports.
- The VS Code package router's stale missing math/graph DSL import was repaired to use the canonical repository script library, preserving the mandated single-`📜️script.ts` route.
- `bun ./📜️script.ts nx run @semio-tech/repo-vscode:test-quick --skip-nx-cache`: passed; the project intentionally has no quick-level suite and reserves its extension-host suite for long/exhaustive levels.
- The repo coordinator's two stale script-library paths were repaired to the same canonical library.
- `bun ./📜️script.ts nx run @semio-tech/repo-coordinator:test-quick --skip-nx-cache`: passed; Vitest correctly found no quick-level test files and exited zero.

The preceding checkpoint had 37 undeclared imports and 304 external rows. This packet therefore removes exactly five ownership findings, one direct external row, and one repository-wide dependency identity.

## Status

The owned story/generated-type changes and dependency ratchet are green. Phase 10 remains open until the residual external UI/build/test implementations and all remaining parity findings are replaced.
