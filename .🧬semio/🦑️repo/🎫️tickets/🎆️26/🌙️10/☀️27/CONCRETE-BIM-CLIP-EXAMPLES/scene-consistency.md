# Scene report: 🔗️consistency (`Beat6_Consistency`)

File: `C/🔗️consistency/🐍️.py` (class `Beat6_Consistency`, `TITLE`, literal `NARRATION` with ten keys, `construct()` calls `self.present("consistency")`, `present_consistency()` runs the ten beat methods). Storyboard: `T/storyboard-consistency.md`; audit: `T/audit-storyboard-consistency.md`.

## What the clip shows

Protagonist: the roof slab `SL-R` of the Villa Savoye teaching model (`IfcSlab`, `ROOF`, `LS-ROOF` layer set). Every number is read from the model: `demo_event("roof_soil")` and `demo_event("roof_soil_alt")` are the payloads, `apply_event` the changed states, `tributary(model, "roof")` the zoned roof load (planted 9.32 to 12.92 kN/m², solarium unchanged) and the reactions, `translate_structure` the analysis models, `consistency_issues` the expected/actual rows, `resolve_link` the resolved model, `global_id` the 22 characters (the count on screen is `len(gid)`).

| Beat | Key | On screen |
| --- | --- | --- |
| 1 | `identity` | villa axonometric with the lit roof slab, tag badge, GlobalId written letter by letter with the counted brace, local STEP number |
| 2 | `metadata` | same axonometric, GlobalId chip, three model badges with revisions linked by leaders to the slab, key-value card (class, storey, `Pset_SlabCommon.IsExternal`, author, revision as model field) |
| 3 | `leak` | villa section, history card (reports, teaching scenario follows), magnified roof build-up from the layer set with rain, zoom circle on the planted zone |
| 4 | `change` | event card (real event dict, GlobalId copied from the chip), soil layer thickening live (`apply_event` per frame), added thickness inside the soil band, revision badges 1 to 2, GlobalId unchanged |
| 5 | `derived` | section with zone-wise area-load arrows (planted: grows, solarium: constant) and reaction arrows under the pilotis of the cut, roof plan with hatched planted and solarium zones, derivation card (sum of zone loads asserted against the model) for the focus and the contrast piloti |
| 6 | `stale` | up-to-date tributary model beside the stale analysis view (orange, shorter arrows), dashed missing length, headers tied to the GlobalId chip |
| 7 | `check` | small pair of views, link record, expected versus actual table from `consistency_issues`, BCF topic card with a leader to the slab |
| 8 | `resolve` | review checklist (assumptions and one loss group from the regenerated translation, ticked by a person), analysis view green, badge and link revision updated, BCF topic resolved |
| 9 | `limits` | two sections and build-ups for edit A and edit B of the same revision, same GlobalId, different soil layer, "a person decides" |
| 10 | `recap` | full axonometric, revision badges, schema / GlobalId / revision / link badges tied to the drawing |

## Audit fixes applied

German caption and title fixes, the "22" removed from the caption, "meant to" instead of "in every exchange", "model records" instead of IFC metadata, teaching-scenario labels (card header, history card), no waterproofing claim and no crack or stain, soil band instead of "+0.20" dimension, estimate label "tributary-area estimate, not FEM", teaching grid label without a piloti count claim, IDS dropped, BCF card tied to the drawn slab, abstract bars/gear/person removed from beat 9, recap caption no longer claims GlobalIds keep models consistent, content kept above y = -2.6, every visible string in English and German.

## Round 2

Story now told end to end on the Villa Savoye roof slab `SL-R` (GlobalId `2DXklET1jK3AZcJz_ZtAN9`): a reported roof leak, a repair that thickens the soil of the planted roof from 0.10 to 0.30 m, derived values that follow, the linked structural analysis (`LK-ROOF`) that still holds revision 1, its detection, review and resolution, and the limit of identity (two parallel edits). Every number is read from the model (`demo_event`, `apply_event`, `tributary`, `translate_structure`, `consistency_issues`, `resolve_link`, `ids_check`, `step_excerpt`); layer weights are `thickness × density` of the layer set and asserted against the zone `load_breakdown`.

### Fixes against `audit-r2-consistency.md`

- B1 one roof load: the clip uses only the planted-roof load `q` (9.32 → 12.92 kN/m²; solarium 8.52 stays). The roof mean (9.11 / 11.78) is never shown; the check table reads `planted_area_load`.
- B2 recap: exploded axonometric with architecture / structure / services layers named and coloured, the analysis card below, four pills (schema, GlobalId, revision, link) with leaders to the drawn parts.
- M1 leak: new beats `leak` (history card, section, magnified build-up with rain) and `leakpath` (numbered water path soil → gap in the waterproofing → slab → ceiling stain, moving drop, stain also on the section; gap labelled "illustration · place not documented").
- M2 link as an object: `LK-ROOF` pill joined to the roof slab of the model and to the analysis slab, dashed orange while stale ("source rev 1"), solid green after resolution ("source rev 2"); in `check` it stays orange.
- M3 base revision: `fork` shows revision 1 as root with two persons and their edits (rev 2, soil 0.30 / 0.20 m, same GlobalId); `decide` compares 12.92 and 11.12 kN/m² as stacked bars.
- M4/M5/N4 assumptions: storey elevations are no longer shown (marks GF / 1F / roof only); every assumed layer thickness, density, imposed load and the soil value carry an orange asterisk, legend "* teaching-model assumption, not a source value" from beat 3 on; sourced values say "sourced"; the 5 × 5 grid count is no longer stated (grid spacing "sourced", planted area asterisked, solarium "sourced").
- M6–M13 layout: no more stacked badges or clipped labels; analysis arrow labels re-placed clear of the mesh; roof plan enlarged (scale 0.18) with grid bubbles, 4.75 m grid dimensions, north arrow, scale bar and the two openings named in the legend.
- M14: growth visible in stills (short bar = old reaction, "before 105 kN", "+41 kN" chip, delta map `spread`, `stale` gap "+41 kN missing").
- M15 German author labels via `AUTHORS` (Tragwerksplaner, Atelier (Architektur), Heizungsplaner); M16 header glyph artefact gone (headers built once per beat).
- M17/M18 depth: red concept chip per beat top left, reference chip top right (IFC4X3_ADD2 · ISO 16739-1:2024; IDS 1.0 in `ids`; BCF-XML 3.0 in `report`), formulas with substituted numbers (`q = 5.00 + 0.20 + 1.32 + 5.40 + 1.00 = 12.92`, `Δq`, `A = 4.75 m · 2.375 m = 11.28 m²`, `R = A · q`, `Σ ΔR = 3.6 · 200.2 = 721 kN`), numbers in the narration (unit-tested against the model).
- M19 conventions: grid bubbles, dimension strings, north arrow, scale bar, section marks; M20: STEP excerpt with `#101` (local) and the GlobalId token underlined, IDS vs link check, BCF topic with viewpoint, review checklist; N9 and the IDS point restored.
- N11 BCF "component" now reads `GlobalId`.

### Final beat list (20 clauses, en and de)

1 `identity` GlobalId letter by letter, 22 characters, STEP line (`#101` local) · 2 `models` exploded axonometric, three model cards with revisions and authors · 3 `metadata` slab record card with leaders (LoadBearing sourced, IsExternal assumed) · 4 `leak` section, history, build-up, rain · 5 `leakpath` water path and stain · 6 `change` design event card, soil grows 0.10 → 0.30 m live, "+0.20 m", gap repaired · 7 `kept` stays-the-same vs changes cards · 8 `load` layer weight table, formula, stacked bar growing live (9.32 → 12.92) · 9 `grid` roof plan, mini cells with dimensions, `A = 4.75 · 2.375 = 11.28 m²`, C3 sliver 2.38 m² · 10 `derived` section with q arrows and reactions growing, ticks for the old values, `R = A · q` panel · 11 `spread` delta map of all 25 pilotis, D3 largest, C4 zero, Σ ΔR = 721 kN · 12 `stale` model beside analysis, link object, "+41 kN missing" · 13 `check` expected vs actual table (revision, q, R A3, ΣR) · 14 `ids` IDS passes (2 of 2 slabs), link check fails · 15 `report` BCF topic, viewpoint, GlobalId pointer · 16 `retranslate` 24 surface members, 25 columns, 35 slab nodes counted up, review checklist ticked by a person · 17 `resolve` link green rev 2, 146 = 146 kN, 0 issues, BCF resolved · 18 `fork` · 19 `decide` · 20 `recap`.

### Verification

- Keyframes en and de read and fixed; both languages: strict findings 0 in all 20 frames and no legacy findings in the last full runs apart from the `retranslate` line spacing that was fixed afterwards. The final full-run reports are saved as `🗑️generated/layout-full-en.md` and `layout-full-de.md`; the last full runs differ only in the `retranslate` rows, which were re-captured alone afterwards (en and de: zero findings).
- `SEMIO_TEACHING_OUTPUT_ROOT=… python C/🧪️tests/🐍️.py`: 138 tests, 137 pass; the single error is `BimBrowserMetadataContract.test_source_discovery_skips_browser_runtime` (no `bun`/`node` executable in this shell, `FileNotFoundError` from `subprocess.Popen`), not touching this scene. New test `test_consistency_narration_numbers_are_the_model_values_in_both_languages` (A3 reaction 105 / 146 kN, areas, 9.32 / 12.92 / 11.12 kN/m², Δq 3.6, 4.75 m × 2.375 m, soil 0.10 / 0.30 m, 24 / 25 members, D3 largest, C4 zero; en and de punctuation). `🎭️presentation/🔣️.cases.json` keys already list the 20 keys.

### Remaining risks

- `ids_check` counts both slabs (`SL-1`, `SL-R`) because the requirement applies to `IfcSlab`; the story says "2 of 2 slabs".
- `check` and `resolve` show only piloti A3 as the arrow example (others collided with the mesh); the section beat `derived` shows five pilotis.
- The first-floor slab analysis carries roof reactions as point loads; the clip labels them "reaction", not a FEM result ("tributary-area estimate, not FEM").
- Live detail rebuilds `apply_event` per soil value (cached by rounding); rendering time of `change` and `load` is higher than other beats.
- The shared `ClipScene.present` no longer draws the chapter title; this scene draws its own red concept chip at the title position and keeps `TITLE` unchanged.
- `🎭️presentation/🐍️.py`: 13 tests, the 12 test bodies pass (all six scenes, en and de, lifecycle render incl. `consistency`); the one error is `tearDownClass` printing the changed-inputs list with emoji paths under the `cp1252` console (`UnicodeEncodeError`), a host encoding issue of the debug print (`PYTHONUTF8=1` avoids it), not scene related.

## Round 3

Source: `verify-r2-consistency.md` (0 blocker, 5 major, 15 minor) and the "Coordinator Decisions" of `fleet-r2.md`. All 20 clauses and the beat list are unchanged except two caption rewordings (`stale`, `retranslate`) and NBSP between numbers and units.

### Majors

- `identity`: the GlobalId leader now starts at the chip and ends on the roof edge (`axon.at(21.5, 0, roof)`), no longer on the slab bounding box.
- `metadata`: the roof outline (external edges) is drawn in cyan and the `IsExternal` leader ends on the front external edge; `LoadBearing` ends on a point of the slab above piloti B3.
- Planted area: the grid frame now shows "Planted roof (model) 200.2 m² *", "Roof garden · terrace void 110.0 m² (sourced)" (read from `quantities("SP-TERRACE")["reference_area"]`, the space the model owner moved to the roof storey; `SP-TERRACE` is the roof-slab void) and "Solarium 70.0 m² (sourced)". The Σ ΔR line carries the asterisk. q, tributary loads and reactions are read from the model at run time and follow any model change; the model file was last changed at 22:28 (roof garden space on the roof storey), the planted soil zone of the roof slab is still 200.21 m² in `tributary()`. If the models owner changes the zone, only a re-capture is needed.
- Piloti count: only "25 pilotis in this teaching model * · sources differ: 15–25" (`grid`, `spread`, `retranslate`), counter "25 curve members (columns) *", narration "25 columns of this teaching model" / "25 Stäbe dieses Lehrmodells".
- `resolve`: the verdict compares the re-translated analysis reaction (`translate_structure(resolved)["used"]["roof-load"]["reaction_A3"]`) with the value recomputed by `tributary(resolved, "roof")`, with a computed `=` / `≠`, plus the old comparison "rev 1: 105 kN ≠ 146 kN".

### Minors

recap: layer labels have leaders and the layers keep their discipline colours (no yellow highlight there) · `spread` card inside the frame · `check` DE header fits · NBSP between number and unit in all narration (no split "11,28 / m²", "105 / kN") · one German term "Pflanzsubstrat" for the soil layer everywhere · `load` thickness and density cells carry the asterisk · chip header "GlobalId" only (tag shown separately) · `ids` leaders end on the slab corners and no longer cross the villa · teaching-choice note for ribbon windows on all faces (`identity`, `models`) · analysis colour kept; state shown on the link pill ("source rev 1 · out of date" / "source rev 2") · `change`: "+0.20 m *" callout with arrow on the section roof · `resolve`: "0 issues" and "BCF · resolved" tied to the link pill with leaders · `retranslate`: only assumptions whose claims are visible on the mesh (mid-surface, mesh grid with 24 shells, linear elastic); the unmeshed ramp void claim is gone · story chip pointer lands on the planted roof piece away from the callout.

Not done: M21 leaders from the `check` table rows to the drawn arrows and the section-cut marker in `grid` (the plan's own cut marks collided with the caption band) remain open as minor.

### Verification

en and de captured; strict 0 in all captured frames (full runs, then targeted re-captures of the changed keys); every changed frame read in both languages. Tests: see the lines below.

### Round 3 addendum: roof planting (coordinator correction)

The 110 m² jardin suspendu is the first-floor terrace (`SP-TERRACE`, reverted to the first floor by the models owner); it is no longer shown or compared. The clip names the soil area "Roof planting (solarium level)" / "Dachbepflanzung (Solariumebene)": the model has no roof planting area distinct from the slab, so it is the `planted` zone of `tributary(…, "roof")` (roof slab minus the solarium zone and the openings = 200.2 m², starred), and the frame says so ("= roof slab minus solarium and openings (model)"). The solarium line reads "Solarium (roof) 70.0 m² (sourced)". Narration and tables say "roof planting load" / "Last der Dachbepflanzung". q, tributary loads and reactions are unchanged and still read from the model.

### Round 3 test results

- Final en and de full captures: strict 0 findings in all 20 frames each (then a targeted `grid` de re-capture for a caption line break, strict 0).
- `python C/🧪️tests/🐍️.py` (SEMIO_TEACHING_OUTPUT_ROOT set): 138 tests, 137 pass; the one error is `BimBrowserMetadataContract.test_source_discovery_skips_browser_runtime` (`FileNotFoundError`, no `bun`/`node` in this shell). Includes the narration-number test and the caption-fit test for all 20 clauses in both languages.
- `python C/🧪️tests/🎭️presentation/🐍️.py` (PYTHONUTF8=1, no edits during the run): 14 tests OK, including the full lifecycle render of `consistency` in en and de.

## Round 4

Source: `final-verify-exchange-consistency.md` (consistency PASS, 5 minors).

- C1 German `stale` caption: reworded so number and unit sit together ("…sie hält noch Revision 1 mit 105 kN statt 146 kN."), checked in the frame (the shared caption wrapper splits on U+00A0, so the NBSP is kept only as a harmless hint).
- C2 German term: the model's tributary method label is now "Lasteinzugsflächen-Näherung" (`🧮️analysis` `METHOD["de"]`) and the JSON note "Pilotilasten sind Lasteinzugsflächen-Näherungen" (Edit, tests green); the scene's card header reads the label from `tributary()["method"]` / `["method_de"]`, so it follows the model ("Lasteinzugsflächen-Näherung, keine FEM" in `derived` de).
- C3 `retranslate`: "25 base nodes *" / "25 Fußknoten *" (the count read from `counts["base_nodes"]`), with the legend line "25 pilotis in this teaching model, sources differ: 15–25" above it.
- C4 `grid`: section cut on the roof plan along axis 3 (dash-dot line with an "S" chip and view arrow at both ends, all inside the plan) and the legend "S = section" / "S = Schnitt" beside the north arrow, clear of the caption band.
- C5 row leaders: `check` rows carry numbered badges 1 to 4 that repeat on the drawn element (link pill, roof cells, the A3 arrows in model and analysis, both views for the sum); `kept`: every "stays the same" row has a leader to the roof slab, the "changes" card has a leader and the planted roof pieces are highlighted in orange (storey marks are faded in that beat so no leader crosses them); `report`: GlobalId row and link row each lead to a point of the slab.
- Stale `Beat6_Consistency-*-limits.png` deleted.

Verification: en and de full captures, strict 0 in all 20 frames each (one `kept` re-capture after hiding the storey marks, strict 0); changed frames read (`kept`, `check`, `grid`, `stale`, `report`, `derived`, `retranslate`). `python C/🧪️tests/🐍️.py`: 138 tests, 137 pass (only the `bun`/`node` source-discovery test errors, no runtime in this shell). `🎭️presentation` with `SEMIO_TEST_BIM_TOPICS=consistency`: 14 tests OK (an unfocused run was rejected by its own guard because another author changed `🧩️models/🐍️.py` during the run).
