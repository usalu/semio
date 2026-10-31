# Staffage End-to-End Completion & Performance Plan

## Objective & Deliverables
1. **Complete all 107 catalog assets across all 8 categories** (`humans`, `animals`, `vegetation`, `furniture`, `objects`, `vehicles`, `machinery`, `components`) under `semio.staffage/v2`, including installing the 4 `NW-V2` vegetation assets (`bamboo-clump`, `ivy-on-trellis`, `agave`, `hosta-clump`) and syncing `🗂️catalog/🔣️.json`.
2. **Fix all validation, physics, and suitability failures** (`validateAsset`, `validateManifest`, `checkAssetDetailed`, `measureSuitability` under `office-laser`, `🌡️catalog-physics`).
3. **Eliminate mobile/phone and desktop lag**:
   - Fast-path `measureSuitability` when `minGapMm <= 0 && minFeatureMm <= 0` (`SCREEN_PROFILE` / `UNCHECKED`), avoiding redundant multi-pose evaluations, hidden-line removal, and $O(E^2)$ gap checks on every screen render.
   - Pre-filter bounding boxes in `visibleRuns`, `crossings`, `pointInRings`, `drawShapes`, and `gapsBetween`, and reuse distance-transform scratch buffers in `featureWidthMm` and `whiteSliver`.
   - Parallelize category chunk loading, cache `validateAsset` in a `WeakMap`, and batch `CatalogStore` progress notifications so loading 107 assets does not trigger 107 full-tree React re-renders.
   - Throttle `StaffageClock` subscriber notifications to actual frame boundaries (`fps`), add `IntersectionObserver` visibility gating to `PlacedFigure`, cache `DrawingScene` in `sceneOf`, wrap looping clip times for stationary/line previews, reduce mobile `LayeredOverview` `budget`, and apply `content-visibility: auto` to browse grids.
4. **Verify all library and React test suites and production site build**.

## Wave 1 Applied & In Progress
- Core thermal (`kelvin` floor), thermal-view (`null` sky/ground guard), animation (`applyTracks` clamping), representation (`measureOnce` screen fast-path + `gapsBetween` / `squaredDistanceTransform` buffer reuse), depiction (`visibleRuns` / `crossings` / `pointInRings` bbox pruning), and export CLI (`evaluateFrames` scale selector + package barrel exports) updated.
- Four parallel Grok 4.7 Medium Non-fast agents launched for:
  1. React frontend & mobile performance overhaul (`🎯️targets/⚛️react/🔨️modules/**`, `🎨️.css`).
  2. React test suite & `reference-walker` v2 migration (`🎯️targets/⚛️react/🧪️tests/**`, `🧫️fixtures/🚶️reference-walker/🔣️.json`).
  3. Humans & Animals catalog validation and suitability fixes (`🗂️catalog/👤️humans/**`, `🗂️catalog/🐕️animals/**`).
  4. Vegetation, Furniture, Objects, Vehicles, Machinery, and Components catalog fixes (`🗂️catalog/{🌳️vegetation,🛋️furniture,🪑️objects,🚜️vehicles,🏗️machinery,🧱️components}/**`).
