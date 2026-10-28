# Storyboard: Clip 🧩️models (`Beat4_PartialModels`, video id `bim_models`)

Title: en "Models Share a Schema" / de "Modelle Teilen Ein Schema". Scene file `C/🧩️models/🐍️.py`. Facts: `research-villa-savoye.md`, `research-ifc.md` (sections 10, 11, 14, 15). API names: `api.md` section 2 (model) and 3 (toolkit); requests the storyboard adds are in "Requests" at the end.

## Idea in one paragraph

One building, three kinds of knowledge. The Villa Savoye is shown whole, then split into the three models that different disciplines keep of it: architecture (walls with ribbon windows, curved glass wall, ramp, roof screens, finishes), structure (25 pilotis, slabs, ramp) and services (boiler, radiators under the salon ribbon window, roof drains and downpipes). A count matrix shows the three models use the same IFC classes with different content. Federation lowers the three layers onto one origin and grid (ValueTracker `explode` 1 → 0) and ties objects by GlobalId. A coordination check finds a downpipe too close to a piloti. Then the structure model is translated into a structural analysis model: slab to surface members, pilotis to line members on the same grid; assumptions add nodes, material, supports, loads; the losses are listed next to the elements they came from.

Journey of the picture: whole villa (beat 1) → one layer at a time (2–4) → three thumbnails and a class matrix (5) → one exploded villa that closes up (6) → plan and zoomed detail (7) → structure axon morphing into the analysis view (8–10) → villa again with the lost elements marked (11). Beat 1 and beat 6 end on the same federated villa picture on purpose: the parts are the whole.

## Conventions used below

- Stage: x in [-6.8, 6.8], y in [-2.5, 2.85] (title chip above y 3.0, two-line caption below y -2.5). Drawing zone L = x in [-6.6, -0.2]; concept zone R = x in [0.4, 6.7], centre x = 3.55, width 6.0.
- `AX` = (-3.2, 0.15), `S1` = 0.16 screen units per metre: a full axon is about 5.6 wide and 4.7 high (estimate: width 35.2·s, height 29.7·s). `S2` = 0.11 (exploded villa), `S3` = 0.12 (translation/analysis pair), `S4` = 0.045 (thumbnail, 1.6 × 1.3). The implementer fits sizes by measuring `drawing.width/height` and capturing frames.
- Colours (toolkit tokens): `ARCH`, `STRUCT`, `SERV` per discipline; `DATA` highlighted value; `WARN` problem or loss; `GOOD` agreement; `INK`, `MUTED` text. Analysis view uses `STRUCT` for members and `DATA` for nodes/supports.
- Row = `[DecimalNumber count | IFC class name | short label]`, text size 21–24, row pitch 0.45. A count is always a `ValueTracker` that runs from 0 to the model value while the elements it counts light up on the drawing in the same animation (the number is the number of lit parts).
- Names of objects follow `api.md`: tags `A1`…`E5` pilotis, `W-S`… walls, `RW-S`… windows, slabs/ramp/curtain wall/boiler/radiator/drain/downpipe tags come from `element()` lookups by class over `partial_model(id)["element_guids"]`. The scene never types a number; classes are typed text (real IFC 4.3 names).
- "Teaching model" is said once, in beat 1 (top-right label, below).
- `self.step(key, *animations, seconds)` plays all animations of the beat together. Multi-phase beats use `Succession` / `LaggedStart` inside the one step. The end state of the step is the keyframe `capture.py` photographs, so each beat below lists its end state and it must be complete and uncluttered.

## Narration (final text; `NARRATION` rows `[key, english, german]`)

| # | key | English | Deutsch |
| --- | --- | --- | --- |
| 1 | whole | Different disciplines organize the building information they need. | Verschiedene Fachrichtungen ordnen die Bauwerksinformation, die sie brauchen. |
| 2 | architecture | The architecture model holds walls, ribbon windows, the ramp, the curved glass wall and the spaces. | Das Architekturmodell enthält Wände, Fensterbänder, die Rampe, die gebogene Glaswand und die Räume. |
| 3 | structure | The structure model holds the pilotis, slabs and ramp. It leaves out render and windows. | Das Tragwerksmodell enthält Pilotis, Decken und Rampe. Putz und Fenster lässt es weg. |
| 4 | services | The services model holds boiler, radiators and roof drains: the same house, other objects. | Das Haustechnikmodell enthält Kessel, Heizkörper und Dachabläufe: dasselbe Haus, andere Objekte. |
| 5 | schema | Three models, one schema: the same vocabulary and rules, with different content. | Drei Modelle, ein Schema: gleiche Begriffe und Regeln, aber anderer Inhalt. |
| 6 | federation | Coordination overlays the models on one origin. GlobalIds link them; each keeps its author. | Die Koordination legt alle Modelle auf einen Ursprung. GlobalIds verbinden sie, jedes behält seinen Urheber. |
| 7 | check | A coordination check finds what no single model shows: a downpipe too close to a piloti. | Eine Koordinationsprüfung findet, was kein Einzelmodell zeigt: ein Fallrohr zu nah an einer Pilotistütze. |
| 8 | translation | For one use case, the structure model is translated into another schema: structural analysis. | Für einen Anwendungsfall wird das Tragwerksmodell in ein anderes Schema übersetzt: die Tragwerksanalyse. |
| 9 | members | The first-floor slab becomes surface members; each piloti becomes a line member on the same grid. | Die Obergeschossdecke wird zu Flächenelementen, jede Pilotistütze zu einem Stab auf demselben Raster. |
| 10 | assumptions | Assumptions add what was never drawn: nodes, material, supports and loads. | Annahmen ergänzen, was nie gezeichnet wurde: Knoten, Material, Lagerung und Lasten. |
| 11 | loss | Ribbon windows, infill walls, render colour and curved glass are not in the analysis. Record the losses. | Fensterbänder, Ausfachungen, Putzfarbe und gebogenes Glas fehlen in der Analyse. Halte die Verluste fest. |

Truthfulness notes for the narration: "ribbon windows" are modelled as `IfcWindow` filling an `IfcOpeningElement` (research 14.8); the structure model "leaves out" windows and render because analysis does not need them, not because the building lacked them; IDS/BCF/ISO 19650 are not named in clauses (see beat 7 for the only BCF mention, which follows research section 11: BCF refers to elements by GlobalId); revisions are integers shown as "rev N", never UK S0..S4 codes (not global ISO); no IFC relationship name is claimed between physical element and analytical member (research 10 and 15.14), the screen shows "source GlobalId" only.

## Beat 1 · whole

Idea: the building as one thing; three people ask three different questions of it.

- Left, `AX`, `S1`: `villa_axon(AX, S1, disciplines=("architecture","structure","services"), explode=0.0)` with the discipline colours (the same picture beat 6 ends on). Draw order: pilotis and slabs first (Create), then walls with ribbon windows, ramp and screens, last services.
- Top right, two lines at (3.55, 2.7) and (3.55, 2.4), size 19, MUTED: `building["name"]` + "Poissy" + `building` years ("Villa Savoye, Poissy · 1928–1931"), then the single "teaching model" mark: en "Teaching model: simplified, rounded values" / de "Lehrmodell: vereinfachte, gerundete Werte".
- Right, three discipline rows (width 5.9, height 1.1) at y = 1.55, 0.2, -1.15; each row has a colour bar in `ARCH` / `STRUCT` / `SERV`, the discipline name (size 28, colour) and the question it asks of the villa (size 21, INK):
  - en "Architecture: where are rooms, walls and windows?" / de "Architektur: Wo sind Räume, Wände und Fenster?"
  - en "Structure: what carries the loads to the ground?" / de "Tragwerk: Was trägt die Lasten in den Boden?"
  - en "Services: where do heat and rain water go?" / de "Haustechnik: Wohin gehen Wärme und Regenwasser?"
- Bottom of the right zone at (3.55, -2.15): a count readout "N objects in three models" (N = `len(set(arch.element_guids) | set(structure.element_guids) | set(services.element_guids))`, ValueTracker 0 → N, size 22).
- Animation (seconds 3.5): axon `Create` 0–1.2 s; `LaggedStart` of the three rows with lag 0.9 s; as each row appears, `Indicate(axon.layers[discipline], color=ROW_COLOR)` pulses the matching layer of the drawing (the question is tied to its drawn answer at the same time); the counter runs with the rows.
- End state: villa in three colours, three question rows, counter at N, teaching-model label.
- Values by meaning: N lit object count; discipline colour = who owns which parts.

## Beat 2 · architecture

Idea: architecture keeps what you see, how it is made and which rooms exist.

- Transition: `FadeOut(axon.layers["structure"], axon.layers["services"])`; the architecture layer stays on screen at `AX`, `S1`, and the drawing is rebuilt only if the toolkit cannot keep a layer (no rebuild preferred, same `Mobject`s).
- Right top: `model_card("architecture", (3.55, 2.05), model=…)` (colour bar `ARCH`, name, author, `rev N`, schema chip `IFC4X3_ADD2`, object total; values from `partial_model("architecture")`).
- Class rows at y = 0.95, 0.5, 0.05, -0.4, count right-aligned at x = 1.2, class at x = 1.55, label at x = 3.4:
  1. count of `IfcWall` / "IfcWall" / en "façade and roof-screen walls" de "Fassaden- und Dachwände" → lights `W-S`… and screens
  2. `IfcWindow` / "ribbon windows" "Fensterbänder" → lights `RW-S`…
  3. `IfcCurtainWall` / "curved glass wall" "gebogene Glaswand" → lights the ground-floor curve
  4. `IfcRamp` / "ramp" "Rampe" → lights the ramp
  counts = `partial_model("architecture")["counts_by_class"]`; each row lights its tags in `DATA` while counting, then settles back to `ARCH`; a thin leader line (toolkit `leader`) from the row to one lit element stays for the selected element only to avoid clutter.
- Material strip, y = -1.55: title at y = -1.1 (size 18, MUTED) en "Infill wall: material layers" de "Ausfachungswand: Materialschichten". A horizontal section strip from the wall type's layer set (render / brick infill / plaster, names from the model, thickness per layer from the model) drawn at 10 screen units per metre, x in [1.0, 1.0 + 10·total], band colours distinct; total thickness as `dimension(start, end, f"{number} m")` below the strip (y = -2.0). Layer names listed at x = 4.5, y = -1.2, -1.55, -1.9 (size 17) with colour swatches.
- Animation (seconds 4.5): `LaggedStart` of card, four rows (each row = count tracker + `Indicate` of its drawn parts, lag 0.5 s) and finally the strip growing left to right (`Create` bands in order) with the dimension.
- End state: architecture layer coloured and clean, card, four counted rows, material strip with dimension.
- Values by meaning: counts by lit elements; layer thicknesses as band widths; total wall thickness as dimension line.

## Beat 3 · structure

Idea: structure keeps only what carries load and leaves out what does not matter for it.

- Transition: the architecture layer dims to 12 % opacity and stays as a ghost under the structure layer (so the audience sees what is missing); the structure layer was part of the first axon and fades in at 100 % (`structure` was removed in beat 2, so it is re-added: `FadeIn(axon_struct)`; either re-use the faded layer or build `villa_axon(AX, S1, disciplines=("structure",))` once, whichever the toolkit supports).
- Card `model_card("structure", (3.55, 2.05))`.
- Rows at y = 0.95, 0.5, 0.05: `IfcColumn` ("piloti" "Pilotis") lit one after another (25 pilotis `A1`…`E5`, `LaggedStart` lag 1/25 so the count tracker follows the number of lit pilotis exactly); `IfcSlab` ("slabs" "Decken": first floor and roof); `IfcRamp` ("ramp" "Rampe"). Counts from `partial_model("structure")["counts_by_class"]`.
- Property chip at y = -0.6: `information("C3", "<LoadBearing property id>", language)` rendered as "Pset_ColumnCommon.LoadBearing = TRUE" (value and unit text from the model, property id as defined by the model agent), leader to the highlighted piloti `C3` (the centre piloti of the 5 × 5 grid) in `DATA`.
- "Not in this model" block (WARN): header at y = -1.3 (size 18, MUTED) en "Not in this model" de "Nicht in diesem Modell"; line at y = -1.7 (size 21, WARN) listing the classes of the architecture model that have count 0 here, with their architecture counts (`IfcWindow ×n · IfcCurtainWall ×n`) and "render" "Putz"; the ghost windows of the architecture layer flash `WARN` once.
- Animation (seconds 4.5): ghost + structure + card (0–1.0 s), pilotis count (1.0–3.0 s, tracker linear), slab + ramp rows, chip with leader (3.0–4.0 s), omission block (4.0–4.5 s).
- End state: structure in full colour with the faint architecture ghost, three rows, LoadBearing chip pointing at a piloti, omission line.
- Values by meaning: 25 pilotis lit one by one; LoadBearing TRUE on the piloti; omitted counts flashed on the ghost windows.

## Beat 4 · services

Idea: services use the same house, but they need different objects and their own geometry.

- Transition: structure fades to ghost (12 %) next to the architecture ghost? Keep only the architecture ghost (12 %, context for walls and windows); structure layer fades out; services layer `FadeIn` at `AX`.
- Card `model_card("services", (3.55, 2.05))`.
- Rows at y = 0.95, 0.5, 0.05, -0.4: `IfcBoiler` (boiler "Heizkessel", ground floor) → boiler lit with a short warm pulse; `IfcSpaceHeater` (radiators "Heizkörper") → radiators lit, and the salon ribbon window `RW-S` of the architecture ghost brightens at the same time so the radiator can be seen "under the ribbon window" (leader from the radiator row to both); `IfcWasteTerminal` ("roof drains" "Dachabläufe") → drains lit; `IfcPipeSegment` ("downpipes" "Fallrohre") → pipes drawn from the drain down to the ground floor (`Create` along the `building_geometry()["services"]` polyline points).
- Water motion: after the pipes appear, one drop per drain runs down its pipe (`MoveAlongPath` / `always_redraw` on a `ValueTracker` 0 → 1 over `building_geometry()["services"]` points, positions through `axon_point`) so the pipe is read as the way the roof garden is drained.
- Roof garden meaning: hatch the roof garden outline in the axon (area from `building_geometry()["spaces"]`, the `terrace` space, value "110 m²") with its area as label (size 19, `SERV`) on the hatch, next to the drains it is drained through.
- Note, y = -1.35 to -1.85 (size 19, MUTED), en "The Savoyes' roof leaked every autumn: drainage is a services concern." de "Das Dach der Savoyes war jeden Herbst undicht: Entwässerung ist ein Haustechnik-Thema." (fact 8.1, high; no cause is claimed).
- Model note (once, small): radiator positions are teaching-model positions; do not add another "teaching model" mark.
- Animation (seconds 4.5): boiler, radiators, drains lit with counters (0–2.5 s), pipes `Create` and the drops (2.5–4.0 s), note fades in (4.0–4.5 s).
- End state: services layer in colour over the architecture ghost, four counted rows, hatched roof garden with area label, drops on the pipes, note.
- Values by meaning: counts by lit objects; garden area as hatch; the leak story as moving water in the real pipe.

## Beat 5 · schema

Idea: the three models speak the same language. A class × model count matrix makes "same vocabulary, different content" visible and keeps the villa on screen as three thumbnails.

- Transition: big drawing leaves; three thumbnails fade in at (-2.1, 1.5), (0.7, 1.5), (3.5, 1.5): `villa_axon(point, S4, disciplines=(d,))` for architecture, structure, services (1.6 × 1.3 each, their colours).
- Top bar at y = 2.7 spanning x in [-6.4, 6.4] (height 0.46): schema chip `IFC4X3_ADD2` (`model["schema_id"]`, colour GOOD) with the text en "one schema: same classes, same rules" de "ein Schema: gleiche Klassen, gleiche Regeln".
- Column headers under the thumbnails at y = 0.62: model name (size 20, colour) + `rev N` (from `partial_model(id)["revision"]`).
- Matrix rows at y = 0.2, -0.16, -0.52, -0.88, -1.24, -1.6; row labels right-aligned at x = -3.4 (left edge -6.8): `IfcColumn`, `IfcSlab`, `IfcWall`, `IfcWindow`, `IfcSpaceHeater`, `IfcPipeSegment` (typed names, real IFC 4.3; replace by the model's classes if any is missing from every `counts_by_class`); cells at x = -2.1, 0.7, 3.5 hold the count from `partial_model(id)["counts_by_class"].get(class, 0)`, tinted in the model colour (opacity .25 + .5·n/max in row), a zero shows "·" in MUTED.
- Footer at y = -2.05 (size 20): en "Shared rules: every object has GlobalId, Name and placement (IfcRoot, IfcProduct)" de "Gemeinsame Regeln: jedes Objekt hat GlobalId, Name und Platzierung (IfcRoot, IfcProduct)"; the first part from `class_chain("IfcColumn")[0]["adds"]`.
- Animation (seconds 4.0): row labels and top bar first (`FadeIn`), then the three columns in sequence (thumbnail fades up from 20 % to 100 %, its counts count up from 0 in the same 0.9 s with a tracker per cell), finally the footer. The pattern of filled and empty cells is the lesson: all rows exist in all columns; the content differs.
- End state: matrix with distinct column patterns, three thumbnails, schema bar, footer.
- Values by meaning: each count shown as tinted cell plus its thumbnail layer; the same row labels for all columns show a single vocabulary.

## Beat 6 · federation (ValueTracker explode 1 → 0)

Idea: the layers come from different authors. Federation puts them on one shared origin and grid; GlobalIds say which objects are the same thing; revisions say which state was federated.

- Transition: thumbnails and matrix leave; the exploded villa appears. `explode = ValueTracker(1.0)`; `always_redraw(lambda: villa_axon((-3.0, 0.0), S2, disciplines=("architecture","structure","services"), explode=explode.get_value(), grid=True))`. At `S2` = 0.11 the villa is 3.9 wide and about 3.3 high; the exploded gaps add about 1.8 high (architecture up, services down), total about 5.1: fits the stage.
- Shared origin and grid on the ground plane do not move with the explode: toolkit draws grid lines 1–5 / A–E and the origin marker (A1 corner, ±0.00) once at the plane `z = 0`.
- Right zone, three model rows (width 5.9, height 0.7) at y = 2.1, 1.35, 0.6: colour bar, name, `author`, `rev N` badge (from `partial_model`); each row pulses when the explode passes through its layer's final position (end of the animation).
- GlobalId link: the first-floor slab exists in the architecture and the structure models with the same GlobalId (`set(architecture.element_guids) & set(structure.element_guids)`). At (3.55, -0.45) a `guid_chip(<first-floor slab tag>)` (the 22-character GlobalId from the model) with text "same GlobalId in {k} models" k = number of models whose `element_guids` contain it; two leader lines (`always_redraw`) from the chip to the slab in the architecture layer and in the structure layer via `axon_point(x, y, z, point, S2, discipline=…, explode=explode.get_value())`. As `explode` falls to 0 the lines shorten to nothing and the two slab drawings coincide; at the end `Flash` on the point.
- Federation state at y = -1.35 (size 20): "Federated: architecture rev a · structure rev b · services rev c" (revisions from the models) with badge `badge("coordinated", GOOD)` after the merge.
- Statement at y = -1.95 (size 19, MUTED): en "Each model keeps its author and scope." de "Jedes Modell behält Urheber und Geltungsbereich." (matches the old clause; shown, not narrated, since the audio already says "each keeps its author").
- Animation (seconds 5.0): `FadeIn` exploded villa with the rows (0–1.0 s), then `explode.animate.set_value(0)` with `rate_func=smooth` over 3.0 s while leader lines shorten and rows light (by the time `explode` is below 0.3), flash and badge in the last second.
- End state: villa in three colours on one origin and grid (the same picture as beat 1), GlobalId chip with coinciding leader points, the three author/revision rows, federation state.
- Values by meaning: the explode value is the vertical gap between layers; the chip's two leader lengths equal the distance between the two slab copies and shrink to zero (the same GlobalId = the same object).

## Beat 7 · check

Idea: coordination means testing the layers against each other. One example on the villa: a downpipe runs too close to a piloti (or, if the model returns a different pair, whichever pair `clash_check` returns; the screen shows the returned tags).

- Left: `villa_plan(level, (-3.4, 0.1), 0.24, highlight=(a_tag, b_tag), dims=(), layers=("structure","services"), grid=True)` with `level` = the level of the check (ground floor), pilotis as circles, downpipe as a small circle, both highlighted `WARN` (plan about 5.2 × 4.6, x in [-6.0, -0.8]).
- Right: a magnifier inset: circle of radius 1.75 at (3.6, 0.55) joined to the clash point by two thin tangent lines. Inside, the two sections at 6 screen units per metre: piloti circle (radius from `building_geometry()["pilotis"]`, 0.15 m), pipe circle (pipe radius from the services element), the required-clearance ring (dashed circle around the pipe, radius = pipe radius + required clearance) and the actual gap as `dimension(...)` text from the check result. The ring overlaps the piloti: violation seen, not stated.
- Check card at (3.55, -1.35), width 5.9: line 1 (size 22): en "clash-01 · downpipe {a_tag} ↔ piloti {b_tag}" de "Konflikt-01 · Fallrohr {a_tag} ↔ Pilotistütze {b_tag}"; line 2 (size 22, WARN): en "clearance {c} m < required {r} m" de "Abstand {c} m < gefordert {r} m", numbers from `clash_check()` (`clearance`, `required`; "teaching rule" appears in the same line in MUTED: en "teaching rule" de "Lehrregel" — a rule, not "teaching model").
- Issue chip at (3.55, -2.15), size 19: en "Issue (BCF) points to both GlobalIds" de "Meldung (BCF) verweist auf beide GlobalIds" with two small `guid_chip`s hidden behind a collapse; BCF is named only here, as a communication format, not a model (research 11).
- Animation (seconds 4.0): plan with highlight `FadeIn` (0–1.0 s), inset circle grows from the clash point (`GrowFromPoint`, 1.0–2.0 s), piloti and pipe circles, dashed ring expanding to its radius (a ValueTracker 0 → required clearance drives the ring radius and the readout; 2.0–3.2 s), the dimension line draws, card and issue chip fade in (3.2–4.0 s).
- End state: plan with the highlighted pair, inset with ring-overlaps-piloti, dimension, card, issue chip.
- Values by meaning: clearance (m) as the dimension line between the two circles, required clearance as the dashed ring radius.

## Beat 8 · translation

Idea: one model, one purpose: translate to a different schema.

- Left: `villa_axon((-4.3, 0.1), S3, disciplines=("structure",))` (the structure model alone; 4.2 × 3.6, x in [-6.4, -2.2]); the pilotis and the first-floor slab glow in `STRUCT`.
- Middle column x in [-1.9, 1.9]: two mapping rules as rows (width 3.8, height 0.8) at y = 0.9 and -0.2, each a pair of class chips with an arrow:
  - `IfcColumn` → `IfcStructuralCurveMember`, count `n` (members of kind "curve" in `translate_structure()["members"]`)
  - `IfcSlab` → `IfcStructuralSurfaceMember`, count `m` (members of kind "surface")
  Each rule fades in together with the matching elements lighting up in the axon (pilotis in `DATA`, then first-floor slab in `DATA`) and a leader from the rule to one lit element.
- Right: dashed frame (x in [2.4, 6.6], y in [-1.9, 1.9]) titled with the target schema chip (`translation` source/target schema names from the model JSON; the chip text is the model's name); inside, empty for now.
- Top badge at (-4.3, 2.4): `badge("structure rev N", STRUCT)` with `N = translate_structure()["source_revision"]` and an arrow to the right frame, text "from rev N" (prepares the link in the consistency clip).
- Animation (seconds 3.5): structure axon `FadeIn` (0–0.8 s), the two rules in sequence (0.8–2.4 s, each lights the matching elements in the axon), arrow and frame (2.4–3.5 s).
- End state: structure axon with lit pilotis/slab, two class-mapping rules, empty dashed frame with schema chip.
- Values by meaning: counts n and m from the translation result are numbers of lit elements; rev N shows which state of the model was translated.

## Beat 9 · members

Idea: idealisation in front of the audience. The solids collapse into the lines and surfaces an analysis uses; pilotis keep their grid.

- Morph on the same position and scale: the structure axon at `AX`-centre of zone L (-3.0, 0.1), `S1`-ish `S3`..0.15 becomes `analysis_view(translate_structure(), (-3.0, 0.1), 0.15)`. First (0–1.5 s) the other elements fade, then `ReplacementTransform`: slab outline thickens to the surface and subdivides into shells on the 4.75 m grid; pilotis cylinders shrink to their axis lines; the slab thickness dimension (vertical dimension at the slab edge, value from the first-floor slab's thickness quantity, e.g. "t = 0.xx m") slides from the slab edge to a note on one shell as an attribute ("Thickness" is a parameter of the surface member, the geometry has none).
- Right zone R, three count rows at y = 1.2, 0.7, 0.2 (counts tracked; elements counted light up in the drawing during the count):
  - `IfcStructuralCurveMember` (pilotis) n lines in `STRUCT`
  - `IfcStructuralSurfaceMember` (first-floor slab) m shells in `STRUCT` with shell outline lit one by one
  - `IfcStructuralPointConnection` (nodes) k dots in `DATA`
  all counts from `translate_structure()` (`members`, `shells`, `nodes`).
- Reference chip at y = -0.5: `guid_chip("C3")` captioned en "source GlobalId C3" de "Quell-GlobalId C3", with a leader to the corresponding member (`analysis.parts["…"]`) in the drawing: each analysis member keeps the GlobalId of the physical object it came from. No IFC relationship name is claimed.
- Mapping note at y = -1.35 (size 20, MUTED): en "line ≠ column body, surface ≠ slab body" de "Linie ≠ Stützenkörper, Fläche ≠ Deckenkörper" (research 15.14: analytical geometry is not the physical solid).
- Animation (seconds 5.0): fade other elements, morph slab and pilotis (0–2.5 s, `ReplacementTransform`, `rate_func=smooth`), rows counting (2.5–4.0 s), chip + leader (4.0–5.0 s).
- End state: analysis view (shells on the grid, line members, nodes), three counted rows, source-GlobalId chip with leader, note.
- Values by meaning: counts equal lit members; thickness t moves from the slab edge to a shell attribute; the grid spacing of the shells equals the piloti spacing (a dimension "4.75 m" between two nodes from `clearance()["spacing"]`).

## Beat 10 · assumptions (second ValueTracker: load q)

Idea: the analysis model needs things the architect never drew. They are assumptions and are marked as such.

- Same analysis view at the same place (zone L). During this beat the following appear in the drawing, each paired with a row in zone R (size 21, items from `translate_structure()["assumptions"]` with `{en, de}` texts; assumed values marked by a small `badge("assumed", WARN)` using the model `basis`):
  1. nodes: the node dots pop at pilotis tops/bottoms and slab grid crossings (`DATA`), row "nodes" "Knoten"
  2. material: member colour fills with concrete tint, row text "reinforced concrete, 25 kN/m³" (value from `loads`), "Stahlbeton, 25 kN/m³"
  3. supports: support triangles grow at the piloti bases (`supports[i]["condition"]` text, e.g. "fixed base" "eingespannt"), row "supports" "Lagerung"
  4. loads: `load_arrows` on the first-floor slab: a `ValueTracker` q goes from 0 to the model's area load `tributary()["area_load"]` (kN/m²); arrow lengths follow q (`load_arrows(values, points, scale)`) while a live readout "q = {q} kN/m²" counts up next to the arrows (`always_redraw`). Arrow length and number rise together.
- Order: nodes → material → supports → loads, each 1 s; the matching row appears when its drawing feature starts.
- Animation (seconds 5.0): `Succession` of four phases (nodes pop, material tint, supports grow, `q.animate.set_value(q_model)` with load arrows).
- End state: analysis view with nodes, supports, load arrows at full length, four assumption rows each with `assumed` badges, load readout.
- Values by meaning: q as arrow length and number at the same time; support condition as the symbol; density as the tint-labelled material row.

## Beat 11 · loss

Idea: every translation drops information. Name it, next to the elements it came from.

- Left: `villa_axon((-4.0, 0.15), S3, disciplines=("architecture","structure"))`; the structure elements stay in `STRUCT`, the architecture elements named in `translate_structure()["losses"]` are in `WARN`: ribbon windows (`RW-*`), infill walls, wall render colour, the curved glass wall.
- Right: loss table at x in [0.4, 6.7], one row per distinct loss entry of `analysis["losses"]` (`source_tag`, `property`, `{en, de}`), at most five rows at y = 1.9, 1.15, 0.4, -0.35, -1.1 (size 20): a WARN chip with the source tag and the lost property, then a short text such as en "opening and glazing: not modelled" de "Öffnung und Verglasung: nicht modelliert". Rows come from the model; list order: windows, infill walls, render colour, curved glass wall.
- Each row, while appearing, draws a leader line to its element in the axon (WARN pulse). At the end all lost elements fade to ghost (opacity 12 %) leaving the structural skeleton: that is what the analysis sees.
- Footer at y = -1.95 (size 21): `badge`s en "{lost} properties lost · {kept} members kept" de "{lost} Eigenschaften verloren · {kept} Elemente behalten" with `lost = len(analysis["losses"])`, `kept = len(analysis["members"])` (counters 0 → value); and the line en "Translation is not automatically reversible." de "Eine Übersetzung ist nicht automatisch umkehrbar." (size 19, MUTED) as in the old clip.
- Animation (seconds 4.5): rows LaggedStart (lag 0.5 s) with leader and pulse, then lost elements fade to ghost (3.5–4.2 s), footer in.
- End state: the villa with ghosted architecture elements and bright structural skeleton, loss table, counters, reversibility line.
- Values by meaning: each lost property is shown beside the very element it came from; the two counters count the same items.

## ValueTrackers and `always_redraw` summary

| Tracker | Beat | Drives | Visible result |
| --- | --- | --- | --- |
| `count_*` (many) | 1–5, 9, 11 | class counts | the number equals the number of lit elements |
| `explode` 1 → 0 | 6 | `villa_axon(explode=…)` plus GlobalId leaders | layers drop onto one origin; leaders shrink to nothing |
| `drop` 0 → 1 per pipe | 4 | water drops along `building_geometry()["services"]` points | roof drainage as motion |
| `ring` 0 → required | 7 | dashed clearance ring radius and readout | the ring reaches the piloti |
| `q` 0 → area load | 10 | `load_arrows` length and "q = … kN/m²" readout | arrow length = load value |

## Teaching-model marking and facts check

- "Teaching model" appears once (beat 1). Values carry `basis: assumed` where the model says so; the assumption rows of beat 10 show "assumed" badges from the model's `basis`, not a second teaching-model mark.
- Historical statements on screen: Villa Savoye, Poissy, 1928–1931 (sourced), roof leaked every autumn (fact 8.1), ribbon windows, curved glass wall following the car's turning circle (not repeated here; beat 2 only shows the wall), roof garden 110 m² (fact 2.7, medium). No U-values, no insulation claims, no piloti count claim beyond the model's 25 (flagged as teaching choice by the model; the label is the model's, not typed here).
- IFC statements: classes from IFC 4.3 (`IfcColumn`, `IfcSlab`, `IfcWall`, `IfcWindow`, `IfcCurtainWall`, `IfcRamp`, `IfcBoiler`, `IfcSpaceHeater`, `IfcWasteTerminal`, `IfcPipeSegment`, `IfcStructuralCurveMember`, `IfcStructuralSurfaceMember`, `IfcStructuralPointConnection`), `IFC4X3_ADD2`. No UK status codes; no `IfcRelAssignsToProduct` claim. BCF as a communication format only.

## Requests to the toolkit and model agents (collected from this storyboard)

Toolkit:
1. `villa_axon(...)` result exposes `.layers[discipline]` (one `VGroup` per discipline, so it can be faded or dimmed alone) and `.parts[tag]` for every element and `ghost` opacity 12 %; `explode` shifts layers vertically (architecture up, structure none, services down), works under `always_redraw`.
2. `axon_point(x, y, z, point, scale, *, discipline=None, explode=0.0)` returns the screen position of a model coordinate, including a discipline's explode offset, so leaders and drops can be drawn in `always_redraw`.
3. `villa_axon(..., grid=True)` draws the ground-plane grid with labels `1…5` / `A…E` and an origin marker that do not move with `explode`.
4. `villa_plan(layers=("structure","services"))` supports the services layer (pipe circles) and `highlight` accepts tags of two different layers.
5. `analysis_view` uses the same projection and scale as `villa_axon` so a structure axon can morph into it in place; exposes `.parts[member_id]`, `.nodes`, `.supports`; `load_arrows` returns arrows whose length follows value × scale and can be used inside `always_redraw`.
6. `model_card(model_id, point, *, model=None)` exposes the card header (name, author, `rev N`, schema chip, total); class-count rows are composed by the scene.
7. Hatch on an arbitrary projected polygon in the axon (`hatch`) for the roof garden.

Model:
1. `clash_check(model=None) -> list[{id, a_tag, a_guid, b_tag, b_guid, clearance, required, point:[x,y,z], level, status}]` including at least one real violation of the teaching rule (the model's services layout must make a downpipe pass within the required clearance of a piloti; `basis` of `required` = assumed).
2. `translate_structure()` keys: `source_revision`, `members` (with `kind`, `ifc_class`, `source_guid`), `shells` (thickness), `nodes`, `supports` (`condition` as `{en, de}`), `loads` (area load, density), `assumptions` as `{en, de, basis}`, `losses` as `{source_tag, property, en, de}` with entries for ribbon windows, infill walls, render colour and the curved glass wall; model `translation` JSON holds `source_schema` and `target_schema` names.
3. `partial_model(id)` returns `name {en,de}`, `author`, `revision`, `element_guids`, `counts_by_class`; slab GlobalIds of architecture and structure are identical.
4. `quantities`/`element` for the wall type's layer set (names `{en, de}`, thickness in m) and for the roof garden space area.

## Implementation notes for the scene file

- `construct()` calls `self.present("models")`; `present_models()` holds the whole animation (the toolkit's `present` dispatches to `present_<topic>`). Keys in `NARRATION` stay as above; old keys `fem` is renamed `members`, `check` and `assumptions` are new.
- Small private helpers inside the scene class for the repeated pieces: `count_row(tracker, ifc_class, label, y, color)`, `discipline_row(...)`, `lit(tags, color)` (flash + settle), no comments inside bodies.
- Keep each `step` within the seconds budget above; narration holds extend the beat when the clause is longer than the animation.
- Capture both languages (`python T/capture.py 🧩️models Beat4_PartialModels en` and `de`), check `…-layout.md` for overlaps; the German strings in rows and chips are the longest, so test `de` first.
