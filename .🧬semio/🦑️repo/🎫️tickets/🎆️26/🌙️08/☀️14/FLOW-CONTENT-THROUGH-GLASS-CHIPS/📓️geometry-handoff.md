# Geometry Contract Handoff

## Implemented

- Added the domain-neutral `window-silhouette-geometry/v1` contract to the existing Chrome element.
- Added deterministic chip-span normalization, metric normalization, LTR/RTL edge construction, zero-inset content outline/path/clip polygon, inset border path, body/glass/content regions, safe clearances, containment, pending metrics, and complete geometry creation.
- Imported and explicitly re-exported the contract from the React barrel.
- Added parity-oriented inline tests in the existing React barrel test region for malformed/reversed/multi-chip normalization, derived ready geometry, containment, pending geometry, and chipless cutout bands.

## Frozen Export Surface

- `WINDOW_SILHOUETTE_GEOMETRY_SCHEMA`
- `WINDOW_SILHOUETTE_PATH_INSET`
- `WINDOW_SILHOUETTE_CHIP_EPSILON`
- `WindowSilhouetteDock`
- `WindowSilhouetteRegion`
- `WindowSilhouetteSafeClearances`
- `PendingWindowSilhouetteMetrics`
- `WindowSilhouetteGeometry`
- `normalizeWindowSilhouetteChips`
- `normalizeWindowSilhouetteMetrics`
- `windowSilhouetteEdgePoints`
- `windowSilhouetteEdgePointsRtl`
- `windowSilhouetteOutline`
- `simplifyWindowSilhouetteOutline`
- `windowSilhouetteOutlineViolations`
- `windowSilhouettePathFromOutline`
- `windowSilhouettePath`
- `windowSilhouetteContentClipPath`
- `windowSilhouetteSafeClearances`
- `windowSilhouetteBodyRegion`
- `windowSilhouetteGlassRegions`
- `windowSilhouetteContentRegions`
- `windowSilhouetteRegionContains`
- `windowSilhouetteContains`
- `pendingWindowSilhouetteMetrics`
- `createWindowSilhouetteGeometry`

## Current Blocker

The pure implementation was copied into the Chrome element and imported/re-exported by the React barrel, but the previous duplicate implementation remains in the React barrel from `WINDOW_SILHOUETTE_PATH_INSET` through `createWindowSilhouetteGeometry`. Work was explicitly frozen before that duplicate could be removed. The barrel therefore currently has duplicate declarations and requires the compositor owner to delete the old block before typecheck can succeed.

## Files Changed By This Workstream

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️Chrome/🟦️component.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️14/FLOW-CONTENT-THROUGH-GLASS-CHIPS/📓️geometry-handoff.md`
- `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️14/FLOW-CONTENT-THROUGH-GLASS-CHIPS/🧪️geometry-validation.txt`

