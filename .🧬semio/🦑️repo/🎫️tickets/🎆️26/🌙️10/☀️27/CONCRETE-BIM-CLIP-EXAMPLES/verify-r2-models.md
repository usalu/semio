# Verify R2: Beat4_PartialModels (clip 🧩️models)

Read-only verification of Round 2. Scene `C/🧩️models/🐍️.py` (21:28), frames `🗑️generated/frames/Beat4_PartialModels/` (21:51 to 22:00, newer than the scene, no re-capture needed). All 19 EN and all 19 DE PNGs were read. Layout reports: 0 strict findings in 19/19 frames (EN and DE); non-strict items are listed below where they matter. Zoom crops in `🗑️generated/verify-models/` (check inset, federation leaders, schema headers, and others). Facts checked against `research-villa-savoye.md`, `research-ifc.md`, `analysis-model.md`, and the scene code.

## 1. Verdict

**FAIL.**

Round 2 fixed most of the R1 defects. The numbers now add up, the labels are bilingual, the overprints are gone, the federation leaders meet, and the loads beat has a real formula bound to a tracker. Two blockers remain against the owner's bar: the ground-floor plans never draw walls, and the analysis-model drawings are still an unlabelled generic FEM grid in three beats. There are also nine majors, including two visible arithmetic and count errors (the translation "24 IfcSlab" row and the loss table that does not sum to its headline).

Defect counts: **2 blocker, 9 major, 14 minor (25 total)**.

## 2. Arithmetic and facts that hold

- Architecture 5+4+4+1+2+4+2+6 = 28; structure 25+2+4+2 = 33; services 1+1+4+7 = 13; 28+33+13 = 74; 74 − 8 shared = 66 (EN and DE).
- Slab 21.50 × 19.00 = 408.50; 408.50 − 28.08 = 380.42 m².
- q = 6.25 + 1.50 + 2.00 = 9.75 kN/m²; 0.25 m × 25 kN/m³ = 6.25; 380.42 × 9.75 = 3,709 kN (3.709 kN in DE). The bar proportions match the values.
- Mid-surface 3.250 + 0.125 = 3.375 m (matches `analysis-model.md`: physical piloti 3.25 m, member 3.375 m to mid-surface).
- Clearance 0.18 m vs 0.25 m, short by 0.07 m. The inset geometry is consistent (gap about 0.18 m).
- Grid 1.25 + 4 × 4.75 + 1.25 = 21.50; 4 × 4.75 = 19.00; 7 × 5 = 35 nodes; 6 × 4 = 24 shells; 25 + 24 = 49 members; 40 + 26 = 66.
- Wall build-up 0.03 + 0.24 + 0.03 = 0.30 m; drawn layer widths are proportional.
- 25 curve members, 35 slab nodes, 25 support nodes: all match `analysis-model.md`.
- IFC names used: `IfcStructuralSurfaceMember`, `IfcStructuralPointConnection`, `IfcStructuralCurveMember`, `IfcStructuralAnalysisModel` (not on screen), `IfcSpace` (aggregated, IfcRelAggregates), `IfcRampFlight`, `Pset_ColumnCommon.LoadBearing`, `Pset_WallCommon.LoadBearing`, `Pset_ColumnCommon.FireRating`, `Pset_ColumnCommon.IsExternal`. All exist in IFC 4.3. See V-M5 for one misuse.
- Villa facts: 21.5 × 19 m (fact 2.1), roof garden 110 m² (2.7), solarium 70 m² (2.8), white exterior (7.1), ground-floor entrance, garage and hall (3.5), ribbon window disputed and labelled as a teaching choice, piloti count labelled as a teaching choice with "the sources disagree".

## 3. Defects

Severity: blocker = breaks the owner's bar or makes a drawing unreadable; major = a number, IFC name, fact or sync error that a reviewer would catch, or a bar rule only partly met; minor = polish or a missing label.

### Blockers

- **V-B1 (blocker) ground-floor plans draw no walls.** Keys `owners` (EN and DE) and `check` (EN and DE). The building is a dashed rectangle with dots; the only solid lines are the curved glass wall, the ramp and two rectangles. `owners` narrates "every line belongs to one model: walls and glass". There are no walls to belong to, so the bar's "never a bare box standing for a wall" is broken. Also no poché, so the clash and the owner colours do not sit on a recognisable plan.
  Fix: in `_owners` and `_check` (scene code), draw the ground-floor walls as poché polygons from the model's wall geometry (outer walls 0.30 m, the solid parts of the south side). Keep the curved glass wall, ramp outline, pilotis with the A1 highlight, and DP-1. Add the 1.25 / 4.75 chain (already present) and a scale bar.

- **V-B2 (blocker) analysis drawings are an unlabelled generic FEM grid.** Keys `translation` (right model), `loads` (left axon), `trace` (right model). These are isometric grids of rhombi with diamond supports. No villa outline, no grid labels A–E / 1–5, no ramp void, no piloti labels. The R1 defect D-A1 was fixed only for `members` and `supports` (plan views), not for these three 3D views. The bar forbids "an abstract grid standing for FEM".
  Fix: in `_analysis` (scene line about 644) and `_loads`/`_trace` (`analysis_view`), project the mesh onto the first-floor slab outline from `building_geometry()`: draw the 21.5 × 19 m outline, the ramp void cut, grid labels, and the 25 pilotis as labelled line members. Keep the nodes and shells on the same projection.

### Major

- **V-M1 (major) translation "24 IfcSlab" is the wrong source count.** Keys `translation` EN and DE. The row reads "24 IfcSlab → IfcStructuralSurfaceMember". The structure model has 2 `IfcSlab` (structure beat), and only one (SL-1, the first-floor slab) is translated. 24 is the shell count, not a source count. The row contradicts the translation narration and the `members` beat ("one slab becomes surface members").
  Fix: source row "1 IfcSlab" → target "24 shells" (or "IfcStructuralSurfaceMember ×24"). Scene `_translation` takes the count from `analysis["mapping"]`; change the mapping so that the source count is the number of source objects and the target count is the number of members.

- **V-M2 (major) loss table does not add up to its headline.** Keys `loss` EN and DE. The rows as printed are: window 4, opening 4, wall 4, **wall 4 (render)**, curtain 1, screen 1, ramp 2, flight 4, space 6, roof slab 1, **slab 1**, services 13. Sum 45. The headline says "40 objects are not in the analysis". Two rows are wrong: (a) the render row labels "IfcWall ×4", but render is a layer in the four façades, not a second set of walls; the architecture card has 5 walls in total, so the table implies 9. (b) the "IfcSlab ×1" row lists the first-floor slab SL-1 as dropped, but SL-1 is one of the 26 kept objects (`members` beat, `properties`, and `analysis-model.md`). Its text "Ramp void … not kept" describes a void, not an object.
  Fix: render row → "render layer · 4 façades" with no object count; slab row → "ramp void · 28.08 m²" with no object count. Then the visible rows sum to 40 without the duplicate. In `_loss` the head count for these two groups must not be `len(source_tags)`.

- **V-M3 (major) roof garden on the wrong level, area precision not sourced.** Key `rooms` EN and DE. The first-floor plan has "Roof garden 110.2 m²" (DE "Dachgarten 110,2 m²"). The services beat ("110 m² roof garden") and the roof plan ("Solarium 70.0 m²") put the roof garden on the roof. Research 2.7 gives 110 m² for the roof garden, not 110.2. Research 4.2 places the roof garden on the roof.
  Fix: move the 110 m² hatch to the roof plan as "Dachgarten 110 m²" (research value, rounded), and label the first-floor terrace as "Terrasse" with no roof-garden name. Use 70 m² for the solarium, not 70.0, if the research rounding is kept. Keep the first-floor and roof plans consistent with the services beat.

- **V-M4 (major) properties states unsourced values as kept facts.** Key `properties` EN and DE. "section Ø 0.30 m" and "reinforced concrete 25 kN/m³" appear in the kept list without "(assumed)". The `check` beat tags the same Ø 0.30 m as assumed, so the clip contradicts itself. Research 2.12 says the piloti diameter was not found and must be marked open; 25 kN/m³ is also an assumption (`loads` tags it).
  Fix: in `_properties` (scene about line 900), add the "assumed" tag to Ø and to the density, in both languages, or drop the diameter number from the kept list.

- **V-M5 (major) IFC misstatement on the support beat.** Key `supports` EN and DE. "IfcBoundaryNodeCondition = fixed" presents `fixed` as an attribute value. In IFC 4.3, `IfcBoundaryNodeCondition` holds the restraint or stiffness per degree of freedom (translation and rotation X, Y, Z) (research-ifc section 10). "fixed" is a teaching shorthand, not an IFC value.
  Fix: write "IfcBoundaryNodeCondition: translation and rotation restrained (assumed)" and keep "fixed" only in the teaching explanation. Keep the same wording in DE.

- **V-M6 (major) ramp not identifiable as a sloped element.** Key `architecture` EN and DE. In the axon the central ramp reads as a flat cream plane inside the curved glass wall. The card says "IfcRampFlight 4: their straight flights", but no flight is drawn with a slope. Research 3.5 gives no slope number, so drawing "gentle" flights is enough. R1 defect D-A4 is still open.
  Fix: draw the ramp as four straight flight slabs between ground and first floor (and the solarium link) with a visible incline. Do not label any slope number.

- **V-M7 (major) midsurface elevation reads as broken.** Keys `midsurface` EN and DE. The left elevation shows the roof as two floating hatched fragments above the first-floor slab, zig-zag lines for the ramp flights, and pilotis with no labels: no ground, first-floor or roof label. Nothing tells the viewer which line is the slab.
  Fix: draw the roof as one continuous slab on the walls (or omit it in the section), label the ground, first floor and piloti, and mark the first-floor slab as SL-1.

- **V-M8 (major) no scale bar and no wall conventions on any drawing.** Keys `owners`, `check`, `members`, `slab`, `midsurface`, `loads`. Dimensions are present (values right), but no drawing has a scale bar or a north/grid convention beyond the bubbles, and the plans have no walls (see V-B1). The energy benchmark (`Beat2_VolumeFlowEquation-formula`) has a scaled, dimensioned section. The plans fall below it.
  Fix: add a scale bar (1 m tick at the drawing scale) to every plan and section, and add the piloti spacing dimension on each plan.

- **V-M9 (major) trace counter has no drawn meaning and its kinds do not match the loss table.** Key `trace` EN and DE. "82 loss records in 13 kinds" appears as text with nothing drawn for the 82 records. On screen the loss table has 12 rows, so "13 kinds" needs the two pilotis FireRating and IsExternal rows that only appear on `properties`. The bar requires every value to be drawn by its meaning.
  Fix: draw the 82 records as small markers on the source elements of the loss table, and state the 13 kinds as "12 rows + 2 pilotis rows" (or count the rows the same way on screen).

### Minor

- **V-m1 storey heights unsourced.** Keys `rooms`, `midsurface`, `properties`: +3.50 m, +6.70 m and the 3.25 m piloti height are not in any source (research fact 2.6). Research 2.9 gives the roof garden about 3.5 m above ground, which conflicts with +6.70 m for the roof. Fix: tag the levels "teaching levels" on the rooms beat.

- **V-m2 services 110 m² not narrated.** Keys `services` EN and DE. "110 m² roof garden, drained by 4 roof drains" is on screen and not in the narration. Fix: narrate it or remove it from the caption-linked card.

- **V-m3 slab card "LoadBearing TRUE: sourced".** Key `slab`. No source gives a slab LoadBearing value (research-ifc only names the property). Fix: tag "teaching value" or "assumed".

- **V-m4 build-up thicknesses untagged.** Key `buildup` EN and DE. Render 0.03 m, brick 0.24 m, plaster 0.03 m; research 7.2 says the render specification was not found. The clip-wide "teaching model" banner covers this once, but the legend should say so. Fix: add "(teaching value)" to the legend title.

- **V-m5 gross outline called the built slab.** Keys `members` (EN "analysis slab is larger than the built one"), `slab`. Research 2.5 gives a 190 m² ground footprint and flags the 408 m² box as a conflict. Fix: cite the conflict in the `slab` card or drop "built one".

- **V-m6 square-bracket placeholders.** Keys `architecture`, `federation`, `check`, `slab`, `loss`, `buildup`: "Atelier [architecture]", "Atelier [Architektur]", "netto [schraffiert]", "[BCF]", "Render colour [white]" (EN and DE). Fix: remove the brackets.

- **V-m7 check inset label overlap.** Key `check` EN and DE. "0.18 m" sits on the outer orange ring (zoom crop en-check-inset). The plan dot DP-1 is partly under the A1 ring. Layout reports "line through '0.18 m'". Fix: move the label off the ring and the ring off the plan dot.

- **V-m8 structure chip and leader.** Key `structure`. The LoadBearing chip text runs edge to edge in its frame, and the leader does not visibly end on C3. Fix: shorten the chip text and end the leader on the C3 column.

- **V-m9 services drawing too faint.** Key `services` EN and DE. Boiler and radiator are a few red strokes; the radiator under the salon window is not recognisable. Fix: draw the radiator as a labelled element at a larger size.

- **V-m10 schema header touches the services thumbnail.** Key `schema`. "Structure model" text ends at about x 910 px; the services thumbnail starts at about 905 px (zoom crop en-schema-headers). Fix: move the structure header 0.1 units left.

- **V-m11 midsurface dimension label outside its callout.** Key `midsurface`. "0.125 m" runs past the right edge of the detail frame. Fix: move the label inside or extend the frame.

- **V-m12 no title on any frame.** The title ("Models Share a Schema" / "Modelle teilen ein Schema") is not drawn by any frame (the scene report says ClipScene.present no longer draws the title chip). Fix: draw the title chip in the scene, or confirm the clip is intended to be untitled.

- **V-m13 federation note not narrated.** Key `federation`. "Teaching model: both reference one slab. Real models often carry their own copies." is an unnarrated claim. Fix: narrate it or move it into the caption-linked card.

- **V-m14 unexplained dashed rectangle.** Keys `slab`, `members`, `supports`. A dashed rectangle sits at the right of the first-floor plan with no label. Fix: label it (ramp void edge, terrace, or the analysis-only extension) or remove it.

## 4. Round-1 audit status

| R1 id | status | note |
| --- | --- | --- |
| D-N1 class rows sum | resolved | all rows shown; 28 / 33 / 13 |
| D-N2 members 49 | resolved | 24 surface members, 49 in loss beat |
| D-N3 piloti count | resolved | "teaching choice", "sources disagree" |
| D-N4 A1 ×25 | resolved | "A1–E5 ×25" |
| D-N5 q composition | resolved | 6.25 + 1.50 + 2.00 with bar |
| D-N6 nodes and shells | resolved | 7 × 5 and 6 × 4 on plan |
| D-N7 slab thickness assumed | resolved | tagged assumed |
| D-N8 190 m² conflict | open | V-m5 |
| D-N9 leak cause | resolved | leak sentence removed |
| D-N10 piloti diameter | partly | clash tagged; properties not (V-M4) |
| D-A1 generic grid | open | members resolved; translation, loads, trace open (V-B2) |
| D-A2 check plan dot grid | open | still no walls (V-B1) |
| D-A3 services thumbnail | partly | downpipes visible; radiator weak (V-m9) |
| D-A4 ramp not identifiable | open | V-M6 |
| D-A5 leaders do not meet | resolved | both leaders end at one SL-1 point |
| D-A6 "hatched" word | resolved | removed |
| D-A7 author roles | resolved | teaching role; brackets remain (V-m6) |
| D-L1 members overprint | resolved | |
| D-L2 schema headers | resolved | headers beside thumbnails |
| D-L3 architecture tile | resolved | clear of caption |
| D-L4 loss table on villa | resolved | table starts right of the drawing |
| D-L5 translation clipped | resolved | |
| D-L6 LoadBearing chip | partly | V-m8 |
| D-L7 check dimension | partly | V-m7 |
| D-L8 services downpipe | resolved | |
| D-S1 render omission | resolved | "render: layer 1 of 3 in 4 walls" |
| D-S2 orange fade | resolved | lost parts stay orange |
| D-S3 services not narrated | open | V-m2 |
| D-S4 chip wording | resolved | "the analysis domain of the same schema" |
| D-S5 LoadBearing leader | partly | V-m8 |
| D-S6 C3 ring leader | resolved | ring removed from members |
| D-P1 energy depth | largely resolved | formulas and bars present; plans below (V-M8) |
| D-P2 drawing conventions | open | V-B1, V-M8 |
| D-P3 counters bound to drawing | resolved | lights follow counters on `owners` |
| D-P4 arrows follow q | resolved | `_loads` arrows use `q()` live |
| D-P5 list beats | resolved | |
| D-G1 author labels DE | resolved | |
| D-G2 rev term | resolved | "Rev. 1" in DE |
| D-G3 / D-G4 DE overprint | resolved | |
| D-G5 DE title | resolved in scene | title not drawn (V-m12) |
| D-G6 DE brackets | partly | "belegt" fixed, other brackets remain (V-m6) |
| D-G7 DE coverage | resolved | 19 of 19 DE frames |
| D-G8 "Lehrmodellzahl" | resolved | "Pilotis (Lehrentscheidung)" |

## 5. Not verified

- The 82 loss-record count and the 13 kinds were not recomputed from the model (not in `analysis-model.md`). Only the on-screen consistency was checked.
- Frames are 1280 × 720 renders; small labels were read at that size, with zoom crops for the check inset, federation, and schema headers only.
- The scene file was not run; the verdict is on the captured frames and the scene code as read.
