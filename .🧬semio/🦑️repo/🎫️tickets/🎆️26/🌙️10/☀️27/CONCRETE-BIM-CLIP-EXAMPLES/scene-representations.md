# Scene report: 👁️representations (`Beat3_SameObject`)

File: `C/👁️representations/🐍️.py` (class `Beat3_SameObject`, `TITLE`, literal `NARRATION` with ten keys, `present_representations()` runs ten beat methods). Storyboard: `T/storyboard-representations.md`; audit: `T/audit-storyboard-representations.md`. Protagonist: the south first-floor wall `W-S` (`IfcWall`) with its ribbon window `RW-S` of the Villa Savoye teaching model.

## Round 2

### Why the keyframe capture crashed

The scene defined a private method `_level(value, digits)`; the shared drawing mixin (`SheetMixin._level(point, value)` in `C/🎨️visuals/🏛️savoye/📏️sheet/🐍️.py`) has the same name and calls it with a screen point, so the scene's method received a tuple (`abs(tuple)`). It was a name clash, not a changed model API. The scene method is now `_sign`, and `T/representations-collisions.py` lists every scene method that shadows a mixin method (only `construct` remains, by design).

### What changed (rewrite of the whole scene, narration and keys unchanged)

- Every drawing is now the toolkit's Villa Savoye drawing with the wall in yellow and the window in cyan (`highlight_colors`); the old private fallback drawings are gone. Every number comes from `quantities`, `building_geometry`, `represent`, `information`, `relations_of`, `demo_event` and `apply_event`; formulas on screen are built from model numbers only.
- Right column (persistent, from beat 1): GlobalId chip, spatial path (`IfcProject` → `IfcSite` Poissy → `IfcBuilding` Villa Savoye → `IfcBuildingStorey` → `IfcWall`, names from the model), object card (thickness, `IsExternal`, `LoadBearing`, `FireRating` with the model's sourced/assumed basis). The path hands over to the four view slots at the first dock; all four slot frames appear at once and are filled one by one.
- Plan: villa first-floor plan with the wall and window dimensions (18.50 m, 21.50 m) growing, a loupe on the east window end and a magnified plan detail (poché in the build-up layers render / brick / plaster, glass lines, 0.30 m thickness, 1.50 m pier), plus a "where the plan cuts" key showing slab, wall, window band and the cut level (+4.95) with sill and head levels.
- Section: the south part of the salon section at large scale (crop with a break line, level marks from the storeys) and a magnified wall cut at the window (slab, piloti stub, layers, glass) whose sill / window / head and wall height dimension chain grows with `ValueTracker`s.
- Elevation: the whole south façade with the window band cyan in the yellow wall, sill / window / head chain, wall height, window length and wall length dimensions, colour key with class, tag and name.
- 3D: villa axonometric with the wall ghosted, the toolkit wall solid (`element_axon`) aligned on top and animated (extrusion, opening lifted out, window slid in), length and height dimensions, relation chain wall → `IfcRelVoidsElement` → `IfcOpeningElement` → `IfcRelFillsElement` → `IfcWindow` with a one-line role under each box.
- Stored: the three real representations from `represent`: Axis (`Curve2D`) over the south pilotis row with its end coordinates, FootPrint (`Curve2D`) outline with corner coordinates and thickness, Body (`SweptSolid`) as a plain extrusion on the villa (no opening in the extrusion; "the void is carried by IfcOpeningElement"), profile and depth from the model; dashed bracket marks plan, section and elevation as derived.
- Derived: the elevation with the three hatches wiping in (gross wall, opening, net wall), live area values with their formulas (21.50 m × 3.00 m, 18.50 m × 1.10 m, 64.50 − 20.35), glazing share 31.6 % drawn as the pill on the window band.
- Update: one sheet regenerated from one `ValueTracker` (18 quantised steps of `apply_event`): live elevation (old sill dashed, opening re-hatched, 0.90 → 0.70 m and 1.10 → 1.30 m dimensions), live section crop, live villa axonometric, live "where the plan cuts" key (+4.95 → +4.85, sill +4.40 → +4.20), the plan thumbnail (geometry unchanged, said so), readout (event, revision 1 → 2, opening area 24.05 m², net wall 40.45 m², glazing share 37.3 %). The GlobalId chip stays.
- Copies: rev 2 elevation regenerated next to the rev 1 copy with the missing 0.20 m of window drawn as a hatched strip, plus six mini views (regenerated vs stale) in green and orange frames.
- Identity: villa axonometric of rev 2, the four filled slots pulsing, rev 1 and rev 2 GlobalId rows with "=".

### Final beat list

| Key | On screen |
| --- | --- |
| `source` | villa axonometric, wall and window callouts with size, spatial path, chip, object card, compass and teaching-orientation badge |
| `plan` | villa plan, growing dimensions, loupe, poché detail, cut key |
| `section` | cropped section, loupe, magnified wall cut with growing dimension chain, level marks, plan cut line |
| `elevation` | south elevation with chain, window and wall length dimensions, colour key |
| `three` | ghosted villa, animated wall solid, dimensions, void and fill relation chain |
| `stored` | Axis / FootPrint / Body rows with coordinates, extrusion of the Body, derived bracket |
| `derived` | hatched areas on the elevation, formulas, counting values, glazing share |
| `update` | one sheet of four regenerating views and readouts |
| `copies` | rev 2 regenerated vs rev 1 copy with the missing strip, six mini views |
| `identity` | rev 2 villa, four slots, same GlobalId in both revisions |

### Tests

- `SEMIO_TEACHING_OUTPUT_ROOT=… python C/🧪️tests/🐍️.py`: 137 tests, one error, `BimBrowserMetadataContract.test_source_discovery_skips_browser_runtime` (`FileNotFoundError` starting a subprocess, a missing executable on this host, unrelated to the scene).
- `SEMIO_TEST_BIM_TOPICS=representations python C/🧪️tests/🎭️presentation/🐍️.py`: every test touching this scene passes (dispatch contract, native lifecycle in en and de, literal dependency closure, dock points, window identity). The remaining failures of the file concern other topics (`information`, `objects`, `models`, `consistency` dispatch keys, `information` clearance dimensions, `_assumptions` beat) and are owned by their scene agents. `SLOTS` is unchanged, so `🔣️.cases.json` needed no edit.
- Keyframes captured and read in en and de for all ten beats (`T/🗑️generated/frames/Beat3_SameObject/`).

### Remaining risks

- The drawings rely on the toolkit (`villa_plan`, `villa_section`, `villa_elevation`, `villa_axon`, `element_axon`); the scene uses `highlight_colors` and the `centre`, `levels`, `labels`, `thumbnail`, `ghost` options, so a signature change there needs a scene fix. The axonometric projection changed once during this round (isometric), the scene only uses `axon.at` and `axon.view.scale`.
- The section crop and the board section are masked with a background-coloured rectangle (z-order −2 drawing, −1 mask); a non-dark background would reveal the uncropped section. The scene title was removed from the shared `present` by the framework owner during this round, so the title area at the top is empty in the frames.
- `Indicate` on a child part of a drawing makes manim log "adding a mobject as a child more than once" warnings (benign, also in the previous version).
- The board rebuilds elevation, section, axonometric and key for 18 states; a final render should confirm the time (about 1.5 s per state in capture runs).
- The plan view does not change geometrically with the sill event (the cut moves to +4.85 only); the clip says so ("same drawing"), the cut key carries the change.
- Assumed values carry the star and the single footnote ("assumed in the teaching model"); the ribbon windows of the other façades are drawn by the toolkit as in the model and are not highlighted (disputed extent, research-villa 4.4).
- FootPrint is shown as `Curve2D` (what `represent` returns); research-ifc open item 3 (`Surface2D` vs `Curve2D`) is still unverified against IFC 4.3.2.0.

## Round 3

Eight beats added on the same first-floor ribbon-window wall (18 narration clauses now, bilingual, every caption at most two lines). `🧪️tests/🎭️presentation/🔣️.cases.json` lists the new key order; `SLOTS` is unchanged.

### New beats

| Key | On screen |
| --- | --- |
| `lod` | the east end of the wall in one plan window at three levels of detail, Axis line (length 21.50 m), FootPrint outline (0.30 m), Body with its three build-up layers and their thicknesses |
| `scale200` | the villa plan drawn at exactly 1:200 (paper constant `PAPER` = 0.055 units per mm, so 1 m is 0.275 units), FootPrint badge filled, drawn scale bar 0 to 5 m, thickness 0.30 m → 1.5 mm, length 21.50 m → 107.5 mm, one layer 0.03 m → 0.15 mm "too thin to draw" |
| `scale20` | the same wall end at exactly 1:20 with the layers as poché, Body badge filled, scale bar 0 to 2 m, 0.30 m = 15 mm, layers 1.5 / 12.0 / 1.5 mm |
| `sweep` | the plan cut height moves 0.50 m → 1.50 m → 0.50 m → 1.45 m above the first floor (`ValueTracker`, redrawn per 1/40 m): full-length strip and magnified east end switch between solid poché and window band with glass lines; the section key dimensions the cut height live (0.50 to 1.50 m) from the first-floor level |
| `takeoff` | the sill event replayed as quantity take-off: elevation with net wall hatched green and window hatched yellow, live net area, window area and glazing share beside their rev 1 values, live formulas (64.50 − 24.05 = 40.45 m², 18.50 × 1.30, 24.05 ÷ 64.50) |
| `volume` | the wall solid from `element_axon` rebuilt per state, void lifted out, live gross volume 19.35 m³, void 7.22 m³, wall volume 12.135 m³ (net side area × thickness) |
| `drafted` | a jamb line drafted only in the plan corner at 19.20 m turns orange; table plan drawing 19.20 m, model 20.00 m, elevation 20.00 m; the elevation, section and 3D views still end the window at 20.00 m |
| `regenerate` | a sweep regenerates the plan from the model, the drafted line and its label disappear, the table reads 20.00 m three times |

Clause order: source, plan, section, elevation, three, stored, lod, scale200, scale20, sweep, derived, update, takeoff, volume, copies, drafted, regenerate, identity. `update` now leaves its sheet as the hero, so `takeoff`, `copies` and the rest fade it through `_leave`.

### Changes to existing beats

- Strict layout findings fixed: compass and callouts of `source` (two lines instead of squeezed text), section crop pruned of parts right of the crop edge (`_prune`, also speeds the build), section detail frame and title margins, plan-cut label above the window band, 3D length dimension below the ground plane, class boxes widened so no German text is squeezed, shorter German note in the update sheet.
- `_cut_key` takes an optional `cut` (moving cut with a live dimension from the first floor).

### Strict findings (final capture, en and de, 18 beats each)

One remaining finding per language, `derived`: "stroke through chip of '31.6%'". It is a false positive: the percentage chip is a fully opaque plate (fill opacity 1.0) laid over the toolkit's window mullion lines (single stroke object spanning the window band), the hatch is split around it, and the chip is lower than the window band so no edge touches it.

### Tests

- `SEMIO_TEST_BIM_TOPICS=representations python C/🧪️tests/🎭️presentation/🐍️.py`: 12 tests, OK (dispatch contract with the 18 keys, native lifecycle en and de, dependency closure, dock points).
- Earlier in this round the full `python C/🧪️tests/🐍️.py` ran 137 tests with one unrelated error (missing executable in the browser metadata test); the narration tests read the literal `NARRATION`, which stays bilingual and distinct per language.

### Remaining risks

- The sweep, takeoff and volume animations are checked in their final keyframe only (capture jumps trackers to the end); the intermediate states use the same builders as the final one (`_cut_plan`, `_takeoff_elevation`, `_volume_scene`).
- `takeoff`, `volume` and `update` rebuild the elevation, the toolkit wall solid and quantities for up to 18 states; `_q` memoizes quantities per state.
- The paper scale constant is the one number not read from the model; it is a presentation constant, the millimetres on screen follow from model metres at 1:200 and 1:20.

## Round 4

Source of the changes: `T/verify-r2-representations.md` (1 blocker, 3 major, 11 minor). Edits are scripted in `T/representations-round4.py` plus small follow-ups. The missing persistent title (m2) is intentional (ENERGY-CLIP-STYLE); instead every beat now swaps a per-beat heading chip with `_fonts.section_heading` (`HEADINGS`, en and de), like the other BIM scenes.

| Finding | Fix |
| --- | --- |
| B1 volume arithmetic | the whole `volume` beat prints three decimals: gross 19.350, void 7.215, wall 12.135 m³, formulas `40.45 m² × 0.30 m = 12.135 m³` and `19.350 − 7.215 = 12.135 m³` hold as printed; no two-decimal volume remains |
| M1 assumption star | stars on the window and wall dimensions of `source`, on the derived formulas and values (areas, share, pill), on the take-off values and formulas, on the sweep window line (sill and head each), on the cut key levels and the update sheet levels, on layer thicknesses, on the Body profile, on the third chain dimension (head to top); the `source` frame carries the footnote; the update sheet carries it at the bottom right |
| M2 IFC property names | the object card shows `Pset_WallCommon.IsExternal`, `.LoadBearing`, `.FireRating` and `TotalThickness` with the localized meaning from `information()` below each name, value and basis (sourced / assumed) on the right |
| M3 FootPrint | drawn as a closed 2D outline curve, no fill and no hatch, in `stored` and `lod`; label stays `Curve2D` as returned by `represent` (open research item: IFC 4.3.2.0 RepresentationType still unverified) |
| m1 stars | one rule, see M1 |
| m3 sweep frame | magnifier scale 1.45, poché ends inside its frame |
| m4 dimension tick | `_dimension` wrapper pushes every dimension label 0.06 units away from the tick |
| m5 "[white]" | layer names rendered with a bullet ("Render · white") instead of brackets |
| m6 volume before value | rev 1 void 6.105 and wall 13.245 m³ printed under the live values |
| m7 hatches | gross wall hatch stays over the whole band, net hatch uses the other angle (−45°) and swatch, so the final frame shows both |
| m8 "belegt" | card basis word is "Quelle" in German |
| m9 LOD scale | the `lod` legend says "layers on paper at 1:20" |
| m10 other façades | `source` carries "other façades: ribbon windows are a teaching simplification" |
| m11 German terms | "Schnittebene" instead of "Grundrissschnitt"; "Öffnung geändert" for `OpeningResized` and German words in the event lines (Brüstung, Objekt, Sturz) |

Also fixed: a duplicated assumption footnote (created in `source` and again in `plan`) that survived every later fade.

Strict layout findings in the final capture (18 beats, en and de): 1 per language, `derived` "stroke through chip of '31.6%*'", a false positive (opaque plate over the toolkit's window mullion strokes). Presentation tests for `representations`: 13 tests OK.

## Round 5

Source: `T/final-verify-representations-models.md` (1 major, 6 minors).

| Finding | Fix |
| --- | --- |
| R-M1 window start marker | `stored` FootPrint row draws the first three metres of the outline with a break line at the crop, the dashed marker at `op["start"]` (1.50 m, read from the model) and a `1.50 m*` dimension from the wall start; corner coordinates moved below the dimension |
| R-m1 slots | `_fill(kind)` replaces the docking: each of `plan`, `section`, `elevation`, `three` puts the thumbnail of its own view into its slot in the same beat (all four frames appear with `plan`, the spatial trail leaves then) |
| R-m2 stars | `OpeningResized` card (sill, head) in `update` and the event line in `takeoff` carry the assumption star |
| R-m3 +9.40 | the top level mark of the section and of the update sheet carries the star (the level reference is assumed) |
| R-m4 hatches | gross wall is a dashed outline of the whole band (legend swatch dashed), net wall a dense green hatch at −45° that stops at the opening, opening a yellow cross-hatch; the gross hatch only wipes in until the net starts |
| R-m5 German card | German meanings shortened for display ("grenzt an die Außenluft", "trägt Lasten", "Feuerwiderstandsklasse") so a gutter remains before "angenommen" |
| R-m6 LoadBearing | `TY-WALL` `Pset_WallCommon.LoadBearing` basis set from `sourced` to `inferred` in `🧱️model/🔣️.json` (schema enum already allows it); the card shows "inferred" / "gefolgert" |

Final capture en and de, 18 beats each: strict findings 1 per language, the known `derived` "stroke through chip of 31.6 %*" false positive (opaque plate over the toolkit's window mullions). Presentation tests for `representations`: 14 tests OK.
