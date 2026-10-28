# Scene objects (`Beat2_ObjectOrientation`)

## Round 2

Scope: `🧬️objects/🐍️.py` rewritten against `audit-r2-objects.md`; test case keys in `🧪️tests/🎭️presentation/🔣️.cases.json` updated; one additive test in `🧪️tests/🎭️presentation/🐍️.py`. No shared toolkit or model file was changed.

### Audit defects fixed

- LAY-1 / SYN-4: elevation coordinates fixed (`v(x, z) = view(x, 0, z)`); wall band, opening, window and the hatch reveal now draw. Opening and window are parts of the elevation and the hatch is a live group (a bare `VMobject` returned from a `Live` builder draws nothing).
- SYN-1 / ABS-1: class tree is the IFC chain plus a bus line with four indented leaves (no crossing fans), counts per class from the model elements, four distinct colours (IfcColumn tan, IfcSlab salmon, IfcWall teal, IfcWindow green); the same colours tint the axonometric.
- NUM-1 to NUM-3, NUM-7: count, diameter and height carry a basis chip (`assumed` / `angenommen`) and a one-line reason; no storey elevation is printed; "Render (white)" becomes "Render" through `short_name`; layer thicknesses carry one `assumed` badge.
- NUM-4 / NUM-5: every product on screen is exact: `A = π · (0.15 m)² = 0.07069 m²`, `V = 0.07069 m² · 3.25 m = 0.2297 m³`, `25 × 0.2297 m³ = 5.7425 m³ ≈ 5.74 m³`; each value comes from `_chain` (area at 5 digits, volume at 4 digits from the displayed area). Test `test_objects_displayed_arithmetic_holds_and_matches_the_model` checks the chain for Ø 0.30 to 0.40 m and against `quantities` of the model.
- NUM-6 / DEP-3: slab volume shown as `380.42 m² × 0.25 m = 95.105 m³` (thickness `assumed`) and two bars on one scale.
- DE-1 to DE-7: `±0,00` through `self.number`; German title case, occurrence note, "Südfassade · Lehrannahme", is-a/has-a sentence, one word per object ("Stütze" singular, "Pilotis" plural).
- LAY-2 to LAY-18: inheritance rows readable, property table wider (German basis words fit), placement dimensions drawn outside the plan on the grid axes, one leader instead of five, STEP excerpt tokens underlined instead of boxed, exchange boxes laid out afresh, no stray baseline.
- DEP-1, DEP-4, DEP-5: poché hatches for the section circle, slab and wall layers; ground hatch and ±0,00; one axonometric style (x-ray with the lit piloti); heading chip per act and the source tag `IFC 4.3 · ISO 16739-1:2024`.

### Final beat list (19 clauses, each with a visible change)

1. `schema`: card + villa; rows flash the four families, 25 sparks for "every object has a GlobalId" (60 objects = 60 GlobalIds), wall and window flash for the relationships.
2. `class`: tree with counts; every class tints all its parts.
3. `instance`: B3 lit, card, GlobalId chip, two other pilotis with their own GlobalIds.
4. `occurrences`: type card, `IfcRelDefinesByType`, ground plan with 25 pilotis lighting one by one, counter with basis chip.
5. `pset` (new): `Pset_ColumnCommon` table with basis per value; B3's tributary area hatched on the plan (15.53 m², from `tributary()`).
6. `representations` (new): Axis, FootPrint, Body switch on the same piloti (ValueTracker).
7. `geometry`: r and h as dimension lines on elevation and enlarged section, basis chips.
8. `methods`: substituted formulas, poché circle, concrete rising with a `V =` level label.
9. `total`: 25 pilotis fill one after the other, `k × v = …` live, bars against the slab.
10. `change` (new): type diameter 0.30 → 0.40 → 0.30 m on a ValueTracker; circle, A, V, total, bar and the stroke of every piloti follow; GlobalId chip stays.
11. `inheritance`: seven supertype chips; attribute rows slide into the `IfcColumn` box.
12. `values` (new): every row gets its value; GlobalId leader, placement dimensions on the grid axes, tag via grid bubbles, three empty rows flagged.
13. `spatial` (new): site → building → storeys, contained elements, section with tinted storey bands (no level numbers).
14. `opening`: elevation, hatched opening, `18.50 m × 1.10 m = 20.35 m²`, wall `21.50 m × 3.00 m = 64.50 m²`.
15. `window`: glass fills the opening, glazing share 31.6 %.
16. `layers`: hatched layer strip, `0.03 m + 0.24 m + 0.03 m = 0.30 m`.
17. `distinction`: is-a / has-a with the opening chip, class names written on the drawn parts.
18. `exchange`: STEP lines (GlobalId, class, radius, height, LoadBearing) underlined and tied to the drawn piloti; relation chips.
19. `recompute`: B receives the values, runs its own `volume()`, concrete fills, same V; code arrow blocked.

### Playback traps found (relevant to every scene)

- Manim adds the mobject of a non-introducer animation (`Indicate`, `UpdateFromAlphaFunc`) at the start of the whole `play`, not when its turn comes: content shows early and sits under frames faded in later. The scene uses `_pulse` / `_appear` with `introducer=True` and `FadeIn(…, target_position=…)` for slides.
- `_ramp` must read its start value when the animation begins (chained ramps on one tracker otherwise restart from the construction-time value).
- A `Live` builder must return a `VGroup`, never a bare `VMobject`.
- `TransformFromCopy` of text left the copies under the target frame (dark rows); replaced by a slide.
- `_arrow` clashed with the `Analysis` mixin; renamed `_pointer`. No other clash (`objects-r2-clash.py`).

### Verification

- `objects-r2-capture.py` (copy of `capture.py` with its own output folder, because another process wrote the same `frames/Beat2_ObjectOrientation` folder concurrently): all 19 frames in en and de, layout report and strict block 0 findings. The end-state `change` frame shows the undone 0.30 m; the 0.40 m state was captured once with `objects-r2-capture-change.py` (`25 × 0.4084 m³ = 10.2100 m³ ≈ 10.21 m³`).
- `SEMIO_TEACHING_OUTPUT_ROOT=… python 🧪️tests/🎭️presentation/🐍️.py` (objects topic): 12 tests OK, including the new arithmetic test.
- `SEMIO_TEACHING_OUTPUT_ROOT=… python 🧪️tests/🐍️.py`: 138 tests, one error `test_source_discovery_skips_browser_runtime` (`repo` binary not found on this host, outside this scene).

### Remaining risks

- Motion is verified only through end states and the three-point lifecycle test; the ramps, the diameter change and the slides were reasoned through, not watched in a full render.
- The 25-piloti count, Ø 0.30 m, storey heights, slab thickness, wall layers and ribbon size are teaching assumptions from the model (badges shown); the ribbon windows on all four façades are a model assumption that the wall beats do not repeat (south façade only on screen).
- IFC claims marked [M] in `research-ifc.md` (direct supertype `IfcBuiltElement` of `IfcColumn` and `IfcWindow`, `IfcRelContainedInSpatialStructure`, `IfcMaterialLayerSetUsage` on walls, `Qto_ColumnBaseQuantities`) still need confirmation against 4.3.2.0 before the final render.
- Capital I and lowercase l look alike in the shared font.
- The section widget in `spatial` draws the ramp zigzag in the cut; it is part of the toolkit drawing.

## Round 3

Source: `verify-r2-objects.md` (0 blocker, 7 major, 18 minor). Scene file only; no shared file touched.

### Decisions

- **Pset binding: type level.** `Pset_ColumnCommon` hangs on `IfcColumnType` (`IfcColumnType.HasPropertySets → Pset_ColumnCommon`), shown in the pset beat (table title "Pset_ColumnCommon · IfcColumnType") and the exchange chip. The exchange excerpt no longer shows `#118 IFCRELDEFINESBYPROPERTIES (…(#101)…)` (occurrence binding); it shows `#120 IFCRELDEFINESBYTYPE`, `#115 IFCPROPERTYSINGLEVALUE('LoadBearing')`, `#107`, `#108` and `#101`. For 🔌️exchange to match: `IFCCOLUMNTYPE` (`#119`) needs `HasPropertySets = (#117)` and the property set must not also be related to `#101`.
- **Count:** "25 in this teaching model*" with legend "* sources differ: 15–25" (occurrences, instance, total); no other 25 headline.
- **Tributary area is an occurrence value:** "occurrence value, B3 alone; all pilotis: 8.61–22.56 m²" (min/max from `tributary()`).

### Fixed

- MAJ-1: tributary text rewritten, hatch captioned as slab above projected. MAJ-2: values narration "six inherited attributes have values … three stay empty" (en, de). MAJ-3: see above. MAJ-4: the GlobalId leader now runs along the grid and ends on the B3 dot. MAJ-5: opening, wall, glazing and dimension labels carry `*` and a footnote "* teaching-model assumption: ribbon size, wall height". MAJ-6: see above. MAJ-7: load path drawn in the pset beat: slab load arrows on SL-1, B3, reaction 151.46 kN* (from `tributary()`), arrow into the foundation, label "LoadBearing TRUE".
- MIN-1 "+ 24 other objects" line in the class beat; MIN-2 student-slide values as MUTED footnote; MIN-5 three GlobalId lines (B3, SL-1, W-S) stay in the schema beat; MIN-6 GUID token linked by an elbow leader to the GlobalId chip in the exchange beat (element line moved to the bottom); MIN-7 storey chips two-line (class above name), arrows labelled `IfcRelAggregates`; MIN-9 instance GlobalId chip stands beside B3 with a short leader; MIN-10 "ribbon windows: extent disputed" in the schema badge; MIN-11 "solarium screen" label; MIN-12 own 4.75 m dimension strings (no clipped 1.25 m); MIN-13 "Grundriss EG"; MIN-14 wrapped German line; MIN-15 "0.2297 m³ = 0.2297 m³ ✓"; MIN-16 caption "hatched: slab above, projected"; MIN-17 drawn labels "Achse" / "Grundfläche"; MIN-18 distinction heading "Is-a and has-a". Class-tree bus line now drawn.

### Not changed, with reason

- MIN-3 [M] IFC claims: needs the standards pages (403 here); still listed as a risk.
- MIN-4 one axon style: x-ray (instance) and ghosted structure (total) are needed to show the lit piloti and the filled pilotis.
- MIN-8 capital I / lowercase l: shared toolkit font.
- MIN-11 ramp zigzag in the `spatial` section: part of the toolkit section drawing.
- MIN-17 "Piloti-Stütze" in the German type card and element name: model data; narration says "Stütze".

### Verification

`objects-r2-capture.py` en and de: 19 frames each, strict block 0 findings, all changed frames read (pset, schema, occurrences, instance, class, values, exchange, recompute, spatial, geometry, distinction, representations, opening, window, total, both languages where text differs). Presentation tests (objects topic): 14 OK.

## Round 4

Source: `final-verify-information-objects.md` (objects: 3 major, 7 minor). Scene file only.

- O-M1: class leaf "25 pilotis*" / "25 Pilotis*" with the legend "* in this teaching model; sources differ: 15–25" under the tree (tree pitch tightened so the "+ 24 other objects" line and the legend fit).
- O-M2: one rule, "area at three digits, product from the printed area": the pset beat prints `15.534 m²` (counted up), `q = 9.75 kN/m² = 6.25 + 1.50 + 2.00` with the names (self-weight + finishes + imposed; from `tributary()["load_breakdown"]`), and `15.534 m² × 9.75 kN/m² = 151.46 kN *`; the reaction label in the load drawing uses the same printed value (`round(round(area, 3) · q, 2)`, equal to the model's 151.46 kN).
- O-M3: the inheritance beat carries the villa: an x-ray axonometric with B3 ringed beside the class chain and the IfcColumn attribute box, a leader from the box to B3 and the label `B3 : IfcColumn`, with a flash when the last attribute has landed; the `values` beat then moves to the plan.
- O-m1 basis chips: type card row "profile, material: assumed"; method formulas "inputs r, h: assumed". O-m2: the 0.40 m state (25 × 0.4084 m³ = 10.2100 m³ ≈ 10.21 m³) is in the frame set as `…-en-change-040.png` and `…-de-change-040.png` (captured by `objects-r2-capture-change.py`). O-m3: German placement "(10,75; 4,75) m". O-m4: German "Annahme des Lehrmodells" in all three places. O-m5: the section beat names what else is cut ("ramp (zigzag), solarium screen") in the left panel and labels the solarium screen on the drawing (a label on the zigzag collided with the toolkit linework). O-m6 (I/l glyphs) and O-m7 stay: shared font, wording already aligned.
- Verification: `objects-r2-capture.py` en and de, strict block 0 findings in 19 + 19 frames; changed frames read (class, occurrences, pset, inheritance, change 0.40 m); presentation tests for objects: 14 OK.
