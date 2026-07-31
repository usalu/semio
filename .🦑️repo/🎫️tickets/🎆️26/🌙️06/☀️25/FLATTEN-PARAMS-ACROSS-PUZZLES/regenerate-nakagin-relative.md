# Nakagin fixture regeneration

Sketchpad design surfaces now emit **relative** puzzle fixtures (identity object origins, local vortex geometry, connection transform params on edges/attractions). Absolute placement is computed by `prepareTopologyModel` → `flatten5d` in the platform FiveD render path.

To regenerate standalone puzzle play fixtures from compose:

1. Export relative `puzzle.2d` / `puzzle.3d` fixtures from the Nakagin design via `sketchpadDesignPuzzle2dFixtureFromDesign` / `sketchpadDesignVolumeFixtureFromDesign` with the metabolism kit loaded.
2. Run puzzle 5d play regeneration: `REGENERATE_NAKAGIN_5D=1 bun ./📜️script.ts test -t "regenerates nakagin 5d fixture"` in `puzzle/5d/play`.

Static nakagin 2d/3d JSON under `puzzle/*/fixture/` still carry legacy absolute poses until re-exported from compose.
