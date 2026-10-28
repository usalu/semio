# Audit r2: 📊️information (`Beat1_DataToInformation`)

Read-only audit. No source file was edited. Crops used for the checks are in the scratchpad, not in the repo.

Inputs read: `agent-rules.md`, `plan.md`, `storyboard-information.md`, `research-villa-savoye.md`, `research-ifc.md`; scene source `C/📊️information/🐍️.py` (10 keys, bilingual `NARRATION`); model `C/🧱️model/🔣️.json` (for basis, source and note fields); all 10 English keyframes and all 10 German keyframes in `T/🗑️generated/frames/Beat1_DataToInformation/`; six energy benchmark frames in `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/energy-frames/`; `C/📊️information/🐍️.py` structure compared with `⚡️energy/📊️demand/🎬️clip/⚡️demand-power/🐍️.py`.

Note on the stills: each keyframe is the state at the end of its beat. The `raw` still already shows the plan and the dimension, which is consistent with this, so start states are not captured and were not verified.

Coordinates below are approximate, in 1280 × 720 pixels, for the English still unless marked DE.

## 1. Verdict

The clip has moved from the bare cuboid to a real Villa Savoye ground plan, a piloti bay with a car and hatched clear width, a column elevation with hatch and load arrows, and a STEP/GlobalId handoff, and the German frames are clean with correct decimal commas. It is still well short of the owner's bar ("nothing abstract, every value drawn by its meaning"): two values are printed against explicit research rules (turning and glass-wall radii, piloti count ×25), one value has no drawn meaning (glass-wall radius), and the raw beat already draws the building. Against the energy clips it is roughly half the depth: no subtitle or standard reference, no typeset symbol formula, no leaders, no poché, scale or section conventions, and only spacing, car path and the reduced state actually move.

## 2. Per-beat table

| key | On screen (EN still; DE checked) | Narration (EN) | Defects |
|---|---|---|---|
| raw | Plan ghosted at 25 %, dimension "4.75" over E/3–E/4 (approx. 423–476 × 82–103), no unit. The glass wall renders as a solid light grey arch (approx. 270–425 × 200–440). Big number and unit guesses are transient, not in the still. | "This number is data. Alone, it does not say what was measured." | S-01, D-01 |
| context | Plan with E/3 and E/4 highlighted, dimension "4.75" without unit. Faint axon top right (approx. 760–1170 × 75–385) with two unlabelled lit piloti. List: IfcSite Poissy, IfcBuilding Villa Savoye, IfcBuildingStorey Ground floor ±0.00, Grid axes E/3 – E/4. North arrow. Top-right "Teaching model" note. | "Context: it is the distance between two piloti axes of the Villa Savoye." | A-05, D-01, D-03 |
| meaning | Right table "TEACHING DATA MODEL · quantity" (approx. 707–1221 × 117–387): Property Grid spacing, Value 4.75, Unit m, Grid E/3 – E/4, Basis sourced with sub-line "teaching-model basis, not an IFC property". Plan label "4.75 m" with "sourced" chip. Sources line. | "The data model adds meaning: property, unit, grid, and whether the value is sourced or assumed." | L-03, N-11 |
| question | Headline "Does it fit through?". Plan: dashed turning circle (5.50 m) around the glass-wall centre, cyan glass arc lit, "?" in the entry bay. Right: car top view 4.60 × 1.75 m; table rows Car width 1.75 m assumed, Turning radius 5.50 m assumed, Glass-wall radius 3.50 m assumed, sub-line "turning radius − car width − clearance". | "Information serves a question: can a 1929 car drive between the pilotis and turn under the house?" | N-01, A-01, A-06, N-10 |
| clearance | Bay detail (approx. 698–1230 × 63–580): E/3 and E/4 yellow discs, "Ø 0.30 m" in orange, hatched green clear band, 4.75 m span dimension, car 1.75 m, margins 1.35 m each. Unlabelled dimension line under the car (approx. 822–1132 × 478–492). Formula "4.75 m − 0.30 m = 4.45 m" and verdict "1.75 m car < 4.45 m → fits", both green. Plan bracket around E/3–E/4. | "The clear width is the spacing minus the piloti diameter. In this model, that is the room the car has." | N-03, N-07, A-02, L-02, D-04 |
| drive | Plan: yellow swept band from the E/3–E/4 gap, around the glass wall, to the car at its end pose under the house (approx. 224–266 × 425–527, grid 1–2 / A–B). Dashed turning circle. Bay car at 30 % opacity. | "The car drives through and turns under the house; the glass wall follows its turning circle." | S-04, L-08, D-02 |
| reduced | As drive; chip "reduced for cost" (approx. 700–895 × 63–92) across the right frame's top border. Spacing 4.75 at the end state, verdict fits. The 5.00 m state is not visible in the still. | "Even after the grid was reduced to save cost, the answer holds: the car still fits." | L-04, S-05, D-02 |
| properties | Column elevation (approx. 675–860 × 115–575): Ø 0.30 m section circle above the plan with a leader from E/3, hatch on shaft and section, slab stub, height dimension "3.25 m", three yellow load arrows, ground arrow. Table (approx. 865–1235 × 147–402): IfcColumn / PredefinedType COLUMN; Material Reinforced concrete (assumed); Load-bearing TRUE (sourced); External TRUE (assumed). Plan: E/3 ring; E/4 still a bright yellow dot. | "A piloti carries more properties: its material, whether it bears load, and whether it is external." | N-04, N-05, N-06, L-05, S-03 |
| relations | Plan becomes one uniform grey rectangle (storey tint plus slab fill, approx. 160–634 × 124–542). Column E/3 and grid lines E and 3 in DATA. Table (approx. 698–1230 × 133–388): Contained in Ground floor ±0.00; Defined by type Piloti × 25 (count assumed); Connected to First-floor slab +3.50 m assumed; Grid position "E × 3", sub "ObjectPlacement". Floating note "teaching-model count, sources disagree" (approx. 751–1177 × 422–442). | "Relationships complete the meaning: storey, type, the floor slab it carries and its grid position." | N-02, N-08, S-06, A-04, S-07, L-06 |
| handoff | Plan keeps the E/3 ring and the 4.75 dimension. Chips "People use the drawing" (approx. 822–1105 × 72–108) with an arrow ending at the plan's north-east corner (approx. 650, 152), and "Applications use the data". STEP excerpt (approx. 694–1234 × 207–350): #101 IFCCOLUMN with GlobalId 3Zc4b5MnPHUxLaBuFmsQ4K, 'Piloti E3', tag 'E3', .COLUMN.; #115 IFCPROPERTYSINGLEVALUE('LoadBearing'…). GlobalId chip (approx. 831–1097 × 395–460). "same piloti · same GlobalId". | "BIM keeps the data in one structured building model, so people and applications refer to the same piloti." | S-02, A-03, N-12, L-01, G-01, G-02 |

## 3. Defect list

Severity: blocker = breaks a hard rule of `agent-rules.md` or a research "do not state" rule; major = visibly wrong or misleading, or a clear depth gap; minor = polish.

### [numbers]

- **N-01 (blocker).** Turning radius 5.50 m and glass-wall radius 3.50 m are printed in the question beat's car table (approx. 707–1221 × 352–548, EN and DE). Research `research-villa-savoye.md` "Recommended teaching values": "Turning radius of curved wall: Not sourced; state the principle, not a radius." The chips say "assumed", but the rule is about stating a radius at all. The sub-line "turning radius − car width − clearance" hides the 0.25 m wall clearance (model `wall_clearance`, JSON line 1999), so the printed arithmetic 5.50 − 1.75 = 3.50 cannot be checked on screen.
- **N-02 (blocker).** "Piloti × 25" in the relations table (approx. 1052–1217 × 207–226) with chip "count assumed" and the note "teaching-model count, sources disagree". Research "Recommended teaching values": "Number of piloti: Do not state a count in the model; mark as open." The model note (JSON line 21) confirms the count is a teaching assumption; lex.dk gives 25 and a student slideshow gives 15.
- **N-03 (major).** Piloti diameter Ø 0.30 m is an assumption, and the verdict, clear width (4.45 m) and margins (1.35 m) depend on it. Research fact 2.12: "Not found"; the only, weak, source gives radius 30 cm, i.e. Ø 0.60 m. The model note (JSON line 28) says "no source gives them", which misstates that source. 0.30 m is also the old abstract WALL-01 thickness from the baseline, so it looks reused rather than chosen. At Ø 0.60 m the clear width is 4.15 m and the margin 1.20 m, so the verdict holds, but the clip never shows this.
- **N-04 (major).** Piloti height "3.25 m" (properties beat, approx. 675–752 × 422–445) has no basis chip. The model derives it from the first-floor elevation 3.50 m (assumed, JSON line 282) minus slab 0.25 m (assumed, line 820), see result line 2252. Research 2.6: storey heights not sourced.
- **N-05 (major).** "Load-bearing TRUE · sourced" (properties row 3; model JSON line 313, source VS-3.1). VS-3.1 supports the post-and-slab reinforced concrete system (research fact 3.1, high). Research-ifc §11 (IDS caution) and §15 item 5 say LoadBearing is per element, never a universal rule. The per-piloti value is an inference and should not carry "sourced".
- **N-06 (minor).** Material "Reinforced concrete · assumed" (WARN chip, properties row 2) contradicts research fact 3.1 (high confidence). It should be sourced, with "assumed" reserved for grade, cover and mix, which are not sourced anyway.
- **N-07 (major).** The clearance formula and verdict are green (GOOD) although two of three inputs are assumed (diameter, car width). The derived 4.45 m is then coloured as if sourced. The verdict "fits" carries no basis.
- **N-08 (minor).** Grid position "E × 3" uses a multiplication sign, while context and clearance use "E/3". Sub-line "ObjectPlacement" names the placement attribute, which is geometry, not a grid relation. IFC grids are IfcGrid and IfcGridAxis (verify against IFC 4.3.2.0; not in research).
- **N-09 (minor).** Unexplained overhang on the plan: grid columns 1–5 span 4 × 4.75 = 19.00 m, inside a 21.50 m box (fact 2.1). The frame shows about 1.25 m of box beyond grid 1 and grid 5 on each side, with no label and no source (research 3.4: cantilever lengths not found).
- **N-10 (minor).** "1929" is typed in the narration (`question` clause) and in the car note "1929 car · size assumed" (code), not read from the model. The model only holds the year in meaning text (JSON lines 49, 181, 186, 192).
- **N-11 (minor).** "Sources: EN Wikipedia · Cité de l'architecture" cites the Cité sheet, which the research read only as a search snippet. Cite the source actually read (EN Wikipedia raw, sites-le-corbusier.org) or mark it "snippet".
- **N-12 (minor).** The STEP excerpt shows `IFCPROPERTYSINGLEVALUE('LoadBearing'…)` without its `IfcPropertySet` ('Pset_ColumnCommon') and without `IfcRelDefinesByProperties`. The Pset membership taught in the properties beat is therefore not visible in the handoff.

### [abstract]

- **A-01 (blocker).** The glass-wall radius row (3.50 m) has no drawn radius. The dashed turning circle is drawn, but there is no centre point, no radius leader to the cyan arc and no offset arrow for "car width + clearance". The storyboard asked for an "R = … m" leader. Value without drawn meaning breaks the bar.
- **A-02 (major).** The clear-width dimension under the car (approx. 822–1132 × 478–492) has no label: the code removes its label (`clear_dim.remove(clear_dim[0])`). A dimension line without a number is an abstract line.
- **A-03 (major).** Handoff: the STEP tokens and the GlobalId chip have no drawn counterpart. There is no leader from the E/3 ring to the IFCCOLUMN token, the LoadBearing line or the GlobalId chip. The storyboard promised a DATA leader; the code only calls `Indicate` on the ring.
- **A-04 (major).** Relations rows are colour dots only. No row is drawn to its target: storey to the tint edge, type to the piloti set, "connected to" to a connector at the piloti head (IfcRelConnectsElements), grid to the axis lines. The storyboard promised leaders and "each relationship is a drawn link".
- **A-05 (minor).** Context axon (approx. 760–1170 × 75–385): no labels on the two lit piloti, no north mark, low contrast, so the axon cannot be matched to plan E/3–E/4.
- **A-06 (minor).** The car in the question and bay beats is a generic rounded rectangle with a dark cabin. It does not read as a 1929 car (no long bonnet, running boards, separate wings or spare). It is labelled "size assumed", which is honest but visually generic.

### [sync]

- **S-01 (major).** `raw` ends with the plan and the 4.75 dimension already on screen, while the narration says the number alone does not say what was measured, and the storyboard says nothing is drawn yet. The ghost plan also renders the glass wall as a solid light grey arch (approx. 270–425 × 200–440), unlike the outlined wall in `context`, so the building looks different before and after.
- **S-02 (major).** Handoff: the arrow from "People use the drawing" ends at the plan's north-east corner (approx. 650, 152), not at the E/3 ring (approx. 397, 124) or any piloti. In DE the same arrow ends at the same point. The arrow points at nothing.
- **S-03 (minor).** Properties: E/4 keeps a bright yellow dot (approx. 502, 124) after the beat. The code calls `set_opacity(0)` and `set_fill(CONCRETE)` on the E/4 parts, which does not show in the still, so the highlighted pair is asymmetric and unexplained. Verify the code path.
- **S-04 (minor).** Drive: the bay car stays at 30 % opacity after the plan car has passed the gap (approx. 915–1040 × 140–445), so the bay shows a car whose plan counterpart is far away.
- **S-05 (minor).** Reduced: the first grid at 5.00 m is only visible during the animation. The still shows only the end state, so the reduction is not visible in a paused view.
- **S-06 (major).** Relations: the storey tint and the slab fill cover the same rectangle at similar greys (approx. 160–634 × 124–542). "Contained in ground floor" and "connected to the floor slab it carries" cannot be told apart on screen.
- **S-07 (minor).** Relations, type row: the 25 pilotis light only through a transient `Indicate`. In the still they are plain grey dots, so the count lesson does not show when paused.

### [layout]

- **L-01 (major).** Handoff STEP block contrast (approx. 694–1234 × 207–350, EN and DE): the lines `#101=`, `'Piloti E3'`, `#115=` and `IFCBOOLEAN` are dark grey on the dark panel, far below readable contrast at 17 px. Only GlobalId, IFCCOLUMN and E3 read clearly.
- **L-02 (minor).** "Ø 0.30 m" label in the bay (approx. 703–800 × 310–330) sits 4–5 px from the panel's left frame line (x ≈ 698).
- **L-03 (minor).** The "sourced" chip in `meaning` (approx. 497–603 × 78–107) touches the right end tick of the 4.75 m dimension and sits on the E/4 ring.
- **L-04 (minor).** "reduced for cost" chip (approx. 700–895 × 63–92) straddles the top border of the right frame. The DE layout report flags it as overlapping a RoundedRectangle; the EN report says "none".
- **L-05 (minor).** Properties: the "Ø 0.30 m" dimension label sits directly on top of the three load arrows (arrows start at about y 245). The DE layout report flags a line through "Ø 0,30 m"; at zoom the dimension end tick touches the "m".
- **L-06 (minor).** The note "teaching-model count, sources disagree" (approx. 751–1177 × 422–442) floats below the relations table, away from the row it qualifies. Use the row's sub-line slot.
- **L-07 (minor).** The layout reports say "none" for most English beats, while the stills show L-02, L-03 and L-04. The overlap check does not test label-to-line, label-to-frame-border or chip-to-dimension-tick collisions, and does not flag dimension lines without a label.
- **L-08 (minor).** Drive: the swept band (DATA at 16 %) covers the E/3 ring and the bay bracket (approx. 383–516 × 97–150). Heavy, but readable.

### [depth]

- **D-01 (major).** No architectural drawing conventions on the plan. Piloti are light grey dots, not solid cut circles. Walls are outlines only, with no poché. There is no scale bar, no level mark (±0.00 triangle), no section cut A–A linking the plan to the column section, and no room or entrance labels (garage, hall, ramp). Research and storyboard call for poché, hatch, dimensions and scale.
- **D-02 (major).** Static values. Only the spacing, the car path and the reduced state move. Height 3.25 m, Ø, load arrows, slab elevation and the count are static. The rule "changing a value changes the drawing" is not met for these values.
- **D-03 (major).** No energy-clip chrome: no subtitle line under the title (energy: "Leistung ist eine Rate, Energie eine Menge"), no standard or schema reference top-right (energy: "DIN V 18599-2"). The top-right note is only "Teaching model · simplified, rounded".
- **D-04 (major).** Formulas are plain text ("4.75 m − 0.30 m = 4.45 m") with no symbols and no colour link to the drawn parts. Energy clips use typeset formulas with colour-coded symbols that tie to the drawing.
- **D-05 (minor).** No threshold gauge. The energy clips use a 0–1 scale with the operating point marked (F_sh, "ganz verschattet" to "ohne Verschattung"). The clip has no way to show how close the answer is to flipping.

### [de]

- **G-01 (minor).** DE handoff STEP name shows `Piloti-St\X2\00FC\X0\tze E3`. The ISO 10303-21 escape for "ü" is correct, but it is unexplained on screen and hard to read.
- **G-02 (minor).** The IFC Name is localised (EN "Piloti E3", DE "Piloti-Stütze E3") while the GlobalId stays. One IFC element has one Name; the clip teaches the opposite. Keep the Name identical in both languages and localise only the UI.
- **G-03 (minor).** Hyphen-minus in the DE sub-line "Wendekreis - Autobreite - Abstand" and in the EN "turning radius - car width - clearance", while the formulas use U+2212. Use one minus sign everywhere.
- **G-04 (minor).** Heading case mixes lowercase inside capitals: EN "TEACHING DATA MODEL · quantity", DE "LEHRDATENMODELL · Größe". Use one style.
- **Checked, no defect:** all 10 DE stills; decimal commas are correct throughout (4,75; 0,30; 1,75; 4,60; 5,50; 3,50; 1,35; 4,45; 3,25; +3,50; ±0,00). All DE captions fit in at most two lines, and no overflow was found. DE "Erdgeschoss ±0,00", "Rasterachsen", "belegt", "angenommen" and "passt" are correct.

Count: 3 blockers (N-01, N-02, A-01), 15 majors, the rest minor.

## 4. Fix proposals

### Numbers

- **N-01.** Remove the printed radii. Keep the dashed turning circle without a number. Add a double-arrow offset between the cyan arc and the dashed circle, labelled "car width + clearance (assumed)". Bind the label to `car_width` and `wall_clearance` from the model. If a number must appear, show the clearance value (0.25 m, assumed) in the sub-line so the arithmetic is complete.
- **N-02.** Replace "Piloti × 25" with the value "count open" (WARN) and sub-line "sources: 25 (lex.dk) vs 15 (slideshow)". Show all 25 grid candidates as dashed rings, not solid lit pilotis. Add a model field `count_status: "open"` and remove the count from the rendered text. Correct the model note at JSON line 21 to match research-villa "Open questions" item 6.
- **N-03.** Keep Ø 0.30 m as assumed, add a sensitivity line in the bay: "at Ø 0.60 m (one weak source): clear 4.15 m, margin 1.20 m, still fits", computed with `clearance(m, diameter=0.60)`. Animate the diameter with a ValueTracker in `reduced` so the hatch narrows. Correct the model note at JSON line 28 to name the weak source.
- **N-04.** Add a basis chip "derived · assumed" beside "3.25 m". Show the formula on the dimension sub-line: "3.50 − 0.25 = 3.25 m (assumed)". Take the values from `quantities(a)` as now.
- **N-05.** Change the LoadBearing basis in the model (JSON line 313) to "inferred" with source VS-3.1, and render an amber chip. Move the "sourced" chip to the system line "reinforced concrete post-and-slab (VS-3.1)".
- **N-06.** Set the material basis to "sourced" (VS-3.1, research 3.1), keeping "assumed" only for grade and cover.
- **N-07.** Colour the formula per symbol and per basis: spacing DATA with "sourced" chip; Ø WARN with "assumed"; car width WARN; result 4.45 m DATA (derived). Colour the verdict GOOD only when all inputs are sourced; otherwise WARN with the text "fits · assumed inputs".
- **N-08.** Use the `_lab` helper: "E/3" everywhere. Replace the sub-line with "IfcGridAxis E · IfcGridAxis 3" after verifying in IFC 4.3.2.0, or "grid axes E and 3 (IfcGrid, verify)". Drop "ObjectPlacement".
- **N-09.** Either draw the 19.00 m grid chain and label the 2.50 m remainder "not sourced: overhang or cantilever?" (see new beat NB-1), or leave the overhang without a dimension and mark it open.
- **N-10.** Put the year in the model as `car.model_year` with basis "assumed" and render it from there. The narration keeps "1929" as typed text; add a test that the narration year equals the model year.
- **N-11.** Replace the Cité entry with the source actually read (sites-le-corbusier.org, EN Wikipedia raw) for grid and curve, or add "(snippet)" to the Cité entry.
- **N-12.** Add the two excerpt lines `#114=IFCPROPERTYSET('…',…,'Pset_ColumnCommon',…)` and `#116=IFCRELDEFINESBYPROPERTIES(…,#114)` so LoadBearing belongs to its Pset. Let `step_excerpt` return them; keep the `_pick` needles.

### Abstract

- **A-01.** Draw the glass-wall radius: a DATA centre dot at `g["curtain_wall"]["centre"]`, a GLASS radius line to the arc midpoint, and a leader label. If N-01 removes the number, the label becomes the principle ("R = turning circle − car width − clearance") with the offset arrow.
- **A-02.** Label the clear-width dimension with `quantity(c["clear_width"], "m")` in GOOD. Remove the `clear_dim.remove(clear_dim[0])` line. Optionally repeat the value on the hatch.
- **A-03.** Add three DATA elbow leaders, faded in with the handoff chip: from the E/3 ring (approx. 397, 124) to the IFCCOLUMN token line, to the LoadBearing line, and to the GlobalId chip. Keep them as `always_redraw` if the ring moves.
- **A-04.** For each relations row, draw a leader from the row dot to its target: storey to the tint edge at the plan corner; type to the piloti set (light the candidates here); connected-to to a short CONCRETE connector at the E/3 piloti head, labelled IfcRelConnectsElements; grid to the E and 3 axis lines. Keep each leader on while its row is active.
- **A-05.** Label E/3 and E/4 in the axon and add a north tick on the axon base matching the plan's north arrow.
- **A-06.** Either draw a period silhouette from a model field `car.outline` (plan polygon: long bonnet, separate wings, running boards), or keep the rounded body and label it "generic body, size assumed".

### Sync

- **S-01.** Remove the ghost plan and the dimension from `raw`. Create the plan with its dimension at `context` only. Add a start-state capture for `raw` to the capture list, so the number-alone state is checked.
- **S-02.** Point the "People use the drawing" arrow at the E/3 ring (or at the plan's piloti set), with the tip 0.1 units short of the target. Point the "Applications use the data" arrow at the IFCCOLUMN token.
- **S-03.** Check the code path for `plan.parts[b]`. Either fade both E/4 parts to MUTED or keep both lit; the same style as E/3 in the still.
- **S-04.** Fade the bay car to 0 once the plan car is past the gap (`drive > gap_progress + 0.1`), not to 0.3.
- **S-05.** Keep a ghost of the old 5.00 m dimension (25 % opacity, muted tag "first grid") in the bay and fade it out at the end of `reduced`. The still then shows the change.
- **S-06.** Draw the storey tint only on the plan area outside the slab outline, or drop the tint. Draw the slab as CONCRETE hatch at the piloti heads in the plan and the full slab only in the column section.
- **S-07.** At the end of the type row, leave all 25 candidates in STRUCT (not a flash). The still then shows the count lesson.

### Layout

- **L-01.** Raise the STEP contrast: highlighted tokens full INK, all other tokens at least 60 % luminance, keeping the panel background. Check at least 4.5:1 for 17 px text.
- **L-02.** Move the Ø label right by about 0.15 units (or above the left piloti) so it clears the frame by at least 0.15 units.
- **L-03.** Move the "sourced" chip up by 0.3 units and right by 0.25 units, keeping 0.1 units clear of the dimension tick.
- **L-04.** Place the badge inside the right frame (y 2.95) instead of on the top border.
- **L-05.** Move the Ø dimension onto the section circle's equator, or lower the load arrows by 0.3 units, and keep 0.15 units between label and arrow tips.
- **L-06.** Use the row sub-line slot for the count note (see N-02), and delete the floating note.
- **L-07.** Extend the layout check with three tests: label-on-line, label-on-frame-border, chip-on-dimension-tick; and flag any dimension without a label.
- **L-08.** Draw the swept band as a 1.5 pt outline with 8 % fill, and remove the bay bracket once the car leaves the bay.

### Depth

- **D-01.** Plan conventions: pilotis as solid CONCRETE discs (drawn at 0.30 m diameter) with poché; walls as solid poché 0.2 m, with the glass wall kept as a GLASS outline; a scale bar 0–5 m bottom-left; a level triangle ±0.00 at the entrance; a section cut A–A through E/3 with a head bubble linked to the column section; keep the grid bubbles.
- **D-02.** Bind moving values with ValueTrackers and `always_redraw`: the diameter tracker (0.30 → 0.60, see N-03) in a sensitivity step; the height dimension bound to the first-floor elevation tracker; the count lights in step with the candidates (see S-07).
- **D-03.** A subtitle chip per beat under the title (EN "A number becomes information"; DE "Eine Zahl wird Information"). Top-right standard reference "IFC 4.3 · ISO 16739-1:2024" (research §1, tier P), plus the schema "IFC4X3_ADD2" as a muted chip. Do not claim ISO 19650 defines information (research §12).
- **D-04.** Typeset `s − Ø = w` with MathTex, colour per symbol (s DATA, Ø WARN, w GOOD), and a second line `4.75 − 0.30 = 4.45 m` with values from `quantity()`.
- **D-05.** Add a gauge (see NB-3).

Benchmark caveats from the energy frames: do not copy the overlap in `AnlagenVerluste-sum` ("30 m Rohr" over "Wärmeerzeuger"), and do not copy "Q=24600kWh" without the German number format. The useful energy patterns are the subtitle chip, the top-right norm chip, circle areas and bars proportional to value, the 0–1 gauge with threshold, and typeset formulas with colour-coded symbols.

### German

- **G-01.** Use an ASCII name in both languages ("Piloti E3") as in G-02, which removes the escape. If the escape is kept for teaching, add the micro-caption "\X2\00FC\X0\ = ü (ISO 10303-21)".
- **G-02.** One IFC Name per element, identical in EN and DE. Localise only UI text.
- **G-03.** One minus sign (U+2212) in sub-lines and formulas in both languages.
- **G-04.** One heading case per language; for example all-caps headings in both, with the unit word in caps as well.

### New beats (depth, using the villa)

- **NB-1 "Does the chain add up?"** (between `context` and `meaning`, about 6 s). Four grid bays on row E as a dimension chain: 4 × 4.75 = 19.00 m (sourced), against the 21.50 m box (fact 2.1, medium). The 2.50 m remainder shows as two unlabelled overhang strips, labelled "not sourced: cantilever or overhang?" in WARN, with the cantilever fact 3.4 as the open item. Teaching point: a sum over parts is information only when it meets the whole, and an open remainder is information too. Bind: `c["spacing"]`, `g["box"]["length"]`, a new model field `overhang` marked open.
- **NB-2 "Where did this value come from?"** (after `meaning`, about 6 s). Provenance card for the grid spacing: record VS-2.10 with its three sources (EN Wikipedia grid reduction; Cité snippet; Designing Buildings snippet), then a revision line 5.00 (first grid) → 4.75 (reduced for cost). The grid row on the plan lights as the revision lands. Teaching point: a value carries source, basis and revision, the same concept as the GlobalId in handoff. Bind to the model's `initial_spacing` and `spacing` with their source fields.
- **NB-3 "How sure is the answer?"** (after `reduced`, about 7 s). A horizontal gauge 0–5 m, in the same style as the energy F_sh gauge. Markers: car width 1.75 m (assumed); threshold spacing = car width + Ø = 2.05 m (below this the car cannot pass); current clear width 4.45 m; sensitivity at Ø 0.60 m (weak source) with threshold 2.35 m. Animate the Ø tracker 0.30 → 0.60 and the car width 1.75 → 2.00 m. Teaching point: the verdict flips only below the threshold, so the answer is robust to the assumed values. Bind to `clearance(m, spacing=…, diameter=…, car_width=…)`.
- **NB-4 (optional) "Which 4.75?"** (before `handoff`, about 5 s). A misreading test: spacing (centre to centre, 4.75 m) against clear width (4.45 m, edge to edge). Show both on the same bay with arrows, then ask which one the car needs. Teaching point: a value without its definition is ambiguous, and the IFC property name settles it.

## 5. Files

- Report: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/audit-r2-information.md`
- Crops (not in the repo): `C:/Users/Ueli/AppData/Local/Temp/claude/C--git-semio/c6859e80-281a-48bf-9c1f-0d7da30016af/scratchpad/crops/`
