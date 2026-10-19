# Result HDR4B (executor prefix `hdr4b`): hdr4 compatibility re-exports removed, consumers repointed

Sonnet 5.5, foreground, no sub-agents, no cargo/nx/`bun repo`, no modifying git command. `C` = this ticket. Freeze honoured: no `package.json`/`bun.lock` edit, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`. Manifest changes are Cargo.toml only. Static verification only; nothing compiled.

## 1. What was removed

| area | removed re-export (hdr4) | consumers repointed to |
|---|---|---|
| table mutations aggregate | `pub use semio_s_artifact_stdio_semio_table_mut_family::schema::mutations::{create_column .. reorder_rows}` in the package root `🧬️schema/🧬️mutations/📦️packages/🦀️rust/🦀️.rs` | `semio_s_artifact_stdio_semio_table_mut_family::schema::mutations::<leaf>` |
| audio mutations aggregate | `pub use semio_s_artifact_stdio_semio_audio_mut_family::schema::mutations::{insert_channel .. set_tag_value}` in `🧬️mutations/🦀️.rs` (now a private `use`, the enum needs the payload types) | `semio_s_artifact_stdio_semio_audio_mut_family::schema::mutations::<leaf>` |
| pptx-base-model | `pub use semio_s_artifact_stdio_pptx_base_vocabulary::{refusal, vocabulary}` in `🧬️schema/🦀️.rs` | `semio_s_artifact_stdio_pptx_base_vocabulary::{refusal, vocabulary}` (explicit name lists, no glob) |
| forms-commands | `pub use semio_s_artifact_forms_forms_commands_try::editor::forms::commands::{next_step, previous_step, reset_try, set_try_value, set_try_value_step, set_try_values, submit}` in the add-question mirror | `semio_s_artifact_forms_forms_commands_try::editor::forms::commands::*` |

Table/audio consumers (all one-line import changes):
* `semio-s-artifact-stdio-semio-editor` and `-semio-schema` (files in `✉️base`): `set_sample_rate` (4 `#[cfg(test)]`-only imports: binary io, text io, schema mutations, unit tests) -> audio family; `remove_row::RemoveRow` (non-test demo cases) and `insert_row` (unit test) -> table family. Manifests: `table-mut-family` in `[dependencies]`, `audio-mut-family` in `[dev-dependencies]` (only test-cfg code names it).
* `semio-s-artifact-stdio-semio-audio` (`🚪️io/🦀️.rs`): `insert_channel`, `set_format`, `set_sample_rate` (`#[cfg(test)]`) -> audio family; `audio-mut-family` added to its `[dev-dependencies]`.
* Table aggregate itself: `use super::<leaf>` x8 in the component, 16 `crate::schema::mutations::<leaf>` in the text io codec, 4 leaf test files with the full aggregate path (`rename-column`, `insert-row`, `reorder-columns`, `reorder-rows`) -> table family (a normal dependency of the aggregate package, so its integration targets see it).
* Audio aggregate itself: the text io codec relied on `use crate::schema::mutations::*` for the eight leaf modules; it now has one explicit `use semio_s_artifact_stdio_semio_audio_mut_family::schema::mutations::{..}`.

pptx consumers: model (`xml-address`, `construction/minimal`, `inferences/presentation`: glob imports replaced by explicit name lists from the vocabulary crate), `semio-s-artifact-stdio-pptx-base` (`construction`, `import/deserializers`, `export/serializers`), `semio-s-artifact-stdio-pptx-base-mutations` (`mutations/xml-address`), `-pptx-strict-mutations` and `-pptx-transitional` (the two `🛡️conformance` files), plus the unmounted `🧬️schema/🧪️tests/🔬️unit/🦀️.rs` (already stale: it names `semio_s_artifact_stdio_pptx::standards`, which the root crate never exported). `semio-s-artifact-stdio-pptx-base-vocabulary` added to the four consuming manifests (`pptx-base`, `pptx-base-mutations`, `pptx-strict-mutations`, `pptx-transitional`).

forms consumers: editor crate `semio-s-artifact-forms-forms-forms` (`editor/🦀️.rs` import list split, `🧪️tests/🔬️unit` two import lines, `✅️submit` tests, `🎯️set-try-value` chunk-value-vectors test; the mirror blocks `commands::submit` and `commands::set_try_value` keep only their test mounts), config-tests crate `...-forms-config-tests` (two import lines in the window test). `semio-s-artifact-forms-forms-commands-try` added to both manifests (the root crate is unchanged).

## 2. What was kept

* The `semio-s-artifact-forms-forms` root facade composition that this report first described was deleted on the coordinator's instruction (no consumers). The root `🦀️.rs` and its `Cargo.toml` are back to their HEAD content (`pub use ..commands::editor::forms::commands;` is the original alias, no `commands-try` dependency); the try family is reachable from `semio-s-artifact-forms-forms-commands-try` only.
* Not touched (mirror scaffolding, not hdr4 compatibility): the `crate::` namespace mirrors (`🪞️mirror/🦀️.rs`, the try crate's own `crate::` copy, svg `pub use ..svg_diff..::diff` lines that already existed and only changed target).

## 3. Verification (static, exact)

* Leftover scans (`git grep -n`, tracked tree): no `semio_s_artifact_stdio_semio_{table,audio}_mutations::schema::mutations::<leaf>` anywhere; no `pptx*::schema::{vocabulary,refusal}` / `schema::{..vocabulary..}` anywhere; no external user of the forms try modules outside the forms workspace.
* Oracle `F/hdr4-tools/pos` (copied to `C/hdr4b-tools/pos`, `POS_PREFIX=hdr4b`, nightly-2026-07-20 `--emit=metadata`, lib and `--test`, baseline git `4dab415ed90`), extended families so that the consumers are real crates and not stubs:
  * `hdr4b-pptx` (model, vocabulary, base, base-mutations, strict-mutations, strict, transitional): lib NEW 0, test NEW 1 (stub noise); privacy/resolve/type/other 0.
  * `hdr4b-tabaud` (table/audio model, base, value-model, both aggregates, both families, `semio-editor`, `semio-schema`, `semio-audio`, `semio-table`): lib NEW 4, test NEW 4; all four are the known `E0220` stub-layer rows of the two family crates (`Diff` associated type) that hdr4 already reported; privacy/resolve/type/other 0.
  * `hdr4b-forms` (all 17 members of the forms workspace): resolve 0, privacy 0. Eight non-stub test-cfg type rows (`E0599`/`E0610` on `OwnedFormsApp`, `✏️editor/🧪️tests/🧪️field-transactions`) exist identically when the removed re-export is restored by `--mutate` (control run `formsctl`): they are independent of this change (stubbed framework plugin types in the wider family, which hdr4's narrower family did not include).
  * Negative control for pptx: reverting one consumer path by `--mutate` produced `E0432 unresolved import 'refusal'` in the pptx-base serializers (the oracle does see these breaks; unresolved imports are classified `stub=True` and hidden from the summary counts, so each run's `new-*.txt` was read, not only the summary).
* Manifests: 10 edited/added `Cargo.toml` parse with `Bun.TOML`, every `path =` dependency exists; the `workspace = true` entries resolve against `workspace.dependencies` rows that already exist (`semio-s-artifact-stdio-semio-{table,audio}-mut-family` in the `🧿️semio` workspace, `semio-s-artifact-stdio-pptx-base-vocabulary` in the `📽️pptx` workspace).

## 4. Crates the coordinator must compile
`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests` after the offline lock refresh of the `🧿️semio`, `📽️pptx`, `📋️forms` workspaces (new edges between existing members change their `Cargo.lock`), and the hub lock (both aggregates and the editor/schema crates are path members of the hub workspace):
* table/audio: `semio-s-artifact-stdio-semio-table-mut-family`, `semio-s-artifact-stdio-semio-audio-mut-family`, `semio-s-artifact-stdio-semio-table-mutations`, `semio-s-artifact-stdio-semio-audio-mutations`, `semio-s-artifact-stdio-semio-editor`, `semio-s-artifact-stdio-semio-schema`, `semio-s-artifact-stdio-semio-audio`, `semio-s-artifact-stdio-semio-table`; tests: `column_mutations`, `row_mutations`, `codec_laws` of the table aggregate.
* pptx: `semio-s-artifact-stdio-pptx-base-vocabulary`, `-pptx-base-model`, `-pptx-base`, `-pptx-base-mutations`, `-pptx-strict-mutations`, `-pptx-transitional`, `-pptx-strict`, `semio-s-artifact-stdio-pptx`.
* forms: `semio-s-artifact-forms-forms-commands-try`, `-forms-commands`, `-forms-forms` (editor), `-forms-config-tests`, Tests: the editor crate lib tests (`submit::tests`, `set_try_value::chunk_value_vectors`, `field_transactions`, unit) and the config-tests window test.

## 5. Open risks
1. Nothing compiled. Most likely compile failures: (a) forms editor mirror: `commands::submit` / `commands::set_try_value` are now test-only modules, so any remaining path `crate::editor::forms::commands::submit::X` would be unresolved (oracle: none); (b) audio text io: glob plus explicit import of the same names is fine, but if a later `crate::schema::mutations::*` glob grows a name collision it would be ambiguous (none today); (c) `use ..::transform;` in pptx snapshot is used by serde `with = "transform::.."` strings: it needs the private module path to resolve in scope (it did before as `mod transform;`).
2. `inverse_laws` integration target of the audio aggregate (`🧪️tests/🔬️inverses`) is broken at HEAD independent of this work: its leaf files import `schema::mutations::tests::*` but `mod tests;` is `#[cfg(test)]` and private. Left untouched (outside the brief); fix by moving the shared helpers to a `testkit`-gated public module.
3. `🔒️dependencies.json` (generated ledger) still lacks the hdr4 and hdr4b edges; regenerate with the dependency gate.

## 6. Files
Edited (rust): table aggregate root, `🧬️mutations/🦀️.rs`, text io, four leaf test files; audio `🧬️mutations/🦀️.rs`, text io, audio facade `🚪️io/🦀️.rs`; `✉️base` binary io, text io, `🧬️mutations/🦀️.rs`, unit tests; pptx model `🧬️schema/🦀️.rs`, `construction/minimal`, `inferences/presentation`, `xml-address`, `mutations/xml-address`, `construction/🦀️.rs`, `import/deserializers`, `export/serializers`, strict and transitional `conformance`, `🧬️schema/🧪️tests/🔬️unit`; forms add-question mirror, editor mirror, `editor/🦀️.rs`, `editor/🧪️tests/🔬️unit`, `✅️submit/🧪️tests`, `🎯️set-try-value/🧪️tests/🔬️chunk-value-vectors`, config-tests window test.
Edited (Cargo.toml): `semio-editor`, `semio-schema`, `semio-audio`, `pptx-base`, `pptx-base-mutations`, `pptx-strict-mutations`, `pptx-transitional`, forms editor, forms config-tests.
Created: `C/hdr4b-tools/pos/` (oracle copy writing to `C/🗑️generated/hdr4b-x`) with families `hdr4b-pptx.json`, `hdr4b-tabaud.json`, `hdr4b-forms.json`. Generated logs and the sandbox `/Users/ueli/.cache/hdr4/hdr4b-work` were deleted.
