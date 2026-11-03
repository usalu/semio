# SCHEMA-TOOLING (wave 2): `schema-evolve`

Tool: `🧰️scripts/schema-evolve/` (`evolve.py`, layers `l_*.py`, specs, `test_evolve.py`, `selfcheck.py`). Usage is in `fleet-wave2-brief.md`, section "Schema evolution tool". Deleted (ported): `🧰️scripts/field-adder/new_collection.py`, `🧰️scripts/en1993-entity-gen.py`. No cargo, no `repo run`, no git writes; heavy commands through `heavy.sh`; freeze.txt only named build-en1992 (not touched).

## What was ported
`new_collection.py`: unit table, schema property templates, UI nodes, SQLite contract replay (python `sqlite3`), fixture value-span insertion, field-meta rows. `en1993-entity-gen.py`: diff Patch/Rows/Key templates, outline/units/integrity wiring ideas. The fragile hard-coded anchors were replaced by anchors found through a reference sibling collection (the last flat collection, or one with a nested array); layers that cannot find their anchors report `UNSUPPORTED` instead of guessing.

## Validation
1. Dry run `add-collection en1997 specs/en1997-investigations.json` (before apply): 630 files (schema x4, model/artifact/diff Rust, SQLite sql/rs/ts/admission/tests, wiring x4, 611 fixture JSON, 3 DSL assets), 0 unsupported layers.
2. Dry run `add-leaf en1997 layers specs/en1997-layers-cptnkt.json` (`layers[].cptNkt`, the layers entity the old field-adder profile lacked): 458 files (schema x3, diff set list, model literal, SQLite DDL + rs + ts + admission, contract, field-meta, 444 fixtures, DSL), 0 unsupported layers.
3. Dry run `add-collection en1998 specs/en1998-tie-beams.json`: 635 files planned, 7 layers reported UNSUPPORTED with reasons: diff row group of the reference `joints` (different diff layout), SQLite write loop of `placements`, incremental units group, duplicate-id integrity call, no `lookup_field_meta` module, no insert/remove edit rules to clone. Apply is refused until those anchors exist.
4. `test_evolve.py`: 6 tests OK (python `sqlite3` executes the generated DDL, CHECK constraint rejects an unknown enum token, contract replay bytes equal the `sqlite_schema` text sizes; generators rebuilt for existing en1997 `layerWaters`, `labTests`, `pileGroups` reproduce the committed DDL, Rust/TS cells and admission cells verbatim; EN + DE label validation; name forms).

## Applied: en1997 `investigations[]` (for EN1997-PART2)
`Investigation {id, positionX, positionY, groundLevel, depth, kind: borehole|cpt|spt (enum InvestigationKind), readings[]}` with nested `InvestigationReading {id, depth, measurement}` (the field is `measurement`, not `value`: the verifier engine treats a field named `value` as the generic `newValue` argument and cannot resolve its kind). Appended behind `cases` in every positional layer (snapshot order, SQLite tables 21 and 22, root field id 20, DSL tables), so no stored index moved.
- Schema JSON (snapshot, artifact, diff), generated twins, model enum `InvestigationKind` + example literal, artifact conversions, diff Key/Patch/Rows macros (rows and readings) with absorb/apply/is_empty/inverse arms.
- SQLite: tables `en1997_investigation`, `en1997_investigation_reading` (DDL, Rust and TS codecs, admission, contract replayed: schemaBytes 20934, 22 tables).
- Wiring: outline, duplicate-id integrity (rows and readings), incremental units (ROW_COLLECTIONS, owners, group), field-meta EN + DE with the choice list, 611 snapshot/diff fixture JSON, 3 DSL assets.
- Mutation kinds (11): `insert/remove-ground-investigation`, `insert/remove-ground-investigation-reading` (cloned from `layerWaters`/`labTests` kinds with all registries: aggregate, KINDS, edit rules, text codec, binary protocol tags 191-194, harness py/feature/rs, oracle manifests, fixtures), and 7 `change-ground-investigation[-reading]-*` set-field kinds from the leaf-kinds en1997 driver (new crates, tags retagged above all others, build-owners synced).
- Gates after apply: `schema twins check --strict --under en1997` identical (ts 206, proto 5, graphql 5, rust 205, drift 0), `schema-sync` 0 mismatches, `handle-paths` ok, `check-cargo-authority` 31 crates 0 errors, `verify build-owners sync` passed (added=0), `verify_fixtures.py en1997 ground-investigation`: 12 scenarios, 11 inverted, 0 failures.

## Fixed on the way
- `leaf-kinds/model.py` `Leaf.label` fell back to the raw field name; it now raises when EN or DE is missing (I18N-MUTATIONS requirement). The tool itself requires EN + DE for every field, entity, singular and enum value, and composes mutation labels from them (`<entity de>: <field de> ändern`, `Insert <singular en>` / `<singular de> einfügen`).
- `leaf-kinds/diff97.py` `patch_rust` wrote `enum:Type` into the `set { .. }` list of the Patch macros (invalid Rust). It now writes `<model>::<Type>`; the 10 broken lines it had produced in the en1997 diff Rust were repaired. The tool removes duplicate Patch declarations the driver leaves behind.
- The stored en1997 SQLite contract (19425 bytes) was already stale against the committed DDL (19845 by python `sqlite3` replay, others' DDL changes); the tool recomputed `schemaBytes`/`tableWidths` from the DDL. Per-case `valueBytes` of the contract were NOT recomputed (cases are unchanged by the empty new tables, but earlier foreign column additions may still be unaccounted for).

## Open / unverified
- Nothing compiled. Rust and TS text is generated from the same templates that reproduce the committed codecs (selfcheck), but only a build proves it. Watch: text codec `#[dsl(block)]` clones, kind `Cargo` edges of the 2 new leaf-kinds crates, byte-bound loop in the SQLite preflight, mutation unit tests (`🧪️tests/🔬️unit`, not extended), error prose of cloned insert/remove kinds still names the reference entity ("LayerWater", "laboratory test").
- No scene entities, checks or catalogue cases use `investigations` yet (EN1997-PART2 owns them); the incremental units group has no check owner for it.
- Only en1997 has a leaf-kinds driver registered in `DRIVERS`; other artifacts skip the set-field kinds with a message.
- Legacy collection specs in `field-adder/specs/en1991-*.json` belonged to the deleted tool and are now orphaned data.
