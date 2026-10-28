# Storyboard: 🔗️consistency (`Beat6_Consistency`, `bim_consistency`)

Title en "Identity and Consistency" / de "Identität Und Konsistenz". Ten beats, one NARRATION row each. Protagonist: the **roof slab of the Villa Savoye** (`IfcSlab`, PredefinedType `ROOF`), one GlobalId through a design change, a stale structural analysis and its resolution. Phase 1 only: no code edited. Facts follow `research-ifc.md` (sections 8, 10, 11, 14, 15) and `research-villa-savoye.md` (7.4, 8.1, 1.13).

## 1. Stage, zones, persistent elements

Manim stage 14.22 x 8 units, centre (0,0). Title band (top-left) occupies y > 3.3, caption band y < -3.0 (from the baseline frames), so the drawing area is **x in [-6.9, 6.9], y in [-2.85, 3.15]**. Zones used in every beat:

| Zone | Rectangle | Content |
| --- | --- | --- |
| L (building) | x [-6.9, -0.2], y [-2.85, 2.9] | villa drawing: axonometric (beats 1, 2, 10), section (3-5), model-side axonometric (6-8) |
| R-head | x [0.4, 6.9], y [1.5, 3.15] | `guid_chip` (y 2.75) and the three model `badge`s (y 2.05); from beat 2 on, never moves again except beats 6-9 and 10 (below) |
| R-body | x [0.4, 6.9], y [-2.85, 1.5] | concept panel of the beat (property table, roof detail, event card, analysis view, issue table) |
| Full width | x [-6.9, 6.9] | beats 6-10 re-use the full width (two side-by-side drawings, revision tree, recap) |

Teaching model note, shown **once** (beat 1): muted 20 pt line at (-3.4, 2.95): en "Villa Savoye · simplified teaching model" / de "Villa Savoye · vereinfachtes Lehrmodell". It fades out with the axonometric in beat 3.

Persistent encodings (never change meaning during the clip):

- GlobalId = `guid_chip` in DATA yellow, same string everywhere (`global_id(roof_tag)`); a copy of it appears in the event card, the issue table, the BCF card and the revision tree (always created with `TransformFromCopy`/copy of the same Mobject text, so the eye sees it is the identical string).
- Model badges: architecture = ARCH, structure = STRUCT, analysis = GLASS (cyan); orange (WARN) = stale, GOOD = consistent.
- Load arrows: down arrows, length = kN (or kN/m²) by a **single** scale per unit for the whole clip (`ARROW_KN = 1.35 / max reaction of the new model`, `ARROW_Q = 0.55 / q of the new model`), so old and new arrows are directly comparable.

Numbers written in examples below (8.1 kN/m², 271 kN, 0.82 m, 22.6 m² ...) are illustrations of the format only; the scene always reads them from the model.

`roof` below means the element returned by `[s for s in building_geometry()["slabs"] if s["kind"] == "ROOF"][0]`; `tag = roof["tag"]`, `gid = global_id(tag)`; `base = load_model()`, `new = apply_event(EVENT, base)`.

## 2. Beat table

| # | key | caption en | caption de | screen | seconds (anim + hold) |
| --- | --- | --- | --- | --- | --- |
| 1 | `identity` | A 22-character GlobalId names the roof slab in every exchange. The tag is for people. | Eine GlobalId aus 22 Zeichen benennt die Dachplatte in jedem Austausch. Das Kürzel ist für Menschen. | axonometric + tag chip + GlobalId written letter by letter | 3.0 + speech |
| 2 | `metadata` | Metadata name the author, the revision and the models that link to this slab. | Metadaten nennen Urheber, Revision und die Modelle, die auf diese Platte verweisen. | axonometric + property table + three model badges | 2.4 + speech |
| 3 | `leak` | The flat roof leaked every autumn. A redesign adds waterproofing and more soil. | Das Flachdach war jeden Herbst undicht. Ein Umbau ergänzt Abdichtung und mehr Erde. | villa section + magnified roof build-up with rain and seepage | 3.0 + speech |
| 4 | `change` | One event deepens the soil layer. The slab keeps its GlobalId; only the revision rises. | Ein Ereignis vertieft die Erdschicht. Die Platte behält ihre GlobalId, nur die Revision steigt. | event card + soil layer grows (ValueTracker) + revision badge ticks | 3.4 + speech |
| 5 | `derived` | Inside the model, derived values follow: the roof load and every piloti load grow. | Im Modell folgen abgeleitete Werte: Dachlast und die Last jeder Pilotis wachsen. | section with load arrows + tributary plan + derivation table | 3.2 + speech |
| 6 | `stale` | The linked structural analysis still uses the old revision. Its arrows stay short. | Die verknüpfte Tragwerksanalyse nutzt noch die alte Revision. Ihre Pfeile bleiben kurz. | model axonometric (green arrows) vs analysis view (orange short arrows) | 3.0 + speech |
| 7 | `check` | Links record identity and revision, so a check finds the stale result: expected versus actual. | Verknüpfungen halten Identität und Revision fest, so findet eine Prüfung das veraltete Ergebnis. | issue table (expected / actual) + BCF topic card + IDS note | 3.0 + speech |
| 8 | `resolve` | Regenerate and review the translation, then update the link. The arrows match again. | Übersetzung neu erzeugen und prüfen, dann die Verknüpfung aktualisieren. Die Pfeile passen wieder. | review checklist, link badge 1 to 2, arrows grow and turn green | 3.6 + speech |
| 9 | `limits` | A GlobalId does not synchronize or resolve conflicts; two edits need a human decision. | Eine GlobalId synchronisiert nicht und löst keine Konflikte; zwei Änderungen brauchen eine Entscheidung. | revision tree: one base, two edits of the same revision, same GlobalId, differing soil depth | 3.2 + speech |
| 10 | `recap` | BIM links meaningful data, objects, representations and models through schemas and traceable relationships. | BIM verbindet Daten, Objekte, Darstellungen und Modelle durch Schemata und nachvollziehbare Beziehungen. | full villa axonometric exploded into three discipline models with link lines | 3.4 + speech |

All clauses are 74-106 characters (one caption, at most two lines). Captions contain no model numbers (revision and values are on the drawing), so they stay true if the base revision changes.

## 3. Beat details

Common: every step is `self.step(key, *animations, seconds=...)`; `self.number(v, d)` formats numbers, `self.tr(en, de)` all labels. Values quoted as `f(...)` come from the model API of `api.md` section 2, never typed.

### Beat 1 `identity`

On screen:

1. L zone: `villa_axon(point=(-3.4, 0.0), scale=0.17, highlight=(tag,), disciplines=("architecture","structure"))`, drawn in grey outline/ghost, the roof slab lit DATA yellow. Drawing animates in with `Create`/`FadeIn` lag over 1.4 s, the roof slab lighting last. Axonometric footprint is about 6.2 x 5.2 units; top edge stays below y = 2.7 so the teaching-model note fits.
2. Muted note "teaching model" at (-3.4, 2.95) (see section 1).
3. `leader(axon.parts[tag], (0.55, 1.25))` from the lit slab to the right zone.
4. R-body (this beat uses R-head and R-body as one area): tag chip `badge(tag, (3.6, 1.95), color=MUTED)` with small label under it "short tag · for people" / "Kürzel · für Menschen".
5. `guid_chip(tag, (3.6, 0.55))`, width 6.0, class label above it "IfcRoot.GlobalId · IfcGloballyUniqueId" (real IFC names). The 22 characters of `gid` write in letter by letter (`AddTextLetterByLetter`, 2.2 s) while a `DecimalNumber` below counts 0 to `len(gid)` (value from the string) with the unit text "characters" / "Zeichen"; a `Brace` under the chip spans the string and carries that number, so the value (22) is shown at the same time as the string it counts.
6. Contrast chip (muted, y = -1.0): "#n · STEP number, local to one file" / "#n · STEP-Nummer, nur lokal in einer Datei", where `#n` is read from `step_excerpt(tag)["lines"]` (the `#n=IFCSLAB('gid'` line); a thin line from the `#n` to the chip's GlobalId with a red cross on the `#n` side is not needed, instead the GlobalId chip gets a GOOD underline "same in every exchange" / "in jedem Austausch gleich".

Animation order: axon in, roof slab lights, leader draws, tag chip fades in, GlobalId writes with counter and brace, STEP contrast chip last.

Values and meaning: the count 22 is `len(global_id(tag))` and is drawn as the brace over the actual 22 characters; the lit slab on the villa is what the string identifies.

### Beat 2 `metadata`

On screen:

1. Axonometric stays in L (same position, scale), colour of roof slab stays DATA; the three discipline layers show in ARCH/STRUCT colours at low opacity.
2. GlobalId chip scales to 0.62 and moves to (3.7, 2.75) (`guid_chip(...).animate`), tag chip moves left of it and shrinks; the chip's string is the same Mobject, so identity visibly stays.
3. Three `badge`s in R-head at y = 2.05 (x = 1.45, 3.7, 5.95): `badge(f"{tr('architecture','Architektur')} · {tr('rev','Rev.')} {partial_model('architecture')['revision']}", color=ARCH)`, `badge(... 'structure','Tragwerk' ..., color=STRUCT)`, `badge(f"{tr('analysis','Analyse')} · {tr('rev','Rev.')} {link_revision}", color=GLASS)` where `link_revision = load_model()["links"][0]["source_revision"]`. Thin coloured leaders run from each badge to the lit slab (`leader`), so each badge visibly points at the same drawn element.
4. `property_table` in R-body (3.7, -0.35), width 6.0, rows (all from `element(tag)` and `information`-style psets, bilingual keys):
   - IFC class: `IfcSlab · ROOF` (`class`, `predefined_type`)
   - `Pset_SlabCommon.IsExternal` and `.LoadBearing` (from `element(tag)["psets"]`, shown as TRUE/FALSE per model)
   - author: `element(tag)["metadata"]["author"]`
   - revision: `element(tag)["metadata"]["revision"]`
   - storey: `element(tag)["storey"]` name in the clip language
   Rows write in top to bottom, 0.25 s apart.

Animation: chip shrinks to header, badges grow from the lit slab along their leaders (`GrowFromPoint` at the slab anchor), table rows write in.

### Beat 3 `leak`

On screen:

1. L zone: axonometric fades out (note "teaching model" also), `villa_section(point=(-3.5, 0.0), scale=0.30, cut="ramp", highlight=(tag,), layers=True)` fades in, spanning x [-6.7, -0.3], y [-1.5, 1.4]; level marks +/-0.00, first-floor and roof level from the model; roof build-up layers visible at the roof. A small circle marks the roof area above the salon; thin zoom lines run from the circle to the detail frame in R-body.
2. R-body: **roof build-up detail**, magnified: layers of the roof slab's `IfcMaterialLayerSet` (read from `element(tag)["material"]` layers; names in clip language, e.g. concrete, waterproofing, screed, soil) stacked as coloured rectangles at `DETAIL_SCALE = 2.6` screen units per metre, anchored at the underside of the concrete at (3.0, -2.65), width 3.0. If the toolkit has no roof-detail widget, the scene draws them itself with `Rectangle` per layer, `hatch()` on the concrete and `dimension()` for total thickness. Per-layer swatch label to the right: "soil · 0.30 m" etc. (thickness from `quantity(layer, "thickness")`/layer record), colours identical to the layers in the section, so a value is read off the layer it belongs to.
3. Historical text card (R-body top, y 0.8): en "1928-1931 built · water through the roof every autumn" / de "1928-1931 erbaut · jeden Herbst Wasser durchs Dach"; years from `load_model()["building"]` (built from/to). Muted label "historical fact" / "historischer Fakt".
4. Rain: 7 GLASS blue drops fall from above onto the soil in the detail and the same in the section (updater-driven, 3 s). Where a hairline crack (red short line) crosses the waterproofing layer, 3 seepage drops continue slowly through screed and concrete down to the ceiling edge, where a damp stain ellipse (low-opacity GLASS) appears on the underside. The chip and badges from beat 2 stay in R-head (rev 1 everywhere).

Animation: section swaps in, detail swaps in, rain loops with seepage, stain appears; leak is shown as a drawn cause (crack at the waterproofing) rather than a text claim. Truthful: leaks "each autumn" is fact 8.1 (high); the crack position is a drawing device.

### Beat 4 `change` (the event)

On screen:

1. R-body upper (y [0.25, 1.45]): **event card** = `property_table` width 6.0, 4 rows from the real event dict `EVENT = {"type":"LayerThicknessChanged","object_guid":gid,"layer":"soil","value":SOIL_NEW,"unit":"m"}`:
   `type · LayerThicknessChanged`, `object_guid · <gid>` (copy of the chip text, `TransformFromCopy` from the chip), `layer · soil`, `value · SOIL_NEW m`. Header tag "design event · teaching scenario" / "Entwurfsereignis · Lehrszenario" (so the redesign is not read as the 1985-1997 works, whose content the research does not know). `SOIL_NEW = soil_old + SOIL_DEEPER` with `SOIL_DEEPER = 0.20` m authored once in the scene as the event payload; everything displayed afterwards is read back from `new`.
2. R-body lower: the detail from beat 3 (same position and scale) with a `ValueTracker soil` going from `soil_old` to `SOIL_NEW` over 3.0 s; the soil rectangle, everything above it, and the total-thickness `dimension()` are redrawn with `always_redraw`; the total reads `soil + other layers` via a `DecimalNumber` (en "0.82 m", de "0,82 m" formatting through `self.number`). A yellow `dimension()` marks the added 0.20 m (appears when the tracker ends: "+ 0.20 m"; value = `SOIL_NEW - soil_old`). The waterproofing layer pulses GOOD with a small flag "new waterproofing" / "neue Abdichtung" (layer record from the model; if the layer thickness is unchanged by the event, only the highlight shows, no number).
3. L zone: the section is replaced at the end of the tracker by `villa_section(model=new, ...)` (`FadeTransform`), so the roof build-up on the villa itself is the new thickness, with a pulsing yellow outline around the thickened roof.
4. R-head: the architecture badge text changes "rev 1" to "rev 2" via `Transform` (DATA flash), values `partial_model("architecture", new)["revision"]`; the structure badge shows `partial_model("structure", new)["revision"]` whatever the model returns; the analysis badge **stays unchanged** (still the link revision of the old state; it turns WARN in beat 6). The GlobalId chip does not move or change; a GOOD label beneath it reads "same GlobalId · revision N to M" / "gleiche GlobalId · Revision N zu M" with `N = base["revision"]`, `M = new["revision"]`, asserted `element(tag, new)["guid"] == gid`.

Animation order (3.4 s): event card writes (0.8 s), `ValueTracker` run (soil grows while rain stops), revision badge tick, "+ 0.20 m" dimension, GlobalId label.

### Beat 5 `derived`

On screen:

1. L zone: the new-model section stays. Roof load as a uniform load: `load_arrows([q]*n, roof_points, ARROW_Q)` with `n = 9` evenly along the roof top (above the soil), length = q (kN/m²); a `ValueTracker q` goes from `tributary(base)["area_load"]` to `tributary(new)["area_load"]` (3.0 s) driving arrow length via `always_redraw`; label "q = 8.1 kN/m²" (value counts via `DecimalNumber` from the tracker, formatted by language).
2. Same section, below the ground line: reaction arrows (up) under the cut pilotis (row C: tags C1 to C5 as exposed by the section's `parts`): `load_arrows(values, points, ARROW_KN)` with values `[p["reaction"] for p in tributary(model)["pilotis"] if p["tag"] in row_C]`, tracker-driven from old to new; each tipped with its kN value.
3. R-body: left `tributary_view(tributary(new), point=(1.9, -1.1), scale=0.11, highlight=("C3",))`: 25 cells in plan (hatched cell C3 in DATA, other cells outline), small load glyph per cell. Right: `property_table` at (5.0, -0.9), width 3.8, for the highlighted piloti: `tributary area A · 22.6 m²` (`area`), `roof load q · old to new kN/m²`, `load on piloti C3 · old to new kN` (`reaction`), counted with `DecimalNumber`s. If `reaction == area * area_load` in the model, an extra line `A x q = R` is shown (verify at implementation; otherwise omit it).
4. The soil and arrows share DATA yellow for the changed quantities; unchanged values (A) stay INK.

Animation: arrows grow from old to new length together with their numbers; the C3 cell pulses once. Hand-off to next beat: the arrows' final lengths are the reference for "up to date".

### Beat 6 `stale`

Layout: full width. L (x -6.9..-0.2): `tributary_view(tributary(new), point=(-3.5, -0.15), scale=0.16, highlight=row_C)` rendered as axonometric with up-to-date reaction arrows for row C in GOOD green (arrows only for C1-C5 in this projection, so diagonal neighbours never overlap; all other pilotis get a small support dot). R (x 0.2..6.9): `analysis = translate_structure(base)`; `analysis_view(analysis, point=(3.5, -0.15), scale=0.16, loads=old_reactions_row_C, stale=True, highlight=row_C)`: shell mesh on the slab outline (`IfcStructuralSurfaceMember`), pilotis as curve members (`IfcStructuralCurveMember`), supports, arrows in WARN orange at the **old** lengths.

Headers: `badge("architecture + structure · rev M")` over L in ARCH/STRUCT colour; `badge("analysis · uses rev N")` over R in WARN (the analysis badge from beat 2 flies down and turns WARN). The GlobalId chip (R-head position y 2.75, now centred at (0, 2.75), scale 0.7) has a thin leader to each header: the same identity on both sides.

Gap shown by meaning: on the analysis arrows, a dashed GOOD extension continues each orange arrow to the new length, labelled for C3: "-N kN" where `N = new_reaction - old_reaction` (both from `tributary`); a mirrored value chip over each side: "C3 · 271 kN" (green) vs "C3 · 205 kN" (orange). Orange outline pulse on the analysis surface.

Animation: left drawing slides to the left half (section dissolves into tributary axonometric), analysis view builds (mesh sweep, supports pop, arrows grow to old length, turn orange), dashed extensions draw, delta label counts up.

### Beat 7 `check`

Layout: both drawings shrink `scale 0.6` and move to y = 1.0 (L at x -3.5, R at x 3.5; chip stays between them at y 2.75). New in the lower half (y -2.85..-0.2):

1. **Issue table** `property_table` at (-2.4, -1.5), width 8.6, three columns *issue | expected | actual*, one row per entry of `consistency_issues(new)` (up to 3): e.g. `stale-link · slab <tag> · revision`: expected `revision of new`, actual `source_revision of link`; `stale-value · roof load kN/m²`: expected q (new), actual q (link `used`). Codes shown as model returns (`stale-link`, `stale-value`), with localized explanatory words. Expected values in GOOD, actual values in WARN; the GlobalId chip copy appears as the object column (`object_guid`). Rows animate in 0.4 s apart; each row draws a leader to the matching drawing (revision badge, the short arrow).
2. **Link record** line above the table (y -0.75): "link · object_guid = <gid> · source_revision = N" from `links[0]`, with the chip text reused; a small `<=>` marker to the current revision M.
3. **BCF topic card** (right, x 3.2..6.8, y -0.4..-1.6): `property_table` "BCF topic · Stale analysis result" / "BCF-Thema · Veraltetes Analyseergebnis" with rows: status `open`, component `GlobalId` = chip copy, comment "analysis uses rev N, model has rev M". Text note under: "an issue points to the object by its GlobalId" / "ein Hinweis zeigt über die GlobalId auf das Objekt". Truthful: BCF carries issues and refers to elements by GlobalId; it is not a model format.
4. **IDS pill** (right, y -2.0..-2.7): `badge("IDS · Pset_SlabCommon.LoadBearing present  OK")` in GOOD with tiny caption "checks required information, not revisions" / "prüft geforderte Information, nicht Revisionen". The check result is read from `"LoadBearing" in element(tag, new)["psets"]["Pset_SlabCommon"]`. Teaching point: IDS passes while the link check fails.

Animation: panels shrink (1.0 s), link line then issue rows write in, BCF card and IDS pill fade in. No new numbers are typed; all five values are read from the model.

### Beat 8 `resolve`

Layout stays as in beat 7 but the lower half changes.

1. **Regenerate**: `analysis_new = translate_structure(new)`; the right drawing re-draws (mesh sweep animation 0.8 s) with `analysis_view(analysis_new, ..., stale=False)`.
2. **Review** (lower-left, replaces the issue table by `FadeTransform`): checklist from `analysis_new["assumptions"]` (localized `{en,de}`, three entries, each with a ticking GOOD check mark one after the other, 0.5 s apart) and the first three entries of `analysis_new["losses"]` as muted lines "not transferred: ..." (e.g. finishes, ribbon windows) so the reviewer sees what the translation drops. Header "review by a person" / "Prüfung durch einen Menschen".
3. **Update the link**: the analysis badge text "rev N" counts to "rev M" (`Transform`), WARN to GOOD; `links` record text line `source_revision N to M`. A scene-side step `resolved = resolve_link(new, link_id)` (request in section 4; fallback: copy of `new` with the link set from `analysis_new`) gives the model that is checked.
4. The orange analysis arrows grow to the new lengths and become GOOD, the dashed extension and the delta label dissolve; both sides now show `C3 · 271 kN`-style identical values (from `tributary(new)` and `analysis_new` loads).
5. The issue table collapses into one GOOD line "0 issues · link up to date" / "0 Abweichungen · Verknüpfung aktuell", count `len(consistency_issues(resolved))`; the BCF card status flips `open` to `resolved` (text transform); the IDS pill remains.

Animation order: regenerate, review ticks, link badge 1 to 2, arrows grow + colour, issue table collapses.

### Beat 9 `limits`

Layout: full width, everything from before fades; the revision tree is the only drawing.

1. Top centre (0, 2.7): GlobalId chip, scale 0.9 (same string).
2. Root node (0, 1.35): small card "rev N · soil X m" with a two-layer mini bar (soil, rest) at `MINI_SCALE = 1.4` per metre, values from `base`.
3. Two arrows from the root to two children at (-3.3, -1.0) and (3.3, -1.0): **two independent edits of the same revision**: `new_a = apply_event(EVENT, base)` (soil `SOIL_NEW`, author "architect") and `new_b = apply_event(EVENT_B, base)` with `EVENT_B` soil `soil_old + SOIL_ALT` (`SOIL_ALT = 0.10` m, author "landscape designer"), both revision `M`. Each child shows a mini build-up bar (same scale) with the value read back `quantity(...)` and the same chip copy; label "rev M · soil 0.50 m" / "rev M · soil 0.40 m" (en/de decimal comma).
4. Between the children a large WARN "not equal" sign and the line "same GlobalId, different content" / "gleiche GlobalId, anderer Inhalt".
5. Below the sign (0, -2.0): a simple person glyph (circle + shoulders) with the label "a person decides · review and merge" / "ein Mensch entscheidet · prüfen und zusammenführen" and a crossed-out "automatic" gear label "no automatic synchronization" / "keine automatische Synchronisation".

Truthful: a GlobalId is identity, not a version or a merge mechanism; no IFC claim is made about merge tooling.

Animation: tree draws from the root, the two bars grow side by side (2 s), sign pulses, person glyph fades in.

### Beat 10 `recap`

Layout: everything fades; `villa_axon(point=(-1.4, 0.15), scale=0.21, disciplines=("architecture","structure","services"))` full villa, shaded, all three disciplines in ARCH/STRUCT/SERV colours.

1. Beat 10.0: assembled villa (`explode=0.0`) with the roof slab lit and the GlobalId chip on it (leader).
2. `explode` animates 0.0 to 0.6 (ValueTracker, 1.8 s) so the three discipline models separate vertically, each with its revision badge `partial_model(id)["revision"]` (all GOOD now); thin GOOD **link lines** run between the roof slab in each layer and to a fourth card, the analysis model (GLASS badge rev M, the one regenerated in beat 8), annotated with the same chip copy. The lines carry labels "GlobalId" and "rev M".
3. R side (x 3.3..6.8): four `badge`s, one per word: `schema · IFC4X3_ADD2` (read from `load_model()["schema_id"]`), `GlobalId`, `revision`, `link`; each lights in sequence (`Indicate`) together with the thing it names on the drawing (schema label on the storey, chip, badge row, link lines).
4. Final 0.8 s: `explode` returns to 0.0 (one villa, many consistent models); all revision badges GOOD.

Truthful: no ISO 19650 status codes (UK convention) and no claim that IDS or BCF are model formats.

## 4. Model and toolkit needs (verify against `T/toolkit-ready.md`; tell the owners if missing)

Used API (api.md): `load_model`, `element`, `quantity`, `building_geometry` (ROOF slab, pilotis row C), `global_id`, `step_excerpt` (the `#n` line), `partial_model`, `translate_structure`, `tributary`, `apply_event` (`LayerThicknessChanged`), `consistency_issues`. Widgets: `villa_axon`, `villa_section(layers=True)`, `guid_chip`, `badge`, `property_table`, `analysis_view(stale=...)`, `load_arrows`, `tributary_view`, `leader`, `dimension`, `hatch`.

Requirements I rely on (assumptions, stated so the owners can confirm or change):

1. `element(roof_tag)["material"]` exposes the layer set with names and thickness; `quantity`/layer lookup works per layer (needed for detail and soil value). `apply_event(LayerThicknessChanged)` also bumps element revision and `partial_model` revisions of the new model.
2. `tributary(model)` reports the **roof** load: `area_load` includes the build-up self-weight (layer thickness x density from `loads`), so soil depth changes `area_load` and `pilotis[*].reaction`. If it only reports a floor-slab load, I need `tributary(model, slab=<roof tag>)` or equivalent.
3. `consistency_issues(new)` returns, for the stale link, entries with `expected` and `actual` (revision and the used load) and the same `object_guid` as the roof slab; the stored link (`links[0]`) refers to the roof slab with `source_revision` and `used = {area_load, reactions}` from the old state.
4. **Requested addition (model agent)**: a function or event that records the regenerated translation in the link, e.g. `resolve_link(model, link_id) -> model` (link `source_revision` = current revision, `used` = values of `translate_structure(model)`), no revision change. Fallback: the scene copies the model dict and sets the link fields from `translate_structure`.
5. `villa_section(layers=True)` draws the roof build-up per layer; the section cut passes through one piloti row so five pilotis stand in section with exposed tags (`parts`). `villa_axon`'s `parts[tag]` is the leader target for the lit slab and the discipline layers can be colour-separated by `explode`.
6. `analysis_view(..., stale=True)` colours loads and badge-compatible outline WARN; `load_arrows` draws vertical arrows with one common scale; `tributary_view` and `analysis_view` use the same axonometric projection as `villa_axon` so vertical arrows on row C do not overlap. If one of them is plan-only, the scene switches to `villa_elevation` (south face, row C visible) with the same arrows.
7. Optional (toolkit): a `roof_detail(point, scale, model, soil=None)` widget for the magnified build-up; otherwise the scene draws it from the layer records (rectangles, `hatch`, `dimension`).

Scene-authored constants (event payloads, not displayed as model results): `SOIL_DEEPER = 0.20` m and `SOIL_ALT = 0.10` m; the arrow scales and the mini-bar scale. Every number on screen is read from `base`, `new`, `new_b`, `resolved`, `analysis`, `analysis_new`, `tributary(...)`, or `consistency_issues(...)`.

## 5. Fact check of this storyboard

- GlobalId: 22 characters, custom 64-symbol alphabet, stable across exchanges; STEP `#n` is a file-local reference (research-ifc 8, 9, 14.4-5). The count 22 is computed.
- Roof class `IfcSlab` with `PredefinedType ROOF` (IfcSlabTypeEnum [P]); build-up as `IfcMaterialLayerSet` [M], labelled as the modelling choice.
- Leaks each autumn through the flat roof: fact 8.1 (high). The redesign is explicitly a "teaching scenario"; the 1985-1997 restoration (fact 1.13) is not claimed to have changed this (14. in the open questions: unknown).
- Analytical model: slab to `IfcStructuralSurfaceMember`, pilotis to `IfcStructuralCurveMember` [P/D]; reactions come from the tributary-area estimate, labelled "estimate" in the table header ("Einflussflächen-Näherung"), not an FEM result.
- BCF: issue carrier that refers to elements by GlobalId, not a model format (11, 14.11). IDS: required information, not revisions (11, 15.13). No UK status codes (S0-S4) and no ISO 19650 claims. A GlobalId does not sync or merge (beat 9).
- Villa values: all geometry from the model (box 21.5 x 19 m, grid 4.75 m, pilotis row), "teaching model" noted once (beat 1).
