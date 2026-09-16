# Aggregator Masterarbeit Reference in Puzzle 3D

## Symptom

Aggregator (`concrete-forest` / puzzle3d) did not show the `ref-masterarbeit` reference plane. CAD (bearbeiten) still showed its reference overlay.

## Root cause

`WorldReferenceLayer` hides a plane when `referenceMediaPort.loadReferenceTexture` fails (404). The concrete-forest DSL pointed at flat URLs that are not on disk:

- `/infinite-assets/🖼️abbau-aufbau-masterarbeit-grundriss.jpg` (missing)
- `/infinite-assets/🖼️rathaus-ahlen-grundriss.png` (missing)

Infinite assets are served from taxonomy folders (Vite `static-dir` → `♾️infinite/🖼️assets`):

- `/infinite-assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`
- `/infinite-assets/🏛️rathaus-ahlen-grundriss/🖼️.png`

CAD uses a separate served asset (`/cad-assets/🌲️concrete-forest-reference/🖼️.png`) and was unaffected.

## Fix

Correct reference `source.url` values in the concrete-forest puzzle3d DSL example; add a unit test on `world_references_json` for the masterarbeit URL; extend Storybook URL overrides for legacy flat paths.

## Verification

```bash
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
  world_references_json_carries_infinite dsl_asset_parses_and_round_trips
```

After rebuilding the puzzle plugin / refreshing aggregator dev, the masterarbeit plan should appear at `@7,0,0.01` with `widthWorld` 50m (same as before, now with a loadable texture).
