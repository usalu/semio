# Filling Missing Terrain Samples

In the Terrain editor, use **Edit Elevation**, then select **Fill Missing Samples**. Enter the column from the west, the row from the south, a radius in samples, and the replacement height in metres. A radius of zero fills one sample. The command uses the same circular sample footprint as the elevation brush. It changes missing samples only and admits exact signed millimetres, including zero.

The typed `EditElevation` action publishes one `ReplaceElevationSource` mutation. The inverse restores the original mask and heights. Filling the last gap omits the canonical mask. Existing elevations, WGS84 bounds and overlays retain their original values. Repeating a fill on a fully present area yields no mutation.

The shared JSON fixture uses fractional negative replacement, a partial mask, a zero-height repair, and a no-op. It also covers invalid strokes, sub-millimetre input and a coordinate outside the source grid. The live Rust raster reducer passed the isolated fixture. Ajv and the independent Math.js raster calculation passed all three action tests with 102 assertions. The mounted test uses the typed file and edit commands, checks one artifact publication per edit, and asserts the exact snapshot over two Undo and Redo steps. Native mounting remains pending on the current build. The browser test accepts `--repair-nodata` and verifies exact exported bytes around replay.

[Browser acceptance](verify-terrain-maximum-grid-browser.mjs) · [Native reducer witness](verify-terrain-fill-missing.py)
