# Production View Eligibility Audit

Read-only source24e6fd48355b (583 total /520 physical); production plan31f55b377717. No whole-building resolver or rendering run was needed. Executable production-view-filter-probes-r92.py extracts actual production predicates with AST; generated/production-view-filter-probes-r92.json records the admitted1.65s result.

## Confirmed omissions

1. `🎨️visuals/🏛️savoye/📋️plan/🐍️.py:108` rejects every FLOORING component with explicit role other than wearing. FIN-PILOTIS-GRAVEL uses pilotis_gravel_inferred_depth throughout: actual predicate accepts0/1652 cells. This rejects the whole visible wearing product, not merely hidden bedding. Comparison FIN-CELLAR-CEMENT40/40 and FIN-HALL82/155 retain intended wearing-layer behavior. Correct surface eligibility by native layer semantics, not a tag exception or suppressing the full gravel assembly. This remains relevant after root gravel datum move.

2. Same file:423 rejects nonstructural details/furnishings/railings when containment storey differs from requested plan. TUBE-STAIR-CENTRAL is architecture-only ground-owned, but actual64 hollow parts extend−2.40..6.34. Thus cellar cut−1.20 and first cut4.37 intersect real tube material yet cannot reach the drawing loop. It should be eligible by actual vertical band/cut, preserving its one containment identity. Roof6.60 is above its end and is not a missing-tube claim. No need duplicate/recontain tube per storey.

## Checked distinctions

Production `_plan_services` lines503–508 includes service parts overlapping the storey band; below-cut components project as thin footprints. Therefore diagnostic-atlas omissions do not demonstrate matching production service omission. `_plan_pilotis` lines321–333 explicitly retains parts crossing the cut and below-cut parts above storey datum when below=True. Its top<=FFL rejection of columns terminating beneath floor is normal hidden-structure policy, not missing source geometry. `_plan_solids` structural branch may retain same-storey buried beams when below=True.

Incoming stairs bypass the nonstructural storey equality gate. Ramp flight eligibility is by actual vertical interval, independent of parent containment, and loops every exclusive flight part. Ramp landing plan remains one authored outline, whereas axon now loops every actual owned landing component; no new missing solid established here.

Axon `_build` enumerates all physical canonical groups, including architectural details/ceilings/planters/gravel and services. `_service_prims` can bypass finite holes in its plain-prism branch, but current authored raw service parts supplied no actual holed nonmesh/nonsection witness; this is not recorded as a current defect. Section/elevation enumerate all canonical solid groups and services without the plan-only containment gate. Final native/profile/occlusion rendering still requires agreed producer/source freeze.

Fresh source-provenance mapping is source-coverage-current-r92.json/.md: all520 physical IDs assigned across40 topics, direct/type/topic evidence separated, no unmapped ID. This is a moving snapshot, not perfection or full archive coverage.

Nonmutating proposed source control confirms all1652 gravel components pass the unchanged production predicate when role=wearing and Z−0.19..−0.15 (1.74s admitted run); this is not a published-source validation. A direct-part inventory of nonstructural proxies/railings/furnishings/coverings found only TUBE-STAIR-CENTRAL crossing another storey cut; no additional cross-storey candidate was invented from bounds. See production-cross-storey-candidates-r92.json.

## Confirmed structural mesh elevation mismatch

`✂️section/🐍️.py:575` checks only top_outline before generic structural extrusion. Ordinary source mesh/section members instead become outline edges extruded between global component z0/z1. Architecture loop:556 and service loop:592 already use actual add_loft. BEAM-RAMP-FIRST-STRINGER contains7 actual convex source mesh cells and reaches this structural path. Independent projected unions of its actual face polygons prove false material in Y/Z side elevation: part5 actual0.903902027101m² versus fallback3.160143510336m²; false silhouette2.256241483235m² at strict candidate(Y9.3705,Z0.389394040882). Part6 actual1.06522 versus4.137731059591m²; false3.072511059591m². These are component diagnostics, not additive unique whole-silhouette areas. Some portions may be occluded by other products in final elevation; this does not correct the component geometry.

Executable production-elevation-mesh-probe-r92.py ran normally admitted3.15s and retained exact projected polygons under generated/production-elevation-mesh-probe-r92.json. It uses face-polygon union, no convex hull. Dynamic STAIR-RETURN-L end-plane mesh follows the same vulnerable dispatch but was not recaptured here; no additional numeric claim for that cap. Repair structural complex-shape eligibility generically and test a neutral oblique mesh/section plus actual stringer; preserve authored stock.
