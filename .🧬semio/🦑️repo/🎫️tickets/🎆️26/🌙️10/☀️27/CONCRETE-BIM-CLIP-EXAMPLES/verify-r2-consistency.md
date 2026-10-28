# ✅ Verify r2: 🔗️consistency (`Beat6_Consistency`)

Read-only verification of the current 20-key clip against `agent-rules.md`, `audit-r2-consistency.md`, `scene-consistency.md`, `analysis-model.md`, `research-villa-savoye.md` and `research-ifc.md`. Scene file: `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔗️consistency/🐍️.py` (class `Beat6_Consistency`, NARRATION lines 42–103). Frames re-captured after the scene file (scene 22:06; EN frames 22:31–22:36, DE frames 22:36–22:41) into `🗑️generated/frames/Beat6_Consistency/`. All 40 PNGs (20 EN, 20 DE) were read. Zoom crops: `🗑️generated/verify-consistency/zoom-*.png`. Energy reference frame read: `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/energy-frames/Beat8_Hauptgleichung-chain.png`. The title is intentionally absent and is not reported.

## 1. Verdict

**FAIL.** 0 blockers, 5 majors, 15 minors.

The clip is now a real build-up: the arithmetic holds, the roof-load contradiction (B1) and the one-model recap (B2) are fixed, and the leak, link, fork and check beats draw their concepts on the villa. Release is blocked by five majors: two pointers that do not land on the element they name (identity, metadata), one planted-area and one piloti-count claim that contradict the research without showing it, and a resolve check that compares a value with itself.

## 2. Defects

Frame coordinates are px on the 1280 × 720 keyframe. Severity: blocker = wrong or contradictory content the picture contradicts; major = concept misdrawn, research contradiction, or a check that does not check; minor = polish or small inconsistency.

### Majors

| Key | Sev | Tag | Location | Defect | Fix |
|---|---|---|---|---|---|
| `identity` | major | [sync] | EN and DE frame, GlobalId leader, end at px (580, 225); roof corner at about (580, 295). Zoom: `zoom-en-identity.png`. Code `_identity` line 494. | The GlobalId leader starts in empty space beside the roof. It is anchored on `axon.parts[self.tag]`, the bounding box of the slab part, so it does not touch the slab outline. The beat's claim "a GlobalId names this roof slab" is not drawn. | Anchor on the outline point used in `_metadata` line 544: `np.array(axon.at(21.5, 0.0, self.geometry["storeys"]["roof"]))`. Check that the leader ends on the roof edge in EN and DE. |
| `metadata` | major | [sync] | EN and DE, orange `IsExternal` leader ends at px (333, 372), inside the roof top. Code `_metadata` lines 545 and 547 (`inner` point). | The row `Pset_SlabCommon.IsExternal TRUE *` points to an interior point of the roof, not to an external face. The row for `LoadBearing` goes to the corner. Audit A6 (external edge not highlighted) and L14 (leader in roof garden) are still open. | Point `IsExternal` to the external edge (`edge`, line 544) and highlight that edge while the row is shown. Point `LoadBearing` to a point over a piloti, not to an interior garden point. |
| `grid`, `spread` | major | [numbers] | EN "Planted roof · 200.2 m² *" (px 612–845, 480–500) and DE "Bepflanzte Dachfläche · 200,2 m² *". Code `_grid` line 735. Σ ΔR formula `_spread` line 833: "3.6 kN/m² · 200.2 m² = 721 kN" (DE "200,2 m²"). | The planted roof area is the model value 200.21 m², marked as an assumption. Research fact 2.7 (and 4.2) gives the roof garden as about 110 m² (medium confidence). The frame never shows 110 m². Audit r2 beat B asked that a discrepancy be shown as "model zones: X m²; sourced: 110 m²". | Show both on the grid frame: "planted zone (model) 200.2 m² * · roof garden about 110 m² (sourced, fact 2.7)". Mark the 200.2 m² factor in the Σ ΔR line as "model zone area". Or change the planted zone to the sourced area in the model and re-run the chain. Either way the difference must be visible. |
| `retranslate`, `grid` | major | [numbers] | EN narration line 89 "…and 25 columns"; DE line 90 "…und 25 Stäbe". Counter `_retranslate` line 979: "25 curve members (columns)" / DE "25 Stabelemente (Stützen)". Plan `_grid` shows 25 pilotis on a 5 × 5 grid. | 25 columns is stated as a fact with no assumption mark. Research fact 2.11 says the count is not settled (lex.dk 25 vs student slideshow 15), and the research table says "Do not state a count in the model; mark as open". The scene report claims the count is no longer stated; that holds only for the grid label. Audit N4 reopened. | Mark the count as a teaching assumption: "25 columns *" in the counter and the narration, or write "columns on the grid (count open)". Keep the 4.75 m spacing as sourced (fact 2.10). |
| `resolve` | major | [numbers] | EN and DE verdict box px (513–767, 518–570). Code `_resolve` line 1013. | The verdict reads "A3: 146 kN = 146 kN". Both operands are `self.reaction_new[self.focus]`, so it is an identity, not a comparison of the analysis result with the model. The "0 issues" badge is computed (line 1012); the equation is not. | Read the right-hand value from the translated analysis (A3 reaction in `self.analysis_new`) and show "model 146 kN = analysis 146 kN" only when the computed difference is within rounding; otherwise show the mismatch. |

### Minors

| Key | Sev | Tag | Location | Defect | Fix |
|---|---|---|---|---|---|
| `recap` | minor | [sync] | EN and DE px (14–130, 290–420); code `_recap` lines 1058–1059. | The labels "architecture / structure / services" have no leaders to the layers. "structure" is beige in its label, but the drawn first-floor slab next to it is yellow (the highlight colour). "services" is red over a grey plane. | Add a leader from each label to `full.layers[key]`. Give the structure layer its STRUCT colour and keep yellow for the lit roof only. |
| `spread` | minor | [layout] | EN and DE, card right border beyond px 1280 (`zoom-en-spread.png`). Code `_spread` line 824: `panel(6.4, 2.3, (4.0, 1.75))`. | The card's right edge is at about 7.2 units, past the frame edge (the border is off-frame). Text fits. | Move to `(3.8, 1.75)` with width 6.0 so the right edge is at most 6.8 units; re-check text fit in DE. |
| `check` | minor | [layout][de] | DE table header "tatsächlich (Verknüpfung)" px (898–1146, 352–372). Zoom `zoom-de-check.png`. Code `issue_table` line 918 (`0.74` width). | The header's ")" touches the table's right border. | Use "tatsächlich (Link)" in DE, or set column 3 at `0.70` of the width. |
| `grid`, `stale` | minor | [de] | DE caption, `grid` (line 69, px 120–1175, 615–675): "11,28 / m²"; `stale` (line 78): "105 / kN". | Number and unit split across a line break. | Use a no-break space (U+00A0) between number and unit in every DE narration string. |
| `load`, `change`, `fork`, `decide`, `leak`, `leakpath` | minor | [de] | DE layer name "Pflanzsubstrat" in the drawing and load table; "Erde" and "Erdschicht" in narration (lines 60, 66) and cards (`event_card` line 595, `_kept` line 625, `_fork` line 1033). | One layer is called two things in German. EN uses "soil" throughout. | Use one German term for the layer in the model name and every string (e.g. "Pflanzerde"). |
| `load` | minor | [numbers] | EN and DE table, `load_table` lines 647–650: thickness "0.30 m", density "× 18 kN/m³" unmarked; only the weight carries "*". | The assumed thickness and density are not marked, only the derived weight. | Add "*" to the thickness and density cells where `basis` is assumed. Keep the weight mark. |
| `identity`, `models`, `leak`, `stale`, `resolve`, `ids` and others with the chip | minor | [sync] | Chip header "GlobalId  SL-R" (every beat with the chip; `guid_chip` calls at lines 480, 491, 529, 548, 895, 1009). | The header pairs the local tag with the GUID label. The narration says the tag is for people. | Header "GlobalId" only, with the tag as a separate row, or "object · SL-R". |
| `ids` | minor | [layout] | EN and DE leaders px (415–482 and 798–865, y ≈ 341); code `_ids` line 956. | Both leaders run from the two panels across the villa body into the roof interior (`axon.at(self.tag)`), crossing the drawing. | End leaders at the slab edge and route them above or below the villa, or use one highlight plus one label. |
| `identity`, `models` | minor | [numbers] | EN and DE axon: ribbon bands on both visible faces. Research fact 4.4 says extent is disputed; FR Wikipedia (fact 10.5) gives south façade only. | Drawn as fact with no teaching-choice label (audit N9 open). | Draw the south face only, or label "teaching choice: ribbon on all faces". |
| `stale`, `resolve`, `recap` | minor | [sync] | Analysis head WARN in `_stale` (line 894), GOOD in `_resolve` (line 1008) and the recap card (line 1060). No legend. | The analysis colour changes meaning between beats (audit S6 open). | Keep one colour for the analysis; show state on the LK-ROOF pill only, or add a legend chip. |
| `change` | minor | [sync] | Section roof band, px (48–512, 287–297); code `_change` line 606. | The +0.20 m growth is about 10 px in the section and has no dimension. Audit S3 open. | Add a dimension "+0.20 m" on the roof edge of the section when `FadeTransform` runs. |
| `resolve` | minor | [layout] | Badges "0 issues" px (465–580, 210–240) and "BCF · resolved" px (667–847, 210–242); code lines 1014–1015. | Both float between the headers and the link, attached to nothing. | Attach them to the link pill or to the BCF topic (`report` beat), or place them in the review panel. |
| `kept`, `check`, `report` | minor | [abstract] | `kept` card rows (`_kept` lines 617–631): the slab is not highlighted when "GlobalId" and "ROOF" are stated. `check` table (`issue_rows` lines 901–908): rows have no leader to the drawn 146 / 105 kN arrows. | Audit M21 partly open: the rows are not tied to the drawn element. | Per row, a leader and a highlight on the element it names (audit M21). |
| `retranslate` | minor | [numbers] | Counter "24 surface members", review text "24 shells, 35 slab nodes, 25 base nodes", and "The ramp void of the slab is not meshed". Code `_retranslate` lines 973–1002. | The counts and the unmeshed ramp void cannot be checked on the drawn mesh, which shows no ramp hole (audit N5 open). | Draw the ramp hole in the analysis mesh and colour the shells as a ring, or drop the claims. |
| `grid` | minor | [depth] | Roof plan (`_grid` lines 719–739). | Grid bubbles, north arrow, scale bar and dimensions are present. No section-cut marker shows where the derived section is taken (audit M19 partly open). | Draw a section cut with arrowheads on the plan at axis 3, labelled "section 3". |

## 3. Checks that passed

- **Arithmetic, as printed (EN and DE frames):** q = 5.00 + 0.20 + 1.32 + 5.40 + 1.00 = 12.92 kN/m² (`load`); Δq = 0.20 m × 18 = +3.60 kN/m²; layers 0.20 + 0.02 + 0.06 + 0.30 = 0.58 m (0.38 → 0.58 m); A = 4.75 m × 2.375 m = 11.28 m² and 0.50 m × 4.75 m = 2.38 m² (`grid`); R = 11.28 × 12.92 ≈ 146 kN, from 105 kN, +41 kN (`derived`); C3 2.38 × 12.92 ≈ 31 kN, from 22 kN, +9 kN; D3 11.41 m² × 3.6 = +41 kN; Σ ΔR = 3.6 × 200.2 = 721 kN; total 2,462 → 3,183 kN; edit B q = 11.12 kN/m² (`fork`, `decide`); check table 2 vs 1, 9.32 vs 12.92, 105 vs 146 kN.
- **No 11.78 or 9.11 roof mean anywhere** (B1 fixed): all loads use the planted zone.
- **IFC, IDS and BCF names** against `research-ifc.md`: IFC4X3_ADD2 and ISO 16739-1:2024 (lines 13, 17), IfcSlab with PredefinedType ROOF (lines 39, 74–76), Pset_SlabCommon.LoadBearing and IsExternal (lines 118, 107), 22-character GlobalId (line 184), STEP #101 local (line 190), STEP excerpt field order correct for IFC4X3 (nine attributes), IDS 1.0 (line 259), BCF-XML 3.0 (line 267), BCF component GlobalId (N11 fixed).
- **Historical facts**: 1928–1931 (fact 1.4, medium), "reports: water through the roof each autumn" (fact 8.1, high), gap labelled "illustration · place not documented" (fact 7.4). No storey heights, no 9.40 m level, no +3.50 or +6.70 levels remain.
- **German**: every visible sentence exists in DE (identity, models, metadata and so on). Author values are German (Tragwerksplaner, Atelier (Architektur), Heizungsplaner). Number format is correct (0,10 m, 12,92 kN/m², 3.183 kN, 200,2 m²). Only the English IFC identifiers and the STEP name "Roof slab over the enclosed rooms" stay in English.
- **Narration ⇄ picture sync**: each narration clause in the 20 beats has its object on screen at that beat. Exception: the `identity` and `metadata` pointers (majors above), and `recap` labels (minor).
- **Layout checker**: `Beat6_Consistency-en-layout.md` and `-de-layout.md` report 0 strict findings. My reading found the pointer, tautology and research defects above, which the checker does not cover.
- **Depth against the energy clip**: the energy keyframe (Beat8, Hauptgleichung) uses a concept chip, a reference chip, an equation with substituted numbers and a bar. This clip has all four per beat, plus drawn plans, sections, grid bubbles and dimension strings. Depth is comparable or greater.

## 4. Audit r2 status

| Audit item | Status now |
|---|---|
| B1 one roof load | resolved |
| B2 recap shows one model | resolved in substance (exploded layers, four pills, analysis card, narration rewritten); residual labelling in minor `recap` |
| M1 leak not drawn | resolved (`leakpath`: water path, gap labelled illustration, stain) |
| M2 link as object | resolved (LK-ROOF pill, dashed while stale, solid after resolution) |
| M3 base revision | resolved (`fork`: rev 1 root, edits A and B; `decide`) |
| M4 storey levels | resolved (GF/1F/roof marks only, no +9.40) |
| M5 unsourced layer values | resolved (asterisks and legend); residual in minor `load` |
| M6–M11 layout (badges, clipping, leader through pills, edit B) | resolved |
| M12 overpainted kN labels, M14 growth in stills, M13 plan too small | resolved |
| M15 DE author, M16 DE glyph | resolved |
| M17, M18 concept chips, formulas, numbers | resolved |
| M19 conventions | partly open (minor `grid`) |
| M20 storyboard elements | mostly resolved (IDS, STEP, crack, root tree, explode present) |
| M21 tables tied to drawing | partly open (minor `kept`, `check`, `report`); the pointer part is major `identity` and `metadata` |
| N4 piloti count | reopened as major `retranslate` |
| N5 analysis counts | open (minor `retranslate`) |
| N9 ribbon windows | open (minor) |
| N11 BCF component | resolved |
| A4, A5, A8 | resolved |
| A6 external edge | open (major `metadata`) |
| S3 section soil growth, S6 analysis colour | open (minor) |
| L8, L12, L13, m15, m16, m17 | resolved |
| L14 metadata leaders | open (major `metadata`) |

## 5. Method

- Frames: `capture.py 🔗️consistency Beat6_Consistency en` and `de`, run in full. Both completed with exit 0.
- Zoom crops by `🗑️generated/verify-consistency/crop-verify.py` (PIL, run as a file). Crops: `zoom-en-identity.png`, `zoom-en-metadata.png`, `zoom-en-spread.png`, `zoom-en-recap.png`, `zoom-de-check.png`, `zoom-de-identity.png`, `zoom-de-metadata.png`, `zoom-de-spread.png`.
- No repository source file was edited; nothing was committed or staged.
