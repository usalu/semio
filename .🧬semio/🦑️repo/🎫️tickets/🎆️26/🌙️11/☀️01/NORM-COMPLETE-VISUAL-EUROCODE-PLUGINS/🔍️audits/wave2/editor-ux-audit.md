# 🔍️ Editor UX Audit, Wave 2 (EN 1990 to EN 1999)

Read-only audit by a Haiku auditor. No source edits, builds, cargo runs or git writes. Scratch scripts and JSON are in `TICKET/🗑️generated/editor-ux/` (`leaves.py`, `leaves.json`, `grip-name-match.json`). Ticket folder `TICKET` = `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`.

`ART(code)` below means `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any`. Scene paths are relative to `ART(code)/🎬️scene/` unless stated.

## 🧭️ Method and measures

Static evidence only. Nothing was run.

1. **3D**: collection is a kind in the artifact's `ENTITY_KINDS` (`🧰️kit/🦀️.rs`), counted per top-level collection.
2. **Grips**: collection has a grip in the scene (`🕹️handles/`, or grip-producing modules such as `🏛️members/`, `🧱️entities/🕹️grips/`, `🛡️safety/`). Collection-level grip coverage comes from `visual-coverage.md`, corrected where the grip lives outside `🕹️handles/` (see section 4). Leaf-level: a scalar leaf is "grip-bound" when its name appears in a scene file that also contains grip or handle code. This is a proxy. Geometric leaves are matched with a name regex, and a miss can still be a naming false negative.
3. **Diagram**: collection is read by `📈️diagrams/🦀️.rs` (from `visual-coverage.md`).
4. **Live path**: `live_scene!` in `🎬️scene/🧰️kit`, `live_scene_of` used by editor and viewer (4 call sites each), live test `🎬️scene/🧪️tests/🔬️live/🦀️.rs` present for all ten. The `SceneDelta` and scene memo live in the shared `📦️norm/🎬️structural-scene/🎯️projection/🗂️scene-memo` and `🛰️live-scene`.
5. **Edit rules**: every scalar leaf, nested leaf and collection has a `SetFieldRule`, `InsertItemRule` or `RemoveItemRule`/`IdRemoval` in `🧬️schema/🧬️mutations/🧭️edit-rules/**`. Checked by `leaves.py` on the snapshot schema (`🧬️schema/📸️snapshot/🔣️.json`). Nested object rows (for example `beams[].steel.rM`) are matched by path prefix. Inspector exposure is assumed generic from the schema (`🎭️modes/✏️edit/🪟️windows/🧾️properties`), not verified per leaf.
6. **Check-to-entity**: `check_focus_effects` and `interaction_topology` are wired in all ten editors (`✏️editor/🦀️.rs`, `shared-v11.md` section 1). Binding of check families to a scene entity (`subject.entity_id` matches an entity) is the remaining question. Figures from `visual-coverage.md` (literal scene binding per family).

## 🗺️ Table per code

Counts are `covered/total`. "Geo bound" is the leaf-name proxy from measure 2 (upper-bound style: includes derived leaves that are not user inputs).

| Code | Colls | (1) 3D | (2) Grip colls | Leaves (top + row) | Geo leaves grip-bound (proxy) | (3) Diagram colls | (4) Live | (5) Leaves with rule | Insert (excl. case) | (6) Check families bound |
|---|---|---|---|---|---|---|---|---|---|---|
| ⚖️en1990 | 14 | 14/14 | 8/14 (bridgeSls has grips, false neg, so 9/14) | 101 (89+12) | 17/31 | 8/14 | yes | 101/101 | 11/13 | 9/17 |
| 🏋️en1991 | 22 | 22/22 | 21/22 | 210 (131+79) | 67/70 | 5/22 | yes | 210/210 | 20/21 | 7/7 (report_owner) |
| 🏛️en1992 | 7 | 4/7 (grades and prestress not drawn, by design) | 2/7 (members richly gripped, see 4) | 58 (52+6) | 9/18 | 1/7 | yes | 58/58 | 2/6 | 10/36 |
| 🔩️en1993 | 26 | 25/26 (loadCases not drawn) | 15/26 | 492 (490+2) | 145/276 | 6/26 | yes | 492/492 | 21/26 | 9/21 |
| 🧩️en1994 | 4 | 4/4 | 3/4 | 61 (54+7) | 25/29 | 3/4 | yes | 61/61 | 3/4 | 6/7 |
| 🪵️en1995 | 13 | 13/13 | 4/13 as literal; typed rows probably gripped via drawing rows (4 below) | 152 (149+3) | 35/78 | 5/13 | yes | 152/152 | 11/12 | 4/17 |
| 🪨️en1996 | 5 | 5/5 | 4/5 | 64 (59+5) | 19/36 | 2/5 | yes | 64/64 | 3/4 | 14/29 |
| 🌍️en1997 | 13 | 13/13 | 11/13 (uplift grips exist, see 4) | 142 (135+7) | 48/71 | 8/13 | yes | 142/142 | 11/12 | 7/33 |
| 🫨️en1998 | 14 | 14/14 | 12/14 | 178 (177+1) | 66/93 | 1/14 | yes | 178/178 | 13/14 | 23/48 |
| 🪶️en1999 | 15 | 15/15 | 14/15 | 91 (89+2) | 45/50 | 7/15 | yes | 91/91 | 11/15 | 5/7 |

Per code notes:

- **⚖️en1990**: no diagram for bridgeSls, effects, fatigueSpectra, factorOverrides, trafficGroups, placements. No grip for seismics, testSeries, trafficGroups, effects, placements. effects and placements are outliner-only (no insert). trafficGroups has no showcase row. 8 of 17 check families unbound (`en1990.6`, `en1990.e10`, `en1990.e11`, `en1990.e12`).
- **🏋️en1991**: best-covered grips. 17 of 22 collections have no diagram. accidentalCases has no grip (and nested `impact` and `explosion` have none). Six collections lack showcase rows (helicopterLandings, tieRuns, constructionLoads, fallingLoads, executionImperfections, derailmentCases). Check binding is complete via `report_owner` (`🎬️scene-members/🧰️kit/🦀️.rs`), which is the pattern to copy.
- **🏛️en1992**: almost no diagrams (only `members`). anchors, placements, cases and the three grade tables have none. 26 of 36 check families unbound. Member geometry grips exist (span, width, height, cover, bars, stirrups, tendons, openings) in `🏛️members/🦀️.rs`, but `RcMember.sectionPoints` has no grip.
- **🔩️en1993**: 20 of 26 collections have no diagram. loadCases has no 3D, no grip, no insert, no diagram. materials and sections have no grip and are outliner-only. 12 of 21 check families unbound.
- **🧩️en1994**: placements has no grip or diagram. Otherwise the strongest small artifact. 1 unbound check family.
- **🪵️en1995**: nine typed collections (tapers, cutouts, fasteners, panels, floors, diaphragms, trusses, bracings, decks) get grips from their case rows (see 4). Diagrams are missing for tapers, cutouts, panels, diaphragms, trusses, bracings, decks and placements. Showcase rows missing for diaphragms, trusses, bracings, decks. 13 of 17 check families unbound.
- **🪨️en1996**: chases and movementJoints have no diagram. placements has no grip. cases is not insertable (by design).
- **🌍️en1997**: slopes, upliftCases, anchors, pileGroups, layerWaters have no diagram. 26 of 33 check families unbound (the biggest share of any artifact).
- **🫨️en1998**: 13 of 14 collections have no diagram. Seismic results live only in the checks table. 25 of 48 check families unbound.
- **🪶️en1999**: outlineVertices has no diagram. materials has no grip (non-geometric). materials, sections, placements, outlineVertices are not insertable. transverseStiffeners not in a showcase row.

## 🔎 Corrections to `visual-coverage.md` (false negatives)

These were marked as gaps there and are gripped in code. Do not act on them as gaps. Verify in the viewport before closing.

- ⚖️en1990 `bridgeSls`: grips in `🕹️handles/🦀️.rs` lines 247 to 257 (`binding(bridgeSls[id=…].{field})`).
- ⚖️en1990 `placements` move grips: `🕹️handles/🦀️.rs` line 266 onwards (`placements[id=MEMBER].x|y|yaw`). The same member move grip exists in every artifact through `move_handles`, and the shared `🕹️direct-manipulation/📍️placement-drag` lane supplies derived yaw. What is missing is a grip on the placement row itself, not on placement drag.
- 🌍️en1997 `upliftCases`: grips in `🕹️handles/🦀️.rs` `uplift_cases` (line 252 onwards, stabilising, destabilising-g, destabilising-q).
- 🏛️en1992 members: grips in `🏛️members/🦀️.rs` (span, width, height, effective-depth, cover, bars, stirrups, tendons, fire axis distance). `🕹️handles/🦀️.rs` is a 183-line shim, so its size understates coverage.
- 🪵️en1995 nine typed collections: `🧱️entities/🕹️grips/🦀️.rs` and `🕹️handles/🦀️.rs` `entity_handles` (line 161 onwards) build grips from the drawing row via `drawing_row` and `homes` (`🧬️schema/⚖️timber/🧱️entities/🦀️.rs` line 883 onwards). Only inputs with a `Home` (for example taper `alpha_deg`, `h_ap`, `r_in`) get a grip. Verify per table which geometric inputs have homes (apex depth, span, width, thickness, spacing) before adding grips. The grip gap is smaller than visual-coverage says.

## 🏆 Ranked 20 biggest gaps (user impact)

File paths are full or start from `ART(code)`. "Missing" is what a Sonnet agent should add.

1. **en1998 diagrams, 13 of 14 collections.** Missing: drift and base-shear diagrams for buildings and towers, capacity curves for piers and bridges, pressure and fill-level for silos and tanks, response for accelerograms. File: `ART(en1998)/🎬️scene/📈️diagrams/🦀️.rs` (only buildings drawn).
2. **en1993 diagrams, 20 of 26 collections.** Missing for joints, coldFormedMembers, platedPanels, siloShells, tensionComponents, bridgeFatigue, towerLegs, piles, craneRunways, storeys, builtUpMembers, weldGroups, parametricSections, hollowJoints, jointRows, frameNodes, frameBars. File: `ART(en1993)/🎬️scene/📈️diagrams/🦀️.rs`.
3. **en1993 geometric section and profile inputs not grip-bound.** Roughly 131 of 276 scalar leaves are not named in any grip-bearing scene file. Many are derived section properties (`sections.iy`, `wElY`, `wPlY`, `areaNet`, `shearArea*`) and should stay read-only, but profile inputs (h, b, tw, tf, area as input) should get grips on the member that references the section. File: `ART(en1993)/🎬️scene-members/🕹️handles/🦀️.rs` and `ART(en1993)/🎬️scene-members/🧰️kit/🦀️.rs`.
4. **en1991 diagrams, 17 of 22 collections.** Missing for windFaces, roofSurfaces, bridgeLanes, constructionStages, specialVehicles, windBridges, impactActions, thermalSections, selfWeightElements, accidentalCases, placements, helicopterLandings, tieRuns, constructionLoads, fallingLoads, executionImperfections, derailmentCases. Wind-pressure plan, lane-position and construction-stage timeline are the obvious ones. File: `ART(en1991)/🎬️scene/📈️diagrams/🦀️.rs`.
5. **en1997 check families unbound, 26 of 33.** Failing checks do not highlight their entity in 3D. File: `ART(en1997)/🎬️scene/🧰️kit/🦀️.rs`. Add a check-id-prefix to entity (or aggregate) map as `ART(en1991)/🎬️scene-members/🧰️kit/🦀️.rs` does with `report_owner`, plus a test that every spec-trace id resolves.
6. **en1992 check families unbound, 26 of 36.** Same fix. File: `ART(en1992)/🎬️scene/🧰️kit/🦀️.rs`.
7. **en1998 check families unbound, 25 of 48.** Same fix. File: `ART(en1998)/🎬️scene/🧰️kit/🦀️.rs`. Includes `.analysis`, `.componentCombination`, `.displacement`, `.elevationRegularity` families.
8. **en1995 check families unbound, 13 of 17.** Same fix. File: `ART(en1995)/🎬️scene/🧰️kit/🦀️.rs`. Also `en1995.material` and `en1995.6`.
9. **en1993 check families unbound, 12 of 21.** Same fix. File: `ART(en1993)/🎬️scene-members/🧰️kit/🦀️.rs`.
10. **en1996 check families unbound, 15 of 29.** Same fix. File: `ART(en1996)/🎬️scene/🧰️kit/🦀️.rs`.
11. **en1990 check families unbound, 8 of 17.** Same fix. File: `ART(en1990)/🎬️scene/🧰️kit/🦀️.rs`.
12. **en1995 diagrams, 8 of 13 collections.** Missing for tapers, cutouts, panels, diaphragms, trusses, bracings, decks, placements. File: `ART(en1995)/🎬️scene/📈️diagrams/🦀️.rs`. Also showcase rows for diaphragms, trusses, bracings, decks (`🖼️assets` or `📚️examples` DSL; see `showcase-docs` for the pattern).
13. **en1997 diagrams, 5 of 13 collections.** Missing for slopes (circle-slip), upliftCases (stabilising vs destabilising stack), anchors (layout), pileGroups (layout), layerWaters (head). File: `ART(en1997)/🎬️scene/📈️diagrams/🦀️.rs`.
14. **en1990 diagrams and grips.** Diagrams missing for bridgeSls, effects, fatigueSpectra, factorOverrides, trafficGroups. Grips missing for seismics (aEk, importance class), testSeries (mean, standard deviation). Files: `ART(en1990)/🎬️scene/📈️diagrams/🦀️.rs` and `ART(en1990)/🎬️scene/🕹️handles/🦀️.rs`.
15. **en1990 insert palette.** effects and placements are outliner-only. File: `ART(en1990)/🎬️scene/🧰️kit/🦀️.rs` (`ENTITY_KINDS`) and the `case`-style exclusion list if needed. Also trafficGroups missing from a showcase.
16. **en1992 diagrams and grade visibility.** Diagrams missing for anchors, placements, cases, and grades. concreteGrades, reinforcementGrades and prestressSteels have a module (`ART(en1992)/🎬️scene/🎨️grades/🦀️.rs`) but no `ENTITY_KINDS` entry, so they are not pickable or insertable. Add a grade legend diagram (fck, fyk) and colour members by grade. File: `ART(en1992)/🎬️scene/📈️diagrams/🦀️.rs` and `ART(en1992)/🎬️scene/🧰️kit/🦀️.rs`. Also a grip on `RcMember.sectionPoints` in `ART(en1992)/🎬️scene/🏛️members/🦀️.rs`.
17. **en1991 grips and showcase.** accidentalCases has no grip, nested impact and explosion have none. roofs.multiSpan and dynamicElements.massPerLength not grip-bound. Six collections missing from the showcase (helicopterLandings, tieRuns, constructionLoads, fallingLoads, executionImperfections, derailmentCases). Files: `ART(en1991)/🎬️scene-members/🕹️handles/🦀️.rs`, showcase DSL under `ART(en1991)/🖼️assets`.
18. **en1993 loadCases and nested action rows.** loadCases has no 3D, grip, insert or diagram. memberActions has no grip (its `action` row, six leaves). fatigueDetails, fireExposures, tensionComponents, towerLegs, craneRunways, frameBars have no grip per the literal check (verify). File: `ART(en1993)/🎬️scene-members/🧰️kit/🦀️.rs`, `ART(en1993)/🎬️scene-members/🕹️handles/🦀️.rs`.
19. **en1999 outlines and insert.** outlineVertices has no diagram. sections, materials, placements, outlineVertices are not insertable. Add a section outline diagram and an outline-vertex handle on sections (height, width, thickness). File: `ART(en1999)/🎬️scene/🧰️kit/🦀️.rs`, `ART(en1999)/🎬️scene/📈️diagrams/🦀️.rs`. transverseStiffeners missing from a showcase.
20. **en1998 assessment section geometry and grips.** assessments leaves `sectionWidthM`, `effectiveDepthM`, `barDiameterMm`, `tensionSteelAreaM2` are not named in grip-bearing scene files. Verify under `ART(en1998)/🎬️scene/🕹️handles/🦀️.rs`; if absent, add grips on the assessed section. accelerograms and placements have no grip (lower impact).

Lower-ranked, not in the top 20: en1996 chases and movementJoints diagrams (`ART(en1996)/🎬️scene/📈️diagrams/🦀️.rs`), en1994 placements diagram, en1999 materials grip (non-geometric), en1995 fasteners.positions and panels.layers grips (`ART(en1995)/🎬️scene/🧱️entities/🕹️grips/🦀️.rs`, verify against the `homes` list first).

## ✅ Not gaps

- **Edit rules**: every leaf of all ten artifacts has a rule (`leaves.json`, 0 uncovered). Every collection has an insert rule (except case by design) and a removal rule.
- **Live path**: all ten artifacts use `live_scene!` and `live_scene_of` in editor and viewer, with a live test. `SceneDelta` and scene memo are in the shared structural-scene crate.
- **Check-row to entity wiring**: `check_focus_effects` and `interaction_topology` exist in all ten editors. The gap is binding (gap 5 to 11), not wiring.
- **3D for measurable collections**: all top-level collections of en1990, en1991, en1994, en1995, en1996, en1997, en1998, en1999 are drawn. Only en1992 (grades, prestress) and en1993 (loadCases) lack 3D, and those are non-geometric tables.

## ⚠️ Caveats

- Static analysis only. The live-scene and realtime-QA waves are not compiled (`live-scene.md`, `realtime-qa.md`), and their golden files are 0 bytes. "Live path" means wired, not proven.
- Leaf-name grip matching is a proxy and produces both false negatives (grip code names leaves through helper arguments) and false positives (name appears in a grip file for another reason). The corrections in section "Corrections" show the false negatives are common.
- Check-family binding is literal (`visual-coverage.md`). A family can be bound through `report_owner` or a subject map not matched by literal id.
- Inspector exposure is not verified per leaf; it is assumed generic over the schema.
