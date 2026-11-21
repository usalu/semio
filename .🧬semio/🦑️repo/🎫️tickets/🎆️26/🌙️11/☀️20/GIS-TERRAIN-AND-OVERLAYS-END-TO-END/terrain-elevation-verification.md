# Terrain elevation

## Provider

The GIS terrain descriptor points at public Mapzen Terrarium PNG tiles on AWS Open Data. The XYZ endpoint is `https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png`. It needs no key, proxy or deployment change. The browser and the native wgpu asset stream can both request the HTTPS URL. [AWS Open Data Registry](https://registry.opendata.aws/terrain-tiles/) · [Terrarium encoding](https://github.com/tilezen/joerd/blob/master/docs/formats.md) · [Data attribution](https://github.com/tilezen/joerd/blob/master/docs/attribution.md)

Requests carrying `Origin: http://localhost:5173` returned HTTP 200, `image/png`, a 256 × 256 image, and `Access-Control-Allow-Origin: *` for the Hannover center tiles at zooms 6, 11, 14 and 15. `pngjs` independently decoded the first, center and last samples of tile `11/1079/672` as 63.4375 m, 52.734375 m and 56.28125 m.

## Tile selection and loading

The shared terrain engine clamps the distance-derived level to the descriptor’s zoom limits and supplies center-first XYZ requests to both renderers. React keeps up to eight requests active, displays completed meshes as they arrive, and cancels offscreen requests as well as all pending work when the source changes or the layer unmounts. A timeout, a network error or a transient HTTP failure retries with capped exponential backoff while the tile remains visible. Terminal misses stay suppressed until the tile leaves the camera footprint.

## Verification

- `repo cargo --store=shared --configuration=prototype -- test -p semio-framework-surface-terrain --lib --locked` passed 22 tests. The language-neutral Terrarium fixture exercises the configured zoom limit, center-first XYZ coverage and an exact 247 m exaggerated mesh height.
- The third-party `pngjs` decoder produced the same fixture height. It also decoded live provider data from `11/1079/672` as reported above.
- The saved `verify-terrain-transport.mjs` Chrome probe exercised the production React transport class with a session stand-in. It observed HTTP 503 recovery, cancellation during a camera move, publication for visible tiles, session disposal and a live browser cross-origin GET. The repository capacity queue delayed the `repo tools exec node` attempt, so the same script was also run with the installed Node executable.
- The existing Storybook suite has a browser test for the 503 retry with its real WASM-backed scene. The repository target was cancelled after it spent five minutes waiting on a shared project graph scan that had exceeded the reported ten-minute estimate.
