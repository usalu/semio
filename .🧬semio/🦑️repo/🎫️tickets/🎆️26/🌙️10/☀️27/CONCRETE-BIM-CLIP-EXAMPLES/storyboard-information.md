# Storyboard: 📊️information (`Beat1_DataToInformation`, `bim_information`)

Title en "Data Become Information" / de "Daten Werden Information". Scene file `C/📊️information/🐍️.py`, class `Beat1_DataToInformation` with `TITLE`, `NARRATION`, `construct()` calling `self.present("information")` and `present_information()` holding the whole animation.

Sources used: `agent-rules.md`, `plan.md`, `api.md`, `research-villa-savoye.md` (facts 2.10 grid 4.75 m / first 5 m / reduced for cost, 5.1 curved wall from a car's turning circle, 5.4 vehicles pass under the building), `research-ifc.md` (sections 4, 6, 12, 14, 15), the baseline still (bare cuboid, "thickness = 0.30 m", loose text) and the energy clips `demand-power` and `solar-radiation` (drawing carries the lesson, `ValueTracker` + `always_redraw`, `hold_for` budgets, nothing leaves the screen until it is explained).

## Story in one paragraph

The raw number 4.75 stands alone and could be anything. It lands as the dimension between two piloti axes (row E, axes 3 and 4) on the ground-floor plan of the Villa Savoye. The schema adds property, unit, grid and basis (sourced). A question needs it: can a 1929 car drive between those pilotis and turn under the house along the curved glass wall? Clear width = spacing − piloti diameter is shown on a magnified bay and as hatch on the plan; the car drives through (continuous motion) and the glass wall follows its turning circle. The grid was first wider and was reduced to save cost: the spacing slides from 5.00 to 4.75 m and the answer still holds. Then the same piloti carries more properties (material, LoadBearing, IsExternal) and relationships (storey, type, slab, grid position). Handoff: people read the plan, applications read the STEP data, both describe the same piloti.

Truthfulness rules kept: no sentence says ISO 19650 defines a data-information hierarchy (the word "information" is used in its plain sense; no standard is cited in this clip); the grid, the curved-wall principle and the cost reduction are sourced, everything else (piloti Ø, car size, turning radius, wall radius) is shown as "assumed"; LoadBearing TRUE is stated for this piloti, never as a universal rule; no insulation, U-values or ramp slope appear.

## Narration (final keys, one caption, at most two lines)

No number appears in any narration clause, so the model can change values without desynchronising captions; all numbers are on screen and come from the model.

| key | en | de |
| --- | --- | --- |
| `raw` | This number is data. Alone, it does not say what was measured. | Diese Zahl ist ein Datum. Allein sagt sie nicht, was gemessen wurde. |
| `context` | Context: it is the distance between two piloti axes of the Villa Savoye. | Der Kontext: Es ist der Achsabstand zweier Pilotis der Villa Savoye. |
| `meaning` | The schema gives it meaning: property, unit, grid, and whether it is sourced or assumed. | Das Schema gibt ihr Bedeutung: Eigenschaft, Einheit, Raster und ob sie belegt oder angenommen ist. |
| `question` | Information serves a question: can a 1929 car drive between the pilotis and turn under the house? | Information dient einer Frage: Kann ein Auto von 1929 zwischen den Pilotis hindurchfahren und unter dem Haus wenden? |
| `clearance` | The clear width is the spacing minus the piloti diameter. That is the room the car has. | Die lichte Weite ist der Abstand minus der Durchmesser der Pilotis. So viel Platz hat das Auto. |
| `drive` | The car drives through and turns under the house; the glass wall follows its turning circle. | Das Auto fährt hindurch und wendet unter dem Haus; die Glaswand folgt seinem Wendekreis. |
| `reduced` | Even when the grid shrank to save cost, the answer holds: the car still fits. | Auch als das Raster aus Kostengründen schrumpfte, bleibt die Antwort: Das Auto passt noch. |
| `properties` | A piloti carries more properties: its material, whether it bears load, and whether it is external. | Eine Pilotisstütze trägt weitere Eigenschaften: Material, ob sie Lasten trägt und ob sie außen steht. |
| `relations` | Relationships complete the meaning: storey, type, the slab it carries and its grid position. | Beziehungen vervollständigen die Bedeutung: Geschoss, Typ, die getragene Decke und die Rasterposition. |
| `handoff` | BIM connects data in a structured building model, so people and applications read it the same way. | BIM verbindet Daten zu einem strukturierten Gebäudemodell, das Menschen und Anwendungen gleich lesen. |

Estimated speech (German 2.2 words/s): 5, 5, 6, 8, 8, 6, 6.5, 7, 7, 6.5 s, about 65 s of clip. The animation seconds below are chosen to sit inside these budgets; `hold_for` fills the remainder.

## Stage geometry (units = Manim units, frame 14.22 x 8, y up)

- Title chrome top-left (x -6.5..-2.7, y 3.2..3.6) and caption band (y < -2.62) are reserved. Persistent muted note top-right corner (like `_din_ref` in the energy clips): **"Teaching model · simplified, rounded" / "Lehrmodell · vereinfacht, gerundet"**, size 18, fades in with beat 2, stays until the end. This is the single "teaching model" mark.
- Left half: ground-floor plan, `PLAN_POINT = (-2.7, 0.15)` (centre of the 21.5 x 19 m box), `PLAN_SCALE = 0.245` units per m. The box covers x -5.34..-0.07, y -2.18..2.48. Axis 3 (the box centre) is at x = -2.70, axis 4 at x = -1.49, row E (north façade) at y = 2.48. Grid number labels at the bottom (y ~ -2.45), letter labels at the left (x ~ -5.65); nothing on the plan's north side so the car can approach from there. No room names, no ramp arrow in this clip.
- Right panel: x 0.55..6.65 (centre `PANEL_X = 3.6`, width 6.1), y -2.4..3.3.
- Colour semantics (toolkit palette): `DATA` yellow = the value in question and its dimension; `STRUCT` = pilotis; `GLASS` = curved glass wall; `GOOD` = fits / sourced / TRUE; `WARN` = assumed / does not fit; `CONCRETE` = slab and hatch; `MUTED` = IFC class names and notes.
- Basis chips: `sourced` green, `assumed` orange, on every value row that shows a quantity.
- Piloti are drawn at true size (Ø 0.30 m is only 0.07 units on the plan); the highlight ring (radius 0.14) and the magnified bay carry the reading.
- "E/3" and "E/4" below are produced from `entry_pair` tags at run time (`tag[0] + "/" + tag[1:]`), not typed.

## Model calls (names from `api.md` section 2 plus the requests listed at the end)

Once at the top of `present_information`: `m = load_model()`, `c = clearance(m)`, `g = building_geometry(m)`, `path = car_path(m)` (request R1), `a, b = path["entry_pair"]` (the two pilotis flanking the entry bay on row E), `lang = self.language()`. Everything numeric is read from these dicts and the functions named per beat; the scene only maps metres to screen units through `plan.to_screen(x, y)` (request R2).

## Beat 1 · `raw` (about 4.5 s)

On screen: black stage, title only. The number `self.number(c["spacing"], 2)` ("4.75" / "4,75"), size 96, `DATA`, centred at (0, 0.6). Under it four muted candidate units one after the other at (0, -0.7), size 34: `m`, `kg`, `kN`, `°C` (generic unit examples, not model values), each shown as "4.75 m?" style ghost text, cross-fading every 0.7 s.
Animation: `FadeIn(raw)` 0.8 s, then `LaggedStart` of the four question tags (`FadeIn` then `FadeOut` each), `seconds=4.5`.
Meaning at the same time: nothing is drawn yet, which is the point: the number has no object.

## Beat 2 · `context` (about 6 s)

On screen: the ground-floor plan `plan = villa_plan("ground", PLAN_POINT, PLAN_SCALE, model=m, highlight=(a, b), grid=True, labels=True, layers=("structure","architecture"))`: poché, 25 pilotis (row E and the rest), curved glass wall, ribbon-free ground floor, dashed box outline of the first floor above. The pair `a`, `b` carries the `DATA` highlight. Grid labels `3`, `4` (bottom) and `E` (left) accent in `DATA`.
Right panel (x 3.6): small axonometric `villa_axon((3.6, 1.55), 0.14, model=m, highlight=piloti tags, disciplines=("architecture","structure"))`, about 3 units wide, y 0.5..2.7. Under it a four-row context list (`property_table`, width 5.6, rows y = -0.15, -0.6, -1.05, -1.5, label muted, value `INK`): `IfcSite` · name from `m["site"]["name"][lang]`; `IfcBuilding` · `m["building"]["name"][lang]` with the years from the model; `IfcBuildingStorey` · the ground storey name with its elevation (`±0.00`); and the place "grid axes 3 – 4, row E" built from `a`, `b`.
Dimension on the plan: `dim = dimension(plan.to_screen(*xy_a), plan.to_screen(*xy_b), self.number(c["spacing"], 2), offset=0.0, color=DATA)` drawn along the row-E axis line between the piloti centres with end ticks; label above the line at about (-2.10, 2.80), no unit yet.
Animation (6 s): the big number `ReplacementTransform(raw, dim)` (it shrinks and flies to the dimension label) together with `LaggedStart` of plan `Create` (walls, pilotis, glass wall), then `FadeIn` of axon and list rows one after the other, then the top-right note.
Meaning at the same time: the value is now a measured span on a recognisable building; the unit is deliberately still missing.

## Beat 3 · `meaning` (about 6.5 s)

On screen: axon and list fade out; right panel becomes `property_table(rows, (3.6, 1.0), width=5.8)` headed by a muted tag `SCHEMA · quantity`. Rows (label left, value right), each from `info = information(a, "grid.spacing", lang)`:
1. Property: `info["label"]` ("Grid spacing" / "Achsabstand").
2. Value: `self.number(info["value"], 2)` in `DATA`.
3. Unit: `info["unit"]` ("m").
4. Grid: "axes 3 – 4 · row E" from the tags (muted).
5. Basis: `info["basis"]` as a chip, `sourced` in `GOOD`, with the one-line `info["meaning"]` beneath (muted, size 20).
Animation (6.5 s): rows arrive one by one (`LaggedStart`, 0.9 s apart). Row 3 `Indicate` and a copy of `m` flies to the plan label: `ReplacementTransform(dim_no_unit, dim_with_unit)` so the label reads "4.75 m". Row 4 pulses axis labels `3`, `4`, `E` on the plan (`Indicate(plan.parts["axis_x:3"], ...)`). Row 5 chip fades in last.
Meaning at the same time: every schema field points at something drawn: unit at the dimension text, grid at the axis labels, basis at the sourced chip.

## Beat 4 · `question` (about 8 s)

On screen: right panel cleared. Top: the question as a headline, size 34, `INK`, y 2.75: "Does it fit through?" / "Passt es hindurch?" (derived from the caption, two short lines allowed). Middle: `car((3.6, 1.0), 0.55, angle=90)` the 1929 car top view lying horizontal, about 2.4 wide, 1.0 tall, with its width dimension (`dimension`, `DATA`, text `self.number(c["car_width"], 2) + " m"`, offset below). Rows below (two rows at y -0.75 and -1.25, `property_table`): "Car width" · `information(a, "car.width", lang)` with `assumed` chip in `WARN`; "Turning radius" · `information(a, "car.turning_radius", lang)` `assumed`.
On the plan: the curved glass wall (`plan.parts["curtain_wall"]`) lights up in `GLASS` (`Indicate`, line width up), a `DATA` "?" (size 40) appears in the entry bay at (-2.10, 2.05) between the two highlighted pilotis. The plan dimension stays.
Animation (8 s): headline `FadeIn`, car `FadeIn` with its width dimension `GrowFromEdge`, curved-wall highlight, "?" pulses once.
Meaning at the same time: the question needs three values (spacing, car width, wall curve) and each is a drawn thing.

## Beat 5 · `clearance` (about 8 s)

On screen: right panel becomes the magnified bay (`bay_detail`, request R3), centre (3.6, 0.6), scale 0.8 units per m (about 3.3 times the plan scale): two piloti circles on the row line y = 0.6 (x 1.7 and 5.5), top-view car (`c["car_width"]` wide) centred between them, hatched `GOOD` band for the clear width between the circle edges, spacing dimension at y 2.75 with label `4.75 m` (`DATA`), clear-width dimension at y -1.45 (`GOOD`), the two side margins as `DATA` arrows with labels from `c["margin"]` (about 1.33 m each). Under the strip at y -2.0 the live formula `4.75 m − 0.30 m = 4.45 m` as `math_readout` in three coloured slots (spacing `DATA`, diameter `WARN` because assumed, clear width `GOOD`), numbers from `clearance(m, spacing=s)` (R7) with `s = ValueTracker(c["spacing"])`.
On the plan: a bracket box around the bay and two dashed lines to the strip (magnifier link, `MUTED`); the same clear width as a `GOOD` hatched band along row E between the piloti edges; the plan label "4.75 m" is moved into the strip (`TransformFromCopy` then the plan label fades) so the number is never on screen twice.
Animation (8 s): bracket and dashed link grow, strip `FadeIn` piloti first, then the spacing dimension (`TransformFromCopy` from the plan label), then Ø marking on one circle (`FadeIn` label `Ø 0.30 m` from `information(a, "piloti.diameter", lang)`, `assumed`), clear-width hatch grows left to right (`Create`), formula slots fill in the same order, car `FadeIn` last with the margins.
Meaning at the same time: subtraction is shown as a hatched strip shrinking from 4.75 to 4.45; the car is drawn inside it.

## Beat 6 · `drive` (about 6.5 s) — continuous motion 1

On screen: strip stays at 35 % opacity; formula row stays. Plan: car appears at its start pose outside the north façade at the entry bay (`path["poses"][0]`), top view `car(pt, PLAN_SCALE, angle)` (about 0.44 x 1.08 units), `DATA` outline with `INK` windows.
Animation: `drive = ValueTracker(0)`, `ApplyMethod(drive.set_value, 1, run_time=6.0, rate_func=linear)`. `always_redraw` car at `plan.to_screen(*pose(drive))` with heading from `path["poses"]` (interpolated between the model samples), and a growing swept area `swept_path(path, c, upto=drive.get_value())` filled `DATA` at 18 % opacity behind it. The strip car fades in and out with the plan car (opacity follows the distance of `drive` from `path["gap_progress"]`) so the magnified bay shows exactly the moment the car passes between the pilotis; the margin arrows flash `GOOD` then. After the gap the car follows the curved glass wall; a dashed circle of radius `c["turning_radius"]` around `g["curtain_wall"]["centre"]` (the car's turning circle, `DATA`, dashed) and a `GLASS` radius leader `R = self.number(c["curve_radius"], 2) m` (`assumed`) appear at about 80 % of the drive. The car stops at its end pose under the house.
Meaning at the same time: the glass wall's curve is the drawn turning circle; the passage through the bay is the shown clear width.

## Beat 7 · `reduced` (about 6.5 s) — continuous motion 2

On screen: strip and formula as in beat 5; plan car parked; swept area stays.
Animation: `grid_s = s` (the tracker from beat 5). Step 1 (1.0 s): `s` 4.75 → 5.00 (`c["initial_spacing"]`) with a muted tag "first grid" / "erstes Raster" at the right piloti; step 2 (3.5 s, `smooth`): `s` 5.00 → 4.75 with tag "reduced for cost" / "aus Kostengründen verkleinert". `always_redraw` re-draws the right piloti, the spacing dimension and its label, the hatched clear width, the margin arrows (car stays centred) and the formula numbers from `clearance(m, spacing=s.get_value())`. At the end a `GOOD` chip "fits" / "passt" with `margin` appears at (3.6, 3.1) driven by `clearance(...)["fits"]` (turns `WARN` "does not fit" if the model said False; with the current model it stays green).
Meaning at the same time: a value changes and the drawing and the answer change with it; the decision does not flip.

## Beat 8 · `properties` (about 7 s)

On screen: strip, formula, chips and magnifier link fade out; plan keeps the dimension line only (label hidden) and a `DATA` ring (radius 0.14) around piloti `a` plus a thin drop line to the right panel. Right panel: left, `piloti_detail((1.7, 0.2), 0.42, model=m, fill=1.0, dims=True)` the column elevation about 1.5 tall with its section circle, Ø and height dimension lines (values from `quantities(a)` and `information(a, "piloti.diameter", lang)`); right, `property_table` at (4.6, 0.8), width 3.9:
1. `IfcColumn` · `COLUMN` (muted class line, from `element(a)`).
2. Material · `information(a, "material", lang)["value"]` (reinforced concrete) — the column elevation fills with `CONCRETE` hatch.
3. `Pset_ColumnCommon.LoadBearing` · `TRUE` (`GOOD`) — three `DATA` load arrows run down the column from a slab stub on top to the ground.
4. `Pset_ColumnCommon.IsExternal` · `TRUE` (`GOOD`) — the dashed first-floor box outline on the plan flashes `GLASS` along the façade line the piloti stands on.
Animation (7 s): `LaggedStart` rows 1.3 s apart, each row triggers its drawn meaning (hatch fill, load arrows, façade flash).
Meaning at the same time: material is the hatch, LoadBearing is the arrows, IsExternal is the façade the piloti stands at.

## Beat 9 · `relations` (about 7.5 s)

On screen: right panel `property_table` replaced by four relation rows (chip, 5.6 wide, y 2.6, 1.55, 0.5, -0.55, each row has a coloured dot matching its target on the plan and the muted IFC name under it); from the highlighted piloti on the plan a `DATA` leader line runs to each row in turn (`leader(plan.parts[a], row_anchor)`):
1. Contained in: ground storey name and elevation (`relations_of(a)` entry `IfcRelContainedInSpatialStructure`). Plan: the whole ground-floor plan tints `GREEN_GROUND` at 12 % with level mark `±0.00`.
2. Type: "Piloti" (`IfcColumnType`, `IfcRelDefinesByType`), with `× len(type_occurrences("Piloti", m))`. Plan: all 25 pilotis pulse `STRUCT`.
3. Connected to: first-floor slab (`IfcSlab FLOOR`, elevation from `g["slabs"]`, `IfcRelConnectsElements`, request R8). Plan: the slab outline (`g["slabs"][0]["outline"]`) draws dashed `CONCRETE` with a faint fill.
4. Grid position: "E / 3" and the attribute `ObjectPlacement`. Plan: grid axis lines `E` and `3` draw through the piloti in `DATA`, their labels light up.
Animation (7.5 s): `LaggedStart` of the four row/leader/plan-effect triplets (1.6 s each), earlier effects dim to 25 % when the next starts.
Meaning at the same time: each relationship is a drawn link to a recognisable part of the building; the count 25 lights 25 real pilotis.

## Beat 10 · `handoff` (about 6.5 s)

On screen: plan keeps only the highlighted piloti ring and a muted ground-floor tint; pilotis of type "Piloti" stay softly lit. Right panel from top: chip "People read the drawing" / "Menschen lesen die Zeichnung" (`ARCH`, y 3.0) with an arrow to the plan; chip "Applications read the data" / "Anwendungen lesen die Daten" (`STRUCT`, y 2.2); below it `step_panel(step_excerpt(a, m), (3.6, 0.6), width=6.0, highlight_roles=("guid","class","tag"), only=("IFCCOLUMN","IFCRELDEFINESBYTYPE","IFCRELCONTAINEDINSPATIALSTRUCTURE","IFCPROPERTYSINGLEVALUE"), wrap=True)` four wrapped monospace lines; under the panel `guid_chip(a, (3.6, -1.35))` and the muted line "same piloti · same GlobalId" / "dieselbe Stütze · dieselbe GlobalId" (y -1.95). A `DATA` leader connects the ring on the plan to the highlighted `IFCCOLUMN` token and the GlobalId chip.
Animation (6.5 s): the two reader chips grow their arrows (`GrowArrow`), STEP lines `FadeIn` with `LaggedStart` (0.4 s apart), GlobalId chip `FadeIn`, then `Indicate` on piloti ring and chip together; the clip ends with the number back as the dimension label "4.75 m" on the plan (`FadeIn`), so the first beat's datum is visibly part of the model.
Meaning at the same time: the same object is a drawn piloti for people and a typed entity for applications.

## Continuous motions (ValueTrackers)

- `drive` 0 → 1 linear 6 s (beat 6): car pose, swept area, strip-car opacity, turning-circle reveal.
- `s` spacing 4.75 → 5.00 → 4.75 (beats 5 and 7): right piloti, spacing dimension, hatched clear width, margins, formula, verdict chip.
- Secondary: row reveal alphas (no tracker needed).

## Requests to the model and toolkit agents (beyond `api.md`; scene adapts if answered differently)

- R1 model: `car_path(model=None) -> {"poses": [[x, y, heading_deg], ...] (about 120 samples, start outside the north façade, through the entry bay, along the curved wall, end under the house), "entry_pair": [tagA, tagB] (two adjacent pilotis on row E flanking the bay where the curtain wall begins, axis 3 and 4 so the bay sits at the plan centre), "gap_progress": 0..1, "gap": {"start": [x, y], "end": [x, y]}}`; plus `swept_path` polygon via the toolkit (R3).
- R2 toolkit: `villa_plan` result exposes `to_screen(x, y)` (metres to screen) and `parts` keys `<tag>` (piloti), `"curtain_wall"`, `"slab:first"`, `"axis_x:<n>"`, `"axis_y:<letter>"`, `"box"`; parameters `label_sides=("bottom","left")`, `names=False`, `ramp_arrow=False`; `highlight` accepts piloti tags.
- R3 toolkit: `bay_detail(point, scale, *, spacing, model=None, car=True, car_opacity=1.0)` returning parts `piloti_a`, `piloti_b`, `span_dim`, `clear_hatch`, `car`, `margin_a`, `margin_b`, `clear_dim`; `swept_path(poses, upto, car_dims, ...)`; `dimension(...)` returns parts `line` and `label` and accepts `unit=None`; `car(point, scale, angle)` returns a group with `width_dim` addressable.
- R4 model: `information(tag, property_id, language)` additionally returns `label` (localized property name) and supports ids `grid.spacing`, `grid.initial_spacing`, `piloti.diameter`, `material`, `Pset_ColumnCommon.LoadBearing`, `Pset_ColumnCommon.IsExternal`, `car.width`, `car.turning_radius`; type-level Psets are returned for the occurrence.
- R5 toolkit: `step_panel(..., only=<substrings>, wrap=True)` to show selected lines; model `step_excerpt` contains IFCCOLUMN, IFCRELDEFINESBYTYPE, IFCRELCONTAINEDINSPATIALSTRUCTURE and a Pset property line for the piloti.
- R6 model: `clearance()["margin"]` is the per-side margin `(clear_width − car_width) / 2`.
- R7 model: `clearance(model=None, spacing=None)` recomputes for a hypothetical spacing (pure function) so the scene never does arithmetic; same keys, including `fits`.
- R8 model: add one `IfcRelConnectsElements` (piloti to first-floor slab) per piloti to `relations`, returned by `relations_of`.

If a request is not available, fallback in the implementation: compose the bay detail from Manim primitives and `dimension`/`hatch`, take the car path from the curtain-wall path with the toolkit and drop the strip-car fade; the beats and captions stay identical.

## Layout checks for the capture pass

Right-panel content stays inside y -2.4..3.3 and x 0.55..6.65; plan labels stay out of the caption band; title (x < -2.7, y > 3.2) is clear of the plan label (label x -2.53..-1.67 at y 2.65..2.95); the plan dimension label disappears before the car appears at the north edge (beat 5 moves it into the strip), so no filled car overlaps a label; the top-right note and the strip's spacing label (y 3.05) do not touch (note y about 3.65). Capture all ten keys in en and de and check Ø, R and margin labels for size (minimum font size 18).
