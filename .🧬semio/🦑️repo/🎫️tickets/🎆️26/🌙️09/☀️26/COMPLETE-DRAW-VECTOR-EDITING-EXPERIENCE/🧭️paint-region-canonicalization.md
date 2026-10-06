# Painted Region Canonicalization Follow-up

The current internal-lasso repair supplements cached boundary enclosure. Raw fill contours can still contain coincident contours, opposite winding cancellations, or retraced source segments outside actual painted area. Raw bounds likewise include those unpainted portions. These cases need schema-first canonical paint preparation and revised neutral fixtures before complete selection accuracy can be accepted.

## Existing Owned Kernel

The framework planar BooleanJob accepts borrowed contour arrays with a fill rule, admits one point per grant, builds a Morton spatial tree, intersects and splits edges, classifies filled boundary sides and emits canonical rings under explicit edge/parameter/output/work budgets. Its intoRetirement returns source and completed output while retaining genuine owners for cooperative cleanup. Using this contour kernel during owned paint preparation would avoid flattening paths again or copying a completed scene during a query. PathBooleanJob is a higher layer that flattens and transforms source path operands again, so the direct contour kernel fits already flattened world-space preparation more closely.

The current Boolean kernel admits at most 4096 contours per operand and 65536 vertices/edges. Painted preparation admits 65536 contours and 262144 output points; stroke polygon union can create additional intersections and boundaries. These bounds must be designed and enforced explicitly in both twins. A silent additional cap, unbounded temporary owner, relaxed stack envelope or fallback to source bounds would not establish the intended contract. The current Boolean side probes use epsilon plus nearest-edge distance; canonical preparation also requires an explicit numeric precision contract.

## Required Witnesses

Author neutral fixtures for even-odd double contours, nonzero opposite-winding cancellation, harmless fill-only retraced external spurs, overlapping stroke polygons, fully cancelled/zero-area fills, reflected/affine transforms and a narrow real painted component. Validate appearance with existing independent Sharp SVG masks and compare complete point, rectangle and lasso picks plus painted bounds. Preparation cancellation, exact/one-short resources, child cleanup, immutable query ownership and full-width authority must remain covered. Shaped text, image-alpha and masks are distinct unresolved semantics. No canonicalization production source was changed in this continuation.

## Sources Read

- [Owned planar Boolean kernel](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻️2d/🔀️booleans/🟦️.ts)
- [Source-path Boolean preparation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻️2d/🔀️booleans/🛤️paths/🟦️.ts)
- [Current painted path preparation](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🎨️paint/📋️prepare/🎯️query/../../🟦️.ts)

Native interior-lasso owner 40112 is waiting on the normal artifact-directory lock while other shared native owners compile. Process observation found live Cargo/rustc workers; no owner was stopped or lock removed. This is a live verification dependency, with independent research continuing.
