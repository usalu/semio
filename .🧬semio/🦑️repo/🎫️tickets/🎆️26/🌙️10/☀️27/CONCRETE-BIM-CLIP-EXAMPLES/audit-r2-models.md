# Audit R2: Beat4_PartialModels (clip 🧩️models)

Read-only audit. Frames: `🗑️generated/frames/Beat4_PartialModels/` (11 EN keys, 5 DE keys: whole, architecture, structure, services, schema). Benchmark: `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/energy-frames/` (six frames). Model facts checked by running the read-only model module (`partial_model`, `translate_structure`, `clash_check`) and reading `C/🧱️model/🔣️.json` and `C/🧱️model/🧮️analysis/🐍️.py`. Scene source: `C/🧩️models/🐍️.py`.

## 1. Verdict

The clip has the right story skeleton (three discipline models, schema matrix, federation by GlobalId, clash, translation, assumptions, losses) and the numbers mostly trace to the model, but it is still well short of the bar. Roughly half the beats end on bare counters, a dot-grid plan, a generic FEM grid or a near-empty thumbnail rather than the villa, the value-to-drawing binding is missing for several values (q = 9.75 kN/m² has no composition, the 35 nodes and 24 shells have no explanation, the loss arrows fade to nothing), and the visible numbers contradict the model in four places (object totals, "26 members kept", "A1 ×25", "one surface member"). Several overlaps are unreadable (members beat, schema headers, DE architecture strip), the German version is untranslated in author labels and is missing six of eleven beats, and the depth is below the energy clips (no inline arithmetic, no poché/hatch/dimension conventions, no value animated against a drawn element except counters).

## 2. Per-beat table

Frames viewed: all 11 EN, all 5 DE. "EN/DE rendered" notes which exist.

| key | what is on screen (EN) | what the narration says | defects |
| --- | --- | --- | --- |
| whole | Villa axon in three layer colours at left; three rows Architecture 28 / Structure 33 / Services 13 with questions; "66 unique objects in three models"; header "Villa Savoye · Poissy · 1928–1931" and "Teaching model: simplified, rounded values". DE rendered. | "Different disciplines organize the building information they need." | [numbers] 28/33 totals are not the sum of any class rows shown later (see D-N1). [depth] Indicate pulses only; no leader from each question to its drawn layer. Acceptable otherwise. |
| architecture | Architecture model card rev 1, IFC4X3_ADD2, 28 objects; rows IfcWall 5, IfcWindow 4 "(all sides)", IfcCurtainWall 1, IfcRamp 2; note "All four sides: a teaching choice"; infill strip 0.03 render (white) / 0.24 brick / 0.03 plaster = 0.30 m, "(assumed)". Caption two lines. | "...walls, ribbon windows, ramp, curved glass wall and the wall build-up." | [numbers] rows sum to 12 of 28; IfcSpace ×6, IfcOpeningElement ×4, IfcRampFlight ×4, IfcSlab ×2 are missing; the card question asks "where are rooms" and no room appears. [layout] "IFC4X3_ADD2" chip and villa ground tile run into the caption band (EN frame, bottom-left, ground tile over caption top edge). DE: "Ausfachung: Schichten (angenommen)" is overprinted by "Außenputz (weiß)". [abstract] ramp cannot be identified as a sloped element in the frame. |
| structure | Structure card 33 objects; rows IfcColumn 25 "pilotis (teaching-model count)", IfcSlab 2, IfcRamp 2; "The piloti count is open in the sources."; chip "Pset_ColumnCommon.LoadBearing = TRUE (piloti C3)"; "Not in this model: IfcWall ×5 · IfcWindow ×4 · IfcCurtainWall ×1". Structure drawn full colour over a ghost of the architecture layer. | "...holds the pilotis, slabs and ramp. It leaves out render and windows." | [numbers] 25 shown as a count while the research says do not state a count (fact 2.11, teaching value table); 25 + 2 + 2 = 29 ≠ 33 (IfcRampFlight ×4 missing). [sync] "render" never appears on screen; the omitted list shows walls, not render. [depth] ghost and structure both teal/cream with overlapping outlines; the LoadBearing chip has no narration and the chip text fills its frame almost edge to edge. |
| services | Services card 13 objects; IfcBoiler 1, IfcSpaceHeater 1 "radiator under salon window", IfcWasteTerminal 4 "roof drains", IfcPipeSegment 7 "downpipes and heating pipes"; hatched roof garden with "110 m² roof garden, hatched"; leak note "The roof leaked every autumn [documented], so drainage matters." Teaching-scenario note. DE rendered. | "...boiler, radiators, roof drains and downpipes: the same house, other objects." | [numbers] counts are correct (1/1/4/7 = 13). [abstract] "hatched" is a drawing instruction printed as a value label. [sync] area 110 m² and the leak sentence are on screen but not narrated; the leak sentence implies a drainage cause that the research does not support (fact 8.1 says leaks, cause not stated). [de] "[documented]" / "[belegt]" brackets read as template markers. [layout] red downpipe runs to the caption band top edge (minor). |
| schema | Top bar "IFC4X3_ADD2 · one schema"; three thumbnails with headers "Architecture model · rev 1" etc.; class lists per model with counts (IfcWall 5 … IfcPipeSegment 7); green = shared class; footer "every object has a GlobalId (IfcRoot)". DE rendered. | "Three models, one schema: the same vocabulary and rules, with different content." | [layout] headers run through the roof apexes of the architecture and structure thumbnails and the red downpipe crosses "Services model · rev 1" (EN and DE). [abstract] services thumbnail is two red lines and a boiler symbol; the model is not recognisable. [depth] the class lists are three different column lists, not one matrix with all rows in all columns, so the "same vocabulary" lesson is not shown as a matrix. [de] "Revision 1" here vs "rev 1" on the cards; English "Architecture"/"Structure" headers fine. |
| federation | Three model cards (Atelier (architecture), Structural engineer, Heating engineer; rev 1); GlobalId chip "SL-1 0pGEEBTyDTuRFhzLBoa7lM"; two leaders from the slab to the chip; ±0.00 origin and grid labels 1–5, A–E; "Federated: rev 1 · rev 1 · rev 1". | "Coordination puts the models on one origin. One GlobalId marks the shared slab; each model keeps its author." | [sync/abstract] the architecture leader starts at the back roof edge (about 110 px above the tan dot), the structure leader at the slab dot; the two lines do not meet at one point, so "one GlobalId marks the shared slab" is not shown. [depth] the 'Teaching model: both reference one slab. Real models often carry their own copies.' note is an unnarrated claim. |
| check | Plan of dot grid 5×5 (pilotis) with dashed outline and grid labels; downpipe DP-1 and orange ring on A1; zoom inset with piloti A1, pipe DP-1, 0.18 m dimension, orange clearance ring; card "clash-01 · downpipe DP-1 ↔ piloti A1 / clearance 0.18 m < required 0.25 m · assumed rule"; two GlobalId boxes. | "In this teaching scenario, a clash check finds what no single model shows: a downpipe too close to a piloti." | [abstract] plan is dots in a dashed rectangle: no walls, no poché, no ramp, no curved glass wall, no room. [numbers] 0.18 m and 0.25 m match the model (distance 0.1808, required 0.25 assumed). Piloti diameter is drawn but the research says diameter is not found (fact 2.12); drawn size is an unlabelled teaching value. [layout] DP-1 red dot in plan is hidden under the orange A1 ring; the 0.18 m dimension crosses the leader. |
| translation | Left: structure axon faded, touching the left stage edge; header "Structure model · rev 1" over the roof apex; middle: "IfcColumn 25 → IfcStructuralCurveMember", "IfcSlab 1 → IfcStructuralSurfaceMember", both "modelling decision"; top chip "IFC4X3_ADD2 Structural Analysis Domain · example analysis schema"; right: analysis axon grid. | "For one use case, the structure model is translated into a structural analysis model: IFC's analysis domain." | [sync] narration says "analysis domain"; chip says "Structural Analysis Domain" as a schema name and the storyboard said "another schema"; research says the analysis domain is part of the same schema (IFC4X3_ADD2) — the chip wording should match. [layout] left drawing clipped at the left edge; header over roof apex. [abstract] right-hand analysis drawing is a plain grey grid with no villa outline and no labels. "modelling decision" printed twice without content. |
| members | Count rows: 25 IfcStructuralCurveMember "line members (pilotis)", 1 IfcStructuralSurfaceMember "one member: the slab", 24 "analysis shells / teaching mesh on the grid"; "Translation record: source GlobalId" chip "C3"; section of slab 0.25 m (assumed) with mid-surface; analysis axon with yellow ring at centre. | "The first-floor slab becomes one surface member, each piloti a line member, on the same grid." | [layout] blocker: "teaching mesh on the grid" and "Translation record: source GlobalId" are overprinted and unreadable (confirmed crop). [numbers] 24 shells are stored as 24 IfcStructuralSurfaceMember records in the model (members total 49), so "one surface member" contradicts the data; the scene derives "1" by counting source GUIDs. [abstract] the analysis grid has no labels, so "on the same grid" cannot be checked; the yellow C3 ring has no leader to its GlobalId chip. [abstract] slab section is a plain grey rectangle (no hatch/poché). |
| assumptions | Legend: nodes 35 on slab / 25 at piloti feet (yellow dot), reinforced concrete 25 kN/m³ (assumed), fixed (assumed) (yellow triangle), area load (tributary estimate, assumed) (green arrow); readout "q = 9.75 kN/m²"; analysis axon with cream nodes, hatched support rhombi, green load arrows. | "Assumptions add what was never drawn: nodes, material, supports and loads." | [numbers] 9.75 = 0.25 m × 25 kN/m³ (6.25) + finishes 1.5 + imposed 2.0, all assumed; the screen shows only the sum. 35 nodes = 7 x-lines (5 grid lines + slab edges 0 and 21.5 m) × 5 y-lines; 24 shells = 6 × 4 bays; none of this is on screen. [sync] legend triangle and yellow dot do not match drawn hatched rhombi and cream dots. [abstract] generic FEM-looking grid without villa outline, labels or ramp void: the case the bar forbids. [depth] arrows carry no scale; their length does not visibly change with q (the q animation shows only the readout). |
| loss | Villa axon ghosted at left (faded, almost empty); table of 12 rows in orange: RW-S ×4, OP-S ×4, W-S ×4 (infill), W-S ×4 (render [white]), CW-1, SC-1, RM-1 ×2, RF-1A ×4, SL-R, SL-1, A1 ×25, A1 ×25; "12 kinds of loss recorded", "26 members kept", "Translation is not automatically reversible." | "Ribbon windows, infill walls, render and the curved glass wall are not in the analysis. Record the losses." | [numbers] "A1 ×25" twice: all 25 pilotis are labelled with the tag of the first one (scene line 549 uses tags[0]); should read A1–E5 ×25 and the two rows should differ (fire rating, external flag). "26 members kept" conflicts with the model: 25 curve + 24 surface = 49 members. "[white]" bracket text. [sync] orange highlights fade to ghost before the end state, so no lost element is orange and the legend "Orange: not in the analysis" describes nothing visible. [layout] loss table starts on the villa's right edge (SL-1 / RF-1A labels touch the drawing). [abstract] no leader lines from rows to elements in the final frame. |

## 3. Defect list

Severity: blocker = breaks the quality bar or makes text unreadable; major = weakens a bar rule or a number is wrong or unexplained; minor = polish.

### Numbers

- D-N1 (major) [numbers] Class rows do not sum to the card totals. Architecture 28 = IfcWall 5, IfcOpeningElement 4, IfcWindow 4, IfcCurtainWall 1, IfcRamp 2, IfcRampFlight 4, IfcSlab 2, IfcSpace 6; the screen shows 12 of 28. Structure 33 = IfcColumn 25, IfcRamp 2, IfcRampFlight 4, IfcSlab 2; the screen shows 29. Services 13 is consistent.
- D-N2 (major) [numbers] "26 members kept" (loss beat) and "one member: the slab" (members beat) contradict the model: the analysis has 25 curve members and 24 surface members (one per shell), i.e. 49. The scene counts source GUIDs (`_surface_members()`) instead of members.
- D-N3 (major) [numbers] Piloti count 25 is stated as a count, while research fact 2.11 and the teaching-value table say "do not state a count; mark as open". The on-screen labels "(teaching-model count)" and "The piloti count is open in the sources" contradict each other.
- D-N4 (major) [numbers] "A1 ×25" in the loss table (two rows) should be a range "A1–E5 ×25" and the two rows need their own labels; the current text reads as if every piloti were called A1.
- D-N5 (major) [numbers] q = 9.75 kN/m² is shown without composition. Model: self-weight 0.25 m × 25 kN/m³ = 6.25, finishes 1.5, imposed 2.0, all assumed. Show a stacked bar or three-term sum.
- D-N6 (minor) [numbers] 35 slab nodes and 24 shells are shown with no explanation. Model: 7 x-lines (5 grid lines 1.25–20.25 m plus slab edges 0 and 21.5 m) × 5 y-lines; 6 × 4 bays. The narration says "on the same grid", which holds only for the interior lines.
- D-N7 (minor) [numbers] Slab thickness 0.25 m (assumed) is not in the research teaching-value table and the research has no slab thickness at all (fact 3.2). Keep the "(assumed)" label and add it to the assumption list in beat 10.
- D-N8 (minor) [numbers] Slab gross area in the model is 408.5 m² (21.5 × 19), net 380.42 m² after the ramp void. Research fact 2.5 says the ground-level footprint is about 190 m² and flags the conflict. If any area is shown, cite the conflict.
- D-N9 (minor) [numbers] Services leak note implies a drainage cause ("so drainage matters"). Research fact 8.1 says leaks each autumn, cause not stated.
- D-N10 (minor) [numbers] Drawn piloti diameter (about 0.3 m implied in the check inset) is unsourced: research fact 2.12 says diameter not found. Label the inset "teaching size" or drop the number.

### Abstract

- D-A1 (blocker) [abstract] The analysis views (assumptions, members, translation right) show a generic grid with pilotis as lines and no villa outline, no grid labels A–E / 1–5, no ramp void and no pilotis labels. This is the abstract FEM grid the bar forbids.
- D-A2 (blocker) [abstract] The check plan (beat 7) is a dot grid inside a dashed rectangle. No walls, poché, ramp, curved glass wall or room is drawn, so the downpipe-piloti clash is not located on the villa.
- D-A3 (major) [abstract] Services thumbnail in the schema beat is two red lines and a boiler icon. Not recognisable as the services layer of the villa.
- D-A4 (major) [abstract] Architecture beat: the ramp is not identifiable as a sloped element. The cream rectangle inside the curved wall reads as a floor plane. Check at full resolution and draw the ramp with its flights (IfcRampFlight ×4) as sloped shapes.
- D-A5 (major) [abstract] Federation: the architecture leader and structure leader do not meet on one slab point. Visible result contradicts "one GlobalId marks the shared slab".
- D-A6 (minor) [abstract] The "hatched" word in "110 m² roof garden, hatched" is a drawing instruction printed as a value label.
- D-A7 (minor) [abstract] Author names "Atelier (architecture)", "Structural engineer", "Heating engineer" are invented roles with no research basis. Mark them as teaching roles on first appearance.

### Layout

- D-L1 (blocker) [layout] Members beat: "teaching mesh on the grid" and "Translation record: source GlobalId" are overprinted; both unreadable at keyframe.
- D-L2 (major) [layout] Schema beat: column headers "Architecture model · rev 1" and "Structure model · rev 1" run through the roof apexes of their thumbnails; the red downpipe crosses "Services model · rev 1" (EN and DE).
- D-L3 (major) [layout] Architecture beat (EN frame, bottom left): the villa's ground tile runs into the caption box top edge. Keep the drawing above y ≈ -2.4 or the caption band clear.
- D-L4 (major) [layout] Loss beat: the loss table begins on the villa's right edge; SL-1 and RF-1A labels touch the drawing.
- D-L5 (major) [layout] Translation beat: left structure drawing touches the left stage edge; header overlaps the roof apex.
- D-L6 (minor) [layout] Structure beat: LoadBearing chip text fills its frame edge to edge; use a smaller size or a wider frame.
- D-L7 (minor) [layout] Check beat: 0.18 m dimension crosses the leader; DP-1 plan dot is hidden under the A1 ring.
- D-L8 (minor) [layout] Services beat: red downpipe reaches the caption band top edge.

### Sync

- D-S1 (major) [sync] Structure beat: narration says the structure model "leaves out render"; nothing on screen shows render being left out. The omitted list shows IfcWall ×5, which is not the same thing.
- D-S2 (major) [sync] Loss beat: the orange lost parts are faded to 18% before the end state, so the table and the legend "Orange: not in the analysis" point at nothing visible.
- D-S3 (minor) [sync] Services beat: 110 m² and the leak note are shown but not narrated. Either narrate or move out of the caption-linked frame.
- D-S4 (minor) [sync] Translation chip "IFC4X3_ADD2 Structural Analysis Domain" is labelled as a schema; the narration says "IFC's analysis domain". Align the chip text with the narration.
- D-S5 (minor) [sync] Structure beat LoadBearing chip (TRUE, piloti C3) has no narration and no leader is visible to the drawn C3 in the end frame.
- D-S6 (minor) [sync] Members beat: the yellow C3 ring has no visible leader to its "source GlobalId C3" chip.

### Depth

- D-P1 (blocker) [depth] Against the energy benchmark: energy beats show a formula with each factor coloured and bound to the drawing, inline arithmetic (400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W), a bar sized by value, and a drawn dimension (25 − 18 = 7 K across a duct section). This clip has no formula, no inline arithmetic, and no drawn dimension with a value that changes, except counters.
- D-P2 (major) [depth] No architectural drawing conventions: plans have no walls or poché, sections have no hatch, there is no scale bar and no dimension chain on the villa (ribbon window height, piloti spacing 4.75 m, slab thickness). Add these to the plan and section drawings.
- D-P3 (major) [depth] Counters run but nothing else moves in the drawing except a few pulses. Energy beats animate the drawn element that the value describes (bar fill, arrow length, growing circle). Bind the 25 pilotis, the 24 shells and the load arrow length to the counters the same way.
- D-P4 (major) [depth] The value "9.75 kN/m²" and the arrows are not tied: arrow lengths do not visibly change with q, and the composition is not drawn.
- D-P5 (minor) [depth] Eleven beats but most end on a list. Beats 5 (schema) and 9 (members) are lists beside small drawings.

### German

- D-G1 (major) [de] Author labels are untranslated in DE frames: "Atelier (architecture)", "Structural engineer", "Heating engineer". Model strings are English-only; add `{en, de}` author names.
- D-G2 (major) [de] DE structure, architecture and services cards show "rev 1" while the DE schema header shows "Revision 1". Use one term.
- D-G3 (major) [de] DE architecture beat: "Ausfachung: Schichten (angenommen)" is overprinted by "Außenputz (weiß) · 0,03 m".
- D-G4 (major) [de] DE schema beat: header "Architekturmodell · Revision 1" and "Haustechnikmodell · Revision 1" overprint the thumbnails (as EN).
- D-G5 (minor) [de] Title "Modelle Teilen Ein Schema" uses English title case; German: "Modelle teilen ein Schema".
- D-G6 (minor) [de] "[belegt]" and "[documented]" bracket text reads as template markers; drop the brackets.
- D-G7 (major) [de] DE rendering covers only 5 of 11 beats (whole, architecture, structure, services, schema). Federation, check, translation, members, assumptions and loss have no DE frames, so their overlaps and number formats are unchecked (e.g. "9,75 kN/m²", "0,25 m").
- D-G8 (minor) [de] "Pilotis (Lehrmodellzahl)" is acceptable; "Lehrmodellzahl" is a compound that reads oddly, "Pilotis (Lehrmodell)" is clearer.

## 4. Fix proposals

Each fix names the object to draw and the model value to bind. Model functions are the ones already in use.

### Numbers and sync

- Fix D-N1: Show every class of each model, or show "other classes ×N" as one row. Architecture adds IfcSpace 6 (rooms), IfcOpeningElement 4, IfcRampFlight 4, IfcSlab 2; structure adds IfcRampFlight 4. Bind to `partial_model(name)["counts_by_class"]`. Sum of rows must equal the card total (add an assertion in the scene).
- Fix D-N2: Count members from the model `members` list by `kind`, and show "25 curve members" and "24 shells of 1 surface member" (`translate_structure()["members"]` grouped by `source_guid`). Remove `_surface_members()` as a count of GUIDs. Narration "one surface member" then matches: one member with 24 shells as its mesh. The loss "members kept" reads `len(members)` = 49, or the shell-grouped count, consistently with the members beat.
- Fix D-N3: Replace the count "25" by "piloti: teaching choice, sources differ (15 to 25)". Narration unchanged. Remove the count-up tracker from the piloti row, or keep the count but label it "teaching choice" with the open-source line on the same card. Choose one; the labels must not contradict.
- Fix D-N4: Derive the label from the tags: "A1–E5 ×25" from the first and last tag of `source_tags`. Split the two loss rows by property: "fire rating (Pset_ColumnCommon.FireRating)" and "external flag (Pset_ColumnCommon.IsExternal)".
- Fix D-N5: In beat 10, show a stacked bar for q: self-weight 6.25 (0.25 m × 25 kN/m³), finishes 1.50, imposed 2.00, sum 9.75 kN/m². Bind to `analysis["loads"]["self_weight"|"finishes"|"imposed"]` and draw each segment with its label; the arrow length follows the sum.
- Fix D-N6: Add a small note under the node count: "7 × 5 nodes: 5 grid lines and the two slab edges; 6 × 4 shells". Bind to `analysis["mesh"]` or to `x_lines` and `y_lines` of the translation spec.
- Fix D-S1: Add an explicit "render" line to the omitted list (`W-* material.layer.render`, count 4) and say "render" in the narration only if the list shows it.
- Fix D-S2: Keep the lost elements in WARN (orange) at 35% opacity in the end state; show a leader from each loss row to its element (`villa.parts[...]`). Remove the "fade to 18%" step for orange parts.
- Fix D-S3, D-S4, D-S5, D-S6: narrate the services area ("110 square metres of roof garden, hatched") and the leak line or drop them; align the translation chip to "IFC's analysis domain"; add a leader from the LoadBearing chip to the C3 piloti and from the C3 chip to its ring.

### Abstract

- Fix D-A1: Replace the analysis grid with the villa's ground outline (21.5 × 19 m, from `building_geometry()["box"]`) and the first-floor slab outline in the mesh, with grid labels 1–5 / A–E and the ramp void drawn as a cut. Add piloti labels on the 25 curve members. Keep the 35 nodes and 24 shells visible as the teaching mesh.
- Fix D-A2: Replace the dot-grid check plan with the ground-floor plan from `building_geometry()`: walls as poché (thick filled polygons), the curved glass wall (`CW-1`) as a thin curve, the ramp outline, pilotis as circles with the A1 piloti highlighted, and DP-1 on the plan with the 0.18 m dimension from the check result. Keep the inset as it is.
- Fix D-A3: Draw the services thumbnail at a larger size or show the services layer over the villa's ghosted outline, so that the roof garden hatch, the drains (4) and the downpipes (7) are visible at thumbnail scale.
- Fix D-A4: Draw the ramp as its IfcRampFlight pieces with a visible slope (sloped slabs between levels), using the ramp geometry from `building_geometry()`; do not state a slope number (research fact 3.5, ramp numbers not sourced).
- Fix D-A5: Compute the two leader start points with `axon_point(..., explode=0)` for both disciplines and check that they coincide; if they do not, the layers use different offsets. Fix the offset so both leaders end on the same slab point.
- Fix D-A6: Replace "hatched" with the value only ("110 m² roof garden").
- Fix D-A7: Add `{en, de}` author names in the model and mark them "teaching role" in the card footer.

### Layout

- Fix D-L1: Move "Translation record: source GlobalId" chip to y ≈ 0.45 and the "teaching mesh on the grid" row to y ≈ 0.00 or shorten "analysis shells" to one line; keep the rows at 0.75 pitch. Re-run `…-layout.md`.
- Fix D-L2: Put the column headers at y ≈ 2.8 (above the thumbnails, under the schema bar) or raise the thumbnails by 0.3; move the services thumbnail's downpipes off the header column.
- Fix D-L3: Keep the architecture drawing's bottom at y ≥ -2.35 by lowering the scale to 0.17 or moving the origin up 0.2.
- Fix D-L4: Move the loss table to x ≥ 0.6 and shrink the villa to 0.12 scale at x = -4.9.
- Fix D-L5: Shift the translation left drawing to x = -4.1 and the header to y = 1.6.
- Fix D-L6, D-L7, D-L8: Shorten LoadBearing text to "LoadBearing = TRUE (C3)"; offset the 0.18 m dimension by 0.12 from the leader; keep the red downpipe above y = -2.3.

### German

- Fix D-G1: Author labels in `{en, de}` in the model (`partial_model` returns the localised value).
- Fix D-G2: Use "Revision 1" on the cards in DE and "rev 1" in EN; or "Rev. 1" in both. One term per language.
- Fix D-G3, D-G4: Move the DE strip title to y ≈ -1.05 and the layer labels to y ≈ -1.45 to -1.85, or shorten the title to "Ausfachung (angenommen)".
- Fix D-G5: "Modelle teilen ein Schema".
- Fix D-G6: Remove the brackets: "Das Dach war jeden Herbst undicht (belegt)" becomes "Das Dach war jeden Herbst undicht; das ist belegt." or drop "(belegt)".
- Fix D-G7: Render the six missing DE keys and check each overlap and number format.

### Depth

- Fix D-P1: Add one formula per beat in the energy style: "Lasten q = 0,25 m · 25 kN/m³ + 1,50 + 2,00 = 9,75 kN/m²" in the assumptions beat (DE: "9,75 kN/m²"), and the "clearance 0,18 m < 0,25 m" in the check beat. Bind each factor to its drawn element.
- Fix D-P2: Add poché to the plan (walls filled), hatch to the sections (slab section, wall section with the infill layers), a scale bar and dimension chain (4.75 m grid spacing between A1 and A2; 21.5 m length). Bind the grid spacing to `grid["spacing"]`.
- Fix D-P3: Make the piloti count and the shell count tracker-driven: a moving highlight that lights each piloti as the counter reaches it (the current `LaggedStart` is there but the end state hides it).
- Fix D-P4: Bind the arrow length to q (`load_arrows` with q), and show the arrow scale as a legend bar "1 kN/m² = x m".
- Fix D-P5: Merge beats 5 and 9 with a drawing that changes, or cut one of them.

### Proposals for new beats (conceptual depth)

1. Ground-floor plan cut with poché (new beat between check and translation). Draw the ground-floor plan of the villa from `building_geometry()`: the garage and entrance hall, the ramp, the curved glass wall following the turning circle (`clearance()["curve_radius"]`), the 25 pilotis on the 4.75 m grid. Bind the downpipe to the nearest piloti and the 4.75 m spacing dimension. Teaching: which model owns which plan element, and the car's swept path as the reason for the curve.
2. One slab, three answers (new beat after federation). The first-floor slab SL-1 as one GlobalId in three models: architecture (floor finish, 0.25 m slab), structure (surface member), services (the drain-free area). Show the slab section with the hatched solid and the mid-surface line. Bind the thickness 0.25 m (assumed) and the GlobalId. Teaching: same object, different representation.
3. Rooms and spaces (new beat after architecture). The six `IfcSpace` objects as hatched polygons on the first-floor plan (salon, terrace, bedrooms; room names only where the model has them), aggregated under the storey. This answers the architecture card's question "where are rooms" and fills the 28-object gap. Teaching: spaces are aggregated, not contained (research section 15.3).
4. Ramp and its flights (new beat after loss). Draw the central ramp as four IfcRampFlight pieces in section between ground and first floor, with the lost-in-analysis flights (RF-1A/B, RF-2A/B) highlighted and the ramp void in the first-floor slab (SL-1 voids, the void area 28.08 m² from `analysis["slab"]`). Teaching: the ramp is geometry, not a property; its slope is not sourced, so draw it as "gentle".
