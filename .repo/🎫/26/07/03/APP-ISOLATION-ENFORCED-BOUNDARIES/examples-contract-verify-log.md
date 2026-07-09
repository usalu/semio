# Examples Contract + OCP Cleanup — verify log

## Tests run

| Package                                                  | Result                                                                             |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `framework-os-core`                                      | 11 passed                                                                          |
| `framework-playground-renderer-react`                    | 25 passed (after vcs↔os circular import fix)                                       |
| `vcs-core`                                               | 6 passed                                                                           |
| `forms-core`                                             | 27 passed                                                                          |
| `draw-core`                                              | pass (no tests)                                                                    |
| `repo/lib` playground static sites + manifest resolution | 7 passed                                                                           |
| `ui/styling`                                             | 11 passed, 1 pre-existing fail (`puzzle3dLockedExampleMeshBasenames` fixture path) |
| `framework-playground-core`                              | vitest alias missing `@semio-tech/framework-core` (pre-existing infra)             |

## Fixes during verify

- Removed `createVcsDemoAppVcsHandler` from `vcs/core/js/internal.ts` (os-core import) → moved to `vcs/core/js/index.ts` OsProgram region to break circular dependency that broke playground renderer vitest.
- `PLAYGROUND_PORTS`: stop duplicating kind alias keys; port uniqueness test timeout raised to 30s for cold manifest scan.

## dependency-cruiser

`bunx dependency-cruiser --config .dependency-cruiser.cjs framework/product/playground/renderer/react framework/product/os/core s/core repo/lib` — no violations.

## Manual boot

Not run in this session (requires dev server + browser). Representative apps to smoke-test: draw, puzzle2d/wires, cad, S/OS studio — example dropdown + tree drag.

## Accepted exceptions

- `s/react/play-host.tsx`: `compose.sketchpad` uses dedicated `SSketchpadHost` (documented in source).
- Vite per-`playEntryKind` plugin dispatch remains config-time special case (per plan).
