# Nested Norm Test Prescan (2026-11-27)

Static scan of en1991, en1996, en1994, en1992, din16798, din18599, en1993, iso16757, vdi3805, en1997, en1999, en1990 and `🏭️bridge`. Scripts (scratchpad only): member-crate walk with `[lib] path` source roots, `#[path]` liveness, `[[test]]` targets.

## Fixed
- en1990: `🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust/Cargo.toml` gained `serde_json = { workspace = true }` under `[dev-dependencies]` (class 1). Used by `📇️catalogue/🗂️cases/🧪️tests/🔬️unit/🦀️.rs:53`. `en1990/Cargo.lock`: added `"serde_json"` to the `semio-s-artifact-norm-en1990-checks` dependency list so `--locked` still holds.

## Checked, clean
1. Missing dev-dependencies: every live test module and `[[test]]` target in all 12 workspaces resolves each workspace-declared crate it names. Only the en1990 case above was missing.
2. Self-crate names in lib unit tests: no hits. The `✏️editor/🧪️tests/🔬️unit|📚️assets` modules name `semio_s_artifact_norm_*_editor` externally, which is correct, because they are modules of the `integration` `[[test]]` target (`pub use <editor>::*`).
3. String literals into typed fields: struct and enum-variant field types were collected from all sources and compared with `field: "..".into()/.to_string()` in test code. Zero mismatches in all 12 (the typed enums are en1995-specific; din16798 `new_comfort_model` is still `String`).
4. Unwrapped `Result`: `HeadlessWorkspace::open_folder` and `HeadlessWorkspace` are not used in any scoped workspace.
5. Cargo.lock: registry packages in all 15 artifact locks and in `🏭️bridge/Cargo.lock` are a subset of the versions in `📕️norm/Cargo.lock`. No drift.
6. Struct-literal drift (unknown or missing fields vs. definition, in test code): only false positives (name collisions with paths, strings, patterns). None real.
7. `🏭️bridge`: a bin-only crate with no tests; nothing to fix.

## Observations, not changed
- Feature-unification risk is not provable statically: some crates use `serde_json` against model/diff/mutations crates without `test-support` in their dev-dependencies (en1996, en1994, en1992, din16798, din18599, iso16757, vdi3805, en1997, en1999, en1990 `-io` and `-scene` crates). This only breaks when the crate is built alone, not in a workspace-wide `cargo test`.
- en1990 mutations/editor/io were edited concurrently by another agent (cold-bounds); not touched.

## Follow-up: the two en1996 "orphan" test files
- `🧪️tests/📇️en1996-catalogue/🦀️.rs` is not dead. It is a repo-test-host Gherkin adapter driven by the sibling `🥒️.feature` and `🐍️.py`, the same layout as en1995, en1997 and en1990 (`🧪️tests/📇️catalogue-*/🦀️.rs`), which no `Cargo.toml` references either. Its API use (`Case::new`, `with_n`, `with_v`, `with_s`, `evaluate_outcomes`, and the `Binding.example` closure of the steps layer) matches the current signatures. No change.
- `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` was truly dead, and is now wired the way en1992, en1997 and en1999 wire theirs: `#[cfg(test)] #[path] mod checks_compliance;` in the artifact crate root `en1996/🦀️.rs`, before `diff_unit`.
  - Dev-dependencies added to `en1996/📦️packages/🦀️rust/Cargo.toml`: `semio-framework-test-fixtures` (path) and `semio-s-plugin-norm-app-surface` (workspace). `en1996/Cargo.lock` entry for `semio-s-artifact-norm-en1996` updated to match.
  - Path drift fixed: its three `fixture_path!` calls used ups relative to the old crate manifest (`🖼️assets`, `🧬️mutations`, schema TS twins). They now go through a local `family_any_dir()` (`fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any")`, the en1992 form) and `.join` the old targets.
  - Every API name the file uses was checked against today's definitions: `evaluate_building`, `fk_factors`, `f_k_pa`, `phi_m`, `phi_s`, `fire_min_thickness_m`, `design_effects`, `AnnexParams::for_document/gamma_m/gamma_m_ndp`, the snapshot example constructors, `ConcentratedLoad` fields, `Remedy` and `CheckResult` fields, `ReliabilityClass::k_fi`, and `ArtifactPack::decode_pack`. Behavioural assertions (numbers, perturbation) cannot be proved statically, so the en1996 run decides.

## Follow-up: framework owner-API drift (sqlite snapshot tests, bare-trait calls)
Pattern source: en1996's migrated `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` and `original_owners` in `🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs`.
- Migrated mechanically with a paren-matching script (scratchpad only) in the sqlite snapshot tests:
  - `with_decoding(|o| X::decode_sqlite_snapshot_native(p, c, o))` became `decode_native::<X>(p, c)`.
  - `with_encoding(|o| v.encode_sqlite_snapshot_native(e, c, o))` became `encode_native(&v, e, c)`.
  - `with_decoding(|o| (p.export)(schema, dialect, payload, c, o))` became `export_sqlite(p, schema, dialect, payload, c)`.
  - `with_encoding(|o| (p.import)(schema, dialect, &mut Some(db), e, c, o))` became `import_sqlite(p, schema, dialect, db, e, c)`.
  - Counts: en1991 18 + 2, en1994 18 + 2, en1992 13 + 4, en1993 16 + 4, en1990 20.
  - Where `p` is an owned `ArtifactSqliteSnapshotCodec` (`bare_artifact_codec(..).snapshot_sqlite.unwrap()`, en1992 and en1993) it is passed as `&p`. No receiver of `encode_native` is a reference parameter, so there is no `&&T`.
- Already current, no change: din16798, din18599, iso16757, vdi3805, en1997, en1999, en1995, din4108, en1998, en1996 (en1996 by the other agent).
- Still on the old form but correct: `store::decode_sqlite_snapshot_record_native(.., native_owner, &mut output)` under `original_owners::with_decoding` (9 sites). Its signature is unchanged and the tests read `.result`. `store::io::testing::with_snapshot_file_decoding/encoding` plus `io_export/import_sqlite_snapshot` also match today's signatures. `original_owners::io_run` (en1997, async) matches.
- Bare-trait calls on generic protocol traits fixed (the traits are generic over the snapshot; there are no `MutationDiff::absorb` bare calls in these workspaces):
  - en1990 `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `<En1990Mutation as protocol::SemanticMutation<En1990Snapshot>>::semantics`.
  - en1997 `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: the same with `En1997`.
  - en1990 `🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `<En1990Mutation as protocol::OpText>::print_op`.
  - The `ColdMutation::diff(&m, &base)` calls infer `P` from `base`; I left them.
- `protocol_laws::assert_mutation_*_law` call sites (about 60) match the current argument order and types.
