# P10h Owned UI Test Boundary

## Scope

This packet introduces the owned `@semio-tech/ui-react/test` contract and routes every renderer-owned use of `@testing-library/react` through it. The adapter exports only repository-owned contracts over platform DOM types; no public signature exposes a Testing Library type. The UI package already owns the temporary external implementation row, so the renderer no longer reaches across package ownership or imports the adapter directly.

The owned surface currently contains bounded fixture rendering, cleanup, semantic document queries, click/change events, synchronous update flushing, and deadline-bounded assertion polling. The package exports it only through the explicit `./test` subpath. The renderer Vitest alias maps that subpath before the existing main UI alias.

## Verification

- `bun ./📜️script.ts nx run @semio-tech/ui-react:typecheck --skip-nx-cache`: passed.
- Renderer source scan: zero direct `@testing-library/react` imports remain; nine imports now address the owned test boundary.
- `bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache`: the owned boundary resolved and executed 436 tests; 422 passed. Fourteen existing renderer assertions and two existing worker `postMessage` rejections remain red, including stale fixture/component expectations unrelated to the adapter. The preceding renderer-wide typecheck is likewise red on the existing cross-package schema/R3F/worker errors; the new UI test adapter itself is covered by the passing UI package typecheck.
- `bun ./📜️script.ts verify dependencies parity js`: expected Phase 10 red exit with 83 manifests, 304 external rows, 142 evidenced rows, 162 unowned rows, and 37 undeclared imports.

The preceding checkpoint contained 46 undeclared imports, so this packet removes exactly nine genuine ownership findings without adding a declaration, allowlist, or suppression.

## Status

The ownership boundary and focused UI type gate are green. Phase 10 remains open, and the external Testing Library implementation remains scheduled for replacement by the owned DOM driver before its dependency row can be deleted.
