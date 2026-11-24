# Norm visual-chrome lib tests green (2026-11-24)

Crate `semio-s-plugin-norm-app-surface-visual-chrome`, run from its `📦️packages/🦀️rust` directory.

## Causes and fixes
1. `golden_layers_match_the_committed_fixture`
   - Cause: `canvas-lowering` (`path_layer`, `text_layer`, `hatch_layer`) emits the framework vector wire format (flat `kind`/`x`/`y`, `text:{content,size,fontFamily}`, `segments`, float rgba), introduced in commit 5db034ffe53. The fixture still held the older `transform`/`node` shape. The new format is the intended one: it is the shape the framework vector admission (`🚪️io/🔌️ports/🧩️admission/🧵️vector`) and the renderer's wgpu canvas2d scene consume.
   - Fix: re-blessed with the test's own mechanism (`SEMIO_UPDATE_GOLDEN=1 repo cargo ... test -p ... --lib golden_layers --offline`). The diff is the single-line fixture, old shape to new shape, ids unchanged.
   - Other families: no other fixture in the Norm tree holds the stale `transform`/`node` layer shape (searched all `🧫️fixtures`); the scene goldens of EN 1992/1996/1998/1999 are scene wire, not diagram layers, and need no re-bless.
2. `hit_distances_match_shapely` ("no Python interpreter")
   - The Python environment existed (`.cache/b/d/m/python/environments/workspace-*`), and `provisionedPythonV1` resolves it. But `repo cargo` goes through `launchRepositoryCargoV1` with `process.env`; only the `runCargoTestsV1` and command-configuration paths called `withProvisionedPythonV1`, so `SEMIO_PYTHON` never reached `repo cargo` test binaries.
   - Fix: `withRepositoryCommandEnvironmentV1` (the single environment wrapper of every owned repo verb) now sets `SEMIO_PYTHON` from `withProvisionedPythonV1(environment, root)` (an explicit value still wins; restored afterwards like the other patched keys).

## Changed files
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🧪️tests/🧫️fixtures/🔣️layers.json` (re-blessed)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📥️context/🟦️.ts` (`SEMIO_PYTHON` in the environment patch via new `repositoryPythonPatchV1`; the apply-and-restore part of `withRepositoryCommandEnvironmentV1` extracted as `withProcessEnvironmentPatchV1`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📥️context/🧪️tests/🟦️.ts` (new test)

## Test lines
- `--lib` (all): `test result: ok. 62 passed; 0 failed; 0 ignored` (was 61 passed, 1 failed after the re-bless; before: diagrams2d 19 passed, 2 failed).
- No `[DEBUG]` was added.

## Follow-up
- none

## SEMIO_PYTHON wiring test (added on request)
New `📥️context/🧪️tests/🟦️.ts` (bun, suite build-command-line) covers four cases against `repositoryPythonPatchV1` and `withProcessEnvironmentPatchV1`, the two pieces `withRepositoryCommandEnvironmentV1` now composes (extracted so the wiring is testable without an owned invocation):
- a provisioned environment sets `SEMIO_PYTHON` to its interpreter;
- an explicit value wins;
- without an environment the key stays unset (patch value undefined, key absent in `process.env` inside the body);
- the key is restored afterwards (previous value, also when the body throws, and absence when it was unset).

Run: `repo test-file <path>` gave `passed ... (build-command-line, 1.3 s, 4 pass, 0 fail)`, `test-file: 1 passed, 0 failed`.
Not re-run after the refactor: the Cargo lib suite (62 passed was measured before the extraction; behavior is identical).

## Verification after the extraction
- visual-chrome `--lib` (`repo cargo --store=shared --configuration=prototype -- test -p semio-s-plugin-norm-app-surface-visual-chrome --lib --offline`): `test result: ok. 62 passed; 0 failed; 0 ignored`, including `hit_distances_match_shapely` and `golden_layers_match_the_committed_fixture`.
- `repo test-file` on `🏗️build/🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts` (suite build-workspace-graph-laws): `3 pass, 0 fail`; `test-file: 1 passed, 0 failed`.
- `repo test-file` on the new `📥️context/🧪️tests/🟦️.ts`: 4 pass, 0 fail (above).
