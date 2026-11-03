# EN 1998 v3 - soil layers collection (fork report)

Script: `🧰️scripts/field-adder/new_collection_en1998.py` (EN 1998 profile of `new_collection.py`; run `/home/deck/semio/.venv/bin/python -I ... [--dry]`; idempotent, second run changed 0 files). NOTHING COMPILED.

Entity `En1998SoilLayer` (`soilLayers[]`, Rust `soil_layers`, SQLite `en1998_soil_layer`), appended after `towers`: `id`, `foundationId`, `thicknessM` (m, dsl unit), `unitWeightNM3` (N/m3), `shearWaveVelocityMS` (m/s), `n160`, `finesPercent`. Rows of one foundation are ordered top to bottom by row position. Diff key `id`.

## Files touched (all under `...en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/`)
- model `🧬️schema/📸️snapshot/🦀️.rs` (struct, field with `#[dsl(table)]`, default literal), artifact `🧬️schema/🦀️.rs` (field, to/from snapshot).
- twins: `🧬️schema/{🟦️.ts,🔣️.json}`, `🧬️schema/📸️snapshot/{🟦️.ts,🔣️.json,🛰️.proto (field 12),🔗️.graphql}`; diff `🧬️schema/🔺️diff/{🦀️.rs,🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto (field 12)}` with Patch / Removal / Insertion / Relocation / Modification / Delta.
- io: import lists in binary + text snapshot codecs; SQLite `🗄️.sql` (table 18, `foundation_logical_id` column), `🦀️.rs`, `🟦️.ts` (write, read, bounds, row count), tests rs + ts (table count 17 -> 18, loops, `set_every_word`, `ownedFixture`), fixture `🔣️.json` (one layer row).
- `🏷️field-meta/🦀️.rs` (EN + DE, units Metre / NewtonPerCubicMetre / MetrePerSecond), outline (section list + entry count), `🧮️units` (`ROW_COLLECTIONS`, foundations unit reads `soilLayers`), inference `reads`, duplicate-id integrity line in `⚖️part-evaluators` (one line next to towers).
- DSL assets: empty `soil-layers [...]` table in the 4 `🗣️.dsl.semio`; 600 mutation fixtures got `"soilLayers": []` / an empty delta (JSON validity checked on write).

## Not done / risks
- Mutation kinds, edit rules and mutation codec rows: requested from leaf-kinds D (`requests.md`). `every_editable_leaf_has_a_set_field_rule` fails until then.
- Scene, kit layer list, oracles, checks not touched (owner). Dangling `foundationId` reference check not added (owner can add next to the duplicate-id line).
- Example rows: the examples keep an empty profile (the multipart foundation has all liquefaction inputs 0); the binary `🎒️.pack.semio` needs one regeneration (requested from infra).
- `placements` landed concurrently in model/twins/SQLite: my field sits between `towers` and `placements` in the struct and DSL order, proto numbers are max+1 (12). The placement `ownedFixture` in the sqlite ts test spreads `...fixture` (placements untouched by `floats`); fine while placements are empty.
- Unverified by compilation: sqlite rs/ts one-line codecs (anchors matched, columns 10, float columns 5..9), diff ts reader order, proto/graphql numbering, JSON twins reformatted by `dump_schema` (it asserted canonical form; no warnings).
- `x-semio-ui` of `foundationId` uses `widget: reference` with `ref.kind: foundation` (free-form; only `building`, `material`, `section` existed).
