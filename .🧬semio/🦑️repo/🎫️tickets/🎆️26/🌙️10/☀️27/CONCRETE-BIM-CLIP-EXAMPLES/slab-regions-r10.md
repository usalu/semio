# Regional Structural Slabs R10

## Native Contract

Schema-first source now requires every IfcSlab geometry to declare level, outline, voids, top_offset and regions. Every region declares tag, outline, thickness and top_offset; optional material identifies a local material. Thickness and top_offset use native quantities in metres. Both offsets are relative to the named storey elevation. Root top_offset identifies the highest occupied structural top, rather than the finished storey surface. Flat slabs use one explicit region.

Region polygons partition the occupied root footprint. The evaluator intersects every region with the actual root, subtracts every void geometrically, supports concave roots and nonconvex voids, rejects overlapping occupied regions and uncovered surface, and emits disjoint closed convex prisms. Root outline, voids, top and maximum thickness remain envelope metadata. Slab storey and level are carried explicitly for consumer ordering. Physical geometry is exclusively parts: outline, voids, top, thickness, z0, z1, region, optional material. Parts have empty void arrays because subtraction has already split their polygons around holes.

## Physical Derivations

Quantities integrate each actual occupied region area times its depth; gross volume integrates regional depth over the unclipped-by-void region partition. Void area is root area less actual occupied union, so intersecting, nonconvex or out-of-bounds void rings cannot inflate the subtraction. Per-region takeoff accompanies the overall quantity.

The 3D and body representations emit all physical prisms. Sections intersect those actual occupied polygons with the documented middle-Y plane; south elevations project their actual X and Z extents. Axis and FootPrint retain native IFC identifiers. An unstationed soffit reference resolves the lowest occupied underside conservatively; common-soffit regions therefore produce their actual measured common underside.

## Independent Validation

Test-first observation failed with KeyError: parts before implementation. Five source-independent tests now pass through python-validation.mjs with Shapely/GEOS as the independent external oracle. The three portable cases cover a rectangular and a concave root, two depths sharing a soffit, a nonconvex void, an out-of-bounds void and overlapping voids. Analytical occupied areas and volumes are40m²/7.94m³,34m²/6.74m³ and38m²/7.54m³. Tests compare polygon differences, horizontal slice union integration, individual prism depths, full body propagation, actual section/elevation unions, conservative mixed-soffit selection and rejection of gaps/overlaps/nonpositive depths.

AJV8.20 independently compiles the JSON Schema, accepts valid regional quantities and rejects zero thickness and non-metre top offsets. A sixth regression now records the measured source datums and validates source solid unions and integrated volumes after source authoring completes.

## Integration Handoff

Source-authoring agent owns measured regional polygons and all five slabs. Parent owns analysis and axon propagation. This agent owns slab STEP multipart propagation. The exchange slab exporter now consumes regional polygon parts, reusing the existing multipart STEP serializer also used by columns. A test-first failure observed one uniform extrusion; the passing regression independently reconstructs every exported closed polygon prism with GEOS and verifies depths, soffits, volume and defined entity references. Analysis must use parts or regions for actual depth and bearing, rather than root top minus maximum thickness for a genuinely mixed-soffit slab. Root scalar metadata is insufficient for physical takeoff.

Source measured datum regression remains pending source authoring completion. Measured targets from structural-register-r9: first slab common soffit2.91m, occupied tops3.14m at23cm and3.08m at17cm; qualified rear20cm strip. Roof common soffit6.34m, most top6.51m at17cm, stair/bath top6.57m at23cm.6.77m is perimeter upstand only. Local finishes3.17m/3.11m and weathering6.60m remain separate.

## Files

- Model core 🧱️model/🐍️.py: slab-specific helpers, soffit resolution, geometry, takeoff and representations only.
- Model schema 🧱️model/🧬️schema/🔣️.schema.json: region definition, geometric properties and IfcSlab required contract.
- Exchange 🧱️model/🔌️exchange/🐍️.py: slab body source and shared STEP multipart dispatch only.
- Portable case 🧱️model/🧪️tests/🪨️regions.json.
- Independent regression 🧱️model/🧪️tests/🪨️regions.py.
