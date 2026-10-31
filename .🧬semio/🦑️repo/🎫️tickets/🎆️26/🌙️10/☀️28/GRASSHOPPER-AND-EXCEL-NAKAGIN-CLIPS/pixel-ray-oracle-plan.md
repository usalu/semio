# Independent Changed-Pixel Surface Oracle

This ticket-only diagnostic consumes the exact renderer export of actual world source polygons, their projected pixel polygons, source identity, material/opacity, retained flags and painter ranks. It does not import the renderer or reconstruct capsule geometry. Separate source-face boundaries are preserved; concave faces use the existing independent circulation ear-clipping helper, never an Item bounding polygon or naive fan.

The projection is independently recovered by NumPy least squares from corresponding world/pixel vertices. Its nullspace gives the parallel ray axis, oriented only by an explicit exported camera-towards vector. Per-pixel centre and a regular subpixel grid query actual triangles using vectorized ray intersections. Physical contributors include transparent layers until the first opaque surface, so hidden underpaint is distinguished from removal behind glass.

Portable cases precede implementation: hidden opaque removal, genuinely visible removal, concave opening, disconnected item polygons, depth crossing/painter disagreement and transparent foreground. The checker reports exact sample visibility and ties; it never waives differences using a colour tolerance. AA underpaint remains a candidate only when physical contributors remain and hidden removed sources lie below opaque facets, with independent no-AA and pixel-ID controls providing the separate raster proof. A finite grid is sampled evidence, not an exhaustive subpixel safety certificate.

GH owns the production renderer and final-source Cairo controls. Physical model/test source remains untouched. Native Excel replay remains ready but on HOLD.
