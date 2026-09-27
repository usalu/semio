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
- Native rerun 77357 remains live, collecting all failures. The renderer suite 45203 and component materialization 43904 also remain live; existing handles are retained.

## Remaining Acceptance Work

The browser fix is not yet verified against a rebuilt component. Precise picking is currently integrated for explicit path layers; primitive shapes, polygons, Boolean results, trace results, image alpha and text glyphs still need their corresponding geometry gates. Stroke picking currently uses centerline proximity and a conservative transformed width; exact caps, joins, dash gaps and nonuniform stroke envelopes remain incomplete. No full image-editor completion is claimed.
