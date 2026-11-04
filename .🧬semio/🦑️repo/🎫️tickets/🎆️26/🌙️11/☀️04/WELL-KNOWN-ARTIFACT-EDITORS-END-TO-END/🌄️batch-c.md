# Batch C: png, jpg, bmp, gif, tiff (T09)

Paths are under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/` unless stated. Nothing below has been compiled or run except the TS tests. The one chained cargo run is `<ticket>/🧊️batch-c-scripts/run-batch-c.sh` (summary in `🗑️generated/c-summary.txt`, per-package logs `c-<package>.txt`); it was still queued when this was written.

## Architecture
Image editors are per-subset `#editor` surfaces on `ImageWindowKit`, published by the hub image family plugin. `🖨️raster` is a separate `RasterSnapshot` artifact; no shared codec code.

## Status

| Artifact | Natural file | Native receipt | Editing |
| --- | --- | --- | --- |
| png | existing | existing | existing paint commands; new test proves tEXt, eXIf and iCCP survive a pixel edit and export (png crate cross-check) and undo restores exact bytes |
| jpg (document, baseline) | existing, independent oracle test exists | existing | NEW `paint-region` on the decoded RGBA raster (revision-free `replace-pixels`, rows painted in real 64-row chunks, cancellable); window shows a localized (en/de) notice that saving re-encodes the lossy scan while EXIF/ICC segments are preserved; dead `SetPixelRegion` variant removed |
| bmp | existing | NEW `stdio.native.bmp.v1` plus schema `codecs[]` entry | paint-indexed-region and paint-direct-region on an editable window; sample edits via `snapshot_edit_special`; NEW palette mutations set, insert and remove (tags 14, 15, 16, see below); demo `🖼️.bmp` replaced by a real 78-byte BMP v3 matching the demo DSL (generator `🧊️batch-c-scripts/generate_bmp_demo.py`, decode test added) |
| gif (87a, 89a) | NEW | NEW `stdio.native.gif.87a.v1`, `stdio.native.gif.89a.v1`, two `codecs[]` entries; 89a capability id no longer names 87a | NEW per-frame `paint-region` (frame, x, y, width, height, palette index) producing `set-image-pixels` (87a) or `set-frame-pixels` (89a); dead `SetPixelRegion` removed |
| tiff (document, baseline) | NEW; document `s.stdio.tiff@6.0`, baseline `s.stdio.tiff@6.0/baseline` (owner unqualified, other subset qualified; T01 to confirm owner) | existing | document `paint-region` is a window action on an editable window and completes in one real turn (no fake per-row progress) |

## Keyboard painting (Paint2dHost), vitest-first
- Pure module `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/⌨️keyboard/🟦️.ts` plus 12 tests in `…/🧑‍🎨engine/🧪️tests/⌨️paint-keyboard/🟦️.ts`; run with `HOST_TEST_ROOT=<engine dir> node node_modules/.bin/vitest run --config <ticket>/vitest.batch-c.config.mjs`: 12 passed.
- Overlay wiring in `✍️editing/🟦️.tsx` (esbuild syntax check only; no type check, no component test yet): arrow-key pixel cursor, Shift+arrows marquee, Enter/Space stamp (one `paintStroke` commit per stamp), Enter on bucket/wand/picker, Enter on a marquee selects, `+ - 0` zoom and arrows pan when no painting tool is active (the overlay is always focusable and pointer-transparent in the layers tool), live-region cursor announcement (en/de), keyboard help text.
- Stroke cap: brush and eraser strokes roll over at 2048 buffered points by streaming the unsent points and continuing from the last point inside the same gesture (one history step); selection paths (lasso) warn at 90 percent and refuse at the cap with a localized message.

## bmp palette-entry mutations
Hand-authored through `🧊️batch-c-scripts/add_bmp_palette_mutations.py` (inputs, not generated output).
- Kinds: `set-palette-entry {index, red, green, blue, reserved}`, `insert-palette-entry {same plus colorsUsed}`, `remove-palette-entry {index, colorsUsed}`. Insert shifts every sample index at or above the position so the picture is unchanged; remove refuses an entry any sample uses; `colorsUsed` is the explicit resulting header value (0 means the profile maximum), so inverses restore the snapshot exactly (insert inverts to remove carrying the base value, remove inverts to insert). Diffs carry the whole edited image (`BmpDiff.image`), validated through `BmpImage::validate`.
- Wired: three schema leaves under `🧬️schema/🧬️mutations/{🌈️set,📥️insert,📤️remove}-palette-entry` (rust, descriptor json, payload schema json, ts, graphql, proto); aggregate enum and re-exports, aggregate json, graphql, proto and ts; canonical traversal arms; package mirror mounts; helpers `palette_entry_set`, `palette_entry_inserted`, `palette_entry_removed` in `🧬️schema/⚙️operations/🦀️.rs`; binary and text leaf codecs, both registries, `📡️.protocol.semio` records and `📖️.grammar.semio`; roster entries in `direct-mutation-contract/🔣️.json`; Details gestures in the editor (`/image/palette/<n>/<r|g|b|reserved>` set, `/image/palette/<n or -` insert, `/image/palette/<n>` remove).
- Tests (unrun): wire-form round trips, exact undo for set, insert and remove, refusal of used, absent and direct targets (`🧪️tests/🧬️mutation-regressions/🦀️.rs`), Details gesture mapping (editor unit tests).
- Left to the T06 generator pass: derived grammar twins (spicy, abnf, ksy, g4, ebnf), oracle roster and history-edit vectors (no third-party `image` cross-check for palette edits yet), hub component json and descriptor, schema catalogs.

## Review items
- Done: window actions use `extend` or `push` (kit `set-pixel-region` kept) for bmp, tiff, jpg, gif; tiff fake progress removed (test updated); tiff baseline format kind qualified; gif 89a capability id fixed.
- Accepted by coordinator: paint actions registered at window level and editor level, as png does.

## Open items
- `ImageView` document name and locale: APPROVED, sequenced into the post-first-compile-wave migration slot with T01's multi-format and T04's MediaError migrations (plan below).
- Gif pack hash: when the run prints the two values from `native_receipt_codecs_carry_the_pack_hash_of_the_registered_document_codecs`, JSON and `native_codec_*` must follow the registered value.
- Generators (T06): `composition-prepare` and `native-codec-projection` for bmp and gif (hub component json and `.descriptor.semio` still name the old gif 89a capability id); natural-file fixture has 4 new rows (count 55).

## ImageView document name and locale: migration plan and site list

Goal: window renderers receive the document display name and the UI locale through the render context; `ImageView` carries both; alt text becomes localized `<name> — PNG image, W×H px` (de: `<name> — PNG-Bild, W×H px`).

Order (each step compiles and tests on its own, schema-first):
1. Schema: add `documentName` (optional string, max 128 UTF-8 bytes, clipped by the host, never persisted) to the view-state schema `🧰️framework/🔨️modules/🛂️manifest/👁️view-state-and-public-invocation/🟦️.ts` (allow-list at line 155 plus its long-string capacity budget: `VIEW_CONTEXT_LONG_STRING_FIELDS` and `MAX_SURFACE_VIEW_CONTEXT_BYTES` raised by one field and 128 bytes) and the JSON schema twin next to it.
2. Rust contract: `ViewModel.document_name: Option<String>` in `🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs` with `#[value(skip_serializing_if = "Option::is_none")]`, set to `None` in `ViewModel::new`; update `🧪️tests/🔬️view-context-capacity/🦀️.rs`.
3. Explicit `ViewModel { .. }` literals that list every field (no `..Default` or `..ViewModel::new`) and need `document_name: None`: the 23 files other than the contract itself in the list below (found by `tree_viewport_rows:` in `*.rs`).
4. Host population: the ShellHost builds the view state where it sets `locale` and `terminology` (same module family as `🛠️ShellHelpers/🌳️tree-windows`); it reads the open document's display name from the document attachment and clips it. Test next to `🛠️ShellHelpers/🧪️tests/🪟️tree-windows`.
5. Window kit: `ImageView` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🦀️.rs` gains `name: String` and `locale: Locale` (no defaults, no compat). `image_alt_text` formats `<name> — <FORMAT> <image|Bild>, W×H px` per locale, with `FORMAT` from the mime. Unit tests for en and de plus a name that needs clipping.
6. Window renderers: every `render(snapshot[, locale])` becomes `render(snapshot, context)` where the editor `render(body_key, doc, cfg, view_state)` passes `view_state.locale` and `view_state.document_name`; the 41 `ImageView { .. }` literals below get `name` and `locale`. Windows owned by other batches (svg, dxf, dwg, raster) get narrow edits after rereading.
7. Run: window-kit tests, view-context capacity law, shell tree-windows tests, then each artifact's window unit tests.

`ImageView { .. }` literal sites (29 files, 39 literals, plus 2 in the plugin window-kit test `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs` = 41), all `…/🪟️windows/🪟️main/🦀️.rs` unless noted, under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`:
- png 1.2 any: viewer, editor (1 each)
- jpg jfif-1.01 document and baseline: viewer, editor (2 each, 8 total)
- bmp v3 any: viewer, editor (1 each)
- gif 87a any and 89a base: viewer, editor (2 each, 8 total)
- tiff 6.0 document and baseline: viewer, editor (1 each, 4 total)
- svg 1.1 base, basic, tiny: viewer, editor (1 each, 6 total) (batch D)
- dxf r12 header: viewer, editor (1 each) (batch D)
- dwg ac1018 viewer, editor; ac1024 viewer, editor (1 each) (batch D)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs` (3)

Explicit `ViewModel` literal files (24 including the contract): `🧰️framework/🔨️modules/🛂️manifest/{🖼️view-model/🦀️.rs, 🧪️tests/🔬️view-context-capacity/🦀️.rs}`; `…/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs`; five wgpu Shell staging files (`🪝️window-actions/🧩️session-and-topology`, `🔲️windows/📨️command-dispatch`, `🔀️actions/🧩️session-and-topology`, `🔀️actions/📨️command-dispatch/📨️dispatch-app-switch`); stdio `🧾️json`, `📰️xml` main window tests, `🎒️zip/✏️editor/🧪️tests`, `📇️registry/🧬️contract/✏️editing/🪟️details/🧪️tests/🔬️unit/🌳️collection-and-table-laws`; plugin tests of cad, forms (4 files), layout, shooting, gis, fem 2d and 3d, norm (`🖥️app-surface` 2 files and `📇️iso16757` inputs), playbook steps.

## bmp palette third-party cross-check
- The existing differential feature `🧪️tests/🪟️mutate-bmp-v3/🥒️.feature` had empty example tables for the small indexed document; it now carries `set-palette-entry`, `insert-palette-entry` and `remove-palette-entry` rows for both the mutation and the undo outlines. The oracle (`🔮️oracles/🦀️.rs`) implements the three kinds independently (own model, image-rs decode as the visual witness) and its own unit test checks that an insert keeps the image-rs visual pixels identical, a remove restores the original projection, a recolor changes only the recolored entry's pixels and a used entry cannot be removed. Oracle manifest (`🔮️oracles/🔣️.json`) lists the three kinds.
- The existing `image` 0.25.10 (bmp) is already in the bmp `Cargo.lock` through the oracle crate, so no new dependency was needed.
