# Vitest Orphan Suite Recovery Report

Ticket: `2026/08/19/VITEST-ORPHAN-SUITE-RECOVERY` (opened manually — repo MCP unavailable in session).

Evidence source audit: `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-web-kernel-package-report.md` (read-only).

## Before / after (exit codes via `echo EXIT_CODE:$?` after each command)

### `@semio-tech/cad-js`

**Before**
```
Test Files  8 failed (8)
     Tests  no tests
Error: Cannot find package '@semio-tech/kernel-3d-js'
EXIT_CODE:1
```

**After**
```
Test Files  8 failed | 1 passed (9)
     Tests  89 failed | 87 passed (176)
EXIT_CODE:1
```

Per-file collection (after): runtime 2f, actions 1f, artifact suite-fail (0 collected), stately 4f, inferences 21f, geometry 23f/21p, brepjs 1f/29p, renderer 37f/32p, spatial-kernel spatial 5p (all green).

Dominant genuine failure class once suites load: `ReferenceError` for module-level caches (`defaultModelDefinitionIdCache`, `typologyStyleCache`, `modelDefinitionInteractionCatalog`, …) — split/move regressions in geometry/artifact, not vitest config.

### `@semio-tech/infinite-canvas-react-renderer`

**Before**
```
No test files found, exiting with code 0
EXIT_CODE:0
```

**After**
```
Test Files  1 passed (1)
     Tests  1 passed (1)
EXIT_CODE:0
```

### `@semio-tech/infinite-world-r3f`

**Before**
```
No test files found, exiting with code 0
EXIT_CODE:0
```

**After**
```
Test Files  1 passed (1)
     Tests  100 passed (100)
EXIT_CODE:0
```

### `@semio-tech/framework-os-mcp` (include/includeSource de-dup)

**Before:** 5 files / 22 tests / EXIT 0  
**After:** 4 files / 20 tests / EXIT 0

### `@semio-tech/framework-os-shell` (include/includeSource de-dup)

**Before:** 2 files / 6 tests / EXIT 0  
**After:** 1 file / 3 tests / EXIT 0

### `@semio-tech/animate-js` (include/includeSource de-dup only)

**Before:** 2 failed suites / 0 tests / EXIT 1 (`animate-present-core` alias → deleted `🎛️apps/🎬️present/…` path)  
**After:** 1 failed suite / 0 tests / EXIT 1 (same missing package; doubling removed)

### `@semio-tech/cad-js-module-aec-building` (extension include/includeSource de-dup)

**After sample:** 1 failed suite / EXIT 1 (`runtime.bootstrapCadModules` undefined at module init)

## Config / wiring changes

1. **cad-js `🧪️vitest.config.ts`**: `DOMAIN_FILES` → artifact `✏️editor/⚙️engine/…`; `include: []` + `includeSource` only; `@vitejs/plugin-react`; fixed `jsx-dev-runtime` alias.
2. **`@semio-tech/kernel-3d-js` → `@semio-tech/s-3d-js`** in 9 source files (3× spatial-kernel + 6× cad artifact).
3. **Stale relative imports** after app→artifact move (spatial-kernel ↔ cad editor paths, runtime geometry import, renderer brepjs test import).
4. **`📦️index.ts`**: schema/io paths under `🏅️standards/🔖️1/🪆️subsets/✳️any/…`; merged `core` export (geometry + spatial + registry); removed dead `cad_decomposer` export (target file absent).
5. **`interactionCompileCacheClear`**: export from registry + import in artifact (module load blocker).
6. **Infinite canvas/r3f**: `includeSource: ["../../🟦️component.tsx"]`, `passWithNoTests: false`, r3f workspace aliases + react jsx aliases.
7. **include/includeSource de-dup**: mcp, shell, animate, 4× cad extensions (`include: []`, in-source only in `includeSource`).

## Net new tests executing

| Project | Δ tests running |
|---|---|
| cad-js | 0 → **176** |
| infinite-canvas | 0 → **1** |
| infinite-world-r3f | 0 → **100** |
| **Total** | **+277** |

## Remaining gaps (reported, not suppressed)

- **cad-js**: 89 failing assertions / load-time errors — real product bugs surfaced by first full run.
- **animate-js**: `@semio-tech/animate-present-core` implementation path still missing (`🎛️apps/🎬️present/…` deleted); 136 in-source tests still unreachable until that package is restored or aliased to its artifact successor.
- **cad extensions**: need runtime export surface for `bootstrapCadModules` in vitest graph (extension tests call `register()` at import time).

## passWithNoTests

Set `passWithNoTests: false` on infinite canvas + r3f configs. Left unchanged elsewhere in this pass (flow/server/coordinator/ui-react still `true`).
