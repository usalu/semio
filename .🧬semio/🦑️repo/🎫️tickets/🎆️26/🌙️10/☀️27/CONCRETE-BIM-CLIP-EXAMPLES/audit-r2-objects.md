# Audit r2: objects clip (Beat2_ObjectOrientation)

Scope: clip folder `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects`, scene `Beat2_ObjectOrientation`, 11 narration keys. Read-only audit; no source file was edited.

Evidence read:
- Frames: all 11 English PNGs and 6 German PNGs (schema, class, instance, occurrences, geometry, methods) in `T/🗑️generated/frames/Beat2_ObjectOrientation/`, plus `…-en-layout.md`.
- German total, inheritance, composition, distinction and exchange frames do not exist yet. German for those five beats was checked from the scene source only.
- Benchmark: six energy frames in `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/energy-frames/` and the energy ventilation volume clip source.
- Facts: `research-villa-savoye.md`, `research-ifc.md`, `storyboard-objects.md`, `agent-rules.md`.
- Source: `🧬️objects/🐍️.py` and `🎨️visuals/📐️drafting/🐍️.py` (View class, lines 199–223).
- Limit: `capture.py` replaces each step with `run_time=.01`, so every frame is the end state of its beat. Motion, ValueTracker behaviour and timing are not verified by the stills. Coordinates below are read from the 1280×720 images and are accurate to about ±5 px.

## 1. Verdict

The clip is far from the bar. The Villa Savoye appears in most beats, but a coordinate bug turns the south elevation of beats 9 and 10 into a single line, the class tree places IfcWall and IfcWindow under IfcColumn and IfcSlab, and 7 of 11 English frames have overlaps, clipped chips or unreadable labels. Several numbers contradict `research-villa-savoye.md` (25 pilotis stated as a value, 0.30 m diameter and 3.25 m height, unsourced storey levels, slab thickness and wall layers), and the total beat's arithmetic fails on screen (25 × 0.230 m³ is shown as 5.74 m³), while depth stays well below the energy clips (no substituted formulas, no value changes, no architectural drawing conventions beyond ground hatch and level marks). Roughly a third of the bar is met, so the two blockers (LAY-1, SYN-1) and the major list in section 3 need fixing before the clip is re-rendered.

## 2. Root cause found while auditing

`_south()` (scene lines 274–314) calls the drafting view with two arguments in most places, for example `view(-1.5, 0)`, `view(0, soffit)`, `view(L, top1)`, `view(op["start"], op["sill_z"])`. `View.__call__(self, x, y=None, z=0.0)` (drafting file lines 211–223) reads the second argument as `y`, so `z` is always 0. Every elevation point therefore lands on one baseline, which is the horizontal line at y ≈ 418 px in the composition, distinction and exchange frames. The wall band, ribbon opening, window, pilotis, roof screens, ground hatch, the hatch reveal in `_composition` (which reads the same collapsed `corners`) and the leaders in `_distinction` all collapse with it. `_total()` and `_detail_static()` use three-argument or direct coordinates and are not affected.

## 3. Per-beat table

| key | what is on screen (en frame) | what the narration says (en) | defects |
| --- | --- | --- | --- |
| schema | Card "IFC4X3_ADD2" at x 28–397, y 135–378 px with rows Vocabulary / Rules / Relationships as text only. Badge "Villa Savoye · Poissy · 1928–1931 / teaching model: simplified, rounded, partly assumed" at y 462–564. Solid axon of the villa at right (ribbon bands on all four faces, perimeter pilotis). Two-line caption. | "A schema defines vocabulary and rules for objects and relationships. For buildings, that schema is IFC 4.3." | ABS-2, DEP-4, NUM-9, LAY-15, LAY-16. Passes: schema id, ISO 16739-1:2024, teaching-model badge wording match research. |
| class | Left tree: IfcRoot → IfcObjectDefinition → IfcObject → IfcProduct → IfcElement → IfcBuiltElement (chain correct), then a 2×2 grid: IfcColumn (39–206, 442–476) and IfcSlab (219–386, 442–476), with IfcWall (39–206, 496–530) directly under IfcColumn and IfcWindow (219–386, 496–530) under IfcSlab. Fan lines from IfcBuiltElement cut through the IfcColumn and IfcSlab chips. Right: solid axon; walls and screens teal, slab pale, pilotis tan, windows teal-green. | "A class describes a kind of thing: IfcColumn, IfcSlab, IfcWall and IfcWindow are kinds of built element." | SYN-1 (blocker), ABS-1, LAY-4, LAY-16. |
| instance | Card "B3 : IfcColumn" (Name Piloti B3, Tag B3, Storey Ground floor) at y 153–288. GlobalId chip "0yoQlyTGDVUBsRS$cAaoWq" at y 296–361. B3 lit yellow with ring on a faint x-ray axon. Leader runs from x 345 to 917 across the building. | "An instance is one particular object: this piloti of the Villa Savoye, with its own GlobalId." | ABS-3 (storey value not drawn), LAY-17, DEP-4. GUID verified (22 characters, alphabet correct). |
| occurrences | Type card "IfcColumnType · Piloti" with Profile circle (assumed), Material reinforced concrete, Pset_ColumnCommon, LoadBearing TRUE (model). Arrow "IfcRelDefinesByType" to chip "B3 : IfcColumn". Counter "25 / 25 occurrences" with note "count: sources disagree, so it is a model value". Ground plan 5×5 grid, lit in order, B3 ringed; building outline only (no poché, no grid dimension). | "A type holds what all pilotis share; every piloti placed in the building is an occurrence of it." | NUM-1, ABS-3, DEP-1, DE-4. Grid labels A–E / 1–5 and B3 at column 3 row B match the model tag. |
| geometry | Table "Body · SweptSolid": Radius r (type) 0.15 m, Depth (piloti) 3.25 m, Ø = 2 · r 0.30 m, each "assumed". Elevation of one piloti with ground hatch, slab band, level marks "+3.50" and "±0.00". Enlarged section circle "section ×5" with r line and Ø dimension. Mini axon top right. | "The geometry holds values too: profile radius (shown as diameter) and extrusion depth, drawn as dimension lines." | NUM-2, NUM-3, LAY-3, LAY-14, DE-1, DEP-1 (section has no hatch), SYN-7. |
| methods | Badges "cross_section_area()" and "volume()". Formulas "A = π · r² = 0.071 m²" and "V = A · h = 0.230 m³" with no values inserted. Section disk and elevation both filled with concrete. Label "V = 0.230 m³" sits over "section ×5" and on the fill-level line. | "A method is behaviour: the application computes cross-section and volume from the geometry." | NUM-5, LAY-14, DEP-2, SYN-7. Arithmetic correct from unrounded values (0.0707 m², 0.2297 m³). |
| total | Text "all pilotis of the model", "25 × 0.230 m³ (assumed)", "= 5.74 m³" (large yellow). "first-floor slab 95.1 m³ (assumed)". Axon with first-floor slab ghosted over the roof, pilotis as pale hairlines. Two leaders from the numbers, one ending in the middle of the box, one at a corner of the building. No bars. | "The same method runs for every occurrence and adds up the concrete of all pilotis." | NUM-4 (5.75 vs 5.74), NUM-1, NUM-6, SYN-2, DEP-3, LAY-18. |
| inheritance | Spine of 7 chips (IfcColumn highlighted). Box IfcColumn with nine rows (GlobalId, OwnerHistory, Name, Description, ObjectType, ObjectPlacement, Representation, Tag, PredefinedType). Row labels almost invisible. Plan with "10.75 m" and "4.75 m" dimensions, five leaders fanning into B3. | "Inheritance: IfcColumn inherits GlobalId, name, placement, representation and tag, and adds PredefinedType." | LAY-2, LAY-7, LAY-8, SYN-5. Row order matches the IFC attribute order in research-ifc section 2. |
| composition | Left: card "IfcWall · W-S", chip "IfcOpeningElement · OP-S", chip "IfcWindow · RW-S", chip "IfcMaterialLayerSetUsage", card "IfcMaterialLayerSet" with Render (white) 0.03 m, Brick infill 0.24 m, Plaster 0.03 m, each with an "assumed" chip. Footnote "ribbon = one IfcWindow in one IfcOpeningElement". Right: readout "opening 20.4 m² of 64.5 m²" (assumed) and a section strip 0.03 / 0.24 / 0.03 with 0.30 m dimension, flat fills, no hatch. The elevation is absent: only a baseline at y ≈ 418 px. | "Composition means parts: this wall has an opening, and a window fills that opening." | LAY-1 (blocker), SYN-3, LAY-5, LAY-6, NUM-7, NUM-8, NUM-11, DEP-1. |
| distinction | Top left: IfcColumn → is-a → IfcBuiltElement (correct direction), sentence "A piloti is a built element", tick at x ≈ 405. Lower left: IfcWall —has-a— IfcWindow with text "IfcRelVoidsElement + IfcRelFillsElement", orange × above the link. Sentence "A window is not a wall: it fills an opening in it". Right: empty except the baseline and a long leader from IfcColumn (298,268) to a point on the baseline (886,418). No opening chip. | "Keep is-a and has-a apart: a piloti is a built element; a window is not a wall, it fills an opening." | LAY-1 (blocker), ABS-4, SYN-4. |
| exchange | Top: STEP panel with #101 IFCCOLUMN (GUID, $, 'Piloti B3', $, $, #102, #105, 'B3', .COLUMN.), #104, #106, #115, #116; heading "IFC4X3_ADD2 · STEP (ISO 10303-21)" on the panel border. Bottom left box "Authoring software": mini piloti filled, pill "volume()", "V = 0.230 m³", "assumed" chip, GUID. Bottom right box "Quantity take-off software": dim piloti, dim GUID, dim pill, value, chip. Centre: "IFC carries: class · values · relationships / never the code of volume() / Qto_ColumnBaseQuantities". A stray baseline at y ≈ 418 px crosses the centre text. | "IFC exchanges declarative data and relationships. The volume method stays in the application." | ABS-5, LAY-9, LAY-10, LAY-11, LAY-12, LAY-13, SYN-6, NUM-11. |

## 4. Defect list

Tags: [abstract], [numbers], [layout], [sync], [depth], [de]. Severity: blocker, major, minor.

### Blockers

- **LAY-1 [layout] blocker. South elevation collapses to one baseline.** Cause in section 2 (scene lines 285–312 call `view(x, z)`). Affects beats composition (wall, ribbon, window, pilotis, hatch reveal and "opening 20.4 m² of 64.5 m²" hatch all invisible) and distinction (right half empty, three leaders end on the baseline). The same baseline remains in the exchange frame.
- **SYN-1 [sync] blocker. Class tree says IfcWall is a kind of IfcColumn and IfcWindow a kind of IfcSlab.** IfcWall and IfcWindow are placed directly below IfcColumn and IfcSlab in a 2×2 grid, and the fan lines from IfcBuiltElement cross the IfcColumn and IfcSlab chips. The narration says all four are kinds of built element. Layout report: "line through" the IfcColumn chip.

### Majors

- **ABS-1 [abstract] major. A kind is shown only by colour, and the colours are not separable.** IfcColumn (STRUCT) and IfcSlab (CONCRETE) are both beige in the class frame. IfcWall (ARCH) and IfcWindow (WINDOW = mix of GLASS and GOOD) are both teal. Nothing counts or names the lit parts.
- **ABS-2 [abstract] major. Schema rules and relationships are text only.** "every IfcRoot object has a GlobalId" and "IfcRelVoidsElement · IfcRelFillsElement" are not drawn on any part. The flashes described in the storyboard are not visible in the end state.
- **ABS-3 [abstract] major. Relation and property values with no drawn meaning.** IfcRelDefinesByType is one arrow between two cards, not a relation to the 25 occurrences. "LoadBearing: TRUE (model)" has no load path or arrow anywhere. "Storey Ground floor" in the instance card is not shown on the drawing.
- **ABS-4 [abstract] major. Has-a diagram omits the opening.** The distinction beat shows IfcWall —has-a— IfcWindow with no IfcOpeningElement, although the narration and the composition beat say the window fills the opening. The "is-a?" arrow fades before the cross is drawn, so the orange × stands alone above "has-a".
- **ABS-5 [abstract] major. STEP tokens are not tied to the drawn piloti.** GUID, IFCCOLUMN and 'B3' have no leader or colour link to the piloti in either app box. The caption promises relationships, but the excerpt has no IFCRELDEFINESBYTYPE, IFCRELDEFINESBYPROPERTIES or IFCPROPERTYSET line, so IFCPROPERTYSINGLEVALUE('LoadBearing') and ('IsExternal') float without their Pset_ColumnCommon.
- **NUM-1 [numbers] major. The count 25 is the headline.** The big "25 / 25" counter, "25 × 0.230 m³" and the 5×5 grid all state 25. `research-villa-savoye.md` (recommended teaching values, and fact 2.11) says: "Do not state a count in the model; mark as open" (lex.dk says 25, a student slide says 15). The note "count: sources disagree, so it is a model value" is correct but does not remove the headline number.
- **NUM-2 [numbers] major. Piloti dimensions contradict the research.** Radius 0.15 m (Ø 0.30 m) and depth 3.25 m are drawn as dimensions. Fact 2.12 says the diameter is "not found, do not state, mark as open". The only cited value, an unverified student slide, gives radius 30 cm and height 2.87 m, so the model contradicts its own cited source on both numbers. The "assumed" chips reduce this but do not meet the research rule.
- **NUM-3 [numbers] major. Storey levels stated as numbers.** "+3.50" (first floor, from `storeys["first"]`) has no basis chip. Research 2.6 and the recommended values say storey heights are not sourced and should be used "without numbers". The +3.50 also equals 3.25 m plus an unsourced slab thickness.
- **NUM-4 [numbers] major. Visible arithmetic error.** The total beat shows "25 × 0.230 m³ = 5.74 m³". 25 × 0.230 = 5.75. The unrounded product (0.22973 × 25 = 5.743) gives 5.74, but a reader multiplies the numbers that are on screen. German shows "25 × 0,230 m³ = 5,74 m³", with the same problem.
- **NUM-5 [numbers, depth] major. Formulas are not substituted.** "A = π · r² = 0.071 m²" does not show r, and "V = A · h = 0.230 m³" shows neither A nor h. The energy benchmark substitutes every term ("400 · 1,92 · 0,70 · 0,60 · 0,80 ≈ 258 W"), and the audit standard here is that every number can be checked from the screen.
- **NUM-6 [numbers] major. Slab volume 95.1 m³ cannot be checked.** It depends on the first-floor slab thickness, which fact 3.2 says was not found. Nothing on screen shows area or thickness. It is labelled "assumed" only.
- **NUM-7 [numbers] major. Wall layer set is invented.** Render 0.03 m, Brick infill 0.24 m, Plaster 0.03 m are not in the research: fact 7.2 says the render specification was not found, and fact 3.1 says only "brick or plaster" with no thickness. "Render (white)" names a finish that is not sourced.
- **LAY-2 [layout] major. Inheritance row labels are nearly invisible.** The nine labels at x ≈ 300–460 px, y ≈ 232–448 px are dark grey on near-black. Only the values are readable.
- **LAY-3 [layout] major. Property-table labels are too small (both languages).** "Radius r (type)", "Depth (piloti)" and "Ø = 2 · r" at x ≈ 39–125 px, y ≈ 220 / 252 / 282 px render at about a third of the value size, roughly 9 px high.
- **LAY-5 [layout] major. Assumed chips sit on the layer values.** In composition the chips at x ≈ 258–366 px, y ≈ 502 / 529 / 556 px overlap "0.03 m" and "0.24 m" (layout report: "Render (white) 0.03 m × assumed"; "Plaster 0.03 m × assumed").
- **LAY-6 [layout] major. Footnote collides with the layer card and enters the caption zone.** "ribbon = one IfcWindow / in one IfcOpeningElement" at x ≈ 90–335, y ≈ 557–598 px overlaps the "Plaster 0.03 m" row and reaches the caption band (top ≈ 603 px). Layout report: "in caption zone".
- **LAY-7 [layout] major. Plan dimensions sit on pilotis and the building outline.** "10.75 m" (x ≈ 927–997, y ≈ 428–446 px) is printed on grid circle B2 at (972, 450). "4.75 m" (x ≈ 980–1046, y ≈ 480–498 px) sits on the garage outline.
- **LAY-8 [layout] major. Five leaders fan into one piloti.** The leaders from the value column (x ≈ 694 px) cross each other and the plan labels, and one crosses the grid bubble C (layout report: "line through 'C'").
- **LAY-9 [layout] major. STEP heading straddles the panel border.** "IFC4X3_ADD2 · STEP (ISO 10303-21)" at x ≈ 80–434, y ≈ 124–142 px sits across the panel border at y ≈ 127 and touches line #101 (layout report: overlap 3.77 × 0.07 units).
- **LAY-10 [layout] major. STEP excerpt is hard to read.** Lines #104, #106, #115 and #116 (x ≈ 95–1000, y ≈ 182–271 px) are dim grey on black. Only the GUID, class and .COLUMN. tokens stand out.
- **LAY-11 [layout] major. Exchange boxes overflow.** The left GUID ends exactly on the box border at x = 428 px. The right "assumed" chip (x ≈ 1152–1260 px) runs past the right box border at x ≈ 1247 px. The left "assumed" chip (x ≈ 335–440 px) overlaps the superscript of "V = 0.230 m³".
- **LAY-12 [layout] major. Stray baseline in exchange.** A horizontal line at y ≈ 418 px (x ≈ 638–1137 px) crosses the "IFC carries:" block and runs into the right box. It is the collapsed south elevation from LAY-1. Check which group still holds it: `_exchange` fades `s.south` in `old`, so the cause should be verified after the LAY-1 fix.
- **LAY-13 [layout, verify] major. Application B looks dimmed in the end state.** Its mini piloti, GUID (x ≈ 1008–1247, y ≈ 530–550 px), volume() pill and V value are faded, but the narration says B computes the same V. Verify on a non-skipped render. If the dimming is real, the recomputation the narration describes is not visible.
- **SYN-2 [sync] major. Total leaders point at nothing specific.** The "= 5.74 m³" leader ends at the middle of the box outline, not at a piloti. The slab leader ends at a corner of the outline. The 95.1 m³ slab value is not narrated.
- **SYN-3 [sync] major. Composition narration omits the layer set that is on screen.** The English narration names the opening and window only. The IfcMaterialLayerSet, IfcMaterialLayerSetUsage, "associated, not a part" and the 0.30 m strip are all shown. The storyboard narration included "a material layer set".
- **SYN-4 [sync] major. Distinction leaders have no visible target.** The ties to the piloti, band and window end on the baseline (consequence of LAY-1), so "a piloti is a built element" points at nothing.
- **DEP-1 [depth] major. No architectural drawing conventions.** The plan has no poché (walls are outlines only). The section disk and the section strip use flat fills, not hatch. There is no scale bar, no north arrow, no 4.75 m grid dimension string and no section cut mark on the plan. Storyboard section 0 and the owner's bar both require these conventions.
- **DEP-2 [depth] major. No value change in the clip.** Every number is static after its beat. Nothing shows that changing a value (diameter, height) changes the section, the volume and the total, which is the owner's rule. The energy clips change an input and show the consequence.
- **DEP-3 [depth] major. No comparison bars in the total beat.** The storyboard promised bars for the pilotis sum against the slab volume on one scale. The clip shows two text numbers instead.
- **DEP-4 [depth] major. Villa is decorative in four beats and styled inconsistently.** In schema, class, distinction and exchange the villa is decoration. Styles alternate between solid axon, x-ray (instance) and ghosted slab (total). The ground-floor set-back and its dark green are not visible at axon scale.
- **DE-1 [de] major. Hard-coded English number.** `"±0.00"` (scene line 243) is a literal, so German frames show "±0.00" where German needs "±0,00". Other German numbers are correct (0,15 m, 3,25 m, Ø 0,30 m, 0,071 m², 0,230 m³, +3,50).
- **DE-2 [de] major. Five German frames are not rendered.** Total, inheritance, composition, distinction and exchange have no German PNG, so overflow cannot be checked. Risk strings found in the source: "Mengenermittlungssoftware" (one long word in the 4.4-unit exchange box), "Planungssoftware", the German composition card "Bandfenster = ein IfcWindow in einer IfcOpeningElement" (3.6-unit card), and the German footnote, which collides with the layer rows just like the English one (LAY-6).
- **LAY-14 [layout] minor. Labels touch.** "+3.50" (x ≈ 682–724, y ≈ 255–272 px) overlaps "section ×5" (x ≈ 712–812, y ≈ 270–287 px) by 0.16 × 0.04 units. In methods "V = 0.230 m³" (x ≈ 622–766, y ≈ 286–307 px) sits on the section label and on the yellow fill-level line.
- **LAY-15 [layout] minor. Badge text fills its frame.** "Villa Savoye · Poissy · 1928–1931" spans x ≈ 33–395 px inside a frame at x ≈ 28–397 px. A longer name would clip.
- **LAY-16 [layout] minor. Capital I and lowercase l look the same.** In the class and schema frames "IfcRoot" reads "lfcRoot" and "IfcColumn" reads "lfcColumn". IFC names must be exact. Check at 2× zoom, since the font belongs to the shared energy toolkit.
- **LAY-17 [layout] minor. Instance leader crosses the building.** The GUID leader runs about 570 px (x 345 → 917, y 362 → 440) across the x-ray villa.
- **LAY-18 [layout] minor. Total fill reads as hairlines.** The 25 pilotis fill as 5-pt strokes over a translucent slab and an outlined roof, so the concrete is hard to read.
- **SYN-5 [sync] minor. Inheritance narration omits the empty rows.** The narration names five inherited attributes plus PredefinedType. The screen also shows OwnerHistory "not in model", Description and ObjectType "not set", and the narration says nothing about them.
- **SYN-6 [sync] minor. Blocked-arrow cross is in the wrong place.** The cross sits beside the A pill at x ≈ 388 px, not on the arrow the narration describes as blocked from A to B.
- **SYN-7 [sync] minor. Term drift.** "Depth (piloti)" (table), "height" (code and readout), "h" (formula) and "3.25 m" (dimension) all name one value.
- **NUM-8 [numbers] minor. Opening size is unsourced and not dimensioned.** "opening 20.4 m² of 64.5 m²" is labelled assumed. Research 4.5 says ribbon size was not found. 64.5 m² implies a 3.0 m high, 21.5 m long wall, and no dimension lines on the elevation show it.
- **NUM-9 [numbers] minor. Ribbons on all faces.** The axon draws ribbon bands on all four façades (schema, class, instance). Research 4.4 and 10.5 say the ribbon is on the south façade (FR Wikipedia) and that its extent is disputed. The composition beat calls the drawn wall "south façade · teaching orientation".
- **NUM-10 [numbers] minor. IsExternal TRUE on pilotis.** Not in the research. A piloti under the building is arguably exposed, but the value needs a teaching-value label or removal from the excerpt.
- **NUM-11 [numbers] minor. [M] IFC claims shown as fact.** IfcColumn and IfcWindow direct supertype IfcBuiltElement are tagged [M] in research-ifc. Qto_ColumnBaseQuantities and IfcMaterialLayerSetUsage on a wall are also [M]. Confirm against 4.3.2.0 before the final render (research-ifc section 16).
- **DEP-5 [depth] minor. No takeaway or reference tag.** The energy frames carry a source tag at top right ("DIN V 18599-2", "VDI 2078") and a red takeaway line under the title. The objects clip has neither.
- **DE-3 [de] minor. Title case.** "Vom Schema Zum Objekt" (TITLE, scene line 44) should read "Vom Schema zum Objekt".
- **DE-4 [de] minor. Awkward occurrence note.** "Anzahl: Quellen widersprechen sich, also ein Modellwert" (lines 420–421).
- **DE-5 [de] minor. Non-standard label.** "Südfassade · Lehrausrichtung" (line 610) should use the "angenommen" vocabulary, e.g. "Südfassade · Lehrannahme".
- **DE-6 [de] minor. Distinction narration grammar.** "Stütze ist ein Bauteil; Fenster ist keine Wand, füllt aber eine Öffnung." lacks articles, and the screen says "Eine Stütze ist ein Bauteil".
- **DE-7 [de] minor. Three names for one thing.** "Piloti-Stütze" (instance and type cards), "Stütze" (narration) and "Pilotis" (plural). Use one.

## 5. Fix proposals

Per defect, in the same order.

- **LAY-1.** In `_south()` (lines 274–314) add `v = lambda x, z: view(x, 0, z)` and use `v(...)` in place of every two-argument `view(...)` call (lines 285–312). Then re-render composition and distinction. Expected: wall band from x 0 to L at base to top, ribbon from `op.sill_z` to `op.head_z`, roof screens above the roof, pilotis in the ground set-back, and a non-zero hatch reveal in `_composition`.
- **SYN-1.** Rewrite `_tree()` (lines 344–365). Keep the vertical spine. Stack the four leaves under IfcBuiltElement as an indented list (x = LEFT + 0.35, 0.5 units apart), with one vertical bus at x = LEFT − 0.25 and a short horizontal tick into each leaf. Nothing crosses a chip. Add a MUTED count to each leaf from `_tags`: "25 pilotis", "2 slabs", "walls + 2 screens", "n windows".
- **ABS-1.** Change the palette once in `CLASS_COLOR` (line 36) and `WINDOW` (line 35): IfcColumn STRUCT (warm tan), IfcSlab a cool grey distinct from the tan, IfcWall one green, IfcWindow one blue. Overlays and chips follow automatically.
- **ABS-2.** In the schema beat, after the Rules row lights, show three GUID chips (`guid_chip`) on B3, the first-floor slab and W-S, each with its own `element(tag)["guid"]`. For Relationships, flash W-S → OP-S → RW-S with the two relation names beside the links, reusing `_beam`.
- **ABS-3.** In occurrences, draw IfcRelDefinesByType as a dashed outline around all 25 occurrence circles, lit in the same counter order. Draw a load path from the first-floor slab underside down to B3's base, labelled "LoadBearing TRUE (model)". When the Storey row is shown, highlight the ground-floor parts on the axon.
- **ABS-4.** In distinction, insert the chip "IfcOpeningElement · OP-S" between IfcWall and IfcWindow (reuse the composition chip). Keep the dashed "is-a?" arrow until the cross is drawn (move its FadeOut after `_cross`), and place the cross on the arrow, not beside "has-a".
- **ABS-5.** In `step_excerpt` add IFCPROPERTYSET('Pset_ColumnCommon', …) and IFCRELDEFINESBYPROPERTIES(…, (#101), …) lines, and IFCRELDEFINESBYTYPE if present. Draw leaders from the GUID token to the mini piloti on both sides. Show the Qto value beside its set name.
- **NUM-1.** Change the counter to "n / N" with a basis chip "assumed" and a sub-caption "research: 25 (lex.dk) vs 15 (student slide), open". Remove "25" from the total headline (see NUM-4).
- **NUM-2.** Keep the dimension lines, but set the basis chip on Ø and depth to "open", and add a MUTED footnote "student slide, unverified: r 0.30 m, h 2.87 m". Store the conflicting values in the model so the scene reads them rather than typing them.
- **NUM-3.** Replace "+3.50" with a level symbol "first floor" (no number), or add "assumed" and the derivation "3.25 m + slab". Keep ±0.00 as the definition.
- **NUM-4.** Show the per-piloti value with four significant digits (0.2297 m³) so that 25 × 0.2297 = 5.7425 → 5.74 matches on screen. Assert in the scene's debug run that the displayed product equals round(N · round(v, 4), 2).
- **NUM-5.** In methods write "A = π · (0.15 m)² = 0.0707 m²" and "V = 0.0707 m² · 3.25 m = 0.230 m³", with inputs coloured DATA as on the dimension lines. Both languages through `self.number` and `self.quantity`.
- **NUM-6.** Show slab area × thickness with thickness labelled "assumed" (`slab["thickness"]`), or replace the 95.1 m³ with a ratio bar on the same scale as the pilotis sum (see DEP-3).
- **NUM-7.** Keep the layer names and the total 0.30 m as a teaching assumption. Drop "(white)" and call the layer "render". Put one "assumed" badge in the card header instead of one chip per row.
- **NUM-8.** Dimension the opening's width and height on the elevation from `op["start"]`, `op["end"]`, `op["sill_z"]` and `op["head_z"]`. Drop the 64.5 m² readout or label it assumed with its derivation.
- **NUM-9.** Draw ribbon bands on the south façade only (the face the research supports) or add a MUTED note "ribbon extent disputed (research 4.4)" in the schema beat.
- **NUM-10.** Set IsExternal to FALSE for under-building pilotis, or label it a teaching value. Alternatively remove it from the excerpt.
- **NUM-11.** Confirm each [M] item against standards.buildingsmart.org (IfcColumn, IfcWindow supertypes; Qto_ColumnBaseQuantities; IfcMaterialLayerSetUsage). Keep only [P]-confirmed names on screen if confirmation fails.
- **LAY-2.** Set the inheritance row labels to mix(MUTED, INK, 0.45). Keep values in DATA.
- **LAY-3.** Pass a label size of S_BODY (or the value size) to the property table's label column, or widen the label column. Check both languages.
- **LAY-5.** Shorten the layer strings ("Render 0.03 m", "Brick infill 0.24 m", "Plaster 0.03 m"), put one "assumed" badge in the card header, and keep the card's right padding clear of the text.
- **LAY-6.** Move the footnote to (4.4, −1.1), under the elevation, in MUTED, and keep it clear of the caption band (y < −2.55).
- **LAY-7.** Move "10.75 m" above the grid with extension lines and "4.75 m" right of column 5, both on a CHROME_FILL pill so they do not sit on pilotis or outline.
- **LAY-8.** Keep only two leaders (GUID and Tag) as elbow connectors. Colour the ObjectPlacement value with the dimension colour instead of drawing a leader.
- **LAY-9.** Place the heading above the panel (about 0.3 units above its top edge) or inside the panel as a header bar with its own background.
- **LAY-10.** Raise the non-highlight STEP token colour to mix(INK, BACKGROUND, 0.75) and use at least S_SMALL.
- **LAY-11.** Break the GUID into two lines, or use a smaller mono size with margin. Put the "assumed" chips under the V values and keep them inside the box.
- **LAY-12.** After LAY-1, check which group holds the baseline. Make sure the composition's `hatch`, `readout` and `strip` are in the groups that `_exchange` fades, and that the ground line of `_south` is included.
- **LAY-13.** Set the opacity of B's group to 1 explicitly at the end of `_exchange`. Re-render without skipping animations to confirm.
- **LAY-14.** Move "section ×5" below the Ø dimension. Move the "V = …" readout to the left of the elevation at the fill level, off the section label.
- **LAY-15.** Widen the badge to 4.3 units or reduce its font by one step. Check the German line too.
- **LAY-16.** Check the glyph at 2× zoom. If the font cannot separate I and l, raise it with the toolkit owner. Do not change IFC spelling.
- **LAY-17.** Show the GUID chip beside B3 with a short leader, or use the MINI axon in the instance beat.
- **LAY-18.** Draw the pilotis fill as filled rectangles (width 2·r) in CONCRETE, like the detail. Ghost the roof, not the first-floor slab.
- **SYN-2.** Draw the "= 5.74 m³" leader to a bracket around the 25 pilotis on the axon, and the slab leader to the slab top face. Narrate the slab (see SYN-3 for the wording pattern).
- **SYN-3.** Add the layer set to the composition narration. EN: "Composition means parts: this wall has an opening, a window fills it, and a material layer set is associated with it." DE: "Komposition bedeutet Teile: Diese Wand hat eine Öffnung, ein Fenster füllt sie, und ein Schichtenaufbau ist ihr zugeordnet." Keep both within two caption lines.
- **SYN-4.** Verify the leader targets after LAY-1: column to the piloti, band to the wall band, glass to the window.
- **SYN-5.** Add to the inheritance narration: "OwnerHistory, Description and ObjectType stay empty here." DE accordingly. Or hide the three empty rows.
- **SYN-6.** Draw a dashed arrow from A's pill to B's box and put the cross at the box edge.
- **SYN-7.** Use "height" for the piloti's extent everywhere on screen (table, formula, dimension). Keep IFC "Depth" only for IfcExtrudedAreaSolid.
- **DE-1.** Replace the literal with `"±" + self.number(0.0, 2)` at scene line 243.
- **DE-2.** Render the five missing German frames and add an overflow check to `capture.py` for the boxes named in the defect.
- **DE-3.** TITLE line 44: "Vom Schema zum Objekt".
- **DE-4.** Lines 420–421: "Anzahl: Quellen widersprechen sich; daher ein Modellwert."
- **DE-5.** Line 610: "Südfassade · Lehrannahme".
- **DE-6.** Distinction DE: "Unterscheide »ist ein« und »hat ein«: Die Stütze ist ein Bauteil; das Fenster ist keine Wand, es füllt eine Öffnung."
- **DE-7.** Use "Stütze" everywhere for the singular piloti on screen and in narration; "Pilotis" only in the plural.
- **DEP-1.** Plan: INK poché at 0.85 opacity on the wall outlines, 4.75 m grid dimension strings between bubbles A–B and 1–2, a north arrow and a section cut through B3 with its label. Section disk and elevation cut: 45° hatch (`_drafting.hatch`) in INK or MUTED. Composition strip: a different hatch per layer.
- **DEP-2.** Add a design-event beat (see NB-3). In methods, a `diameter` ValueTracker drives an always_redraw of the circle, the dimension, A and V, and the total.
- **DEP-3.** In total, add two bars on one scale: pilotis sum (5.74 m³) and first-floor slab (95.1 m³, with "assumed"). Bar lengths proportional to value, count-up via the same tracker as the total.
- **DEP-4.** Use one axon style for the whole clip. Ghost only in the instance beat. Make the ground-floor set-back and GREEN_GROUND visible at axon scale.
- **DEP-5.** Add the top-right chip "IFC 4.3 · ISO 16739-1:2024" and a one-line takeaway under the title per beat, as the energy clips do (for example "Type = what all occurrences share").

### New beats (raise conceptual depth with the villa)

Each one adds a beat to the 11 and needs its own narration row in EN and DE.

- **NB-1. "One object, three representations" (after geometry).** Draw the piloti's three representations on the same detail: Axis (vertical centre line), FootPrint (circle at the base) and Body (SweptSolid cylinder). Identifiers come from research-ifc section 7 [P] ('Axis', 'FootPrint', 'Body'; 'SweptSolid'). Layers switched by a ValueTracker. Model call: `represent(tag, kind, model)` for "axis", "footprint" and "body" (add to the model JSON if missing). Narration EN: "One object can have several representations: its axis, its footprint and its body." DE: "Ein Objekt kann mehrere Darstellungen haben: Achse, Grundfläche und Körper."
- **NB-2. "Property set and tributary area" (after occurrences).** On the ground-floor plan, hatch the 4.75 m × 4.75 m square around B3, with dimension strings on each side. The 4.75 m pitch is sourced (research 2.10); 22.56 m² is derived and labelled. Load arrows from the first-floor slab underside to B3's base, labelled "LoadBearing TRUE (model)". Pset_ColumnCommon values from the model. Narration EN: "A property set adds values: LoadBearing says the piloti carries load; its grid square is 4.75 m on each side." DE accordingly.
- **NB-3. "Change a value, keep the identity" (after total).** The type's diameter moves from the assumed 0.30 m to 0.40 m on a ValueTracker. The section circle, A, V, the total, and the property line in the STEP excerpt update. The GUID chip stays. Everything is marked "assumed". Narration EN: "Change the type's diameter and every occurrence follows; the GlobalId stays the same." DE accordingly.
- **NB-4. "Where it sits: site, building, storey" (after composition).** Chips IfcSite → IfcBuilding → IfcBuildingStorey (ground). B3 and W-S are contained in the ground storey, the first-floor slab in the first-floor storey (IfcRelContainedInSpatialStructure). Level bands on the fixed south elevation, with no numbers (research rule). Relation names are research [M] items, so confirm first (NUM-11). Narration EN: "Elements are contained in a storey, and the storey belongs to the building." DE accordingly.

## 6. Verified correct (keep)

- IFC names are spelled correctly: IfcColumn, IfcSlab, IfcWall, IfcWindow, IfcBuiltElement, IfcOpeningElement, IfcRelVoidsElement, IfcRelFillsElement, IfcRelAssociatesMaterial, IfcMaterialLayerSet, IfcMaterialLayerSetUsage, IfcRelDefinesByType, Pset_ColumnCommon.LoadBearing, IfcCircleProfileDef, IfcExtrudedAreaSolid, IfcShapeRepresentation with 'Body' and 'SweptSolid'.
- The supertype chain matches research section 2. The attribute rows in inheritance follow the IFC order.
- The GlobalId "0yoQlyTGDVUBsRS$cAaoWq" has 22 characters from the correct alphabet and is the same in instance, STEP and exchange.
- STEP line #101 uses the IfcColumn attribute order and ends with .COLUMN., which is a valid IfcColumnTypeEnum value.
- A = 0.0707 m² and V = 0.2297 m³ are correct from unrounded inputs; the displayed values 0.071 and 0.230 are the correct roundings.
- B3 at (10.75, 4.75) m is column 3 of row B on a 4.75 m grid centred on the 21.5 m box, consistent with the plan.
- Captions fit two lines in every en frame. The teaching-model badge appears once, with the German wording correct.

## 7. Priority order

1. LAY-1 (elevation collapse). This unblocks composition, distinction and the exchange baseline.
2. SYN-1 with ABS-1 (class tree and palette).
3. NUM-1 to NUM-7 (research conflicts and arithmetic). Fix NUM-4 with NUM-5.
4. LAY-2, LAY-3, LAY-5 to LAY-12 (legibility and overlaps).
5. DEP-1 to DEP-3, then NB-1 to NB-4.
6. DE-1 and the five missing German frames (DE-2), then the remaining German wording items.
