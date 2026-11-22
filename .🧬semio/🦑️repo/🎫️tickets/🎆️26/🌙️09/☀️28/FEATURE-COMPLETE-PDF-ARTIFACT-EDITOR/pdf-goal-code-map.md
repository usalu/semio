# 📘 PDF Artifact Goal Code Map

> 🗺️ Read-only map of the `stdio.pdf` artifact (PDF 1.7 and 1.4 standards, their subsets, the 1.7 editor) and the related hub, variant and framework paths. Produced for ticket `FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR`. No source file was changed and no git write command was run.

Root: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf` (3453 files: 1344 `.rs`, 397 `.ts`, 560 `.json`, 323 `.pdf` fixtures, 280 `.md`, 193 `.proto`, 193 `.graphql`, 76 `Cargo.toml`, plus grammar files `.g4 .ebnf .abnf .ksy .spicy .feature .semio`).

## 1. Top-level structure

| Path (under the artifact root) | Role |
|---|---|
| `🦀️.rs`, `🟦️.ts`, `📜️artifact-definition.json`, `🧾️artifact-declaration/` | Artifact entry: Rust `StdioPdf` artifact kind (re-exports 1.7 and 1.4 schema, snapshot, diff, mutations, io), TypeScript facade (`PdfSnapshot`, `PdfPage`, sqlite snapshot codec), artifact definition. |
| `🏅️standards/7️⃣1.7/` | PDF 1.7 standard. `🪆️subsets/` holds 9 subsets: `🧱️base` (core), `📐️e` (engine/viewer/edit extension), `🖨️x` (print), `🧾️vt` (variable text), `♿️ua` (accessibility), `🗄️a` (archive), `⚕️h` (health), plus `📦️packages`. |
| `🏅️standards/4️⃣1.4/` | PDF 1.4 standard with subsets `🧱️base`, `🖨️x`, `🗄️a`. Same codec, schema, snapshot, diff and mutation layering. |
| `🧱️base` (1.7) | Engine core. `🧬️schema` (snapshot, mutations, inferences, graph-projection, conformance-support, testkit), `🚪️io` (binary/text import, export, sqlite snapshot, error types, foreign artifacts), `🔨️modules` (lexer, xref, lift/lower, filters, fonts, images, content), `🏭️generator` (codec generation), `👁️viewer`, `🧪️tests`, `✏️editor` (the editor, see section 3). |
| `🧱️base/✏️editor` | PDF editor layer: `💾️native-save` (graph writer, proof, job, stored-flate), `📄️document` (image authoring, picker), `📥️local-image` (import), `🖼️page` (scene, model, actions, object/annotation/resource/document edits, text layer, inspector, windows, geometry, content ops), `🎭️modes/✏️edit` (single `main` window), `👥️presence` and `🎮️commands`, `🫧️transient`, `🎚️config` (schema or placeholder only). |
| `🏅️standards/*/🧬️schema/🧬️mutations` | 64 mutation folders (e.g. `set-page-content`, `insert-page`, `move-page`, `remove-page`, `insert-annotation`, `set-form`, `set-font`, `set-image`). Each has `🦀️.rs`, `🟦️.ts`, schema, and a `🧪️tests/🔬️unit` folder (61 of 64 have one). |
| `📦️packages/🦀️rust` and `📦️packages/🟦️typescript` | Publishable packages: Rust workspace member and `@semio-tech/stdio-pdf` (TS, build output in `📤️dist`). |
| `🔮️oracles` | Oracle package (`🦀️.rs`, `🔣️.json`) for cross-checking outputs. |
| `🧪️tests` | `🔬️unit`, `🔬️rust-package-commands`, `🔬️typescript-package-commands`. |
| `🖼️assets/🌈️icc` | ICC colour profiles. |
| `🧩️composition/🔣️.json`, `🧾️artifact-declaration/🦀️.rs` | Composition and declaration glue. |

Viewer surface: `🏅️standards/*/🪆️subsets/📐️e/👁️viewer` (7 Rust files), with 1.7 `pdf17` and `pdf17e` viewers. Editor and viewer per dialect are registered in the hub component (`🌎️hub/.../📘️pdf/🧩️component/🦀️.rs`): `Pdf17Editor`, `Pdf17Viewer`, and the 1.4, 1.7-A, 1.7-E, 1.7-H, 1.7-UA, 1.7-VT, 1.7-X variants.

## 2. Build projects and test targets

Projects are `📋️project.json` (14 files) plus Cargo manifests (76 `Cargo.toml`, 63 workspace members in `Cargo.toml`). Targets use `executor: repo:script` with `repo ./📜️script.ts <target>`.

| Project (`name`) | File | Targets |
|---|---|---|
| `@semio-tech/stdio-pdf-rs` | `📦️packages/🦀️rust/📋️project.json` | `build`, `check`, `test`, `test-snapshot-sqlite` (+ `source`, `native`), `verify-snapshot-sqlite-source`, `test-recursive-retirement`, `test-stream-roles` |
| `@semio-tech/stdio-pdf` | `📦️packages/🟦️typescript/📋️project.json` | `build`, `check`, `test`, `verify-physical-codecs` |
| 1.7 page editor (Rust) | `🏅️standards/7️⃣1.7/…/✏️editor/🖼️page/📦️packages/🦀️rust/📋️project.json` | (not inspected in detail) |
| 1.7 page editor unit tests | `…/✏️editor/🖼️page/🧪️tests/🔬️unit/📦️packages/🦀️rust/📋️project.json` | test project |
| Codec and schema generators (7 for 1.7, 3 for 1.4) | `🏭️generator/📋️project.json` under `⚕️h`, `🖨️x`, `📐️e`, `🗄️a`, `🧱️base`, `🧾️vt`, `♿️ua` (1.7) and `🖨️x`, `🧱️base`, `🗄️a` (1.4) | generator targets (not inspected) |
| Hub extension | `🌎️hub/.../📘️pdf/📦️packages/🦀️rust/📋️project.json` | included in the hub build |

Test-like code: 716 `#[test]` functions across 428 Rust files in the artifact. In the 1.7 editor, 60 `#[test]` plus 13 `async_test` (the `registered_pdf_*` tests in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, undo and redo at lines 369 and 409). One TypeScript test: `…/🧭️geometry/🧭️coordinates/🧪️tests/oracle.test.ts`. 21 language-agnostic JSON fixture cases under `…/✏️editor/🖼️page/🧪️tests/🔬️unit/` (e.g. `delete-path.json`, `truetype-outline.json`, `move-text-run.json`). The variant test folder is `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧪️tests/🚪️route`.

## 3. Editor feature matrix

The 1.7 editor is one window, `main` (`🎭️modes/✏️edit/🪟️windows/🪟️main`), a canvas of all pages with 68 `action(...)` ids in `🖼️page/🪟️windows/🦀️.rs`. Commands are the `Pdf17EditorCommand` enum (`✏️editor/🦀️.rs:38`): `set-page` (text replace with revision guard), `page-edit` (generic action + payload), `pick-image`, `import-local-image`. Mutations are the events (`PdfMutation`).

Legend: E = engine (Rust mutation or model code), U = UI action or window surface, T = tests. Paths are relative to `…/🧱️base/✏️editor` unless stated.

| Feature | Engine | UI | Tests | Status |
|---|---|---|---|---|
| Open (document) | `…/🧱️base/🚪️io/📥️import`, framework artifact load | not a page action | io tree tests | Implemented (framework), not an editor action |
| Open (local image) | `📥️local-image/🦀️.rs`, `pick-image` / `import-local-image` commands | image picker in `📄️document/🖼️image-authoring` | no `#[test]` in `📥️local-image` | Implemented, untested here |
| Render pages (vector canvas) | `🖼️page/🗺️scene`, `🖌️graphics`, `🧱️model`, `🧭️geometry` | `🪟️windows` `render_window`, canvas | `🖼️page/🧪️tests/🔬️unit/🦀️.rs` (`page_lists_text_vector_and_image`, `canvas_vector_paint_matches_document_paths`) | Implemented, tested |
| Page viewport and crop | `🗺️scene` (`page_viewports`, `clip_rect`) | canvas viewport | `canvas_viewport_crops_page_paint_and_hit_testing` | Partial (viewport only, see zoom) |
| Zoom | none | no zoom action in the 68 ids | none | Not implemented |
| Thumbnails | `📄️document-edits` `thumbnail` key (sets `/Thumb`) | none (no thumbnail strip or render) | `🖼️page/🧪️tests/🔬️unit/🦀️.rs:458-463` | Partial (property only) |
| Search (text or object) | none (only `binary_search` in `💾️native-save`) | none | none | Not implemented |
| Select object (pointer) | `🖱️object-edits`, `🪟️windows` pointer handling | `canvasPointerDown/Move/Up` | `pointer_down_selects_the_hit_object_and_clears_a_miss` | Implemented, tested |
| Move, resize, delete object | `🖱️object-edits`, `🎬️actions` (`move`, `resize`, `delete`) | `move`, `resize`, `delete` | `move_resize_and_delete_address_text_vectors_and_images` | Implemented, tested |
| Text edit (content stream) | `📝️page-text` (`page_text_edit_mutations`, revision guard), `set-text`, `set-font-size` | `set-text`, `set-font-size`, `insert-text` | `set_text_replaces_the_run_and_keeps_the_rectangle`, `text-paint`, `font-edit` | Implemented, tested |
| Insert text, rectangle, line, image | `🎬️actions`, `🎨️resource-edits`, `🖌️graphics` | `insert-text`, `insert-rectangle`, `insert-line`, `insert-image` | `insert_text_rectangle_and_image_round_trip_onto_the_page` | Implemented, tested |
| Paint (fill, stroke, shading) | `🎨️resource-edits`, `🌄️shading-preview`, `set-fill`, `set-stroke` | `set-fill`, `set-stroke`, `set-pattern` | `restyling_one_path_preserves_its_neighbors`, `shading-paint`, `vector-paint` | Implemented, tested |
| Images (replace samples, masks) | `📄️document-edits`, `set-image`, `set-image-mask` | `set-image`, `set-image-mask` | `set_image_replaces_samples_and_keeps_the_placement`, `canvas_repaints_image_samples_after_replacement` | Implemented, tested |
| Annotate (edit existing) | `🔖️annotation-edits` (451 lines), `set-annotation` family | `set-annotation`, `-style`, `-border`, `-markup`, `-kind`, `-appearance` | `moving_resizing_and_deleting_an_annotation_preserves_its_note...`, `annotation_markup_form_settings_and_extra_entries_publish` | Implemented, tested |
| Annotate (create new) | `insert-annotation` mutation exists | no create-annotation action in the window list | none specific | Partial (no UI to create) |
| Forms (fields, settings, content) | `🔖️annotation-edits`, `set-form`, `set-acro-form`, `set-field-data` | `set-form-field`, `set-form-settings`, `set-form-content`, `set-field-data` | `form_move_and_shading_move_change_their_geometry`, `canvas_paints_nested_form_content...` | Implemented (metadata and geometry), field value fill not exposed |
| Page insert | `insert-page` mutation (`📥️insert-page`) | `insert-page` ("Insert page after") | mutation unit test | Implemented, tested |
| Page delete | `remove-page` mutation (`🗑️remove-page`) | `remove-page` | mutation unit test | Implemented, tested |
| Page reorder | `move-page` mutation (`🔀️move-page`), `MovePage` | `move-page`, `move-page-earlier`, `move-page-later` (`🪟️windows` lines 214-225) | mutation unit test | Implemented, tested (no drag reorder) |
| Page replace | `replace-page` mutation (`🪄️replace-page`) | not exposed in UI | mutation unit test (3 tests) | Engine only |
| Merge pages or documents | none (`merge` is only the selection merge mode) | none | none | Not implemented |
| Split pages or documents | none (`split` hits are string parsing) | none | none | Not implemented |
| Export and save | `💾️native-save` (15 files: graph writer, producer, proof, job, window), `🚪️io/📤️export` | `ArtifactEditor` publish of `Pdf17Editor` (`✏️editor/🦀️.rs`) | `registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes` (editor unit test); no `#[test]` inside `💾️native-save` | Implemented, tested indirectly |
| Undo and redo | framework history (`artifact_app_laws::settle_history_verb`) | framework history verbs | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:369` and `:409` | Implemented via framework, tested |
| Revision guard (stale refusal) | `SetPage { revision }` | optimistic revision in `set-text` | `registered_pdf_draft_...refuses_stale` | Implemented, tested |
| Document info, metadata, catalog | `set-info`, `set-metadata`, `set-catalog-entry`, `set-viewer-preferences`, `set-page-label`, `set-outline`, many more | matching `set-*` ids | `document_info_pages_and_annotations_are_editable`, `catalog_collections_publish_onto_the_document`, `page_viewing_settings_publish...` | Implemented, tested |
| Page size, box, rotation, user unit | `set-page-box`, `set-page-rotation`, `set-page-user-unit`, `set-page-size` | same ids | `page_viewing_settings_publish_onto_the_document` | Implemented, tested |
| Embedded files, encryption, output intent | `set-embedded-file`, `set-encryption`, `set-output-intent` | same ids | `conformance_form_identity_and_font_program_publish` | Implemented, tested (partially, see mutation tests) |
| i18n (en and de) | `LocalizedLabel::native(en, de)` (89 uses), `Locale::En/De` (18 each), `pick(locale, …)` (9) | mode label `Edit` and `Bearbeiten` (`🎭️modes/✏️edit/🦀️.rs:19`); window labels | `set-language` test uses `de` (`🖼️page/🧪️tests/🔬️unit/🦀️.rs:272`) | Implemented, not every label tested |
| Accessibility (tagged, reading order) | `♿️text-layer` (accessible reading order), `♿️ua` subset, `set-mark-info`, `remove-mark-info` | text layer window (`♿️text-layer/🦀️.rs`) | no `#[test]` in `♿️text-layer`; `♿️ua` mutation unit tests | Partial (text layer untested) |
| Presence (multi-user) | schema only (`👥️presence/🧬️schema`) | none | none | Not implemented |
| Commands and transient UI | `🎮️commands`, `🫧️transient`, `🎚️config` | none | none | Placeholders only (`📌️.empty.md`) |
| Progress and cancellation | variant entry uses `AbortController` and `semio:catalog-progress` (`✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🚀️entry/🟦️.ts`) | none in editor window | none | Partial (boot only, not editor operations) |

Note on UI code: this artifact has no `.tsx`, `.html`, `.css` or framework UI. UI is declarative (`ComponentTree`, `WindowKindDefinition`, `ActionDefinition`) rendered by the framework. Browser development host: `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf` (Vite, `stdio-pdf-1-7` variant, route test at `🧪️tests/🚪️route`).

## 4. Stubs and gaps

Markers under all four paths (artifact, hub extension, dev variant, framework print product):
- `todo!`, `unimplemented!`, `TODO`, `FIXME`, `not implemented`, `throw new Error("not`: 0 each.
- `unsupported` or `not supported` (rs and ts): 72 occurrences. These are refusals in validators, not stubs. Top files: `🔨️modules/🗜️filters/🎚️decoded-stream/🦀️.rs` (11), `🔨️modules/🖼️images/🦀️.rs` (7), `🚪️io/🚨️error/🦀️.rs` (5), `🚪️io/📦️foreign-artifacts/🦀️.rs` (5), `🫳️borrowed` text snapshot (4), `💾️native-save/🧮️graph-quote.rs` (4).
- Empty function bodies: 25 single-line `{}`. Checked: most are drain loops (`while cursor.close_step(...) {}`) or no-op `register()` codec hooks (`🗿️artifacts/…/🔖️raw/✳️any`). No obvious stub among them. Worth a manual look: `🧬️schema/🔗️graph-source/🦀️.rs:21,23` (no-op trait methods) and `✏️editor/📄️document/🖼️image-authoring/🧵️reserved-picker.rs:90` (`begin_close`).
- Placeholder markers: 280 `📌️.empty.md` files (`🫧️transient` 60, `👥️presence` 60, `🎚️config` 60, `🎮️commands` 40, `🪛️utilities` 20, `🎬️actions` 20, `☑️options` 20). These are structure placeholders for unbuilt features, not code stubs.
- Ignored parameters: `🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` `render_windowed` discards `windows` and `locale` (`let _ = (windows, locale);`). Locale is intentionally not used for geometry, but the window does not use it for chrome either.
- Gaps with no code: merge and split of documents or pages, search, zoom, presence, clipboard, create-annotation UI, page-level thumbnail strip, document-level undo of mass edits (framework only).

## 5. Files that matter most

- Editor entry and commands: `…/🧱️base/✏️editor/🦀️.rs` (366 lines)
- Page window and actions: `…/✏️editor/🖼️page/🪟️windows/🦀️.rs` (267), `…/🖼️page/🎬️actions/🦀️.rs` (238)
- Text edit: `…/🖼️page/📝️page-text/🦀️.rs` (109)
- Annotations and forms: `…/🖼️page/🔖️annotation-edits/🦀️.rs` (451)
- Object edits: `…/🖼️page/🖱️object-edits/🦀️.rs` (174), resource edits `🎨️resource-edits/🦀️.rs` (365)
- Document edits: `…/📄️document/🦀️.rs` (321)
- Mode and window: `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs`
- Undo, redo, revision tests: `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (369-437)
- Hub composition: `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/🦀️.rs`
- Dev variant: `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🚀️entry/🟦️.ts`

Ticket not reopened or closed (no repo MCP calls were made). Only this file was created.
