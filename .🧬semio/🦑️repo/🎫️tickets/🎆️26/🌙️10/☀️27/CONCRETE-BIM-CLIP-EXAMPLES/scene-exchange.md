# Scene `Beat5_Interfaces` (🔌️exchange)

Scene file `C/🔌️exchange/🐍️.py` (clip folder `C` = `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip`). Frames: `T/🗑️generated/frames/Beat5_Interfaces/` (`-en-`, `-de-`, one PNG per narration key, layout reports `-layout.md`).

## Round 2

### Crash

`ValueError: zip() argument 3 is shorter than arguments 1-2` came from beat `loss`: `ReplacementTransform(villa_axon(full), villa_axon(ghosted))` between two axonometrics whose families differ (ghosted elements become dashed outlines). Fixed by a cross-fade (`FadeOut(full)` and `FadeIn(ghosted)`). No other `Transform` between drawings is left in the scene (`mapping` also cross-fades plan and analysis view).

### What changed (every beat now pairs the concept with the Villa Savoye drawing)

The story object is the piloti **B3** (grid 3/B, x 10.75 m, y 4.75 m, directly beside the ramp opening and inside the curved glass wall) and the first-floor slab SL-1. All numbers come from the model functions (`quantities`, `step_excerpt`, `mesh`, `translate_structure`, `tributary`, `ids_check`, `partial_model`, `information`).

New helpers inside the scene file (nothing in the shared model or toolkit was touched): `_wire` (curved leader from a STEP token to the drawing, colour = meaning), `_hatched`, `_volume` (mesh volume by the divergence theorem), `_inside`, `_pick` (STEP lines in a chosen order), and the scene methods `_bay` (plan crop around B3 at the true piloti diameter), `_post` (piloti in elevation under the slab with ground, level marks, concrete hatch, height dimension), `_strip` (section along axis 3 A to C through the slab and A3, B3, C3 with `collapse` to the mid-surface), `_load_path` (tributary cell, load arrows and reaction arrow of one piloti in the axonometric), `_file` (the exported mesh text).

### Final beat list

| # | key | what is on screen |
| --- | --- | --- |
| 1 | `interface` | sender axonometric (architecture, 28 objects) and receiver axonometric (structure, 33 objects), both with B3 lit; schema chip `IFC4X3_ADD2` / ISO 16739-1:2024, the real `ISO-10303-21;` and `FILE_SCHEMA` lines; packet "B3 · IfcColumn" moves along a `ValueTracker` path from sender B3 to receiver B3; the one "teaching model" badge |
| 2 | `semantic` | STEP file A and file B (renumbered, same GlobalId, different `#n`); token rings for class, placement, GlobalId are wired to B3 in a 1 m scale plan crop (grid axes 3 and B, ramp opening, glass wall) and from B3 to chips ("IfcColumn · COLUMN (our modelling choice)", "placement x = 10.75 m, y = 4.75 m", "GlobalId: stable identity"); site plan thumbnail with the x and y dimension lines and a callout rectangle |
| 3 | `properties` | six STEP lines (LoadBearing value, Pset, IFCCOLUMN `.COLUMN.`, extrusion depth, material, circle radius) each ringed and wired to its meaning: LoadBearing arrow onto the slab head, type chip, concrete hatch, enlarged section circle with radius and diameter (assumed), height dimension grown by a `ValueTracker` from 0 to 3.25 m, ground and level marks +3.50 / ±0.00, plan thumbnail |
| 4 | `mesh` | struck-out STEP tokens (class, GlobalId, LoadBearing, material); the exported file (`v x y z`, `f a b c`) as text; the triangulated piloti in axonometric (3.25 m dimension) and its top-cap triangle fan at 6.5 units per metre; `ValueTracker` segments 6 to 16 drives both drawings, the vertex and triangle counts, and the mesh volume against the true cylinder volume (0.2239 vs 0.2297 m³, −2.6 %); lost items as chips |
| 5 | `purpose` | first-floor plan with SL-1 lit and the A3–C3 cut line; slab facts (21.5 × 19.0 m, net 380.4 m²); section along axis 3 through A3, B3, C3 with ground, hatch and the `t = 0.25 m` dimension; `ValueTracker` collapse thins slab and pilotis to the mid-surface (+3.375 m) and leaves dashed ghosts; thickness remains an attribute |
| 6 | `mapping` | class box (IfcColumn → IfcStructuralCurveMember, 25; IfcSlab FLOOR → IfcStructuralSurfaceMember, 24 shells; 35 slab nodes at z = 3.375 m; GlobalId recorded); analysis view on the first-floor plan with the ramp void outline (28.1 m², not meshed); GlobalId chips of B3 and SL-1 wired to the B3 member and a shell |
| 7 | `assumptions` | property table with colour swatches (self-weight 25 kN/m³ × 0.25 m = 6.25, finishes 1.50, imposed 2.00, total 9.75 kN/m², fixed support, each "assumed"); strip with supports at the feet; three stacked arrow rows grown by three `ValueTracker`s, total q readout; plan thumbnail with the strip position |
| 8 | `reaction` | plan with all 25 tributary cells and areas, B3 / A3 / A1 lit (interior, edge, corner); per lit piloti an axonometric load path: the cell grows with `k`, load arrows (kN/m²) stand on it, the reaction arrow (kN) enters the foot; readouts "15.5 m² × 9.75 kN/m²" and "R = 151 kN" count up; load check Σ R = 3,709 kN = q × 380.4 m² and the "estimate, no finite element calculation" badge |
| 9 | `loss` | villa axonometric with the lost elements (4 ribbon windows, 4 openings, 4 infill walls) lit, cross-fade to dashed ghosts; translate arrow to the analysis view; dashed return arrow crossed out; loss chips with counts from the model |
| 10 | `domain` | analysis view with a shell, B3 member and a support wired to the class box (IfcStructuralSurfaceMember 24, IfcStructuralCurveMember 25, IfcStructuralPointConnection 25); a tool's own input format built from the same ids (SHELL / BEAM / FIX lines) labelled "example" |
| 11 | `check` | IDS card (applicability, requirement, cardinality), wired to B3 with its own LoadBearing value; ground plan with architecture and pilotis; `ValueTracker` lights 25 of 25 rings; translation checks (GlobalIds 49 / 49, units, revision 1 = 1); the mesh file has no IfcColumn (fan icon, count 0) |

Narration is unchanged from the audited storyboard text (only the corrections of `T/audit-storyboard-exchange.md`); `C/🧪️tests/🎭️presentation/🔣️.cases.json` already lists the 11 keys.

### Checks

Both languages were captured (`python T/capture.py 🔌️exchange Beat5_Interfaces en|de`) and every PNG was read. Layout reports list only wires that start at a STEP token ring and cross the trailing characters of that line, thin dashed callouts, and labels on plan lines with a background halo; no caption-band overlap, no clipped or off-frame text, no overlapping labels.

### Remaining risks

- Wires start at the right edge of the token ring and cross the last characters of the STEP line (`);`); the layout checker reports these as "line through". Intended, thin, coloured by meaning.
- The reaction beat redraws three axonometric load paths per frame (tolerable: three small drawings, key rounded to 0.01).
- The ramp opening, glass wall and ribbon windows are teaching-model geometry; the piloti count, tags, diameter and loads are assumed values labelled as such (the "teaching model" statement is made once, in beat 1).
- `step_panel` text is 16.2 pt monospace; the long STEP lines wrap after a comma, so the panels are tall (up to 4.6 units in beat 3).

### Tests (Round 2)

- `python C/🧪️tests/🐍️.py`: 137 tests, one error that does not touch this scene (`test_source_discovery_skips_browser_runtime` cannot start the `repo` binary in this shell). Log: `T/🗑️generated/tests-clip.log`.
- `SEMIO_TEST_BIM_TOPICS=exchange python C/🧪️tests/🎭️presentation/🐍️.py`: the exchange cases pass (`test_dispatch_contract` for `exchange` with the 11 keys, native lifecycle en and de, consuming sequence on `_seq`); the remaining failures are the other topics (information, objects, models, consistency), whose scene agents are still editing. Log: `T/🗑️generated/tests-exchange-presentation.log`. `🔣️.cases.json` already lists the 11 exchange keys, so it was not changed.

## Round 3

### Beats added (11 → 16 clauses, same file, narration in `NARRATION`, keys added to `🔣️.cases.json`)

Final order: `interface`, `semantic`, `properties`, `mesh`, **`roundtrip`**, `purpose`, `mapping`, **`slab`**, `assumptions`, **`buildup`**, **`cells`**, `reaction`, **`feedback`**, `loss`, `domain`, `check`.

| key | content (every number from `mesh`, `quantities`, `translate_structure`, `tributary`, `building_geometry`) | ValueTracker |
| --- | --- | --- |
| `roundtrip` | IFC file (piloti B3 in elevation, classified, 5 lost items listed as present) → mesh file (34 vertices, 64 triangles, class, GlobalId, LoadBearing, material, type struck) → "IFC again, read from the mesh": the returned piloti fades from the classified colours to grey triangles, volume −2.6 % | `u` colours and strikes of the returned piloti |
| `slab` | first-floor plan with SL-1 lit, the real ramp opening `V-RAMP` (hatched, named), the hanging garden `SP-TERRACE` (110.2 m², hatched green), 1.25 m cantilever dimensions at both ends; gross 408.5 − opening 28.1 = net 380.4 m² | `v` grows the opening and counts the net area |
| `buildup` | section through SL-1 at B3 (slab t = 0.25 m, B3 below, finishes skin, people on top) and a stacked bar 6.25 + 1.50 + 2.00 = 9.75 kN/m², then 9.75 × 15.5 m² = 151 kN | three trackers, one per layer |
| `cells` | crop of the first-floor plan around B3: midlines at half the 4.75 m grid (2.375 m dimensions), full cell 22.6 m², notch cut by the ramp opening 7.0 m², B3 cell 15.5 m² | `k` grows the cell and counts the area |
| `feedback` | IFC model (B3 GlobalId) ⇄ analysis tool: forth arrow (class, GlobalId, geometry, loads), back arrow (reactions R, source revision) with a packet "R = 151 kN", crossed arrow (geometry, class not sent back); on arrival the result table linked to B3 (R, q, A, revision), marked as an own result set, not a standard Pset | `p` moves the packet |

The patio of the brief is not in the model; the villa's own named feature of the first-floor slab besides the ramp opening is the hanging garden `SP-TERRACE`, so that is what the slab beat names.

### Fixes

- Leaders from STEP tokens (`_route`): each leaves the token ring downward into the gap between two STEP rows (below the row's bounding box), runs out of the panel along that gap and only then curves to the drawing; same-row tokens use separate lanes. No leader touches a STEP character any more.
- Reaction beat: the cell labels are gone from the plan. The three lit cells carry only their tag outside the plan (A1 and A3 under the bottom edge, B3 above with a vertical leader), the areas and forces are listed on the right with the load paths, a note says so.
- Property tables and labels that the strict check read as under 11 px (de `Eigengewicht 25 kN/m³ × 0,25 m`, feedback table) were shortened or split so no text is squeezed.

### Strict check (both languages, `capture.py` with the strict block)

en and de both end at 22 findings in 4 frames. All of them are justified false positives:

- 12 on `semantic` and `properties`: the colour ring around a STEP token is a rectangle that hugs the token inside its own STEP line; the check reads it as a frame touching text. Rings are intended.
- 10 on `roundtrip` (5 in the mesh column, 5 in the returned column): the strike-through lines through the lost item names. The strike is the message.
- `mesh` strike-throughs (class, GlobalId, value, material) are the same intended mark and count within the 22 when the frame is kept.

Labels that sat on hatches or grid lines were moved off them (ramp label under the crop, tags outside the plan, cells beat with only the B3 axes), instead of being backed.

### Tests

`SEMIO_TEST_BIM_TOPICS=exchange python C/🧪️tests/🎭️presentation/🐍️.py`: 12 tests, OK (all topics currently pass; `test_dispatch_contract` compares the 16 keys with `🔣️.cases.json`).

### Risks

- The `buildup` finishes layer has no authored thickness in the model; it is drawn as a thin schematic skin and labelled "layer thickness not modelled".
- The result set in `feedback` is an example of a custom property set, not a claim about a specific IFC result entity (the research marks the 4.3 relationship as unverified).

## Round 4 (fixes after `verify-r2-exchange.md`)

| finding | fix |
| --- | --- |
| B1 `domain` | one object per role: shell `SH-A0` (`analysis["shells"][0]`), member `M-B3`, support `F-B3`; highlight, leaders and the example block read the same three ids. The view is now the axonometric analysis model, so the member (middle) and its support (foot) are separate leader targets. |
| B2 `purpose` | `IfcStructuralAnalysisDomain` is no longer printed as an entity; the line reads "IFC structural analysis domain (schema section)". Real entities are listed in `domain` (SurfaceMember, CurveMember, PointConnection) plus `IfcStructuralAnalysisModel` (`analysis["model_class"]`). Names still only checked against `research-ifc.md`, not against buildingSMART 4.3.2.0. |
| M1 `interface` | sender is the design model (architecture + structure, 53 objects = union of the two partial models, shared slabs and ramps once), which is what is drawn and exported (B3 is a structure object); receiver 33 objects. |
| M2 `cells` | two decimals and the equation as printed: 22.56 − 7.03 = 15.53 m² (live, scaled by the tracker). |
| M3 `feedback` | R with 0 decimals everywhere (151 kN), A 15.53 m²; the reaction beat and `buildup` also use 15.53 m². |
| M4 assumed values | assumption star `*` plus legend "* assumed value (teaching model)" on every beat that shows an assumed value: level +3.50 and height 3.25 m, thickness, mid-surface z, cantilever, ramp opening and net area, B3 cell, example input; `t` in `assumptions` carries "(assumed)". |
| M5 garden | the slab beat no longer names a roof garden on the first-floor slab. The model owner moved `SP-TERRACE` to the roof storey (model file 22:28); the beat now names the first-floor space read from the model (`SP-SALON`, 63.5 m²) and picks it by the slab's storey (`element(SL-1)["storey"]`), so it follows the model. Narration changed accordingly (en/de). |
| M6 volume | `mesh`: orange slivers between each chord and the true circle (live with the segment tracker) and a legend; `roundtrip`: a top-view inset of the returned piloti with the slivers. |
| m1 | example block with units and star (`t 0.25 m*`, `D 0.30 m*`, `FIXED*`). |
| m2 | the unit check is computed from every unit string in the model (`_units`) instead of hard-coded. |
| m3 | loss chip "× 4 walls: render layer (white) not kept". |
| m4, m5 | German wording: "Lasteinzugsflächen-Näherung", "Ausbaulasten" everywhere, slab narration "Die Decke gehört der Villa …". |
| m6 | `mapping` footer split into two lines. |
| m7 | `interface` axonometrics at shell opacity 0.5. |
| m8 | `slab`: labels ordered like their targets (no crossing), cantilever named once between the two dimension lines. |
| m9 | badge "Teaching model: 25 pilotis (sources differ) …". |

Captured en and de (16 frames each, all read). Strict check: 22 findings in 4 frames per language, unchanged and justified in Round 3 (12 rings around STEP tokens, 10 intended strike-throughs). The model file was last changed at 22:28 and was not touched by this scene; final captures were taken after that.

Tests: `SEMIO_TEST_BIM_TOPICS=exchange python C/🧪️tests/🎭️presentation/🐍️.py`: 14 tests, OK.

## Round 5 (type-level property set, German title)

- `step_excerpt` (`C/🧱️model/🔌️exchange/🐍️.py`): when the element has a type and the property set comes from that type (`Pset_ColumnCommon` of `TY-PILOTI`, `Pset_WallCommon` of `TY-WALL`), the set is written once for the type: `IFCPROPERTYSET` (GlobalId derived from the type tag, so all 25 pilotis share it), `IFCCOLUMNTYPE` carries it in `HasPropertySets` (`(#117)`), and the occurrence reaches it only through `IFCRELDEFINESBYTYPE`; no `IFCRELDEFINESBYPROPERTIES` is written for it. Elements without a type (slabs) and `spatial=False` keep the old form. Model test `test_pset_material_type_and_storey_entities_link_back_to_the_element` now asserts exactly that (no `IFCRELDEFINESBYPROPERTIES`, one pset in `HasPropertySets`).
- Scene `properties`: the STEP panel now shows the `IFCCOLUMNTYPE` line; the `(#117)` reference is ringed like `Pset_ColumnCommon` and `.T.`; the LoadBearing leaders end at the drawn piloti, and the note reads "via type Piloti" (de "via Typ Piloti"). The diameter dimension is shortened to `Ø 0.30 m*` (star legend) so the profile leader no longer crosses it.
- `mesh` and `roundtrip` still list `Pset_ColumnCommon.LoadBearing` as lost (model `mesh()["lost"]`, unchanged).
- German title: "Schnittstellen und Übersetzung".
- Captured en and de (properties, mesh, interface frames read, all 32 produced). Strict: 23 (en) and 24 (de) findings in 4 frames; the justified 22 plus one more token ring (`(#117)` on the type line, same false positive as the other rings).
- Tests: exchange presentation 14 tests OK; clip/model tests 138 run, one error (`test_source_discovery_skips_browser_runtime`: `repo` binary not found in this shell), unchanged and unrelated.

## Round 6 (final verification: X1–X9)

- X1/X9 `cells`: the formula is built with the language number helper and the true minus U+2212 in both languages (de "B3-Zelle = 22,56 − 7,03 = 15,53 m²*"); every other printed formula already used `self.number` and "−" (checked: slab "− Rampenöffnung", cells, buildup and assumptions sums). The font draws U+2212 short, so it looks like a hyphen at 720p; the source character is the minus sign.
- X2 piloti count: `interface` (badge split: "Teaching model: tags and values simplified", "pilotis: 25 in this teaching model*", "* sources differ: 15–25"), `mapping` (25* members, footer, legend), `domain` (25* members and supports, legend), `check` (25 / 25* IfcColumn pass, legend). The count and the range both come from `len(geometry["pilotis"])` (`_count_legend`).
- X3/X4 `slab`: the hatches are now named from the model: `SP-SALON` (63.5 m²*) and `SP-TERRACE` "Hanging garden (first floor, open air)" (110.2 m²*), each with a leader and a hatch colour; both are read from the spaces of the slab's storey, so the labels follow the model. The first-floor plan shows the structure layer only so the only hatches are the named ones.
- X5: `_percent` computes the deviation from the volumes rounded as printed: 0.2239 / 0.2297 − 1 = −2.5 % (`mesh`, `roundtrip`, en and de).
- X6 `loss`: "teaching choice: ribbon windows on all faces" (de "Lehrentscheidung: Bandfenster an allen Seiten"). X7: `r = 0.15 m*` and the `mesh` height `3.25 m*`. X8: "Lasteinzugsfläche A".
- Captured en and de; read interface, mapping, slab, domain, check, loss, mesh (en) and slab, cells (de); strict: 23 (en) / 25 (de) findings, all the known token rings and strike-throughs (nothing else listed). Exchange presentation tests: 14 OK.
