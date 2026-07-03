# Fully Derived Playground Infrastructure — verify log

## Tests

| Package | Result |
|---------|--------|
| `repo-lib` playground static sites + manifest | 7 passed |
| `framework-playground-renderer-react` | 25 passed |
| `draw-core` | 6 passed (subagent) |
| `ui-styling` | 12 passed (after lockedExampleFixtures path fix) |
| dependency-cruiser (framework/playground, s/core, sketchpad, repo/lib) | no violations |

## Architecture after refactor

- **Manifest-only config**: `site`, `assets`, `programIds`, `lockedExampleFixtures`, `optimizeDepsExclude` on `semio.app`
- **Ports/site hosts**: derived from manifest scan (only `storybook` remains as tooling seed)
- **Vite plugins**: union of active app manifest `assets` (not `playEntryKind` switches)
- **Bodies**: `AppRendererContribution.windowBodies` / `sidePanelBodies` — no `registerBodies` on `createPlaygroundApp`
- **Program routing**: `programIdToPlaygroundKind` fully from manifest scan (no `PROGRAM_ID_RESIDUAL`)
- **Sketchpad**: full `semio.app` manifest + `SketchpadInstanceHost` on `sketchpadAppRenderer`; no s/react special-case
- **Env rename**: `PUZZLE_PLAY_ENTRY` → `PLAYGROUND_APP_KIND`

## Fixes during verify

- `collectLockedExampleFixturesFromManifests`: merge paths across manifests (3d + 5d)
- Locked fixture paths corrected to `puzzle/*/example/*.json` (fixture/ paths did not exist)

## Manual boot

Not run in this session — smoke draw, puzzle 2d, cad, gis-2d, S/OS studio + sketchpad instance via launch.json.
