# Semantic Path Editing Design

## Existing Boundaries

`DrawingLayerPatch` currently has visibility/lock/name/opacity/blend/transform/fill/stroke/Boolean/trace fields and a generic `layer_json` field. The mutation catalog has no path geometry mutation. The live store applies mutations through `DrawingMutationCandidateAuthority`, not merely the synchronous sparse-diff implementation. Adding an editor-only array edit would therefore bypass the actual persistence authority and history semantics.

The retained store already has reusable components: `DrawingRetirementOwner::Segments`, per-segment `DrawingPathSegmentDigestAuthority`, incremental path copying inside `DrawingLayerCloneAuthority`, and exact source/overlay rollback. These should support a typed `UpdatePathGeometry` mutation containing the layer ID and owned path segments. It must have schema, Rust and TypeScript twins, sparse diff/inverse, digest, admission, prepare/apply and retirement coverage. Extend the diff with a typed path geometry facet rather than exposing arbitrary JSON replacement to end users.

## Implementation Sequence

1. Specify open/closed path edit fixtures: move anchor (translate neighboring handles), move handle, insert with de Casteljau split, delete anchor, reverse, close/open, and rejected nonfinite/index inputs.
2. Implement bounded cubic splitting and affine inversion in the shared geometry module, with exact fixture outputs and Three.js independent curve/matrix samples.
3. Add `UpdatePathGeometry` in the drawing geometry domain and integrate every retained mutation phase. Clone segments one item per grant, budget their allocation, retire old/new arrays on completion/cancellation, and reject non-path targets.
4. Add one editor command with an explicit operation enum and typed node/handle parameters; calculate new geometry then publish one semantic mutation/undo step. Split large operations into a resumable command work owner.
5. Project node/handle overlays from ephemeral selection and gesture state. Drag previews stay ephemeral; pointer release commits exactly once; Escape and cancelled release discard previews. Convert pointer deltas by the full inverse ancestor transform.
6. Reuse the same command from accessible numeric controls and keyboard actions so pointer editing and inspector editing share validation/history.

## Adjacent Corrections Still Required

- Selection arrangement is currently synchronous inside the old bounded catalog path. Its admission only counts root layers/assets, which undercounts nested geometry and must be replaced with a retained traversal/operation planner.
- Native retained transform application currently requires strictly positive scale factors although the schema accepts any numbers. Decide and implement singular-transform policy consistently; flipping requires negative nonzero scales or a complete affine representation.
- Existing direct-selection gesture only picks on release; lasso currently shares rectangle selection. These are user-visible capability gaps, not optional polish.
- Exact ungrouping/reparenting across arbitrary rotated nonuniform scales may introduce skew. The current five-number transform cannot express every composed affine transform. A schema-first affine representation is needed before claiming lossless transforms across nested groups.

## Current Implementation

`UpdatePathGeometry` now exists under the transform subset, with JSON schema, metadata, Rust semantic diff/inverse and TypeScript/Immer fixture validation. The sparse diff has a typed `path_segments` facet. The retained store includes segment-wise digest/copy, bounded admission, cancellation retirement and atomic replacement. Native roundtrip/retirement/cancellation/digest tests have been added but not yet executed. Exact cubic splitting and inverse affine helpers have passing independent TypeScript geometry checks.

Next: complete native compile/test validation, implement path anchor/handle operations using this mutation, expose localized numeric node controls and pointer gestures, and replace the selection command's synchronous planning with a resumable traversal. Full editor workflows remain unverified.

## Typed Geometry and Node Commands

The geometry mutation now uses the correct DSL statements block for path segment enums. The initial native check and preview activation both exposed the missing statements annotation; it has been corrected and the check and activation are running again.

PathSegment now has one canonical JSON Schema definition, a strict owned TypeScript parser, and GraphQL/protobuf twins. The typed pathSegments diff facet is mirrored across every existing diff surface. The mutation schema references the canonical segment schema.

Added schema-first PathEdit operations and shared language-neutral fixtures for coordinate edits, tangent-preserving anchor movement, cubic/quad/line subdivision, anchor deletion, contour opening/closing, and path reversal. Rust and TypeScript implementations preserve the source on rejection. Arc reversal preserves radii and toggles sweep; arc subdivision remains explicitly unavailable.

Published editPath through the existing command bridge, retained command catalog, publication contracts and action registry. The bridge accepts structured edits or action-pane JSON and applies numeric input events to the typed coordinate operation. The selected-path inspector exposes localized anchor/control coordinates and node actions through windowed tree sections. Locked paths disable coordinates and omit node actions. These UI and native behaviors are not yet runtime-verified.

Validation: Draw TypeScript Nx test passed after node core and command registration: 10 tests, 150 assertions, 26 independent field-patch cases, 27 publication routes. Node fixture subdivision is independently sampled with Three.js; mutation/control edits are compared against Immer. The new native roundtrip, cancellation, inspector and bridge tests remain pending. The preimplementation test run failed because the node-edit implementation module did not yet exist.

Found and corrected two older test integration errors during this pass: an accidentally duplicated argument in the PDF export settled call and a stale bounded route count.

## Follow-Up Validation

The native command check additionally required `DslScalar` on the point/axis unit enums and `Box<PathEdit>` for the DSL's required tagged statement field. These declarations are corrected; the fresh `check-path-command.txt` run is pending. The prior check did not pass.

A new failing regression test confirmed that parseDrawingLayerNode stripped every layer field except kind. The parser now preserves the additional fields allowed by the artifact schema, and the path mutation twin uses the canonical strict segment parser. The regression passes in the `ts-layer-parser-fixed.txt` Nx run alongside the node fixtures, including added quadratic subdivision and multi-contour arc reversal.

Outstanding immediate validation: finish native library check, run all new path/inspector/bridge/retained-cancellation tests, and activate the actual Draw preview. Runtime canvas controls have not been verified. Arc subdivision, direct canvas node handles, direct object transforms, keyboard editing, richer appearance controls, and large-document resumable planning are still outstanding.

## Arc Geometry and Selection

Implemented exact arc subdivision in Rust and TypeScript, preserving corrected ellipse radii, rotation, sweep, endpoints and the appropriate large-arc flag on each half. Zero-radius/coincident arcs become degenerate line subdivisions. The preimplementation fixture failed with the explicit unavailable-arc error; the implementation passes an independent Three.js EllipseCurve sampling oracle, including rotation, reverse sweep and radius correction. The inspector now exposes Split Segment for arcs.

Consolidated SVG endpoint-to-center ellipse conversion and reused it for rendering conversion and exact segment extrema. Segment bounds now operate directly in the full affine transform and include exact arc extrema; document bounds avoid converting arcs to cubic approximations merely to measure them. Added shared arc-bound fixtures and Rust/TypeScript twins.

Repaired the retained canvas query: compose ancestor transforms; skip hidden/locked parent subtrees; compute curve extrema one segment per work item; retain the frontmost candidate when specificity ties; recognize control handles outside the filled curve bounds. Empty group placeholder bounds no longer act as selectable phantom rectangles. Added language-neutral nested/transformed/hidden/locked/control query cases and an independent Three.js oracle, plus native frontmost selection regression.

Native library checks `check-path-command.txt` and `check-selection-geometry.txt` both passed. The latter completed in 3m43s. The latest small control-hit regression adjustment and its fixture were made around that completion and must remain in the subsequent native test/check scope. Native tests still have not returned; do not claim they pass. The TypeScript arc-extrema run passed 13 tests and 264 assertions, plus 26 field-patch cases and 27 command publication routes. A subsequent control-fixture run is pending.

The subsequent `ts-selection-controls.txt` run passed: 13 tests, 265 assertions, including the transformed control handle outside curve bounds. Native execution of that regression is still pending.

## Real Lasso and Current Validation

Lasso now retains the drawn polygon, consumes each coalesced pointer sample through the retained command owner, and draws its actual contour. At the fixed 256-point capacity it downsamples in place and adjusts spacing, preserving the initial point and the final release endpoint. Selection traverses the document incrementally and requires each candidate bound to be contained by the polygon. Hidden/locked ancestors remain excluded, and the existing framework owns additive/subtractive/invertive selection. Escape abandons the gesture.

The new Rust/TypeScript polygon containment twins handle boundary points, concave shapes, and degenerate line bounds. A failing concave-notch fixture exposed that checking only a degenerate rectangle's midpoint accepted a line crossing outside the lasso. The fix splits that line at polygon intersections and checks each interval. Shared fixtures and an independent Three.js triangulation oracle now pass. Native tests cover coalesced sample yielding, actual polygon selection, cancellation, and 10,000 samples under fixed capacity.

Actual completed runs: `ts-lasso-fixed.txt` passed 14 tests / 285 assertions plus 26 field-patch oracle cases and 27 publication routes. `check-lasso.txt` passed the native library check in 4m51s. The earlier filtered native run (`tests-current.txt`) passed 15 tests, with 292 skipped; it does not establish the subsequently added lasso gesture cases. A fresh full native library suite (`tests-lasso-all.txt`) is running.

The earlier Draw activation succeeded, but predates lasso. Browser reload still shows an empty `root` element and readyState `interactive`. The default hub process is alive but port8787 is not yet listening: its log shows catalog publication/build work. No browser workflow is verified.

## Mutation Catalog Completeness

The audit found that UpdatePathGeometry had been added to the mutation enum but omitted from its exhaustive KINDS list and subset mutation catalog. Both the aggregate catalog and transform catalog now include it, with a canonical reshape-curve scenario containing before/after snapshots, the mutation, exact sparse diff and outcome. Ten existing sparse-diff fixture files now include the new nullable pathSegments facet, matching serialization. The declared enum count is updated to15.

Added native assertions for the canonical diff/apply/inverse and a TypeScript/Immer comparison over the same snapshots. The first TypeScript run caught a missing required assets map in the new fixture; both snapshots were corrected. Its follow-up run `ts-geometry-catalog-fixed.txt` passed: 15 tests, 287 assertions, 26 independent field-patch cases and 27 publication routes. The full native run remains live in `tests-lasso-all.txt`; do not launch another native test process while it is queued/building.
# Segment Conversion Checkpoint

The schema-first PathEdit union now includes convert with a line/cubic target. Rust and TypeScript implementations promote straight segments and quadratics to cubics exactly. SVG arcs use the existing renderer's quarter-arc cubic approximation in Rust and an equivalent TypeScript implementation; the final endpoint is pinned exactly to the source. Degenerate arcs become straight cubics. Straightening replaces one drawable segment with its endpoint line and preserves contour boundaries. Move/close markers are rejected as conversion targets.

The windowed node inspector exposes Convert to Curve / In Kurve umwandeln and Straighten Segment / Segment begradigen where applicable. Locked paths expose no conversion actions. The same EditPath command publishes UpdatePathGeometry; no new command route was introduced. Native tests cover localized controls, locks, and both text/binary command codecs but have not executed yet.

Five language-neutral cases were added before implementation. The initial TypeScript run failed with Unknown path operation (`ts-convert-red.txt`), then passed after implementation. The extended run (`ts-convert-arcs.txt`) passes20 tests/696 assertions,42 field-patch cases and28 publication routes. Independent Three.js LineCurve, QuadraticBezierCurve and CubicBezierCurve checks establish exact promotion; Immer independently validates straightening. Rotated multi-quadrant ellipse cases check both sweep directions, endpoint equality, tangent direction and normalized radial error below0.0003. This is approximation evidence, not a claim of exact ellipse representation by cubics.

Native check session89881 (`check-convert.txt`) failed because the renderer's private arc helper was addressed through the public schema facade. Conversion now calls the existing helper through its actual ancestor module. Fresh check session36213 (`check-convert-fixed.txt`) and full native test session48654 (`tests-convert-all.txt`) are live. The test run was launched before that helper-path repair; inspect source-capture timing if it reports the same error. The previous full suite session46142 ended after53m20s with nine NoConfig value-construction errors in newly added lasso/drag tests; all nine now use NoConfig::default(). No native runtime pass is claimed. Joining contours, whole-shape conversion and direct canvas node/handle manipulation remain incomplete.
# Contour Joining Checkpoint

PathEdit now includes Join { index, other } in the shared schema and both implementations. The operation accepts two distinct open-contour endpoints, reverses either contour as needed (including Bézier handles and arc sweep), adds a straight connector only when endpoints differ, and preserves unrelated contours. Joining the two endpoints of the same open contour closes it. Closed contours, interior anchors and missing endpoints are rejected without modifying the input. This joins subpaths within one path layer; merging different path layers remains a separate missing workflow.

The bilingual windowed inspector now offers Join Next Contour on eligible endpoints. Availability is calculated in one reverse pass over the segment list, and locked paths expose no join action. Shared fixtures cover separated/reversed/coincident contours, same-contour closure and invalid targets. The initial test failed with Unknown path operation (`ts-join-red.txt`); `ts-join-oracle.txt` now passes21 tests/736 assertions,42 field cases and28 publication routes. Immer independently validates splicing, while Three.js checks preservation of the reversed cubic.

Native library check session11330 (`check-join.txt`) passed in8.1s. The preceding corrected conversion check session36213 also passed in2m5s. Native tests were added for the Join command's text/binary codecs and a full editor undo/redo roundtrip for joining and conversion. Those tests are not yet executed; session48654 remains live and may have captured earlier sources. No browser editing or history pass is claimed. `git diff --check` passed for Draw.
# Whole-Shape Conversion Checkpoint

The shared EditSelection schema now admits toPath. The native planner converts rectangle, line, polygon, ellipse and circle layers into DrawingPathBody using the same local geometry as rendering. Each replacement preserves the complete base (identity, name, style, transform, visibility and lock metadata), parent and stack index. The delete/create mutation pair is published as one editor edit. Selection may span different parents; already-converted paths are skipped, and an entirely unchanged selection emits no history entry. Locked ancestors or unsupported selected kinds reject the entire plan.

The inspector exposes Convert to Path / In Pfad umwandeln only for editable shape/path selections containing at least one shape. Five language-neutral primitive fixtures are shared with the TypeScript shapePath implementation. The red run (`ts-shape-red.txt`) failed on the missing implementation; `ts-shape.txt` passes22 tests/930 assertions,42 field cases and28 publication routes. Three.js cubic evaluation independently checks ellipse/circle outlines; the native tests verify complete base preservation, stack order, nested parents, invalid selections and idempotence. Ellipses use the existing renderer's four-cubic approximation, not an exact rational ellipse claim.

Native library check session61560 (`check-shape.txt`) passed in7s. A full editor undo/redo test now covers primitive-to-path conversion as one edit; native inspector and conversion fixture tests are authored but not yet verified. The full native suite is now session31528 (`tests-shape-all.txt`) with --no-fail-fast to collect all remaining failures.

The preceding suite session48654 reached323 native tests after14m19s, then stopped after11 passes and one lasso fixture failure. Its pointer samples incorrectly relied on the default camera (512,512 at0.75 zoom) while asserting origin-based world geometry. The fixture now explicitly uses an origin camera,100×100 viewport and canvas samples mapping to the intended triangle, with an exact world-point assertion. No full-suite pass or browser editing success is claimed.

## Native History Verification

The full 327-test native run `tests-regressions-admission.txt` executed the real registered retained-operation editor fixtures. Path joining and segment conversion each restored the exact original geometry through one undo and redo. Whole-shape conversion restored the original primitive snapshot and reapplied the converted path snapshot. All three workflows passed. The whole-shape language-neutral fixture, stroke fixtures, typed path diff scenario, and paged localized node inspector also passed. Overall this run had 326 passes and one remaining direct-drag assertion failure; it was not a fully passing suite.

Fixture setup now loads a named document through the actual retained envelope ingress/acknowledgement API and compares the admitted snapshot exactly. `SetSnapshot` emits a host effect and does not itself execute that effect in a native app fixture. Direct drag assertions distinguish artifact publications from window-transient publications instead of assuming every revision is a document revision.
