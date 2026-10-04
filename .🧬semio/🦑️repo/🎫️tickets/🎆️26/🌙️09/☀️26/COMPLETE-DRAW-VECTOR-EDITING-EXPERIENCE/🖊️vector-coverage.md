# Exact Vector Coverage — 2026-10-02

## Implemented

Added a first-party, domain-neutral polygon coverage kernel at `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage`, with Rust and TypeScript implementations of the same schema and eighteen manually authored neutral fixtures. Inputs contain dimensions, six affine coefficients, nonzero/evenodd fill rules and implicitly closed contour points. Output is a completed 8-bit coverage mask. No runtime dependency was added.

Preparation validates and transforms one point or closes one contour per work step. A resumable merge sort orders edge activation events. Each row is divided at edge endpoints and adjacent edge crossings; within each band, fixed edge order lets winding transitions identify filled trapezoids. The kernel integrates clamped linear left and right boundaries to compute exact pixel area rather than use a fixed supersampling grid. Active edges, sorting, crossing checks, winding transitions, pixel area writes and row publication advance under explicit positive integer work grants. Progress identifies preparation, sorting, rasterization and completion, with cumulative work. Rust uses a 64-bit cumulative counter so a long job cannot overflow a 32-bit WASI counter.

Jobs reject invalid dimensions, affine coefficients and points; geometry is bounded to 4096 contours, 65536 total points and coordinates within one billion. Image allocation uses the existing 16384-side / 16-million-pixel framework limits. Cancellation releases candidate buffers and prevents partial result access. TypeScript's `polygonCoverage` yields between grants, supports an AbortSignal and progress callback, and preserves caller-owned geometry. Rust owns its input. TypeScript callers must retain immutable geometry until completion.

Existing pixels test commands now cover the module and strict production TypeScript checking. Native package glue exposes `coverage`; no new executable command was created. Existing launch registrations `🧪️test🔲️pixels🟦️` and `🧪️test🔲️pixels🦀️` already run these commands, so launch.json was not changed.

## Verification

Schema and tests preceded implementation. The initial TypeScript stub run failed. The native stub compiled, then all three new native test groups failed as intended (`tests-coverage-native-red.txt`, terminal exit 1). Independent SVG checks initially differed by eight/nine alpha levels for small bow-tie and sheared polygons: low-resolution librsvg antialiasing does not provide exact area. The reference now renders at 128 times the requested resolution and independently averages alpha per requested pixel; tolerance remains two alpha levels. Expected exact fixture bytes were not weakened.

The distant diagonal fixture spans coordinates ±100 million. librsvg returned empty coverage at that scale despite correct kernel bytes. For a distant single nonzero contour, the test oracle now uses the already installed third-party polygon-clipping library to intersect transformed geometry with the canvas before rendering through Sharp/librsvg. This is test-only; kernel clipping remains first-party. The fixture's exact expected half-pixel area remains 128.

The eighteen fixtures cover integer/fractional rectangles, diagonal coverage, holes, overlap, reversed winding, self-intersections, crossings within a pixel, duplicate parity cancellation, shear, reflection, off-canvas geometry, degenerate contours, singular transforms, intersecting triangle unions/parity/opposite winding and the distant clipped diagonal. Both implementations reproduce exact expected bytes under grants 1, 7 and 4096. Every advance is checked against its work grant. An additional 96 deterministically seeded SVG comparisons cover multiple crossing contours, both fill rules and affine transforms. TypeScript checks asynchronous yielding, progress, cancellation before/during work, refusal of partial results and preservation of caller geometry and already published masks.

Latest uncached `@semio-tech/pixels:test-typescript`: terminal exit 0, **198 passed, zero failures**, 56865 assertions across six files. This target also passed strict production TypeScript checking. Final uncached native rerun after the 64-bit-counter refinement: terminal exit 0, **46 passed, zero skipped** (`tests-coverage-native.txt`, 5.9-second Nx run). `git diff --check` passed for tracked pixels changes. No browser action, Draw download or PNG production behavior is claimed from these module tests.

## Changed Files

- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🟦️.ts`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🦀️.rs`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🔲️pixels/🖊️coverage/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🔲️pixels/📜️script.ts`
- Ticket research, this report and acceptance notes.

## Remaining Work

This is prerequisite progress, not completion of the requested editor or PNG export. Draw's PNG serializer still projects through SemioDrawing and loses authored gradients, fill rules, affine shear and group effects. The kernel accepts polygon contours; resumable curve flattening and stroke outline generation are still required. Gradient/image/text sampling and scene/group compositing must be connected to a responsive Draw export job, followed by actual download/browser verification. The wider acceptance list remains open, including node lasso/join/algorithm workflows, typography, SVG support, large-operation cancellation and multiuser/reconnect behavior. The goal and ticket remain active. Repo ticket MCP is unavailable in this tool catalog; no ticket lifecycle operation was attempted.

## Coverage Budget Update

The subsequent stroke-region continuation raised the contour budget to 65536 in schema/Rust/TypeScript while keeping total points at 65536. This accommodates the generated positive primitive union; both implementations render a 9000-segment stroke, and pixels native/TypeScript suites remain green. See [stroke-region verification](🖊️stroke-regions.md) for the current limit and executed evidence; the earlier 4096 contour limit described above is superseded.
