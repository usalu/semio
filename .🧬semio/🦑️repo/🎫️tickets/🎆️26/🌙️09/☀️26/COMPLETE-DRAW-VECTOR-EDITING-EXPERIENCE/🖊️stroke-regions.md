# Resumable Stroke Regions — 2026-10-02

## Implemented

Added first-party Rust and TypeScript stroke-region jobs at `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke`, with schema-first inputs and thirty-two manually authored neutral mask fixtures. Inputs contain local flattened contours and closure, full affine transform, device tolerance and a color-independent stroke style: width, butt/round/square cap, miter/round/bevel join, miter limit, dash array and dash offset. Output is a union of local polygons intended for nonzero coverage. No runtime dependency was added.

Each centerline segment creates a rectangle. Outer corners contribute a bevel triangle, a limited miter region or an adaptive circular sector. Endpoints contribute appropriate cap regions. Every region has positive local winding, so crossings, overlapping and reversing centerlines retain their painted union. Short round joins use sectors rather than whole circles, preserving the boundaries of short adjacent segments. Round sectors subdivide under a transformed chord-error bound; one polygon or round-subdivision node is processed per work step. Tangent-oriented square dots retain their direction even when a dash has zero length.

Consecutive duplicate source points are normalized incrementally, retaining zero-length drawing/closing subpaths while omitting a moveto without a drawing command. Dashes restart at every contour, repeat odd patterns, support positive/negative offsets, preserve zero-length dots and connect adjacent painted intervals across zero-length gaps. A pattern with no positive gap is solid. Painted runs reconnect across closed seams with a join and without internal endpoint caps. A wholly painted closed contour remains closed.

Preparation, offset traversal, dash pieces, primitive generation and round subdivision are work-granted and report phase, source/run progress, emitted points and cumulative work. Source input is bounded to 4096 contours and 65536 points. Dash and outline expansion each have an explicit 65536-point budget, with finite coordinates, bounded styles and a subdivision depth limit. Exceeding these limits returns an error rather than silently truncate geometry. Candidate buffers are released on cancellation and partial results are unavailable. Native work counters use u64 for WASI safety. TypeScript's asynchronous `prepareStroke` yields between grants, exposes AbortSignal/progress and preserves caller-owned and already published geometry.

The TypeScript public 2D API reexports the job, asynchronous helper and first-party types; native glue exposes `stroke`. Existing 2D scripts perform strict checking and run the new suite through Bun/Nx; the already registered 2D launch commands cover it. Native tests use the first-party pixels coverage package as a dev dependency, and the existing workspace Cargo-lock maintenance task passed. Root Cargo.lock now records that dev dependency; inspected plugin/hub lock entries correctly retain only their runtime dependencies.

Coverage's contour limit was raised from 4096 to 65536 in its schema and both implementations, retaining its total 65536-point budget. Primitive unions can contain many inexpensive polygons even when their total geometry is bounded. The 9000-segment regression produces 9000 stroke polygons within the point budget and is rendered by both implementations. The old contour cutoff would reject that valid bounded geometry.

## Verification

Schema and fixtures preceded implementation. TypeScript stub run 51726 failed against incomplete stroke preparation. Native stub run 57889 compiled and failed both initial new native test groups. These were behavioral red failures.

All thirty-two shared fixtures reproduce exact alpha bytes under grants 1, 7 and 4096 in both implementations. Every advance is checked against its grant. Fixtures cover cap styles, vertical/reversed strokes, joins and miter clipping, open/closed zero-length subpaths, moveto-only behavior, dash parity/offsets/all-zero patterns/dots/zero gaps, a diagonal square dot, closed rings and reconnection across a dashed closed seam. Expansion-budget failure, sticky candidate refusal and cancellation are also executed.

The independent Sharp/librsvg oracle renders at 128× resolution and averages each requested pixel. It checks the fixtures plus **432** combinations of three caps, three joins, eight centerline/degeneracy cases and six dash patterns, with reflection or shear. Comparisons differ by at most two alpha levels using outline tolerance 0.0001. These checks caught and corrected internal caps across zero gaps, a near-collinear round join wrapping to a full circle, lost square-dot tangents and omitted closed degenerate caps. The separate 9000-segment test matches an independent SVG render and the native coverage result. Async tests execute yielding, progress, abort before/during work, and ownership of published candidates.

A reference exception required checking the actual standard: [SVG 1.1 stroke rules](https://www.w3.org/TR/SVG11/painting.html#StrokeProperty) exclude an isolated moveto, while zero-length drawing and closing subpaths can receive caps. [SVG 2 cap rules](https://www.w3.org/TR/SVG2/painting.html#StrokeLinecapProperty) confirm closed degenerates and tangent-oriented squares. Sharp/librsvg incorrectly paints an isolated moveto with square cap. The test oracle removes only that nonpainting subpath before SVG rendering, and an independent native-canvas test confirms empty moveto-only square coverage. Closed degenerate subpaths remain in the SVG oracle. Runtime code follows the standard; no compatibility layer was added.

Final uncached runs, all handles polled terminal:

- `@semio-tech/s-2d-js:test` (46839): exit 0, **67 tests passed** across four files; strict production TypeScript checking passed.
- `semio-framework-2d:test -- --lib --no-fail-fast` (3930): exit 0, **34 native tests passed, zero skipped**.
- `@semio-tech/pixels:test-typescript` (40974): exit 0, **198 passed, zero failures**, 56865 assertions; strict production typecheck passed after the coverage budget change.
- `@semio-tech/pixels:test-rust -- --lib --no-fail-fast` (44360): exit 0, **46 passed, zero skipped** after the coverage budget change.
- `workspace:deps-cargo-lock` (13608): exit 0.

Logs are retained under the ticket's `🗑️generated` folder. Tracked `git diff --check` for 2D, pixels and root Cargo.lock passed. Stable preview 75879 was polled and live at the start of the turn. No browser action, rebuilt component or Draw PNG download is claimed from these module checks.

## Changed Files

- `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🟦️.ts`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🦀️.rs`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🧬️schema/🔣️.json`
- Root `Cargo.lock` and ticket research/report/acceptance notes.

## Remaining Acceptance

This is prerequisite progress toward faithful PNG output. Draw's production serializer still uses the lossy SemioDrawing projection, and the old fixed-step curve/stroke consumer is unchanged. The next required integration must combine prepared authored curves and strokes with paint, image and text sampling, scene/group compositing and a responsive export lifecycle. Actual component/browser export and download comparisons remain required. The wider editor acceptance list still includes node lasso/join, algorithm workflows, typography, import coverage, large-operation cancellation and multiuser/reconnect behavior. Goal and ticket remain active; neither completion nor a ticket lifecycle operation was attempted. Repo ticket MCP remains unavailable in this tool catalog.
