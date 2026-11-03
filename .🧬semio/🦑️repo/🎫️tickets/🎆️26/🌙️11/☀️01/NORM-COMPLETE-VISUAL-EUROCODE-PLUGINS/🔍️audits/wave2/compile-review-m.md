# Compile review M (wave 2, SCHEMA-CODEGEN v2 Rust output)

No cargo/rustc, no build, no git write. All scripts niced; logs and scripts under `🗑️generated/compile-review-m/`. en1994 is in the graph-plan freeze: nothing in `🧩️en1994` was touched, no en1994 twin regenerated.

## Method
- Struct-level comparison working tree vs `git show HEAD:` for every changed `🦀️.rs` (derives, `cfg_attr` derives, struct/field `pub`, field order, field types, struct/field attributes). Result: no derive lost on any harvested record/root/leaf (annotate only keeps a record that reads back equal); no field-order change; no `pub` lost. The only derive removals are the NO-DEFAULTS `Default` removals and the `ViewCommand` enums (not generator output). Type changes are the TYPED-ENUMS conversions of wave 1. The 1063 structs new vs HEAD (243 leaves, 81+ records) carry the same derive sets as their hand-written siblings; every `CanonicalJsonTree` struct has `#[canonical_json(owner..)]`, every `MutationLeaf` has `#[mutation_leaf(contract..)]`.
- Attribute forms: all `#[dsl(..)]` keys (block, extension, id, key, keyword, layout, retire_with, statements, table, unit) and `#[state(artifact)]` are accepted by the derive parser; every `dsl(unit)` symbol (m, N, Pa, m2, N/m, N*m, m2/m, kg/m3, ...) is in the 50-entry `UNITS` table; all unit fields are `f64` (one `Option<f64>` pre-existing).
- Cargo/features (script over every norm crate with its `#[path]`/`mod` closure): `feature = "test-support"` used but undeclared, serde not available, `test-support` not forwarded to workspace deps that have it, fully qualified `semio_*::` paths without a dependency.

## Fixed (certain)
1. en1997 `mut-pile-dynamic`, `mut-lab-test-consolidation`: leaves gate serde on `test-support` but the crates declared no such feature (cfg never true -> aggregate serde derive E0277). Feature added (`dep:serde`, diff, model); `mut-pile` and `mut-lab-test` forward it.
2. en1991 (8 `mut-*` crates) and en1995 `mutations`: `test-support` not forwarded to `mut-gravity-floors`, `mut-deck-prestress`, `mut-deck-geometry`. Forwards added.
3. en1998 `diff`, `mut-buildings-annex-building`, `mutations-demo` name `semio_s_plugin_norm_compliance::ReliabilityClass` without the dependency (E0433). Dependency added, `test-support` forwarded in the first two.
4. 22 hand-written files (en1995 `diff` + 19 deck leaves + insert/remove-deck, en1998 `diff`; 48 attributes) still had `cfg_attr(test, derive(serde..)/serde(..))`: leaf crates compiled as dependencies of an aggregator under `test-support` lacked serde. Now `any(test, feature = "test-support")` (the generator's leaf template).
5. en1999 schema: root `reliabilityClass` carried `x-semio-rust.type = "AnnexChoice"` (copy error: generation would have typed the field `AnnexChoice`, E0308); the snapshot property had no type (generator problem, `check --strict` failing). Both now `semio_s_plugin_norm_compliance::ReliabilityClass`; en1999 twins regenerated; the unused `use ...ReliabilityClass` of the en1999 `change-reliability-class` leaf removed. en1999 `check --strict`: 423 identical, 0 drift, 0 problems.
6. en1998 twins regenerated (3 written, aggregate leaf twins): `check --strict` 582 identical, 0 drift.
7. en1996 twins regenerated on coordinator request (CHOICE-TOKENS hand patches): `generate` wrote 0, `check --strict` 302 identical, 0 drift, 0 problems (the hand patches already equal the generated form).

## Open / routed (`📌️important/requests.md`)
- en1994 (freeze): same two `x-semio-rust.type` annotations missing on `reliabilityClass` (3 generator problems at 03:50: En1994Artifact, En1994Snapshot; the leaf and the snapshot/diff Rust drift). Steps in requests.
- Judgement for shared: `ReliabilityClass` serde is gated on `compliance-testing`, not `test-support`.
- Global `check --strict` before my regenerations: 15 drift (en1994 6+, en1998 and en1999 twins, now clean) + the en1994 problems; remaining after: en1994 only.
- Not harmful/false positives: `store::semio_format`, strings naming `semio_s_artifact_norm_en1995_kernel`, macro bodies in app-surface live-lanes. The struct-literal completeness scan could not separate same-named structs of different modules and was dropped (L's scan stands).
- Compliance dev-dependency `serde` (cfg(test) without `compliance-testing`) left as in HEAD (edit reverted: compliance is an en1994 dependency).

"- ready for rebuild: compile-review M" appended to `build-en1991|1995|1997|1998|1999/latest.md`.
