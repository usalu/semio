# Verify Log — S Technology Extension Loading

## Automated tests (2026-07-02)

- `bun nx run s-core:test` — 13/13 passed (includes new VCS handler round-trip test)
- `bun nx run s-play:test` — 11/11 passed (registry completeness, extension bootstrap)

## SAppHostRouter manual sample checklist

| Technology      | Host                                | Write-back                              |
| --------------- | ----------------------------------- | --------------------------------------- |
| draw            | DrawCanvas                          | patchAppSource                          |
| raster          | RasterCanvas                        | applyAppOperation                       |
| lowpoly         | SLowpolyHost + LowpolyCanvas        | patchAppSource                          |
| vcs             | SVcsHost + HistoryTable             | applyAppOperation setCounter            |
| trinity/jack    | Writer + TrinityCanvas jackDispatch | patchAppSource                          |
| trinity/rewrite | STrinityRewriteHost composite       | patchAppSource (trinity graph)          |
| puzzle2d        | SPuzzle2dHost                       | patchAppSource                          |
| puzzle3d        | SPuzzle3dHost                       | patchAppSource (brush/connect/relocate) |
| puzzle5d        | SPuzzle5dHost                       | patchAppSource                          |
| gismap          | SGisMapHost + Position/Route        | fixture-bound render                    |
| catalogue       | SCatalogueHost                      | read-only kind list                     |
| presentation    | PresentationDeck + JSON editor      | patchAppSource                          |

## Notes

- CAD still boots isolated `CadPlayRoot` (full play chrome); external model injection deferred.
- Layout canvas is blueprint view-only (`LayoutCanvas` has no document change callback yet).
