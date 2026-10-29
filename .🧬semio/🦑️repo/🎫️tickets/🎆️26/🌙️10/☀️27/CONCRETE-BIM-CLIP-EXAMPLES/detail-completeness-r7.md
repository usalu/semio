# Detail Completeness Audit R7

Read-only audit, 2026-10-28. Inspected the current authored model JSON, independent storeys-qa.png and previously rendered cmn-plans.png. The source snapshot has 164 elements and 19 spaces. This audit distinguishes absent geometry from geometry that merely has a descriptive name. It does not alter source or establish surveyed dimensions.

## Reference Key and Coordinate Convention

A: [CMN visitor leaflet 2021](https://www.villa-savoye.fr/content/download/9824393/file/document-visite-villa-savoye-FR-2021.pdf?inLanguage=fre-FR&version=3), PDF page index 1, numbered plans and corresponding numbered descriptions.
B: [CMN teaching dossier hosted by Fondation Le Corbusier](https://www.fondationlecorbusier.fr/wp-content/uploads/2022/05/2049_3993.pdf), PDF page indices 7–9 (printed pages 8–10).
C: [CMN suspended garden](https://www.villa-savoye.fr/decouvrir/un-jardin-suspendu).
D: [CMN architectural promenade](https://www.villa-savoye.fr/decouvrir/une-promenade-architecturale).

Positions use plan-left/right/top/bottom. Source coordinates have x right, y up; north points approximately (-x,-y). Proposed metric bounds below are existing source extents for locating work, not measurements of the monument.

## Primary Evidence Inventory

| Evidence | Source | Position |
|---|---|---|
| Two column tables and entrance basin; masters double door and separate service door | A §3 | Ground entrance/stair vicinity |
| Servants washbasins/radiators; twin laundry troughs; chauffeur living/bed/bath subdivision | A §§6–8 and ground plan | Left service strip; top-left laundry; top-right studio |
| Three echelon parking positions and rotating metal hoist to cellar | A §9 | Ground right garage |
| Guest wardrobe separates sleeping/toilet zones with zenith light; son's concrete desk; shared bath two doors | A §§12–14 | First left private strip |
| Pantry cupboards/pass-through and kitchen tiled counters | A §11 | First bottom-left |
| Parents bath/chaise, two-column bed alcove; boudoir shelf | A §§15–16 | First upper middle/right |
| Covered glazed kiosk, table, two planter-concealed garage skylights, grass paving joints | A §17 | First right terrace |
| Roof view aperture on ramp axis | A §18 | Roof bottom end of ramp |

Further B evidence: stair has a solid curved guard topped by metal tube (p8); low sliding storage and window shelves recur in bedrooms, the shared bathroom wall curves into its neighbour, the parents dressing passage has sliding cupboards and its bath has zenith light (p9); salon terrace glazing is full-height sliding, kitchen has a small terrace, and solarium alternates straight/curved windbreaks (p10). C independently confirms concrete paving with grass joints, planters, fixed table and semi-open shelter. D confirms an interior-to-exterior parallel-flight ramp and helical stair.

## Actionable Missing or Incomplete Geometry

1. **Chauffeur suite: missing topology, high priority.** SP-CHAUFFEUR is one five-point rectangular envelope (x7.3–15,y16–20.3) and W-G-CHAUFFEUR only its lower wall. No internal studio walls, doors, bath fixture or sleeping/living distinction exist. Trace the small upper-right CMN plan subdivision, including its diagonal boundary; preserve independent exterior access. Do not simply add a generic bed to the present open room.

2. **Garage shape and capacity: wrong/unproven, high priority.** W-G-E is straight and W-G-GARAGE-END diagonal; SP-GARAGE has no actual curved outside segment. The source name claiming three angled cars is not three parking bays. Reconcile the CMN curved outer envelope, internal angled separation and real usable parking geometry. Show three inferred vehicle envelopes only after clearance checks; do not assert historic car dimensions. No hoist/cellar opening is authored. If cellar is outside scope, model the evidenced hoist and explicitly identify omitted cellar access rather than fabricate an underground plan.

3. **Entrance built-ins: partially present.** FIX-HALL-BASIN is a solid block at basin height; there are no two column-integrated tables and no basin recess/stand. Place these from plan symbols without obstructing entry or stair. The curved glazing and actual clear double-door aperture now exist; retain those repairs. Check a separate service entrance against A rather than treating every door as interchangeable.

4. **Service rooms: missing equipment.** FIX-LAUNDRY has two plain rectangular parts, not hollow troughs. The two servant rooms have no individual washbasins or radiators. Add only these documented fixtures; boiler cupboard and heating pipe routes remain explicitly teaching assumptions.

5. **Guest/son/shared bath: incomplete.** ROOFLIGHT-1 exists but guest dressing wardrobe and wash area are absent. There is no son's desk, bedroom window storage/shelf or shared bath fixture/curved wall. Current rectangular private-room and corridor repairs provide circulation, but cannot be advertised as the detailed plan. Keep the two-door shared-bath logic and add the source-evidenced curve and fitments with independent clearances.

6. **Parents suite: descriptive primitives, insufficient.** FIX-BATH consists of two rectangular blocks; there is no concave bathtub, curved tiled chaise profile, basin fitment, dressing cupboards or parents zenith opening. ROOFLIGHT-1 is over guest dressing only. Check the actual two-column sleeping alcove rather than relying on unrelated upper grid columns. Keep dimensions inferred and preserve the accessible corridor repair.

7. **Pantry/kitchen: incomplete built-ins.** FIX-KITCHEN is two opaque rectangular counter solids, no sinks/counter surface distinction or pantry storage/pass-through. Add authored opening through the correct pantry wall, cupboards and hollow sink geometry; tile modulation can be a surface convention, not thousands of solids. Verify the kitchen's own small terrace against the plan before enlarging programme.

8. **Boudoir and bedroom storage: partial.** FIX-BOUDOIR supplies one shelf. Add the documented low cupboard/window ledge system where plan/photo evidence supports it; avoid generic loose furniture. Verify both boudoir views/doors against the small room's actual boundaries.

9. **Garden: absent architectural subtype.** SP-TERRACE combines open and covered garden into one undifferentiated L. Source has table, but no kiosk roof/columns/enclosure, two planters, garage rooflight openings or paving joints. Split semantic regions without overlap, create a real partial canopy and retain the larger open-to-sky void. Put planters over actual lightwells through SL-1, not decorative blocks on an intact slab. Table parts should read as tabletop/support, not a single slab-box.

10. **Stair safety/appearance: missing.** ST-GF/ST-FR have treads and arrival slab parts; the only IfcRailing objects are ramp guards. Add continuous helical solid guard plus upper tube for each stair, with terminal joins and clear arrival openings. Confirm top access rather than capping the stair hole for visual convenience.

11. **Roof screens/caps/frame: verify, not arbitrary enrichment.** SC-1 and SC-FRAME now have a clear aperture, but the independent plan shows simplified crescent/straight geometry. Compare continuous top profiles and end returns with the reference image. Current four full perimeter parapets and single guest rooflight cannot stand in for every roof-level object. The leaflet roof plan shows additional small cap/rooflight symbols; their precise function and height are not verified, so record them as unresolved instead of naming invented chimney types. Preserve ramp-frame alignment and sightline after every screen/guard change.

## Validation Requirements

Build a portable feature inventory listing tag, source paragraph/plan symbol, storey, semantic function and geometry. Tests should prove the new fixtures lie in their rooms, lightwells penetrate the correct slabs, the kiosk covers only its intended region, guards follow stair elevations, and circulation remains clear. Independent plan/section/axon checks must show the same authored objects. Existence counts and nonoverlapping room polygons alone do not demonstrate architectural completeness.

No missing item above authorizes unsourced exact dimensions, plumbing networks, structural reinforcements or generic furniture. The current model is a strong coarse topology reconstruction; the remaining work is substantive documented architectural geometry, not cosmetic object multiplication.
