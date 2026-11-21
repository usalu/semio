# Editing terrain and terrain data in s

The registered GIS Terrain editor exposes **Create Flat Grid** and **Edit Elevation** in Actions. A flat grid has four WGS84 bounds and an initial height. The elevation action can set a sample, raise or lower an area, grade toward a height, smooth nearby samples, or offset the whole grid. Inputs use west-to-east columns and south-to-north rows, numbered from 0 through 32. Elevation values are signed whole meters. Smoothing accepts a percentage. Edits that would overflow the elevation representation are refused before publication.

The viewport utilities **Raise Terrain**, **Lower Terrain**, and **Smooth Terrain** use the same edit action. They pick the visible elevation mesh and address the nearest sample. A click applies the brush once. The registered Actions form supplies explicit radius and strength controls for edits that need precise values. **Grade Region** applies a target elevation to samples covered by an existing GIS polygon and preserves its holes. **Terrain Data** shows the grid dimensions, elevation range, mean height, geographic extent, and overlay feature IDs.

GIS overlay actions add or move positions, create or extend routes and regions, and remove or clear features. Terrain source actions preserve GIS overlays. Overlay actions preserve the elevation source. Both use the same Semio artifact store and Undo and Redo history. ESRI ASCII Grid import starts from the normal shell file picker and feeds the elevation replacement mutation. The corresponding export uses the standard shell download route. It requires equal angular spacing on both axes.

## Current verification

- The three terrain command fixtures passed all 55 assertions using Ajv, math.js, and an independent triangulation and geometry check. The checked-in Rust tests cover the neutral cases plus the mounted artifact, scene projection, and undo and redo.
- The overlay fixture passed independently. Rasterio, GDAL and SciPy validated the ASCII source fixture, and a focused Rust probe passed both tests against the actual ASCII parser and exporter. Mounted Rust editor and host suites are still progressing through repository dependencies. The browser and native renderer results in the prior integration notes cover embedding and draping. The new click-to-edit paths need their own verification.
- The focused Surface ray-pick test passed against the actual elevation mesh. The first native brush journal compiled and reached input handling. A primary drag exposed an unclaimed input fault. The native brush now consumes primary drag and move events while the separate camera gestures stay active. The corrected journal is queued.
- This checkout still has no published workspace graph. The Nx dev launcher previously reached provisioning and refused an authenticated derivative material reference. The browser and native dev launchers are using the repository's Semio backend as the fallback. A live s session has yet to open.

The current source representation is a single 33 × 33 grid. Larger imported rasters are resampled. Direct brush tools apply click edits, while precise numeric operations use Actions.
