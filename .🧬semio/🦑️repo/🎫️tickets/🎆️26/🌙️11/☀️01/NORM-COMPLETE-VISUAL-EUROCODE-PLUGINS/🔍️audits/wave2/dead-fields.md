# Dead fields (wave 2): `reliabilityClass` removed from en1997 and en1998

Status: done end to end, nothing compiled (no cargo, no git write). `grep -i reliability` over both artifact trees: 0 files. en1994 untouched.

## Tools
`🧰️scripts/dead-fields/{dead_lib,dead_en1997,dead_en1998}.py` (dry run by default, `--apply` writes; every edit asserts its match count).

## Removed in both artifacts
- Schema JSON: `reliabilityClass` property and `required` entry in artifact, snapshot and diff; twins (ts, proto, graphql, rust structs) regenerated with `schema twins generate --under <artifact>`; the hand-written `En199xDiff` struct field and its impls (`absorb`, `apply`, `is_empty`, `inverse`) edited by hand.
- Model: snapshot field, default (`RC2`), artifact `to_snapshot`/`from_snapshot`.
- SQLite: DDL column + CHECK, Rust and TS codecs (cell, reader, entity widths en1997 9->8, en1998 3->2, `bound.repeated(3,24)`), en1997 admission `Enumeration(20, ...)`, TS test `unknown class` row, snapshot fixtures.
- Contract (en1997): `schemaBytes` and `tableWidths` replayed with python sqlite3 from the new DDL (stored contract matched the replay before: 19506), `valueBytes` -3 per case in `🎛️semantic.json` and `📏️native-extent`, independent fixture column and cell removed (`totalCells` 422 -> 420), TS literals updated.
- Kind `change-reliability-class`: leaf directory (rust, json, ts twin, inverse, diff), group module mount and re-export, mutation enum variant, `KINDS`, edit rule, text codec (en1997 DSL enum mirror too), binary protocol record (tags 189 and 278 left as gaps; the protocol resolves records by table), demo cases (en1998), oracle manifest (en1997: 3 packs, en1998: 1), feature rows, python and rust test harness, apply fixtures.
- Field-meta picker and `RC` choices (en1997 checks, en1998 field-meta), outline, inference reads (en1998), DSL assets (own-line `reliability-class=rc2` token), 1500 JSON fixtures (snapshot/diff: en1997 610, en1998 892).
- Tests removed: en1997 `reliability_class_does_not_scale_the_geotechnical_partial_factors`, `reliability_class_of_the_document_leaves_every_verdict_unchanged`; en1998 `reliability_class_rc3_raises_the_fundamental_design_effect_by_k_fi_but_not_the_seismic_one`, `reliability_class_of_the_document_reaches_the_evaluated_snapshot_without_changing_seismic_verdicts`; unused imports pruned.

## Compile bugs of the RC threading found and fixed on the way
- en1997 `resolve_params` still passed an undefined `reliability_class` (E0425).
- en1997 text and binary mutation codecs had a cloned duplicate `use ...::{change_reliability_class, change_design_approach, ...}` line (E0252 once the kind exists); gone now.
- en1997 and en1998 `mutate-*` rust tests had an interleaved `"change-annex" => (` / `"change-reliability-class" => (` fixture arm (8 include_str lines mixing both kinds); now only the annex arm remains.
- The DSL assets carried the token on its own last line in the current layout (not inline); handled.

## Checks run (niced)
- `schema twins check --strict`: identical=4922 drift=0 (whole repo).
- `schema-sync/check.py --artifact en1997 --artifact en1998`: mismatches=0.
- `leaf_coverage.py en1997 en1998`: 0 content leaves without a kind (156 and 239).
- Fixture check (own script, inline): all snapshot (`before/after`, 408 and 594) and diff (193 and 289) mutation fixtures have keys within the schemas and all required keys; the only invalid JSON are the two zero-byte `🎬️scene/.../🏛️scene/🔣️.json` (created Nov 2 14:49, not from this work).
- `verify build-owners sync`: removed=0 added=0, passed.

## Handoff
`- ready for rebuild: dead fields` appended to `build-en1997/latest.md` and `build-en1998/latest.md`.

## Unverified
Nothing compiled; expect no rustc surprises beyond unused imports. `ReliabilityClass` stays in the shared compliance crate and the other seven artifacts.
