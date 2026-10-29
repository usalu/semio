# Physical Architecture Audit R29

Read-only audit of live 449-element source, 2026-10-28. Read R24, R25, R26, R27, R28 and coordination R28. Inspected CMN plan pixels and complete registered Annex6 section23 pixels; read cached primary CSIC2022 article text (DOI10.3989/ic.88625). No model/schema/test edits, shell, Git operation, worktree, IDE or PDFJS execution. The sole write is this durable report. Parent owns existing ticket lifecycle.

## New Actionable Finding: Garden Beam/Post Joint Has Double Occupied Ownership

16-FIRST and22-FIRST now correctly contain three distinct functional sections. However their new cornice_support sections are returned in full by model/🐍️.py column_geometry (line1249), while primary beam parts are independently returned in full by _solid_geometry (line1118). Neither routine clips or assigns the beam/post common solid. Their full occupied parts therefore overlap at the actual support joint; this is not merely their bounding boxes touching.

The upper post outline, in local metres, includes(.12,-.008) and spansX18.84..19.02 after placement atX18.9. BEAM-PRIMARY-R-6-22 has two disjoint rectangles: X19..19.08/Y6..15.5/Z6.19..6.34 andX18.92..19/Y6.11..15.5/Z6.19..6.51. Both posts continue toZ6.34. BEAM-PRIMARY-R-22-29 occupiesX18.92..19.08/Y15.5..21.55/Z6.04..6.34, also inside the22-FIRST head.

Exact source-polygon clipping against the rectangular beam footprints, followed by shoelace area and vertical interval intersection, gives the following positive common volumes. These are direct authored-coordinate calculations, not independent production API/GEOS tests:

| Post | Beam | Part | Common area m² | Common height m | Common volume m³ |
|---|---|---:|---:|---:|---:|
|16-FIRST |BEAM-PRIMARY-R-6-22 |0 |.00139392857143 |.15 |.000209089285715 |
|16-FIRST |BEAM-PRIMARY-R-6-22 |1 |.01158079365081 |.15 |.001737119047621 |
|22-FIRST |BEAM-PRIMARY-R-6-22 |0 |.00073742857143 |.15 |.000110614285714 |
|22-FIRST |BEAM-PRIMARY-R-6-22 |1 |.00596829365077 |.15 |.000895244047615 |
|22-FIRST |BEAM-PRIMARY-R-22-29 |0 |.00626900000000 |.30 |.001880700000001 |

Total positive common solid is.004832766666667m³ (4.833litres), within these exclusive beam parts alone. The post envelope is MAT-RENDERED while primary beams are MAT-RC, so this also creates simultaneous material ownership in the common solid. R28 tests expressly cover garden auxiliaries/wall leaves, not these primary beam products; their passing result does not close this case. _material_quantities integrates parts within an object/material, not a cross-product joint ownership partition. No assertion is made that existing aggregate takeoffs currently sum these volumes: the physical product outputs themselves duplicate the occupied material, and any unqualified combined takeoff would do so.

CSIC2022 §6.2/Figure10 and Annex6 establish post-to-cornice engagement. Their overlapping modeled envelopes may be a natural initial description of a monolithic joint. The correction should be an explicit source-neutral ownership/quantity boundary with conserved union, retaining the intended post profile and beam extents. Do not shorten the historic post, move the primary beam, or delete the supporting relation to make an intersection disappear. A language-neutral joint fixture plus GEOS/SciPy union and native placed-solid checks should verify the chosen owner/material partition; any rendered finish at this joint needs its separately acknowledged inference. Broader primary beam/column ownership remains unaudited by this bounded polygon calculation.

## Fresh Checks That Do Not Produce New Absence Claims

REL-SUPPORT-16-FIRST names W-GARDEN-E,FACADE-L-GARDEN-E and BEAM-PRIMARY-R-6-22. REL-SUPPORT-22-FIRST names both garden/kiosk walls and lintels plus both adjacent primary beams. The pre-split W-E relation defect is corrected. These are inferred support topology, not a demonstrated load transfer calculation.

GARDEN-WEATHER-CAP is contained by roof and REL-CON-roof; local base_level first is a coordinate reference. Its roof-storey assignment is correct in live source. No fresh callback render was executed here, and R28 supplies the previous actual callback verification.

CMN room/access topology was compared with live cellar, ground and first-floor records. Cellar heating plant is now cellar-contained; the old ground BL-1-location claim is obsolete. SP-BOILER still names a ground Boiler cupboard, but this space label alone does not establish a conflicting physical plant or an access defect. No new defensible privacy or circulation collision was established from these pixels and source inspection. The registered guest dressing rooflight is a current constructed assembly; the obsolete solid glass cube claim is not repeated.

## Evidence Limits

The known unregistered CW flat-pane stations, exact original ribbon/salon pane train and hardware, three original roof-terminal footprints/full hidden plant and provisional solarium boundary remain existing work, not new findings. Full module framing must remain source-bound: absent original stations cannot license a uniform invented historical schedule. The cached author reconstruction is source evidence with stated uncertainty, not exact original shop geometry. No native Manim frame was claimed; the repo launcher gate remains parent-owned. Exact CAG fragment-count improvements do not alter the physical fidelity conclusion above.

## Broad Source Joint Sweep Requested by Parent

Extended the same source-coordinate rectangle/polygon oracle to every IfcColumn section and every exclusive BEAM-PRIMARY part on both structural floors. All authored primary beam exclusive outlines are axis-aligned four-corner rectangles (verified before using this oracle). Circle sections use96vertices, matching production default; profile voids are subtracted with their offsets. Section-specific axes and absolute/storey elevation mappings are honored. No production execution or third-party validation was performed, so these are audit measurements requiring an independent implementation test, not certified takeoff totals.

| Beam family | Positive section/part intersections | Affected column products | Sum of pairwise common volumes m³ |
|---|---:|---:|---:|
|First ceiling,F |51 |32 |.302108246149 |
|Roof ceiling,R |46 |27 |.227512373772 |
|Combined |97 |49distinct |.529620619921 |

These are pairwise overlap sums, not a global union error estimate: beam-to-beam and column-to-column ownership was not unioned by this oracle. The circle approximation also bounds metric precision. All positives involve exclusive source beam parts, therefore cannot be excused as the intentional beam/slab shared_parts contract. The existing slab/member ownership contract is valid in purpose: beam full quantities include member-labelled slab regions, while drawings combine exclusive beam parts with slab parts once; exclusive_volume/shared_volume make that sharing explicit. There is no corresponding registered column-joint ownership partition.

Examples beyond posts16/22: column2 intersects F-2-7 by.009877348501m³ after its bore is deducted; column3 intersects F-3-34 by.008849091838m³; column27 intersects F-20-32 by.017647166721m³; column20-FIRST intersects R-20-32 by.011299275788m³. First-floor column bases also intersect upstand F beams:1-FIRST/F-1-6=.009504m³ and16-FIRST/F-6-29=.007987827273m³. Thus correction must account for column bases as well as heads, transfer members and staged sections. Column14 is cellar-contained but has a ground structural section and contributes.00726m³ against F-13-14; grouping by storey alone would miss it.

The full pair register follows so the next executor can reproduce and independently test it:

|Column|Beam|Part|Pairwise volume m³|
|---|---|---:|---:|
|1-FIRST|BEAM-PRIMARY-F-1-6|0|0.009504000000|
|1-FIRST|BEAM-PRIMARY-R-1-6|0|0.010560000000|
|2|BEAM-PRIMARY-F-2-7|0|0.009877348501|
|2-FIRST|BEAM-PRIMARY-R-2-7|0|0.008837036878|
|3|BEAM-PRIMARY-F-3-34|0|0.008849091838|
|3-FIRST|BEAM-PRIMARY-R-3-34|0|0.005183551799|
|4|BEAM-PRIMARY-F-4-10|0|0.009877348501|
|4-FIRST|BEAM-PRIMARY-R-4-10|0|0.008837036878|
|5-FIRST|BEAM-PRIMARY-F-5-33|0|0.009504000000|
|5-FIRST|BEAM-PRIMARY-R-5-33|0|0.010560000000|
|6-FIRST|BEAM-PRIMARY-F-1-6|0|0.004752000000|
|6-FIRST|BEAM-PRIMARY-F-6-29|0|0.000291600000|
|6-FIRST|BEAM-PRIMARY-F-6-29|2|0.005054400000|
|6-FIRST|BEAM-PRIMARY-R-1-6|0|0.005280000000|
|6-FIRST|BEAM-PRIMARY-R-6-34|0|0.002631200000|
|6-FIRST|BEAM-PRIMARY-R-6-34|1|0.000151800000|
|6-FIRST|BEAM-PRIMARY-R-6-34|3|0.000759000000|
|6-FIRST|BEAM-PRIMARY-R-6-22|0|0.001320000000|
|7|BEAM-PRIMARY-F-2-7|0|0.008849091838|
|7|BEAM-PRIMARY-F-7-24|0|0.006998023487|
|7-FIRST|BEAM-PRIMARY-R-2-7|0|0.005697920619|
|7-FIRST|BEAM-PRIMARY-R-6-34|0|0.004065076867|
|7-FIRST|BEAM-PRIMARY-R-6-34|1|0.000303328941|
|8|BEAM-PRIMARY-F-8-9|0|0.004424545919|
|8|BEAM-PRIMARY-F-8-9|2|0.004424545919|
|9|BEAM-PRIMARY-F-8-9|0|0.004424545919|
|9|BEAM-PRIMARY-F-8-9|1|0.004424545919|
|10|BEAM-PRIMARY-F-4-10|0|0.007260000000|
|10-FIRST|BEAM-PRIMARY-R-4-10|0|0.007260000000|
|10-FIRST|BEAM-PRIMARY-R-10-11|0|0.003630000000|
|11-FIRST|BEAM-PRIMARY-F-5-33|0|0.009504000000|
|11-FIRST|BEAM-PRIMARY-R-5-33|0|0.010560000000|
|11-FIRST|BEAM-PRIMARY-R-10-11|0|0.000990000000|
|11-FIRST|BEAM-PRIMARY-R-10-11|2|0.000990000000|
|12|BEAM-PRIMARY-F-7-24|0|0.013910354587|
|13|BEAM-PRIMARY-F-13-25|0|0.001980000000|
|13|BEAM-PRIMARY-F-13-25|1|0.001980000000|
|13|BEAM-PRIMARY-F-13-14|0|0.000990000000|
|13|BEAM-PRIMARY-F-13-14|1|0.003630000000|
|13|BEAM-PRIMARY-F-13-14|2|0.000660000000|
|14|BEAM-PRIMARY-F-13-14|1|0.003630000000|
|14|BEAM-PRIMARY-F-13-14|2|0.003630000000|
|14-FIRST|BEAM-PRIMARY-R-14-26|0|0.002418990839|
|14-FIRST|BEAM-PRIMARY-R-14-26|1|0.002418990839|
|15|BEAM-PRIMARY-F-15-20|0|0.007260000000|
|16-FIRST|BEAM-PRIMARY-F-6-29|2|0.007987827273|
|16-FIRST|BEAM-PRIMARY-R-6-22|0|0.000209089286|
|16-FIRST|BEAM-PRIMARY-R-6-22|1|0.001737119048|
|17-FIRST|BEAM-PRIMARY-F-5-33|0|0.009504000000|
|17-FIRST|BEAM-PRIMARY-R-5-33|0|0.010560000000|
|18|BEAM-PRIMARY-F-13-25|0|0.003960000000|
|18|BEAM-PRIMARY-F-13-25|1|0.003960000000|
|19-FIRST|BEAM-PRIMARY-R-14-26|0|0.004837981679|
|19-FIRST|BEAM-PRIMARY-R-14-26|1|0.004837981679|
|20|BEAM-PRIMARY-F-15-20|0|0.007260000000|
|20|BEAM-PRIMARY-F-20-32|0|0.007260000000|
|20-FIRST|BEAM-PRIMARY-R-20-32|0|0.011299275788|
|21|BEAM-PRIMARY-F-7-24|0|0.013910354587|
|22-FIRST|BEAM-PRIMARY-F-6-29|2|0.007987827273|
|22-FIRST|BEAM-PRIMARY-R-22-29|0|0.001880700000|
|22-FIRST|BEAM-PRIMARY-R-6-22|0|0.000110614286|
|22-FIRST|BEAM-PRIMARY-R-6-22|1|0.000895244048|
|23-FIRST|BEAM-PRIMARY-F-5-33|0|0.009504000000|
|23-FIRST|BEAM-PRIMARY-R-5-33|0|0.010560000000|
|24|BEAM-PRIMARY-F-7-24|0|0.006955177294|
|24|BEAM-PRIMARY-F-24-30|0|0.003245749404|
|24-FIRST|BEAM-PRIMARY-R-24-30|0|0.013662000000|
|25|BEAM-PRIMARY-F-13-25|0|0.001980000000|
|25|BEAM-PRIMARY-F-13-25|1|0.001980000000|
|25|BEAM-PRIMARY-F-25-28|0|0.001760000000|
|25|BEAM-PRIMARY-F-25-28|1|0.001760000000|
|26|BEAM-PRIMARY-F-26-28|0|0.001760000000|
|26|BEAM-PRIMARY-F-26-28|1|0.001760000000|
|26-FIRST|BEAM-PRIMARY-R-14-26|0|0.002464000000|
|26-FIRST|BEAM-PRIMARY-R-14-26|1|0.002464000000|
|27|BEAM-PRIMARY-F-20-32|0|0.017647166721|
|27-FIRST|BEAM-PRIMARY-R-20-32|0|0.011299275788|
|28|BEAM-PRIMARY-F-28-31|0|0.005280000000|
|28|BEAM-PRIMARY-F-28-CROSS|2|0.002560000000|
|28-FIRST|BEAM-PRIMARY-R-28-31|0|0.005280000000|
|29-FIRST|BEAM-PRIMARY-F-6-29|1|0.005346000000|
|29-FIRST|BEAM-PRIMARY-F-6-29|2|0.005346000000|
|29-FIRST|BEAM-PRIMARY-R-22-29|0|0.010560000000|
|30|BEAM-PRIMARY-F-24-30|0|0.002772000000|
|30-FIRST|BEAM-PRIMARY-R-24-30|0|0.007260000000|
|31|BEAM-PRIMARY-F-28-31|0|0.008168701170|
|31-FIRST|BEAM-PRIMARY-R-28-31|0|0.005697920619|
|32|BEAM-PRIMARY-F-20-32|0|0.007260000000|
|32-FIRST|BEAM-PRIMARY-R-20-32|0|0.005649637894|
|33-FIRST|BEAM-PRIMARY-F-5-33|0|0.009504000000|
|33-FIRST|BEAM-PRIMARY-R-5-33|0|0.010560000000|
|34|BEAM-PRIMARY-R-3-34|0|0.005940000000|
|34|BEAM-PRIMARY-R-34-35|0|0.001683000000|
|34|BEAM-PRIMARY-R-34-35|1|0.001683000000|
|34|BEAM-PRIMARY-R-6-34|4|0.000561600000|
|35|BEAM-PRIMARY-R-34-35|0|0.001683000000|
|35|BEAM-PRIMARY-R-34-35|1|0.001683000000|

## Concurrent Garden Wall Subtraction Investigation

Parent reports that the old garden-band3.86–3.92m MAT-HOLLOW-CLAY decomposition had4048pieces and area sum1.794m² against raw area approximately1.13m²; a proposed exact AABB pruning reduces wall parts to56but old/new integrated difference is.1067m³. This audit has not independently reproduced either decomposition or its GEOS expected source-cutters union. Therefore this is a parent-reported active correctness investigation, not a verified second finding. Fragment reduction alone cannot establish conserved occupied geometry; comparing each output union to the source-minus-all-relevant-cutters oracle is required before preserving old overlap or accepting new loss. Parent owns that computation.
