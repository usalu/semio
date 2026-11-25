# Undeclared-links audit: norm insert-mutation rows (2026-11-25)

Read-only audit, produced by `undeclared-links-audit.py` (same folder) plus manual tracing of the sources. No build, no git write, no other file edited.

## Method

- Scope: every insert mutation folder under `🧬️schema/🧬️mutations/` whose name carries `insert-` (after the emoji prefix) in the 13 artifact families that have one. The script found 227 such folders (the ticket note says 236; not reconciled).
- Row schema: the payload property of `🧬️schema/🔣️.json` that is an object or an array of objects, followed through `$ref` (e.g. `…/en1993/snapshot.json#/$defs/…`) into the family snapshot.
- Scanned: every required string field of that row, recursing into required nested objects and required arrays (paths like `layers[].grade`). The row's own top-level `id` is excluded.
- Class A: the field declares `x-semio-ui.ref.kind` and no insert rule of the family has a matching item kind (compared without case and separators).
- Class B: the field declares no reference, and its name ends in `Id`/`Ids`, or its title/description says it refers to or identifies another entity (en/de regex; `points to` false positive removed by hand).
- Usage: the family's inferences (`💡️inferences`), validators (`⚖️` folders), diff (`🔺️diff`), examples and tests were searched for lookups of the field. Targets are stated where a lookup names the collection; otherwise the schema text is the only source and the entry is marked accordingly.
- Empty string: schema check (`enum`, `pattern`; no `minLength` exists in any family schema) and the family's own code at the read site. Verdicts: **refused** (schema enum, or a finding or error at the read site with no empty skip), **accepted** (explicit by schema text or code, or silent at the read site), **unclear** (no read site found, or the empty branch not traced).
- Line numbers for declarations are found by searching the property name forward from the definition, so they can be a few lines off. Code citations were read directly.
- All paths are relative to each family's `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<family>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/` folder.

## Counts per family

| Family | Insert rules | Entries | A (declared, unmatched) | B (undeclared link) | Refused | Accepted | Unclear |
|---|---:|---:|---:|---:|---:|---:|---:|
| ⚖️en1990 | 14 | 3 | 1 | 2 | 1 | 2 | 0 |
| ⚡️din18599 | 2 | 1 | 0 | 1 | 0 | 0 | 1 |
| 🌍️en1997 | 20 | 7 | 1 | 6 | 4 | 1 | 2 |
| 🌬️din16798 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🏋️en1991 | 35 | 2 | 0 | 2 | 1 | 1 | 0 |
| 🏛️en1992 | 15 | 3 | 0 | 3 | 0 | 0 | 3 |
| 🔩️en1993 | 43 | 31 | 0 | 31 | 1 | 6 | 24 |
| 🧩️en1994 | 9 | 2 | 0 | 2 | 0 | 1 | 1 |
| 🧱️din4108 | 6 | 4 | 2 | 2 | 1 | 2 | 1 |
| 🪨️en1996 | 15 | 1 | 0 | 1 | 0 | 0 | 1 |
| 🪵️en1995 | 17 | 4 | 3 | 1 | 1 | 2 | 1 |
| 🪶️en1999 | 22 | 8 | 2 | 6 | 0 | 2 | 6 |
| 🫨️en1998 | 27 | 10 | 0 | 10 | 0 | 6 | 4 |
| **Total** | **227** | **76** | **9** | **67** | **9** | **23** | **44** |

An entry is one mutation and one field. `🌬️din16798` (2 inserts) has no entries.

## Entries whose empty value is refused

- ⚖️en1990 `📎insert-effect` `actionId`: finding: `💡️inferences/⛓️referential-integrity/🦀️.rs:115-117` (`push_dangling_ref`), no empty skip
- 🌍️en1997 `➕insert-lab-test` `layerId`: finding: `link_check` in `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86` fails when the value is not in the options; `""` is not an id
- 🌍️en1997 `➕insert-layer-water` `layerId`: finding: `link_check` Fail, `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86`
- 🌍️en1997 `➕insert-pile-group` `pileId`: finding: `link_check` Fail, `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86`
- 🌍️en1997 `➕insert-anchor` `wallId`: finding: `link_check` Fail, `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86`
- 🏋️en1991 `➕️insert-snow-guard` `roofId`: finding: `💡️inferences/🧷️wave3-rows/🦀️.rs:141` `invalid(...)` when no roof matches
- 🔩️en1993 `➕️insert-frame-action` `targetId`: error: `💡️inferences/⚖️checks/🕸️frame-eigen/🦀️.rs:55` / `:62` `Err(missing(...))`
- 🧱️din4108 `🏠️insert-element` `zoneId`: finding: `💡️inferences/🦀️.rs:197-199` `push_dangling_ref`, no empty skip
- 🪵️en1995 `➕️insert-panel` `strengthClass`: schema: enum of strength classes at snapshot line 19 does not contain ""

## Table per family

### ⚖️en1990

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 📎insert-effect | `actionId` | A | kind `action`, no insert rule (schema text Action/Einwirkung) | action ids of the document: `💡️inferences/⛓️referential-integrity/🦀️.rs:90-91` (`action_ids`, `action_set`), checked at `:115` | finding: `💡️inferences/⛓️referential-integrity/🦀️.rs:115-117` (`push_dangling_ref`), no empty skip | refused |
| 2 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry / Katalogeintrag' (snapshot line 33) | catalogue entry: `💡️inferences/⚖️checks/📇️catalogue/🗂️cases/🌱️demo/🦀️.rs:44-45`, `📐️geometry/🦀️.rs:44-45` | silent: `🌱️demo/🦀️.rs:45` returns the values unchanged when no demo entry matches; no finding | accepted |
| 3 | 🎲️insert-reliability-analysis | `memberId` | B | no ref; text 'Member (empty = whole project)' | member; schema text only (`📸️snapshot/🔣️.json` decl line 19); no reader of this field found in `💡️inferences/⛓️referential-integrity/🦀️.rs:105-125` (reads effects and bridgeSls only) | by design: schema text 'empty = whole project' | accepted |

### ⚡️din18599

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 🆕insert-element | `zoneId` | B | no ref; no text | zones: `💡️inferences/⚖️checks/🔥️derived-balance/🦀️.rs:149` (`zones.find(zone.id == element.zone_id)`) | the zone integrity check for elements was not located in the sources read | unclear |

### 🌍️en1997

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕insert-slope | `governingLayerId` | A | ref kind `soil-layer`, no insert rule (decl snapshot line 24) | soil layers: `💡️inferences/⚖️checks/📐️geotechnical-parts/🦀️.rs:291` (`layers.any(l.id == governing_layer_id)`) | `:291-292` returns None for empty; the caller's finding was not traced | unclear |
| 2 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | catalogue entry: `💡️inferences/⚖️checks/📇️catalogue/🗂️cases/🌱️demo/🦀️.rs:91` | silent: `:91` `else { return values }`, no finding | accepted |
| 3 | ➕insert-lab-test | `layerId` | B | no ref; text 'Layer / Schicht' | soil layers: `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:188` (`link_check(.."layerId", layers)`) | finding: `link_check` in `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86` fails when the value is not in the options; `""` is not an id | refused |
| 4 | ➕insert-layer-water | `layerId` | B | no ref; text 'Layer / Schicht' | soil layers: `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:247` (`link_check`) | finding: `link_check` Fail, `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86` | refused |
| 5 | ➕insert-characteristic-basis | `layerId` | B | no ref; text 'Layer / Schicht' | soil layers: `🌐️geo-entities/🦀️.rs:50` (`b.layer_id == layer_id`); no `link_check` for bases found | no empty-value check for characteristic bases located | unclear |
| 6 | ➕insert-pile-group | `pileId` | B | no ref; text 'Pile id / Pfahl-ID' | piles: `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:163` (`link_check("pileGroups", ..., doc.piles)`) | finding: `link_check` Fail, `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86` | refused |
| 7 | ➕insert-anchor | `wallId` | B | no ref; text 'Supported wall id / Gestützte Wand-ID' | embedded walls: `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:92` (`link_check("anchors", ..., embedded_walls)`) | finding: `link_check` Fail, `💡️inferences/⚖️checks/🌐️geo-entities/🦀️.rs:75-86` | refused |

### 🌬️din16798

No required string field was flagged by class A or B.

### 🏋️en1991

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | catalogue entry: `💡️inferences/⚖️checks/📇️catalogue/🗂️cases/🌱️demo/🦀️.rs:104-105` | silent: `:105` `else { return values }` | accepted |
| 2 | ➕️insert-snow-guard | `roofId` | B | no ref; text 'Roof / Dach' | roofs: `💡️inferences/🧷️wave3-rows/🦀️.rs:140` (`roofs.find(r.id == row.roof_id)`) | finding: `💡️inferences/🧷️wave3-rows/🦀️.rs:141` `invalid(...)` when no roof matches | refused |

### 🏛️en1992

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | route table: `💡️inferences/⚖️checks/📋️members/🧷️entity-cases/📍️reach/🦀️.rs:207-208` (`REACH.binary_search`) | `route_of` returns None for ''; the caller's handling (`📋️members/🧷️entity-cases/🦀️.rs:8-9`) not traced | unclear |
| 2 | ➕️insert-member | `layerId` (`bars[].layerId`) | B | no ref; text 'Layer id / Lage-Id' | the member's longitudinal layers: `💡️inferences/⚖️checks/🧰️helpers-and-annex-parts/🦀️.rs:24-25` (path `members[id].longitudinal[id=…]`), bars matched at `💡️inferences/⚖️checks/📐️section-resistance/🦀️.rs:328` | no empty-value check for bar layer ids located | unclear |
| 3 | 📍️insert-bar-point | `layerId` (`bars[].layerId`) | B | no ref; text 'Layer id / Lage-Id' | as insert-member (`💡️inferences/⚖️checks/📐️section-resistance/🦀️.rs:328`) | no empty-value check for bar layer ids located | unclear |

### 🔩️en1993

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕️insert-bridge-fatigue | `detailId` | B | no ref; text 'Orthotropic deck detail id' | orthotropic detail catalogue: `💡️inferences/⚖️checks/🔗️special-checks/🏛️annex-block-checks/🦀️.rs:272` (`bridge_fatigue::orthotropic_detail(&bf.detail_id)`) | fallback: `:272` `map_or(f64::from(bf.category), ...)` | accepted |
| 2 | ➕️insert-fatigue-stress | `detailId` | B | no ref; text 'Detail-Id orthotroper Fahrbahn' | fatigue details: `💡️inferences/⚖️checks/⏳️stress-range-checks/🦀️.rs:31,53` | silent: `:53` `else { continue }`, no finding | accepted |
| 3 | ➕️insert-frame-bar | `endNodeId` | B | no ref; text 'End node id' | frame nodes: `💡️inferences/⚖️checks/🕸️frame-eigen/🦀️.rs:109,112`, `💡️inferences/⚖️checks/🔗️special-checks/🕸️frame-checks/🦀️.rs:125` (via helper `at`, definition not located) | the `at` helper that resolves node ids was not located | unclear |
| 4 | ➕️insert-frame-bar | `startNodeId` | B | no ref; text 'Start node id' | frame nodes: `💡️inferences/⚖️checks/🕸️frame-eigen/🦀️.rs:484` (`frame_nodes.find(n.id == bar.start_node_id)`) | `:484` `origin` is an Option; the consumer of None not traced | unclear |
| 5 | ➕️insert-foundation-block | `jointId` | B | no ref; text 'Base plate joint id' | joints: `💡️inferences/⚖️checks/🔗️special-checks/🔩️bolt-group-ic-checks/🦀️.rs:20` (`joints.find(j.id == p.joint_id)?`) | silent: `:20` `?` returns None, no finding | accepted |
| 6 | ➕️insert-joint-plate | `jointId` | B | no ref; text 'Base plate joint id' | joints: `💡️inferences/⚖️checks/🔗️special-checks/🔩️joint-row-model/🦀️.rs:135` (`joints.find(j.id == joint_id)?`) | silent: `:135` `?`, no finding | accepted |
| 7 | ➕️insert-joint-row | `jointId` | B | no ref; text 'Base plate joint id' | joints: `💡️inferences/⚖️checks/🔗️special-checks/🔩️joint-row-model/🦀️.rs:91,135` (`joint_rows.filter(r.joint_id == j.id)`) | silent: `:91` filter, `:135` `?` | accepted |
| 8 | ➕️insert-frame-action | `loadCaseId` | B | no ref; text 'Load case / Lastfall' | load cases: `💡️inferences/⚖️checks/🕸️frame-envelope/🦀️.rs:36` (`load_cases.any(c.id == a.load_case_id)`) | `:36` only detects a missing load case; the reported outcome for `""` not traced | unclear |
| 9 | ➕️insert-material-ductility | `materialId` | B | no ref; text 'Material / Werkstoff' | materials (schema text only); members' `material_id` readers at `💡️inferences/⚖️checks/🔗️special-checks/⏳️fatigue-fire-and-thin-walled-checks/🦀️.rs:255` | no reader of this row's materialId located | unclear |
| 10 | ➕️insert-fatigue-stress | `maxCaseId` | B | no ref; text 'Load case of the maximum' | load case by schema text (`📸️snapshot/🔣️.json` decl line 79); no inference reader located | no reader located | unclear |
| 11 | ➕️insert-fatigue-stress | `minCaseId` | B | no ref; text 'Load case of the minimum' | load case by schema text (decl line 80); no inference reader located | no reader located | unclear |
| 12 | ➕️insert-bridge-fatigue | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 13 | ➕️insert-buckling-length | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 14 | ➕️insert-built-up-member | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 15 | ➕️insert-crane-runway | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 16 | ➕️insert-fastened-joint | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 17 | ➕️insert-fatigue-detail | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 18 | ➕️insert-fire-exposure | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 19 | ➕️insert-forming-gain | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 20 | ➕️insert-frame-bar | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 21 | ➕️insert-frame-member-basis | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 22 | ➕️insert-hollow-joint | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 23 | ➕️insert-joint | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 24 | ➕️insert-member-action | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 25 | ➕️insert-restraint | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 26 | ➕️insert-storey | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 27 | ➕️insert-tower-leg | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 28 | ➕️insert-weld-group | `memberId` | B | no ref; no text | members: e.g. `💡️inferences/⚖️checks/🔗️special-checks/🪜️built-up-checks/🦀️.rs:170` (`members.find(m.id == row.member_id) else { continue }`) | `:170` skips silently; integrity tests expect Fail for unknown memberId (`🧪️tests/⚖️compliance/🦀️.rs:415,425`); the empty branch of the integrity check not traced | unclear |
| 29 | ➕️insert-parametric-section | `sectionId` | B | no ref; text 'Id des beschriebenen Querschnitts' | sections (schema text 'described cross-section'); member section lookup at `💡️inferences/⚖️checks/🔗️special-checks/🧷️restraint-checks/🦀️.rs:154` is a different row | no reader of this row's sectionId located | unclear |
| 30 | ➕️insert-frame-action | `targetId` | B | no ref; text 'Target / Ziel' | frame node or frame bar, chosen by the action kind: `💡️inferences/⚖️checks/🕸️frame-eigen/🦀️.rs:54-55` (nodal) and `:61-62` (bar) | error: `💡️inferences/⚖️checks/🕸️frame-eigen/🦀️.rs:55` / `:62` `Err(missing(...))` | refused |
| 31 | ➕️insert-member | `webMaterialId` | B | no ref; text 'empty = the member material' | materials: `💡️inferences/⚖️checks/🧱️member-checks/🦀️.rs:117` (`find_material`) | by design: `:116` `if !member.web_material_id.is_empty()` | accepted |

### 🧩️en1994

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 📍️insert-stud | `beamId` | B | no ref; text 'Beam / Träger' | beams: `💡️inferences/⚖️checks/🔩️studs/🦀️.rs:58` (`studs.filter(s.beam_id == beam.id)`) | silent: `:58` groups studs per beam; no finding for unmatched studs | accepted |
| 2 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | catalogue entry by schema text; no inference reader located (only `🔺️diff/🔣️.json:69`) | no reader located | unclear |

### 🧱️din4108

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕️insert-layer | `materialId` | A | ref kind `material`, no insert rule (decl snapshot line 89) | catalogue materials: `💡️inferences/🦀️.rs:210-211` (`catalogue_set`) | explicit skip: `:210` `!layer.material_id.is_empty() && ...` | accepted |
| 2 | 🏠️insert-element | `materialId` | A | ref kind `material`, no insert rule | catalogue materials: `💡️inferences/🦀️.rs:223` (segments) | explicit skip: `:223` `!seg.material_id.is_empty()` | accepted |
| 3 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | catalogue entry by schema text; no inference reader located | no reader located | unclear |
| 4 | 🏠️insert-element | `zoneId` | B | no ref; no text | zones: `💡️inferences/🦀️.rs:197` (`zone_set.contains(element.zone_id)`) | finding: `💡️inferences/🦀️.rs:197-199` `push_dangling_ref`, no empty skip | refused |

### 🪨️en1996

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | masonry catalogue entries: `📇️masonry-catalogue/🗂️cases/🧪️tests/🔬️unit/🦀️.rs:11,42,48` (tests only) | only test readers located | unclear |

### 🪵️en1995

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕️insert-panel | `grade` (`layers[].grade`) | A | ref kind `strength-class`, no insert rule (decl snapshot line 21) | strength classes: `⚖️kernel/📚️strength-tables/🦀️.rs:142` (`strength_class_names`) | no enum or pattern in schema; no inference reader of `.grade` located (only `🧪️tests/⚖️compliance/🦀️.rs:1510`) | accepted |
| 2 | ➕️insert-panel-layer | `grade` | A | ref kind `strength-class`, no insert rule | strength classes: `⚖️kernel/📚️strength-tables/🦀️.rs:142` | no enum or pattern in schema; no inference reader located | accepted |
| 3 | ➕️insert-panel | `strengthClass` | A | ref kind `strength-class`, no insert rule (decl snapshot line 19) | strength classes: `⚖️kernel/📚️strength-tables/🦀️.rs:142`; used as `m.strength_class.id()` at `⚖️timber/🧱️entities/🦀️.rs:360` | schema: enum of strength classes at snapshot line 19 does not contain "" | refused |
| 4 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | catalogue cases: `⚖️kernel/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/🦀️.rs:31,36` (tests only) | only test readers located | unclear |

### 🪶️en1999

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕insert-fatigue-category | `elementId` | A | ref kind `element`, no insert rule (decl snapshot line 34) | element: schema ref only; `🔺️diff/🔣️.json:36` shows the field; no inference reader located | no reader located | unclear |
| 2 | ➕insert-outline-vertex | `elementId` | A | ref kind `element`, no insert rule | element: schema ref only; no inference reader located | no reader located | unclear |
| 3 | ➕insert-weld-run | `elementId` | B | no ref; text 'Welded plate element' | section elements: `💡️inferences/⚖️checks/🧵️weld-run-checks/🦀️.rs:40,72` (`s.elements.find(e.id == run.element_id)`) | silent: `:72` `section.and_then(...)` returns None, no finding at this site | accepted |
| 4 | ➕insert-fire-insulation | `fireId` | B | no ref; text 'Fire scenario' | fire scenarios: `💡️inferences/⚖️checks/🎯️entity-checks/🦀️.rs:227` (`fire_scenarios.any(f.id == insulation.fire_id)`) | fallback: `:227` owner becomes the insulation's own id; no finding | accepted |
| 5 | ➕insert-connection | `memberId` | B | no ref; no text | members: schema ref absent; test `🧪️tests/⚖️compliance/🔎️leaf-influence-and-material-laws/🦀️.rs:425` expects a Fail for a dangling connection memberId; production check not traced | production empty-value branch not traced | unclear |
| 6 | ➕insert-fatigue-detail | `memberId` | B | no ref; no text | members: no reader located for this row's memberId in the sources read | no reader located | unclear |
| 7 | ➕insert-fire-scenario | `memberId` | B | no ref; no text | members: `🧪️tests/⚖️compliance/🔎️leaf-influence-and-material-laws/🦀️.rs:152`; production `💡️inferences/⚖️checks/🧾️structure-evaluation/🦀️.rs:331` only runs when the member is found | the no-member branch not traced | unclear |
| 8 | ➕insert-sheet-profile | `sheetId` | B | no ref; text 'Sheet / Blech' | cold-formed sheets: `💡️inferences/⚖️checks/🎯️entity-checks/🦀️.rs:252-255` | `:252` falls back to profile id; the `else` at `:255` not traced | unclear |

### 🫨️en1998

| # | Mutation | Field (path) | Class | Declared or text evidence | Points to (source) | Empty value | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | ➕️insert-isolation-system | `bridgeId` | B | no ref; no text | bridges: `💡️inferences/⚖️part-evaluators/🦀️.rs:80-84`, lookup at `:252` | silent: `:81-84` only checks a named bridge; empty means 'building instead' | accepted |
| 2 | ➕️insert-pier-base | `bridgeId` | B | no ref; no text | bridges: reader for the pier-base row not located (`💡️inferences/⚖️part-evaluators/🦀️.rs:66-67` is for bearings) | no reader located | unclear |
| 3 | ➕️insert-critical-region | `buildingId` | B | no ref; text 'Owning building / Zugehöriges Gebäude' | buildings: reader not located | no reader located | unclear |
| 4 | ➕️insert-isolation-system | `buildingId` | B | no ref; text 'Owning building' | buildings: `💡️inferences/⚖️part-evaluators/🦀️.rs:82,85`, lookup at `:253` | silent: `:85` requires a named building; `:82` only fires when both are named | accepted |
| 5 | ➕️insert-case | `entryId` | B | no ref; text 'Catalogue entry' | catalogue entry by schema text; `🔺️diff/🔣️.json:153` only | no reader located | unclear |
| 6 | ➕️insert-tie-beam | `foundationAId` | B | no ref; text 'First foundation / Erstes Fundament' | foundations: `💡️inferences/⚖️part-evaluators/🦀️.rs:581` (`foundations.filter(f.id == t.foundation_a_id ...)`) | silent: `:581` filter only, no finding | accepted |
| 7 | ➕️insert-tie-beam | `foundationBId` | B | no ref; text 'Second foundation / Zweites Fundament' | foundations: `💡️inferences/⚖️part-evaluators/🦀️.rs:581` | silent: `:581` filter only, no finding | accepted |
| 8 | ➕️insert-seismic-slope | `governingLayerId` | B | no ref; text 'Governing soil layer / Maßgebende Bodenschicht' | soil layers: `💡️inferences/🫧️liquefaction-checks/🦀️.rs:59` (`liquefied_ids.contains(sl.governing_layer_id)`) | silent: `:59` a non-matching value simply does not select the slope | accepted |
| 9 | ➕️insert-critical-region | `memberId` | B | no ref; no text | members of the building: `💡️inferences/🌍️full-seismic-check/🧱️system-checks/🔩️critical-regions/🦀️.rs:42` (`building.members.find(m.id == r.member_id)`), else-branch not traced | the else-branch at `:42` not traced | unclear |
| 10 | ➕️insert-pier-base | `pierId` | B | no ref; text 'Pier carrying the bearing (empty = abutment)' | piers: `💡️inferences/⚖️part-evaluators/🦀️.rs:68` | by design: schema text 'empty = abutment'; `:68` `!k.pier_id.is_empty()` skips the check | accepted |

## Unclear entries

44 of 76 entries stayed unclear. The sources do not show the read site for the empty value, or the target is only given by schema text. These are the candidates for a runtime check once the build works.

## Caveats

- Insert count 227 (script) against 236 (ticket note). The script only counts folders under `🧬️schema/🧬️mutations/` with `insert-` after the emoji prefix; the difference was not traced.
- Class A and B checks run on the row schema only. Other required string fields that are linked only by a hand-written check (for example en1997 `wallId`, `pileId`, `layerId` through `link_check`) were still classified by name or text, and their empty verdict was traced by hand.
- The `refused` verdict for en1993 `targetId` is an `Err`, which stops that check, not a finding.
- The en1995 `grade` entries are accepted only because no reader of the field was found; a reader outside the searched folders would change this.
