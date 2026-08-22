# Phase 10 Source-Empty JavaScript Rows

## Scope

This packet removes four external manifest rows with no live import, configuration reference, package-script reference, or dynamic loader evidence outside manifests and the dependency census: `@types/reveal.js`, `its-fine`, `jose`, and `jsonc-parser`.

The affected packages are Animate TypeScript, infinite-canvas React renderer, repo coordinator, and repo VS Code. No source API, generated fixture, allowlist, root command, or dependency-gate severity changed.

## Implementation

- Removed `@types/reveal.js` from Animate. Reveal's installed package owns the types actually consumed by the renderer; the separate ambient package had no source/config evidence.
- Removed `its-fine` from infinite-canvas React renderer. The package has no import or dynamic load of it.
- Removed `jose` from repo coordinator. Authentication source has no import or dynamic load of it.
- Removed `jsonc-parser` from repo VS Code. Extension source and scripts have no import or dynamic load of it.
- Regenerated `bun.lock` with `bun install --ignore-scripts`.

## Verification

```text
bun x nx run @semio-tech/animate-js:test-quick --skip-nx-cache
```

Exit `0`; the package currently declares no quick test files.

```text
bun x nx run @semio-tech/infinite-canvas-react-renderer:test-quick --skip-nx-cache
```

Exit `0`: one file and one test passed.

The repo-coordinator and repo-vscode quick routers remain red before test discovery because their permanent package scripts import already-missing internal modules. Direct TypeScript checks likewise reach the live tree's pre-existing shared `.ts`-extension, missing internal alias, and extension-source diagnostics. Neither gate reports any of the four removed packages or a removed declaration.

```text
bun ./📜️script.ts verify dependencies
```

Exit `0`: 181 current identities against the 238-entry baseline, 57 removals, and no additions.

```text
bun ./📜️script.ts verify dependencies parity js
```

Expected Phase 10 red exit after the concurrent owned artifact-test cohort: 83 manifests, 304 external rows, 142 evidenced rows, 162 unowned rows, and 54 undeclared imports. This packet accounts for exactly four fewer external and unowned rows and does not suppress an undeclared import.

## Closure State

This packet is green. Phase 10 remains open until every remaining external row and undeclared import is replaced by an owned implementation and all zero-dependency gates pass.
