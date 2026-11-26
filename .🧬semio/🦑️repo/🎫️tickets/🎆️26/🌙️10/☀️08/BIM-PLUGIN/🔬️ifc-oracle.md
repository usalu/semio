# IFC Example Oracle

The IFC4 standalone oracle resolves example snapshots from the subset assets directory. Its previous path resolved inside the tests directory and prevented example validation.

Running `🐍️.py check <fixtures> example-house example-office` with the workspace Python environment and IfcOpenShell 0.8.4.post1 reached the external validator after that correction. The committed house measurement differs from the IFC file; the office IFC contains 154 beams against 156 in the snapshot, instantiates abstract `IfcFlowTerminalType`, and differs from its committed measurement. Current exporter source selects `IfcAirTerminalType` for IFC4, so at least that error reflects a stale committed export. Both example exports and measurement tables require regeneration and revalidation.

The repository `test-oracle` target was also attempted; it waited for the graph service rather than reaching tests during this investigation.

The IFC4 non-example house passes its independent check. The IFC2x3 house check finds `o-door-2` has phase `New` in its IFC and `Demolished` in its snapshot. Production fixture regeneration was started through the BIM viewer's `BIM_BLESS=1` IFC4 example export tests.

The IFC4 psets, ceilings and wall-depth cases pass. The notated fixture fails an EXPRESS grid-axis ownership rule; the ramps fixture fails two EXPRESS rules for negative height and a USERDEFINED ramp without ObjectType. The current writer already has IFC4-specific corrections for these three conditions, indicating stale fixtures.

The three.js world oracle passes all six tests for demo and placed-demo via `repo test-file`. Shapely 2.1.2 standalone checks pass for storey levels and wall heights, wall joins, element solids and spaces. The solids check emits Shapely `oriented_envelope` runtime warnings while returning `check: ok`.

The IfcOpenShell wall-solids oracle passes all six element cases and the straight walls of the house and joins fixtures; arc walls are excluded from those two kernel checks.

The energy-envelope oracle passes its three Shapely/numpy cases. The gbXML lxml/Shapely/numpy structural audit passes; the official XSD is not committed, so that check does not validate against the schema.

The sheets SVG oracle and the house and notated SVG oracle cases pass. The full SVG oracle aborts because `🚪️svg/🪑️components/🪑️components.svg` is missing.

Additional independent checks pass: quantities (17 elements), families (8 families and 18 solids), sheets (3 sheets and 7 viewports), schedules (18 schedules and 114 rows), effective properties (13 holders and 7 findings), gestures (60 cases), modifications (36 cases), components (16 components and 6 MEP elements), stairs (15), ramps (16), wall depth (6 attached walls, 3 sweeps and 1 reveal), zones (2 zones and 4 schemes), opening frames (16 openings), finishes (4 spaces), ceilings (6), and plan/diagnostics (5 plan rows and 10 diagnostic rows). The annotations fixture was empty; it now contains one dimension and one tag and its independently generated Shapely table passes. Dependent notated exports need regeneration.

The mutation oracle's schema paths and typed-diff envelope were stale. After correction, sparse leaf schema fixes, and production regeneration of six inconsistent cases, the independent oracle passes all 1,576 quintets. The three native energy exports and their independently measured tables pass schema, geometry, and integrity checks. The zoning IFC oracle originally found three invalid empty group inverses; the writer and reader have since been corrected separately and need production fixture regeneration.

## Fresh native example export validation

The final native IFC4 exporter produced example-house (515,521 bytes) and example-office (1,363,581 bytes). IfcOpenShell 0.8.4.post1 independently parsed and audited the results, including EXPRESS validation: both passed with 53 entity classes; 29 and 336 geometry elements respectively were measured, and the measurements were refreshed from the new outputs.

## Preliminary gbXML producer

The native 17:04 gbXML archive predates the latest XSD fixes. Independent lxml/Shapely/NumPy analysis of its generated box, zoning and stack found box geometry correct (1 space, 6 surfaces). Zoning (2 spaces, 11 surfaces) has one closure mismatch: space sp-b encloses 30.78 m³ while the file declares 29.97. Stack (3 spaces, 16 surfaces) has two closure mismatches, sp-0 59.755 vs 57.9975 m³ and sp-b 51.319 vs 49.5615 m³, and two ceiling heat-loss mismatches, 32.3218390845 vs 41.1512195122 in the independent inference tables. All three also lack required location postal code and contain incomplete DocumentHistory; both XSD defects have since been fixed in source and require a fresh producer run. The audit script is `gbxml-probe-audit.py`; its outputs are preliminary and were not published as final fixtures.

## Restored rich annotation export

The restored annotation snapshot has 8 dimensions and 5 tags. Independent regenerated IFC2X3 measurement passes with 15 annotation objects and 5 measured solids; IFC4 passes with 5 solids. Both pass read-back comparison and IfcOpenShell EXPRESS validation. The SVG output has 2 views and passes its independent lxml/Shapely audit and read-back comparison.

The preliminary gbXML volume discrepancies above used shared surface polygons, whereas inferred volumes describe clear interior space. Interstitial wall and slab thickness can account for the larger shared shells; the writer and independent oracle are being extended to use explicit per-space interior boundaries. The discrepancies are therefore not evidence of a defective energy volume by themselves.

## Refreshed gbXML checks

The current production gbXML archive was linked with its exact snapshot dependency and serial native backend. Box (1 space, 6 surfaces), zoning (2 spaces, 11 surfaces), and stack (3 spaces, 16 surfaces) were generated at the canonical fixture paths with no exporter notes. All three passed official gbXML 7.03 XSD, lxml/Shapely/NumPy geometry, interior space-boundary closure, energy inference differential, and independently computed layer-based ISO interior film corrections; their measured tables were regenerated. House remains in production export. The CLI reported the missing fourth case rather than passing an incomplete fixture tree.

The IFC2X3 native suite found 14 passed and one stale test assumption: the house fixture contains non-new phases. The test now explicitly resets its input phases to New. IFC4 psets roundtrip found that importing a type with existing Semio_Authoring but no authored thermal values wrongly promoted an unrelated standard property into authored UValue; the importer now preserves the exact absence, and the existing roundtrip regression tests it after rebuilding. Additional existing IFC4 house, psets, and wall-depth fixture drift requires regeneration; notated, ramp, ceiling, and example exports have passed the native drift tests seen so far.

The regenerated house gbXML initially passed XSD but independent geometry found six bay openings extending outside host surfaces and 27 interior boundary failures. Investigation traced this to energy inference: uncovered horizontal portions carried the full room outline despite reduced remainder area, while hosted apertures extended beyond each curved wall piece. The production inference now uses true boolean difference for remaining regions, preserves disconnected portions and holes via triangulation, and clips openings horizontally and vertically while deducting the clipped area. A coherent selective WASI build of the changed spaces and downstream packages succeeded. The resulting box/zoning/stack outputs are byte-identical to their previously validated outputs and pass schema and geometry again; the house producer remains running. The pre-fix house result is intentionally not a passing fixture.

Fresh IFC4 house, property-set and wall-depth files have since been regenerated by the production exporter (zero notes) and independently pass EXPRESS and table read-back, with 11, 11 and 2 measured solids respectively.
