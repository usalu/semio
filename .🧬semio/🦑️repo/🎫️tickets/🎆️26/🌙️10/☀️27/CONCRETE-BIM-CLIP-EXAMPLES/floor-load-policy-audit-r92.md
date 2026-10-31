# Physical Floor Weight and Analytical Allowance R92

The existing1.5kN/m² finishes entry is an explicitly assumed teaching allowance, not a measured or geometry-derived floor mass. Runtime analysis does not currently add the physical IfcCovering weights, so there is no present doublecount. Root will preserve this intentional analytical scope and clarify in English and German that the physical floor coverings are represented within the nominal allowance and are not additionally summed. The numerical difference below is evidence of distinct modelling scopes, not by itself a model error. No extra mass term is proposed for the current implementation.

## Independent nominal physical quantities

Stable sourcef81a2ea02f2f425933479b5cef8173441f478a86391b93b90498ac37a5170ef4; sparse calculation2.984seconds. APIaf51b1fb and analysis912e74cf remained unchanged. Each authored polygon/hole area was independently checked using shoelace arithmetic and Shapely. Part volume is area×height; weight is volume×the source material unitweight. The density fields are already in kN/m³, so no further9.81 factor applies. Every relevant unitweight remains assumed; these are exact nominal source-geometry quantities, not measured original1930 mass.

| Covering | Footprint m² | Volume m³ | Weight kN | Mean on own footprint kN/m² |
|---|---:|---:|---:|---:|
| FIN-GUEST-PARQUET | 15.900520 | 0.477016 | 3.450413 | 0.217000 |
| FIN-GUEST-TILE | 1.259480 | 0.037784 | 0.717904 | 0.570000 |
| FIN-TERRACE-PAVING | 93.732400 | 8.182827 | 159.949426 | 1.706448 |
| FIN-TERRACE-GRASS | 6.535775 | 0.566035 | 9.132408 | 1.397295 |
| FIN-KITCHEN-PATIO | 7.491620 | 0.224749 | 4.270223 | 0.570000 |
| FIN-SALON | 78.094070 | 2.342822 | 44.513620 | 0.570000 |
| FIN-KITCHEN | 14.978460 | 0.449354 | 8.537722 | 0.570000 |
| FIN-PANTRY | 9.450231 | 0.283507 | 5.386632 | 0.570000 |
| FIN-SON | 19.524250 | 0.585728 | 4.236762 | 0.217000 |
| FIN-SHARED-BATH | 5.795750 | 0.173872 | 3.303577 | 0.570000 |
| FIN-PARENTS | 24.461602 | 0.905098 | 8.048160 | 0.329012 |
| FIN-PARENT-BATH | 15.984284 | 0.479529 | 9.111042 | 0.570000 |
| FIN-BOUDOIR | 19.195680 | 1.319619 | 16.065447 | 0.836930 |
| FIN-FIRST-HALL | 32.454235 | 0.973627 | 18.498914 | 0.570000 |
| FIN-WC-FIRST-WEST | 1.033503 | 0.031005 | 0.589097 | 0.570000 |
| FIN-WC-FIRST-EAST | 1.109041 | 0.033271 | 0.632153 | 0.570000 |
| FIN-FIRST-THRESHOLDS | 35.777513 | 1.357264 | 23.862877 | 0.666980 |

Total17covers:18.423106210m³ and320.306376240kN. Their nonoverlapping union is382.778413923m², giving0.836793206kN/m² on covered area. The actual analytical SL1 structural projection is384.039406425m²; all physical floorweight is inside it, giving0.834045598kN/m² over that domain. Structural area without a floor-product footprint is1.260992502m²; this is not automatically missing floor and may include structural ownership. A subsequent sparse occupied-column check classifies1.260992501544m² as32 actual column reservations, including STAIR-RETURN-L’s.009599999984m². Remaining difference is2.77746×10⁻¹¹m² numerical residue with no component above10⁻⁹m². There is no genuinely walkable uncovered patch.

At1.5kN/m² the current nominal allowance contributes576.059109637kN over the structural domain,255.752733397kN above the nominal authored floor sum. That does not demonstrate a global error or a universal local safety margin: the terrace-paving product alone averages1.706447562kN/m² over its own footprint, whereas ordinary tiled floors average.57 and basic parquet.217. A single global mean does not reproduce spatially varying support reactions.

## Runtime evidence

In model/analysis/🐍️.py, _composite_weight lines252–265 uses structural regional depth and the RC/hollow-unit fraction only. _local_breakdown lines268–289 adds loads.finishes.first directly at lines275–277, then the imposed load. It does not iterate IfcCovering products. _zones lines317–347 distributes that local breakdown through slab structural regions; _mean_breakdown lines293–309 area-averages it. tributary lines361–413 multiplies the actual regional area by those entries and sums reactions. The currently evaluated mean breakdown is structuralselfweight3.362089645 +nominalfinishes1.5 +imposed2 =6.862089645kN/m².

The bilingual explanation at _assumptions lines433–444 currently labels these loads assumed. A clarification that actual floorproducts are represented within the nominal allowance, rather than added again, is consistent with the retained scope. link_values lines416–425 derives stored slab-load links from the runtime tributary result; final snapshots must use the actual final producer after source freeze.

## Breadth and acceptance

This table covers exactly17 first-storey FLOORING products, including terrace joints/paving, the kitchen patio, regional leveling, wearing and bedding parts. It does not quantify partition/wall mass, plaster, skirting, cabinetry, sanitary fixtures, services, stairs, roof floors or imposed loads. Density, timber split, leveling, bedding and unresolved paver thickness/datum remain qualified in the source. The generic1.5 allowance has no inspected per-component decomposition, so this audit does not infer which unmodelled nonfloor finishes it includes.

For a future geometry-derived mode, local physical weights would require intersection with actual load cells/regions and an explicitly separate residual allowance. Adding the full physical sum on top of the unchanged allowance would count represented floor finishes twice. This is a research alternative only; root has explicitly retained the current nominal teaching allowance. Portable input floor-load-policy-cases-r92.json records exact per-material/product expectations and this decision. No production edits.

## Final reservation and policy check

The second sparse run completed2.613seconds on stablea3cedfeff10a6fbfd49522d7fddfebaecc280df8b09c33dd57aa1cd4415d0dc9 after root’s bilingual finish-load-ownership clarification. All physical quantities remain identical. Actual occupied column geometry overlapping the3.14..3.17 floor band accounts for the entire apparent area difference to floating tolerance:32 column identities,1.260992501544m². No hull approximation or invented slab patch was used. The tiny residual is numerical, with no reportable polygon. Thus the floor-owner coverage result stands; no missing-floor issue was sent. The analytical allowance remains1.5 with no additional physical mass sum.

Subsequent scope change: the ramp owner added FIN-RAMP-INTERIOR and FIN-RAMP-EXTERIOR after this17-covering audit. Their weight follows the ramp load path and was not included in the horizontal SL1 floor-covering table. A future calculation must distinguish owning structural support from merely filtering storey plus FLOORING; the exterior ramp finish is not automatically a direct uniform load over SL1. The retained nominal allowance remains unchanged.

Raised-beam correction later removed actual overlapping finish material on cb9b138f. Consequently the earlier320.306kN table is an exact nominal quantity for its stated pre-repair snapshot, not a final current physical mass claim. A final nominal mass table should be recomputed after source freeze if required. The explicitly assumed1.5kN/m² analytical policy is preserved and is not additionally summed.
