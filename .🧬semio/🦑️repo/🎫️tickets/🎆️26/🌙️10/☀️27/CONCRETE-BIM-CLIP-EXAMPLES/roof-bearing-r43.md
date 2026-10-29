# Continuous Roof Post RC Junctions R43

Repaired the R42 bearing-on-finish/floating-material defect with explicitly inferred continuous reinforced-concrete column junctions. The two historic axes, exposed22×22cm roof profiles over6.60–9.03m, canopy soffit9.03m, source product identities and448-product count are preserved. No shim product or claim of measured concealed reinforcement was introduced.

## Source Publication and Inference

The parent approved the exact source patch and independent forecast before publication. 10-FIRST now has one continuous22cm square section over3.14–9.03m, merging its identical adjacent profiles. 15-FIRST retains its22cm circular section over3.14–6.34m and changes to the independently sourced22cm square roof profile at the6.34m deck soffit. Its square segment continues to9.03m. The newly modeled embedded interval6.34–6.60m is inferred continuous RC, not an archival measurement of hidden construction.

Both source products carry Pset_EmbeddedJunction with inferred classification, lower datum6.34m, unchanged exposed base6.60m and reinforcement=not_assessed. English/German historical uncertainty states that the concealed continuation is inferred and fabrication/reinforcement were not assessed. 10-FIRST's merged dimensional provenance retains the lower CSIC register and upper SAB2024p94 evidence. No force-transfer or reinforcement-detail assessment is claimed.

Each column gains0.012584m³, the0.0484m² square area over0.26m. Total added column RC is0.025168m³. Column10's full volume is0.285076m³;15's full analytic volume is0.251838467547m³.

The focused geometry publication was448products at33d8277cadf299f60e27e1f6ea8ea695a7b2fd01a80809f4e9129df00860ed35. The parent subsequently changed only notes, stair names and translation-loss wording, preserving all geometry/owners/materials, to final verified448-product source88d4aede8fab62deba894fe7fed5720b13c3287d85bae243c9bd47c41f3d4bfe. Fresh reads/narrow source replacements preserved the R40 boiler, opening and other concurrent work.

## Explicit Structural and Finish Ownership

Schema-first geometry.column_reserves references10-FIRST and15-FIRST on SL-R. These are material ownership reservations, not architectural openings or fictitious IFC portals. The generic column partition reuses the same exact occupied-prism subtraction/intersection interface as beam joints, with actual column section placement and vertical intervals. AABB pruning avoids clipping irrelevant source regions. Reserved-column geometry is part of the slab cache dependency; returned partitions remain isolated.

For every slab, parts and finish_parts are owned physical material. full_parts and full_finish_parts preserve complete semantic member geometry net of genuine architectural openings. shared_parts and shared_finish_parts explicitly reference column-common material with owner, column material and joint_host. A reserve can intersect structural material without reaching finish; this is separately tested. Stale structural owner references fail rather than silently cutting an arbitrary host.

The final roof structural partition has455owned prisms and10column-common parts. Structural sharing is0.0207328m³ and weathering/finish sharing is0.0044352m³. Their sum0.025168m³ exactly equals the new column RC interval. The full local occupied union is conserved, and former weathering material below the exposed post is assigned to the inferred RC column instead. The independent material proof finds no simultaneous slab/finish/beam occupied material inside either embedded column footprint.

Slab quantities.volume and area retain full semantic meanings. exclusive_volume/shared_volume partition structural volume; finish_volume/exclusive_finish_volume/shared_finish_volume separately partition weathering. material_quantities, weight and mass describe owned structural material. For the final roof, full structural volume46.741249664m³ equals owned46.720516864m³ plus shared0.020732800m³. The existing weathering layer remains a separate physical finish partition.

Selected public slab representations deliberately use full_parts and derived.scope=full_member, consistent with their full member quantity. Global building/drawing/vehicle material consumers continue to use owned parts and owned finish_parts. Native slab Body retains its existing structural-only export scope, now explicitly named exclusive_owned_structural_material; finish quantities are separately declared rather than silently adding a new native finish body.

## Beam Member Semantics and Updated Source Register

Beam full member geometry reads full semantic slab strips. Beam shared geometry combines actual slab-owned strips with slab column-common references, preserving the true physical owner. joint_host distinguishes raw beam/column common material from slab/column common material; it prevents treating a slab-owned member label as additional exclusive beam material. Native shared owner references include actual indirect column owners as well as direct source references.

The source-only NumPy/GEOS/SciPy forecast found four new raw beam-part/column pairs, all for10-FIRST:

|Beam|Part|Common volume m³|
|---|---:|---:|
|R-10-11|1|0.000726000|
|R-10-15|0|0.000076860|
|R-34-10|0|0.000002340|
|R-34-10|1|0.000726000|

The original97pairs are unchanged. Explicit owner references were added only toR-10-15 andR-34-10;R-10-11 already referenced10-FIRST. 15-FIRST adds no raw exclusive beam joint. The portable fixture now contains101pairs over33affected beam products, with pairwise sum0.5320605210904005m³. This is a sum of pair overlaps, not an asserted global union correction. Existing primary member envelopes and full member quantities remain conserved.

A final independent source-only sweep imports no model API:63nonauxiliary beams against67columns/99sections produces exactly101pairs, zero new/removed/changed values relative to the corrected fixture and zero owner-reference differences. The section count drops by one because10-FIRST's identical profiles were merged.

## Analytical Domain and Load/Support Preservation

analysis._slab selects full_parts/full_finish_parts in an isolated analytical view. It does not treat the material reserve as a load-domain hole. Tests verify both actual column supports10-FIRST/15-FIRST remain selected and analytical full slab area matches the full semantic quantity. Region load/section geometry remains complete while physical material outputs exclude column-owned volume. Source global analytical links and final snapshot refresh remain parent-owned.

## Test-First Geometry Proof

Portable 🏗️bearing.json and the native-product continuity test were written first. The first run failed on the real6.34→6.60m gap. The repaired source passes continuous native section coverage at6.341/6.50/6.54/6.58/6.599m, correct exposed top, and full0.0484m² contact with the actual canopy underside at9.03m.

Six RoofPostBearingContract tests verify continuous RC support, independent SciPy hull volumes and GEOS material identities/union conservation, explicit inference, source profile merge/staging, cache invalidation/isolation, preserved analytical supports/load domain and the structural-reserve-without-finish case. Local GEOS overlays use an explicit1e-10m precision grid with1e-9m² material tolerances; product geometry is not snapped by tests. The existing101pair fixture/37joint contracts retain their documented1e-10m independent precision grid and bounded cell area check.

Final combined regression: six bearing contracts,37joint contracts and two existing beam contracts pass,45tests in26.688s. Full source JSON Schema2020-12 validation passes with zero errors. Core, analysis and exchange modules compile. The final whole-course cohort is parent/clips_r11 integration work; the new six-test class was handed off for one standard load_tests registration.

## Independent Native IFC

Pinned web-ifc0.0.78 independently decoded16English/German samples: both columns, the owned structural roof slab, and R-4-10/R-10-11/R-10-15/R-34-10/R-14-15. It verified actual classes/GUIDs/localized names, RC material identity for both columns, inferred construction uncertainty, body counts, placed bounds, native profile/depth volumes and full/owned/shared/finish ownership properties/references.

|Product|Native bodies|Native mesh volume m³|Native analytic/profile volume m³|
|---|---:|---:|---:|
|10-FIRST|1|0.285075990448|0.285076000000|
|15-FIRST|2|0.251816531894|0.251838467547|
|SL-R structure|455|46.720517854803|46.720516863868|
|R-4-10|1|0.296225720122|0.296225706809|
|R-10-11|5|0.351626962878|0.351626940000|
|R-10-15|1|0.000002340000|0.000002340000|
|R-34-10|2|0.032294398197|0.032294400000|
|R-14-15|1|0.116379999068|0.116380000000|

15-FIRST retains an exact native IfcCircleProfileDef for its lower circle. The independent test reader requests CIRCLE_SEGMENTS=192; the observed boundary has191sides. Its expected polygonization loss is bounded independently by0.000021938325420m³, covering the observed circle mesh difference. Native analytic radius/depth and total profile volume match source quantities. No claim is made that a tessellated circle mesh has zero approximation error. The slab's placed mesh volume error is approximately9.91e-7m³; its independently decoded profile/depth volume agrees to about1.4e-10m³.

A separate oracle reads only SDK-decoded native profile/placement data, never the model API. It confirms complete0.0484m² column sections throughout the embedded interval and zero occupied intersection with actual native slab/beam bodies. At6.341m and6.50m, both native columns share0.88m of actual RC slab side boundary. At6.54m contact is0.428m for10-FIRST and0.88m for15-FIRST, reflecting the stepped local RC tops. Above6.57m the column remains continuous through the weathering zone. The finish partition is verified by the independent API/source geometry contract and separate native quantity scope; no native finish body was claimed.

No native Manim frame, concealed reinforcement fabrication detail or validated structural capacity was produced.

## Files and Handoff

Production/source changes:

- model/🐍️.py: generic column partition, slab full/owned/shared structural and finish geometry, cache dependency, partition quantities, full selected slab representations, and beam slab/member ownership references.
- model/🧮️analysis/🐍️.py: full semantic slab domain selector only; support/kernel algorithms unchanged.
- model/🔌️exchange/🐍️.py: direct slab selection and explicit structural/finish/indirect-owner property scopes.
- model/🔣️.json: the two continued column records, SL-R reserve references, two beam owner records and inferred junction evidence only; parent-owned later metadata publication preserved these fields.
- model/🧬️schema/🔣️.schema.json: column_reserves field.
- model/🧪️tests/🏗️joints.json and🏗️joints.py:101source-backed pairs and correct joint-host classification.
- model/🧪️tests/🏗️bearing.json and🏗️bearing.py: new portable fixture and six contracts.

Durable ticket inputs include roof-bearing-forecast-r43.py, roof-bearing-native-r43.py/.mjs, roof-bearing-native-contact-r43.py, this report and the source-only joints-r42-oracle.py with an optional in-memory forecast input. joints-r30-native.py now records actual indirect shared owners for its existing reusable sampler. Generated forecasts, native samples/profile dumps/results and reader dependencies remain under 🗑️generated for parent cleanup. No checked-in source file was copied to the ticket.

Source/API/exchange/schema producers are frozen. Parent and clips_r11 received the exact semantic/physical/native/analysis handoff. No shell, Git mutation, worktree, IDE, AGENTS edit, migration, proxy product or external runtime dependency was introduced.
