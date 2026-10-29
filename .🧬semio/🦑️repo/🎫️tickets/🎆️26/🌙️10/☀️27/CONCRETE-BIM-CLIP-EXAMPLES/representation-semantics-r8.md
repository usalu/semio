# Representation Semantics R8

## Changes

The generic part-object public representation branch now defines a south view with horizontal X and vertical Z. Elevations project each actual prism onto XZ. Sections intersect each authored polygon with the plane Y = (minimum part Y + maximum part Y) / 2 and emit one vertical rectangle per occupied interval. The returned derived metadata exposes this plane as cut_y and projection axes/view. Concave sections retain disconnected intervals; hollow fixtures retain their actual cavities. Axis uses Axis/Curve2D and footprint uses FootPrint/Curve2D. Existing 3D/body prisms remain unchanged.

Stair quantity volume now integrates the occupied union of step and landing prisms. A stdlib-only polygon sweep integrates merged intervals between polygon vertices and edge intersections, then integrates areas between prism height boundaries. No runtime library was introduced.

## Test-First Evidence

Before implementation, independent Shapely horizontal-slice unions found that the stair landing prisms overlap the steps. ST-GF summed prism volume is 1.129746124646 m³ but occupied union is 1.123035613276 m³; ST-FR summed volume is 1.076944822753 m³ but occupied union is 1.070234311384 m³. The old API returned steps-only 1.059546125 and 1.006744823 respectively. Tests first failed on missing cut_y, missing union helper, and wrong stair quantities.

## Executed Verification

Executed actual Python through the existing ticket python-validation.mjs CPython/Pyodide validator, using independent Shapely/GEOS operations. All five new co-located representations tests pass:

- Sections compare occupied projected areas against Polygon.intersection(LineString) for table, basin, planter, guard, and both stairs. Elevations compare polygon unions and full vertical bounds.
- Section cavity checks require both genuinely empty cavity samples and genuinely occupied material samples.
- Stair quantities match independently integrated horizontal slice unions including landings, and explicitly differ from both steps-only and double-counted prism sums.
- Portable polygon-union fixtures cover crossing edges and concavity.
- Portable U-shaped polygon sections cover disjoint intervals and cuts coinciding with boundary edges.

Additionally, 150 seeded random triangle-set unions match independent Shapely unions within 1e-8 m². All 44 current generic part/stair objects successfully produce all seven representation kinds with nonempty actual primitives. The five semantic tests were rerun after that broad API check and all pass. Final stair API values are ST-GF 1.123035613 m³ and ST-FR 1.070234311 m³.

## Scope and Limits

The language-agnostic contracts and Python tests are 🧱️model/🧪️tests/📐️representations.json and .py. Generic non-stair quantity policy remains unchanged. Projection primitives may overlap; their occupied union is the projection, preserving component identity rather than flattening into a single contour. Native rendering was not executed; no drawing provider changed. No generated report/log files were left by this agent; runtime cache lifecycle remains with the coordinator's existing shared validator.
