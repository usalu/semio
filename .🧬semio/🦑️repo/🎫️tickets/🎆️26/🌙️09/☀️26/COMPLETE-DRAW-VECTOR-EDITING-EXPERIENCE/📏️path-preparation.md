# Resumable Path Preparation — 2026-10-02

## Implemented

Added a first-party path flattening module under `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten`. It consumes the existing shared move/line/quad/cubic/arc/close vocabulary rather than introduce another path model. Rust and TypeScript implement the same input schema and seventeen manually authored neutral fixtures. No runtime dependency was added.

Output contours retain local coordinates and an explicit closure flag. Tolerance is evaluated after the full supplied affine transform, so nonuniform scale, shear, rotation and reflection can refine geometry appropriately while subsequent stroke construction can retain authored local widths. Quadratic and cubic curves subdivide with de Casteljau and use the distance of transformed controls to the chord segment as a conservative flatness test. Using the segment rather than its infinite supporting line preserves collinear reversals and closed loops with distant controls.

Elliptical arcs use SVG endpoint-to-center conversion, including rotation, both flags, zero/equal-endpoint behavior and radii correction. They are evaluated directly on the ellipse rather than approximate each quarter turn with a cubic. For an angular band no larger than π, the chord deviation is bounded using the sum of transformed ellipse-axis lengths and `2 sin²(span/4)`. This form remains stable for small angles. Overlarge spans subdivide first. The tiny-radii fixture forces overflowing initial endpoint/radius ratios and validates a finite corrected-radius calculation.

Each grant processes at most its specified number of source segments or subdivision nodes. A node emits one endpoint or splits into two stack nodes; there is no whole-path expansion in a grant. Jobs report preparation/subdivision/completion, consumed source segments, emitted points and cumulative work. Geometry is bounded to 65536 source segments, 65536 output points, 4096 contours and depth 32; coordinates and transforms use finite billion-unit bounds, and tolerance is 0.000001–16 device units. Excess geometry is rejected explicitly rather than silently truncated. Native cumulative work uses u64 for 32-bit WASI safety.

Cancellation releases owned candidates and rejects partial results. TypeScript also exports asynchronous `preparePath` with progress, AbortSignal and yields between work grants; callers retain immutable source geometry until completion. Cancelling a job does not mutate previously returned contours or caller geometry. Rust owns its source. The TypeScript public 2D module reexports the new first-party API; native package glue exposes the `flatten` module.

The existing TypeScript test command now performs strict production type checking before Vitest. Vitest includes the new suite, and Nx inputs include its source/fixtures and the coverage oracle dependency. Native test inputs include the new JSON fixtures. Added `🧪️test◻️2d🟦️` and `🧪️test◻️2d🦀️` to launch.json beside raster/pixels tests in group 3_dev, orders 386.82 and 386.84. Both call existing Bun/Nx commands; no new executable script or command was created.

## Verification

Schema, fixtures and tests preceded implementations. TypeScript stub run 69677 terminated with exit 1: all nineteen initial new tests failed against incomplete preparation. Native stub run 12097 compiled and terminated with exit 1: all three new native test groups failed. Those failures were the intended behavioral red phase.

Both implementations reproduce the shared contours and closure flags under grants 1, 7 and 4096; every advance is checked against its grant. Fixtures include empty paths, an implicit origin, open/closed contours, line-after-close semantics, multiple contours and repeated endpoints, quadratic/cubic subdivision, transformed flatness with local output, collinear reversal, singular transforms, zero/equal-endpoint arcs, quarter/semicircles, radii correction and tiny radii. Invalid inputs, partial publication, cancellation and point/contour limits are exercised.

The independent Sharp/librsvg oracle checks one quadratic, one cubic and twelve rotated arcs (three rotations × both large-arc flags × both sweep flags), each under identity and a sheared affine transform: 28 curve/transform cases. Original authored SVG paths are rendered at 64× resolution and averaged per requested pixel; flattened contours are rendered by the previously independently verified first-party coverage kernel. All compared alpha values differ by at most two levels with flattening tolerance 0.0001 device units. Exact fixtures retain tolerance 1e-10 for floating point coordinates.

Final uncached `@semio-tech/s-2d-js:test`, handle 81354: terminal exit 0; **29 tests passed in three files**; strict production TypeScript checking passed. Final uncached `semio-framework-2d:test -- --lib --no-fail-fast`, handle 74744: terminal exit 0; **31 native tests passed, zero skipped**. Logs remain in `🗑️generated/tests-flatten-ts.txt` and `tests-flatten-native.txt`. `git diff --check` passed for the changed tracked 2D and launch files. The stable Draw preview was polled at the beginning of this continuation and confirmed live on handle 75879. No browser action or PNG download was verified this turn.

## Changed Files

- `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/📋️project.json`
- `.vscode/launch.json`
- Ticket research, this report and acceptance notes.

## Outstanding Acceptance

This is prerequisite progress toward faithful, responsive vector export. Draw's PNG serializer remains on its reduced SemioDrawing projection, and its old fixed-step flattening consumer is not yet replaced. Next work must construct complete stroke outlines (caps, joins, dashes), sample authored paint/images/text, preserve scene/group blending and connect the new jobs to Draw's export lifecycle. Actual browser export/download verification remains required. The wider editor acceptance list retains node lasso/join, algorithm workflows, typography, broader import support, operation cancellation and multiuser/reconnect behavior. Goal and ticket remain active; no completion or lifecycle operation was made. Repo ticket MCP remains unavailable in the current tool catalog.

Final launch verification parsed the complete launch.json as JSON, asserted both new names are unique and checked their exact Bun/Nx commands and group. Runtime output: `[DEBUG] Launch JSON and both unique 2D test registrations validated`. Tracked whitespace checks remained clean.
