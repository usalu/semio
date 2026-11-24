# Test serde derive vs schema `required` (2026-11-24)

## Cause

The test-only serde derive (`cfg_attr(any(test, feature = "test-support"), ...)`) omitted zero-valued fields although the snapshot JSON Schema lists them as `required`. Only en1999 did this (the other `skip_serializing_if` in the Norm family are on genuinely optional fields: din16798 `unoccupiedVentilation`, vdi3805 `unit`; they stay).

- `AluminiumSection.height|width|flangeThickness|outerDiameter`: the attribute lived in the schema JSON (`x-semio-rust.attributes`), which the Rust projection splices into `🦀️.rs`. So the schema, not a generator, was the source.
- `PlateElement.width|weldPosition` and `FatigueDetail.deltaSigmaC` (`skip_serializing`): hand-kept Rust (records without `x-semio-rust.derive`).
- A shared helper `is_zero_f64` supported them. The doc text ("omitted when 0", "derived ... not serialized") was stale: `deltaSigmaC` is persisted (sqlite, mutation `ChangeFatigueDeltaSigmaC`).

Three more places hid the same class of mismatch behind a workaround: the din16798 and en1993 `snapshot-schema/🐍️.py` oracles and the en1997 schema test lowercased `annex` (`De` -> `de`) before validating, although the schema enum is `["En","De"]`.

## Contract

A `required` field is always serialised; the serde output validates against the family schema and round-trips exactly. Only fields outside `required` may be skipped.

## Fix

- en1999 schema `📸️snapshot/🔣️.json` (source) and projected `🦀️.rs`: removed all `serde(default, skip_serializing_if)` and `skip_serializing`, the `is_zero_f64` helper, and the stale docs. Rust projection check (`--only rust`, en1999): identical=223 drift=0.
- Shared law in `⚖️compliance/🧪️tests/🔬️python-oracle/🦀️.rs`: `validate_json_schema(schema, label, json)` (third-party `jsonschema`) and `assert_serde_snapshot_law(schema, label, &snapshot)` (serde output validates, reads back, re-serialises to identical JSON). Wired into en1999 (both example snapshots, also the canonical writer output), en1995 and din16798.
- Exact round trip: `serde_json` lacked `float_roundtrip` (0.12 read back as 0.12000000000000001, so documents drift per save). Enabled in `✏️s/🔌️plugins/📕️norm/Cargo.toml` and root `Cargo.toml` (`workspace.dependencies.serde_json`). The law uses exact JSON equality.
- Zero/derived leaves are no longer hand-listed in the leaf-influence test. The schema declares them with one vendor annotation family `x-semio-active` (rules `{leaves, when: {field: {in|notIn, ignoreSuffix}}}`): `AluminiumSection` geometry per section kind (`chs`: outerDiameter, webThickness; others and `free`: height, width, flangeThickness, webThickness) and `FatigueDetail.deltaSigmaC` active only when `detailCategory` is not an Annex J id (the mutation `ChangeFatigueDeltaSigmaC` stays legitimate; no `readOnly`). `every_editable_leaf_influences_a_check` reads it from the schema; `schema_tabulated_detail_categories_match_annex_j` pins the id list to `part_1_3::category_ids()`.
- Vocabulary: `x-semio-active` registered with its meta-schema in `🧰️framework/🔨️modules/🧬️schema/🧬️vendor-annotation-vocabulary/🔣️.json` (the single list the strict Ajv oracle reads). Twin projection needed no change: TS check en1999 identical=224 drift=0 (`--strict`), Rust identical=223.
- Oracle workarounds removed: lowercasing of `annex` in din16798 and en1993 `snapshot-schema/🐍️.py`, and in the en1997 `example_snapshot_validates_against_json_schema` test.
- test-support forwarding gap: schema crates of 13 families (all but din18599, en1996 which already had it) now list their `-diff`, `-model`, `-mutations` crates with `features = ["test-support"]` in `[dev-dependencies]`, so a bare `repo cargo -- test -p <schema crate>` compiles. Verified bare for `semio-s-artifact-norm-en1999-schema` (exit 0, 0 tests; the compliance tests live in `semio-s-artifact-norm-en1999`).

## Changed files

- `✏️s/🔌️plugins/📕️norm/Cargo.toml`, `Cargo.toml` (serde_json `float_roundtrip`)
- `⚖️compliance/🧪️tests/🔬️python-oracle/🦀️.rs` (law)
- en1999: `🧬️schema/📸️snapshot/🔣️.json`, `🧬️schema/📸️snapshot/🦀️.rs`, `🧬️schema/🧪️tests/⚖️compliance/🧬️facet-and-asset-parity/🦀️.rs`, `🧬️schema/🧪️tests/⚖️compliance/🔎️leaf-influence-and-material-laws/🦀️.rs`
- en1995 `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`, din16798 same path (law wired)
- din16798 and en1993 `🔮️oracles/🧬️snapshot-schema/🐍️.py`; en1997 `🧬️schema/🧪️tests/⚖️compliance/🔮️oracle-and-remedy-laws/🦀️.rs`
- `🧬️schema/📦️packages/🦀️rust/Cargo.toml` of 13 families (dev-dependencies)
- framework `🧬️vendor-annotation-vocabulary/🔣️.json`
- ticket scripts: `serde-law-chain.sh`, `twins-check-lang.ts`

Touched in others' areas (minimal): en1993 oracle `.py`, en1992/en1998/en1990/... schema `Cargo.toml` dev-dependencies (only the added lines).

## Test lines

- en1999 (`-p semio-s-artifact-norm-en1999 --lib checks_compliance`, bare): `test result: ok. 106 passed; 0 failed`; examples: `en1999 exit 0: 4 passed, 0 failed`.
- din16798 (`--lib`): `47 passed; 4 failed`. `jsonschema_validates_compliant_and_noncompliant_snapshots` (with the serde law) passes. The 4 failures (`unoccupiedVentilation` read-only/field meta, `usageType` perturbation, hygienic minimum) are din16798 content work, owned by `din16798-content-tests-2026-11-24.md`. Examples: `din16798 exit 0: 3 passed, 0 failed`.
- en1995: not run. The crate exceeds its source boundary before compilation (5 of 32 files, 294013 of 262144 bytes); pending the split in `en1995-source-boundary-2026-11-24.md`. The law is wired in; rerun after the split.
- en1997: examples `exit 0: 3 passed, 0 failed`. The schema test does not compile for reasons outside this change (`semio_s_plugin_norm_mechanics` unresolved in `catalogue-laws`, `OriginalSnapshotRefusal.kind` in the sqlite test). My edit (annex lowercasing removed) is therefore unverified; rerun once those are fixed.
- en1993: schema test fails to compile in the in-flight split (`ChangeFastenedJointKind: serde::Serialize` not satisfied in `🧬️mutations`); examples `exit 1` (build). Oracle script edit: `py_compile` ok, unverified at run time.
- Norm JS native (`run-norm-js-native.sh`): `12057 pass, 1 fail`. The one failure, `the complete fifteen-family rule declarations and companion arguments have closed shapes`, expects 48 din16798 bindings and finds 50 (the new `unoccupiedVentilation` bindings of the din16798 content work); the fixture count belongs to that work. Without the vocabulary entry the run dropped to 136 pass / 2 fail (`unknown keyword "x-semio-active"`).

## Open

- Rerun en1995, en1997, en1993 after their blockers clear; remaining families (en1990, en1991, en1992, en1994, en1996, en1998, din4108, din18599, iso16757, vdi3805) were not run here beyond the earlier JS suite; their dev-dependency edits are unbuilt.
- Other families' serde snapshot tests (en1992, en1997, din4108, iso16757) can adopt `assert_serde_snapshot_law`; they already validate serde output against the schema but do not check the round trip.
