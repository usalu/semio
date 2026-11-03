# Compile review H, lane 94A: EN 1994 model, snapshot/diff twins, io, spec-trace, schema tests, mutation umbrella and group roots

Pure static review (no cargo, rustc or nx). `B` = `🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any`. Scripts and notes: `🗑️generated/compile-review-h/94A/`.

## Scope read (every file read in full unless listed under limits)
- schema crate root `B/🧬️schema/🦀️.rs` and its Cargo.toml (descriptor, conversions, `include_str!`).
- model crate `B/🧬️schema/📸️snapshot/🦀️.rs` (all 717 lines) and Cargo.toml; the four snapshot twin crates (root + Cargo).
- model-mounted io files: `🚪️io/🪶️sqlite/🦀️.rs`, `🪶️sqlite/📸️snapshot/🦀️.rs` (encode, decode, 12 tables, bounds), `🛂️admission/🦀️.rs`, `🗄️.sql` (all 12 tables, every REAL/INTEGER/TEXT column index against the `FloatColumn` lists, cell counts and decoder column numbers), `🪞️records/🦀️.rs` (`impl_norm_artifact_record!` expansion checked against the macro), the sqlite tests file and its fixtures (`🎛️semantic.json` widths 67/54/..., schemaBytes 10804 recomputed from the DDL, row counts 12 and 1).
- diff crate: root (macros `row_key!`, `scalar_patch!`, `nested_patch!` expanded by hand against `protocol::list_delta`, `MutationApplyError::under`, `ApplyCapability: Copy`), the four row files (every patched name and type checked against the model structs by script: no gaps), `apply-and-inverse` (trait signatures against `MutationDiff`/`ColdMutationDiff`/`DiffAlgebra`/`ColdDiffAlgebra`), the unit test, Cargo.toml, four twin crates.
- io crate: root `🦀️.rs`, Cargo.toml, `📝️text`, `💾️binary` (module roots, snapshot, diff, inferences, mutations files), all io unit tests.
- mutation umbrella `🧬️mutations/🦀️.rs` (119 variants = 119 `KINDS` = 119 distinct imports, kebab names and struct names verified by script against the leaf files), `🧭️edit-rules`, `🧰️testkit` (payload literal fields of 113 cases checked against the leaf structs), `🧪️tests/{unit,kinds-catalog,vectors}`, umbrella Cargo.toml, all 22 group roots and Cargo.toml (mounts exist, dependency heads covered, features valid).
- `🧭️spec-trace/🦀️.rs` (54 ids, `SpecTrace::new` arity 5, `SpecTraceTable` fields), `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (every call against the current checks signatures).
- artifact root `Cargo.toml` (42 members all exist, each has a workspace dependency, no stray manifests), `📦️packages/🦀️rust/Cargo.toml` and `📜️script.ts` package list (42 = 42), artifact `🦀️.rs` umbrella root.
- Whole-artifact scripts: struct-literal completeness for all 11 model structs over 534 files (after the fix: none missing or extra), `include_str!`/`#[path]` resolution over the lane (73 refs, none missing), feature/dependency validity of all 42 manifests (`feats.py`).

## Fixed
| file | error class | fix |
|---|---|---|
| `B/🧬️schema/📸️snapshot/🦀️.rs` `CompositeColumn::default_example` | E0063 (missing `introduced_force_n`, `section_height_m`, `section_width_m`, `section_tw_m`, `section_tf_m`, `rebar_cover_m`) | set the six fields to 0.0 |
| `B/🚪️io/🦀️.rs` last line | E0432 (`pub use mutation_bridge::*;` after the module moved to the mutations crate) | `pub use semio_s_artifact_norm_en1994_mutations::mutation_bridge::*;` |
| `B/🚪️io/📝️text/🔺️diff/🦀️.rs`, `💾️binary/🔺️diff/🦀️.rs`, `🚪️io/📝️text/🦀️.rs`, `💾️binary/🦀️.rs`, `🧬️schema/🔺️diff/🦀️.rs`, diff `Cargo.toml` | E0117 orphan (`diff_text!`/`diff_binary!` implement foreign `DiffText`/`DiffBinary` for the foreign `En1994Diff` inside the io crate) | the diff crate now mounts both files (`pub mod text; pub mod binary;`), the macros take `crate::En1994Diff`, io re-exports `pub use ..._diff::text as diff;` / `::binary as diff;`, diff crate gains `semio-framework-diagnostic` (named by the expansion) |
| `B/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs` | runtime (not rustc): borrowed-native admission used stale record slot ids: placements moved to slot 6 so fire/insulation/fatigue are 7/8/9, beams gained 10 leaves (actions slot 27, 24 cells), columns gained 6 leaves (actions slot 19, 19 cells) | ids and cell lists corrected so they equal the typed writer and the DDL (26 / 19 / 10 / 7 cells) |
| `B/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs` | logic (unreachable second `_` arm; nested `RemoveItem` never reached `resolve_with_id_removals`, `ID_REMOVALS` dead) | removed the first `_` arm and the duplicated docstring |
| `B/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs` | E0599 (`mutation.diff`, `Diff::inverse` need `ColdMutation`/`ColdDiffAlgebra` in scope) | imports `ColdDiffAlgebra as _, ColdMutation as _` |
| `B/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` | E0599/E0576 (`protocol::Mutation::diff` no longer exists) | `ColdMutation::diff(&operation, &base)` |
| `B/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` `set_every_word` | test gap | also sets the seven new beam leaves (cracked fraction, stud cross spacing, edge distance, transverse top bars, end plate w/h, end zone) |
| `📦️packages/🦀️rust/Cargo.toml` (umbrella dev-deps) | E0277 guard | compliance features `["test-support", "compliance-testing"]` (see open item 4) |

## Open (routed to `📌️important/requests.md`, "compile-review H (94A)")
1. Same dangling `pub use mutation_bridge::*;` in every other artifact's io root (en1990, en1991 next).
2. Same orphan `diff_text!`/`diff_binary!` in every other artifact's io crate (fix pattern in the request).
3. Same stale `Mutation as _` diff-unit-test imports in 9 other artifacts.
4. Shared compliance: `AnnexChoice` serde derive is cfg'd on `compliance-testing`, but `test-support` features only forward `compliance/test-support`.
5. en1994 testkit lacks the 6 placement kinds; orphan test file `🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` mounted nowhere.
6. `edit-rules` regeneration (if a generator emits it, it must carry the arm fix).

## Checked and found consistent (not guessed)
- Orphan rule for all remaining impls of the lane: `impl_norm_artifact_record!` (local snapshot), `ArtifactSqliteSnapshotHost`/`ArtifactSqliteSnapshot` (local snapshot), `OpText`/`OpBinary` (local mutation enum, mounted in the mutations crate), `RowPatch`/`KeyOf` (local patch/marker), `MutationDiff` family (local diff), `From` pairs between `CharacteristicAction` and `SlabAction`.
- All `use`/dependency heads of the moved files against the host crates (model: pack, store alias, contract `sqlite_native`, value-native-owned, dsl-record, compliance; mutations: pack-json, diagnostic, all 21 group crates; io: plugin, artifact-reference, diagnostic).
- `dsl::Mutations` expansion paths (`os-command`, `os-store-one-item`, `schema-state`, `ui-locale`, `value`) all in the mutations Cargo.toml.
- Feature declarations: every `dep/feature` and `workspace = true` key resolves; `test-support` spelled consistently; serde derives under `any(test, feature = "test-support")` match the declared features.
- `#[dsl(unit = ...)]` symbols in the model (`m`, `m2`, `m3`, `m4`, `Pa`, `N`, `m2/m`) all exist in the unit table (a missing one would panic at spec build).
- `concat!(include_str!(..))` in `DIFF_LEAVES` is an eagerly expanded builtin and accepted.

## Not read / limits
- The 119 mutation leaf folders (another lane); only their struct names and payload fields were compared mechanically.
- `🧬️schema/💡️inferences` and `⚖️checks` bodies (other lane); the compliance test here was matched to their signatures only.
- Generated non-Rust twins (`.ts`, `.proto`, `.graphql`, `.json`, `.sql` beyond the column mapping) were not reviewed.
- Runtime fixtures not recomputed: the `🔢️binary64.json`, `🛂️choices.json`, `🎛️control.json` files; `🎛️semantic.json` was recomputed from the DDL and the showcase counts (12 rows, 71 bytes empty case not rechecked).
- The value `39` passed as the borrowed column extent in `n::admit` is exercised by the tests as a fixed contract; I did not change it.
- Files saved by other agents after about 02:17 (the model gained `catalogue_root_radius_m`) were seen only once.
