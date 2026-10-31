# Initial Whole-Model Interference Findings

Source575/d575c427 capture includes all512 physical products and33,952 exclusive owned parts, with no missing product. Source bytes remained unchanged through711.37 seconds of capture. The initially loaded API was57ba53f6; its file changed to2275e68e during capture, so this is an initial immutable geometry snapshot, not a final producer-freeze certificate.

The first actionable cases are actual prism intersections, not bounding boxes or convex hulls. GUARD-RM-1 part48 (MAT-RENDERED) and W-G-GARAGE part4 (MAT-BRICK) overlap0.006778931249999877m³, with strict point[10.898,10.062078125,0.716953125]. Across52 positive component pairs their additive diagnostic sum is0.4237114m³. GUARD-RM-2/W-PARENT-BATH has42 component pairs summing0.140027351572838m³. No shared-ownership exemption appears on the raw guard source; these values need physical ownership repair, not a renamed overlap exception. Group sums are expressly not unique spatial union volumes because guard components may themselves overlap.

Exact extracted source/target component geometry and witnesses for these two pairs are in [guard-wall-owned-cases-r92.json](guard-wall-owned-cases-r92.json). The broader initial actionable set is [whole-interference-positive-cases-r92.json](whole-interference-positive-cases-r92.json), currently730 component pairs above1e-6m³; this extraction is a review priority threshold, not an acceptance tolerance or deletion of smaller records. All smaller records remain in generated audit evidence.

Other source-owner review priorities: FIN-ROOF-PAVING/BEAM-PRIMARY-R-34-10 additive0.0175946914466m³; FIN-ROOF-GRAVEL/R-10-11 additive0.0148867778546m³; FIN-ROOF-GRAVEL/R-34-10 additive0.0144853667717m³; GARDEN-WEATHER-CAP/PAR-E additive0.1886345372619m³. Ground FIN-HALL/RF-1A additive0.0154059179811m³ and FIN-RAMP-INTERIOR/W-G-CHAUFFEUR-RETURN additive0.0114972m³ also need review.

The audit remains in progress, with numerical circle/mesh boundary cases retained unresolved. No complete collision-free claim is made. Controls and method are documented in [whole-owned-interference-execution-r92.md](whole-owned-interference-execution-r92.md).


## Completed Initial Pass

The initial pass completed in200.0005 seconds after capture. It checked25,259 fixture/finish parts against8,693 structural/wall parts. Of923,886 XY candidates,905,482 were strict-Z negatives and2,727 strict-XY negatives;15,677 received independent refinement. These produced11,835 exact-zero results,3,495 positive component candidates across118 product pairs, and347 unresolved cases (324 numerical-volume cases and23 open section graphs). No part construction was unsupported and no physical source product was omitted. This is not a collision-free result.

The retained positive-case input now includes the completed pass, not only the earlier partial batch. Additional non-guard priorities include ribbon frames RW-W/RW-E versus concrete posts, GL-RAMP-GARDEN/18-FIRST, SALON-CANOPY/PAR-E, CELLAR-COAL-CHUTE/FT-CELLAR, COLLECTOR-KITCHEN-PATIO-9/SL-1 and adjoining partitions, RAD-HALL-BEHIND-STAIR/RF-1A, D-ENTRY and CW-1 versus BEAM-PRIMARY-F-3-34, exterior light mounts/columns6 and11, and TUBE-ST-GF/L-column. Each remains a source-owner review candidate; no automatic acceptance/reclassification was applied.

Adaptive circle/mesh quadrature emitted numerical warnings at some tiny event intervals. Such cases require a sparse follow-up before any zero/positive certificate; warnings were not themselves interpreted as architectural defects. The23 open independent section graphs remain unresolved, not silently healed. The initial API-file hash change also requires the planned final frozen rerun.


## Independent Section Diagnostic

A sparse23-case replay identifies14 failures exactly at a source vertex Z, and six more within5.56e-16m of a source vertex. They arise in the independent horizontal graph at event/vertex rounding, not from demonstrated native shell nonclosure. Three light/column cases instead stop first at a quadrature warning when warnings become errors. These20+3 cases remain numerical follow-ups; no source shell defect is asserted. The source solids must not be healed or moved merely to satisfy this auditor. Executable whole-interference-section-diagnostic-r92.py and generated/whole-interference-section-diagnostic-r92.json retain exact identities and Z values.
