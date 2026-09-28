# Painted Path Picking

The browser reproduction selected the Red Frame when clicking visible orange geometry inside the frame’s empty bounding-box region. The retained pointer query used bounds as its final test, so a front concave path intercepted clicks outside its painted contour.

## Implementation

Rust and TypeScript now share an incremental contour hit-test contract and neutral fixtures in the geometry picking module. Nonzero winding tests filled contours, including implicit closure of open paths; explicit segments alone contribute to stroke proximity. Cubic and quadratic curves use adaptive subdivision. Elliptical arcs use a transformed ellipse and a curvature-based subdivision bound. A call consumes one source segment or one subdivision, with a maximum pending subdivision stack of 33. Invalid or exhausted geometry reports failure instead of silently returning an approximate hit.

The retained native pointer query now follows path bounds with the painted-path test. Work still yields through the existing bounded continuation and cancellation lifecycle. Node/control selection, marquee/lasso behavior, and selection bounds retain their own semantics. No path clone is made for a hit-test continuation.

## Verification

- TypeScript initial painted-hit suite: 91 tests passed, 5604 assertions, plus 44 independent Ajv cases.
- Native run 69843 compiled and began 371 tests. It stopped after 23 passed and one failed: exact diagonal selection at zero tolerance. The floating-point distance residual was nonzero. This is a test failure, not a compilation failure.
- The same diagonal case was added to the neutral fixture and reproduced in TypeScript: 91 passed, one failed.
- Both implementations now include coordinate-scaled machine rounding in stroke-distance comparison. Three.js Line3 independently confirms the exact diagonal point.
- Current TypeScript run: 94 tests passed, 5630 assertions, plus 44 independent Ajv cases. Fixtures include concave interiors/exteriors, hollow strokes, open fill closure, explicit stroke closure, cubic bulges, elliptical corners, contour holes, zero tolerance, reflection and shear. Three.js triangulation, curve sampling and closest-point calculations serve as independent oracles.
- The transformed curve selection oracle now triangulates sampled curve geometry instead of repeating the previous bounding-box behavior. An empty-region case was added to its shared native fixture.
- Additional curve-exhaustion tests demonstrate yielding after 16 steps and refusal before 100 steps when a requested precision cannot be reached within depth 32. The latest TypeScript run passes 95 tests / 5668 assertions.
- Native rerun 77357 remains live, collecting all failures. The renderer suite 45203 and component materialization 43904 also remain live; existing handles are retained.

## Remaining Acceptance Work

The browser fix is not yet verified against a rebuilt component. Precise picking is currently integrated for explicit path layers; primitive shapes, polygons, Boolean results, trace results, image alpha and text glyphs still need their corresponding geometry gates. Stroke picking currently uses centerline proximity and a conservative transformed width; exact caps, joins, dash gaps and nonuniform stroke envelopes remain incomplete. No full image-editor completion is claimed.

## Primitive And Polygon Integration

The same retained geometry cursor now handles rectangle, line, circle, ellipse and polygon layers. The schema supplies one primitive segment by index; polygons read exactly one vertex, without cloning or materializing their full contour. Existing shape-to-path conversion uses that same source. Rust and TypeScript expose matching indexed cursor inputs, including terminal idempotence. Current ellipse conversion still uses the existing four cubic segments; exact ellipse/arc conversion remains a fidelity item.

Thirteen new neutral primitive cases cover empty ellipse/circle corners, painted interiors, hollow rectangle centers, diagonal lines, concave polygon gaps and arms, including reflected/sheared ellipse and polygon variants. Three.js inverse matrices, vector distances, line closest points and triangulation independently validate expected hits.

- Primitive test red run 34016: failed because the indexed segment/cursor API did not exist.
- Primitive first green run 3529: 104 tests passed / 12,565 assertions.
- Affine primitive run 86758: 108 tests passed / 16,687 assertions.
- Post-fixture-repair run 18547: 108 tests passed / 16,687 assertions, plus 44 Ajv cases.
- Native run 77357 finished: 373 tests run, 325 passed, 48 failed. The full failure list contains no painted-picking or primitive-picking failure. The failures and fixes are recorded in [native fixture repairs](🧪️native-fixture-repairs.md). This is not a green native gate.
- Fresh full native run 31911 is live, log `🗑️generated/tests-native-fixtures-current.txt`.
- Renderer run 45203 ended with 12 compile errors in shared viewport/test code. Current sources already address those issues, including a newly supplied NonEmptyVec.push method; the window fixture retains its original push call. Fresh scoped renderer run 81252 is live, log `🗑️generated/tests-native-canvas-current.txt`.
- Component build 43904 remains live and is retained. Browser tab 4 at port 6064 is marked for handoff. No rebuilt browser behavior is claimed.

Primitive and polygon bounds-only picking is now replaced in source. Remaining selection work includes Boolean/trace results, image alpha/text glyph policy, stroke fidelity, large-scene indexing, direct node gestures and rebuilt browser acceptance.


Stable browser verification: on port 6065 (HMR disabled), explicitly selecting Direct Select and clicking screen (420,360) in the Demo's orange interior selected Orange Wedge, rather than Red Frame. The inspector displayed Orange Wedge and Shear=0; the canvas showed the wedge's bounds and transform handles. Browser error/warning log query was empty. Screenshot: `🗑️generated/painted-selection-verified.png`. This resolves the earlier concave-frame picking reproduction in the running app.
