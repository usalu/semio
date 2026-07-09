# Verify log — App Isolation and Enforced Boundaries

## Follow-up fixes (2026-07-03)

### PlayHost isolation

- Split all 23 `*PlayHost` regions from app `react/index.tsx` into `play-host.tsx`
- Added `"./play": "./play-host.tsx"` export on each app react package
- Updated all `bootRenderer` call sites to `import("@semio-tech/<app>-react/play")`
- Prevents canvas/library imports (e.g. `@semio-tech/puzzle-2d-react` from platform renderer) from loading playground shell + circular framework deps

### Circular import fixes

- `puzzle/2d/core`: lazy `puzzle2dPlayLodTiers()` instead of module-init `getPuzzle2dLodScale()` call
- `puzzle/2d/play-host`: dynamic import for `WriterCanvas` / `createWriterDocument`
- `writer/play-host`: dynamic import for trinity Jack LSP worker before `WriterCanvas` mount

### Vite / vitest

- `createWorkspaceViteResolveConfig`: removed scene-host path aliases (broke vitest jsdom `@react-three/drei` resolve); dedupe retained
- `flow/react`: restored `flowOverlayLabelFill` export (delegates to `dagOverlayLabelFill`)

### dependency-cruiser

- Fixed invalid `local-dev` dependency type → `local` in `.dependency-cruiser.cjs`
- Repo-wide cruise: **965 modules, 0 violations**

## Verification results

| Check                                                                                        | Result                                              |
| -------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| `layout-react` tests                                                                         | 8 pass                                              |
| `flow-react` tests                                                                           | 142 pass                                            |
| `repo/lib/js` boundary + manifest tests                                                      | 75 pass (2 pre-existing micro-commit hook failures) |
| `dependency-cruiser` (compose, framework, flow, layout, puzzle, ui, draw, note, sequence, s) | 0 violations                                        |
| Flow dev port 6016                                                                           | HTTP 200 (existing server)                          |
