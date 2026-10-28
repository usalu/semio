# Storyboard 🧬️objects · `Beat2_ObjectOrientation` · "From Schema to Object" / "Vom Schema Zum Objekt"

Phase 1 (storyboard only; no code touched). Sources read: `agent-rules.md`, `plan.md`, `api.md`, `research-ifc.md`, `research-villa-savoye.md`, the current scene and `_objects` visuals, the energy clips `🌬️ventilation/…/📦️volume` (ValueTracker + `always_redraw` + `math_readout` style, `hold_for` timing) and the rejected frame `Beat1_DataToInformation-en-context.png` (bare cuboid + yellow text on black).

## 0. Idea in one paragraph

One real object is followed from the schema down to a single piloti and back out into an exchange file. The **Villa Savoye axonometric** is on screen from the first to the last-but-three beat; every IFC term on the left is tied to a part of the villa on the right by colour or leader. The class tree is no longer a list of four chips: it is the true IFC 4.3 chain `IfcRoot → … → IfcBuiltElement → IfcColumn | IfcSlab | IfcWall | IfcWindow`, and "kind" is shown by lighting **all parts of the villa that belong to the class at once**. Instance = one lit piloti (`B3`) with its GlobalId. Type vs occurrence = one `IfcColumnType` "Piloti" and the **count of 25 occurrences** lighting up one by one on the ground-floor plan. Attributes = Ø and height as **dimension lines** on a piloti detail with the `assumed` basis chip. Methods = the **concrete fills the cylinder while the volume counts up**, then the same method runs for all pilotis. Inheritance = attribute rows copied from every supertype into the `IfcColumn` box, each row tied to something on the plan (GlobalId, tag, placement). Composition = first-floor south wall `W-S` has an opening, a window that fills it and a material layer set; distinction = is-a (piloti → built element) vs has-a (window fills the wall's opening, not a kind of wall). Exchange = STEP lines travel, the `volume()` method does not: application B recomputes the same value.

Differences to the plan text: two beats are added (`occurrences`, `total`) so type/occurrence and the concrete total get their own clause (11 beats); narration contains **no numbers** (all numbers are on screen from the model, narration is a literal for the AST tests); the exchange beat also states that IFC may carry the *result* of a method (a quantity set) but never its code.

"Teaching model" is said once, in beat 1 (badge under the axonometric). "assumed" is shown wherever the model says `basis: assumed` (beats 5, 6, 9).

## 1. NARRATION (literal list `[key, english, german]`, 11 rows, keys unique)

| # | key | English | German |
| --- | --- | --- | --- |
| 1 | `schema` | A schema defines vocabulary and rules for objects and relationships. For buildings, that schema is IFC 4.3. | Ein Schema definiert Begriffe und Regeln für Objekte und Beziehungen. Für Gebäude ist das Schema IFC 4.3. |
| 2 | `class` | A class describes a kind of thing: IfcColumn, IfcSlab, IfcWall and IfcWindow are kinds of built element. | Eine Klasse beschreibt eine Art von Dingen: IfcColumn, IfcSlab, IfcWall und IfcWindow sind Arten von Bauteilen. |
| 3 | `instance` | An instance is one particular object: this piloti of the Villa Savoye, with its own GlobalId. | Eine Instanz ist ein bestimmtes Objekt: diese Pilotis-Stütze der Villa Savoye mit eigener GlobalId. |
| 4 | `occurrences` | A type holds what all pilotis share; every piloti placed in the building is an occurrence of it. | Ein Typ hält, was alle Pilotis gemeinsam haben; jede platzierte Stütze ist ein Vorkommen davon. |
| 5 | `attributes` | Attributes hold values: diameter and height, drawn as dimension lines. The model marks both as assumed. | Attribute halten Werte: Durchmesser und Höhe, gezeichnet als Maßlinien. Das Modell markiert beide als angenommen. |
| 6 | `methods` | A method is behavior: the application computes cross-section and volume from the attributes. | Eine Methode ist Verhalten: Die Anwendung berechnet Querschnitt und Volumen aus den Attributen. |
| 7 | `total` | The same method runs for every occurrence and adds up the concrete of all pilotis. | Dieselbe Methode läuft für jedes Vorkommen und summiert den Beton aller Pilotis. |
| 8 | `inheritance` | Inheritance: IfcColumn receives GlobalId, name, placement, representation and tag from its supertypes. | Vererbung: IfcColumn erhält GlobalId, Namen, Platzierung, Darstellung und Tag von seinen Oberklassen. |
| 9 | `composition` | Composition means parts: the south wall has an opening, a window that fills it, and a material layer set. | Komposition bedeutet Teile: Die Südwand hat eine Öffnung, ein Fenster darin und einen Schichtenaufbau. |
| 10 | `distinction` | Keep is-a and has-a apart: the window fills the wall's opening, but a piloti is a built element. | Trenne ist-ein und hat-ein: Das Fenster füllt die Öffnung der Wand, aber die Stütze ist ein Bauteil. |
| 11 | `exchange` | IFC exchanges declarative data and relationships. The volume method stays in the application. | IFC tauscht deklarative Daten und Beziehungen aus. Die Volumenmethode bleibt in der Anwendung. |

Lengths 84–112 characters (en), at most two caption lines. Word choices checked against `research-ifc.md` §14/§15: IFC 4.3 names (`IfcBuiltElement`, never `IfcBuildingElement`), "opening" chain wall → `IfcOpeningElement` → window, no "LoadBearing is always TRUE", GlobalId never called UUID. "Pilotis" in German is used as collective noun; singular is "Pilotis-Stütze" / "Stütze".

## 2. Stage grid, constants, persistent objects

Stage between title chip (top-left, bottom edge y ≈ 3.3) and caption (top edge y ≈ -2.66): **x ∈ [-6.8, 6.8], y ∈ [-2.55, 2.58]**. Every group of every beat is passed through `fit_band(group)` (energy toolkit, `SAFE_*`). Font floor: nothing smaller than the toolkit label size.

| Region | Box (x, y) | Use |
| --- | --- | --- |
| LP left panel | x ∈ [-6.8, -2.7], centre x = -4.75, width 4.0–4.1 | concept panels: schema card, class tree, instance/type card, property table, method readouts, sum card, has-a diagram |
| RR right region | x ∈ [-2.4, 6.8], centre (2.35, -0.1) | the villa drawing of that beat |
| MID | x ∈ [-2.4, 3.2] | piloti detail (beats 5–7) |
| MINI | centre (5.0, 1.2), width ≈ 3.0, height ≈ 2.5 | small axonometric that keeps the lit piloti in view while the detail is big |

Constants at the top of `present_objects()` (all derived, none typed numbers):

```text
model     = load_model()                                  # one model object passed to every call
SEL       = "B3"                                          # chosen story piloti (tag exists in model; piloti in row B, column 3)
geo       = building_geometry(model)
sel_geo   = next(p for p in geo["pilotis"] if p["tag"] == SEL)
piloti_t  = next(t for t in model["types"] if t["class"] == "IfcColumnType")      # type id for type_occurrences / quantities
occ       = type_occurrences(piloti_t["id"], model)       # 25 tags, row-major A1..E5
q_sel     = quantities(SEL, model)                        # cross_section_area, height, volume
q_all     = quantities(piloti_t["id"], model)             # count, volume
slab_tag  = next(s["tag"] for s in geo["slabs"] if s["kind"] == "FLOOR")
wall_tag  = "W-S"                                         # first-floor south façade wall; opening/window tags from geo["walls"][…]["openings"][0]
CLASS_COLOR = {"IfcColumn": STRUCT, "IfcSlab": CONCRETE, "IfcWall": ARCH, "IfcWindow": GLASS}
```

Colour roles used consistently: **DATA** = the one selected instance / the value being read; **STRUCT** = pilotis (class IfcColumn); **CONCRETE** = slab and every concrete fill; **ARCH** = walls; **GLASS** = windows; **GOOD** = methods / behavior and "is-a" ✓; **WARN** = `assumed` chip and "has-a ≠ is-a" ✗; **MUTED** = inherited rows, scale labels.

Numbers: all strings come from `self.number(value, digits)` (en `0.30`, de `0,30`) with units; counts are integers from `len(occ)` / `q_all["count"]`.

## 3. Beats

Common rule: one `self.step(key, <animations>, seconds=…)` per beat. Sequences inside a beat are built with `Succession` / `LaggedStart`; `seconds` is the total motion time (3–6.5 s), `step` holds the rest of the reading time. Continuous motions are `ValueTracker`s with `linear` rate, readouts are `always_redraw`/updaters reading the tracker.

### Beat 1 · `schema` — "schema defines vocabulary and rules; for buildings: IFC 4.3"

- **Left (LP)**: `class_box(model["schema_id"], rows, point=(-4.75, 1.2), color=GOOD, width=4.0)`; header `IFC4X3_ADD2` (from `model["schema_id"]`), sub line `IFC 4.3 · ISO 16739-1:2024`. Rows (label bilingual, value IFC names): `Vocabulary/Begriffe → IfcColumn · IfcSlab · IfcWall · IfcWindow`, `Rules/Regeln → every IfcRoot has a GlobalId / jedes IfcRoot hat eine GlobalId`, `Relationships/Beziehungen → IfcRelVoidsElement · IfcRelFillsElement`.
- **Under it**: `badge(…, point=(-4.75, -1.5), color=MUTED)` two lines: `Villa Savoye · Poissy · 1928–1931` (from `model["building"]`) and `teaching model: simplified, rounded, partly assumed / Lehrmodell: vereinfacht, gerundet, teils angenommen`. This is the **only** "teaching model" mention.
- **Right (RR)**: `villa_axon(point=(2.35, -0.1), scale=0.14, model=model)`; target bounding box ≤ 5.6 × 4.6 (build with ground floor dark green and set back, white box, ribbon band, ramp, roof screens, all 25 pilotis visible: ground-floor enclosure rendered light so every piloti reads).
- **Animation (`seconds=6.0`)**: card `Create` + rows `LaggedStart` (1.6 s) while the axon builds ground → box → roof (`LaggedStartMap(FadeIn)`, 2.2 s); then each row flashes its examples in the axon: row 1 vocabulary → `Indicate` four families one after the other in class colours (pilotis STRUCT, slab CONCRETE, walls ARCH, windows GLASS; parts from `geo` tags), row 2 rules → pulse the `GlobalId` text in the card, row 3 relationships → pulse wall + ribbon window together (0.5 s each).
- **Values by meaning**: the schema id and the building name/years come from the model; the family flashes show that "vocabulary" is a set of kinds of parts of this very building.

### Beat 2 · `class` — a class is a kind; four kinds of built element

- **Left**: schema card fades out (0.4 s). `class_tree` replaces it: the true chain from `class_chain("IfcColumn")` (`IfcRoot`, `IfcObjectDefinition`, `IfcObject`, `IfcProduct`, `IfcElement`, `IfcBuiltElement`) as a vertical spine at x = -4.75, y = 2.25, 1.7, 1.15, 0.6, 0.05, -0.5 (chip w 2.6, spacing 0.55, arrows downward), then four leaf chips under `IfcBuiltElement` in a 2 × 2 grid: `IfcColumn` (-5.75, -1.35), `IfcSlab` (-3.75, -1.35), `IfcWall` (-5.75, -1.95), `IfcWindow` (-3.75, -1.95), each w 1.9, outlined in its `CLASS_COLOR`, fan lines from `IfcBuiltElement`.
- **Right**: axon stays; every part is neutral grey-white at start.
- **Animation (`seconds=6.0`)**: spine chips `LaggedStart` top to bottom (1.5 s); then per leaf (0.9 s each) the chip appears and **all parts of that class light up in its colour at once**: `IfcColumn` → 25 pilotis (STRUCT), `IfcSlab` → floor + roof slab (CONCRETE), `IfcWall` → first-floor walls and roof screens (ARCH), `IfcWindow` → ribbon windows (GLASS); the previous family stays tinted. End: `Indicate` the `IfcColumn` chip in DATA.
- **Functions**: `class_chain` for the four leaves (the first six levels are shared, assert equality); part tags from `geo["pilotis"]`, `geo["slabs"]`, `geo["walls"]`, `geo["walls"][i]["openings"]`, `geo["screens"]`.
- **Values by meaning**: no number; "kind" = every lit part of the same colour.

### Beat 3 · `instance` — one object with its own GlobalId

- **Left**: the highlighted `IfcColumn` chip `ReplacementTransform`s into the header of the instance card `class_box("B3 : IfcColumn", rows, point=(-4.75, 1.0), color=DATA, width=4.0)`; rows `GlobalId` (value `element(SEL)["guid"]`, monospace, 22 characters), `Name` (`element(SEL)["name"][lang]`), `Tag` (`SEL`). The rest of the tree fades (0.6 s).
- **Right**: axon: the other tinted families fade back to neutral; the 24 other pilotis stay faintly STRUCT; **B3 lights in DATA** with a pulse ring (`Indicate` + `ShowPassingFlash` circle at its base) and a thin leader (`leader(card_row_globalid, axon.parts[SEL])`) from the GlobalId row to the piloti.
- **Animation (`seconds=3.8`)**: 0.7 s tint reset + transform, 1.1 s card rows, 1.2 s pulse and leader.
- **Values by meaning**: the GlobalId (22 characters) is attached by a line to exactly one lit piloti of the drawing.

### Beat 4 · `occurrences` — one type, all pilotis are occurrences · ValueTracker `count`

- **Left**: instance card shrinks and moves to `(-4.75, -0.3)` as a compact chip `B3 : IfcColumn`; above it the type card appears `class_box("IfcColumnType · " + type name, rows, point=(-4.75, 1.45), color=STRUCT, width=4.0)` with shared rows `Profile: circle/Profil: Kreis`, `Material: reinforced concrete/Stahlbeton` (from the type), `Pset_ColumnCommon.LoadBearing: TRUE` (from the type's pset). A down arrow from card to chip labelled `IfcRelDefinesByType` (small, MUTED). Below the chip a big counter at `(-4.75, -1.5)`: `n / N` with `N = len(occ)`, label `occurrences/Vorkommen`.
- **Right**: the axon `FadeOut`s (0.5 s) and the ground-floor plan `villa_plan(ground_storey_id, point=(2.35, -0.1), scale=0.2, model=model, layers=("structure",), grid=True, labels=True)` fades in (0.5 s): grid axes 1–5 / A–E, 25 piloti circles drawn at min radius (toolkit), all dim outlines at start; small label `Ground-floor plan / Grundriss Erdgeschoss` under it (MUTED).
- **Animation (`seconds=5.5`)**: `count = ValueTracker(0)`, `count.animate.set_value(N)` with `rate_func=linear`. An updater on every piloti `i` (order `occ`, row-major A1 … E5) sets fill opacity = `clip(count - i, 0, 1)` and a size pop `1 + 0.8·sin(π·clip(count - i, 0, 1))`; lit pilotis are STRUCT, **B3 is DATA from the start** and pops when its turn comes. A moving tag label (`always_redraw` next to the piloti currently lighting) reads `A1 … E5`. The counter text is `int(count)` (and `N`) from the tracker. After the run: `Indicate(B3)` ring.
- **Values by meaning**: the count 25 is the number of lit circles on the 5 × 5 grid; tag labels follow the grid labels.

### Beat 5 · `attributes` — Ø and height as dimension lines · ValueTracker `grow`

- **Transition**: plan, type card, chip and counter fade (0.6 s). The axon returns small as MINI: `villa_axon(point=(5.0, 1.2), scale=0.085)` with B3 lit in DATA and a dashed DATA leader from B3 to the detail.
- **Left**: `property_table(rows, point=(-4.75, 1.4), width=4.0)`, columns `name | value | basis`: `Diameter Ø (type)/Durchmesser Ø (Typ)` · value · chip, `Height (this piloti)/Höhe (diese Stütze)` · value · chip, `Material` · value from model · (no chip). Values via `information(SEL, "diameter", lang)` and `information(SEL, "height", lang)` → `{value, unit, basis}`; the `basis` becomes a chip `assumed/angenommen` (WARN) or `sourced/belegt` (GOOD). A MUTED footnote under the table: `assumed = teaching assumption, not surveyed / angenommen = Lehrannahme, nicht vermessen` (research fact 2.12: real Ø not published).
- **Middle**: `piloti_detail(point=(0.0, -0.05), scale=0.62, model=model, tag=SEL, dims=True, grow=g, fill=0, area_fill=0)` inside `always_redraw`: elevation with ground hatch below, slab above, level marks `±0.00` and the slab level from `building_geometry()["storeys"]`; an enlarged section circle (own scale ×4) at `(1.6, 0.3)` with centre mark and poché; Ø dimension under the circle, height dimension left of the elevation. In both, the label shows the value.
- **Animation (`seconds=4.5`)**: `grow = ValueTracker(0)` → 1 (`linear`): Ø line spans the circle while the circle grows, the height line rises while the column grows; the table values count up with `grow` (`number(grow·value)`). Then the basis chips `FadeIn` + `Indicate` WARN (0.6 s).
- **Values by meaning**: Ø = length of the dimension line across the section; height = the dimension line along the elevation; "assumed" is visible next to both numbers.

### Beat 6 · `methods` — the application computes; the concrete fills · ValueTrackers `a_fill`, `fill`

- **Left**: the table shrinks to two rows at the top (Ø, h; compact, y 2.3–1.4); below, two method rows at `(-4.75, 0.7)` and `(-4.75, -0.1)` built like the energy clips with `math_readout`: pill `badge("cross_section_area()", GOOD)` + `A = π · (Ø/2)² = {a} m²`, pill `badge("volume()", GOOD)` + `V = A · h = {a} · {h} = {v} m³`; inputs Ø and h printed in DATA (same colour as their dimension lines), results in CONCRETE. Header chip `Application / Anwendung` (GOOD) above the two rows.
- **Middle**: same `piloti_detail` (no re-grow).
- **Animation (`seconds=5.2`, `Succession`)**: (1) `a_fill` 0 → 1 (1.5 s): the section circle fills radially with concrete poché while `A` counts up to `q_sel["cross_section_area"]`; (2) `fill` 0 → 1 (3.2 s, linear): concrete rises inside the elevation from base to top with a fill-level line, `V` counts up to `q_sel["volume"]`; (3) 0.5 s `Indicate` result.
- **Functions**: `quantities(SEL)` → `cross_section_area`, `height`, `volume`; Ø, h from `information`. Nothing typed.
- **Values by meaning**: A = the poché area of the circle; V = the filled cylinder; the formula shows which attribute enters where.

### Beat 7 · `total` — same method for all occurrences · ValueTracker `fill_all`

- **Transition (0.9 s)**: the detail shrinks into B3's position in the MINI axon (`animate.scale(0.15).move_to(...)` + fade) while the MINI axon scales up to full (`point=(2.35, -0.1)`, scale 0.14); left table and formulas fade.
- **Right**: all 25 pilotis in the axon are drawn as slim cylinders that can be filled; `fill_all = ValueTracker(0)` fills **all at once** from base to top in CONCRETE (updater on `axon.parts[tag]`).
- **Left**: sum card at `(-4.75, 1.2)`: `N × v = V_total` as `math_readout`: `{N} × {v} m³ = {fill_all · total} m³` with `N = q_all["count"]`, `v = q_sel["volume"]`, `total = q_all["volume"]` (check equal to `N·v`, assert in the scene's debug run, not on screen). Below it, two bars (max width 3.6, scaled by value, count up with the same tracker): `all pilotis/alle Pilotis` = `q_all["volume"]` (CONCRETE-STRUCT mix) and `first-floor slab/Geschossdecke` = `quantities(slab_tag)["volume"]` (CONCRETE) with their m³ labels; the slab flashes in the axon (`Indicate`, CONCRETE) when its bar appears.
- **Animation (`seconds=6.0`)**: 0.9 s transition, 3.5 s fill (linear) with the counter, 1.6 s bars + slab flash.
- **Values by meaning**: total concrete = 25 filled columns at the same time in the drawing; the slab bar puts the number in proportion to the floor above.

### Beat 8 · `inheritance` — rows are copied from every supertype · ValueTracker `slide` (optional)

- **Transition (0.8 s)**: axon fades; left panel clears; plan fades in right: `villa_plan(ground_storey_id, point=(4.4, -0.1), scale=0.19, highlight=(SEL,), layers=("structure",))`, B3 lit in DATA, others dim.
- **Left/middle (x ∈ [-6.8, 0.1])**: spine `class_tree(class_chain("IfcColumn"), point=(-5.55, 0.1), highlight="IfcColumn")` with 7 chips (w 2.3) at y = 2.2, 1.5, 0.8, 0.1, -0.6, -1.3, -2.0 (the leaf-less chain). To its right the box `class_box("IfcColumn", rows=[], point=(-1.95, 0.15), width=4.2)` (x ∈ [-4.05, 0.15], y ∈ [-1.75, 2.05], nine rows of height 0.38 reserved). Each row is stamped with a colour dot and a small MUTED origin name (`IfcRoot`, `IfcProduct`…).
- **Animation (`seconds=6.2`)**: for each class in the chain in turn (`IfcRoot` → `IfcColumn`, 0.6 s each): its chip pulses, then its `adds` rows are `TransformFromCopy`-ed from the chip into the box (rows slide out of the chip and stack in order): `GlobalId, OwnerHistory, Name, Description | ObjectType | ObjectPlacement, Representation | Tag | PredefinedType`. Levels with empty `adds` (`IfcObjectDefinition`, `IfcBuiltElement`) pulse without rows. Then (1.8 s) three leaders link box rows to the plan (DATA): `ObjectPlacement` → two dimension lines from the grid origin to B3 (`dimension(...)` x-offset `sel_geo["x"]` and y-offset `sel_geo["y"]`, labels from `sel_geo`), `Tag` → grid label `B / 3` pulses, `GlobalId` → `guid_chip(SEL, point)` next to the lit piloti.
- **Values by meaning**: the inherited row `ObjectPlacement` is the two drawn offsets; `Tag` is the grid label of that circle; GlobalId is the chip on it.

### Beat 9 · `composition` — wall has opening, window, layer set · ValueTracker `reveal`

- **Transition (0.8 s)**: plan, tree and box fade. Right: `villa_elevation("south", point=(2.6, 0.55), scale=0.32, model=model)` (≈ 6.9 × 3.0; ground-floor set-back dark green with pilotis, white box, ribbon band, roof screens, ground line).
- **Left (has-a diagram, top to bottom)**: `class_box("IfcWall · W-S", …, point=(-4.75, 2.0), color=ARCH, w 3.6)`; below it a diamond-ended link labelled `IfcRelVoidsElement` to chip `IfcOpeningElement` (y 1.0, DATA); a link labelled `IfcRelFillsElement` from the opening to chip `IfcWindow` (y 0.1, GLASS); a link labelled `IfcRelAssociatesMaterial` from the wall to `class_box("IfcMaterialLayerSet", rows=[render, brick infill, plaster] with thickness + chip, point=(-4.75, -1.4), w 3.6)`. Relation names appear exactly as in `relations_of(wall_tag)`; opening/window tags from `geo["walls"]`.
- **Bottom right**: layer detail strip, horizontal cut through the wall: three hatched layers (render / brick infill / plaster; names bilingual, thickness from the layer set) at scale 8 u/m around `(4.4, -1.9)`, with `dimension` lines per layer and the total, `assumed` chip; label `section through wall / Schnitt durch die Wand`. Callout leader from the wall end in the elevation.
- **Animation (`seconds=6.5`, `Succession`)**: (1) wall band `W-S` tints ARCH, wall box + relation line `FadeIn` (1.0 s); (2) the ribbon opening outlines DATA, the wall tint is cut out of it (hole shows the dark stage), hatch of the opening area fills left → right with `reveal` 0 → 1 (1.6 s) and the readout `opening/Öffnung {opening_area} m²  of/von {gross_side_area} m²` counts (from `quantities(wall_tag)`), opening chip + `IfcRelVoidsElement` appear; (3) glass fills the hole (`FadeIn` GLASS 0.8 s), window chip + `IfcRelFillsElement` appear; (4) layer strip slides out of the wall end (`TransformFromCopy`), layer rows appear with thickness count-up (1.6 s).
- **Values by meaning**: the opening area is the hatch cut in the wall elevation; layer thicknesses are the widths of the hatched strips in the section.
- **IFC modelling choice stated on screen once** (MUTED footnote bottom left, `y = -2.4`): `ribbon = one IfcWindow in one IfcOpeningElement / Bandfenster = ein IfcWindow in einer IfcOpeningElement`.

### Beat 10 · `distinction` — is-a vs has-a

- **Right**: the elevation stays, layer strip and callout fade. During the beat: pilotis `Indicate` STRUCT while the is-a half speaks, ribbon window `Indicate` GLASS inside the wall band while the has-a half speaks.
- **Left**: has-a diagram morphs (`ReplacementTransform`) into two stacked mini diagrams.
  - **is-a** (top, GOOD, y ∈ [0.7, 2.4]): `IfcBuiltElement` chip (y 2.1) ← hollow-triangle arrow ← `IfcColumn` chip (y 1.0, STRUCT); sentence below `A piloti is a built element. ✓ / Eine Stütze ist ein Bauteil. ✓` with a green check.
  - **has-a** (bottom, DATA/WARN, y ∈ [-2.4, -0.1]): `IfcWall` (x -5.7, y -0.9, ARCH) and `IfcWindow` (x -3.6, y -0.9, GLASS) joined by a diamond link labelled `IfcRelVoidsElement + IfcRelFillsElement` (via the opening); above them a dashed hollow-triangle arrow `IfcWindow → IfcWall` labelled `is-a?` that is struck out by `cross_mark(WARN)` and fades; sentence `A window is not a wall: it fills an opening in it. ✗ / Ein Fenster ist keine Wand: Es füllt eine Öffnung darin. ✗`.
- **Animation (`seconds=5.5`)**: 1.3 s morph to is-a half + pilotis `Indicate`; 2.4 s has-a link grows, the wrong arrow draws dashed, `cross_mark` pops, arrow fades; 0.8 s final `Indicate` of both sentences.
- **Values by meaning**: no number; the contrast is carried by the drawing (window inside wall band vs piloti under the box).

### Beat 11 · `exchange` — data travels, the method stays · ValueTracker `fill_b`

- **Transition (0.8 s)**: elevation and left diagram fade.
- **Top (full width)**: `step_panel(step_excerpt(SEL, model), point=(0.0, 1.55), width=12.4, highlight_roles=("guid", "class", "tag", "predefined", "property"))`, a subset of at most five lines of the excerpt (IFCCOLUMN line, local placement, body representation, `IFCRELDEFINESBYTYPE` if present, one property line), tokens coloured by role; local `#n` numbers in MUTED. Heading chip `IFC4X3_ADD2 · STEP (ISO 10303-21)`.
- **Bottom left** (x ∈ [-6.6, -2.6], y ∈ [-2.4, -0.1]): app box `Authoring application / Planungsprogramm`: mini piloti (`piloti_detail(scale=0.3, fill=1)`), `guid_chip(SEL)`, pill `volume()` (GOOD) with `V = {q_sel["volume"]} m³`.
- **Bottom right** (mirror): app box `Quantity take-off application / Mengenermittlungsprogramm`: empty mini piloti, empty `guid_chip` slot, pill `volume()` (GOOD, greyed).
- **Bottom centre** (x ∈ [-2.3, 2.3]): two chips: `IFC carries: class · values · relationships / IFC trägt: Klasse · Werte · Beziehungen` (GOOD) and `Result may be stored (Qto_ColumnBaseQuantities), the code never / Ergebnis darf gespeichert sein (Qto_ColumnBaseQuantities), der Programmcode nie` (MUTED, two lines).
- **Animation (`seconds=6.5`, `Succession`)**: (1) panel lines `LaggedStart` (1.2 s); (2) tokens (GlobalId, class, tag) fly as small chips from the panel into the right app box (`flow`/`Succession` along `smooth_path`, 1.4 s) while the `volume()` pill of A tries to leave and stops at the box edge with `cross_mark` (0.8 s); (3) in B the piloti appears with the same GlobalId chip (`guid_chip(SEL)`), `volume()` turns GOOD and **runs**: `fill_b` 0 → 1 fills B's mini piloti with concrete while `V` counts up to `q_sel["volume"]` (2.4 s); (4) `=` check between both V values (0.7 s).
- **Values by meaning**: the same GlobalId on both sides identifies the same object; the same V is *recomputed* by B's own method (concrete fill in B), not copied.

## 4. Timing summary (s)

| beat | motion | beat | motion |
| --- | --- | --- | --- |
| schema | 6.0 | inheritance | 6.2 |
| class | 6.0 | composition | 6.5 |
| instance | 3.8 | distinction | 5.5 |
| occurrences | 5.5 | exchange | 6.5 |
| attributes | 4.5 | total | 6.0 |
| methods | 5.2 | | |

Reading-time floors in `hold_for` extend short motions; no beat is longer than its motion budget by design. Implementation keeps expensive `always_redraw` only on the piloti detail (small), the readouts and the pilotis' updaters; the axon is built once.

## 5. Value-to-meaning table

| Value | Source | Seen at the same time as |
| --- | --- | --- |
| schema id, building name, years | `model["schema_id"]`, `model["building"]` | axon of that building |
| GlobalId, name, tag | `element("B3")` | lit piloti + leader; `guid_chip` on plan; same chip in app B |
| occurrence count N | `len(type_occurrences(...))` = `quantities(type)["count"]` | 25 lit circles, tag following the grid |
| Ø, height + basis | `information("B3", "diameter"|"height", lang)` | dimension lines on section circle and elevation; `assumed` chip |
| A, V per piloti | `quantities("B3")` | radial poché fill, concrete rising in the cylinder |
| total V, per-type count | `quantities(type)` | 25 cylinders filled at once; bar against slab volume |
| slab V | `quantities(slab_tag)["volume"]` | bar + slab flash in axon |
| placement | `building_geometry()["pilotis"]` entry of B3 | two dimension lines on the plan from the grid origin |
| opening area / gross side area | `quantities("W-S")` | opening hatch in the south elevation |
| layer names, thickness, basis | wall type material layer set | hatched strips with dimension lines |
| STEP tokens | `step_excerpt("B3")` | role colours on the panel, chips flying into app B |

## 6. What this storyboard needs from the model and toolkit agents (please confirm or tell me the final names)

Model (`C/🧱️model/🐍️.py`, `🔣️.json`):
1. `information(tag, property_id, language)` with property ids `diameter`, `height`, `material`, `load_bearing` for pilotis and `{value, unit, basis}`; `quantities("B3")` keys `cross_section_area`, `height`, `volume`; `quantities(type_id)` keys `count`, `volume`.
2. Type lookup: `model["types"]` items carry `id`, `class` (`IfcColumnType`), `name {en,de}`, `material`, `psets`.
3. `class_chain` for `IfcColumn`, `IfcSlab`, `IfcWall`, `IfcWindow` with identical first six levels.
4. Wall `W-S` with `quantities` keys `gross_side_area`, `opening_area`, `net_side_area`; `relations_of("W-S")` returning the three relation classes with the other side's tag; the wall type's material is an `IfcMaterialLayerSet` with `layers: [{name {en,de}, thickness {value, unit, basis}}]` (render, brick infill, plaster).
5. `building_geometry()["pilotis"]` entries contain `x`, `y`, `base`, `top`; slabs have `kind` and `tag`; storeys keyed by id in elevation order.
6. `step_excerpt` roles at least `guid, class, name, tag, predefined, placement, representation`; lines short enough to show five at width 12.4.

Toolkit (`C/🎨️visuals/**`):
1. `villa_axon` / `villa_plan` / `villa_elevation`: `parts[tag]` for every piloti, slab, wall, opening/window and screen, recolourable by `set_fill`/`set_stroke`; neutral tint default; pilotis drawn so they stay visible at scale 0.14 (min stroke/radius), ground-floor enclosure light so no piloti is hidden. If `B3` is occluded in the final view the scene switches `SEL` to the nearest visible piloti and keeps the story.
2. `piloti_detail(point, scale, *, model, tag, dims, grow=1.0, area_fill=1.0, fill=1.0)` with `grow` scaling Ø and height together (values in labels follow), separate `area_fill` for the circle and `fill` for the cylinder, and `parts` named `elevation`, `section`, `dim_diameter`, `dim_height`. If missing, the scene draws the detail from `Rectangle`, `Circle`, `dimension(...)` itself.
3. `class_tree(chain, point, *, highlight, leaves=())` with addressable chips (`parts[class_name]`) and fan lines to optional leaves; `class_box(name, rows, ...)` with addressable rows for `TransformFromCopy`; `property_table` with a third `basis` chip column (WARN/GOOD); `step_panel(..., width=12.4)` supports a `lines=` subset; `badge`, `guid_chip`, `leader`, `dimension`, `hatch`, `level_mark` as in `api.md`.
4. Optional `wall_layers(point, scale, *, model, tag)`: hatched strips with dimensions; otherwise drawn in the scene.

## 7. Fact check list for the audit (IFC 4.3, per `research-ifc.md`)

- ISO 16739-1:2024, IFC 4.3.2.0 = `IFC4X3_ADD2` [P]; `IfcBuiltElement` instead of `IfcBuildingElement` [P].
- Chain `IfcRoot → IfcObjectDefinition → IfcObject → IfcProduct → IfcElement → IfcBuiltElement → IfcColumn` and the attribute split (GlobalId, OwnerHistory, Name, Description | ObjectType | ObjectPlacement, Representation | Tag | PredefinedType) [M]: confirm against the official pages before publication.
- `IfcColumnType`, `IfcRelDefinesByType`, `Pset_ColumnCommon.LoadBearing` [P/M]; Ø attributed to the type and height to the occurrence is a teaching simplification (type profile vs occurrence extrusion): keep wording "type"/"this piloti".
- `Qto_ColumnBaseQuantities` row names (`CrossSectionArea`, `Length`, `GrossVolume`) [M]: only the set name appears on screen (beat 11).
- Wall → `IfcOpeningElement` via `IfcRelVoidsElement`, window via `IfcRelFillsElement`, material via `IfcRelAssociatesMaterial` / `IfcMaterialLayerSet` [M]; one `IfcWindow` per ribbon is a modelling choice and labelled as such.
- Villa claims: only the building name, Poissy, 1928–1931 and the teaching-model badge. The piloti count (25), Ø, heights, wall layers and ribbon window sizes are model values, partly `assumed`; the sources disagree on 15 vs 25 pilotis and on which façades carry a ribbon window, so narration never states them and the badge says "teaching model".
