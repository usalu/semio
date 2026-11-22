# PDF Artifact Editor Authoring Features

Scope: annotation creation, form fill, page replace, merge, extract, page structure (G15) and controlled import (G11). Another agent owns zoom, search, thumbnails, `render_windowed`, runtime traps, build, large-file IO and fonts/paint.

## Verification status (read first)

The implementation and its tests are written, but **no test run has completed**. Every `repo cargo` attempt was blocked by other agents' concurrent work (capacity policy schema, native-build admission export, a missing crate manifest, the PDF io dependency cycle, and finally `semio-ui-layout` failing with E0432 on `validate_window_layout` in the last run). After the coordinator's io-export fix the dependency graph resolved and compilation reached unrelated framework crates. None of the new crates (authoring, authoring-tests) has been compiled to completion yet, so compile errors in them are still possible.

Run once the tree builds:

```
repo cargo --store=shared --configuration=prototype -- test --offline --manifest-path '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.toml' -p semio-s-artifact-stdio-pdf-page-editor-authoring-tests --lib
```

Treat all code below as unverified until that run is green.

## What was built

Everything is an event: each command emits `PdfMutation` leaves (one undo group) or a host `Effect`. Nothing is CRUD, nothing mutates the snapshot directly.

| Feature | Action id | Result |
|---|---|---|
| Create annotation: note, highlight, underline, strike-out, squiggly, ink, link (URI or `page:N`) | `create-annotation` | `InsertAnnotation` at the end of the page list. Highlight quad points come from the rectangle or an explicit list; ink rectangle is the stroke bounds plus 1 pt. |
| Fill form field (text, checkbox, radio, combo/list) | `fill-form-field` | `SetForm` (regenerated appearance), `SetAnnotation` (appearance and `/AS` state), `SetAcroForm` (value). Refuses read-only, push button, signature, over-length and unknown options. |
| Replace page | `replace-page`, `replace-page-pick` | Blank page of a size, or one page of a picked PDF: `ReplacePage` plus renamed resources. |
| Merge pages of another PDF | `merge-pick`, `merge-document` | `InsertPage` per page plus used resources, form fields and link destinations rewired. |
| Split or extract pages to a new PDF | `extract-pages` | One `DownloadMediaExport` effect per `;`-separated selection (`1-3,5;7`). A query, not a mutation. |
| Page structure keeps widgets and links | `insert-page`, `remove-page`, `move-page` | Now also emit `SetAcroForm` and `SetAnnotation` renumbering widget refs and link destinations (previously stale after any page change). |

Merge details: only resources the picked pages reach (transitively through forms, patterns, graphics states, soft masks, Type 3 glyph programs, image soft masks and appearance forms) are copied, all renamed with a prefix `mN_` that is free in the target. Source outlines, named destinations and document-level extras are not merged. Colliding field names get a `_N` suffix, and their widget annotations are renamed.

The picker is the host file picker (`RequestFileOpen`, data URL, 16 MiB bound) that redispatches the action with `payload`, so the transfer is host-cancellable. Base64 runs through the controlled codec with a work and output bound and a progress hook; the synchronous page transform has no cooperative cancel point yet (see Gaps).

UI (accessible, English and German labels, buttons and inputs bound to the actions): the page inspector has Add note, highlight, underline, strike-out, freehand stroke, an Add link input, Insert pages from PDF, Replace page from PDF, Replace with blank page and Extract page to PDF. A selected widget annotation shows a Field value input bound to `fill-form-field`. All seven actions are declared on the page window, in `PAGE_ACTIONS` (now 75) and in all seven editor command rosters (base, A, E, H, UA, VT, X).

## Where it lives

- `…/✏️editor/🖼️page/✍️authoring/🦀️.rs`: the engine. Own crate `semio-s-artifact-stdio-pdf-page-editor-authoring` (`…/✍️authoring/📦️packages/🦀️rust`) because the page crate is at its source boundary. The page module re-exports it (`pub use`).
- Wiring: `🎬️actions` (payload mapping, emit), `🪟️windows` (action definitions, inspector call), `🔎️inspector` (controls), `🧱️model` (action list), seven editor `🦀️.rs` rosters.
- Workspace: new members and lock entries for the authoring crate and the authoring tests crate in `📖️pdf/Cargo.toml` and `Cargo.lock`; dependency added to the page crate.
- Third-party oracle: `🔮️oracles` of 1.7 base gained `reference_annotation_subtypes`, `reference_field_values`, `reference_widget_appearances` (lopdf).

## Tests

`…/🖼️page/🧪️tests/🔬️unit/✍️authoring/` (own crate `semio-s-artifact-stdio-pdf-page-editor-authoring-tests`):

- language-agnostic fixtures `🧪️authoring.json` (page selections, 12 annotation cases, 13 fill cases, 7 page-structure cases) and `🧪️field-controls.json` (snapshot field to control action map plus expected inspector bindings);
- lopdf cross-checks: annotation subtypes per page, field `/V`, widget appearance streams (text shows `(Ada) Tj`, checkbox tick path, radio states `Yes=Yes`, `L=L`, `Off=Off`), page counts after merge, extract, structure edits;
- law checks: merge renames only used resources, rewires widgets and link destinations, extraction re-reads to the same sizes and one font, picker and download effects, both languages render the same bindings, every `PdfSnapshot` field is either derived or mapped to a declared window action (the test parses the struct source, so a new field fails until mapped).

## G15

- Page insert, move, remove and replace now keep widget references and link destinations correct, with fixture cases for each (including a move target past the end). Previously widget `[page, annotation]` pairs went stale.
- Undo and redo run through the same mutation leaves; the registered-app history test for these actions was not added (the editor test crate is also at its source boundary and the registered harness could not be built).
- Every typed `PdfSnapshot` field now has a declared control action; the new test enforces it. Fields with no inspector input are still reachable only through the action palette (documented in the fixture, not hidden).

## G11 (controlled import barriers)

Not changed. Reading the code: the controlled cursor in `⬇️lift/…/🧭️document-lift/🧭️controlled` already admits resource-free documents, simple fonts and images and Text and URI Link annotations (reports `controlled-annotations`, `controlled-images`, `content-program`). What still defers: shapes outside those (`Barrier::Resources` for unadmitted resource bindings, `Barrier::Program` for text-bearing programs outside the admitted font set, `Barrier::Roles` for non-canonical stream roles) and the mounted `ReferenceRequired` error path. These are grant and borrow engineered (about 215 KB source closure, governed native execution) and each closure needs its own oracle and governed run, so a safe partial edit was not possible within this ticket. Next smallest closure: nonempty `/Resources` with only admitted fonts and images on pages that also carry annotations.

## Second pass (coordinator gap list)

1. Generated appearances: highlight (Multiply blend `MarkupMultiply`), underline, strike-out, squiggly, ink and note now get a form XObject and `/AP` at creation (`markup_form`).
2. Cooperative cancel and progress: `import_pages_controlled` and `extract_pdf_controlled` take a `Control` closure fed `Progress{phase,done,total}` and stop with a fault, applying nothing, when it returns false. The plain entry points use a 20 s `deadline_control`. The host has no per-command cancel token in `handle`, so the deadline is the cancel source there.
3. Page edits now also renumber outline entries (recursive) and named destinations (`retarget_documents`); a removed target clears the outline destination and removes the named destination.
4. History test `registered_pdf_page_and_authoring_actions_undo_and_redo_step_by_step` appended to `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: insert, move, annotate, fill, replace, remove, then undo and redo each step against the stored snapshots.
5. Inspector inputs added for layout, mode, XMP metadata, tagging (3), document id (2), open link, bookmark, named destination, page label prefix. Still palette-only (listed in `paletteOnly` of `🧪️field-controls.json`): fonts, images, forms, graphics states, shadings, patterns, color spaces, properties, embedded files, output intents, optional content, viewer preferences, encryption, catalog entries. Each needs a multi-input form (key plus value), not one field.
6. Hub JSON: the real generator is the wasm descriptor emitter (needs a component build). `hub-composition-sync.py` (this folder) added the seven actions to all seven app entries (49 entries, pure addition). Regenerate with the emitter when the component builds.
7. G11 not changed, deliberately. The mounted path turns a deferral into `ReferenceRequired` on purpose: the only "admit more" shortcut is a cold `decode_pdf` fallback, which is unbounded, uncancellable work on a governed path. Real closure means extending the governed lifter (`🧭️document-lift/🧭️controlled`, 50 KB, inside a 215 KB source closure) with an oracle and a governed run per shape; it cannot be done blind on a tree that does not build.

Compile status: the authoring library crate compiled (run before the second pass). The last test run stopped on `semio-os-store-engine-replay` (`original_report_key` missing), not on this code. The authoring-tests crate has not passed once; the only errors seen in it were two `Fault`-has-no-Display lines, fixed.

## Third pass

1. Retained work: merge, replace-from-PDF and extract now run as the generic `PageAuthoringWork<A>` (authoring crate, `//#region ✍️Work`), an `ArtifactCommandWork` registered through a four-line hook in `native_work` (the editor mount crate is at its source boundary, so the work cannot live there). Each scheduler step does one bounded unit (`ImportJob`, `ExtractJob`, `ImportPlan::advance`: decode, then eight resources per step, then pages, then encode), yields on low fuel (`Pending`), reports `Progress` and stops with a fault when `cx.is_cancelled()`. The 20 s deadline remains only for the synchronous test and fallback entry points. Not verified: the store's per-edit row cap (`NATIVE_MAXIMUM_WORK_ITEMS` is 2) and the raw command-wire cap may still refuse a large merge payload, and the source decode is one cold `decode_pdf` unit (at most 16 MiB), not a governed cursor.
2. Generic key-value form: `entry_form` and `entry_forms` in the inspector (headed group, one labelled input per editable entry, read-only lines for non-name values, an add input taking `key=value`, en and de) bound to the new action `set-entry` (families catalog, trailer, info, viewer; `set_entry`, `entry_rows` in the authoring crate). Catalog and viewer entries therefore leave `paletteOnly`. Fonts, images, forms, graphics states, shadings, patterns, color spaces, properties, embedded files, output intents, optional content and encryption are not key-value shaped (nested typed records) and still use the palette.
3. G11: `g11-shape-oracle.py` (four shapes) and `g11-lifter-extension.md` (per-shape edit, quote and pass condition). The lifter itself is not changed; see that file for why.

Last build attempt (after this pass): stopped in `semio-s-artifact-stdio-pdf-inferences` (`stream_roles` import and a type mismatch, another agent's change), before reaching the new crates. Nothing from the second or third pass has compiled yet.

## Fourth pass

- `ImportJob` now decodes through the paged natural decoder (`build_pdf_natural_decode`, one governed `advance(cx)` turn per step) when a step context exists; the cold `decode_pdf` remains only for the context-free test path and for sources at most one work unit long. Not compiled yet.
- G11: `g11-shape-oracle.py` ran via `uv run --with pymupdf==1.26.5 --with pikepdf python -I` and its pikepdf projections match the expected shapes (media boxes, rotation 90 with inherited keys, operator lists, resource names). It also writes `🧪️fixtures/📌️controlled-shapes/🔣️.json`, and the test `controlled_shapes_match_the_independent_reader_and_the_cold_lift` (in the pdf-snapshot-value tests) lifts each shape cold and controlled and fails naming the barrier if the governed lifter still defers. **The lifter itself is still unedited**: which barrier fires per shape is only known from that test's first run, and the tree has not built since (last stop: `semio-s-artifact-stdio-pdf-page-editor` references an unlinked `semio_s_artifact_stdio_pdf_presence_lane`, another agent's change). The reference-required shape is not expressible as graph rows and stays a decoder-level case.
- Third-party oracle: extract and merge tests now also compare the content operators of the picked and merged pages against lopdf's reading of the source, not only page counts.
- Not done: running `registered_pdf_font_change_preserves_encoded_text_and_the_next_line` and `registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes` (both live in the editor unit tests, which were last failing on 11-21; no run was possible), and a lopdf comparison of undo/redo content.
