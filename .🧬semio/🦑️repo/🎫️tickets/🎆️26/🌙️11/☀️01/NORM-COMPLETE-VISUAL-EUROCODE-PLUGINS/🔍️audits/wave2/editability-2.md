# 🔍️ Editability audit 2: EN 1990–1999 artifacts (wave 2, re-run)

Read-only re-run of `editability.md` (afternoon). Ticket: `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`. Source as found on 2026-11-02 at 22:00. Raw outputs: `🗑️generated/editability-2/`.

## Key findings

- The earlier report is stale. The generic fallbacks it documented (en1993 `update(collection, row)`, en1994 `replace_row`, en1999 `update(collection, list)`) no longer exist in source. Every `resolve`/`resolve_edit` now goes through `EDIT_RULES` (shared `NormEditRules::resolve` in `📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs`), which errors with `no kind sets '…'` for any leaf without a `SetFieldRule`. Fallbacks in the current source: 0 across all ten artifacts.
- The earlier leaf counts were under-counted. The Rust-struct parser missed external row types (`CaseRow` from `semio_s_plugin_norm_compliance`) and collapsed some nested structs. This re-run uses the snapshot JSON Schema facet (`🧬️schema/📸️snapshot/🔣️.json`), which is complete (it resolves `anyOf` nullable structs and `$defs`).
- Most "non-editable" leaves are `id` fields (identity, not meant to be edited). The content gaps are concentrated in en1991 (39 leaves), plus 1 each in en1992 and en1999.
- en1999 regressed in practice. Its whole-collection fallback is gone, and 12 collections (`coldFormed`, `connections`, `fatigueDetails`, `fireScenarios`, `materials`, `sections`, `shells` and nested `actions`/`elements`) have no insert or remove kind. Their rows cannot be added or removed; their leaves are editable only where a `SetFieldRule` exists.
- Naming is inconsistent (`add-placement` vs `insert-placement`, `set-case-*` vs `change-*`, index vs id selectors). See the last section.

## Method

- **Leaves**: `leaves-schema.py` walks `📸️snapshot/🔣️.json`, recursing into `$ref`/`$defs` and nullable `anyOf` objects; arrays of objects use `[]`. Scalar/enum/array-of-scalar leaves count once. `id` leaves are kept in totals but reported separately.
- **Typed set-field kind**: a leaf whose template equals a `SetFieldRule.path` (`rules-kinds.py`, line-based extraction, cross-checked against `grep -o 'SetFieldRule {'` counts). No dead rules (every set template matches a leaf).
- **Collections**: rows that appear as `[]` paths in the schema. Insert/remove kinds from `InsertItemRule`, `RemoveItemRule` and `IdRemoval`.
- **3D handles**: write templates traced per artifact from `🎬️scene/` (`🕹️handles/` and the diagram/anchor binding sites), output in `handle-trace.json` (one subagent per artifact group, cross-checked: en1993 matched exactly). Diagram-only markers that are not drag grips are excluded from the counts. Ids and view state are excluded.
- **Not editable** = leaf not in a `SetFieldRule` and not a handle write template.

## Per-artifact table

Delta columns compare with `editability.md` (earlier). "Set" = leaves with a typed set-field kind. "Handle" = leaves with a 3D handle write. Earlier "Props" included fallback-only leaves, so it is not like-for-like.

| Artifact | Total leaves (earlier) | Set-field leaves (earlier) | Insert / remove kinds | Collections without insert kind | 3D handle leaves (earlier) | Handle-only (not set) | Non-editable content leaves (earlier) | Non-editable `id` leaves | Fallbacks (earlier) |
|---|---|---|---|---|---|---|---|---|---|
| ⚖️en1990 | 101 (55) | 88 (11) | 12 / 12 | 2 (`cases[].numbers`, `cases[].texts`, edited via `set-case-*`) | 16 (6) | 0 | 0 (38) | 13 | 0 (0) |
| 🏋️en1991 | 175 (102) | 104 (77) | 9 / 9 | 8 | 53 (7) | 17 | 39 (25) | 15 | 0 (0) |
| 🏛️en1992 | 125 (101) | 111 (23) | 13 / 13 (2 by id) | 0 | 21 (14) | 0 | 1 (74) | 13 | 0 (0) |
| 🔩️en1993 | 495 (354) | 461 (1) | 34 / 34 | 0 | 55 (18) | 0 | 0 (0) | 34 | 0 (353) |
| 🧩️en1994 | 109 (88) | 102 (19) | 4 / 4 | 3 (`beams/columns/slabs[].actions`) | 26 (16) | 0 | 0 (0) | 7 | 0 (69) |
| 🪵️en1995 | 75 (62) | 67 (57) | 6 / 6 | 2 (`cases[].numbers/texts`) | 21 (18) | 0 | 0 (5) | 8 | 0 (0) |
| 🪨️en1996 | 115 (54) | 100 (50) | 13 / 13 | 2 (`cases[].numbers/texts`) | 35 (12) | 0 | 0 (4) | 15 | 0 (0) |
| 🌍️en1997 | 146 (129) | 127 (14) | 17 / 17 (2 by id) | 2 (`cases[].numbers/texts`) | 30 (16) | 0 | 0 (106) | 19 | 0 (0) |
| 🫨️en1998 | 189 (113) | 173 (93) | 13 / 13 | 3 (`buildings[].members/storeys/systems`) | 25 (6) | 0 | 0 (20) | 16 | 0 (0) |
| 🪶️en1999 | 153 (130) | 134 (130, 122 via fallback) | 4 / 4 (id-based) | 12 | 39 (30) | 2 | 1 (0) | 18 | 0 (122) |

Notes per artifact:

- **en1990**: `betaComputed` appears among handle templates via a diagram marker described as draggable (`📈️diagrams/🦀️.rs:229`). Treat as uncertain; dropping it gives 15 handle leaves. Case numbers and texts have set kinds by `caseId`+`key` but no insert or remove for individual numbers/texts.
- **en1991**: the 17 handle-only leaves are building and wind dimensions (`envelope` write sites) that have no `SetFieldRule`. Non-editable (39) include `floors[].category`, `roofs[].roofType`, `windFaces[].zone`, `dynamicElements`, `impactActions`, `internalExplosions`, `specialVehicles`, `thermalSections`, `windBridges` leaves, and `accidentalCases` nested `impact`/`explosion` fields. Seven diagram read-only annotations (`windBridges`, `dynamicElements`, `impactActions`, `internalExplosions`, `constructionStages`, fire duration) are not counted as editable.
- **en1992**: `members[].tightness` is the only content leaf without a set kind.
- **en1993**: every content leaf has a typed set kind (461 set rules). A `choose_section` special case handles numeric writes to `members[].sectionId` (catalogue height pick, inserts the section if missing). All 55 handle leaves also have typed set kinds.
- **en1994**: `resolve_edit` runs a `conform()` coercion on every set (not a fallback). Action rows (`beams/columns/slabs[].actions`) have no insert/remove.
- **en1999**: `members[].bucklingLengthT` has no set kind and no handle. `connections[].bolts.boltsPerRow` is now handle-editable. The 12 collections without insert/remove are a regression against the earlier report.

## Still non-editable content leaves (full list)

Identity `id` leaves are excluded here (listed per artifact in `🗑️generated/editability-2/coverage-2.json`, key `nonEditable`).

- ⚖️en1990: none.
- 🏋️en1991 (39): `accidentalCases[].explosion[].assumedPressure`, `accidentalCases[].explosion[].explosionMass`, `accidentalCases[].explosion[].standoff`, `accidentalCases[].impact[].vehicleMass`, `accidentalCases[].impact[].vehicleSpeed`, `dynamicElements[].assumedCscd`, `dynamicElements[].firstFrequency`, `dynamicElements[].massPerLength`, `dynamicElements[].strouhal`, `floors[].assumedPartitions`, `floors[].assumedQkConcentrated`, `floors[].category`, `impactActions[].kind`, `impactActions[].mass`, `impactActions[].speed`, `impactActions[].stiffness`, `internalExplosions[].assumedPressure`, `internalExplosions[].ventPressure`, `roofs[].cE`, `roofs[].cT`, `roofs[].hasParapet`, `roofs[].multiSpan`, `roofs[].roofType`, `selfWeightElements[].material`, `specialVehicles[].designation`, `thermalSections[].alphaMicro`, `thermalSections[].assumedRange`, `thermalSections[].deckType`, `thermalSections[].surfacing`, `windBridges[].assumedForce`, `windBridges[].parapetHeight`, `windBridges[].parapetKind`, `windBridges[].trafficKind`, `windFaces[].cD`, `windFaces[].cPe1`, `windFaces[].cPe10`, `windFaces[].cPi`, `windFaces[].cS`, `windFaces[].zone`.
- 🏛️en1992 (1): `members[].tightness`.
- 🔩️en1993: none.
- 🧩️en1994: none.
- 🪵️en1995: none.
- 🪨️en1996: none.
- 🌍️en1997: none.
- 🫨️en1998: none.
- 🪶️en1999 (1): `members[].bucklingLengthT`.

## Fallback list (current source)

No generic fallback is reachable from the editor in any artifact. Remaining generic or whole-record shapes:

1. `🫨️en1998/.../🧬️schema/🧬️mutations/🦀️.rs:632` `replace_all(base, target, remove, insert)`: whole-collection replace helper, dead code (no callers). Delete.
2. `🫨️en1998/.../🧬️schema/🧬️mutations/🌚️update-site`: `UpdateSite { site }` rewrites the whole site record. Not reachable through `EDIT_RULES` (site leaves use `change-site-*` kinds), so it is a whole-record kind, not an edit path. Delete or wire to leaf kinds.
3. `🔩️en1993/.../🧭️edit-rules/🦀️.rs:544` `resolve`: special case for `members[].sectionId` numeric values (catalogue pick, may insert a section). Not generic, but a resolve-side branch to document.
4. `🧩️en1994/.../🧭️edit-rules/🦀️.rs:140` `resolve_edit`: `conform()` coerces every set value before `EDIT_RULES`. Not a fallback; listed for completeness.
5. ID-based removals (`IdRemoval`): en1992 `members`, `anchors`; en1999 `members`, `weldRuns`, `outlineVertices`, `placements`. Not generic, but a second removal mechanism beside the index-based `RemoveItemRule`.

## Naming inconsistencies and proposed convention

Observed:

| Concept | Current names | Issue |
|---|---|---|
| Insert verb | `insert-placement` (en1990–1998), `add-placement` (en1999) | Two verbs for one operation |
| Insert noun | `insert-accidental` (en1990), `insert-accidental-cases` (en1991); `insert-layer` (en1997) vs `insert-soil-layer` (en1998); `insert-floors`, `insert-roofs`, `insert-bridge-lanes` (en1991, plural) | Singular vs plural; same concept, different nouns |
| Set verb | `change-*` everywhere; `set-case-number`, `set-case-text` (en1990, en1995, en1996, en1997) | Two set verbs |
| Case fields | `change-case-label-en`, `change-case-entry-id` vs `set-case-number`, `set-case-text` | Case fields split by verb |
| Member label | `change-member-label-en/de` (en1990) vs `change-member-label` (en1993) | Language suffix only in one artifact |
| Selector for rows | id (`[id=…]`) in en1990, en1992, en1993, en1995, en1997, en1999; **index** in en1991 (23 set rules), en1994 (92), en1996 (87), en1998 (161) | Positional selection breaks on reorder or removal |
| Nested row parent selector | `insert-bar-layer` by `memberId` (id); `insert-load-case` by `wallIndex` (index, en1996); `insert-load-case` by `footingId` (id, en1997) | Same concept, two selector kinds |
| Removal | index-based `RemoveItemRule` (most) vs id-based `IdRemoval` (en1992, en1999) | Two removal semantics |
| Placement set kinds | `change-placement-x/y/z/yaw` in all ten, except that en1999 uses `add-` for insert | Consistent apart from the insert verb |

Proposed single convention:

- Kind = `<verb>-<entity>[-<field>]` with verb in {`change`, `insert`, `remove`} only. Drop `add-`, `set-`.
- Entity noun singular and kebab-case, same across artifacts: `insert-placement`, `insert-case`, `insert-load-case`, `insert-soil-layer`, `insert-accidental-case`, `insert-bridge-lane`.
- Field kinds: `change-<entity>-<field>`; case values `change-case-number`, `change-case-text`.
- Selectors: rows always by stable id (`[id=…]`), never by index. Index only as the insert position (`index` payload field).
- Removal: by id only (`remove-<entity>` with `id`); drop index-based `RemoveItemRule` and the `IdRemoval` split.
- Every collection with editable rows gets an insert and a remove kind, including nested action/element rows (en1994 `actions`, en1999 `actions`/`elements`, en1991 `impact`/`explosion`).

## Tool notes

- Earlier scripts `analyze.py`, `cover.py`, `handles.py`, `final.py` (`🗑️generated/editability/`) are stale: `final.py` hard-codes handle lists and fallbacks from the old source, and `analyze.py` misses external row types. Not used for this report.
- New scripts in `🗑️generated/editability-2/`: `leaves-schema.py` (leaves), `rules-kinds.py` (rule tables), `coverage-2.py` (coverage, merges `handle-trace.json`). Run with `python3 -I`.
