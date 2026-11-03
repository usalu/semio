# Live units audit, wave 2 (en1990 to en1999)

Static reading only; nothing compiled or run. `- ready for rebuild: live units en19xx` is appended to every `🗑️generated/build-en19xx/latest.md`.

## Shared change (compliance crate, minimal)
- `⚡️incremental-evaluation/🔍️read-tracker`: new `with_every_collection(documents)` returns the documents plus one more: the document with the most rows, with every empty top-level collection filled with the first row any other fixture has.
- `incremental_laws!` (`🧪️laws`) now runs all four laws (fold == full, `audit_reads`, unique ids, retained session after 40 single-leaf edits) over `with_every_collection($documents)`. Reason: `audit_reads` perturbs leaves, and an empty collection has no leaves, so a collection that no fixture populates was never audited or edited. This covers every collection (cases, placements, factorOverrides, soilLayers, lanes, weldRuns, ...) in all nine artifacts without touching their fixtures. All nine editor `🧮️incremental/🧪️tests` already use `incremental_laws!`, so no per-artifact test edit was needed.

## Cross-cutting finding fixed in 5 artifacts: placements dirtied every row
Placements are only read by duplicate-id integrity (global unit) and, in EN 1997, by the `sls-pairs` part (reads `*`). Because they were part of the shared reads, every placement drag re-ran every row unit. Now excluded from the shared reads in en1990, en1991, en1994, en1997, en1998 (en1992, en1995, en1996, en1999 use explicit shared lists and never had them). EN 1999 reads placements only in the `global:document` unit (placement validity), correct.

## Per artifact

### en1990
- Families to unit: `en1990.integrity.*`, `input.*`, `annex-b.*`, `a2.working-life`, `override.*`, `a2.traffic-groups`, `annex-c.beta` -> `global:document`; `str/geo/equ/sls/fat/a2.*/a14.*/e10-e12/fatigue.miner/table-2.1/6.x` -> `members:<id>`; `annex-c.form/alpha/partial-factors` -> `reliabilityAnalyses`; `annex-d.*` -> `testSeries`; catalogue design cases -> `cases`.
- Reads: member rows read the shared scalars plus effects, bridgeSls, fatigueSpectra, factorOverrides, permanents, variables, accidentals, seismics (all shared), `members.<i>` and `members.*.id`; analysis, test and case rows read shared scalars (annex, reliabilityClass, referencePeriod) plus own row.
- Fix: `placements` removed from the shared reads. No missing reads found (factorOverrides is evaluated in the global unit by index and is shared by members).

### en1991
- 16 row collections plus 10 parts map 1:1 to the scoped evaluator (floors, selfWeightElements, roofs, windFaces, accidentalCases, bridgeLanes, constructionStages, roofSurfaces, specialVehicles, windBridges, dynamicElements, impactActions, internalExplosions, thermalSections, cases; parts integrity, floors-none, self-weight-none, fire, snow, wind, thermal, construction, accidental-none, traffic).
- MISMATCH (stale verdict): `roofSurfaces` rows run `check_roof_drift`, which reads `roof_surfaces[index + 1..]` (next surface of the same face, last-of-face parapet drift). The unit declared only `windFaces`, `roofs`. Fixed: extra `roofSurfaces`.
- Fix: `placements` added to the excluded collections (ROW_COLLECTIONS 15 -> 16).
- Verified: windFaces rows read `roofs` (declared), roofSurfaces read windFaces and roofs (declared); wave3 rows read scalars only; traffic/crane/silo/fire/thermal parts read `*`.

### en1992
- Units: `global:document` (duplicate ids, NA edition, material cases of the grades), `members`, `anchors`, `cases`. Member and anchor checks read only annex, title, designWorkingLifeYears, deltaCDev, cementType, concrete/reinforcement grades, prestressSteels (all in the 8 shared keys); no read of anchors/members/cases/placements from a row. No mismatch, no change.

### en1994
- Units: `global:document` (empty verdict), beams, columns, slabs, `part:fire-rating`, fireBeams, fireSlabs, fireColumns, `part:bridge-kind`, bridgeBeams. Reads are scalars (annex, structureKind, steelFyPa, fireRating, insulationThicknessM, fatigueDetail) plus the own row; no cross-collection reads. Fix: `placements` excluded from shared reads.
- BLOCKER for the owner (not mine, compile error): `En1994Snapshot` in `🧬️schema/📸️snapshot/🦀️.rs` now declares `pub placements: Vec<Placement>` twice (lines 42 and 52; two agents added it at 00:12). One must be removed.

### en1995
- Units: `global:document` (integrity, empty verdict, caseNumbers/caseTexts duplicates), members, connections, cases, tapers, cutouts, fasteners, panels, floors (entity rows declare `members`, `connections`). Member and connection rows read only annex and ulsEquation. Entity rows read members (taper, cutout, floor, fastener), connections (fastener) and annex/ulsEquation: all declared. No mismatch, no change.

### en1996
- Units: `global:document`, walls, cases, `part:detailing` (reads `*`).
- MISMATCH: the cases rows call `evaluate_design_cases`, whose duplicate-id check runs on the first row of a repeated id by scanning all cases (`position(|o| o.id == case.id)`). A row unit read only `cases.<i>`, so renaming another row's id left a stale/missing duplicate verdict. Fixed: extra `cases.*.id`.
- Walls read only the four shared scalars plus the own wall. Placements are not in the shared list.

### en1997
- Units: integrity, layers, footing-na, footings, sls-pairs, pile-na, piles, wall-na, retainingWalls, slope-na, slopes (T2), upl-na, upliftCases, embeddedWalls, anchors, pileGroups, labTests, layerWaters, cases; `summarise` = governing summary.
- Verified declared extras are complete: footings/embeddedWalls/labTests/layerWaters read layers; piles/walls/slopes/uplift read layers and footings (design situations from footing load cases); anchors read embeddedWalls; pileGroups read piles; the wall-section catalogue reads layers and groundwater.
- Fix: `placements` excluded (ROW_COLLECTIONS 12 -> 13); the only reader is the `sls-pairs` part, which reads `*`.

### en1998
- Units: integrity, buildings-na, buildings (T2), bridges-na, bridges, assessments-na, assessments, silos-na, silos, tanks-na, tanks, foundations-na, foundations, walls-na, retainingWalls, towers-na, towers.
- MISMATCH (stale verdict): the building at index i is checked against `buildings.get(i + 1)` (seismic joint gap, `check_building_details(.., next, ..)`), but the unit read only its own row. Editing the next building left the joint verdict stale. Fixed: extra `buildings.<i+1>` (only the neighbour, not the whole collection).
- Other extras verified: buildings read diaphragms and accelerograms; assessments/foundations/retainingWalls read buildings; foundations read soilLayers.
- New collections: `piers`, `bearings` (just added by the owner) are only duplicate-id checked in the integrity unit today; already in the excluded collections. If a bridge check starts to read them, add `piers`/`bearings` to the `bridges` row extras. soilLayers, diaphragms, accelerograms have no own row units (no row-level checks), covered by integrity and the extras above.
- Fix: `placements` excluded (ROW_COLLECTIONS 13 -> 14).

### en1999
- Units: integrity, action-field rows (members, connections, coldFormed, shells), per collection `-empty` part plus rows: members, connections, fireScenarios, fatigueDetails, fatigueCategories, patchLoads, ringStiffeners, coldFormed, shells, weldRuns, sectionOutlines.
- MISMATCH: `ringStiffeners` rows read the other rings of their shell (`doc.ring_stiffeners.filter(shell_id)` for bays) but declared `shells`, `materials`. Fixed: extras now `shells`, `ringStiffeners` (materials is not read there; removing it avoids re-running rings on material edits).
- Verified: members read sections, materials, weldRuns, outlineVertices; connections read members, materials; fire rows read members, sections, materials; fatigueDetails read members, fatigueDetails (spectrum groups), fatigueCategories; fatigueCategories read members, fatigueDetails; patchLoads read members, sections, materials, coldFormed; weldRuns read members, sections; sectionOutlines read outlineVertices.

## Edited files
- `✏️s/🔌️plugins/📕️norm/⚖️compliance/⚡️incremental-evaluation/🔍️read-tracker/🦀️.rs`, `…/🧪️laws/🦀️.rs`
- `…/🗿️artifacts/{⚖️en1990,🏋️en1991,🧩️en1994,🪨️en1996,🌍️en1997,🫨️en1998,🪶️en1999}/…/🧮️units/🦀️.rs` (en1997 and en1999 under `💡️inferences/⚖️checks/`, en1996 under `⚖️masonry/`)

## Risks
- Unverified by the compiler. The new law inputs can make `audit_reads` slower (one extra document per artifact); the document is small (one row per collection beyond the richest fixture). If `from_value` rejects the merged document it is skipped silently.
- Part units (`Rows::part`) still read `*`; they are cheap (T0) but run on every edit. A finer `part` with declared reads would need a shared API addition.
