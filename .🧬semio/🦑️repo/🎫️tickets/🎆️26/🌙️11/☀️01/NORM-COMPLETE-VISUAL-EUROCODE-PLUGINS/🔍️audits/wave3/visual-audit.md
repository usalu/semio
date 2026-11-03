# 🔍️ Visual Audit, Wave 3 (EN 1990 to EN 1999, re-measured)

Read-only audit by a Haiku auditor. No source edits, builds, cargo runs, repo runs or git writes. Scratch scripts and JSON are in `TICKET/🗑️generated/wave3-visual/` (`measure.py`, `measure.json`, `checkbind.py`, `checkbind.json`, `summary.md`, `tables.md`). `TICKET` = `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`.

`ART(code)` = `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 🧭️ Method (changes against wave 2)

- Collections: every top-level array in `ART(code)/🧬️schema/📸️snapshot/🔣️.json`, so tonight's additions (tieBeams, slopes, investigations, characteristicBases, frameActions, frameMemberBases, jointPlates, foundationBlocks, openings, studRows, craneHoists, footbridges, …) are included.
- (1) 3D: the collection is a `collection:` / `kind("…", "<collection>", …)` entry in `ENTITY_KINDS` of a `🧰️kit/🦀️.rs` file. "weak" means the key is read by scene code but not registered, so it is not pickable and not drawn as an entity.
- (2) grip: the collection key (or its kebab form) appears in a grip source (`🕹️handles`, `🏛️members`, `🧱️entities`, `🛡️safety`, `grips`). (2b) is a per-leaf proxy: a numeric leaf of the item type is "grip-proxy" when its camel or snake name appears in those grip sources. It is an upper bound (names can match for other reasons; geometric and non-geometric leaves are not separated).
- (3) diagram: the collection key or item type name appears in a diagram source (`📈️*` or `diagrams` folders under `🎬️scene` and `🎬️scene-members`). Wave 2 read only `📈️diagrams`, which hid the `entity-diagrams`, `design-diagrams` and `annex-diagrams` modules of en1993. Those are counted now.
- (4) palette: registered in `ENTITY_KINDS` (the palette is `insert_specs()` = `ENTITY_KINDS` minus `case` in en1997; verified in en1997 and en1999 viewport modules). EN+DE: the registry row has non-empty `label_en` and `label_de`, or the item type has both properties in the schema.
- (5) showcase: the collection key or kebab form appears in a `🗣️.dsl.semio` under `🖼️assets` or `📚️examples`.
- (6) check binding: `checkbind.py` (rerun), literal spec-trace ids against scene source. The family split differs from wave 2 (two-segment prefix), so counts are not directly comparable with `visual-coverage.md`.

Static evidence only. Nothing was compiled or run; "wired" is not "proven in the viewport". Verify each gap in the viewport before closing.

## 📊 Summary per code

Column meanings as in the method. "weak" 3D = not registered.

| Code | Colls | (1) 3D | (2) grip colls | (2b) numeric leaves grip-proxy | (3) diagram colls | (4) palette insert | EN+DE labels | (5) showcase | (6) check families unbound |
|---|---|---|---|---|---|---|---|---|---|
| ⚖️en1990 | 14 | 14/14 | 12/14 | 23/67 | 13/14 | 14/14 | 14/14 | 14/14 | 8/17 |
| 🏋️en1991 | 28 | 27/28 (+1 weak) | 28/28 | 83/115 | 19/28 | 27/28 | 27/28 | 23/28 | 0/10 |
| 🏛️en1992 | 7 | 6/7 (+1 weak) | 3/7 | 10/34 | 3/7 | 6/7 | 6/7 | 7/7 | 26/36 |
| 🔩️en1993 | 30 | 27/30 (+3 weak) | 27/30 | 124/413 | 28/30 | 27/30 | 27/30 | 30/30 | 12/23 |
| 🧩️en1994 | 4 | 3/4 (+1 weak) | 4/4 | 27/47 | 4/4 | 3/4 | 3/4 | 4/4 | 1/7 |
| 🪵️en1995 | 13 | 12/13 (+1 weak) | 12/13 | 37/118 | 11/13 | 12/13 | 12/13 | 13/13 | 9/17 |
| 🪨️en1996 | 5 | 4/5 (+1 weak) | 5/5 | 17/41 | 4/5 | 4/5 | 4/5 | 5/5 | 15/29 |
| 🌍️en1997 | 15 | 13/15 (+1 weak) | 14/15 | 46/124 | 15/15 | 13/15 | 13/15 | 15/15 | 43/54 |
| 🫨️en1998 | 17 | 14/17 (+3 weak) | 17/17 | 65/174 | 1/17 | 14/17 | 14/17 | 17/17 | 29/69 |
| 🪶️en1999 | 15 | 15/15 | 15/15 | 42/52 | 11/15 | 15/15 | 15/15 | 15/15 | 2/7 |

## 🏆 Biggest 25 remaining gaps (owner, measured number, exact file)

Paths are absolute. `ART(x)` expands as in the header.

1. **en1998 check binding**: 29 of 69 check families unbound (`.analysis`, `.combination`, `.modal`, `.joint`, …). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧰️kit/🦀️.rs` (map each check family to its subject entity; copy `report_owner` from en1991 `🎬️scene-members/🧰️kit/🦀️.rs`).
2. **en1997 check binding**: 43 of 54 families unbound (`anc-*`, `dew-*`, `gi-*`, `geo-f03`, …). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧰️kit/🦀️.rs`.
3. **en1992 check binding**: 26 of 36 families unbound (`en1992-4.*` anchors, `en1992-2.*`, `en1992.a-16`, `en1992.annexC`). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧰️kit/🦀️.rs`.
4. **en1998 unregistered collections**: `tieBeams`, `slopes`, `placements` are read by scene code but absent from `ENTITY_KINDS`, so no pickable 3D entity, no palette insert, no EN/DE registry labels. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧰️kit/🦀️.rs`.
5. **en1997 `characteristicBases` has no 3D, grip or palette entry**. It has a showcase row and a diagram by name. Files: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧰️kit/🦀️.rs`, `…/🎬️scene/🕹️handles/🦀️.rs`.
6. **en1998 diagrams 16 of 17 collections missing** (only `buildings` is drawn). Missing: `bridges`, `assessments`, `silos`, `tanks`, `foundations`, `retainingWalls`, `towers`, `soilLayers`, `diaphragms`, `accelerograms`, `piers`, `bearings`, `joints`, `placements`, `tieBeams`, `slopes`. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs`.
7. **en1998 numeric inputs not grip-bound**: `foundations` 5/30, `assessments` 7/25, `buildings` 6/21, `piers` 7/12, `bridges` 7/13, `silos` 3/8, `tanks` 4/8, `retainingWalls` 3/8, `soilLayers` 1/5. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs`.
8. **en1993 `joints` (SteelJoint) numeric grips 12 of 54**. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene-members/🕹️handles/🦀️.rs`.
9. **en1993 `platedPanels` 15 of 45, `jointPlates` 11 of 19, `hollowJoints` 5 of 17**. Same file as item 8.
10. **en1993 `coldFormedMembers` 5 of 26, `siloShells` 7 of 26, `towerLegs` 2 of 22, `tensionComponents` 1 of 20, `craneRunways` 4 of 22, `bridgeFatigue` 5 of 21, `piles` 1 of 12, `builtUpMembers` 3 of 17**. Same file as item 8.
11. **en1993 `sections` (weak 3D, not in palette, 6 of 17 numeric) and `memberActions` (weak 3D, not in palette)**. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene-members/🧰️kit/🦀️.rs`.
12. **en1993 check binding**: 12 of 23 families unbound (`en1993.1-3`, `1-4`, `1-7`, `1-11`, `1-12`, `3`, `3-1`, `3-2`, `4-1`, `4-2`, `4-3`, `sections`). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene-members/🧰️kit/🦀️.rs`.
13. **en1990 `members` numeric grips 5 of 30; `bridgeSls` 2 of 14; `fatigueSpectra` 0 of 3; `effects` 0 of 1**. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs`.
14. **en1990 check binding**: 8 of 17 families unbound (`en1990.6`, `e10`, `e11`, `e12`, `equ`, `members`, `sls`, `table-2`). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧰️kit/🦀️.rs`.
15. **en1996 `walls` numeric grips 7 of 31**; check binding 15 of 29 families unbound (`en1996.2`, `en1996.3`, `m-01`…`m-33`). Files: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs` and `…/🎬️scene/🧰️kit/🦀️.rs`.
16. **en1995 typed collections numeric grips**: `tapers` 0 of 7, `cutouts` 0 of 7, `decks` 2 of 16, `floors` 2 of 10, `diaphragms` 1 of 10, `trusses` 2 of 10, `bracings` 1 of 8, `members` 8 of 21. Files: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs` and `…/🎬️scene/🧱️entities/🕹️grips/🦀️.rs`. Add grips where a geometric input has a `Home`.
17. **en1995 check binding**: 9 of 17 families unbound (`en1995.1-2`, `2`, `6`, `material`, `t-18`, `t-31`, `t-32`, `t-33`, `t-46`). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧰️kit/🦀️.rs`.
18. **en1997 numeric grips**: `piles` 4 of 24, `layers` 2 of 12, `retainingWalls` 5 of 17, `labTests` 1 of 9, `anchors` 4 of 10, `embeddedWalls` 4 of 10, `pileGroups` 3 of 9. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs`.
19. **`placements` not registered (no 3D, no palette, no EN/DE registry labels)** in en1991, en1992 (weak 3D), en1993, en1994, en1995, en1996. Files: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene-members/🧰️kit/🦀️.rs`, `…/🔩️en1993/…/🎬️scene-members/🧰️kit/🦀️.rs`, `…/🧩️en1994/…/🎬️scene/🧰️kit/🦀️.rs`, `…/🪵️en1995/…/🎬️scene/🧰️kit/🦀️.rs`, `…/🪨️en1996/…/🎬️scene/🧰️kit/🦀️.rs`, `…/🏛️en1992/…/🎬️scene/🧰️kit/🦀️.rs` (all under `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/`).
20. **en1991 diagrams 9 of 28 collections missing**: `roofSurfaces`, `specialVehicles`, `craneHoists`, `footbridges`, `railDynamics`, `loadDispersals`, `bulkSolids`, `rotatingMachines`, `placements`. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs`.
21. **en1991 showcase missing for 5 new collections**: `footbridges`, `railDynamics`, `loadDispersals`, `bulkSolids`, `rotatingMachines`. File: showcase rows under `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/` (`🗣️.dsl.semio`).
22. **en1992 grades not bound to 3D pick, grip or diagram**: `concreteGrades` (no grip, no diagram), `reinforcementGrades` (1 of 4 numeric), `prestressSteels` (0 of 2), no grade legend. File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🎨️grades/🦀️.rs`. Also `RcMember.sectionPoints` has no grip (`…/🏛️en1992/…/🎬️scene/🏛️members/🦀️.rs`).
23. **en1992 `cases` and `grades` have no diagram** (3 of 7 collections drawn). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs`.
24. **en1995 diagrams missing for `cutouts` and `placements`** (11 of 13 drawn). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs`.
25. **en1999 diagrams missing for `placements`, `connections`, `fatigueCategories`, `ringStiffeners`** (11 of 15 drawn). File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs`.

Also open, below the top 25: en1993 diagrams missing for `frameMemberBases` and `placements`; en1996 diagram missing for `placements` (4 of 5); en1994 `placements` not in palette (3 of 4); en1997 `piles` and `layers` numeric grips; en1998 `bridges` and `assessments` have no diagram; en1990 `trafficGroups` has no grip and no numeric leaf (not gripped by design, to confirm).

## ⚠️ Discrepancies to verify

- en1990 `effects` and `placements`: wave 2 said outliner-only. The `ENTITY_KINDS` registry lists both, so `insert_specs()` should offer them. Check the en1990 viewport `insert_specs` filter before treating them as palette-missing.
- en1993 diagrams: wave 2 said 6 of 26; the module-level count is 28 of 30 (`entity-diagrams`, `design-diagrams`, `annex-diagrams` were not counted before). Diagram counts here are name-based, so a word hit inside a diagram can over-count.
- Check binding: wave 2 said en1991 7 of 7 bound; the rerun shows 0 of 10 unbound with the two-segment family split. en1997 and en1998 are now worse than wave 2 (43 and 29 unbound) because tonight's check families were added.

## 📋 Per-code tables

### ⚖️en1990

Check families unbound (spec-trace literal binding): 8 of 17. Unbound families: en1990.6, en1990.e10, en1990.e11, en1990.e12, en1990.equ, en1990.members, en1990.sls, en1990.table-2.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `permanents` | PermanentAction | ✓ | ✓ | 1/1 | ✓ | ✓ | ✓ | ✓ |
| `variables` | VariableAction | ✓ | ✓ | 1/1 | ✓ | ✓ | ✓ | ✓ |
| `accidentals` | AccidentalAction | ✓ | ✓ | 1/1 | ✓ | ✓ | ✓ | ✓ |
| `seismics` | SeismicAction | ✓ | ✓ | 1/1 | ✓ | ✓ | ✓ | ✓ |
| `members` | Member | ✓ | ✓ | 5/30 | ✓ | ✓ | ✓ | ✓ |
| `bridgeSls` | BridgeSls | ✓ | ✓ | 2/14 | ✓ | ✓ | ✓ | ✓ |
| `effects` | MemberEffect | ✓ | ✗ | 0/1 | ✓ | ✓ | ✓ | ✓ |
| `fatigueSpectra` | FatigueSpectrum | ✓ | ✓ | 0/3 | ✓ | ✓ | ✓ | ✓ |
| `reliabilityAnalyses` | ReliabilityAnalysis | ✓ | ✓ | 2/4 | ✓ | ✓ | ✓ | ✓ |
| `testSeries` | TestSeries | ✓ | ✓ | 5/6 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ✓ | ✓ | 4/4 | ✗ | ✓ | ✓ | ✓ |
| `cases` | CaseRow | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `factorOverrides` | FactorOverride | ✓ | ✓ | 1/1 | ✓ | ✓ | ✓ | ✓ |
| `trafficGroups` | TrafficGroup | ✓ | ✗ | 0/0 | ✓ | ✓ | ✓ | ✓ |

### 🏋️en1991

Check families unbound (spec-trace literal binding): 0 of 10. Unbound families: none.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `floors` | FloorArea | ✓ | ✓ | 2/4 | ✓ | ✓ | ✓ | ✓ |
| `selfWeightElements` | SelfWeightElement | ✓ | ✓ | 1/2 | ✓ | ✓ | ✓ | ✓ |
| `roofs` | RoofArea | ✓ | ✓ | 3/6 | ✓ | ✓ | ✓ | ✓ |
| `windFaces` | WindFace | ✓ | ✓ | 2/8 | ✓ | ✓ | ✓ | ✓ |
| `accidentalCases` | AccidentalCase | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `bridgeLanes` | BridgeLane | ✓ | ✓ | 3/3 | ✓ | ✓ | ✓ | ✓ |
| `constructionStages` | ConstructionStage | ✓ | ✓ | 2/3 | ✓ | ✓ | ✓ | ✓ |
| `roofSurfaces` | RoofSurface | ✓ | ✓ | 2/2 | ✗ | ✓ | ✓ | ✓ |
| `placements` | Placement | ~ (not registered) | ✓ | 4/4 | ✗ | ✗ | ✗ | ✓ |
| `specialVehicles` | SpecialVehicle | ✓ | ✓ | 2/2 | ✗ | ✓ | ✓ | ✓ |
| `windBridges` | WindBridge | ✓ | ✓ | 6/8 | ✓ | ✓ | ✓ | ✓ |
| `dynamicElements` | DynamicElement | ✓ | ✓ | 4/7 | ✓ | ✓ | ✓ | ✓ |
| `impactActions` | ImpactAction | ✓ | ✓ | 7/8 | ✓ | ✓ | ✓ | ✓ |
| `internalExplosions` | InternalExplosion | ✓ | ✓ | 2/4 | ✓ | ✓ | ✓ | ✓ |
| `thermalSections` | ThermalSection | ✓ | ✓ | 2/6 | ✓ | ✓ | ✓ | ✓ |
| `cases` | CaseRow | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `helicopterLandings` | HelicopterLanding | ✓ | ✓ | 5/5 | ✓ | ✓ | ✓ | ✓ |
| `tieRuns` | TieRun | ✓ | ✓ | 6/9 | ✓ | ✓ | ✓ | ✓ |
| `constructionLoads` | ConstructionLoad | ✓ | ✓ | 2/2 | ✓ | ✓ | ✓ | ✓ |
| `fallingLoads` | FallingLoad | ✓ | ✓ | 5/6 | ✓ | ✓ | ✓ | ✓ |
| `executionImperfections` | ExecutionImperfection | ✓ | ✓ | 5/6 | ✓ | ✓ | ✓ | ✓ |
| `derailmentCases` | DerailmentCase | ✓ | ✓ | 4/6 | ✓ | ✓ | ✓ | ✓ |
| `craneHoists` | CraneHoist | ✓ | ✓ | 2/2 | ✗ | ✓ | ✓ | ✓ |
| `footbridges` | Footbridge | ✓ | ✓ | 3/3 | ✗ | ✓ | ✓ | ✗ |
| `railDynamics` | RailDynamic | ✓ | ✓ | 1/1 | ✗ | ✓ | ✓ | ✗ |
| `loadDispersals` | LoadDispersal | ✓ | ✓ | 4/4 | ✗ | ✓ | ✓ | ✗ |
| `bulkSolids` | BulkSolid | ✓ | ✓ | 1/1 | ✗ | ✓ | ✓ | ✗ |
| `rotatingMachines` | RotatingMachine | ✓ | ✓ | 3/3 | ✗ | ✓ | ✓ | ✗ |

### 🏛️en1992

Check families unbound (spec-trace literal binding): 26 of 36. Unbound families: en1992-2.6, en1992-2.7, en1992-4.edge, en1992-4.interaction, en1992-4.na, en1992-4.pryout, en1992-4.pullout, en1992-4.splitting, en1992-4.steel-shear, en1992.3, en1992.a-16, en1992.annexC, en1992.b-01, en1992.b-02, en1992.b-09, en1992.integrity, en1992.m-02, en1992.m-03, en1992.m-08, en1992.m-09.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `concreteGrades` | ConcreteGrade | ✓ | ✗ | 0/1 | ✗ | ✓ | ✓ | ✓ |
| `reinforcementGrades` | ReinforcementGrade | ✓ | ✗ | 1/4 | ✗ | ✓ | ✓ | ✓ |
| `prestressSteels` | PrestressSteel | ✓ | ✗ | 0/2 | ✗ | ✓ | ✓ | ✓ |
| `members` | RcMember | ✓ | ✓ | 5/15 | ✓ | ✓ | ✓ | ✓ |
| `anchors` | Anchor | ✓ | ✓ | 1/8 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ~ (not registered) | ✓ | 3/4 | ✓ | ✗ | ✗ | ✓ |
| `cases` | CaseRow | ✓ | ✗ | 0/0 | ✗ | ✓ | ✓ | ✓ |

### 🔩️en1993

Check families unbound (spec-trace literal binding): 12 of 23. Unbound families: en1993.1-11, en1993.1-12, en1993.1-3, en1993.1-4, en1993.1-7, en1993.3, en1993.3-1, en1993.3-2, en1993.4-1, en1993.4-2, en1993.4-3, en1993.sections.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `materials` | SteelMaterial | ✓ | ✓ | 1/5 | ✓ | ✓ | ✓ | ✓ |
| `sections` | SteelSection | ~ (not registered) | ✓ | 6/17 | ✓ | ✗ | ✗ | ✓ |
| `members` | SteelMember | ✓ | ✓ | 5/15 | ✓ | ✓ | ✓ | ✓ |
| `loadCases` | LoadCase | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `memberActions` | MemberAction | ~ (not registered) | ✓ | 0/0 | ✓ | ✗ | ✗ | ✓ |
| `joints` | SteelJoint | ✓ | ✓ | 12/54 | ✓ | ✓ | ✓ | ✓ |
| `fatigueDetails` | FatigueDetail | ✓ | ✗ | 1/1 | ✓ | ✓ | ✓ | ✓ |
| `fireExposures` | FireExposure | ✓ | ✓ | 1/7 | ✓ | ✓ | ✓ | ✓ |
| `coldFormedMembers` | ColdFormedMember | ✓ | ✓ | 5/26 | ✓ | ✓ | ✓ | ✓ |
| `platedPanels` | PlatedPanel | ✓ | ✓ | 15/45 | ✓ | ✓ | ✓ | ✓ |
| `siloShells` | SiloShell | ✓ | ✓ | 7/26 | ✓ | ✓ | ✓ | ✓ |
| `tensionComponents` | TensionComponent | ✓ | ✓ | 1/20 | ✓ | ✓ | ✓ | ✓ |
| `bridgeFatigue` | BridgeFatigue | ✓ | ✓ | 5/21 | ✓ | ✓ | ✓ | ✓ |
| `towerLegs` | TowerLeg | ✓ | ✓ | 2/22 | ✓ | ✓ | ✓ | ✓ |
| `piles` | SteelPile | ✓ | ✓ | 1/12 | ✓ | ✓ | ✓ | ✓ |
| `craneRunways` | CraneRunway | ✓ | ✓ | 4/22 | ✓ | ✓ | ✓ | ✓ |
| `storeys` | Storey | ✓ | ✓ | 3/10 | ✓ | ✓ | ✓ | ✓ |
| `restraints` | Restraint | ✓ | ✓ | 5/9 | ✓ | ✓ | ✓ | ✓ |
| `builtUpMembers` | BuiltUpMember | ✓ | ✓ | 3/17 | ✓ | ✓ | ✓ | ✓ |
| `weldGroups` | WeldGroup | ✓ | ✓ | 4/7 | ✓ | ✓ | ✓ | ✓ |
| `parametricSections` | ParametricSection | ✓ | ✓ | 5/5 | ✓ | ✓ | ✓ | ✓ |
| `hollowJoints` | HollowJoint | ✓ | ✓ | 5/17 | ✓ | ✓ | ✓ | ✓ |
| `jointRows` | JointRow | ✓ | ✓ | 5/12 | ✓ | ✓ | ✓ | ✓ |
| `frameNodes` | FrameNode | ✓ | ✓ | 2/4 | ✓ | ✓ | ✓ | ✓ |
| `frameBars` | FrameBar | ✓ | ✗ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ~ (not registered) | ✓ | 4/4 | ✗ | ✗ | ✗ | ✓ |
| `jointPlates` | JointPlate | ✓ | ✓ | 11/19 | ✓ | ✓ | ✓ | ✓ |
| `foundationBlocks` | FoundationBlock | ✓ | ✓ | 6/11 | ✓ | ✓ | ✓ | ✓ |
| `frameActions` | FrameAction | ✓ | ✓ | 5/5 | ✓ | ✓ | ✓ | ✓ |
| `frameMemberBases` | FrameMemberBasis | ✓ | ✗ | 0/0 | ✗ | ✓ | ✓ | ✓ |

### 🧩️en1994

Check families unbound (spec-trace literal binding): 1 of 7. Unbound families: en1994.3.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `beams` | CompositeBeam | ✓ | ✓ | 11/21 | ✓ | ✓ | ✓ | ✓ |
| `columns` | CompositeColumn | ✓ | ✓ | 10/16 | ✓ | ✓ | ✓ | ✓ |
| `slabs` | CompositeSlab | ✓ | ✓ | 2/6 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ~ (not registered) | ✓ | 4/4 | ✓ | ✗ | ✗ | ✓ |

### 🪵️en1995

Check families unbound (spec-trace literal binding): 9 of 17. Unbound families: en1995.1-2, en1995.2, en1995.6, en1995.material, en1995.t-18, en1995.t-31, en1995.t-32, en1995.t-33, en1995.t-46.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `members` | TimberMember | ✓ | ✓ | 8/21 | ✓ | ✓ | ✓ | ✓ |
| `connections` | TimberConnection | ✓ | ✓ | 10/12 | ✓ | ✓ | ✓ | ✓ |
| `cases` | CaseRow | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ~ (not registered) | ✓ | 4/4 | ✗ | ✗ | ✗ | ✓ |
| `tapers` | Taper | ✓ | ✓ | 0/7 | ✓ | ✓ | ✓ | ✓ |
| `cutouts` | Cutout | ✓ | ✓ | 0/7 | ✗ | ✓ | ✓ | ✓ |
| `fasteners` | Fastener | ✓ | ✓ | 4/9 | ✓ | ✓ | ✓ | ✓ |
| `panels` | Panel | ✓ | ✓ | 3/4 | ✓ | ✓ | ✓ | ✓ |
| `floors` | Floor | ✓ | ✓ | 2/10 | ✓ | ✓ | ✓ | ✓ |
| `diaphragms` | Diaphragm | ✓ | ✓ | 1/10 | ✓ | ✓ | ✓ | ✓ |
| `trusses` | Truss | ✓ | ✓ | 2/10 | ✓ | ✓ | ✓ | ✓ |
| `bracings` | Bracing | ✓ | ✓ | 1/8 | ✓ | ✓ | ✓ | ✓ |
| `decks` | Deck | ✓ | ✗ | 2/16 | ✓ | ✓ | ✓ | ✓ |

### 🪨️en1996

Check families unbound (spec-trace literal binding): 15 of 29. Unbound families: en1996.2, en1996.3, m-01., m-02., m-03., m-04., m-05., m-06., m-07., m-16., m-20., m-33., m-40., m-82., m-83..

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `walls` | MasonryWall | ✓ | ✓ | 7/31 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ~ (not registered) | ✓ | 4/4 | ✗ | ✗ | ✗ | ✓ |
| `cases` | CaseRow | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `chases` | WallChase | ✓ | ✓ | 5/5 | ✓ | ✓ | ✓ | ✓ |
| `movementJoints` | MovementJoint | ✓ | ✓ | 1/1 | ✓ | ✓ | ✓ | ✓ |

### 🌍️en1997

Check families unbound (spec-trace literal binding): 43 of 54. Unbound families: en1997.anc-01, en1997.anc-02, en1997.anc-03, en1997.anc-04, en1997.dew-01, en1997.dew-02, en1997.dew-03, en1997.g-02, en1997.geo-f03, en1997.gi-01, en1997.gi-02, en1997.gi-03, en1997.gi-04, en1997.gi-05, en1997.gi-06, en1997.gi-07, en1997.gi-08, en1997.gi-09, en1997.gi-10, en1997.gi-11.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `layers` | SoilLayer | ✓ | ✓ | 2/12 | ✓ | ✓ | ✓ | ✓ |
| `footings` | SpreadFoundation | ✓ | ✓ | 3/5 | ✓ | ✓ | ✓ | ✓ |
| `piles` | Pile | ✓ | ✓ | 4/24 | ✓ | ✓ | ✓ | ✓ |
| `retainingWalls` | RetainingWall | ✓ | ✓ | 5/17 | ✓ | ✓ | ✓ | ✓ |
| `slopes` | Slope | ✓ | ✓ | 7/7 | ✓ | ✓ | ✓ | ✓ |
| `upliftCases` | UpliftCase | ✓ | ✓ | 4/5 | ✓ | ✓ | ✓ | ✓ |
| `anchors` | Anchor | ✓ | ✓ | 4/10 | ✓ | ✓ | ✓ | ✓ |
| `embeddedWalls` | EmbeddedWall | ✓ | ✓ | 4/10 | ✓ | ✓ | ✓ | ✓ |
| `pileGroups` | PileGroup | ✓ | ✓ | 3/9 | ✓ | ✓ | ✓ | ✓ |
| `labTests` | LabTest | ✓ | ✓ | 1/9 | ✓ | ✓ | ✓ | ✓ |
| `layerWaters` | LayerWater | ✓ | ✓ | 3/8 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ~ (not registered) | ✓ | 3/4 | ✓ | ✗ | ✗ | ✓ |
| `cases` | CaseRow | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `investigations` | Investigation | ✓ | ✓ | 3/4 | ✓ | ✓ | ✓ | ✓ |
| `characteristicBases` | CharacteristicBasis | ✗ | ✗ | 0/0 | ✓ | ✗ | ✗ | ✓ |

### 🫨️en1998

Check families unbound (spec-trace literal binding): 29 of 69. Unbound families: .analysis, .analysisMethod, .combination, .componentCombination, .cyclicShear, .elevationRegularity, .groundClass, .joint, .lfm, .limitState, .masonryWall, .modal, .multipleSystems, .periodRatio, .planDims, .planRegularity, .profile, .qDetail, .qLimit, .radiationDamping.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `buildings` | En1998Building | ✓ | ✓ | 6/21 | ✓ | ✓ | ✓ | ✓ |
| `bridges` | En1998Bridge | ✓ | ✓ | 7/13 | ✗ | ✓ | ✓ | ✓ |
| `assessments` | En1998Assessment | ✓ | ✓ | 7/25 | ✗ | ✓ | ✓ | ✓ |
| `silos` | En1998Silo | ✓ | ✓ | 3/8 | ✗ | ✓ | ✓ | ✓ |
| `tanks` | En1998Tank | ✓ | ✓ | 4/8 | ✗ | ✓ | ✓ | ✓ |
| `foundations` | En1998Foundation | ✓ | ✓ | 5/30 | ✗ | ✓ | ✓ | ✓ |
| `retainingWalls` | En1998RetainingWall | ✓ | ✓ | 3/8 | ✗ | ✓ | ✓ | ✓ |
| `towers` | En1998Tower | ✓ | ✓ | 3/6 | ✗ | ✓ | ✓ | ✓ |
| `soilLayers` | En1998SoilLayer | ✓ | ✓ | 1/5 | ✗ | ✓ | ✓ | ✓ |
| `diaphragms` | En1998Diaphragm | ✓ | ✓ | 1/8 | ✗ | ✓ | ✓ | ✓ |
| `accelerograms` | En1998Accelerogram | ✓ | ✓ | 2/2 | ✗ | ✓ | ✓ | ✓ |
| `piers` | En1998Pier | ✓ | ✓ | 7/12 | ✗ | ✓ | ✓ | ✓ |
| `bearings` | En1998Bearing | ✓ | ✓ | 2/5 | ✗ | ✓ | ✓ | ✓ |
| `joints` | En1998Joint | ✓ | ✓ | 2/4 | ✗ | ✓ | ✓ | ✓ |
| `placements` | En1998Placement | ~ (not registered) | ✓ | 4/4 | ✗ | ✗ | ✗ | ✓ |
| `tieBeams` | En1998TieBeam | ~ (not registered) | ✓ | 4/7 | ✗ | ✗ | ✗ | ✓ |
| `slopes` | En1998Slope | ~ (not registered) | ✓ | 4/8 | ✗ | ✗ | ✗ | ✓ |

### 🪶️en1999

Check families unbound (spec-trace literal binding): 2 of 7. Unbound families: en1999.1-4, en1999.8.

| Collection | Item | (1) 3D | (2) grip | numeric leaves grip-proxy | (3) diagram | (4) palette | EN+DE | (5) showcase |
|---|---|---|---|---|---|---|---|---|
| `materials` | AluminiumMaterial | ✓ | ✓ | 0/0 | ✓ | ✓ | ✓ | ✓ |
| `sections` | AluminiumSection | ✓ | ✓ | 5/5 | ✓ | ✓ | ✓ | ✓ |
| `members` | AluminiumMember | ✓ | ✓ | 5/8 | ✓ | ✓ | ✓ | ✓ |
| `connections` | AluminiumConnection | ✓ | ✓ | 0/0 | ✗ | ✓ | ✓ | ✓ |
| `fireScenarios` | FireScenario | ✓ | ✓ | 2/2 | ✓ | ✓ | ✓ | ✓ |
| `fatigueDetails` | FatigueDetail | ✓ | ✓ | 2/5 | ✓ | ✓ | ✓ | ✓ |
| `coldFormed` | ColdFormedSheet | ✓ | ✓ | 3/3 | ✓ | ✓ | ✓ | ✓ |
| `shells` | AluminiumShell | ✓ | ✓ | 3/3 | ✓ | ✓ | ✓ | ✓ |
| `placements` | Placement | ✓ | ✓ | 4/4 | ✗ | ✓ | ✓ | ✓ |
| `weldRuns` | WeldRun | ✓ | ✓ | 5/5 | ✓ | ✓ | ✓ | ✓ |
| `outlineVertices` | SectionVertex | ✓ | ✓ | 2/2 | ✓ | ✓ | ✓ | ✓ |
| `fatigueCategories` | FatigueCategory | ✓ | ✓ | 1/3 | ✗ | ✓ | ✓ | ✓ |
| `patchLoads` | PatchLoad | ✓ | ✓ | 4/6 | ✓ | ✓ | ✓ | ✓ |
| `ringStiffeners` | RingStiffener | ✓ | ✓ | 3/3 | ✗ | ✓ | ✓ | ✓ |
| `transverseStiffeners` | TransverseStiffener | ✓ | ✓ | 3/3 | ✓ | ✓ | ✓ | ✓ |
