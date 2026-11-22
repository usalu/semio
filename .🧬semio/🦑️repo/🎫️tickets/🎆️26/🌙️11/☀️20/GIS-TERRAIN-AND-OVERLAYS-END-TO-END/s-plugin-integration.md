# GIS Terrain in s

The GIS plugin registers the `s.gis.gisterrain@1/*` artifact with the normal s editor and viewer. The registered Liège example loads as a Semio document with a persisted WGS84 elevation grid plus a map containing positions, a route, and a region.

The native document format supports an elevation source with ordered west, south, east and north bounds and a rectangular grid of 2 to 65 samples per axis, stored as signed integer heights in millimetres (`rowsMillimetres`) with optional canonical missing samples. Rows run south to north and samples run west to east. The Rust and TypeScript schemas define the source, mutation, diff and SQLite projection. The editor's source-clear action goes through the artifact store. Map and elevation workflow inputs produce undoable semantic mutations with independent ownership.

The viewport receives the artifact's grid through the Semio World3d scene lane. The React/WASM and native hosts consume the same mesh and drape code. The mesh workflow output uses the `mesh.document` transfer and the LAS, PLY, STL, glTF and OBJ exporters use the corresponding Semio mesh snapshot. Feature geometry is clipped to the elevation grid. A terrain whose elevation source is missing displays a localized notice.

## Verification

- The shared Surface Rust unit suite passed all 23 tests, including the embedded-grid fixture through the first-party JSON boundary.
- The current browser WASM component built successfully and passed all three tests with 707 assertions. The suite checks embedded-grid and streamed-tile overlays against independent raycasts and exercises the embedded grid without tile uploads.
- The native World3d terrain suite passed all three tests. The embedded grid published a native mesh and draped feature geometry, issued zero tile requests, and retired the mesh when the source was cleared.
- The elevation and SQLite TypeScript suites passed 13 checks with workspace aliases directed to the source packages.
- The independent glTF test checks exported counts, winding, positions and relief from the language-neutral elevation fixture. Its repository Rust compile and the mounted Terrain workflow/history compile hit `semio-plugin-job-receiving` before executing GIS tests. The failing build could not see `unquoted_receiving_work`, which the live checkout exports. A fresh artifact-package run and the mounted GIS Map export run are still in progress.

## Development launch

The normal GIS terrain developer routes select the s plugin and its registered GIS Terrain editor.

```sh
repo run @semio-tech/framework-os-dev:dev-gis3d-react-prototype --lang=en
repo run @semio-tech/framework-os-dev:run-gis3d-native-prototype --lang=en
```

The launchers are waiting for the shared repository project's first sealed graph. The repository's current-graph query reported that this machine holds no sealed workspace generation, so it could not supply a cached graph. No current browser or native s session has been verified.

The first live checks use the registered Demo document. Open the terrain viewport and orbit to inspect the relief, route and region outline. Run **Set Exaggeration** from Actions, then use Undo and Redo. **Clear Elevation Source** should replace the viewport with the source notice, and Undo should restore the same grid and overlays. In Workflow, connect GIS Map's `map:out` to Terrain's `map:in` and run the connection. A second Terrain document can send its saved grid through `elevation:out` to `elevation:in`.
