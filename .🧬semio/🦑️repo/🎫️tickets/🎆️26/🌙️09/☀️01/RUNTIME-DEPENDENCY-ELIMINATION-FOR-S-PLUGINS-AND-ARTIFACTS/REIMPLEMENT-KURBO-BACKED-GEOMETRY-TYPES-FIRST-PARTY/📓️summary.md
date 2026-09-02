# Reimplemented Kurbo-Backed Geometry Types First-Party

`semio-framework-geometry` now exposes plain first-party 2D value types and algorithms while
retaining `kurbo` only as a dev-dependency differential oracle. Adaptive Circle, Arc, and
RoundedRect path generation matches `kurbo` element-for-element at caller-supplied tolerances;
affine arithmetic, cubic evaluation, tight path bounds, normalization edge cases, and path
transforms are first-party as well.

The public-API integration suite passes 5/5 tests, the Nx raster gate passes 3/3 tests, native
direct consumers compile, the geometry crate compiles for `wasm32-wasip2`, and reverse dependency
queries report no `kurbo@0.13.1` path for puzzle, flow, trinity, or animate plugins.

See `📓️research.md` for the repository-wide API/consumer inventory, algorithm audit, exact
commands, and the two unrelated pre-existing test/build blockers encountered during verification.
