# SCENE v2 report (wave 2, nothing compiled)

## 1. Persisted `placements` (shared persisted, `Placement { id, x, y, z, yaw }`, SI)
Tool: `🧰️scripts/placements-adder/` (`padd.py` library, `en1990.py`, `en1996.py`, `en1997.py` drivers; python3 -I, idempotent, `--dry`). It clones the artifact's own last row collection through every twin: model struct, root + snapshot rs/ts/graphql/proto/json (inline and `$defs`/`definitions` variants), diff lane (rs keys + `row_patch!`/`list_delta!` macros, ts, graphql, proto, json incl. hybrid formatting), Default/struct literals, JSON fixtures (`"placements": []`, empty diff deltas), SQLite (sql, rs, ts, admission, rs/ts tests), schemaBytes via the repo `check_schema` formula (sum of statement bytes + table-name bytes; verified 12337 on en1999), tableWidths (16), field-meta EN+DE, outline and duplicate-id check.

| Artifact | State |
|---|---|
| en1999 | already had it |
| en1990 | DONE: Delta lane `En1990PlacementDelta`, table `en1990_placement` (13 tables), admission field 23, 137 fixtures, example literals (7), field-meta + test list, duplicate-id check |
| en1996 | DONE: `En1996PlacementDelta` (diff graphql is a `walls: JSON` stub: cloned; diff proto has no walls lane), table `en1996_placement` (13), admission field 5, fixtures, field-meta. Unverified: the test `empty.walls.clear(); tables.len() == 5` |
| en1997 | DONE (priority): `En1997PlacementsRows/Patch`, `PlacementKey`, table `en1997_placement` (17), admission field 18, 337 fixtures, field-meta (in checks `FieldSpec` registry), outline, duplicate-id check. Footing positions persist as `placements[id=<footingId>]` |
| en1991, en1993, en1994, en1995, en1998 | NOT done (budget). Needs per-artifact driver like the three above; en1994 uses sub-crates for rows and Rows lanes, en1993 has binary/text io per collection, en1991 has a different sqlite widths layout |

Open per done artifact: LEAF-KINDS v2 needs `add-placement`/`remove-placement` and `change-placement-x|y|z|yaw`; DSL assets (`🖼️assets/**/🗣️.dsl.semio`, packs) regenerate on first test; `schema-document-index.json` refresh by infra; every-word SQLite tests do not yet set placement words (coverage only); x/y/z/yaw checks of placement targets (existing entity) not added except duplicate ids.

## 2. Visuals
- EN 1990 gamma*psi bars: DONE (`gamma-psi` diagram, bars gamma_Q*psi0 / psi1 / psi2 per variable action, table twin, unit test).
- psi single source: DONE for EN 1991 (registry `ndp.en1990.psi.*` in compliance; `psi_factors`, `psi_factors_of_key`; `psi_1_2` and PSI_ACTIONS values removed; tests adjusted). `psi_0(category)` keeps its signature and reads the EN annex value (DE differs only where the registry differs).
- NOT done: EN 1993 haunches/gussets/effective width (needs snapshot fields), EN 1994 3D slip arrows, EN 1992 crack pattern (s_r,max, w_k).

## 3. scene_of coordination
No call site changed. `placements_of` and `revision_of` stay the input stage; see requests.md.

Unverified/risks: everything uncompiled; json twins preserve their formatting by round-trip check; en1990 snapshot `Default` literal and 7 example literals were extended by `placements: vec![]`.
