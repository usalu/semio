# 🔍️ PDF Artifact Editor Gap Audit (versus XLSX)

Read-only audit of `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`, compared with `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx`. No build or test was run. Line counts are from `wc -l`. "Stub" means a construct that is a real placeholder, not a defensive `unreachable!()` or a doc comment that mentions the word.

Abbreviations: `P` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`, `B7` = `P/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base`, `B4` = `P/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base`.

## 1. Summary

- No `todo!`, `unimplemented!`, `TODO`, `throw new Error("not implemented")` anywhere in the PDF tree.
- Real stubs are limited to: `📜️artifact-definition.json` status fields (`unimplemented`), and one `MediaError::NotImplemented` fallback in the native-save producer.
- The 1.7 base editor, viewer and io are real code, but the viewer and the io facade have no unit tests and the io TypeScript facade is empty.
- Composition (`🧩️composition/🔣️.json`) references module paths that do not exist in the crate tree.
- The 1.7 base demo has no real `.pdf` asset. The 1.4 base demo does (`📚️examples/🎬️demo/🖼️assets/🎬️demo.pdf`).
- The PDF artifact definition declares `mutations: []` and `inferences: []`. XLSX declares 27 mutations and 1 inference.

## 2. Per-file implementation status (base subset, 1.7 first)

| File | Lines | Status | Notes |
|---|---|---|---|
| `B7/✏️editor/🦀️.rs` | 385 | Real | `Pdf17EditorCommand` (4 variants: `set-page`, `page-edit`, `pick-image`, `import-local-image`), `ArtifactEditor` impl, `handle`, `render`, native media export/import, manifest `create_pdf17_editor()` (line ~364). Lines 383–385 are an empty `//#region 🧪️Tests` block. No tests in this file. |
| `B7/✏️editor/🟦️.ts` | 8 | Thin | Exports dialect const, mode id, and `mainWindow` re-export only. No command or mutation types. |
| `B7/✏️editor/🎭️modes/✏️edit/🦀️.rs` | 30 | Real | Mode definition. |
| `B7/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` | 26 | Real | `definition()`, `render()`, `render_windowed()`. |
| `B7/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 1141 | Real | About 17 test fns: natural-file round trip, encryption refusal, definition, dialect, missing/explicit `set-page` payload, draft publish/undo/redo, page-edit undo/redo, `qpdf_accepts` (shells out to an external `qpdf` binary, test-only), every-subset save, text layer. |
| `B7/✏️editor/🖼️page/*` | ~many files | Real | `🎬️actions` (277 lines), `📝️page-text` (109), `🦀️.rs` (79), `🖼️image-authoring` (13 files), `🔭️view`, `🗺️scene`, `🖌️graphics`, `🧮️content-ops`, etc. Not reviewed line by line; grep found no stubs. |
| `B7/✏️editor/💾️native-save/*` | ~2361 total | Real, one fallback | `📤️producer.rs:143`: `*self.pending = Some(Err(MediaError::NotImplemented));` inside `decline()`. Check whether this is reachable from a real save path. |
| `B7/✏️editor/📥️local-image/🦀️.rs` | 65 | Real | Local image import, referenced by `#[path]` from editor root. |
| `B7/👁️viewer/🦀️.rs` | 114 | Real, read-only | `Pdf17Viewer` impl. `handle()` returns `ViewEmit::default()` for the only variant `Noop`, by design (see doc comment at line ~62). |
| `B7/👁️viewer/🟦️.ts` | 9 | Thin | Dialect const, view mode id, `mainWindow` re-export. |
| `B7/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs` | 47 | Real | Per-page read-only overview (`render(document)` at line 39). |
| `B7/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | 31 | Stub tests | 0 `#[test]`. Viewer has no tests. |
| `B7/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs` | 34 | Unverified | Not counted; check it has real assertions. |
| `B7/🚪️io/🦀️.rs` | 114 | Facade | `pub use` of sibling crates `semio_s_artifact_stdio_pdf_*`. No logic. |
| `B7/🚪️io/🟦️.ts` | 1 | Empty | `export {};`. The 1.7 TS io surface is only at repo root `🟦️.ts` (sqlite functions) and at `🪶️sqlite/📸️snapshot/🟦️.ts`. |
| `B7/🚪️io/📥️import/🦀️.rs` | 168 | Real | `sniff_pdf`, `decode_pdf`, `decode_pdf_with_password`, `page_text`. Natural-decode admit paths in `🔁️controlled/🦀️.rs`. |
| `B7/🚪️io/📤️export/🦀️.rs` | 63 | Real | `demo_pdf17_snapshot()`, `blank_pdf_snapshot()`. `encode_pdf` is at `📤️export/🧬️encode-and-reconcile/🦀️.rs:21`. |
| `B7/🚪️io/📤️export/🧪️tests/🔬️unit/🦀️.rs` | 501 | Real | 16 `#[test]`. |
| `B7/🚪️io/📝️text/🦀️.rs` | 9 | Facade | Re-exports only. |
| `B7/🚪️io/💾️binary/🦀️.rs` | 9 | Facade | Re-exports only. |
| `B7/🚪️io/🪶️sqlite/🦀️.rs` | 3 | Facade | Re-export of `inferences` only. |
| `B7/🚪️io/🧪️tests/` | 2 json | Fixtures only | `retained-text/🔣️.json`, `logical-character-codes/🔣️.json`. No `.rs` tests. |
| `B7/🚪️io/🔤️text-string/🦀️.rs` | n/a | Not reviewed | Assumed real (PDF text string codec). |
| `B7/🧬️schema/🧬️mutations/🦀️.rs` | n/a | Real | `PdfMutation` enum, 61 variants (lines 151–211). |

## 3. 1.4 base (`B4`) and other subsets

| Area | Lines | Status | Notes |
|---|---|---|---|
| `B4/✏️editor/🦀️.rs` | 7 | Macro call | `semio_s_artifact_stdio_pdf_editor_kit::pdf14_editor!(...)`. Real code is in the kit. |
| `B4/✏️editor/🟦️.ts` | 8 | Thin | Same shape as 1.7. |
| `B4/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | n/a | Present | |
| `B4/👁️viewer/🦀️.rs` | 99 | Real | |
| `B4/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | n/a | Present | |
| `B4/🚪️io/🦀️.rs` | 804 | Real | Full 1.4 reader and writer (classic xref, `/Prev` chain, rescan). `encode_pdf` at line 539. |
| `B4/🚪️io/🟦️.ts` | 1 | Empty | |
| `B4/📚️examples/🎬️demo/🖼️assets/🎬️demo.pdf` | file | Present | Real PDF asset. Demo also has `🦀️.rs`, `🟦️.ts`, `🧪️tests/🔬️unit/🦀️.rs`, `🗣️.dsl.semio`, `🎒️.pack.semio`. |
| `B4/🧬️schema/🧬️mutations/` | 38 `🔣️.json` | Present | |
| 7.1.7 `♿️ua`, `⚕️h`, `📐️e`, `🖨️x`, `🗄️a`, `🧾️vt` | 6 each for editor root | Macro calls | Each `✏️editor/🦀️.rs` is 6 lines using `pdf17_editor!`. Each `👁️viewer/🦀️.rs` is ~114 lines and contains a real `create_pdf17_*_viewer`. |
| 4.1.4 `🖨️x`, `🗄️a` | n/a | Present | Editor and viewer files exist. |

## 4. Stub and placeholder grep

Command pattern: `todo!|unimplemented!|panic!("not|TODO|placeholder|not implemented|Not implemented|"unimplemented"|NotImplemented|unreachable!()` over `.rs .ts .json .graphql .md`, excluding `node_modules` and `dist`.

| File | Line | Hit | Classification |
|---|---|---|---|
| `P/📜️artifact-definition.json` | 37, 43 | `"status": "unimplemented"` on `profiles` (1.4 and 1.7 `profile.any`) | Real gap in definition metadata |
| `P/📜️artifact-definition.json` | 52, 59 | `"status": "unimplemented"` on `source_dialects` | Real gap |
| `P/📜️artifact-definition.json` | 133 | `"status": "unimplemented"` on `resources` (external references) | Real gap |
| `P/📜️artifact-definition.json` | 155 | `"status": "unimplemented"` on `conformance_suites` (`public-corpus`, empty `fixtures`) | Real gap |
| `P/📜️artifact-definition.json` | 569–572 | `"read"`, `"write"`, `"lossless"`, `"canonical"` = `"unimplemented"` | Real gap, in an entry at lines 560–575 next to `validators`, `mutations`, `inferences` |
| `B7/✏️editor/💾️native-save/📤️producer.rs` | 143 | `*self.pending = Some(Err(MediaError::NotImplemented));` | Real stub fallback. Verify reachability. |
| `B7/👁️viewer/🦀️.rs` | 62, 75 | "not `unreachable!()`" in a doc comment | False positive |
| `B7/👁️viewer/...main/🦀️.rs` | 6 | "never a placeholder invented by this window" | False positive |
| `B7/✏️editor/🖼️page/🧪️tests/...paint-fidelity/🦀️.rs` | 148 | assertion message containing "placeholder" | False positive (test wording) |
| Many `🦀️.rs` | various | `unreachable!()` in `let ... else` and match arms (lexer, xref, filters, images, text) | Invariant assertions, not stubs. Low risk. |

No hits for `todo!`, `unimplemented!`, `TODO`, `throw new Error`, or `panic!("not` in the PDF tree.

## 5. Examples and tests

| Item | PDF 1.7 base | PDF 1.4 base | XLSX base |
|---|---|---|---|
| `📚️examples/🎬️demo/` real source asset | No `.pdf` (only `.dsl.semio` and `.pack.semio`) | Yes: `🎬️demo.pdf` | Yes: `📕️example.xlsx` |
| Demo `🦀️.rs` / `🟦️.ts` | No | Yes | Yes |
| Demo `🧪️tests` | No | Yes | Yes |
| Editor unit tests `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | Yes, 1141 lines | Yes | Yes, 570 lines |
| Viewer unit tests | **No** (0 `#[test]`) | Present | Present |
| Mutation / sqlite / model law suites (root `🧪️tests`) | **No** (`🔬️unit`, `rust-package-commands`, `typescript-package-commands` only) | n/a | Yes: `🔬️mutation-laws`, `🔬️sqlite-laws`, `🔬️model-laws` |
| Gherkin feature files | 3 (`create-minimal-pdf`, `edit-existing-pdf`, `mutate-pdf-1-7`) with Rust runners | n/a | n/a |
| Test fixtures | ~30 `⬅️before.pdf` / `➡️after.pdf` pairs, `two-pages.pdf`, `indirect-length.pdf`, `repair-stream.pdf` | n/a | `📊️stable-cell-edit`, `📊️sheet-grid` |

## 6. Mutations and commands

### 6.1 Schema-level mutations (`PdfMutation`, 1.7 base)

Source: `B7/🧬️schema/🧬️mutations/🦀️.rs:150–211`. Schema JSON: `B7/🧬️schema/🧬️mutations/🔣️.json` (61 `$ref`s, one `🔣️.json` per leaf under `🧬️mutations/<leaf>/`).

Leaves (61):
`insert-page`, `remove-page`, `replace-page`, `move-page`, `set-page-media-box`, `set-page-crop-box`, `set-page-content`, `set-page-rotation`, `set-page-box`, `set-page-user-unit`, `set-page-labels`, `set-page-layout`, `set-page-mode`, `append-page-content`, `insert-content`, `remove-content`, `replace-content`, `insert-annotation`, `remove-annotation`, `set-annotation`, `set-info`, `set-metadata`, `set-document-id`, `set-encryption`, `set-language`, `set-mark-info`, `set-open-action`, `set-viewer-preferences`, `set-properties`, `remove-properties`, `set-catalog-entry`, `remove-catalog-entry`, `set-dict-entry`, `remove-dict-entry`, `set-trailer-entry`, `remove-trailer-entry`, `set-object-value`, `insert-object`, `remove-object`, `set-font`, `remove-font`, `set-image`, `remove-image`, `set-form`, `remove-form`, `set-acro-form`, `set-ext-g-state`, `remove-ext-g-state`, `set-shading`, `remove-shading`, `set-pattern`, `remove-pattern`, `set-color-space`, `remove-color-space`, `set-embedded-file`, `remove-embedded-file`, `set-outlines`, `set-named-destination`, `remove-named-destination`, `set-output-intents`, `set-optional-content`.

Count check: the enum at `🦀️.rs:150–211` has 61 variants. The JSON has 61 `$ref`s. Both match.

### 6.2 Editor-level commands

`Pdf17EditorCommand` (`B7/✏️editor/🦀️.rs`, lines ~22–27):
- `set-page` (page, item, revision, text)
- `page-edit` (action, payload)
- `pick-image`
- `import-local-image`

Snapshot editing tools declared in `bounded_native_editing_editor!` (`B7/✏️editor/🦀️.rs`, near line 350): `set-page`, `set-text`, `move`, `resize`, `delete`, `set-fill`, `set-stroke`, `insert-text`, `insert-rectangle`, `insert-line`, `insert-page`, `remove-page`, `move-page`, `set-page-size`, `set-info`, `set-ann...` (truncated in the source output). Host tools: `insert-image`, `replace-image`. Transient tools: `zoom-in`, `zoom-out`, `fit-page`, `fit-width`, `actual-size`, `go-to-page`, `set-query`, `toggle-match-case`, `next-hit`, `previous-hit`, presence and selection tools.

Page-level actions (`B7/✏️editor/🖼️page/🎬️actions/🦀️.rs`, `is_page_action` at line 184 of `🧱️model`): `move`, `resize`, `delete`, `set-text`, `set-font-size`, `insert-text`, `insert-rectangle`, `insert-line`, `insert-image`, `insert-page`, `remove-page`, `move-page`, `set-fill`, `set-stroke`, `presence-*`.

### 6.3 Gap against XLSX

- PDF definition `mutations: []` (`P/📜️artifact-definition.json`). XLSX definition declares 27 mutations and 1 inference.
- All composition apps have `"mutationRoster": false` in both PDF and XLSX.

## 7. Editor registration and composition

### 7.1 XLSX

- `📕️xlsx/🧩️composition/🔣️.json`: `order: 26`, package `semio-s-artifact-stdio-xlsx`, 6 apps (`XlsxEditor`, `XlsxViewer`, `XlsxStrictEditor`, `XlsxStrictViewer`, `XlsxTransitionalEditor`, `XlsxTransitionalViewer`).
- Type paths: `semio_s_artifact_stdio_xlsx_base::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor`, and similar. Factories: `create_xlsx_editor` (`B/✏️editor/🦀️.rs:429`, a literal `fn`).

### 7.2 PDF

- `P/🧩️composition/🔣️.json`: `order: 23`, package `semio-s-artifact-stdio-pdf`, 20 apps (10 editor + 10 viewer). `nativeFactories` includes `stdio.native.pdf.v1` pointing at the definition and protocol files. `playgrounds` and `mutationDescriptors` exist.
- Type paths use the form `semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor`, `...::viewer::pdf17h::...`, etc. **These module paths do not exist.** The crate tree is `standards::v1_4|v1_7::subsets::...`. Only the `editor`/`viewer` leaf names are guesses.
- Factory existence check against literal `fn create_...(` definitions:
  - Viewer factories: all 10 found as literal `fn` in subset `👁️viewer/🦀️.rs` files (`create_pdf14_a_viewer`, `create_pdf14_viewer`, `create_pdf14_x_viewer`, `create_pdf17_a_viewer`, `create_pdf17_viewer`, `create_pdf17_e_viewer`, `create_pdf17_h_viewer`, `create_pdf17_ua_viewer`, `create_pdf17_vt_viewer`, `create_pdf17_x_viewer`).
  - Editor factories: only `create_pdf17_editor` is a literal `fn` (`B7/✏️editor/🦀️.rs`). The other 9 (`create_pdf14_a_editor`, `create_pdf14_editor`, `create_pdf14_x_editor`, `create_pdf17_a_editor`, `create_pdf17_e_editor`, `create_pdf17_h_editor`, `create_pdf17_ua_editor`, `create_pdf17_vt_editor`, `create_pdf17_x_editor`) are generated by `semio_s_artifact_stdio_pdf_editor_kit::pdf14_editor!` / `pdf17_editor!` macros at each subset's `✏️editor/🦀️.rs`. They exist, but the composition module paths for them are wrong.
- No consumer of `composition-contribution` was found under `✏️s/` outside the artifact folders. The consumer may live in a generator not yet located.

### 7.3 Is PDF wired like XLSX?

Partly. Both use `🧩️composition/🔣️.json` with the same schema (`semio.stdio.composition-contribution/v1`). The difference is that XLSX paths match its crate layout. PDF paths do not. Both have `mutationRoster: false`, and both have `openTargets` empty.

## 8. Artifact definition (`P/📜️artifact-definition.json`)

| Status | PDF | XLSX |
|---|---|---|
| implemented | 2 | 32 |
| declared | 4 | 3 |
| unverified | 2 | 1 |
| unimplemented | 6 | 1 |

PDF unimplemented entries: `profiles` (1.4 and 1.7 `any`), `source_dialects` (1.4 and 1.7), `resources` (external references), `conformance_suites` (public corpus, no fixtures), and codec `read`/`write`/`lossless`/`canonical` in one entry at lines 560–575.

Also: `mutations: []` and `inferences: []` at the definition root.

## 9. Root `🦀️.rs` and `🟦️.ts`

### 9.1 `P/🦀️.rs`

- `pub use artifact_declaration::*;` (from `🧾️artifact-declaration/🦀️.rs`).
- `pub mod standards { pub mod v1_4 { ... } pub mod v1_7 { ... } }`. Re-exports from `semio_s_artifact_stdio_pdf_*` crates for:
  - v1_4: `base` (schema, io with `import`/`export` serializers), `a`, `x`.
  - v1_7: `base` (`modules` for lexer, filters, xref, encryption, fonts, content, colour, images, lift, lower, writer; `schema`; `io`), plus `a`, `e`, `h`, `ua`, `vt`, `x`.
- Imports `PdfMutation`, `PdfSnapshot`, `STDIO_PDF17_DOCUMENT_SCHEMA`, `STDIO_PDF_DOCUMENT_SCHEMA` for internal use.
- `use semio_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};`.

### 9.2 `P/🟦️.ts`

- Re-exports `definition` (JSON), type `ArtifactDefinition`.
- Re-exports `PDF17_SQLITE_SCHEMA`, `pdf17SnapshotToSqliteDatabase`, `pdf17SnapshotFromSqliteDatabase`, `PDF14_SQLITE_SCHEMA`, `pdf14SnapshotToSqliteDatabase`, `pdf14SnapshotFromSqliteDatabase`.
- Types: `PdfSnapshot`, `PdfPage`, `Binary64` (re-exported from `@semio-tech/framework/value/ieee754`, a framework type).

## 10. Prioritised gaps for the end-to-end goal

1. **Composition paths are wrong** (`P/🧩️composition/🔣️.json`). Fix the `type` and `factory` module paths to the real crate layout before anything consumes this file.
2. **No real PDF demo asset for 1.7 base**. `📚️examples/🎬️demo/` has only `.dsl.semio` and `.pack.semio`. Add a real `.pdf` and a demo `🦀️.rs`/`🟦️.ts`/tests, as 1.4 has (`🎬️demo.pdf`).
3. **Viewer has zero tests** (`B7/👁️viewer/🧪️tests/🔬️unit/🦀️.rs`, 31 lines, 0 `#[test]`).
4. **Editor root has no unit tests in `🦀️.rs`**. Its `//#region 🧪️Tests` block at lines 383–385 is empty. The real tests live in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, which is fine, but no law suites (`mutation-laws`, `sqlite-laws`, `model-laws`) exist for PDF as XLSX has.
5. **Definition metadata gaps**: `mutations: []`, `inferences: []`, codec `read`/`write`/`lossless`/`canonical` unimplemented, profiles and conformance suite unimplemented, external-resource policy unimplemented.
6. **`NotImplemented` in native save**: `B7/✏️editor/💾️native-save/📤️producer.rs:143`. Confirm whether a normal save can reach it, then implement or remove it.
7. **`B7/🚪️io/🟦️.ts` is `export {};`**. The 1.7 io TypeScript surface is missing. Only the root and sqlite TS facades exist.
8. **`qpdf` is an external test-time binary** (`B7/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `qpdf_accepts`). Keep as a test-only tool behind an interface, per the repo rules.
9. **Native media export and decline fallback**: the export path is substantial (2361 lines), but `producer.rs:143` shows a fallback that completes with `NotImplemented`. Trace whether it is hit.
10. Not verified: whether any of this compiles or passes. This audit is read-only.

## 11. Verification limits

- Read-only. No `cargo`, `repo test`, or other build was run.
- Grep and line counts only for most files. The page-editor subtree (`✏️editor/🖼️page/**`), `📄️document/🖼️image-authoring/**`, and `B7/✏️editor/💾️native-save/**` were checked for stub markers, not read.
- The viewer window test file (`B7/👁️viewer/🎭️modes/.../🧪️tests/🔬️unit/🦀️.rs`, 34 lines) was not read.
