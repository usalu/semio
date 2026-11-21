# GIS Terrain and Overlays

GIS Terrain streams real Terrarium elevation data into its editor and viewer. The Liège demo includes two pins, a route, and a region. An imported GIS map supplies its own geographic frame and replaces the demo features. Pins honor higher authored elevations. Routes and region rings, including holes, follow the triangles of the loaded DEM mesh. Regions display as boundaries.

The tile feed is the public Mapzen dataset. The viewport source link points to the provider and upstream credits. [AWS Open Data Registry](https://registry.opendata.aws/terrain-tiles/) · [Mapzen attribution](https://github.com/tilezen/joerd/blob/master/docs/attribution.md)

The browser and native wgpu renderers share the Rust projection. The camera uses the DEM at the project origin to raise a new automatic view above high terrain. A saved camera remains scoped to its geographic frame. React shows the source link, lists feature names for assistive technology, and displays the names on hover. Native wgpu shows the source as viewport text.

## Verification

- The shared Rust terrain suite passed 22 tests. The compiled browser WASM passed two checks, including 533 assertions against a shared fixture and independent Three.js ray intersections on the actual terrain mesh.
- Chrome mounted the real React World3d host and the newly built surface WASM with the Rust-produced Liège descriptor. It fetched 25 live Terrarium tiles without browser errors. The screenshot contained 755 pin pixels, 897 route pixels, 1,193 region-boundary pixels, and 300,027 terrain pixels matching the independent PNG color checks. The test started the camera and target 3 km below their authored heights and verified that the loaded DEM moved the viewport back above ground. The focused harness is `verify-terrain-browser.mjs`.
- A separate Chrome transport check passed cross-origin PNG delivery, retry after HTTP 503, off-screen cancellation, and progressive tile publication. The producer projection probe passed the imported overlay fixture, and the terrain window configuration contract passed in Bun.
- The native World3d host passed both focused integration tests. They exercised scene ingestion, Terrarium decoding, camera correction, all three feature classes, rejection of stale tile responses, overlay-only updates, and retry on returning to a failed tile. Native failures stay cached while the tile remains visible.
- The wider wgpu package build check stopped in the concurrent kernel-pool migration after the wake handler and kernel protocol fixes. [Native build details](native-terrain.md)
- Full Play activation did not reach the server. The shared graph spent over 21 minutes scanning the checkout before the request was cancelled. The normal Play acceptance suite now includes an editor-to-viewer GPU regression using a controlled, non-flat DEM.
- The registered GIS editor compiled and passed its runtime test for rendering the World3d window. The new overlay-specific test reached runtime and exposed a spine-only decoder in the test harness. It now uses the existing lane-aware decoder. The follow-up build stopped in the shared `semio-os-dsl` dependency before reaching GIS. The native producer probe and live Chrome run independently verified the overlay payload and its rendering.
- The renderer package type checker reported errors outside the changed terrain modules. One missing locale argument in the spawned-window title path was corrected. Scoped whitespace and path-length checks passed.

The repository Cargo tool refreshed the GIS artifact lockfile for changes already present in the dependency graph.

Implementation and reproduction details are in [overlay production](terrain-overlay-production.md), [elevation verification](terrain-elevation-verification.md), and [native terrain](native-terrain.md).
