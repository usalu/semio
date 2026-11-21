# Draw Scene View and Boolean Native Verification

The earlier Scene companion finished 48 native test bodies with 22 passing and 26 failing. This review covers the selected rectangle overlay and the two Boolean failures. The shared retained-allocation marker errors in two additional view tests are covered by the Scene preparation work.

- The rectangle's shared vector bounds matched the neutral selection fixture. The test then decoded only the Canvas2d surface spine. Canvas2d pages its layer records into a child carrier and places an empty JSON array in the spine. The Scene test and editor canvas law helpers now reconstruct the typed canvas with its actual payload lanes using the existing artifact fixture projection. The Scene test checks cached selection bounds against the same neutral fixture before counting mounted overlay records.
- The Boolean raster test called `DocumentRasterJob::result`, which moves the raster image out. Retirement received an empty output slot and the test unwrapped it. The test now takes the final image at retirement, validates its dimensions in bytes, and closes the image under its funded owner. It retains its progress and incomplete-result assertions.
- Boolean publication consists of physical custody work that does not advance the semantic progress counter. Advancing two semantic steps during publication ran through private custody and reached completed publication. The retirement fixture now uses the reported physical work for its offsets and continues to assert that partial retirement exposes no completed plan.

The existing Three.js selection bounds and Sharp Boolean raster tests continue to consume the same neutral fixtures. The focused native command ran from the Draw drawing Cargo workspace and exited 101 before running any test. Its earlier-admitted compiler input failed in `semio-plugin-reactor-lifecycle` with two unresolved `semio_plugin_decl_media` references. The refreshed full companion run also stopped before executing tests. Its source gate refused the framework crate `semio-plugin-instance-app` at 263,166 source bytes against a 262,144-byte limit. The framework owner corrected the size and a new full companion run is in progress.

```sh
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-draw-drawing-scene-tests --lib -- view::tests booleans::tests
```
