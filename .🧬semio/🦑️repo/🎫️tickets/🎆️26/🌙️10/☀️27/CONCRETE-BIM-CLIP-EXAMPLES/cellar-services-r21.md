# Cellar Services R21

## Result and Scope

Published nine native source entities and three types on the fresh386-element source, preserving concurrent roof changes. This implements the registered hoist pit and coal chute as hollow construction with actual vertical cellar-wall mouths, native horizontal slab apertures, separate removable covers and a minimal inferred metallic jib. It does not establish a fully measured historical mechanism or verified parking capacity.

### Source Elements

|Tag|Native class|Physical function|
|---|---|---|
|CELLAR-HOIST-PIT|IfcBuildingElementProxy|Three-sided concrete shaft, open to actual east cellar passage|
|CELLAR-COAL-CHUTE|IfcBuildingElementProxy|Inclined floor and channel sides, separate shallow terminal shaft|
|OP-HOIST-CELLAR / OP-COAL-CELLAR|IfcOpeningElement|Actual apertures through W-CELLAR-E / W-CELLAR-W|
|OP-HOIST-HATCH / OP-COAL-HATCH|IfcOpeningElement|Horizontal apertures in expanded SL-GROUND-CELLAR|
|COVER-HOIST / COVER-COAL|IfcSlab|Independent cover plates linked by IfcRelFillsElement|
|HOIST-JIB|IfcBuildingElementProxy|Minimal plausible inferred metal post, arm and lifting line envelope|

SL-GROUND-CELLAR has disjoint registered projection regions before its native holes are subtracted. SL-CELLAR extends only beneath the flat hoist pit. The coal floor is an actual inclined channel belonging to its assembly, rather than a filled horizontal extension. Existing measured base-cellar slab thicknesses remain70/80mm; applying them to the projections is explicitly inferred. Type, material, spatial containment, void/fill and partial-model relationships are authored. Jib membership includes services; geometry dispatch uses native flow classes rather than treating every services-discipline element as a pipe.

## Evidence and Limits

Read horizontal-openings-r19.md, hoist-projections-r17.md and its input, foundations-and-hoist-r15.md, and model author's hoist-coal-terminal-r20.md. Visually inspected archive-r8-page18.png (Annex1) and register-r9-section23.png (Annex6) at source resolution.

The crossed coal terminal box at approximatelypixels709..727,782..806 gives boundsX2.562670..3.028610,Y15.648639..16.269048m (about0.466×0.620m). This is distinct from the whole long chute clear candidate. Plan registration has approximately0.04m uncertainty and does not resolve the frame/clear distinction; the traced box is authored as an inferred rough aperture. Its terminal cover datum−0.13m and cellar floor−2.42m follow Annex6 annotations. The terminal bend−0.50m is a graphical inference, not a labelled measurement. The ramp and shallow terminal shaft remain hollow.

Bookp69 describes planned concrete for both cellar projections; installed material and detailed thicknesses are unmeasured. Concrete material choice, channel floor80mm vertical thickness, projection wall/roof extents and cover20mm equivalent metal plate are explicit teaching inferences. Hoist-versus-coal side attribution follows textual functions and adjacency, not a plan label.

CMN2021 section9 attests a metallic pivoting lifting jib from garage to cellar. The model only gives a minimal plausible inferred envelope:180mm base plate,70mm post,685mm arm and1.95m overall top. It does not claim surveyed pivot bearings, anchorage, rated capacity, exact profiles, hardware or a simulated mechanism. Cover material, thickness, hinges, frames and lifting hardware remain unresolved. No horizontal IfcDoor contract is invented; independent native slabs represent the covers.

The coal assembly and jib use identified native physical proxies rather than falsely assigning an attested manufactured product type. Native type inheritance follows [buildingSMART IFC4X3_ADD2 IfcBuildingElementProxyType](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcBuildingElementProxyType.htm). The classroom source is not a complete validated IFC model; selected STEP export remains limited to its explicitly supported classes. Proxy section-sweep STEP export is unsupported and is not claimed here.

## Schema and Shared Geometry

A solid_part is now discriminated between existing XY prism/positive compatible convex loft and a native planar section extrusion: section={plane:XZ|YZ,profile:[[horizontal,z],...],sweep:[a,b]}. Profiles must be convex, positive-area and swept over an increasing interval. No zero-area tapered end profiles or epsilon wedges were introduced. Existing loft validity remains unchanged.

solid_geometry(key,model), part_vertices, part_faces, part_solid and part_body expose exact shared physical geometry. Normalized section parts retain section plus bounds; outline is the plan silhouette and is never used as occupied prism geometry by quantity or drawing consumers. Horizontal slices, sections, elevations, footprints, closed outward faces and native oriented extrusion placement derive from the same section. Occupied volume integrates actual horizontal slice unions, with profile vertex heights included as integration stations. Native body placement specifies axis/ref_direction and world extrusion direction. All seven representation modes and production plan/section/axon providers consume those exact shapes.

The private _part_vertices name was removed and its checked-in callers updated to public part_vertices; no aliases remain. New native type classes participate in class_chain, type_occurrences and aggregate quantities. Both new test modules are registered by main load_tests.

## TDD and Independent Validation

Before source mutation, portable services.json plus five initial tests failed because service entities were absent. Native section fixture was authored before implementation and failed on the missing public physical interface. Final portable fixtures are model/tests/🏗️services.json and model/tests/🧊️sections.json.

Current checks:

- Eight service tests PASS2.652s: registered small hatch versus whole chute; actual native slab subtraction with covers hidden; hollow shaft/channel; independent SciPy hull volume; source relationships/type/material/partial models; worldZ-aware column and door nonoverlap; real wall mouths; cover separation; native type hierarchy and totals.
- Three section tests PASS34.409s: XZ/YZ independent GEOS area×sweep and SciPy hull volumes, manifold edge counts and signed volume, native profile-axis placement, every representation, actual production plan/section/axon vertices, rejected concave/reversed/ambiguous source forms, Draft202012 schema.
- Existing five positive-loft geometry tests PASS; existing nine production loft drawing tests PASS1.953s.
- Two existing cellar regressions PASS. Their stale tests were corrected to regional slab thickness and world datums (source-local stair/footing coordinates were previously compared directly to worldZ). Existing geometry was not changed to satisfy those assertions.
- Fresh full Draft202012Validator source PASS386 elements10.297s; complete building_geometry PASS8.038s; consistency_issues=[]; no checked-in _part_vertices callers remain.
- Test-only Matplotlib3.8.4 and its12-package dependency closure were loaded from official Pyodide0.29.0 and every cached archive SHA256 verified against its official pyodide-lock. No external runtime dependency was introduced.

Selected quantities, in m³: hoist shaft3.685947935; coal assembly0.730270211; hoist cover0.017424683; coal cover0.005781467; jib0.0140365. Native hatch areas are0.87123417318m² hoist and0.28907336946m² coal. Native removed slab volumes are0.06098639212m³ and0.02023513586m³ respectively.

## Visual QA and Parking Handoff

Retained inputscript cellar-services-r21.py generates 🗑️generated/cellar-services-r21.png using actual source parts. Inspected its cellar cut, ground cut with covers hidden, coal longitudinal section and hoist/jib section. The diagonal floor, shallow terminal shaft, separate cover, continuous cellar mouth and hollow hoist pit are visible; no filled projection block conceals a nominal opening.

The old two-car control[12,14,90°] intersects the new physical jib by0.033m²; this is an actual volumetric interference over the vehicle height. Its0.554893150644m² hatch-footprint overlap is a separate operating/delivery-reserve issue: the closed flush cover ends atZ0 and is not itself an above-floor car collision. The other old control[13.79,9.5,90°] intersects neither. No shaft, jib, column, building or vehicle was moved or shrunk to preserve parking poses. The parking owner received the exact parts, overlap diagnostics and input plot and now owns vehicle obstacle integration/control/signature refresh. Historical three-car capacity and maneuver verification remain separate unresolved checks.

## Touched Files

- model/🔣️.json: scoped cellar assets/regions/relations/types/class tree and memberships.
- model/🧬️schema/🔣️.schema.json: native service type classes and planar section sweep domain.
- model/🐍️.py: shared section geometry, public physical part interfaces, native type aggregates and class-based flow geometry dispatch.
- model/tests/🏗️services.json,.py and 🧊️sections.json,.py: portable fixtures and independent tests.
- model/tests/🔺️frustums.py: public part_vertices consumer.
- model/tests/🕳️cellar.py: current native openings and relative/world datum oracle corrections.
- main tests/🐍️.py: new module registrations, fresh scoped edit.
- visuals/savoye/core,plan,section Python providers: exact shared section support.
- ticket/cellar-services-r21.py and this report: retained diagnostic input and evidence/validation record.

Native Manim render remains blocked by the repository launcher registry; no full render or perfect historical completion claim is made. Generated diagnostic files remain under the ticket's generated directory for parent review and normal ticket cleanup.
