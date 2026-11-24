# PDF multi-resource pages and worker jobs (2026-11-24)

Native evidence only. Every cargo run went through `native-cargo.sh` (global gate, one cargo, watchdog `watchdog-cargo.py`), always filtered; no wasm rebuild. Inputs kept in this folder: `multi-resource-fixture.py` (writes the committed fixtures), `multi-resource-oracle.py` (pymupdf oracle), `controlled-barrier-audit.py` (real-world barrier audit), `run-pdf-multi.sh`, `run-retry.sh`, `run-multi-regression.sh`.

## Result

| Test (`editor::pdf17`) | Before | Now | Cause |
| --- | --- | --- | --- |
| `registered_pdf_font_change_preserves_encoded_text_and_the_next_line` | `ReferenceRequired` for Resources | passes | the controlled lifter qualified one font per page and only base encodings; now several fonts, `/Differences`, MacRoman |
| `controlled_pdf_terminal_publication_requires_both_owned_headers` | `original page allocation attempt is occupied` | passes | `close_chained` ran a second allocating close action inside one effect |
| `registered_pdf_page_and_authoring_actions_undo_and_redo_step_by_step` | `replace-page` in `Worker` for 32,768 turns | passes | two causes below (contract fuel, terminal frame witness) |
| `registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit` | `insert-image` in `Worker` for 32,768 turns | still fails, much further along (first settlement) | worker causes fixed; the picker effect waits for a reactor step the fixture host does not run, and the test asserts a superseded wire (below) |
| new: `registered_pdf_with_several_fonts_images_and_inherited_resources_opens_like_both_reference_readers` | n/a | passes | oracle test of the multi-resource lifter |
| new: `simple_type1_differences_and_mac_roman_decode_like_the_reference_codec` | n/a | passes | codec oracle for the new text decode |

## 1. Multi-resource pages in the controlled lifter

### Cause

`resource_bindings` (`🔨️modules/⬇️lift/.../🧭️controlled/🦀️.rs`) accepted a resource dictionary of at most two keys, exactly one `/Font` entry and one `/XObject` entry (`single_resource`), and `ControlledFontResources` / `ControlledImageResources` bound one font and one image to a page (`page_font`, `page_image`). Anything else answered `Barrier::Resources`, the decoder answered `ReferenceRequired`, and the registered Open route has no reference reader, so the browser could not open a PDF with two fonts on a page. The font admission additionally qualified only name encodings (`StandardEncoding`, `WinAnsiEncoding`) and decoded text through static tables, so `/Differences` and `MacRomanEncoding` were barriers too, and only one embedded font program per document was admitted (`self.embedded.is_none()`).

### Design (one page, any number of resources, resumable under turn fuel)

- `resource_shape` qualifies a resource dictionary (direct or indirect, own or inherited through the page tree) with the categories `Font`, `XObject`, `ExtGState`, `Properties` and `ProcSet` (ignored like the cold lifter); every category sub-dictionary is direct, at most 32 entries, every entry an indirect object with a unique authored name of at most 128 bytes.
- The stage machine walks the entries one work unit at a time: `Stage::Entries { location, category, index }` reads entry `index` (an exact re-inspection, nothing is held between units), then hands it to `Stage::Font`, `Stage::Image` (existing resumable image cursor) or `Stage::State`. Order is the cold lifter's order: Font, XObject, ExtGState, Properties, each in dictionary order, so ids, vectors and roles equal `decode_pdf`.
- Per page the cursor holds `PageFonts` (32 slot bytes) and the image resources hold 32 page slot bytes; the typed fonts and images stay the document's own vectors, a name resolves by comparing the typed id with the authored name (ids equal authored names; a different object under a used name still defers, see limits). `PageFontTable` resolves every `Tf` and every text operator per page against all bound fonts, `PaintXObject` against all bound images.
- Inherited resources: the page's `inheritance.resources` attribute is processed at the page, the role check at the content ancestry compares the resolved resource location with the one the page used (`page_resources`), so a content role never depends on a different dictionary than the page was lifted from. Branch resources are lifted before the first page only (cold order); a branch with entries after a page defers.
- Bounds per allocation: font and image tables reserve 32 slots once under the allocation quote, the role table reserves `pages + 32 + 2 * 32`, every graphics state / property list is quoted (`size_of` + name + text bytes) before it is built, `/Differences` are quoted (count, glyph bytes) before they are copied; one unit does at most one entry, the dependency list of a content role is deduplicated in place without allocation beyond its quoted capacity.
- Images: the image role is inserted before the content roles of earlier pages (`role_target = roles - content_roles`), which keeps the cold order `[resource roles, content roles]` for several images.
- Content role dependencies equal the cold `dependencies_of` order: contents identity, font dictionary identity, content owners, every page font in dictionary order, remaining content owners, the dependencies of the page fonts, then the other document fonts sorted by object with their dependencies (embedded fonts now keep one dependency set per slot).
- Fonts: `/Encoding` may be `StandardEncoding`, `WinAnsiEncoding`, `MacRomanEncoding` or a direct encoding dictionary (`Type`, `BaseEncoding`, `Differences` with at most 128 glyphs of at most 64 bytes). Text decodes through `controlled_simple_text(base, differences, byte)`: the base table refined by the differences, a glyph shown by an earlier code stays raw so the text re-encodes to one code (the old rule), glyph to text through the Adobe Glyph List. `FontTable::controlled_type1_text` is the new decode hook; the content program takes the font table into its scan and copy phases.
- Graphics states: a direct-scalar `ExtGState` (keys `Type LW LC LJ ML RI OP op OPM BM CA ca AIS SA FL SM TK`, no arrays, no references) is built with the cold `lift_ext_g_state`; a property list of atomic values with `PdfNamedProperties`. Both are admitted once per exact owner.

### Limits (what still answers `ReferenceRequired`, named barrier `Resources`)

Form XObjects, patterns, shadings, colour space resources, graphics states with arrays, soft masks or fonts, property lists with containers, inline (non-indirect) resource entries, a different object under an authored name that is already used (the cold lifter renames to `name_2`; the controlled lifter has no content rename pass yet), more than 32 fonts, images, states or property lists per document, resource dictionaries above 32 keys, type 3 fonts and embedded Type 1 programs.

### Real-world audit: the bachelor thesis (65 pages, pdfTeX, 6.3 MB)

`controlled-barrier-audit.py` (pymupdf 1.26.7) on `🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf`:

- resource categories per page: `Font` 65, `XObject` 11, `ProcSet` 65, so the page resources of the thesis are now qualified (before: two keys and a single font).
- first remaining barrier is the catalog: `Names`, `OpenAction`, `Outlines`, `PageLabels` (the controlled catalog accepts `Type Pages PageLayout PageMode Version Lang MarkInfo`), and the info key `PTEX.Fullbanner`.
- fonts: 322 Type 3 (bitmap) uses, 8 embedded Type 1, 16 embedded TrueType; xobjects: 19 forms, 5 images; no annotations.

So the thesis does not open through the controlled lifter yet and is not claimed: it needs lifted catalog name trees, outlines, page labels and open action, Type 3 fonts, embedded Type 1 programs and form XObjects. The test uses a real-world-style fixture instead (below).

### Tests with independent oracles

Fixtures (`🧪️tests/🧫️fixtures/🧩️multi-resource/`, written by `multi-resource-fixture.py`, spec in `spec.json`):

- `generated.pdf`: 3 pages, resources inherited from the page tree root (3 fonts, 2 images, 1 graphics state, 1 property list), a page with its own indirect resources and `ProcSet`, fonts with WinAnsi, MacRoman, no encoding and an encoding dictionary with `/Differences`.
- `report.pdf`: 12 pages, pdfTeX style: three shared indirect `Resources` objects with `ProcSet`, five global font names shared across pages, two images shared across pages, `Tj` and kerned `TJ`.

The test opens both through the registered route under the browser grant contract (`open_registered_pdf`: paged natural source, one browser turn of fuel, interactive component grants) and checks: pages, fonts, images, admitted roles, objects, trailer, graphics states, property lists equal the cold reader `decode_pdf`; font and image counts equal the spec; per page the fonts and images declared by the page resources equal lopdf (`reference_page_resources`, nearest-ancestor inheritance through lopdf objects), the fonts selected and images painted by the content equal the spec and are declared and typed, the decoded `Tj` text equals the spec and, where the spec says so, lopdf `Tj` text and lopdf `Do` count. pymupdf (`multi-resource-oracle.py`) agrees with the same spec on fonts, images and text per page (`generated 3 / report 12 pages: all OK`).

The font decode is checked against the reference codec for every byte of six encodings (MacRoman, Standard and WinAnsi with differences, swapped glyphs, duplicate glyphs).

## 2. Terminal publication: one allocation attempt per effect

Cause: `RetainedAllocationEffect::nested` shares one `actual` marker ("no nested attempt slot is created"). `NaturalFileDecodeCursor::close_chained` chained up to 16,384 close actions inside one effect; the paged-source retirement allocates (birth of its retirement frontier), the next chained action reserved a page again and `reserve_page_using_for_limit` refused with `original page allocation attempt is occupied`. Fix: the chain ends at the first action that allocated (`effect.actual().is_some()`), the next turn continues. This is a product path (the browser closes the controlled decoder through the same chain), not only a test defect.

## 3. Worker-stage jobs that never finish

Causes found by instrumentation (`[DEBUG]` lines, removed):

1. Fuel quote above the contract. `BoundedNativeEditingEditor::native_edit_execution_contract` declares `bounded_first_step(.., max_work_units = 1, ..)` for every native tool, and `drive_worker_step` loans each worker step `min(turn fuel, contract.max_work_units_per_step)`. `PageAuthoringWork::step` needs 8 units and `PngPickerWork::step` 1,030 (its birth work is 1,025): both answered `Pending` forever. Measured fuel inside the step: 4,096 after the fix. Fix: the PDF editor declares its own contract (`bounded_native_editing_editor!` gained `contract:`; `NATIVE_STEP_UNITS = 4_096`).
2. No terminal frame witness. After the first cause the job reached its terminal retirement and failed with `retained command input requires its original app-owned retirement authority`: `PageAuthoringWork` and `PngPickerWork` never implemented `terminal_frame_release_bytes` (the default is `None`, the other works answer `Some(size_of::<Self>())`). Fix: both answer it.
3. Picker publication (insert-image only, not fixed): the work completes and its `RequestFileOpen` carries the hidden typed marker (`issue_args`); the public effect is produced by the reactor step `continue_original_picker_effects` (`⚛️reactor/🏃️runtime/🧰️ops/🎞️raw-media/📤️picker.rs`) against the instance's `NativeLifetimeSlot` (`slot.admit_original_media`, then `picker_args(&slot.raw_media.picker)`). `take_typed_operation_effect` returns `None` while the marker is at the head of the effect outbox, and the fixture host (`artifact_app_laws::settle_registered_typed_operation`) never runs that step (the fixture apps are bare `VcsArtifactApp`s without a runtime slot), so `has_pending_typed_operations` stays true until the settlement ceiling (4,149 turns; the trace shows exactly one picker work step and then nothing). Two further findings make this more than a missing call: (a) the published arguments are the original-media capsule `{ "originalMedia": { loan { operationId, generation, instanceId, documentId, baseRevision }, kind, maximumFileBytes } }`, while the test still asserts the superseded wire (`page`, `operator`, `revision` arguments and an `import-local-image` callback carrying `chunk`/`payload`); (b) the bytes of the picked file now travel through the fixed-page original-media transport, not through that action. The test therefore describes a contract that the original-media redesign replaced; it needs to be rewritten against a host that owns a runtime slot (as `⚛️reactor/🧪️tests/📤️picker` does), together with a fixture lifetime slot or a `PluginRuntime`-hosted instance. That is a harness and contract decision of its own and was not started here.

Browser: causes 1 and 2 are product causes (the contract and the work are the shipped ones), so `replace-page` / `merge-document` / `extract-pages` (authoring tools) and the picker work could not run in the browser either; with the fixes the authoring tools complete. Cause 3 is not a browser cause: the reactor runs the publisher every kernel turn.

## 4. Verification

- Targeted run `r3` (snapshot-value tests, filtered): terminal publication, font change, page and authoring undo/redo, multi-resource oracle test pass; the picker test fails at its first settlement (cause 3).
- Content unit crate `mut-family-8`: 11 / 11 pass, including the codec oracle test and the existing `standard_type1_program_preserves_roundtrip_text_and_alias_codes`.
- Regression (`run-multi-regression.sh`): see the final status below.

## Files changed

- Lifter: `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (stages, `PageFonts`, `PageFontTable`, `GraphicsStates`, shape and entry readers, dependency order); `🔨️modules/🖼️images/🎞️controlled/🦀️.rs` (32 images, page slot list, role target); `🔨️modules/🔤️fonts/🎚️controlled-opentype/🦀️.rs` (32 fonts, per-slot dependencies, `/Differences`, MacRoman); `🔨️modules/🔤️fonts/🦀️.rs` (`controlled_simple_text`); `🔨️modules/🖋️content/🦀️.rs` and `🧵️program/🦀️.rs` (`controlled_type1_text` hook).
- Worker: `🔌️plugin/📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs` (`close_chained`); `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs` (`contract:`); PDF `✏️editor/🦀️.rs` (`NATIVE_STEP_UNITS`), `🖼️page/✍️authoring/🦀️.rs` and `📄️document/🖼️image-authoring/📂️picker-work.rs` (`terminal_frame_release_bytes`).
- Oracles and tests: `🔮️oracles/🦀️.rs` (`reference_page_resources`), editor unit tests (new multi-resource test), content unit tests (new codec oracle test), fixtures `🧪️tests/🧫️fixtures/🧩️multi-resource/`.
