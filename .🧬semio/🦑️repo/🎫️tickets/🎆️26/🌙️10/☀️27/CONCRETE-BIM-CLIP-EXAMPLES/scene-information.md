# Scene 📊️information (`Beat1_DataToInformation`)

## Round 2

Scene file: `C/📊️information/🐍️.py` (rewritten). Captured en and de with `capture.py`; all 21 keyframes per language read.

### Changes against `audit-r2-information.md`

- Numbers (N-01, N-02, N-03, N-04, N-05, N-06, N-07): no turning or glass-wall radius is printed any more; the turning circle and the glass wall are drawn and labelled with symbols only (`R_glass = R_turn − b_car − c`) plus the chips "principle sourced" and "radii not sourced". The piloti count is not stated: the relations row says "count open" and the model note `piloti-count` (25 versus 15) is shown below it. Diameter, car size, safety margin and height (`3.25 m = 3.50 − 0.25`) carry "assumed"; LoadBearing carries "inferred" and material "sourced" (structural system, VS-3.1). The verdict chip reads "fits · assumed" in the assumption colour. The 0.60 m weak-source diameter and a 2.00 m car are shown as a sensitivity run.
- Abstract (A-01, A-02, A-03, A-04, A-05): the glass radius is drawn from the centre dot as three coloured segments; every dimension has a label; the handoff has an elbow trunk from the piloti ring on the plan to the IFCCOLUMN token, the LoadBearing token and the GlobalId chip; each relation row has a coloured leader to storey (box outline), type (all pilotis ringed), slab (hatch) and grid (axes E and 3, bubble 3); the context axonometric has labelled leaders to E/3 and E/4. The car is a period car (long bonnet, separate wings, running boards).
- Sync (S-01 to S-07): `raw` shows only the bare number; the plan appears in `context`; the first-grid ghost dimension (5.00 m) stays visible in `source`; the swept band no longer covers the bay; relation targets stay lit.
- Layout and depth (L-*, D-*): subtitle chip "Villa Savoye · Ground floor", top-right "IFC 4.3 · ISO 16739-1:2024" with the one-time teaching-model note, poché plan with scale bar, north arrow and level marks, typeset formulas with colour-coded symbols and substituted numbers, a spacing gauge with threshold, bays animated by ValueTrackers (spacing, diameter, car width, drive, running sum).
- German: one IFC name ("Piloti E3", English in both languages), one minus sign, one heading case.
- Shared additions (Edit only, with tests): `clearance_limits` and `path_pinches` in `C/🧱️model/🐍️.py`, `BimClearanceLimits` in `C/🧪️tests/🐍️.py` (bisection and NumPy oracles).

### Final beat list (21 clauses, en and de, no digits in captions)

raw, guess, context, place, chain, meaning, source, question, diameter, clear, fits, reduced, gauge, sensitive, drive, pinch, wall, properties, relations, handoff, same.

| key | visible change |
| --- | --- |
| raw | bare number 4.75 |
| guess | unit candidates m, kg, kN, °C |
| context | plan with poché, scale, north; number lands on the E/3–E/4 dimension; axonometric with both pilotis |
| place | IfcSite, IfcBuilding, IfcBuildingStorey, grid axes |
| chain | dimension chain 1.25, 4 × 4.75, 1.25; running sum 19.00 m; 21.50 m box; hatched open remainder |
| meaning | property table: property, value, unit, grid, basis |
| source | provenance of the grid value; dimension slides from 5.00 to 4.75 with ghost |
| question | car with dimensions, assumed chips, "?" in the gap |
| diameter | magnified bay: axis-to-axis span and Ø |
| clear | hatched clear width, `s − Ø = w` with numbers |
| fits | car, margins, keep-out zones, `b + 2m ≤ w`, verdict |
| reduced | spacing 4.75 → 5.00 → 4.75 on the live bay and gauge |
| gauge | threshold grows in; spacing slides below it and back |
| sensitive | Ø 0.60 m and car 2.00 m; threshold moves |
| drive | car drives the model path, swept band, distance counter |
| pinch | two closest approaches (E/3, D/2) magnified |
| wall | glass-wall radius split into glass radius, clearance and car width |
| properties | piloti elevation and section with numbered property table |
| relations | four leaders from rows to the plan |
| handoff | people read the drawing, applications read STEP |
| same | GlobalId trunk and taps link the drawn piloti to STEP tokens |

### Remaining risks and strict findings

- Strict check: both languages report 4 findings, all judged false positives after zoomed crops: `5.00 m` ghost dimension label (its own extension lines and the halo rings touch the label's enclosing shape), `1.35` margin labels (the tick ends of their own dimension lines come within a pixel), `0.24 m` vignette label (vignette frame and car wing bounding boxes; the label stands clear in the frame). The `H−`/`H=` unreadable-text lines come from the shared math typesetter's measurement probes.
- The straight-through test (`spacing − Ø` against car width and margins) is the teaching principle; the swept path shows real pinch points (0.42 m at E/3, 0.24 m at D/2).
- Sources 2 and 3 for the grid value (Cité sheet, Designing Buildings) were only read as snippets and are labelled so.
- Model JSON bases (material assumed, LoadBearing sourced) are overridden in the scene display only; the model owner should align them.
- The capture shows end states; the animated transitions (sliders, drive) were not rendered as video.

## Round 3

Answers `verify-r2-information.md`. Re-captured en and de (21 keyframes each, all read).

- V-B1 piloti count: the pilotis ringed or linked on screen are only the bay pair E/3, E/4 and the two path pinches. Where a count appears it reads "25 in teaching model*" with the legend "* sources differ: 15–25" (coordinator decision), never as a historical fact. The "25 pilotis … assumption" paragraph is gone; the model note `piloti-count` now says the real number is open.
- V-B2 car: narration and labels say "a car (size assumed)" / "ein Auto (Maße angenommen)"; no claim about the Savoyes' own car.
- V-B3 units: every dimension, margin, width, chain term, formula term and level carries `m` (grid bubbles and the gauge scale numerals excepted). Typeset units use a visible space.
- V-M1: the "Connected to" leader ends at a CONCRETE connector square on the E/3 piloti head; the type leader ends on the E/4 ring; the storey leader ends on the box corner.
- V-M2 margin rule: the 0.50 m side margin is the straight-through rule (`fits` narration says "Driving straight through"). `pinch` draws the dashed 0.50 m margin ring around each piloti, shows 0.42 m and 0.24 m inside it, labels them "below the 0.50 m margin" and the narration says the car still clears them "but only just".
- V-M3 basis: every chip comes from the model. Model edits: LoadBearing of TY-PILOTI basis `inferred` (source VS-3.1), schema basis enum and the basis test accept `inferred`, case `B3` updated, `BASIS_NAMES` gains `inferred`. Material stays `assumed` per the coordinator. Box length keeps the model basis `sourced` with the sub-line "medium confidence, one source".
- V-M4: the two remainder strips carry rotated "1.25 m" labels; both totals are drawn as a bar (19.00 m grid, 21.50 m box with a labelled dimension) beside the table; "split assumed" is stated in the row.
- V-M5: poché overlay on the ground walls (solid bands), solid cut pilotis, section mark A–A through E/3 with heads and view arrows, and the "A–A" label on the column elevation.
- V-m1 trunk moved to y 2.34; V-m2 remainder leader re-routed away from the north arrow; V-m3 gauge has metre ticks 0 to 5 m; V-m4 the weak case is Ø 0.60 m with car 2.05 m so the displayed margins add up (1.05 m); V-m5 "assumed" under both first Ø labels; V-m6 `meaning` sub-line explains the basis; V-m7 count status moved into the row chip; V-m8 the drive draws a dashed centreline with a tick every 5 m; V-m9 the table row "Turning circle: radius not sourced" matches the dashed circle's colour and the wall detail carries the "radii not sourced" chip (no extra tag on the plan); V-m10 unit spacing unified; V-m11 class name `IfcGrid` dropped ("grid axes E and 3"); V-m12 "+3.50 m" in the assumption colour next to "3.25 m = 3.50 m − 0.25 m · assumed"; V-m14 note `piloti-diameter` names the weak student slideshow.
- Not done: V-m13 (animated height in `properties`) is optional per the verifier; the sensitivity beat already animates Ø and car width.
- Strict check: both languages report only touch-only findings judged false positives on zoomed crops: `5.00 m` ghost label, `1.35/1.05 m` margin labels (they sit exactly between their extension lines), `0.24 m` vignette label, `1.25 m` strip labels (de), and probe text `H−`/`H=`.

## Round 4

Answers the 8 minors of `final-verify-information-objects.md`. Re-captured en and de; changed frames read.

- I-m1: relations chip reads "25 in this teaching model*" / "25 in diesem Lehrmodell*" with the legend "* sources differ: 15–25".
- I-m2: type name is the single IFC name "Piloti" in both languages; German slab value "Decke über dem Erdgeschoss".
- I-m3: building box length, width and height now carry the model basis `inferred` (single source of medium confidence, source VS-2.1 / VS-2.2 kept); the chain chip reads "inferred" / "gefolgert" with the sub-line "medium confidence, one source". Test `test_sourced_facts_match_the_research` expects `inferred` for the box. The representations clip only stars `assumed`, so it is unaffected.
- I-m4: `pinch` narration names the rule: "closer than the straight-driving margin" / "unterschreiten den Abstand der Geradeausfahrt".
- I-m5: the drive adds a ruler under the legend that grows with the drive, with ticks and numerals 0 to 25 m (the path ticks on the plan are the same 5 m ticks).
- I-m6: German verdict chip "passt · angenommen".
- I-m7: `diameter` shows the legend "s: spacing axis to axis · Ø: piloti diameter" in the empty lower panel (it fades when the formulas appear); `drive` fills the lower panel with the ruler.
- I-m8: IFC identifiers (Ifc…, Pset_…, GlobalId) in tables and chips use the toolkit's bundled monospace face (`self.mono`, Share Tech Mono) where capital I and lowercase l differ; the GlobalId chip head and the "same GlobalId" label are set in it too. The narration caption stays in the shared display font (toolkit owner).
- Tests: information presentation tests 14/14 OK; model tests touching sourced facts, quantities, information, schema, basis, clearance and bilingual content 21/21 OK (the full model suite was last run in round 3, with the one `repo`-missing environment error).
- Strict check: only the known touch-only findings remain (`5.00 m` ghost label, `1.35/1.05 m` margin labels, `0.24 m` vignette label, `1.25 m` strip labels in de, probe text).
