# XLSX descriptor and gate follow-ups

## 3. Localized `snapshot_details_split_layout` (done, compile-checked)

Contract: `snapshot_details_split_layout(main_window_kind_id)` no longer takes a title. Both stacks carry `title: None`, so the dock falls back to the localized window-kind label from the catalog. The details window kind is declared `LocalizedLabel::native("Details", "Details")`. All 108 call sites across every artifact (editor roots, edit-mode layouts, ifc/inspector, xlsx, mp4, stl, svg, step, ...) were rewritten by a single regex pass; no caller passes a title any more (no English-only "Workbook", "Strict workbook", "Details", "Main", "Table", ...). The contract unit test `split_layout_keeps_canvas_and_details_visible` now asserts `title.is_none()` on every window and the details label in en and de.

Verification: the library of `semio-s-artifact-stdio-contract-editing-details` compiled with the change. Its `lib test` target does not compile because of a foreign error (`semio_plugin::artifact_app_laws` missing in `union-source-and-window-laws`, another agent's framework refactor), so the unit tests could not run.

## Coordinator correction

`s.stdio.xlsx.resource.external-references` and `s.stdio.xlsx.conformance-suite.calamine-oracle` are `unimplemented` again. Re-counted from the current definition: codecs 1 implemented, mutations 27 implemented (`executable_registration: false`), inferences 1 implemented, profile/source dialect implemented. Capability counts (declared mutations 30, inferences 68; implemented mutations 27, inferences 1) depend only on mutations and inferences, so the hub catalog expectations stay as edited. Resources and conformance suites are not counted. The note in `xlsx-i18n-fixes.md` (item 6) is stale on those two entries.

## 1 and 2. Not completed: blocked by other agents' in-flight work (observed, not guessed)

Nothing generated was hand-edited. Observed blockers, in order:

- `repo run @semio-tech/stdio-artifact-contract-editing-details-rs:test` and `run-many @semio-tech/stdio-js:stdio-quick` fail in the graph service: `Workspace import is not resolved by current package metadata: @semio-tech/stdio-mp3 from ✏️s/🧑‍💻dev/🗄️stdio/🟦️.ts`. So the `stdio-ledger` gate (stdio-js `stdio-quick`, `stdio-mutation-law`, `stdio-inference`) never started.
- Cargo through `repo cargo`: first `plugin-app-impl` referenced missing module directories, then `semio-os-store-document` failed (`From<Vec<u8>> for ArtifactGenesisPack`, `admit_verified_physical_pack`), then the repo tool itself failed (`hostUnderPressureV1` export missing in host-capacity), all from concurrent edits. The details crate lib finally compiled.
- Hub catalog test (`full_catalog_preserves_definition_codec_and_ledger_counts`, feature `full-artifact-catalog`): after about 24 min the hub workspace lock refresh failed with `cyclic package dependency: semio-s-artifact-stdio-pdf-tests-2 depends on itself` (pdf `vt` mutations tests crate; not xlsx). The catalog counts are therefore still unobserved.
- xlsx base lib test: `semio-plugin-app-builder` (lib) does not compile (2 errors, framework work in progress).
- Natural-file lifecycle test (`🧰️framework/.../🔌️plugin/🧪️tests/📄️natural-file-lifecycle/`): not reachable, same plugin crate state.
- Descriptor regeneration: `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🔣️.json` (and `🛂️.descriptor.semio`) is produced by `@semio-tech/os-plugin-describe-rs:describe` on the compiled `semio-hub-stdio-component` (xlsx now lives in the stdio component; there is no separate office component directory). It needs a compiled component, so it cannot run until the hub workspace resolves. The format catalog target is `@semio-tech/s-dev-composition:generate-stdio-format-claims` (check twin `check-stdio-format-claims`, then `generate`).

## Next commands once the tree builds

1. `repo test @semio-tech/stdio-artifact-contract-editing-details-rs`
2. `repo cargo -- test --manifest-path 🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📦️packages/🦀️rust/Cargo.toml --lib --features full-artifact-catalog` and correct the counts in `🧪️tests/🔬️unit/🦀️.rs` only from observed values.
3. `repo run-many -t stdio-quick,stdio-mutation-law,stdio-inference -p @semio-tech/stdio-js`
4. `repo run @semio-tech/s-dev-composition:generate-stdio-format-claims`, then `repo run @semio-tech/os-plugin-describe-rs:describe` for the stdio component, then `...:check-stdio-format-claims`.
