# Column Profiles R9

The model now derives actual circular, rectangular, elliptical and polygonal column profiles, with optional local rotation in degrees. No runtime third-party dependency was introduced. Source assets remain owned by the structural-register author; this implementation does not invent an oval major axis or classify inferred dimensions as sourced.

## API Handoff

Type profile shapes are lowercase circle, rectangle, ellipse and polygon. Circle uses diameter quantity; rectangle and ellipse use width/depth quantities; polygon uses local-coordinate outline. All accept optional numeric rotation in degrees. Optional occurrence geometry.sections contains profile, base_level, top_level and optional absolute axis per section. Sections describe adjacent vertical extrusions; they are not a loft interpolation. Parent geometry base/top provides extent metadata while sections drive the physical parts.

building_geometry.pilotis entries expose source profile metadata; parts containing outline, z0, z1, profile, area and perimeter; outline for the first physical contour; base/top; and bounding_radius. Each circle part additionally exposes scalar radius. Overall radius exists only when all parts are circular. bounding_radius is explicitly for halos; consumers must use actual part outlines for physical drawing/intersection.

_profile_geometry(profile,axis,segments=96) resolves a profile to physical outline, analytic area (circle/ellipse/rectangle), perimeter and source profile. _column_geometry(model,obj,segments=96) resolves occurrence parts. quantities uses analytic area times actual section height and sums all actual occurrences for type volume. Diameter is absent for noncircles; width/depth remain shape-specific. information now resolves those dimensions and basis from the actual occurrence type, rejecting a uniform dimension for vertically changing sections.

represent emits polygon plans/footprints, actual extrusions/prisms and per-section elevations. Its section view intersects the actual contour with the middle Y cut. Curve contours use 96 samples by default; quantities retain analytic area. Ellipse perimeter uses Ramanujan's approximation.

STEP exports native IfcCircleProfileDef, IfcRectangleProfileDef, IfcEllipseProfileDef and IfcArbitraryClosedProfileDef, with each vertical section as its own IfcExtrudedAreaSolid. Rotation is explicit on native profile placements. Polygon contours use an explicitly closed IfcPolyline. Mesh export samples curves at requested segments, preserves polygon corners, ear-clips concave caps and exports each section as a closed prism. Layout contains bottom/top index arrays and face ranges; no cap-centre vertices exist. part_layouts contains one layout per section. Consumers formerly assuming centre-fan topology must use the triangle list/face ranges.

## Executed Tests

Portable fixtures: clip/🧱️model/🧪️tests/📏️columns.json. Executable independent oracle: matching 📏️columns.py.

Executed through the ticket python-validation.mjs openValidator(root) direct Node API, using isolated Python/WASM with NumPy and Shapely. Initial red test failed because _column_geometry did not exist. Final run passed circle, rectangle, rotated ellipse, chamfered polygon, rotated concave polygon and vertically changing sections. Tests independently compare analytic source areas and volumes; Shapely contour areas; NumPy signed triangle extrusion volumes (relative tolerance .0002 for discretized curves); closed edge manifold counts; STEP profile entity choice/reference closure; all seven public views; and exact type occurrence volume sum. No native renderer or external IFC parser was executed by this subtask.

The source-specific schema extensions were coordinated with cellar_remodel; schema and source JSON were not edited by this subtask. Physical graphics, collision and analysis consumer updates remain coordinated by the parent.
