# Local Elevation Grid Exchange

The registered GIS Terrain editor exposes **Import Elevation Grid…** in its action palette. The action opens the host file picker for a single `.asc` file. Semio’s existing picker wire delivers a complete bounded text callback to the editor. That tool publishes one `ReplaceElevationSource` mutation through the artifact Store. The same elevation source drives the terrain scene, elevation editing, document persistence, and typed media outputs. Importing identical samples is a no-op.

**Export Elevation Grid…** writes `terrain-elevation.asc` through the retained host download effect. The exported west and south coordinates are cell centres, so sample positions and signed metre values survive re-import. Export requires equal longitude and latitude sample spacing because Esri ASCII Grid has one cell size. The Semio snapshot and typed elevation ports preserve rectangular extents.

## Format and Resampling

- Supports Esri ASCII Grid with case-insensitive headers, a square cell size, and paired lower-left corner or centre coordinates. Input coordinates are WGS84 longitude and latitude and elevations are metres. The file carries no CRS declaration, so projected grids must be reprojected beforehand.
- Preserves the first and last source cell centres on each axis. The extent of a singleton axis is one cell wide. Bilinear interpolation uses north-to-south source cell centres and rounds to whole metres on the terrain’s 33 × 33 lattice. Only the current and previous projected rows are buffered while reading the original samples.
- The picker and callback cap input at 24 KiB, below the shell’s single-chunk text boundary. Negative sample values are supported. Invalid geometry, out-of-range elevations, missing or extra samples, and explicit or default NoData values are refused before the artifact Store changes.
- The Terrain I/O codec uses Rust’s standard library. It is shared by the picker and download actions. GDAL’s AAIGrid reader through Rasterio and SciPy’s independent interpolation serve only as the fixture oracle.

[Esri ASCII raster format](https://resources.arcgis.com/en/help/main/10.1/009t/009t0000000z000000.htm) · [GDAL AAIGrid driver](https://gdal.org/en/stable/drivers/raster/aaigrid.html)

## Verification

The shared ASCII raster and language-neutral JSON fixture specify geographic sample extent, signed elevation checkpoints, and checksums for all 1,089 samples. The independent Rasterio/GDAL and SciPy oracle passed. A Rust probe compiled the actual ASCII codec and passed two tests covering the fixture, round-trip export, streaming resampling of a larger rectangular source, and singleton axes. The native editor package includes the picker, download, and history workflow in its `mounted` Cargo test target.

Run the external comparison with `repo tools exec uv -- run --no-project --with rasterio --with scipy <path-to-the-oracle-script>`.
