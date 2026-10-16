# Permanent Workspace Static Import Preflight

The Source-only genuine60 gate completed terminal0 with57,417ms remaining. It derives the four exact Bun test roots from the actual permanent BUILD TestScript `runOwnedCommand` AST literal test array, rather than a duplicate hand-maintained root list. Current origin hash is `0b68aa8ff6f244f4e472d2c8dbe841eb8bcace776ab407f4b46c0fd9714bd232`.

Roots:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🟦️.ts`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts`.

Bun.Transpiler.scanImports and the independent TypeScript AST identify literal static/export/require/import references. Relative paths resolve with actual Bun.resolveSync and are checked against independent Node path candidates. Builtins are skipped; external bare package imports are recorded but not traversed. Every scanned Source has bounded regular-file metadata and SHA256, and all observed Sources are reread with matching pre/post stamps and hashes at the end. Application modules are not executed.

Actual result: 509 files, 3635141 bytes, 1690 relative edges, zero unresolved literal imports. Limits remain8192files,64MiB aggregate,4MiB perfile,256pathunits. The four selected roots are the actual existing permanent300 child inputs; this diagnostic has a separate finite60 Original and does not alter that public route. An initial ticket harness argument-index mistake refused zero roots before scanning; its output is retained separately and is not a production/import failure.

Scope limits: 26 computed import expressions are unproved, 46 bare-package references are not traversed, 688 builtin references are skipped; type-only static references are included by TypeScript even where Bun omits them. Native binaries are not loaded. This preflight does not prove dynamic runtime imports, external package acceptance, test behavior, fresh selected graph admission or full public target success.

Retained input: `workspace-imports/📜️script.ts`. Output: `🗑️generated/workspace-static-import-preflight-current.json` and `.log`. No permanent command, runtime Source, SDK, Tools image or graph producer changed. All diagnostic runtime is closed.
