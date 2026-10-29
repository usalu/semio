# Hollow Piloti R22

## Result and Historical Basis

Piloti2/4 now have real profile voids, a real inferred shallow side outlet clearance, net section quantities and analytic moments, hole-aware physical drawings, closed component meshes and native hollow IFC profiles. This changes four existing column occurrences and TY-CIRC33-DRAIN, without moving outer placements, changing footings, adding fake wall openings or changing the solid22cm lessoncolumn19.

Primarybookp69 specifies nominal8cm drains and enlarged25×25cm buriedstems2/4. It does not establish that8cm is a measured outside diameter. Services geometry uses80mm outside envelope and3mm wall as explicit inferences. A100mm centred clearance bore, shallow outlet axis−0.60m,100mm exit band and+X300mm stub are explicitly inferred. Servicesowner authored the real hollow, mitered tube/stub and slab apertures; this agent authored the column material clearance. No underground network or bored footing is invented.

The330mm drainage-column contour is a finished/enlarged envelope. It does not establish a330mm RCcore or finish thickness. Geometric net area, volume and moments describe the authored envelope; the RCcore/finish split and stiffness remain unassessed. Existing280mm finished-column uncertainty is unchanged.

## Final Source Sections

|Occurrence|WorldZ interval,m|Physical profile|
|---|---|---|
|2/4|−1.00..−0.65|Original250mm square, solid|
|2/4|−0.65..−0.55|250mm outer square bounds with real C-shaped side clearance to+X|
|2/4|−0.55..−0.30|250mm square with100mm circular bore|
|2/4|−0.30..2.91|330mm circular envelope with100mm circular bore|
|2-FIRST/4-FIRST|3.14..6.34|330mm circular envelope with100mm circular bore|

The open Cprofile retains the original250mm outer bounds and removes the circular/side-channel material. Its100mm-high band surrounds the inferred−0.60m outlet. The deeper350mm of stem and all original footing solids stay intact. The Cnotch semicircle uses48segments; all diameters/positions remain editable source data. No facet reduction was used to conceal material overlap.

BoreConvention is one canonical inherited Pset_HistoricalEvidence property on TY-CIRC33-DRAIN. Native STEP writes it as IfcText, retaining the distinction between specified nominal drain size and inferred physical diameter/clearance/exit. Surface quantities have English/German labels.

## Schema and Physical Interface

column_profile supports explicit voids with tag, shape=circle, positive quantity diameter and local offset. API validation requires holes strictly inside the outer contour and disjoint. Existing circle, rectangle, ellipse, chamfered and aero polygon sections remain valid.

profile_geometry returns outer contour and inner rings, exact analytic net area/centroid/centroidal secondmoments, void area, outer and total perimeters and section_scope=authored_geometry_net_of_voids. I_x integrates squaredY distance, I_y squaredX distance and I_xy the product of centroidalX/Y distances; units arem⁴. Offset voids subtract parallel-axis terms before profile rotation.

column_geometry propagates these exact sections. Quantities subtract bore material and distinguish outer_surface_area, bore_surface_area and total lateral_surface_area. Analytical member sections carry net area and moments; stiffness_basis=not_assessed prevents these geometric moments being presented as calibrated RCcore stiffness. Structural bearing also subtracts void contact: slab material wholly inside a bore cannot count as support; the actual annular contact of source2/4 with SL1 remains valid.

part_intervals and part_sections return occupied cuts, including the two separated material strips through a closed bore. part_regions returns occupied horizontal material regions. Material cutting uses the exact identity A minus(O minusH)=(A minusO)union(A intersectH), avoiding annular fragment explosion. Collision-band footprints for native hollow profile prisms exclude the bore. Root separately owns partialZ clipping of the servicesowner's new polyhedral miter parts.

All seven column representation modes preserve inner contours. Production plan uses a region with holes; section/detail cuts show the actual void. Axon avoids the filled-cylinder fast path for hollow columns and draws inner walls plus one multicontour cap, without fake radial sector faces.

## Mesh and Native IFC

The stdlib model/🧊️profiles domain creates a conforming cap triangulation with shared boundary stations. Inner and outer walls share cap vertices; closed component mesh winding and edge multiplicity are validated. Each authored height section is exported as a closed manifold component. A multi-section column mesh is a multipart material representation, not a Boolean-unioned single exterior surface; coincident interface caps can remain between touching components.

Concentric circular bores export as native [IfcCircleHollowProfileDef](https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/lexical/IfcCircleHollowProfileDef.html). Its WallThickness parameter is the geometric outer-minus-inner radius; for330/100mm this is115mm, not an asserted measured RCwall thickness. General offset/rectangular/elliptical profiles use [IfcArbitraryProfileDefWithVoids](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcArbitraryProfileDefWithVoids.htm), with exact IfcCircle inner curves. Curved outer circles/ellipses remain native exact curves. Polygon outer curves remain true source polylines. Selected column STEP resolves the requested physical column directly instead of deriving the full building.

Native exchange remains the existing selected-product interface. This is not completebuilding/MVD/EXPRESSglobal-rule certification.

## TDD and Independent Validation

Portable model/tests/🕳️profiles.json was saved before implementation; the first four tests failed because profile holes, hole projections and source bores were absent. A later bearing fixture first failed because material wholly inside the bore incorrectly counted as support, then passed after actual contact subtraction.

Final checks:

-13bore/profile tests PASS3.023s: concentric circle, offset rectangle, ellipse and offset circle; analytic area/I; GEOS difference and SciPy integration; conforming closed manifold mesh; material ownership; all seven representations; production plan/section/axon; valid source/buried exit/unchanged footings; actual pipe/core nonoverlap at12Z stations−0.64..6.3m; invalid bores; native inference text; bilingual surface quantities; actual structural contact.
- Four actual source meshes PASS independent signed triangle volume versus GEOS material: ground2/4 each878vertices,1748triangles at48segments; upper2/4 each456vertices,912triangles. Finite circle tessellation is compared to its corresponding GEOS polygon, while analytic quantities retain exact circle area.
- Independent web-ifc0.0.78 engine PASS12models:8actual bilingual source exports plus4portable geometric cases. It reads real column/type/profile entities, exact native inner curves, placement/bounds, material volumes and IfcText historical inference. Its circle resolution is2048. No external library object enters runtime APIs.
- Actual source analytic envelope volumes:m³2/4 each0.28948365535174975;2-FIRST/4-FIRST each0.24856281075202447. Independent IFC mesh volumes are0.2894832905638555 and0.24856244509871264 respectively, within1e−6m³ numerical tolerance.
- Full Draft202012 source validation PASS448elements11.065s. Fresh full building_geometry PASS448elements18.326s after root's exact slab optimization.
- Full analytical derivation completed and both2/4 four-section members were checked for exact net annular area/I, geometric scope and unassessed stiffness. Initial50s observation expired; the completed result was checked asynchronously without repeating derivation. Actual net SL1 bearing for2/4 also passes after the new bearing predicate.
- Existing5positive-loft geometry tests PASS. Existing9production loft drawing tests PASS1.886s after handcrafted fixtures gained the current explicit domains field. Existing3planar section contracts PASS37.099s, including full schema and native body placement.

The native engine setup/cache remains test-only behind native-ifc-r18.mjs. Native-bore-samples-r22.py retains reproducible input generation, producing only derived selected IFC samples in generated output. Runtime implementation uses stdlib only.

## Visual QA and Handoff

Retained hollow-pilotis-r22.py plots actual source column material, bore air, tube metal and mitered outlet in three plan slices and a longitudinal section. Inspected generated/hollow-pilotis-r22.png and refined labels. The diagram visibly confirms the annulus, square buried bore, open side clearance, hollow bend and deeper solid stem. It labels the unresolved core/finish split.

Root/clips received the exact final source section contract and now own the global portable baseline, parking snapshot and link refresh. Eight stale link-used values were observed after the cohort's new slab penetrations (LKSLAB and LKROOF); this is recorded for that final refresh rather than hidden. Native Manim remains blocked by the repository launcher registry, so no full render claim is made.

## Files

- model schema/profile domain, model core, analytical core and exchange core.
- Scoped modelJSON column/type/provenance/quantity-label edits.
- model/tests/🕳️profiles.json and.py; main load_tests registration.
- Production savoye core,plan,section,detail,axon consumers.
- Existing ortho-remodel/test_frustums.py explicit domain fixtures.
- Ticket native-ifc-r18.mjs, native-bore-samples-r22.py, hollow-pilotis-r22.py and this report.

Unresolved historical facts remain the exact bore diameter/centering, pipe outside/inside convention and wall, shallow outlet location/direction, core/finish split, capacity and actual mechanical detailing. They are not promoted to measurements by the successful numerical tests.
