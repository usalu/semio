# PDF 1.7 editor feature inventory (static, read-only)

**Scope and method.** This is a static read of the source, with no app run. Every status below is "static", not "E2E verified". I used Glob/Grep/Read only and changed no files.

**Path aliases used below**
- `PDF` = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`
- `ED` = `PDF/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor` (the base 1.7 editor root)
- `HUB` = `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/🔣️.json` (generated composition)
- `DEV` = `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf`

## 1. Premise corrections

- **Keybindings are not declared by the PDF artifact.** `PDF/📜️artifact-definition.json` has no keybindings. `mod+z` and `mod+shift+z` are injected by the framework: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🕹️actions/⏪️history-and-host-event-actions/🦀️.rs:12` (undo) and the builder test at `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🎛️descriptor-owner-layouts-terminology-and-history/🦀️.rs:219`. The generated `HUB` lists `undo` (line 2508), `redo` with `mod+shift+z` (2565, 2579), `copy` with `mod+c` (4204, 4218), and `cut` with `mod+x` (4256, 4270). I did not confirm a `paste` / `mod+v` entry.
- **The `finalizeHistoryEdit` dialog is framework-owned.** Its constant is at `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏪️time-travel/🕹️actions/🦀️.rs:105`, and `HUB` declares it at line 6107. The history verbs `historyEditBegin/Input/UseSelection/Withdraw/Accept/Discard/Restore/Finalize/Commit/Back/Exit/CancelReplay/Rerun` are also in `HUB`. The PDF plugin has no history code of its own.
- **Window kinds match the premise.** The page window `pdf.page` is at `ED/🖼️page/🧱️model/🦀️.rs:6`, `pdf.object` at line 8, and `pdf.text-layer`, `pdf.thumbnails` and `s.stdio.window.snapshot-details` at their own modules. Declarations are in `ED/🦀️.rs:380-398`.
- **Documentation and feature flags.** There is no README or markdown doc for the PDF editor. The `.md` files under `ED` are `📌️.empty.md` placeholders. The only real markdown is `ED/🔨️modules/🔤️fonts/🖼️base14/SOURCE.md`. There are no runtime feature flags for the editor. The `feature` hits are Cargo test features only.

## 2. Editor shape

- **One document mode**: `edit` (`ED/🎭️modes/✏️edit/🦀️.rs:11-18`). `edit::layout()` (line 22) is never called. The manifest uses `document_layout` instead (`ED/🖼️page/🪟️windows/🦀️.rs:189-202`).
- **Default layout**: thumbnails 0.12, page 0.34, text-layer 0.18, object 0.18, snapshot-details 0.18.
- **Mismatch**: `HUB` (lines 6080-6098) lists `framework.window.document` and `s.stdio.window.snapshot-details` in its layout. That does not match the descriptor's `document_layout`. The hub may be stale.
- **Placeholder folders**: `🎚️config`, `🎮️commands`, `👥️presence` and `🫧️transient` under the mode and the window are `.empty.md` only.
- **Config**: `ED/🎚️config/🧬️schema/` exists, but the editor uses `type Config = NoConfig` (`ED/🦀️.rs:126`). The schema is not wired.
- **Action roster**: `edit_action_definitions()` (`ED/🖼️page/🪟️windows/🦀️.rs:49-148`) declares 74 document edit verbs. Three non-edit window actions (`canvasPointerDown/Move/Up`) and the 10 view actions are added alongside them. The editor's tool list is at `ED/🦀️.rs:358-369`.

## 3. Window kinds and in-window interactions

| Window kind | What it renders | In-window interactions | Status (static) |
|---|---|---|---|
| `pdf.page` (main, Canvas2d) | Every page stacked top to bottom, one layer per object. Source: `ED/🖼️page/🪟️windows/🦀️.rs:151-165`. | **Pointer select.** `canvasPointerDown` hit-tests an object (or the page, `p{n}:paper`) and dispatches `interactionSelect` with `merge=replace`, or `invertive` when shift, ctrl or meta is held. Empty space clears the selection. Source: `ED/🖼️page/🎬️actions/🦀️.rs:262-295`. | Select: implemented. |
| | | **Pointer drag, move and resize on the canvas.** `canvasPointerMove` and `canvasPointerUp` return `Emit::default()` (`ED/🖼️page/🎬️actions/🦀️.rs:210`). No engine handler moves or resizes an object on drag. Move and resize only happen through inspector fields or thumbnails. | **Partial / not implemented engine-side.** |
| | | **Zoom, fit, actual size, go-to-page.** Actions `zoom-in/out`, `fit-page`, `fit-width`, `actual-size`, `go-to-page(page)`. Zoom steps 0.1 to 8.0. Source: `ED/🖼️page/🔭️view/🎬️actions/🦀️.rs:10-46`, `ED/🖼️page/🔭️view/🦀️.rs:77-142`. | Logic implemented. Shell exposure not verified. |
| | | **Search.** `set-query`, `toggle-match-case`, `next-hit`, `previous-hit`. Hits are highlighted on the canvas. Source: `ED/🖼️page/🔭️view/🔍️search/🦀️.rs` (not read), `ED/🖼️page/🪟️windows/🦀️.rs:310-319`. | Logic implemented. Shell exposure not verified. |
| | | **Hover and presence.** `presence-pointer`, `presence-pointer-leave`, `presence-caret`, `presence-caret-clear`. Peer overlays come from `peer_overlay_layers` (`ED/🖼️page/🪟️windows/🦀️.rs:327-335`). | Implemented in the presence lane. Live multi-user not verified. |
| | | **Context menus, clipboard handlers.** None in the PDF source (grep for `contextMenu`, `clipboard`, `paste`, `copy` found nothing under `ED`). | **Absent** from PDF code. |
| `pdf.object` (inspector, BlockList) | With an object selected: kind-specific fields. With nothing selected: a page and document panel. Source: `ED/🖼️page/🔎️inspector/🦀️.rs` and `ED/🖼️page/🪟️windows/🦀️.rs:204-299`. | See the object and page panels below. | See below. |
| `pdf.text-layer` (BlockList) | Accessible reading-order rows: Title, Language, Tagged, then per page `Page n / N` and each object as `Kind: text`. Source: `ED/🖼️page/♿️text-layer/🦀️.rs:47-108`. | Each object and page row is a button that dispatches `interactionSelect`. No edit actions. | Implemented, read-only navigation. |
| `pdf.thumbnails` (tree) | One row per page with the scaled size and any collaborator captions. Source: `ED/🖼️page/🔭️view/🖼️thumbnails/🦀️.rs:90-140`. | Row activate selects the page. **Drag a row onto another row** dispatches `move-page` (`page` from the drag payload, `x` = drop target). Child rows `Move page N earlier/later`. Empty document shows `No pages`. | Implemented. |
| `s.stdio.window.snapshot-details` | Raw snapshot fields, from the shared contract. Source: `semio_s_artifact_stdio_contract::editing::render_snapshot_details` (not read). | Owned by the shared contract. | Out of local scope. |

### Object panel (selected object)

Each field commits on blur through a document edit verb, with the object's own page and id as arguments.

- **Text**: Text (`set-text`), Font (`set-font`, base-14 or admitted font), X and Y (`move`), Font size (`set-font-size`), Fill red/green/blue (`set-fill`). Declared in `ED/🖼️page/🔎️inspector/🦀️.rs:23-31`.
- **Vector**: X, Y, Width, Height (`resize`), Fill RGB, Stroke RGB and Line width (`set-stroke`). Declared at lines 32-40.
- **Image** and **Form**: X, Y, Width, Height. For Image, also Interpolate, Intent and Decode (`set-resource-detail`). Lines 41-53.
- **Image replacement button**: shown only when the controller is exactly `s.stdio.pdf@1.7/*#editor` (`ED/🖼️page/🪟️windows/🦀️.rs:285`). Subset controllers such as `s.stdio.pdf@1.7/x#editor` do not get it. This is an inconsistency.
- **Shading**: X, Y, Width, Height. Lines 54-59.
- **Annotation**: Text (`set-annotation` contents), X, Y, Width, Height (`set-annotation`), Color RGB (`set-annotation-style`), Border width (`set-annotation-border`), and Field value for form widgets (`fill-form-field`). Lines 60-75.
- **Delete** button (`delete`), for every selected object. `ED/🖼️page/🪟️windows/🦀️.rs:292-298`.

### Page and document panel (nothing selected)

- **Page info**: Title, Author, Subject, Keywords, Creator, Producer, Creation date, Modification date, Trapped (`set-info-field`). Language (`set-language`). Page width and height (`set-page-size`, MediaBox only). Rotation (`set-page-rotation`). User unit (`set-page-user-unit`). Lines 214-236 of `ED/🖼️page/🔎️inspector/🦀️.rs`.
- **Boundary boxes**: Media, Crop, Bleed, Trim, Art, each through `set-page-extra` with a `box.*` key. Lines 238-250.
- **Document**: Page layout (`set-page-layout`), Page mode (`set-page-mode`), XMP (`set-metadata`), Tagged, User properties, Suspects (`set-mark-info`), Permanent and Changing document id (`set-document-id`), Open action URI (`set-open-action`), Bookmark (`set-outline`), Named destination (`set-named-destination`), Page label prefix (`set-page-label`). Lines 147-166.
- **Page buttons**: Insert page after (`insert-page`, using the current page's size), Move page earlier and later (`move-page`), Remove page (`remove-page`, when there is more than one page). Lines 240-257.
- **Entry forms**: Catalog, Trailer, Info and Viewer key/value groups (`set-entry`), with an add row `key=value`. Lines 168-212.
- **Authoring controls**: Add note, highlight, underline, strike-out, freehand ink (`create-annotation`). Add link (text field: an address or `page:N`). Insert pages from PDF (`merge-pick`). Replace page from PDF (`replace-page-pick`). Replace page with blank page (`replace-page`). Extract page to PDF (`extract-pages`). Lines 96-136.

## 4. Feature matrix

Status key: **Implemented** means static logic and a UI control exist. **Partial** means a gap or inconsistency exists. **Roster-only** means the verb is declared and dispatchable but has no visible control in the window, inspector or authoring code.

| Feature | Entry point | Expected behavior | Implementing files (under `ED` unless noted) | Existing tests | Status (static) |
|---|---|---|---|---|---|
| Open (import) PDF | Host open; `import_media` | Decode the picked PDF (paged natural decoder, size cap `PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES`) | `ED/🦀️.rs:161-197`, `ED/🦀️.rs:195` (`import_media_as_load`) | `PDF/.../base/🧪️tests/➕️create-minimal-pdf/🦀️.rs`, `.../base/🧪️tests/✏️edit-existing-pdf/🦀️.rs` (base subset) | Implemented (static) |
| New blank PDF | Reference birth | Empty snapshot `blank_pdf_snapshot` | `ED/🦀️.rs:248-250` | Same as above | Implemented (static) |
| Save (native) | Host save; `encode_natural_file` | Encodes via `encode_pdf`, keeps original custody via `retain_file` | `ED/🦀️.rs:167-175`, `ED/💾️native-save/🦀️.rs`, `ED/💾️native-save/🪟️window.rs` | `ED/💾️native-save/🧪️tests/🔬️original-save/🦀️.rs` | Implemented (static) |
| Save-cause diagnostics | Internal | Which subsystem forced a full rewrite (`Memory`, `Control`, `Original`, `Encrypted`, `Root`, `Baseline`, `Retained`, `Tree`, `Document`, `Page`, `Resources`, `Extent`) | `ED/🧭️save-cause/🦀️.rs:5-30` | Not checked | Unverified exposure |
| Extract pages to PDF (download) | Authoring button "Extract page to PDF" | Button extracts the current page only (`(page+1)`). Multi-range `1-3,5` is only reachable through the roster payload. | `ED/🖼️page/✍️authoring/🦀️.rs:374-389, 897-931`, `ED/🖼️page/🪟️windows/🦀️.rs:146` | `ED/🖼️page/🧪️tests/🔬️unit/✍️authoring/🦀️.rs` | **Partial**: UI is single-page only |
| Insert pages from PDF (merge) | Authoring "Insert pages from PDF" | Host file pick, then `merge-document` with resource renaming and link and form retargeting | `ED/🖼️page/📄️document-edits/`, `ED/🖼️page/✍️authoring/🦀️.rs:818-895`, `ED/🖼️page/🎬️actions/🦀️.rs:215-221` (pick effect) | `ED/🖼️page/🧪️tests/🔬️unit/✍️authoring/🧪️authoring.json` | Implemented (static); needs host file picker |
| Replace page from PDF | Authoring "Replace page from PDF" | Same plan with `ImportTarget::Replace` | `ED/🖼️page/✍️authoring/🦀️.rs:695-697` | Same as above | Implemented (static) |
| Replace page with blank | Authoring "Replace page with blank page" | `replace-page` with empty payload | `ED/🖼️page/🎬️actions/🦀️.rs:200` (`replace_page_with`, implementation not read) | Not checked | Unverified |
| Insert page after / remove / move | Page panel buttons; thumbnails drag and child rows | `insert-page`, `remove-page` (guard when count is 1), `move-page` (retargets links, outlines, named destinations, form widgets) | `ED/🖼️page/🎬️actions/🦀️.rs:125-140`, `ED/🖼️page/✍️authoring/🦀️.rs:600-623` | `ED/🖼️page/🧪️tests/🔬️unit/🦀️.rs` | Implemented (static) |
| Page size | Page panel Width and Height | `set-page-size` changes MediaBox only | `ED/🖼️page/🎬️actions/🦀️.rs:141-146` | `PDF/.../base/🧪️tests/...` fixtures | Implemented (static) |
| Page boxes (crop, bleed, trim, art) | Page panel Boundary fields | `set-page-extra` with `box.*` keys | `ED/🖼️page/📄️document-edits/🦀️.rs:86-88` (`set_page_boundary`, not read) | `ED/🖼️page/🧭️geometry/🧭️coordinates/🧪️tests/` | Implemented (static) |
| Document info | Page panel info fields | `set-info-field` per key | `ED/🖼️page/🔎️inspector/🦀️.rs:214-236` | `ED/🖼️page/🧪️tests/🔬️unit/✍️authoring/🧪️field-controls.json` | Implemented (static) |
| `set-info` clear | Action `set-info` (not used by the inspector) | Empty values are ignored, so a field cannot be cleared through this verb | `ED/🖼️page/🎬️actions/🦀️.rs:112-124` | Not checked | **Potential defect** |
| Text edit (run) | Object panel Text field | `set-text` replaces the shown text if the font can show it. Otherwise refuses with `GlyphsMissing`. | `ED/🖼️page/🖱️object-edits/🦀️.rs:35-40` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️text-paint.json`, `text-rendering.json`, `move-text-run.json` | Implemented (static) |
| Text font and size | Object panel Font, Font size | `set-font` rebinds the run to a base-14 or admitted font. `set-font-size` restyles. | `ED/🖼️page/🖱️object-edits/🦀️.rs:41-46, 144-176` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️font-edit.json` | Implemented (static) |
| Move and resize (inspector) | Object panel X, Y, Width, Height | `move` and `resize` translate or scale the object's content operators | `ED/🖼️page/🖱️object-edits/🦀️.rs:49-50` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️image-move.json`, `resize-curve.json` | Implemented (static) |
| Move and resize (canvas drag) | Canvas drag | Not handled by the engine (see the `pdf.page` row above) | `ED/🖼️page/🎬️actions/🦀️.rs:210` | — | **Partial / not implemented** |
| Delete object | Object panel Delete | `delete` removes text, paint or annotation | `ED/🖼️page/🖱️object-edits/🦀️.rs:47-48` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️delete-path.json` | Implemented (static) |
| Fill and stroke | Object panel colour fields | `set-fill` on Text and Vector, `set-stroke` on Vector only | `ED/🖼️page/🖱️object-edits/🦀️.rs:51-52` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️edit-paint.json`, `stroke-style.json` | Implemented (static). The error text for other kinds reads "unknown pdf page action" (misleading). |
| Create annotations | Authoring: note, highlight, underline, strike-out, ink | Creates annotation with appearance stream | `ED/🖼️page/✍️authoring/🦀️.rs:82-139, 141-191` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️annotation-appearance.json`, `✍️authoring/🧪️authoring.json` | Implemented (static) |
| Squiggly annotation | No button | Engine supports it (`ED/🖼️page/✍️authoring/🦀️.rs:94-100`) | — | — | **Partial**: no UI control |
| Link annotation | Authoring "Add link" text field | URL, or `page:N` resolved to a page link | `ED/🖼️page/✍️authoring/🦀️.rs:114-123` | `ED/🖼️page/🧪️tests/🔬️unit/✍️authoring/🧪️field-controls.json` | Implemented (static) |
| Edit annotation contents and rect | Object panel (annotation) | `set-annotation`, `set-annotation-style` (colour), `set-annotation-border` | `ED/🖼️page/🔖️annotation-edits/🦀️.rs`, `ED/🖼️page/🖱️object-edits/🦀️.rs:15-28` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️annotation-appearance.json` | Implemented (static) |
| Fill form field | Object panel "Field value" | `fill-form-field` regenerates appearances for text, button and choice fields. Sets NeedAppearances when incomplete. | `ED/🖼️page/✍️authoring/🦀️.rs:278-371` | `ED/🖼️page/🧪️tests/🔬️unit/🧪️form-paint.json`, `✍️authoring/🧪️field-controls.json` | Implemented (static) |
| Image insert (PNG picker) | Host tool `insert-image` (`ED/🦀️.rs:367`); PNG import pipeline | Picks a PNG, then decodes through the paged PNG graph. Routes by size: inline data URL, paged PNG, or paged DCT. | `ED/📥️local-image/🦀️.rs`, `ED/🖼️image-route/🦀️.rs:1-44`, `ED/🦀️.rs:265` | `ED/🖼️image-route/🦀️.rs:43-44` (inline), `ED/🖼️page/🧪️tests/🔬️unit/🧪️image-paint.json`, `coded-image.json`; `ED/🦀️.rs` line ~2258 test | Implemented (static) |
| Image replace (PNG picker) | Object panel image button (controller `s.stdio.pdf@1.7/*#editor` only) | Same PNG flow with `REPLACE` | `ED/📥️local-image/🦀️.rs`, `ED/🖼️page/🪟️windows/🦀️.rs:285-288` | As above | Implemented (static); **Partial**: subsets get no button |
| Placeholder image (8x8 grey) | Roster `insert-image` path in `insert_object` | Adds an 8x8 grey image | `ED/🖼️page/🖱️object-edits/🦀️.rs:94-99` | — | Dead from UI (roster entry removed; path still in `apply_payload`) |
| Image samples | Roster `set-image` | Replaces raw samples | `ED/🖼️page/🖱️object-edits/🦀️.rs:115-142` | — | **Roster-only** (no control) |
| Image interpolate, intent, decode | Object panel (Image) | `set-resource-detail` | `ED/🖼️page/🔎️inspector/🦀️.rs:47-51` | — | Implemented (static) |
| Page text replacement (command) | `set-page` (typed command, revision-checked) | Replaces the addressed page's text projection | `ED/🦀️.rs:116`, `ED/🖼️page/📝️page-text/🦀️.rs` | — | **Roster-only** (no UI) |
| Document edits without a control | Roster verbs (see list below) | Implemented in apply code, but no control in the window, inspector or authoring code | `ED/🖼️page/🎬️actions/🦀️.rs` | — | **Roster-only** |
| Undo, redo | `mod+z`, `mod+shift+z` (framework) | Framework time-travel history; PDF adds no history code | Framework `⏪️time-travel`, `HUB` 2508, 2565 | Framework tests (not PDF-specific) | Framework-owned; PDF E2E not verified |
| History panel and alternatives | Framework history tab (`kind: "history"` entries in `HUB`) | Checkpoints, alternatives, editing via `historyEdit*` | `HUB` 2519-3992; `finalizeHistoryEdit` dialog `HUB` 6105-6139 | Framework tests | Framework-owned; panel identity not verified |
| Clipboard copy, cut, paste | `mod+c`, `mod+x` (framework, `HUB` 4204-4270) | No PDF handler | None under `ED` | — | **Absent in PDF**: framework verbs with no PDF semantics |
| Presence | Collaborator pointers, carets, selections, viewports | Peer overlays and thumbnail captions | `ED/👥️presence/`, `ED/🖼️page/🪟️windows/🦀️.rs:327-335` | `ED/👥️presence/🧪️tests/🔬️unit/🦀️.rs`, `ED/👥️presence/🧪️tests/🔬️fixtures/🧮️two-collaborators.json` | Implemented (static); live not verified |
| Notices (faults) | Refused edits | Localized fault text (en, de) | `ED/📄️document/🚨️faults/🦀️.rs:122-156` (`pdf_fault_notices`) | Not checked | Implemented (static) |
| Transient view state | Zoom, page, query, match case, active hit | Ephemeral, not persisted | `ED/🫧️transient/🦀️.rs:9-67` | — | Implemented (static) |
| Editor and viewer toggle | Framework | Viewer subsets exist (`👁️viewer`, mode `view`) | Not verified | `ED/../🖨️x/👁️viewer/...` | Unverified |

**Roster-only verbs (declared, dispatchable, no visible control found).** From `ED/🖼️page/🪟️windows/🦀️.rs:49-148` and grepping the inspector, window and object-edit files: `set-page`, `insert-text`, `insert-rectangle`, `insert-line`, `set-image`, `set-annotation-kind`, `set-annotation-markup`, `set-annotation-appearance`, `set-form-settings`, `set-form-field`, `set-form-content`, `set-form-content`, `set-extra-entry`, `set-field-data`, `set-font-reference`, `set-font-metrics`, `set-glyph`, `set-indirect-object`, `set-mesh-reference`, `set-image-mask`, `set-page-transition`, `set-pattern`, `set-graphics-state`, `set-color-space`, `set-properties`, `set-encryption`, `set-optional-content`, `set-embedded-file`, `remove-embedded-file`, `remove-named-destination`, `set-viewer-preferences`, `set-catalog-entry`, `set-trailer-entry`, `set-output-intent`, `set-info`, `set-page-box` (direct). These are reachable only through the command or agent surface. Confirm before treating any as user-facing.

## 5. Subset and 1.4 delta

- **Subsets**: `a`, `e`, `h`, `ua`, `vt`, `x` (`ED/../🗄️a`, `📐️e`, `⚕️h`, `♿️ua`, `🧾️vt`, `🖨️x`) each expand the same kit macro `pdf17_editor!` (`ED/🧰️kit/🧾️pdf17/🦀️.rs`). For example `PDF/.../🖨️x/✏️editor/🦀️.rs:6` and `PDF/.../♿️ua/✏️editor/🦀️.rs:6`. **There is no editor-level action delta.** `set-output-intent` is in the base roster for every subset (`ED/🦀️.rs:358`).
- **Subset-only mutations** (schema or IO only, not editor actions): `set-trim-box` (`🧾️vt/🧬️schema/🧬️mutations/📐️set-trim-box/`), `insert-launch-action` (`🗄️a/🧬️schema/🧬️mutations/🚀️insert-launch-action/` and `🖨️x/🚪️io/.../🚀️insert-launch-action/`), `set-output-intent` (`📐️e/🚪️io/.../🏳️set-output-intent/`, `🖨️x/...`). None of `set-trim-box` or `insert-launch-action` appears in the editor roster.
- **Subset tests**: `PDF/.../🧾️vt/🧪️tests/🧾️mutate-pdf-1-7-vt/🦀️.rs`, `🗄️a/🧪️tests/🗄️mutate-pdf-1-7-a/`, `🖨️x/🧪️tests/🖨️mutate-pdf-1-7-x/`, `📐️e/🧪️tests/📐️mutate-pdf-1-7-e/`, `⚕️h/🧪️tests/⚕️mutate-pdf-1-7-h/`, `♿️ua/🧪️tests/♿️mutate-pdf-1-7-ua/`.
- **1.4**: no Rust editor roster was found under `PDF/🏅️standards/4️⃣1.4`. The editor folders contain `.empty.md` placeholders and `🟦️.ts` files. The 1.4 root `🦀️.rs` was not read.

## 6. Dev route and playground

- Dev variant entry: `DEV/🚀️entry/🟦️.ts`. It requires `VITE_SEMIO_PLUGIN === "stdio-pdf-1-7"`, and boots `bootFrameworkOsDev`.
- Route fixture: `DEV/🧪️tests/🚪️route/🧫️fixtures/🔣️.json` sets `"variant": "stdio-pdf-1-7"` and `"reactPort": 6260`.
- Route test: `DEV/🧪️tests/🚪️route/🟦️.ts`.
- Catalog rows: `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧫️fixtures/🔣️.json` lines 174-180 list `stdio-pdf-1-7`, `-a`, `-e`, `-h`, `-ua`, `-vt`, `-x`. There is no 1.4 row.

## 7. Partial, unreachable or inconsistent items

1. **Canvas drag move and resize are not implemented in the engine.** `canvasPointerMove` and `canvasPointerUp` are no-ops (`ED/🖼️page/🎬️actions/🦀️.rs:210`).
2. **Roster-only verbs** (listed in section 4). Most are dispatchable but have no visible control.
3. **Squiggly annotation** has engine support but no UI button (`ED/🖼️page/✍️authoring/🦀️.rs:94-100`).
4. **Extract** is single-page from the UI. Range selection is only available through the roster payload (`ED/🖼️page/✍️authoring/🦀️.rs:374-389`).
5. **`set-info` cannot clear fields.** Empty values are skipped (`ED/🖼️page/🎬️actions/🦀️.rs:112-124`). Confirm which verb the inspector actually uses.
6. **Image replace button is 1.7-base only.** It is gated on the exact controller `s.stdio.pdf@1.7/*#editor` (`ED/🖼️page/🪟️windows/🦀️.rs:285`), so subset object panels miss it.
7. **Config schema is unwired.** `ED/🎚️config/🧬️schema/` exists, but the editor is `NoConfig` (`ED/🦀️.rs:126`).
8. **Unused single-window layout.** `edit::layout()` (`ED/🎭️modes/✏️edit/🦀️.rs:22`) is never called.
9. **Hub layout may be stale.** `HUB` lines 6080-6098 describe a different window set from the descriptor's `document_layout`.
10. **Clipboard**: `copy`, `cut` and `paste` are framework verbs in `HUB` with no PDF semantics. `paste` / `mod+v` was not confirmed in `HUB`.
11. **Misleading error text** for `set-fill` on Image, Form or Shading objects (`ED/🖼️page/🖱️object-edits/🦀️.rs:53`).
12. **Placeholder 8x8 image path** is still reachable inside `insert_object` (`ED/🖼️page/🖱️object-edits/🦀️.rs:94-99`) even though the roster entry was removed.

## 8. Unverified or blocked

- No runtime, E2E, save-reopen or export checks were run. Every status above is static.
- Shell exposure of zoom, search, Editor/Viewer toggle, history panel tab, and any toolbar or menu entries. None are in the PDF source.
- Behavior of the framework history and clipboard verbs for PDF objects.
- Image routes beyond the inline lane (paged PNG and DCT), and the `PdfGraphSaveJob` save path.
- `replace-page` with a blank payload, `set-page-box` and `set-info-field` implementations (not read).
- The `🧪️tests/🔭️pdfjs-page/🟦️.ts` render test and the `🧭️geometry/🧭️coordinates/🧪️tests/oracle.test.ts` oracle test were not run.