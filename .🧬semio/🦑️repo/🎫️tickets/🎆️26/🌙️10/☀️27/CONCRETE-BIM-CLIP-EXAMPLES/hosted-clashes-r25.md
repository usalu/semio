# Hosted Material Clashes R25

## Producer Defect

Current clash_check minimizes pipe centreline distance to filled column outer contours and subtracts the pipe radius. It ignores the source column bore, hollow tube wall and miter shape, producing false concealed-drain violations. Currentgeneric rule requires0.25m as an assumed teaching clearance; it is not a sourced installation requirement for an intentional conduit inside a piloti.

## Approved Direction and Contract

Use actual occupied convex solids and minimum Euclidean material distance. distance is nonnegative; intersects states physical material contact/intersection. Remove the duplicate clearance alias. Expose a_point and b_point as explicit closest occupied witnesses and point as their marker midpoint. Level follows the structural element native storey. Positive-distance witnesses lie on actual material boundaries.

Native IfcRelConnectsElements host-to-pipe relationships gain connection={kind:hosted_conduit,clearance_rule:RULE-RAINWATER-PILOTI,basis:inferred,source:...}. Six pairs connect2/2-FIRST toDP1,2toDP1-OUTLET, andthecorresponding4/DP2 family. The generic rule remains0.25m for unhosted arrangements. Its explicit hosted policy reserved_void requires strictly separated material with0m assumed minimum; actual available boregap (~10mm nominal,~9.995mm faceted) is reported rather than being compared with the unrelated externalteaching clearance. This is not a sourced minimuminstallation-gap claim.

Hosting is not a pair ignore: the pipe axis must intersect the host's gross convex envelope, stale relations fail validation, and actual pipe/concrete contact remains a violation. Removing/filling a bore or enlarging/shifting a pipe cannot be hidden by the relationship. Hypothetical reroutes must remove/translate host relations, penetration references and miter planes.

## Implemented Contract

Schema-first rule/relationship and result contracts; portable convex/annular/hosted/interference cases; stdlib GJK convex distance with AABB pruning and exact hole-aware material decomposition. New test-only independent_distance helper uses GEOS occupied polygon distances for prisms and independent SciPy halfspace optimization/SAT for actual3D miter parts. It returns plain data and is shared with the main clash consumers. Preserve source tessellation and all other collisions.

Aeropost/cap registration has not started and remains paused for this producer priority.

## Verification Recorded

The stdlib occupied-distance producer uses exact-coordinate bounded caches, AABB lower bounds, convex material decomposition and GJK witnesses. The test-only independent oracle uses GEOS polygons with holes and SciPy constrained hull optimization; it exposes only plain data. Five initial contracts passed in 2.717 s warm; three additional interference/lumen/3D contracts passed in 2.356 s. Cold current-source clash calculation took 18.219 s; an identical repeat took 1.355 s with identical rows. Full schema/progress validation and the newly requested all-beam passage proof are being completed separately.

Scope is the configured rainwater-pipe versus column rule, not universal BIM clash detection. The ordinary external 0.25 m clearance remains an assumed teaching policy. Reserved-void hosting requires physical non-contact; the sourced nominal 80 mm drain designation does not establish an outer diameter, installation tolerance, or minimum clearance. Geometric OD80, wall3 and bore100 are explicitly inferred conventions.

Fresh-worker verification: all nine occupied-clash contracts passed in 33.952 s, including the complete live model through Draft202012Validator, every result through the strict clash schema, progress completion, cancellation, stale hosting rejection, filled-bore interference, an empty-lumen counterexample, and independent SciPy 3D distances.
