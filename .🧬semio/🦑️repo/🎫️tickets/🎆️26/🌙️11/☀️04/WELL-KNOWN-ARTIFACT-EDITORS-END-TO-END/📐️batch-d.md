# Batch D (svg, dxf, dwg, pdf) - T10

Status: edits written; compile and test results are NOT yet in (queued, see "Verification"). Nothing below is claimed as passing.

## Surface model
Editors are per-subset `#editor`/`#viewer` apps published by hub family plugins (`🌎️hub/…/🧩️extensions/{🛠️cad,📘️pdf,🖼️image}`); the window code lives in the subset `✏️editor`/`👁️viewer` dirs and renders through window kits only.

## Findings and changes per artifact
### svg
- All three subsets (`base`, `basic`, `tiny`) already had image-kit render, Details edits (`special -> tree_edit`) and a natural-file codec. No change.
- OPEN (per `📙️recipe-edit-rules.md`, not started): element rename (`set_element_name` in the tree edit kit plus schema mutation), typed `set-transform` / `set-view-box`, `strip/restore-non-tiny` and `stamp-base-profile` as editor actions with en/de labels. These need schema-first mutation additions and generated twins.

### dxf (r12 header subset, the only DXF surface)
- Main window was a placeholder (MeshWindowKit boxes). Now a real 2D ImageWindowKit render: shared pure projection `🚪️io/📝️text/📸️snapshot/🖼️preview/🦀️.rs` (`preview_svg`): LINE, CIRCLE, ARC, POLYLINE with bulges, TEXT, SOLID, INSERT with nested block expansion (depth 8). Editor and viewer windows both use it (viewer is read-only).
- Natural-file codec `s.stdio.dxf@r12` (`.dxf`, `image/vnd.dxf`) via `print_dxf_document` / `parse_dxf_document`; `import_media_as_load`.
- Existing edits kept: layers, styles, linetypes, header vars, entities, blocks (set/insert/remove through the edit rules).
- Tests: editor window preview test (entity kinds, edit changes output); natural-file round trip with a coordinate edit; invalid-file refusal; third-party cross-check: new `oracle_project_entities` in the dxf oracle (the `dxf` 0.6 crate parses the saved bytes, asserted to hold the edited line and the unedited circle). ezdxf is not installed; the `dxf` crate is the independent parser.

### dwg (ac1018, ac1024)
- ac1018 note: its snapshot and mutation types are re-exports of the ac1024 types (`4️⃣ac1018/…/🧬️schema/📸️snapshot/🦀️.rs` is `pub use …v_ac1024…::*`), and the ac1018 io owns no encode/decode of its own (its raw serializers call the ac1024 `encode_dwg`/`decode_dwg`). The ac1018 editor therefore correctly uses the ac1024 codec functions.
- Natural-file codecs `s.stdio.dwg@ac1018` and `@ac1024` (`.dwg`, `image/vnd.dwg`) over `decode_dwg` / `encode_dwg`; failures surface as `MediaError::Payload`.
- Main windows (editor and viewer, both standards) now render the typed geometry through ImageWindowKit; shared pure projection `…/📸️snapshot/✏️drawing/🖼️preview/🦀️.rs` draws the 11 typed kinds (point, line, circle, arc, ellipse, lwpolyline, spline, text, 3dface, polyline3d, polyface mesh).
- Typed entity coverage (unchanged by me, from the earlier roundtrip ticket): 11 geometry kinds in the logical view; the object reader types 18 geometry entities of 652 frames; untyped objects are preserved through the logical model per the no-raw-bytes rule.
- Entity editing: entity edits route through `set-drawing` (whole drawing value). New test `moving_a_line_endpoint_edits_the_drawing_object_and_round_trips_through_a_dwg_file` locates a real line in `architectural.dwg`, edits one coordinate, saves and reopens. Its result decides whether the writer must be extended; if it fails the failure message names the writer gap.
- OPEN (recipe): dedicated layer/object edit kinds and rules (`/drawing/layers/<n>` set/insert/remove keyed by name, `/drawing/objects/<n>/geometry`) need new mutation kinds in the ac1024 mutations schema; not started.

### pdf
- 1.7 base: page editor and natural-file codec were already complete (lopdf cross-check in the existing test).
- 1.7 subsets a, e, h, ua, vt, x: natural-file codec added, subset-qualified format kinds `s.stdio.pdf@1.7/<subset>` (base keeps `s.stdio.pdf@1.7` and owns plain `.pdf` resolution).
- 1.4 base, a, x: natural-file codec `s.stdio.pdf@1.4` (`/a`, `/x`) added after wiring `semio-s-artifact-stdio-pdf-schema-5` to `semio-s-artifact-stdio-pdf-graph-projection` (clean downward dependency, no adapter).
- Loss decision: the 1.4 model is size plus text only. `decode_pdf_lossless` opens a file only if re-encoding reproduces it byte for byte (its own output); anything else is refused with a message naming the PDF 1.7 editor, so saving can never silently drop fonts, images, graphics, annotations or metadata. The 1.7 COS model is the lossless path (other objects retained, see the base lopdf test).
- Tests: every 1.7 subset opens and saves the same bytes as the base and `qpdf --check` accepts them; 1.4 own-output round trip plus `qpdf --check`, foreign PDF refused.
- `.expect("variant spec must exist…")` in the ten PDF editors replaced by a non-panicking keyword fallback in `print_op` (infallible signature).

## Shared fixture
`🧰️framework/…/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json` gained 12 rows (dxf, dwg x2, pdf 1.7 subsets x6, pdf 1.4 x3). The count assertion no longer hard-codes a number: it checks every app id is declared once and that a named batch-D set (`BATCH_D_NATURAL_FILE_APPS`) is present, so concurrent additions by other batches do not conflict.

## Verification
- Cargo (shared store): `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-dxf-tests -p semio-s-artifact-stdio-dxf-mutations --lib editor`, log `🗑️generated/d-dxf-editor-shared.log`: queued behind the capacity gate, no result yet. Not yet run: dwg editor crates (ac1018, ac1024), pdf schema-5 crate, framework `natural-file-lifecycle` test.
- Expect a Cargo.lock refresh in the dxf, dwg and pdf workspaces (new path dependencies).

## Remaining work
1. Results of the queued runs; fix compile errors if any.
2. DWG writer/entity coverage if the line-endpoint test fails; layer/object edit kinds.
3. svg element rename, typed transform/view-box edits, profile actions.
4. Shared vector-preview module (DXF and DWG each carry their own small SVG helpers).

## Later additions
- DXF demo asset: `🖋️dxf/…/📰️header/📚️examples/🎬️demo/🖼️assets/🧪️example/🖊️.dxf` replaced (was placeholder text) by a real hand-written R12 drawing (layers 0 and WALLS, CONTINUOUS linetype, two lines, one circle). Editor test `demo_dxf_is_a_real_drawing_that_the_reference_parser_reads_identically` compares our reading with the `dxf` crate reading (`oracle_project_entities`). No snapshot embeds this file (dsl/pack demo snapshots are the typed model, unchanged).
- PDF text layer (1.7, all seven subsets): window `pdf.text-layer` (BlockList, en/de) in `✏️editor/🖼️page/♿️text-layer/🦀️.rs`, added to the layout and to every 1.7 editor. Lists title, language, tagged flag, then each page's objects in content-stream order (the model has no typed structure tree). Test `text_layer_lists_a_real_document_in_reading_order_with_localized_labels` on `two-pages.pdf`.
- 1.4 editors: no text layer added on purpose; their page window is a `DocumentWindowKit` text document, already a textual surface.
- Canvas accessible label (`render_windowed` locale): NOT done; `scene_surface` takes no label and I did not find a supported way to attach one without framework changes.
- DWG layer/object edit kinds, SVG rename/transform/view-box/profile actions, shared vector-preview module: NOT started. Reasons: each new mutation kind touches the leaf, two JSON schemas, proto, graphql, TS twin, oracle catalog, the mutation roster, the diff (`DwgDiff.drawing` is a whole-block replace, so layer-granular diffs need a new sub-diff type) and the hub generators (`composition-prepare`, `native-codec-projection`), which must run through the T06 queue and could not be validated while the shared build is queued. The vector-preview extraction would move code across crates that currently compile unverified.

## Deferred schema work: exact footprint (no half edits were left in the tree)
I did not start these because a partial edit would break the roster/catalog laws and I cannot validate hand-synced twins without a build. Per-item footprint measured by grepping the last existing kind (`set-application-history`):
- DWG layer/object kinds (`set-layer`, `insert-layer`, `remove-layer`, object geometry): per kind a leaf dir (`🦀️.rs`, `🔣️.json`, `🧬️schema/🔣️.json`), then the shared `🧬️mutations/{🦀️.rs (enum, kind(), KINDS, demo cases), 🔣️.json, 🛰️.proto, 🔗️.graphql, 🟦️.ts}`, the binary protocol `🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio` (record tag) and its ABNF/Kaitai/Spicy twins, the text grammar twins (`🅰️.g4`, `🔤️.ebnf`, `📖️.grammar.semio`), the two oracle catalogs (ac1024 and ac1018 `🔮️oracles/🔣️.json`), the edit rules, and the framework schema index/catalog. The diff: `DwgDiff.drawing` is a whole-block `Option<DwgLogicalDrawing>`; a layer sub-diff adds a diff type to `🔺️diff/{🦀️.rs,🔣️.json,🛰️.proto,🔗️.graphql,🟦️.ts}` and the diff codecs. Note `DwgLogicalDrawing.layers` is not the layer authority for the writer (layer table records are objects), so a layer kind must edit both the vector and the matching `DwgLayerTableRecord` object.
- Interim reality: pointers under `/drawing` already raise `set-drawing` (whole drawing value); the ac1024 test `moving_a_line_endpoint_edits_the_drawing_object_and_round_trips_through_a_dwg_file` exercises it.
- SVG basic/tiny rename/transform/view-box/profile actions: same pattern in the svg mutation set plus `TreeEditKit::set_element_name`.
- Canvas accessible label: needs a new optional label field on `Canvas2dScene` (`🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🔲️canvas2d-scene`), its JSON schema and TS decode, and the React host emitting `role="img"` plus `aria-label` and `aria-describedby` to the `pdf.text-layer` body; shared framework surface, needs an owner decision to avoid colliding with other implementers.
- Shared vector-preview module: do last, after DXF and DWG compile.

## Transcribed evidence (T20, before the `🗑️generated` deletion)

Full text of each entry is in `✅️validation.md` (E5 and E6). Summary for the batch:
- `d-dxf-editor-baseline.log`: command `repo cargo` for dxf tests, cancelled by SIGTERM; exit not recorded; no result.
- `d-dxf-editor-shared.log`: `cargo test -p semio-s-artifact-stdio-dxf-tests -p semio-s-artifact-stdio-dxf-mutations --lib editor`; 461 `Compiling` lines, 0 `error[E…]`, warnings only; still compiling when last read; no pass/fail counts.
- Contract (offline) for dxf: passing in E1 of `✅️validation.md` (the 194-test contract run; dxf is not among its two failures).
- Not an editor e2e result.

## Decisions round (coordinator): DWG writer, SVG, canvas label
- DWG new layer/object kinds: dropped as decided. Writer consistency instead: the writer already materializes every object frame from `drawing.objects` (objects are the authority; `encode_r2010_object_frame` covers table records, line, arc, lwpolyline, block begin/end, insert, dimension, viewport and a generic body frame for the other typed entities), so an entity edit through `set-drawing` is written by construction. The inconsistent path was the editable `drawing.layers` view: the writer ignored it, so a layer rename was silently lost on save. Fixed: `DwgLogicalDrawing::layers_applied` (`…/📸️snapshot/✏️drawing/🦀️.rs`) pairs listed layers to layer table records (by name first, then in order) and writes edited names/colors into the matching `DwgLayerTableRecord`; `encode_dwg` (`…/🚪️io/📤️export/🦀️.rs`) applies it first, and refuses a layer count that differs from the record count with an accurate message (adding/removing layers needs a new record and handle and is not writable yet). Tests (ac1024 editor): layer rename round trip, unbacked layer refused, line endpoint round trip.
- Typed entity coverage numbers are unchanged (11 geometry kinds in the logical view; 18/652 frames typed by the object reader per the earlier ticket). No new frames were added; whether more are needed depends on the first real run of the line-endpoint test.
- SVG basic and tiny: `TreeEditKit` (`🧱️base/…/🧬️mutation-support/🦀️.rs`) gained `set_transform`, `set_view_box`, `rename_element` (defaults keep base unchanged) and an `attribute_row` that routes typed `transform`/`viewBox` values to the typed kinds. Basic and tiny kits implement them in their `🧭️edit-rules`; rename is composed of the subset's own gated `remove-element` + `insert-*-element` rows, so a rename to an excluded element name is refused by the profile gate and undo works row by row. No new mutation kind or twin file was needed. Tests added in the basic mutation tests (rename, typed transform).
- NOT done: `strip/restore-non-tiny` and `stamp-base-profile` as editor actions (needs command variants, action definitions and retained-tool-job proofs in the tiny/basic editors; the existing `SetPixelRegion` command is also not wired to an action).
- NOT done: canvas accessible label. `Canvas2dScene` is built by struct literal in many crates and carries an explicit pack wire (`scene_pack_wire!`), so an optional label field touches every construction site, the wire schema, the JSON schema and the TS decode plus host; changing it blind would break other implementers. Needs an owner/time window; design: optional `label: Option<String>` plus `described_by: Option<String>` in the spine, TS host sets `role="img"`, `aria-label`, `aria-describedby="pdf-text-layer"`.
- NOT done: shared vector-preview extraction (after the crates compile).

## First compile wave (DXF) and DWG layer insert/remove
- DXF run 1 (`🗑️generated/d-dxf-editor-shared.log`) compiled the whole dependency chain and failed in the `semio-s-artifact-stdio-dxf-tests` lib test with 11 errors: 10 x `DxfDiff::absorb` (trait `ColdMutationDiff` not in scope in the existing mutations unit test, not my code) and 1 x `ImageWindowKit::KIND_ID` (trait `WindowKit` not in scope in my viewer window test). Fixed: import `ColdMutationDiff` in `🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`, import `WindowKit` in all six window tests (dxf editor/viewer, dwg ac1018/ac1024 editor/viewer). Rerun started: `🗑️generated/d-dxf-editor-shared2.log`.
- DWG layer insert/remove (coordinator decision): `DwgLogicalDrawing::layers_applied` now also adds a new `LAYER` table record at the next free handle (modelled on an existing record, owner = the layer control, listed in the control's `entry_handles`) and deletes a record plus its control entry for a layer that was removed from the list; refuses layer `0` and any layer an object still owns/references. `encode_dwg` raises the header and auxiliary-header `handle_seed` to the new drawing seed. Test `adding_and_removing_a_layer_writes_and_deletes_its_table_record` (real fixture, reader round trip). Unverified until the DWG crates compile.
- SVG profile actions (strip/restore-non-tiny, stamp-base-profile): NOT done yet. They need the retained tool-job factory (`SvgTinyEditorExampleFactory` is bound to the example tool ids and `snapshot_editing_bounded_first_step_tool_proofs!`), new command variants, `command_from_action`, `command_id`, publication contracts and en/de action definitions; to be done after the DXF/DWG compile wave so the factory contract can be checked against a green build of the sibling editors.
- Canvas label and vector-preview extraction: sequenced after the compile wave per coordinator.

## Review fixes (DWG layers)
- Duplicate tests/helper in the ac1024 editor unit test removed (my script had appended a second copy).
- Pairing: unmatched layers now pair with an unmatched record only at the same position (in-place rename); anything else is add + delete, so deleting a used layer is refused ("still used") instead of silently re-owning its entities. Layer `0` can be neither renamed nor deleted. New record handles start at `max(object seed, header seed)` (`layers_applied(handle_floor)`), so a freed maximum handle is not reused.
- Tests: rename asserts the record handle is unchanged; add/remove asserts the layer control entry (and its removal) and a fresh handle; new tests for add-plus-delete of a used layer, layer 0 rename, and no handle reuse.
- Not changed: the `WindowKit` imports in the DXF editor/viewer window tests. The first compile run failed without it (`ImageWindowKit::KIND_ID` is a trait constant, and the glob import of `main::*` does not re-export the window file's private import).

## SVG tiny profile actions (written, unbuilt)
`strip-non-tiny` (`stripNonTiny`) and `stamp-base-profile` (`stampBaseProfile`, args `baseProfile`, `version`; empty clears) are now editor actions of the tiny editor with en/de labels and descriptions, through the existing retained-tool factory: new command variants, ids in the tool/command id lists, publication contracts, proofs `tools` list, `command_from_action`, `handle`, retained reducer, and a static tool id per command in `build_tool_job`. `restore-non-tiny` is deliberately not an action: its payload is the list of rows a strip removed, so it exists only as the inverse of a strip (history undo), not as a user-initiated gesture. The basic editor has no strip/restore; `stamp-base-profile` for basic is not yet exposed (same pattern, next). Unit test `profile_actions_resolve_to_their_commands_and_typed_mutations`.
