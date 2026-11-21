# Terrain GeoJSON Exchange

**Import GeoJSON Overlays…** opens the Semio file picker for a local FeatureCollection. Its retained callback replaces the current overlay map through one `ChangeImportedFeatures` mutation. Undo and redo operate on the exact saved map states. **Export GeoJSON Overlays…** passes the current feature collection to the Semio host download as `terrain-overlays.geojson`.

The shared stdio GeoJSON profile checks geometry and WGS84 coordinates. The editable overlay map stores native positions, routes, and polygon rings with holes. Source FeatureCollection order, string and numeric feature IDs, absent IDs, and nested properties including explicit null survive export. Stable unique native IDs identify anonymous features and deconflict repeated source IDs. Polygon export closes rings and applies RFC 7946 winding. New native overlay features appear after imported features in Terrain collection order.

The file picker admits up to 24 KiB and the atomic import admits up to 12 KiB of serialized overlay data. Supported geometries are Point, LineString, and Polygon with two- or three-dimensional positions. Other geometries and invalid coordinates refuse the collection before publication. Export reconstructs the standard FeatureCollection fields from the current features. Collection-level bounding boxes and foreign members are outside that projection.

The retained export admits up to 256 KiB. It encodes one overlay per work step, moves each full 4 KiB page into the supplied segmented queue, and publishes the download after the final page seals. The host acknowledges the result and drains the pages. The terminal read retires the download handle. Cancelling retires the still-owned output before publication. Native contour attributes such as `elevationMetres` are ordinary GeoJSON properties.

[RFC 7946 GeoJSON specification](https://www.rfc-editor.org/info/rfc7946/)

## Verification

The shared GeoJSON fixture covers a polygon with a hole, a three-dimensional point with an exact integer ID beyond JavaScript’s safe integer range, and an anonymous three-dimensional line. It places geometry-like property names in the property bags to check that conversion retains them. Rust tests compare the round trip, malformed-collection refusal, duplicate source IDs, and the mounted picker, download, and artifact history.

The independent Python GeoJSON and GEOS check passed. It parses the same fixture, verifies the hole area, and reproduces the FeatureCollection with its property values. The oracle explicitly restores the null property bag before writing because the third-party GeoJSON constructor normalizes it to an empty object. Run it with `repo tools exec uv -- run --no-project --with geojson --with shapely <path-to-oracle>`.

The large fixture gives 48 named contour routes to exercise download size, attributes, page acknowledgments, terminal read, and cancellation. The pure exchange codec and its fixture tests live in the Terrain I/O layer. The editor package runs host workflow tests in the `mounted` target, which standard Cargo test includes. Run the focused host workflow tests with `repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path <Terrain-editor-Cargo.toml> --test mounted geojson_`.

The editor reached compilation after the test target split. The first build stopped in Store initialization replay because it could not resolve a StepContext method already present in the live shared source. Later attempts encountered concurrent dependency changes in the PDF workspace and plugin reactor lifecycle. Their live manifests now point to the required first-party crates. The full editor test suite is running against the current manifests, and its mounted workflow assertions remain unverified.
