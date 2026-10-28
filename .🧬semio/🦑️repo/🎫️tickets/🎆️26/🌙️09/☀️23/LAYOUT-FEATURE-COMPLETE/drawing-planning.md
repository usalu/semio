# Bounded Layout Drawing Primitive Planning

The new Layout-owned `scene::drawing_planning` adapter measures retained Drawing geometry in bounded turns, fits it to a page or rotated placed frame, then streams a second pass of source-qualified primitives. It consumes the real child-owned lower port, retains only numeric/matrix ancestry and fixed scalar state, and never materializes a complete VectorScene, display list, label string, image payload or path point vector.

## Consumer API

Native `DrawingPlanCursor::new(LayoutBounds target)` and TypeScript `new DrawingPlanCursor(DrawingTarget)` retain the caller-authored placement. `advance(snapshot, VectorSceneGrant)` returns Progress, MeasureText, Fitted, Primitive or Done. The same original work/copy/capacity/release currencies fund source ancestry and matrix ancestry; each turn exposes its performed receipt. Matrix growth uses a separate new block, copies numeric ancestry and releases the old block only when both capacities and release are funded. Source identifiers are compared incrementally without copying arbitrary names; source byte/code-unit reads are bounded per turn.

The first pass analytically measures line, quadratic, cubic, elliptical arc and closed-path extents after full spatial quaternion/TRS composition and planar projection. Image rectangles include all transformed corners. Hidden layers do not contribute drawn bounds or emitted primitives. Canvas extent is used only when no drawn extent exists. Uniform fitting and rotation about the target centre apply identically to backgrounds and placed-frame targets. The geometric path envelope follows the existing centreline/path fitting policy; it does not enlarge paths by stroke thickness.

MeasureText carries `DrawingTextPlan { source, index, content, at, affine, paint, font }`. The stable index counts all child text leaves, including hidden earlier leaves. Text remains borrowed from the retained canonical child. The planner waits for `include_text_extent(index, Option<DrawingBounds>)`, where bounds are real shaped glyph ink coordinates relative to that text's source origin; None denotes no ink. It adds the source origin, applies the original group transform and includes that extent. It never estimates text width by character count. `text(snapshot,index,grant)` reborrows the same active supplier while the shared paragraph owner advances.

Core's shared paragraph owner now supplies `begin_external_label`, `advance_external`, `external_cluster` and borrowed glyph/segment access with stable pointer/index guards. Root PDF integration can measure at origin [0,0], accumulate actual outline extents, acknowledge the measurement, then use Primitive::Text plus the fitted affine for emission. That source was not edited here. The adapter preserves the original Layout label font policy: family/weight/tracking from the first paragraph or Layout Sans fallback, 11-unit size and 13.2 leading; Drawing's neutral styles supply paint only.

Primitive events are BeginPath, one original VectorSegment, EndPath, borrowed Text and borrowed Image. They carry stable layer/leaf source ordinals and the final affine. Paint retains binary32 RGBA/opacity and unbounded finite authored stroke width. Images retain original MIME/octet backing rather than base64 copies. Text, image, path and segment counts have no former 32/8/128 caps. Formatting consumers remain responsible for preserving the command semantics, shaping, pixel decoding and encoding under their own budgets.

Completion, refusal and cancellation quote `next_release_bytes()`, then `close_one(maximum_bytes)` retires one original numeric or matrix block under explicit authority. Underfunding leaves that backing intact. `terminal_is_empty()` proves the planner owns no remaining dynamic backing. The original document stays with its parent owner. TypeScript receipts cover explicit typed-array custody; JavaScript GC timing is not asserted.

## Files

All new planner files live under `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing`: neutral protocol/fixtures, Rust/TypeScript planner, paired analytic geometry modules, native/TypeScript laws and the test-owned independent oracle. The existing scene Rust root mounts the planner. The neutral lower cursor also adds current_style/currentStyle reborrowing and the Layout model adds background_drawing_leaf/background_drawing_style wrappers for bounded lookup.

Retained ticket verification input is `drawing-planner-check.mjs`. Emitted verification modules are under the ticket's generated directory and are removed after each run. No runtime external dependency or build metadata was introduced.

## Executed Evidence

- Actual TypeScript production planner, canonical Drawing boundary and real composition: strict diagnostics 0 across 232 actual sources.
- Ten shared scenes, 60 cancellation/completion cuts and 49 expected primitive events pass. Separate cases isolate transformed quadratic/cubic extrema, a large rotated elliptical arc, zero-radius arc, placed-frame rotation, spatial quaternion projection, canvas fallback, full emptiness and hidden earlier text indices.
- Wide retained input publishes 40 texts, 12 images, 141 paths and 4377 segment commands. A 4097-unit style identifier is resolved incrementally without copying, and stroke width 512 remains 512.
- Independent test-owned interfaces use Ajv for canonical source schema, Skia Path2D tight bounds, Three matrix composition and the actual packaged MapLabelSans font through Canvas metrics. Bounds/affines agree within 1e-4 to account for the independent path backend's binary32 geometry; raster paint agrees within one octet of alpha quantization. Neutral expected measurements were captured from those independent sources, not from the production planner.
- Immutable source equality, original image custody, active text reborrowing, explicit measurement acknowledgement, underfunded work/backing refusal, successful retry, quoted retirement and terminal emptiness pass.
- Four planner Rust files parse without grammar errors; maximum path 152 characters. The seven lower-port Rust files also parse. New native laws consume the same ten cases/cuts and wide parameters, but remain unrun.

## Remaining Verification And Integration

No native Rust compilation, native runtime test or allocation observer execution occurred. Grammar parsing is not type checking. Root owns the PDF integration and interchange owns STEP integration; this report establishes the adapter and executable TypeScript laws, not completion of those format consumers or removal of every synchronous whole-scene consumer.

## Active Image Borrowing

DrawingImagePlan and cursor.image(snapshot,grant) now lend the last emitted image's original mime/byte owner and exact source-qualified affine across bounded encoding turns. Any next advance invalidates this active-image supplier; no raster or arbitrary string copy is introduced. The same neutral mixed-scene laws assert byte backing identity through the new supplier; native pointer laws are authored. Actual rerun after the concurrent segment-root export:368-source strict0 with installed node+bun ambient declarations,10cases/60cuts/49small-events and wide40texts/12images/141paths/4377segments/4097-characterstyle;4Rustgrammarfiles clean. Native tests remain unrun.
