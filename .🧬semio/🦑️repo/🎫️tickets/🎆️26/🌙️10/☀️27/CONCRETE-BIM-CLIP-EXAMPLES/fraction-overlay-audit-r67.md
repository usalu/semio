# Exact Rational Overlay Area Audit R67

Read-only mathematical/implementation review,2026-10-29. Production geometry/source c53 and physical fixtures are outside this change. Actual Fraction helper and results are pending R40 delivery; no implementation pass is claimed here.

## Mathematical Basis

Convert each finite binary64 coordinate to its exact Fraction value. Partition the x-axis at every segment endpoint and every finite pairwise segment intersection across both input collections. Between consecutive critical x values there is no boundary vertex or order-changing edge crossing. Every intersecting nonvertical edge contributes an affine y(x), and their vertical ordering is constant throughout that open stripe. Per-polygon even-odd cross-sections therefore retain the same interval combinatorics; taking union within each side and XOR across sides changes only which affine endpoints contribute. XOR cross-section length is affine on that stripe, so midpoint length multiplied by exact stripe width integrates its area exactly. Summed stripe areas equal global area of the symmetric difference.

Collinear overlaps need no infinite intersection set: their endpoint x values already partition changes in coverage, and coincident affine lines do not exchange order. Vertical edges lie at critical x and have zero area contribution. Endpoints, tangencies and T-junctions likewise change topology only at included critical x values. Boundary-only contact has zero area and must not manufacture a positive stripe.

## Implementation Conditions

Apply even-odd parity to all outer/hole rings of EACH polygon, then merge interval coverage using OR across polygons of the same side. Applying parity across overlapping same-side polygons would incorrectly erase their overlap. Polygon/MultiPolygon nesting must preserve this distinction; ring orientation alone must not decide holes. Ignore only exact-zero-length edges/stripes, never small positive values. Exact x/y comparisons and intersection parameters must not use epsilons, grid snapping, coordinate rounding or a GEOS/PC fallback. Reject malformed odd crossing counts explicitly rather than fabricating a filled interval.

Include pairwise intersections within/between all actual and expected boundary segments. Exact disjoint-AABB rejection is a valid broad phase and can avoid unnecessary rational pair algebra; binary64 endpoint-order comparisons preserve exact input ordering. At parallel/collinear denominator zero, included vertices suffice. Output may use math.fsum of float-converted exact positive stripe areas; that is a final binary64 presentation/aggregation step, not a geometry tolerance. Keep the single original total<1e−8assertion.

## Required Independent Cases

Cover holes with either ring orientation, overlapping same-side polygons, missing/extra and expected-only extents, empty sides, negative coordinates, vertical edges, tangency/T-junction/collinear overlap, displaced equal-area shapes, and tiny positive1e−18area. Small ordinary cases can compare against independent GEOS; Node PC can compare ordinary cases and captured104frame where it works. The captured circular-sector/hollow-column cases must not be routed through a known-failing global PC/GEOS union merely to obtain a purported oracle. Exact analytic annulus-sector or independent per-part coverage checks are safer. Actual helper/data review and measured results will be appended when available.

## Actual Implementation Review

Reviewed actual test-only overlay.py. _compile converts coordinates with Fraction.from_float, records every vertex x and reuses exact edge slopes/intercepts. _crossings evaluates rational parameters over closed segment bounds, with only exact disjoint endpoint-AABB pruning; parallel/collinear pairs rely on registered endpoints. _intervals computes strict interior-stripe crossings, checks even parity per polygon including rings, then merges OR intervals across polygons. _xor_length computes exact union lengths minus twice interval intersection, and _xor_area integrates midpoint lengths over every critical stripe, raising on negative exact area. No epsilon, snap or topology fallback exists. LiteralCounter uses exact float.hex ring values only; equal literal geometry returns exact zero after input validation. Public overlay_json is a builtin string-to-string JSON interface; Fraction/_Edge remain private standard-library implementation details.

## Independent Actual Runtime Proof

Ran actual backend through its public JSON interface, not a copied substitute algorithm. Twelve small cases independently match GEOS at1e−12or tighter: overlapping same-side coverage, both hole orientations, missing hole, vertical edges, shared-edge and shared-vertex tangency, T-junction, collinear overlap, expected-only negative extent, both empty, positive1e−18area and crossing triangles. Exact output values:

```json
{
  "portable": [
    {
      "id": "overlap",
      "fraction_area": 0,
      "geos_oracle": 0
    },
    {
      "id": "holes-orientation",
      "fraction_area": 0,
      "geos_oracle": 0
    },
    {
      "id": "hole-missing",
      "fraction_area": 1,
      "geos_oracle": 1
    },
    {
      "id": "vertical-edges",
      "fraction_area": 1,
      "geos_oracle": 1
    },
    {
      "id": "tangent-edge",
      "fraction_area": 2,
      "geos_oracle": 2
    },
    {
      "id": "tangent-vertex",
      "fraction_area": 2,
      "geos_oracle": 2
    },
    {
      "id": "t-junction",
      "fraction_area": 0,
      "geos_oracle": 0
    },
    {
      "id": "collinear-overlap",
      "fraction_area": 0,
      "geos_oracle": 0
    },
    {
      "id": "expected-only",
      "fraction_area": 4,
      "geos_oracle": 4
    },
    {
      "id": "both-empty",
      "fraction_area": 0,
      "geos_oracle": 0
    },
    {
      "id": "tiny-positive",
      "fraction_area": 1e-18,
      "geos_oracle": 1e-18
    },
    {
      "id": "crossing-triangles",
      "fraction_area": 1.5,
      "geos_oracle": 1.5
    }
  ],
  "annular": {
    "engine": "fraction-sweep",
    "cases": [
      {
        "id": "annular",
        "xor_area": 8.870865977772452e-13
      },
      {
        "id": "annular-area",
        "xor_area": 0.07762043377033379
      }
    ]
  },
  "annular_simple_geos_polygon_area": 0.07762043377033376,
  "annular_seconds": 2.531954300000507
}
```

The captured annular column group that defeated PC executes in2.532s. Fraction XOR8.87086598e−13m² passes the unchanged1e−8criterion. Its union area0.07762043377033379m² independently agrees with GEOS evaluation of the single actual annular polygon0.07762043377033376m²; no pathological global GEOS/PC union was invoked. This validates the annular area reference while avoiding a false claim that an approximate global boolean kernel supplied the exact tiny-XOR oracle. Retained reproduction fraction-overlay-independent-r67.py. Captured104frame area check is running separately; full339group benchmark remains R40-owned.

## Captured104Frame Independent Result

Actual backend completes the captured104frame union-area plus exact identity-XOR batch in0.4041s. Area0.45947224758806315m² differs from root’s independent NodePC0.4594722475880584m² by only4.75e−15m², and identical-input XOR is exactly0. Thus the very group that stalled original globalGEOS and1m-localGEOS now completes with unchanged coordinates and no tolerance relaxation. Result generated/fraction-overlay-104-independent-r67.json; retained reproduction fraction-overlay-104-independent-r67.py. This independent review accepts the mathematical/interface and selected pathological-case gates; it does not claim the parent/R40full339batch or resumed cohort completed.
