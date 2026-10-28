# Storyboard: 🔌️exchange (`Beat5_Interfaces`, video id `bim_exchange`)

Title: en "Interfaces and Translation" / de "Schnittstellen Und Übersetzung". Scene file `C/🔌️exchange/🐍️.py`, implemented as `present_exchange()`.

One story, one object: the piloti **B3** (an interior piloti of the Villa Savoye, grid row B, column 3) and the **first-floor slab** travel from the architecture model to a structural check. Three exchange forms are compared on the same piloti: IFC STEP (semantic), triangle mesh (geometry only), analysis model (idealisation with assumptions and losses).

Stage rules used below: Manim units, origin at screen centre, 16:9 (x in -7.1..7.1, y in -4..4). Title chip top-left (y > 3.3), caption band y < -2.6. **Free band: x -6.8..6.8, y -2.5..3.1.** Zone L = x -6.75..-1.0 (concept column: STEP / tables / chips), zone R = x -0.7..6.75 (drawing column). Every beat is checked with `capture.py` at the end state of the beat (one PNG per narration key); the end state of each beat must be complete and readable.

Palette (toolkit): INK text, MUTED secondary, ARCH architecture, STRUCT structure, SERV services, DATA highlighted value (yellow), GOOD accepted/kept, WARN lost/stale/estimate (orange), CONCRETE, GLASS. Token colours in the STEP panel come from `step_panel` by role. STEP file syntax is never localised (decimal point stays `.` in both languages); every other number uses `self.number(v, d)` (de `0,25`). Every number on screen is read from the model functions; the numbers in this document are shown only as `[model: …]`.

"Teaching model" is stated once, in beat 1.

## NARRATION rows (copy into the class; clauses 94–115 characters)

```python
NARRATION = [
  ["interface",
   "An interface defines the exchange. IFC is a standard schema, ISO 16739-1, that both sides know.",
   "Eine Schnittstelle definiert den Austausch. IFC ist ein Standardschema, ISO 16739-1, das beide Seiten kennen."],
  ["semantic",
   "Semantic exchange sends an IfcColumn at its place, with a stable GlobalId. Line numbers are only local.",
   "Semantischer Austausch sendet eine IfcColumn an ihrem Ort mit stabiler GlobalId. Zeilennummern gelten nur lokal."],
  ["properties",
   "Its properties travel along: type COLUMN, load-bearing, material, and a circular profile with its extrusion depth.",
   "Seine Eigenschaften reisen mit: Typ COLUMN, tragend, Material und ein Kreisprofil mit seiner Extrusionstiefe."],
  ["mesh",
   "A mesh export keeps only triangles. The shape survives; class, identity and properties do not.",
   "Ein Mesh-Export behält nur Dreiecke. Die Form bleibt, Klasse, Identität und Eigenschaften gehen verloren."],
  ["purpose",
   "To check the first-floor slab structurally, we choose an analysis schema and idealise the slab at its mid-surface.",
   "Für die statische Prüfung der Geschossdecke wählen wir ein Analyseschema und idealisieren sie als Mittelfläche."],
  ["mapping",
   "The slab becomes shells on the grid, each piloti a member. Every analysis item keeps its source GlobalId.",
   "Die Decke wird zu Schalen im Raster, jede Pilotis-Stütze zum Stab. Jedes Analyseobjekt behält seine Quell-GlobalId."],
  ["assumptions",
   "The analysis needs assumptions: self-weight from the thickness, finishes, imposed load, and how supports are fixed.",
   "Die Analyse braucht Annahmen: Eigengewicht aus der Dicke, Ausbau, Nutzlast und die Art der Lagerung."],
  ["reaction",
   "Each piloti carries its tributary area. This is a hand estimate, not a finite element result.",
   "Jede Pilotis-Stütze trägt ihre Einflussfläche. Das ist eine Handschätzung, kein Finite-Elemente-Ergebnis."],
  ["loss",
   "Ribbon windows, infill walls and render colour do not cross over. Reversing the translation cannot recover them.",
   "Bandfenster, Ausfachungswände und Putzfarbe werden nicht übertragen. Die Umkehrung stellt sie nicht wieder her."],
  ["domain",
   "IFC also has a structural analysis domain. This translation is just one chosen example.",
   "IFC kennt auch eine Domäne für Tragwerksanalyse. Diese Übersetzung ist nur ein gewähltes Beispiel."],
  ["check",
   "Before trusting the exchange, validate what the receiver needs, for example with an IDS requirement.",
   "Bevor man dem Austausch traut, prüft man, was der Empfänger braucht, zum Beispiel mit einer IDS-Anforderung."],
]
```

Narration contains no numbers on purpose; all numbers live on screen and come from the model. Each beat is one `self.step(key, <one LaggedStart / AnimationGroup of the sub-steps>, seconds=…)`; the sub-steps follow the word order of the clause. `seconds` is 55–75 % of the clause's spoken length (4.5–6.5 s); `hold_for` waits for the rest. Keep persistent mobjects between beats by `ReplacementTransform` so that the story is one continuous board.

## Running object and shared model values

```python
m = _model.load_model()
b3 = _model.element("B3", m)                      # class IfcColumn, predefined COLUMN, psets Pset_ColumnCommon, geometry axis/base/top
geo = _model.building_geometry(m)                 # grid, pilotis, slabs, walls, ...
q3 = _model.quantities("B3", m)                   # {cross_section_area, height, volume, radius}
excerpt = _model.step_excerpt("B3", m)            # lines + spans (roles guid|class|name|tag|predefined|pset|property|value|placement|representation)
tri = {s: _model.mesh("B3", segments=s, model=m) for s in range(6, 17)}   # cached for the tracker
analysis = _model.translate_structure(m)          # members, nodes, shells, supports, loads, assumptions, losses, source_revision
trib = _model.tributary(m)                        # area_load, pilotis [{tag, area, reaction, cells}]
slab = next(s for s in geo["slabs"] if s["kind"] == "FLOOR")
```

Colour by meaning, kept for the whole clip: GlobalId = DATA (yellow); class / type = STRUCT; property / value = GOOD; placement / dimensions = INK on the drawing; lost = WARN with a strike line; estimate = WARN outline.

---

## Beat 1 · `interface` — one exchange between two models (continuous motion: packet)

Screen (three columns):
- Left, sender: `villa_axon(point=(-4.75,-0.1), scale=0.12, disciplines=("architecture","structure"), highlight=("B3",), model=m)`; label under it (y -2.05) en "Sender · architecture model" / de "Sender · Architekturmodell".
- Right, receiver: `villa_axon(point=(4.75,-0.1), scale=0.12, disciplines=("structure",), highlight=("B3",))`; label en "Receiver · structural check" / de "Empfänger · Tragwerksprüfung".
- Middle (x -1.9..1.9): chip at y 1.75 `chip "IFC4X3_ADD2"` (from `m["schema_id"]`) with the second line en "ISO 16739-1:2024" / de "ISO 16739-1:2024"; below it at y 0.75 a small `step_panel(excerpt, point=(0,0.75), width=3.5, lines=<the two header lines "ISO-10303-21;" and the line containing FILE_SCHEMA>)` (29 characters per line, readable); under it at y -0.2 the label en "interface = shared schema" / de "Schnittstelle = gemeinsames Schema".
- Two arrows sender → middle → receiver at y 0.3 (MUTED, GOOD when the packet passes).
- At y -1.55 a `badge`: en "Teaching model: simplified, rounded values" / de "Lehrmodell: vereinfacht, gerundet" (MUTED; the one and only "teaching model" statement).

Animation (`seconds` ≈ 5.5): FadeIn of both axons and the middle column; B3 on the sender axon pulses (Indicate, DATA). A **packet chip "B3 · IfcColumn"** (small `badge`) travels along the path sender-B3 → middle panel → receiver-B3: `u = ValueTracker(0)`, `always_redraw` positions the chip at `path.point_from_proportion(u.get_value())`, `u` animates 0 → 1. While it passes the panel the FILE_SCHEMA line flashes (Indicate GOOD); on arrival the receiver's B3 lights DATA, the sender's stays lit: the same piloti exists on both sides.

Meaning-at-once: B3 is the real piloti in both axonometrics; the schema line is the actual STEP header text.

## Beat 2 · `semantic` — class, place, GlobalId; line numbers are local

Screen:
- Zone L: `step_panel(excerpt, point=(-3.85,1.55), width=5.7, lines=<IFCCOLUMN entity>, wrap=46)` (entity wrapped over 2–3 rows); small caption above it en "file A" / de "Datei A". Below it at y -0.15 an identical panel "file B" (hidden until step 3).
- Zone R: `villa_plan(level="ground", point=(3.4,0.2), scale=0.23, highlight=("B3",), layers=("structure",), grid=True, labels=True)` → x 0.9..5.9, y -2.0..2.4; B3 carries a DATA halo ring (r = 0.2 screen units, so the 0.15 m radius piloti is visible). Grid labels B and 3 are tinted DATA.
- Class chip next to B3: en "IfcColumn · COLUMN (our modelling choice)" / de "IfcColumn · COLUMN (unsere Modellierung)" (pilotis as IfcColumn is a modelling decision per research-ifc.md section 15).
- Placement dimension lines from the plan origin: `dimension(origin, (b3.x,0), "x = "+self.number(x)+" m")` along the bottom edge and `dimension(origin, (0,b3.y), "y = "+…)` along the left edge, values from `b3["geometry"]["axis"]`; thin dashed guide lines from the dimensions to the halo.
- `guid_chip("B3")` at (3.4,2.85) (the 22-character GlobalId, DATA).

Animation (`seconds` ≈ 6.5, LaggedStart in clause order):
1. STEP rows type in (rows appear top to bottom); token `IFCCOLUMN` (role class) rings STRUCT, a `leader` runs to the B3 halo and the class chip fades in.
2. Token for the placement (role placement, the `#n` reference of the placement line is shown with the next row `IFCLOCALPLACEMENT(…)` appended under the entity; the placement coordinates are the numbers in that row) rings INK; the two dimension lines grow from the origin to B3 with their readouts (x, y in m).
3. Token GlobalId (role guid) rings DATA and its `leader` runs to the GlobalId chip and to B3. Then panel B appears (`TransformFromCopy` from A) with the same GlobalId and a **different line number**: the `#n=` token of A and of B flash WARN with the label en "line number: local to this file" / de "Zeilennummer: nur lokal in dieser Datei", while the GlobalId tokens of A and B both lead (two leaders) to the same lit B3. A WARN "≠" appears between the `#n` tokens, a GOOD "=" between the GlobalIds.

Model: `step_excerpt("B3", first_id=…)` for panel B (renumbered export of the same element; see requests), `element("B3")`, `building_geometry()["grid"]`, `guid_chip`.
Meaning-at-once: place values = the two dimension lines on the plan; class = the chip on the piloti; GlobalId = the one identity both files lead to.

## Beat 3 · `properties` — type, load-bearing, material, profile, extrusion (continuous motion: extrusion)

Screen:
- Zone L: `step_panel(excerpt, point=(-3.85,0.6), width=5.7, lines=<IFCCOLUMN, IFCPROPERTYSET Pset_ColumnCommon, IFCPROPERTYSINGLEVALUE LoadBearing, IFCMATERIAL, IFCCIRCLEPROFILEDEF, IFCEXTRUDEDAREASOLID>, wrap=46)`; ≈ 14 wrapped rows (0.2 per row) from y 2.4 to -0.4. Panel A/B from beat 2 are replaced (`ReplacementTransform` of panel A into this panel).
- Zone R: plan shrinks to a thumbnail `villa_plan(point=(5.6,2.0), scale=0.1, highlight=("B3",))` (ReplacementTransform), B3 still lit. Main drawing `piloti_detail(point=(2.7,-0.15), scale=0.62, model=m, dims=True, grow=h.get_value())`: elevation of the piloti with the section circle beside it.

Animation (`seconds` ≈ 6.5, LaggedStart in clause order). Each token rings (GOOD for property/value, STRUCT for class/type, INK for geometry) and links with a `leader` to its meaning on `piloti_detail`:
1. `.COLUMN.` (predefined) → ring + chip on the elevation en "type COLUMN" / de "Typ COLUMN".
2. `Pset_ColumnCommon` / `LoadBearing` / `.T.` → the piloti head receives a down-pointing arrow from the slab and the chip en "LoadBearing = TRUE: meant to carry load" / de "LoadBearing = TRUE: soll Last tragen"; piloti outline turns STRUCT.
3. Material token → the section circle and the elevation fill with the CONCRETE hatch; chip en "reinforced concrete" / de "Stahlbeton" (`element("B3")["material"]`).
4. `IFCCIRCLEPROFILEDEF` radius token → on the section circle a radius line with readout "r = " + `self.number(radius)` + " m" and the diameter dimension "Ø …" (from `quantities`).
5. `IFCEXTRUDEDAREASOLID` depth token → **ValueTracker extrusion**: `h = ValueTracker(0)` goes 0 → 1; the elevation grows from the circle upward (`always_redraw(piloti_detail(..., grow=h))`), the height dimension line follows the top and its readout counts up to `self.number(q3["height"])` m; the depth token in the STEP row and the readout carry the same number.

Model: `element("B3")` psets and material, `quantities("B3")`, `step_excerpt`.
Meaning-at-once: every token is tied by a leader to the drawn thing it means (type, load flag, hatch, radius, height); the depth number is the grown height.

## Beat 4 · `mesh` — only triangles remain (continuous motion: tessellation resolution)

Screen:
- Zone L: the beat-3 STEP panel stays as a ghost (opacity 0.35) at the same place, reduced to the rows entity / property / material (the other rows fade out). Under it (y -1.2) two columns of chips from `mesh("B3")["kept"]` (GOOD, en "kept: vertices, triangles" / de "bleibt: Punkte, Dreiecke") and `["lost"]` (WARN, items en/de: class IfcColumn, GlobalId, LoadBearing, material).
- Zone R: `mesh_view(tri[s], point=(1.7,0.4), scale=0.75, view="side")` (the cylinder as a triangle strip, 2.2 units tall) and `mesh_view(tri[s], point=(4.6,0.9), scale=6.0, view="top")` (the top cap as a triangle fan inside a dashed true circle of radius 0.15 m ×6 = 0.9 units); connector lines between the two views labelled en "same piloti" / de "dieselbe Stütze". Counter chip under the views (y -1.7): en "N vertices · M triangles" / de "N Punkte · M Dreiecke" with N = `vertex_count`, M = `triangle_count` of the current mesh.

Animation (`seconds` ≈ 6): **ValueTracker `seg`** 6 → 16 (`UpdateFromAlphaFunc`): `always_redraw(lambda: mesh_view(tri[int(round(seg.get_value()))], …))` on both views and the counter chip, so the coarse hexagon visibly becomes a finer polygon around the dashed true circle and the counts grow. In parallel the strike lines (WARN `Line` over the ghost tokens `IFCCOLUMN`, GlobalId, `.T.`, material in the STEP panel; `panel.token(...)`) are drawn one after another and the matching lost chips light up.

Model: `mesh("B3", segments=s)` for s in 6..16 (vertices, faces, counts, kept, lost).
Meaning-at-once: the triangles are the drawn piloti; counts change with what is drawn; the struck-out tokens are exactly the ones we highlighted in beats 2–3.

## Beat 5 · `purpose` — idealise the slab at its mid-surface (continuous motion: collapse)

Screen:
- Zone L: context section `villa_section(point=(-3.9,1.55), scale=0.26, cut="salon", highlight=(slab["tag"],), model=m)` (x -6.7..-1.1, y 0.35..2.75) with its slab marked DATA. Below it a `chip` en "Purpose: structural check of the first-floor slab" / de "Zweck: statische Prüfung der Geschossdecke" (y -0.3) and a `chip` for the target `analysis["target_schema"]` (IFC structural analysis) with the label en "analysis schema (chosen)" / de "Analyseschema (gewählt)" (y -1.0).
- Zone R: zoom detail `slab_strip(point=(3.3,0.0), scale=0.62, row="B", bays=(2,4), collapse=c.get_value(), model=m)`: section through slab and pilotis B2–B4, hatched slab (thickness × 0.62 ≈ 0.16 units), pilotis below, dimension of the slab thickness "t = " + `self.number(thickness)` + " m". Thin dashed zoom lines from the context section slab to the detail frame.

Animation (`seconds` ≈ 6.5): 1. zoom lines and detail appear with the thickness dimension; the target-schema chip fades in. 2. **ValueTracker `c = ValueTracker(0)`** goes 0 → 1: `always_redraw(slab_strip(..., collapse=c))` — the hatched slab thins to its mid-plane (a dashed line labelled en "mid-surface" / de "Mittelfläche"), the piloti solids thin to their axis (a dashed line ending at a node dot on the mid-surface); the thickness dimension migrates into a chip attached to the mid-surface: en "thickness t = … m kept as attribute" / de "Dicke t = … m bleibt Attribut" (`translate_structure()["shells"][0]["thickness"]`).

Model: `building_geometry()["slabs"]`, `quantity(slab, "thickness")`, `translate_structure()` (target schema, shell thickness).
Meaning-at-once: the value t is drawn as the slab thickness, then visibly changes its role from geometry to attribute.

## Beat 6 · `mapping` — shells, members, kept GlobalIds

Screen:
- Zone L: `property_table(rows, point=(-3.85,1.3), width=5.7)` with three rows built from `analysis["members"]` (counts by `kind`), text en/de: row 1 en "IfcSlab FLOOR → IfcStructuralSurfaceMember (shells: N)" / de "IfcSlab FLOOR → IfcStructuralSurfaceMember (Schalen: N)"; row 2 en "IfcColumn ×25 → IfcStructuralCurveMember" / de "IfcColumn ×25 → IfcStructuralCurveMember" (25 = `len(...)`); row 3 en "source GlobalId → kept" / de "Quell-GlobalId → bleibt". Class names are taken from `member["ifc_class"]` of the model, never typed.
- Zone R: start state `villa_plan(level="first", point=(3.4,0.25), scale=0.26, layers=("structure",))` (slab outline, 25 piloti dots; x 0.7..6.1, y -2.2..2.7) → end state `analysis_view(analysis, point=(3.4,0.25), scale=0.26, highlight=("B3",))`: shell grid on the slab outline, curve members as support squares at the grid nodes.
- Two `guid_chip`s: the slab's GlobalId (right of the shells, y 2.95, x 5.0) and `guid_chip("B3")` (below the plan, y -2.3, x 3.4) in DATA, the same strings as in beat 2.

Animation (`seconds` ≈ 6): `ReplacementTransform(plan, analysis_view)`: the slab outline subdivides into shells (cells appear LaggedStart from the centre), the piloti dots become support squares; leaders from the two GlobalId chips lead to a shell and to the B3 support; the property_table rows appear in order. Each row's `source_guid` text highlights DATA.

Model: `translate_structure()` members (`ifc_class`, `source_guid`), shells, supports, `global_id("B3")`.
Meaning-at-once: shells and supports are drawn on the real grid; B3's GlobalId is the same chip that was linked to the STEP token.

## Beat 7 · `assumptions` — loads and supports (continuous motion: stacked load)

Screen:
- Zone L: `property_table` at (-3.85,1.0), width 5.7, five rows with live values (from `analysis["loads"]`, see requests): en "Self-weight: 25 kN/m³ × t = … kN/m²" (density and thickness from the model, product counted by the tracker) / de "Eigengewicht: 25 kN/m³ × t = … kN/m²"; en "Finishes (assumed) … kN/m²" / de "Ausbau (angenommen) … kN/m²"; en "Imposed load (assumed) … kN/m²" / de "Nutzlast (angenommen) … kN/m²"; en "Total q = … kN/m²" / de "Summe q = … kN/m²" (= `trib["area_load"]`); en "Supports: … (assumed)" / de "Lager: … (angenommen)" (condition from `analysis["supports"]`).
- Zone R: `slab_strip(point=(3.4,-0.35), scale=0.62, row="B", bays=(2,4), collapse=1.0, supports=<condition>)` (the idealised slab as a line on pilotis; pilotis axis lines with support symbols at the feet), and above the slab `load_arrows(values=[q_sw, q_fin, q_imp], points=<12 x positions along the slab>, scale=0.1)`: stacked arrow rows, each colour = one component, each arrow length ∝ kN/m² (0.1 units per kN/m²), pointing down onto the slab.

Animation (`seconds` ≈ 6.5): **ValueTracker `q = ValueTracker(0)`** in three runs of `UpdateFromAlphaFunc` (self-weight 0→q_sw, then finishes, then imposed): the stacked arrows lengthen segment by segment, the matching table row counts up with its readout, the total readout follows the sum. At the end the support symbols appear under the pilotis feet with the label en "support: … (assumed)" / de "Lager: … (angenommen)". All assumed values carry the word "assumed/angenommen"; the total is not presented as measured.

Model: `translate_structure()["loads"]`, `["supports"]`, `["assumptions"]`, `tributary()["area_load"]`, `quantity(slab, "thickness")`.
Meaning-at-once: arrow length is the kN/m²; the self-weight arrow is the density times the thickness we saw collapse in beat 5.

## Beat 8 · `reaction` — tributary areas and the reaction of B3 (continuous motion: tributary growth)

Screen:
- Zone L: `tributary_view(trib, point=(-3.3,0.2), scale=0.22, highlight=("B3","A3","A1"))` (x -5.65..-0.95, y -1.9..2.3): cells hatched in alternating tints; B3's cell DATA, the edge piloti and a corner piloti in lighter WARN-less tints; piloti dots on the grid.
- Zone R: three vertical arrows from one common slab line (y 1.2) downward: `load_arrows(values=[R_B3, R_edge, R_corner], points=[(2.3,1.2),(3.8,1.2),(5.3,1.2)], scale=0.012)` — lengths ∝ kN (interior B3 the longest); under each arrow its tag, area and force: "B3 · A = … m² · R = … kN" (both languages; numbers `self.number`). The edge and corner tags are the piloti with the smallest reactions on row A (`trib["pilotis"]`, chosen by min/max over the model data, never typed).
- Equilibrium chip at (3.4,-2.1): en "Σ R = … kN = q × slab area ✓" / de "Σ R = … kN = q × Deckenfläche ✓" with the sum of `trib["pilotis"]` reactions and `quantities(slab)["area"]`.
- A WARN-outlined badge top right of R: en "estimate, not FEM" / de "Schätzung, kein FEM".

Animation (`seconds` ≈ 6.5): **ValueTracker `k = ValueTracker(0)`** 0 → 1 drives everything at once: the hatched cells grow from each piloti (`cell.scale(sqrt(k), about_point=piloti_xy)`), the area readouts count up to `area`, the arrows lengthen to `q × area × k`, the kN readouts count. At k = 1 the B3 cell pulses DATA, then the equilibrium chip fades in.

Model: `tributary()` (`area_load`, `pilotis[].area`, `reaction`, `cells`), `quantities(slab)`.
Meaning-at-once: area = hatch, force = arrow length; interior vs edge vs corner differ visibly in both.

## Beat 9 · `loss` — what does not cross

Screen:
- Zone L: `villa_axon(point=(-3.7,0.5), scale=0.15, disciplines=("architecture","structure"), model=m)`; below it a list of three WARN chips from `analysis["losses"]` (grouped by `source_tag` prefix: ribbon windows `RW-*`, infill walls `W-*`, render colour (property of the wall material layer)), each text en/de from the model (`losses[i]["en"|"de"]`); leaders from each chip to its elements on the axon.
- Zone R: `analysis_view(analysis, point=(4.1,0.4), scale=0.17)` (small); a translation arrow between (y 0.4, x -0.3..2.0, GOOD) and a dashed return arrow below it (y -0.5, WARN) with an "×": en "reverse: cannot recover" / de "Rückweg: nicht wiederherstellbar".
- No second miniature: the left axon itself shows the answer after the return arrow (see animation).

Animation (`seconds` ≈ 6.5): the left axon with the lost elements highlighted WARN (their tags from `analysis["losses"]`) transforms (`ReplacementTransform`) into `villa_axon(..., ghost=<lost tags>)`: windows, infill walls and render colour become ghost outlines while slab and pilotis stay solid; slab and pilotis send a copy along the translation arrow into the analysis view. Then the dashed return arrow draws toward the axon, and the axon does **not** refill: the ghosts stay ghosts; the chips pulse WARN once.

Model: `translate_structure()["losses"]`, `building_geometry()` tags.
Meaning-at-once: the lost data are the real window band, walls and white render of the villa, shown as what remains missing after the round trip.

## Beat 10 · `domain` — one chosen example among several

Screen:
- Zone L: `analysis_view(analysis, point=(-3.6,0.3), scale=0.22, highlight=("B3",))` (x -6.0..-1.2).
- Zone R: `class_box(en "IFC structural analysis domain" / de "IFC-Domäne Tragwerksanalyse", rows=<distinct ifc_class values of analysis["members"] plus the label of the supports from analysis["supports"]>, point=(3.6,1.0), width=5.0)`: rows en/de e.g. "IfcStructuralSurfaceMember – slab shells" / "… – Deckenschalen", "IfcStructuralCurveMember – pilotis" / "… – Pilotis-Stützen"; each row has a `leader` to the drawn shell / member / support symbol. Below, two ghost cards (opacity 0.4, dashed frame) at (2.4,-1.4) and (5.0,-1.4): en "Energy model" / de "Energiemodell" and en "Quantity take-off" / de "Mengenermittlung", with the small label en "other translations of the same source" / de "andere Übersetzungen derselben Quelle" (y -2.2).

Animation (`seconds` ≈ 5): class_box rows appear one by one with their leaders (GOOD); the ghost cards fade in last and a dashed arrow runs from the source (left of the analysis view) to each ghost card; no new numbers.

Model: `translate_structure()["members"][i]["ifc_class"]`, `["supports"]`.
Meaning-at-once: the IFC class names are the real names, each tied to what it labels in the drawing.

## Beat 11 · `check` — validate what the receiver needs (continuous motion: check counter)

Screen:
- Zone L: `class_box(en "IDS requirement" / de "IDS-Anforderung", rows=[applicability, requirement, cardinality], point=(-3.85,1.5), width=5.7)`: row 1 en "applies to: every IfcColumn" / de "gilt für: jede IfcColumn"; row 2 en "requires: Pset_ColumnCommon.LoadBearing" / de "verlangt: Pset_ColumnCommon.LoadBearing"; row 3 en "required, value must match the structural model (not forced to TRUE)" / de "Pflicht, Wert muss zum Tragwerksmodell passen (nicht auf TRUE erzwungen)". Under the card three check chips (y -0.4, -1.0, -1.6): en "GlobalIds unchanged" / de "GlobalIds unverändert"; en "units m · kN" / de "Einheiten m · kN"; en "source revision r = r" / de "Quellrevision r = r" (both r from `analysis["source_revision"]` and `m["revision"]`); each gets a GOOD check mark when computed true from the model.
- Zone R: `villa_plan(level="ground", point=(3.4,0.3), scale=0.26, layers=("structure",))`; all 25 pilotis start MUTED; counter chip at (3.4,-2.2): en "n / 25 IfcColumn pass" / de "n / 25 IfcColumn bestanden" where 25 = number of applicable elements.
- At the end a second, smaller counter next to it for the mesh file: en "mesh export: 0 IfcColumn found" / de "Mesh-Export: 0 IfcColumn gefunden" (WARN).

Animation (`seconds` ≈ 6): **ValueTracker `n = ValueTracker(0)`** 0 → number of applicable pilotis: `always_redraw` turns the first `int(n)` pilotis (in model order, row by row) GOOD with a tick while the counter reads `int(n) / total`; then the three check chips tick one by one; finally the mesh counter appears in WARN. Pass/fail logic (IFC source passes when the piloti has `Pset_ColumnCommon.LoadBearing`, mesh source has no `IfcColumn`) comes from `ids_check` (see requests).

Model: `ids_check(...)`, `analysis["source_revision"]`, `load_model()["revision"]`.
Meaning-at-once: each of the 25 drawn pilotis is the checked object; a failed export shows as an empty result, not as a number.

---

## Summary of ValueTracker motions (all drive the drawing and the numbers together)

1. `u` packet along the exchange path (beat 1); 2. `h` extrusion height with counting dimension (beat 3); 3. `seg` tessellation resolution with live vertex/triangle counts (beat 4); 4. `c` slab and piloti collapse to mid-surface (beat 5); 5. `q` stacked load arrows with counting readouts (beat 7); 6. `k` tributary hatch, area, arrow length and kN (beat 8); 7. `n` IDS check counter lighting the 25 pilotis (beat 11).

## Truthfulness notes (research-ifc.md)

- Schema id `IFC4X3_ADD2`, ISO 16739-1:2024; `IfcColumn` with `.COLUMN.`; `Pset_ColumnCommon.LoadBearing` is "meant to carry loads", not a universal TRUE (beat 3 chip, beat 11 row 3); STEP `#n` is local, GlobalId is the 22-character identity (beat 2).
- Piloti as `IfcColumn` is stated as our modelling choice (beat 2).
- The structural analysis domain classes (`IfcStructuralSurfaceMember` with a thickness attribute, `IfcStructuralCurveMember`) are shown only as names returned by the model; the IFC relation that links an analytical member to the physical element is not named (unverified in 4.3.2.0). The "keeps its source GlobalId" statement is about our translation record (`source_guid`).
- Tributary values are labelled "estimate, not FEM"; all loads and support conditions are labelled assumed.
- Energy-model / quantity take-off cards in beat 10 are plain labels with no IFC class names.

## Requests (needed for this storyboard; fallbacks in brackets)

Model agent:
- `step_excerpt(key, model=None, first_id=101)` so that a renumbered export of the same element has the same GlobalId and different `#n` (beat 2; fallback: scene renumbers with a regex and shifts spans). Spans for the profile radius and extrusion depth (roles `profile`, `extrusion`) and `material`; the excerpt lines wrap at commas, or the panel does it.
- `mesh()["kept"]` / `["lost"]` as lists of `{"id","en","de"}` (lost ids: class, guid, property, material).
- `translate_structure()`: `source_schema`, `target_schema`, per-kind counts, `loads {self_weight {density, thickness, value}, finishes, imposed, area_load}` (each with `basis`), `supports[].condition` with en/de label, `losses[]` with `source_tag`, `property`, en/de.
- `tributary()`: per piloti `kind` interior/edge/corner, `area`, `reaction`; plus `total_reaction` and `slab_area`.
- `ids_check(spec=None, source="semantic"|"mesh", model=None) -> {applicable:[tags], passed:[tags], failed:[tags], requirement}`.
- `quantities("B3")` includes `radius`; `element("B3")["geometry"]["axis"]`; slab thickness via `quantity(slab,"thickness")`.

Toolkit agent:
- `step_panel(..., lines=[…], wrap=46)` and `panel.token(line_match, role, nth=0)` (a mobject for one span), `panel.rows` (so strike lines and rings can be drawn); mono text ≥ 17 px at 720p (character width ≈ 0.11 units, row pitch 0.2).
- `piloti_detail(..., grow=1.0)`, `.parts` with `elevation`, `section`, `dim_height`, `dim_diameter`, `dim_radius`.
- `mesh_view(mesh, point, scale, *, view="axon"|"side"|"top", highlight=())` with the dashed true circle in `top`.
- new `slab_strip(point, scale, *, row="B", bays=(2,4), collapse=0.0, supports="none"|<condition>, model=None)` (section through slab and pilotis along a grid row; collapse 0..1 thins slab and piloti to mid-surface/axis; `.x_of(tag)`, `.slab_top_y`).
- `villa_axon(ghost=<tags>)` fades those tags to outline; `villa_plan(highlight=…)` adds a DATA halo; all villa widgets document their origin convention or provide `.at(x,y,z)`; `.parts[tag]` for every drawn element.
- `analysis_view` and `tributary_view`: `.parts` per tag (shell cells, supports, hatched cells) so a cell can be scaled about the piloti centre; `load_arrows(values, points, scale, colors=…)` returns stackable arrows.
