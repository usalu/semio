# Orthographic Archival Integration R9

## Implemented Geometry

Plans, sections, elevations and column details now use the canonical numeric part outlines and z0/z1 intervals. No physical column drawing reads radius or derives a bounding circle. Only the optional selection halo uses bounding_radius. Polygon sections use exact scanline intersections; circle, ellipse, rectangle, chamfer and concave outlines all follow the same interface.

Column details display authored part heights and source contour width/depth. The section contour preserves asymmetry relative to the source axis. Structural strips select the role('floor') slab, resolve their default y plane from role('column_example'), clip supports against the actual profile, and retain actual support heights until the explicit idealization control interpolates to the slab middle. Reference-grid row and bay controls remain optional drawing-plane selectors; physical product IDs are not inferred from grid labels.

Plans accept every authored storey, including cellar, with elevation +1.20 m cut height. Slab context selects the nearest physical slab below and above that cut, rather than treating every FLOOR as the first floor. Section/elevation centres and z_bounds include cellar slabs, foundations, beams and all authored solid parts. Level marks and height dimensions derive from the full vertical range.

Foundations and beams use the same explicit part protocol in all three orthographic families. Cellar stairs use source steps plus landings. Buried foundation contours are admitted only on their authored storey when below-context is requested.

## Material and Layering

Generic physical surfaces use shared core surface_color/surface_role. Source parquet, tile, paving and grass-joint parts paint at rank 1.0 beneath fixtures, ramps, walls and columns. Capped finish layers remain visible even when their top is slightly below the storey elevation. Room names no longer infer garden finish or blanket roof grass hatches.

## Verification

The ticket python-validation.mjs harness ran the actual pure Python production helpers with NumPy and Shapely. Fresh module reloads produced six passing tests:

- Wall occupied area and true boundary perimeter against independent NumPy raster occupancy.
- Canonical service envelopes, including actual radiator bounds 3.60–4.20 m.
- Polygon cuts against independent Shapely/GEOS line intersections, including circular, elliptical, rectangular, chamfered and concave contours.
- Four-storey cut heights, neighboring slabs and foundation-to-rooflight bounds against NumPy expectations.
- Capped floor-finish visibility, finish painter rank and buried-footing storey admission against independent NumPy masks.
- Execution of the production column detail with a portable staged noncircular column containing no radius field, through a geometry-recording drawing interface; NumPy confirms exact contour widths/depths and both part heights. This is a consumer geometry check, not a native Manim render.

All updated source/test modules also passed Python ast.parse in the WASM interpreter. A source-default role contract is present and awaits the shared source freeze supplying roles.column_example=19 and roles.floor=SL-1; it deliberately does not accept obsolete grid product tags. No native Manim rendering or frame QA has been performed. The repository native launcher remains the rendering gate.

## Files

- savoye/📋️plan/🐍️.py
- savoye/✂️section/🐍️.py
- savoye/🔩️detail/🐍️.py
- savoye/🧪️ortho-remodel/cases.json
- savoye/🧪️ortho-remodel/profiles.json
- savoye/🧪️ortho-remodel/test_defaults.py
- savoye/🧪️ortho-remodel/test_profiles.py
- savoye/🧪️ortho-remodel/test_profile_consumer.py

The existing wall/service fixtures remain in the same focused test folder. No source assets were copied into the ticket.
