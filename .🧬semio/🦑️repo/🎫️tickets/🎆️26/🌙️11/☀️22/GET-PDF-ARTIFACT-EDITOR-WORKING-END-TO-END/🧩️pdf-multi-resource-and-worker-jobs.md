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
- Regression, snapshot-value crate, filters `editor::pdf14 editor::pdf17 natural_file_route paged_natural_decode closes_mid_reserved_commit media_in_of_a_small_pdf shard_fuel_open natural_source_route` (every `pdf14*`/`pdf17*` editor variant, the large-document, natural-source, media-in, save and history laws): 125 passed, 1 failed (the picker test, cause 3), 2,770 s native, test binary run directly (logs of the first attempt were deleted by a concurrent cleanup of `🗑️generated`, so the run was repeated from the same binary with its log outside the ticket folder).
- Regression, library crates (`pdf`, `io-export`, `mut-family-5/8/11/12/13/14/15`, `page-editor-tests`, `page-editor-authoring-tests`, `native-save`): all pass (38, 19, 32, 120, 79, 73, 59, 0, 11) except one native-save test, `save::original_save_tests::a_save_over_many_small_turns_writes_the_same_bytes_as_one_over_large_turns` (14 passed, 1 failed, `the Save plan settles or is cancelled: Some(false)`, i.e. `ReferenceRequired` from the Save plan under a 16 KiB copy grant). The native-save sources were last modified at 04:15 the same day and that test drives `ClassicPdfSaveProgram` over a snapshot without the lifter, the contract or the work types changed here, so it is not attributed to this ticket; the repeat run was blocked behind shared memory pressure (`native-cargo: waiting for memory`) and was cancelled, so it stays unverified and open.


## Files changed

- Lifter: `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (stages, `PageFonts`, `PageFontTable`, `GraphicsStates`, shape and entry readers, dependency order); `🔨️modules/🖼️images/🎞️controlled/🦀️.rs` (32 images, page slot list, role target); `🔨️modules/🔤️fonts/🎚️controlled-opentype/🦀️.rs` (32 fonts, per-slot dependencies, `/Differences`, MacRoman); `🔨️modules/🔤️fonts/🦀️.rs` (`controlled_simple_text`); `🔨️modules/🖋️content/🦀️.rs` and `🧵️program/🦀️.rs` (`controlled_type1_text` hook).
- Worker: `🔌️plugin/📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs` (`close_chained`); `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs` (`contract:`); PDF `✏️editor/🦀️.rs` (`NATIVE_STEP_UNITS`), `🖼️page/✍️authoring/🦀️.rs` and `📄️document/🖼️image-authoring/📂️picker-work.rs` (`terminal_frame_release_bytes`).
- Oracles and tests: `🔮️oracles/🦀️.rs` (`reference_page_resources`), editor unit tests (new multi-resource test), content unit tests (new codec oracle test), fixtures `🧪️tests/🧫️fixtures/🧩️multi-resource/`.

## Real-world open

### Design principle and why "retain" means "lift exactly like the reference lifter"

Every typed lane the cold reference lifter (`lift_document`) fills is what the editor models; everything it does not model already survives in the retained graph (`snapshot.objects`) and in the `extra` entries of the typed records. The Save planners decide what moved by lifting the retained graph with that same reference lifter (`Reading::of`, `Moved::between`, `reconcile`). So a controlled lift that leaves a modelled lane empty would not "retain" it: the planner would see the lane as removed and the reference reconciliation would graft the removal. The controlled lifter therefore lifts each construct into the identical typed value the reference lifter produces, one bounded and quoted work unit per item, and only the retained graph carries the rest. The reference builders are reused per item (`Lifter::bind` over the page references in hand, `cold(..)` in the controlled cursor) after an inspection-based quote (`reach`, `direct_bytes`), and a builder that would admit files, ids or stream roles is refused (it would need cursor state the slice does not own yet).

### Slice 1: catalog, Info, page group, annotations (done)

- Catalog: `Outlines`, `Names`/`Dests` name tree, catalog `/Dests`, `PageLabels`, `OpenAction`, `ViewerPreferences`, `Metadata` (with its `MetadataText` stream role in the reference position) and every unmodelled key (`StructTreeRoot`, `PieceInfo`, `GSConverterData`, ...) as `catalog_extra`. They are lifted after the page walk (lane order of `lift_catalog`), one outline item, one name/number tree pair or one dictionary entry per unit, through a resumable inline frame stack (34 levels, outline depth 32 like the reference) and a quoted sorted visited table. `OCProperties`, `AcroForm`, `OutputIntents` and `Names/EmbeddedFiles` still stop (they need ids and roles of later slices).
- Page references: a first page-tree walk (`Prewalk`) collects every page reference before any page is lifted, so outline items, named destinations, the open action and link annotations resolve later pages exactly like the reference lifter.
- Info: any text string (PDFDocEncoding or UTF-16, no 128-byte cap), indirect values (`InfoValue` stage), unknown keys as `info.extra` (`PTEX.Fullbanner`).
- Page: indirect `/Group` (`PageGroup` stage); annotations beyond the inline-quoted subset (GoTo actions, `/Dest`, named destinations, any subtype without appearance stream, pop-up, reply or optional content) through the reference builder; up to 4,096 annotations per page instead of 8.
- Barriers now name their entry: `deferred_detail()` returns barrier, entry and pages lifted, the importer's refusal message carries it (for example `Resources Font/Type3 after 0 pages`).
- Incremental Save: the planner's document slice keeps the real page tree with bare page stubs when outlines, named destinations or an open action exist, and a page slice with link or widget annotations does the same, so destinations read back as their page indices and the Save stays append-only instead of falling back to the compact route.

### Ledger (controlled lift of each document, same retained graph as the reference lifter)

`🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json`, test `controlled_real_world::real_world_documents_lift_like_the_reference_lifter_or_stop_at_their_ledgered_barrier` (testkit crate):

| document | before slice 1 | after slice 1 |
| --- | --- | --- |
| navigation fixture (4 pages: outlines, name tree, labels, open action, XMP, struct root, Info extras, indirect group, 10 links) | `Catalog` | equal to the reference lift, every lane |
| multi-resource generated / report | equal | equal |
| bachelor thesis (65 pages) | `Catalog` (Outlines) | `Resources Font/Type3`, page 1 (catalog, Info and the indirect group now pass) |
| site plan (`🗺️site.pdf`) | `Catalog` | `Catalog OCProperties` |
| institution diagram (`📕️.pdf`, 1,345 forms) | `Resources` | `Resources XObject` (dictionary above the 32-entry cap; the forms behind it are slice 4) |

Remaining barriers of the thesis, in the order the lifter meets them (pymupdf audit): Type 3 fonts (322 uses, glyph procedures with inline bitmap images, one `Operators` role per glyph), embedded Type 1 (8) and TrueType (16) programs, form XObjects (19), 511 link annotations (now supported), nested page tree (supported by the lift, but the incremental Save planner still needs a flat tree: it goes compact). bauteilbörse (22 MB) exceeds the 16 MiB paged-source ceiling and was not used.

### Next slices (designed, not implemented)

1. Fonts (thesis next): Type 3 lifts resources, then every `CharProcs` stream as a content program with its own `Operators` role; the reference lifter appends the role dependencies of every claimed font in `HashMap` iteration order (`parse_and_rename`), which makes its role order nondeterministic, so equality needs that order fixed first (sorted owners, as the content roles already do). Embedded Type 1/TrueType need the font program roles of `FontCodec` and the per-font codec table for text decode in the content program.
2. Form XObjects: claim the form id, lift its nested resources with the same stage machine (a resource frame stack instead of the single `Resume`), parse its content with the form's font table, record its role in the source-role group.
3. Optional content (`OCProperties`) before pages, then `Properties` that point at groups.
4. Caps: the concurrent agent replaced the fixed 32-slot role, font, state and property tables with growable quoted tables (`table_growth`/`grow_table`); the 32-entry resource-dictionary cap remains (institution diagram).

The UI notice for preserved-but-not-displayed content and the edit guards for retained constructs are not started: in this design no construct is opened without being typed, so the notice belongs to the first slice that types a construct the renderer cannot paint (Type 3 glyphs).

### Slice 1 tests and verification

- New, testkit crate (`run-real-world.sh <log> semio-s-artifact-stdio-pdf-testkit controlled_real_world document_contracts page_content_contracts`): `controlled_real_world::real_world_documents_lift_like_the_reference_lifter_or_stop_at_their_ledgered_barrier` (ledger above, controlled lift compared lane by lane with `lift_document` over the same retained graph, every unit inside its grant), `controlled_real_world::navigation_fixture_lanes_agree_with_lopdf_and_the_schema_first_spec` (outline titles and depths, named destinations with pages, page labels, open action, annotation counts, Info extras, catalog extras against the new lopdf oracle `reference_navigation` and `spec.json`). 11 of 11 pass with the existing controlled document and page contracts (`resource_free_document_projects_identically_without_cold_semantic_grants` keeps the 32 KiB cursor bound; the tree frames use a 1-byte key instead of key strings for that).
- New, snapshot-value crate: `registered_pdf_with_outlines_names_labels_and_links_opens_and_saves_incrementally_like_the_reference_readers` — open through the registered browser route (paged natural source, browser fuel, interactive grants), every lane equal to `decode_pdf`, lopdf agrees; `set-info-field` title and `insert-text` on page 1, Save: the original file is an exact byte prefix of the saved file (append-only, under 16 KiB appended), lopdf reads every navigation lane unchanged; reopen through the registered route: title edited, outlines, named destinations, labels, open action, viewer preferences, metadata, catalog extras, Info extras and pages 2 to 4 equal to the first open, the inserted text present. Passes together with the multi-resource, save-append, edit-title, controlled-annotation and natural-source tests (6 of 6).
- pymupdf 1.26.7 (`navigation-oracle.py spec.json navigation.pdf --edited "Edited navigation" Hello navigation-saved.pdf`): 38 of 38 checks on the original and the saved file (toc titles and explicit pages, resolved names, page labels, open action, Info and extras, catalog keys, XMP, 10 link annotations, indirect group, page text; the inserted text is checked in the page program because `insert_text_edit` places it at y 700, outside the 300 pt page, which pymupdf's clipped extraction skips).
- Fixture: `🧪️tests/🧫️fixtures/🧭️navigation/` written by `navigation-fixture.py`; real-world inputs copied to `🧪️tests/🧫️fixtures/🌍️real-world/` (`🗺️site.pdf`, `🏰️institution.pdf`), the thesis is read from its asset folder.
- `run-multi-regression.sh`: library crates all green (pdf 38, io-export 19, mut-family 5/8/11/12/13/14/15: 32, 120, 79, 73, 59, 0, 11, page-editor-tests 15, page-editor-authoring-tests 20, native-save 66, including the save-over-many-small-turns test that was open before). Snapshot-value filters: 126 passed, 1 failed (`registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit`, cause 3 above, unchanged), with `every_declared_pdf14/17_window_publishes_its_surface_before_and_after_the_document_mounts` skipped: both abort the test process in the reactor (`cold pair ingress requires bounded terminal close before teardown` after the instance never reaches `Retired`); the PDF 1.4 variant never reaches the 1.7 lifter, and another agent is instrumenting exactly these tests, so they are not attributed to this slice and stay open.

### Slice 1 files

- `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (navigation lanes, prewalk, Info extras and indirect values, page group, reference-builder annotations, barrier details, release of the navigation tables on success and close) and its new `🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/⬇️lift/📚️catalog-and-page-mode/🦀️.rs` (`lift_page_label`, `lift_open_action`, `lift_metadata`, `lift_viewer_preferences`, `CATALOG_KEYS` shared by both lifters), `🔨️modules/⬇️lift/🧭️navigation-and-annotations/🦀️.rs` (`lift_outline_item`)
- `🚪️io/📥️import/🔁️controlled/🦀️.rs` (refusal names barrier, entry and pages)
- `✏️editor/💾️native-save/🧷️increment.rs` (page stubs keep destinations resolvable in document and link-page slices)
- `🔮️oracles/🦀️.rs` (`reference_navigation`, `ReferenceNavigation`)
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (new registered-route test), `🧬️schema/🧪️testkit/📦️packages/🦀️rust/🦀️.rs` (mount)
- fixtures `🧪️tests/🧫️fixtures/🧭️navigation/`, `🧪️tests/🧫️fixtures/🌍️real-world/` (ledger `🔣️.json`, two PDFs)
- ticket inputs: `navigation-fixture.py`, `navigation-oracle.py`, `run-real-world.sh`

### Slice 2: fonts (done), plus soft-mask images, nested page trees, optional content and forms (started)

**Deterministic reference order first.** `parse_and_rename` appended the role dependencies of every claimed font in `HashMap` iteration order, so the dependency list of every Type 3 glyph and form program differed between two lifts of one file. Both content-role sites now use `Lifter::font_owners()` (claimed fonts in ascending object order, the order page roles already used). Test `reference_glyph_programs_depend_on_every_claimed_font_in_object_order`: nine lifts of the font fixture record identical roles, and each of the 42 glyph roles equals `GraphSource::dependencies_of(glyph)` followed by the closures of the fonts claimed so far in object order (the fixture claims the Type 3 font after fonts of higher and lower object numbers, so any other order fails).

**Reference font builder made resumable, output unchanged.** `begin_font` (claim, ToUnicode, descriptor and program, encoding, widths, CMap, descendant, Type 3 resources), `lift_char_proc` (one glyph program and its `Operators` role), `finish_font` (font value and codec); `lift_font` loops over them.

**Controlled lifter.**
- Fonts outside the inline subset are lifted by the reference builder over a persistent reference state the cursor owns (`cold: DocumentLiftState`): `Stage::ColdFont` (quoted by `font_quote`: the dictionaries and arrays read plus every decoded stream twice, admitted and parsed), `Stage::ColdGlyph` (one glyph per unit, quoted with its exact dependency list), `Stage::ColdFinish`. Roles a step records are merged before the content roles, replacing a role of the same identity and kind in place like `GraphSource::record_role`. Inline-typed fonts (and, before a form, images, graphics states and property lists) are registered in the reference id table first, so the reference builder sees every claimed object. A refused embedded-font cursor is retired (`retire_pending`) before the reference builder runs. All reference state is released with its quoted bytes at completion and on close.
- Font and graphics-state owner tables grow under quotes (no 32-per-document cap); a page still binds at most 32 fonts.
- Page content roles depend on the graph exactly like the reference lift: `source.role_dependencies` of the contents, of the font dictionary, then of every claimed font in object order, deduplicated (the hand-built order only matched one embedded font).
- Text: `PageFontTable` resolves a reference-typed font to its codec; the content program decodes such a string in one quoted unit with the reference parser's `text_string` (`Phase::CodecText`); the page program accepts codec fonts; named marked content (`/OC /oc1 BDC`, `DP`) stays on the controlled path.
- Resource category dictionaries may be indirect (`/Font 7 0 R`).
- Images: `qualified_image_dictionary` (shared by both lifters) accepts an indirect `/SMask`, so a large masked image is an artifact body in the reference lift too instead of a multi-megabyte sample vector; the controlled cursor types the image with `soft_mask`, then its mask under `<name>SMask` (or references a mask typed before), role after the image's role.
- Optional content: `Stage::OptionalContent` runs the reference `lift_optional_content` before the first page; a property list that is an optional content group is typed `OCG <group id>` like the reference.
- Form XObjects (`Stage::ColdForm`): the reference `lift_x_object` lifts one form per unit (its resources, nested forms, `Operators` role, renames inside the form exactly as the reference does); its fonts, graphics states, property lists and forms are moved into the document lanes in reference order and bound to their owner slots; a form that types images, shadings, patterns or colour spaces still stops (`XObject/Form/Resources`). Page `Do` accepts bound forms.

**Text extraction per font capability.** Predefined UCS-2 / UTF-16 CMaps (`UniGB-UCS2-H`, ...) now decode their codes as Unicode and encode back (both lifters share the codec).

**Editing guard.** `FontCodec::shows_text` also requires every encoded code to name a defined glyph — a Type 3 glyph procedure or a glyph listed in a Type 1 subset's `/CharSet` — so neither `set-text` nor `insert-text` picks a subset that lacks the glyphs. `set-text` refuses with the new catalog fault `stdio.pdf.glyphs-missing` (param `font`; en "The font {font} has no glyphs for the requested text, …", de "Die Schrift {font} enthält keine Glyphen für den gewünschten Text, …").

**Incremental Save on nested page trees.** The planner collects the leaves of any page tree (`page_leaves`, depth 64), seeds the ancestor chain of the edited page restricted to that page (inherited attributes and resources kept), and, for documents with navigation, every node as authored with page stubs under their own parents; the save stays append-only.

**Fixtures and oracles.**
- `🧪️tests/🧫️fixtures/🔤️fonts/` (`font-fixture.py`, from the thesis' own font objects): page 1 CMR8 Type 1 (`FontFile`, `/CharSet` subset), Consolas TrueType (`FontFile2`), pdfTeX Type 3 F27 (42 bitmap glyph procedures, claimed third); page 2 MyriadPro CFF (`FontFile3`, `/Differences`), Type 0 with predefined `UniGB-UCS2-H`, Type 0 / CIDFontType2 Consolas with ToUnicode. pymupdf (`font-oracle.py`) and lopdf (`reference_page_fonts`) agree with `spec.json` on fonts, subtypes, embedded programs and text.
- `🧪️tests/🧫️fixtures/🌳️nested-tree/` (`nested-fixture.py`): 5 pages under two nodes, inherited MediaBox, an outline into the second node.

**Ledger after slice 2** (`🔣️.json`, controlled lift with exactly the grant each unit quotes, compared lane by lane and role by role with `lift_document`):

| document | after slice 1 | after slice 2 |
| --- | --- | --- |
| navigation, multi-resource generated / report | equal | equal |
| fonts fixture (new) | n/a | equal |
| nested page tree fixture (new) | n/a | equal |
| bachelor thesis (65 pages, 47 fonts, 770 Type 3 glyphs, 19 forms, 5 SMask images) | `Resources:Font/Type3:0` | equal (intermediate: `Resources:XObject/Form:24` before forms) |
| site plan (OCProperties, CID CFF font, indirect `/Font`, `/OC` marked content) | `Catalog:OCProperties:0` | equal |
| institution diagram | `Resources:XObject:0` | `Resources:XObject:0` (1,345 forms in one dictionary above the 32-entry category cap; its ExtGStates are direct dictionaries) |

The ledger test now reports every document's outcome in one assertion, and a role mismatch names the role index, identity, kind and first differing dependency.

### Slice 2 tests and verification

- Testkit crate (`run-s2.sh`, filtered): `controlled_real_world::real_world_documents_lift_like_the_reference_lifter_or_stop_at_their_ledgered_barrier` (ledger above), `…::reference_glyph_programs_depend_on_every_claimed_font_in_object_order` (new), `…::fonts_fixture_types_every_font_kind_like_lopdf_and_the_schema_first_spec` (new: subtypes and embedded program kinds against lopdf and the spec, decoded text per font, 42 Type 3 glyph programs with their inline images), `…::navigation_fixture_lanes_agree_with_lopdf_and_the_schema_first_spec`, document and page-content contracts (32 KiB cursor bound kept), the thesis codec retention law: 14 of 14 pass once the thesis and site plan entries read `equal`.
- Content unit crate (`mut-family-8`): `codec_fonts_decode_text_and_codes_like_the_reference_parser_in_one_unit_turns` (new: a codec font with WinAnsi and ToUnicode, `Tj`, `TJ`, `'`, `"`, a font outside the table and named `BDC`/`DP` equal `parse_content`, every decoded string quoted, the account balances) and the 7 existing content-program tests: 8 of 8.
- Page editor unit (`page-editor-tests`): `set_text_refuses_glyphs_the_font_lacks_with_a_localized_notice_and_changes_nothing` (new) and the fault catalog law (17 codes, en/de, same placeholders).
- Snapshot-value crate, registered browser route (paged natural source, browser fuel, interactive grants): `registered_pdf_with_a_nested_page_tree_saves_edits_on_both_nodes_incrementally` (new, passes: insert-text on page 1 and page 5 plus a title edit stay an appended section, lopdf reads the outline into the second node unchanged, reopen equal), `registered_pdf_with_type3_embedded_and_composite_fonts_opens_edits_and_saves_like_the_reference_readers` (new: fonts, pages, roles, objects equal `decode_pdf`, lopdf subtypes, `set-text` refused with `stdio.pdf.glyphs-missing` on the Type 3 and the CMR8 subset, save append-only, reopen keeps every font), `registered_real_world_site_plan_opens_edits_saves_and_reopens_like_the_reference_readers` (new), multi-resource and navigation (unchanged, pass).
- pymupdf: `font-oracle.py` (fonts, subtypes, embedded programs, text per page: 2 of 2 pages OK); `nested-fixture.py` and `font-fixture.py` assert their own output through pymupdf.
- Library regression (`run-s2-regression-libs.sh`, the library half of `run-multi-regression.sh` plus the testkit): all green — pdf 38, io-export 19, mut-family 11/12/13/14/15/5/8: 32, 120, 79, 73, 59, 0, 12, native-save 15, page-editor-authoring-tests 20, page-editor-tests 68, testkit 60 (ledger, thesis codec retention and lossless-flow laws included).
- Registered-route runs: before the last font-quote change the nested page-tree, multi-resource and navigation tests passed (`s2-route-1`); the font fixture then failed only on the per-unit backing grant (`PDF decoder needs a 546890-byte backing grant`: the Type 0 Consolas font pymupdf embeds whole quoted its W array and its streams twice, above the 512 KiB copy grant of the interactive policy), which the quote now states once. The final run (`s2-route-3`) could not verify any registered test: every open, including the unchanged navigation and multi-resource tests, fails in the framework with `interactive-job.worker-continuation` "WorkLimit: original effect performed more work than its ceiling" while another agent changes the worker-continuation code in the shared tree; earlier runs were blocked by two foreign compile breaks (value receiver protocol debug code, plugin ownership refactor) that their owners fixed. The snapshot-value regression half was therefore not run for this slice.
- The bachelor thesis through the registered route: the open exhausted the fixture scope's total copy budget (`maximumCopyBytes` 528 MB, `🎟️caller.json`) after 4,106 turns, so the thesis is proven only by the controlled-lift ledger (exact grants, every unit inside its quote), not yet by a browser-route open/save/reopen.

### Remaining barriers after slice 2

- Institution diagram: `Resources:XObject:0` — 1,345 form XObjects in one dictionary (category cap 32), direct (non-indirect) `ExtGState` entries (`a0 <</CA 1/ca 1>>`), and the reference renames the page's `a0` to `a0_2` after the forms claimed `a0`, which needs a controlled content rename pass (the controlled page program does not rename resource names yet).
- Forms that type images, shadings, patterns or colour spaces (`XObject/Form/Resources`); page-level colour spaces, patterns, shadings; inline (direct) resource entries; graphics states with arrays or soft masks at page level; `AcroForm`, `OutputIntents`, `Names/EmbeddedFiles` (slice 4).
- Per-unit bound: a font program above about 250 KB, or an inline sampled image between 256 KB and the 1 MiB inline threshold, cannot be admitted in one unit under the 512 KiB copy grant; those need paged admission (artifact copy and sample vector across units).
- Browser-route proof for the thesis (turn count and the fixture's total copy budget) and the in-page localized notice for preserved-but-not-displayed content (slice 5).

### Slice 2 files

- Reference: `🔨️modules/⬇️lift/🧩️lifter-and-document/🦀️.rs` (`font_owners`, deterministic glyph and form role dependencies), `🔨️modules/⬇️lift/🪟️resources-and-fonts/🦀️.rs` (`FontLift`, `FontStart`, `begin_font` / `lift_char_proc` / `finish_font`), `🔨️modules/⬇️lift/🦀️.rs` (exports).
- Controlled: `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (cold font, glyph, finish and form stages, reference state, role merge, optional content stage, OCG property lists, indirect category dictionaries, growable state tables, reference-exact page role dependencies, codec font table, quotes); `🔨️modules/🔤️fonts/🎚️controlled-opentype/🦀️.rs` (growable owner table, `bind`, `retire_pending`); `🔨️modules/🖼️images/🎞️controlled/🦀️.rs` (soft masks, owner accessor); `🚪️io/📦️foreign-artifacts/🦀️.rs` (`qualified_image_dictionary` admits an indirect `/SMask`).
- Content: `🔨️modules/🖋️content/🧵️program/🦀️.rs` (`Phase::CodecText`, named `BDC`/`DP`), `🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🧺️program/🦀️.rs` (codec fonts, named property lists).
- Fonts: `🔨️modules/🔤️fonts/🦀️.rs` (`defined_codes` glyph guard, UCS-2 / UTF-16 predefined CMaps).
- Editor: `✏️editor/📄️document/🚨️faults/🦀️.rs` (`GlyphsMissing`), `✏️editor/🖼️page/🖱️object-edits/🦀️.rs`, `✏️editor/💾️native-save/🧷️increment.rs` (nested page trees).
- Tests and oracles: `…/🧭️controlled/🧪️tests/🔬️unit/🦀️.rs`, `🔨️modules/🖋️content/🧪️tests/🔬️unit/🧪️content-program.rs`, `✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (three new registered-route tests), `🔮️oracles/🦀️.rs` (`reference_page_fonts`).
- Fixtures: `🧪️tests/🧫️fixtures/🔤️fonts/`, `🧪️tests/🧫️fixtures/🌳️nested-tree/`, ledger `🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json`.
- Ticket inputs: `font-fixture.py`, `font-oracle.py`, `nested-fixture.py`, `run-s2.sh`, `run-s2-regression-libs.sh`.

### Slice 3: browser-route proof, open cost, resource caps and the preserved-content notice (2026-11-25)

Native, debug, every cargo run through `native-cargo.sh` (one at a time, `watchdog-cargo.py`), filtered; no wasm rebuild. Runners kept in this folder: `run-route.sh` (testkit + snapshot-value, filters), `run-real-world-all.sh` (the full slice-3 set, one run), `real-world-oracle.py` (pymupdf on original vs saved), `form-resources-fixture.py`. The PDF workspace was left compiling (`check --workspace --tests`, exit 0) after a concurrent lift/lifter crate split; two stale test calls were repaired on the way (1.4 canvas test, `take_images(.., None)`).

#### Measured causes (all printed by temporary `[DEBUG]` lines, removed)

| fixture | registered route before | cause found |
| --- | --- | --- |
| fonts | `needs a 546890-byte backing grant` | stage `type3-glyph` (the refusal now names phase, lift stage and pages: `during lift type3-glyph after 0 pages`): the glyph quote is `2 * (size_of::<PdfOp>() * (len/2+1) + len)` and `size_of::<PdfOp>()` was **376 bytes**, because `PdfOp::InlineImage` held a whole `PdfInlineImage` inline |
| site plan | `needs a 770048-byte backing grant` | stage `content`: one page of 12,843 operators doubles its operator vector to 2,048 × 376 bytes |
| nested tree, navigation, two-page | `WorkLimit` in worker continuation | foreign in-flight framework edit; gone after the owners' fix (pass now) |
| thesis | copy stock 528 MB spent after 4,106 turns, still decoding | the decoder charged `COPY_PER_WORK = 128` bytes for every **byte-level** filter unit; the thesis retains 37 MB of *decoded* streams (24 MB from 226 KB of Flate images, 11.8 MB of Illustrator `AIPDFPrivateData`, 0.6 MB page content), ~4 cursor units per decoded byte |

#### Fixes

1. **Decoder unit and copy model** (`🚪️io/📥️import/🔁️controlled`): while the stream cursor only decodes body bytes (identity or any filter: `decodes_body_bytes` down the chain `PdfStreamDataCursor` → `PdfDecodedStreamCursor` → normal projection → `decoding_body_bytes` of the retained graph), one contract unit covers `BODY_BYTES_PER_UNIT = 64` cursor units and each cursor unit is charged `BODY_COPY_PER_BYTE = 2` bytes; a bulk call takes at most half the copy grant so a page or history reservation still fits. Fuel and progress count contract units.
2. **`PdfOp` 376 → 64 bytes**: `InlineImage { image: Box<PdfInlineImage> }` (wire shape unchanged). Every operator vector (pages, roles, glyph procedures, forms) shrinks 5.9×; the fonts fixture's largest lift unit is now 231,696 bytes.
3. **One content parse per page**: the `Mirror` stage no longer re-tokenizes the page; it copies the collected operators into the content role one quoted operator per unit (`operator_bytes`, now exported by the page-content crate).
4. **Resource caps**: category dictionaries of any size (`resource_shape`, `read_entry`, `unsupported_resource`), page font and form slots are a growable quoted `PageSlots` table (kept across pages, released with the navigation tables); direct (inline) `ExtGState` entries of a page are lifted through the reference builder under the page scope (`Stage::ColdState`), and the page program renames authored names to the reference ids (`Stage::Rename`, one operator per unit, through the shared `content_mapping::rename_target`; the content role keeps authored names like the reference). Forms may now type shadings, patterns and colour spaces (moved into the document lanes in reference order); an image typed inside a form still stops (`XObject/Form/Resources/Image`).
5. **Pack export chunk runs** (`🧰️framework/.../🎒️document/⏳️cursor`, `⏳️cursor::chunk_bytes`): chunk payload read+hash and emit move up to `CHUNK_RUN = 64` bytes per event (one projection per run instead of one per byte); output bytes are identical. Not yet measured on the thesis (its open does not reach Encode).
6. **Refusal detail**: `PDF decoder needs a N-byte backing grant during <phase> <lift stage> after <pages> pages` (`ControlledDocumentLiftCursor::stage_name`, `pages_lifted`).
7. **Preserved-but-not-displayed notice (task 4)**: `canvas_paint` reports every page where the canvas paints a placeholder (image or shading without preview, XObject without a typed body, annotation without appearance or default paint) or the page keeps `Unknown` operators; such a page gets a localized notice layer `p{n}:preserved` and the canvas's accessible description lists the pages (`PdfEditorLabels::preserved_notice` / `preserved_pages`, en/de; labels moved into the page-editor base crate). Edit guard: every object edit on an undisplayed object is refused with the new fault `stdio.pdf.content-preserved` (param `page`, en "This content on page {page} is preserved in the document but cannot be displayed or edited here. Nothing was changed.", de "Dieser Inhalt auf Seite {page} bleibt im Dokument erhalten, kann hier aber nicht angezeigt oder bearbeitet werden. Es wurde nichts geändert.").

#### Ledger (`🔣️.json`) and route status

| document | controlled lift (exact grants) | largest lift unit (browser bound 524,160) | registered browser route |
| --- | --- | --- | --- |
| navigation, multi-resource ×2, nested tree | equal | 126,266 / 7,168 / 7,584 / 19,269 | navigation, nested, multi-resource, two-page: **pass** (open, edit, Save append-only, reopen) |
| fonts | equal | 231,696 (`type3-glyph`; was 1,325,568) | open passes the glyph stage now; fails at app close: one retirement quotes a 5,473,073-byte release against the 1 MiB release grant (the controlled lift's reference state is released in one unit) |
| form resources (new: form with shading, pattern, colour space) | equal (was `XObject/Form/Resources`) | 35,992 | not routed |
| institution diagram (1,345 forms) | **equal** (was `Resources:XObject:0`) | 784,324 (`form`) | blocked by that form unit |
| site plan | equal | 1,048,576 (`content`, 12,843 operators × 64 B doubled) | blocked: one contiguous operator vector per page |
| bachelor thesis | equal (887 roles, 2,861 operators max per page) | 1,293,321 (`form`, page 38) | decode now completes: **2,474,979 units, 467.5 MB copy, 49.8 MB live** (before: stock exhausted at 528 MB after 4,106 turns still inside prepare-file, ≈4.1 M units at 128 B); the lift then exhausts the remaining ~60 MB of the scope's 528 MB stock |

Tests (new or changed): testkit `every_real_world_lift_unit_fits_one_browser_turn` (new law, **fails** honestly with the three units above), `real_world_documents_lift_like_the_reference_lifter_or_stop_at_their_ledgered_barrier` (9 documents, all equal with the ledger update); page editor `preserved_but_undisplayed_content_shows_a_localized_notice_and_refuses_edits` (new, passes), fault catalog law (18 codes, passes), all `canvas_*` laws (pass); content unit 8/8; snapshot-value `registered_bachelor_thesis_opens_edits_saves_and_reopens_like_the_reference_readers` (new, fails as above), `registered_real_world_site_plan_…` (now through `real_world_round_trip`: open within `512 + 512 × MiB` turns and `32 MiB + 24 × file` copy, cold-reader and lopdf lanes, page-by-page lopdf digest of the saved file, navigation unchanged, reopen), navigation/nested/multi-resource/two-page pass. `OpenCost` now reports `copied_bytes`. pymupdf `real-world-oracle.py` is ready (266 checks on the thesis) but has no saved real-world file to check yet.

#### Remaining barriers (measured, not fixed)

- **Thesis cost is decoded-stream volume, not page count**: the retained graph keeps Flate streams *decoded* (schema rule of `PdfStreamFilter`), so 6.3 MB become 37 MB that the decoder inflates, the copy model charges and the Pack export would encode (images in chunks, the 180 × 64 KiB Illustrator private streams through per-byte segment deflate). Proportional cost needs streams the editor does not interpret (images whose samples are only previewed, `PieceInfo` private data) retained encoded and decoded on demand, and the Pack segment deflate fed by runs instead of bytes. The current per-unit charge (2 bytes per cursor unit) overstates real copy (~3 bytes per decoded byte against ~8 charged); exact receipts (bytes written per cursor step) would lower it truthfully.
- **Contiguous per-page operator vectors** (`PdfPage.content`, role content): a page above 8,190 operators needs > 512 KiB in one allocation; needs a paged operator list (schema).
- **Atomic reference form units**: forms are lifted by `lift_x_object` in one unit (quote 2 × operators + nested forms); large forms need the controlled content program for forms (and inline images in that program).
- **Reference state release in one unit** (fonts route close), **1-bit images as `Vec<u32>` samples** (32× expansion of every Type 3 bitmap glyph in the typed snapshot), **font programs** cloned whole into the reference artifact table and the codec, images typed inside forms, catalog `AcroForm` / `OutputIntents` / `EmbeddedFiles`, page-level colour spaces, patterns and shadings, more than 32 images per document.

### Slice 4: uninterpreted streams stay encoded, bounded lift units, browser-grant law green (2026-11-25)

Native, debug, every cargo run through `native-cargo.sh` (FIFO queue, `watchdog-cargo.py`), filtered; no wasm rebuild. New ticket inputs: `run-s4.sh` (one gated run, any cargo args), `s4-stream-volumes.py` (pymupdf census of encoded and decoded stream bytes per kind), `s4-encoded-images.py` and `s4-many-images.py` (the two patch steps, applied once after the tree before them compiled).

#### Measured volumes (pymupdf `s4-stream-volumes.py`, and the new law's report of the retained graph)

| document | streams encoded in the file | decoded (what every import held before) | retained after slice 4: kept encoded / decoded |
| --- | --- | --- | --- |
| bachelor thesis | 5,913,867 B (1,101 streams: 10 images 226 KB, 984 other incl. 180 Illustrator `AIPrivateData`, 23 font programs, 65 page contents, 19 forms) | 37,023,963 B (images 23.9 MB, other 11.6 MB) | 188 streams / 5,144,934 B kept encoded, 2,168,608 B decoded (17× less than before) |
| site plan | 29,514 B | 141,384 B | 0 / 126,923 B (everything it holds is interpreted) |
| institution diagram | 186,593 B | 343,228 B | 0 / 343,228 B |
| fonts fixture | 70,994 B | 206,307 B | 0 / 206,307 B |

#### Fixes

1. **Uninterpreted streams stay encoded** (`🗜️filters::declares_logical_codec`, `📦️normal-projection`, `🔗️xref::normalize_framed_stream`, new `🚪️io/📥️import/📖️interpreted-streams`). An import (paged browser route and cold `decode_pdf` alike, so both graphs stay equal) now retains every stream whose declared chain has a logical codec (Flate, LZW, ASCIIHex, ASCII85, RunLength) as the file's own encoded body with its `Filter`/`DecodeParms` still in the dictionary and no typed chain — the representation the snapshot already used for chains without a controlled decoder, so writers carry such streams through byte for byte (incremental and compact Save never re-encode them). Object and cross-reference streams are still decoded (they are structure). A new bounded pass, `InterpretedStreamCursor` (one walked object or one bounded decode run per unit, `PreparePhase::Interpret` after the retained-text pass; `admit_interpreted_streams` drives the same cursor under reference grants for `decode_pdf`), walks the document from the catalog exactly the way the lift reads it (page tree, contents, resources, fonts with descriptors, programs, ToUnicode and embedded CMaps, Type 3 glyph programs, forms, patterns, shadings, functions, colour spaces with ICC and lookup data, annotations and their appearances, actions and scripts, name trees, attachments, AcroForm fields, output intents, catalog and page metadata, thumbnails) and decodes in place, with one atomic swap, only the streams in an interpreted role; everything else — private application data under `PieceInfo`, structure, unreferenced objects — is never inflated. The raw-body copy of an encoded stream and the in-place decode run as body bytes (bulk grant, 2 bytes of copy per byte) in the importer.
2. **Large images stay encoded too** (`🚪️io/📦️foreign-artifacts`: `encoded_sampled_image`, `PdfSampledResourceIdentity::encoded`, identity of an encoded stream hashes its declared chain ahead of its body; `🖼️images` controlled and reference lifters; the walk). A qualified 8-bit DeviceGray/DeviceRGB image whose chain is one Flate filter and whose samples exceed the inline ceiling is typed as a sampled artifact of its *encoded* body; `NativePdfArtifactResources::resolve` decodes such a resource when a consumer (preview, lowering) asks for its pixels. The thesis' 23.9 MB of image samples are no longer decoded at open.
3. **Bounded release** (`🧭️controlled`): completion (`Lane::Release`) and close retire the navigation tables, content-role mirror, graphics-state and image owner tables and the reference builder state in units of at most 64 KiB credited release (one table, one font codec or the reference snapshot per unit). Fonts fixture: largest release 5,473,073 → 524,288 B (a page program's operator table).
4. **Exact program quotes** (`🖋️content::content_extent`, `program_quote`): forms and Type 3 glyphs are quoted by their measured operator count, operand heap (numbers 8 B, strings 4 B per byte, untyped operators' whole values) and inline-image samples instead of one operator per two body bytes. Thesis page-38 form unit 1,293,321 → 433,650 B (a first bound that charged two value nodes per number operand measured 1,703,394 B and was replaced by the per-kind bound). Law `measured_program_extents_count_every_operator_the_reference_parser_types`: every form and glyph program of the nine ledger documents measures to exactly the operators the reference parser types.
5. **Empty reservations are bounded by the release grant** (`PageProgramCursor::reserving_empty_backing`, `reserving_empty_slots` for `Program` and `Mirror`): a page program grows into a fresh empty vector and migrates its operators one per unit, which copies nothing; the importer already bounds such units by the 1 MiB release grant. Site plan's 12,843-operator page: 1,048,576 B empty reservation (fits), its largest copying unit 450,546 B.
6. **Stream roles shrink** (`PdfStreamRoleValue::Image { image: Box<PdfImage> }`, wire unchanged): the role table's growth for 1,345 forms no longer copies ~600-byte roles; institution diagram's largest unit was 774,942 B with the 600-byte roles (the role table's doubling growth for 1,345 forms, not the form program); after boxing it is below 524,160 B (law run `s4-final` passes).
7. **Any number of images** (`ControlledImageResources`: growable quoted owner and page tables instead of 32 slots).

#### Law, ledger and route status

Testkit (`controlled_real_world`, `document_contracts`, `page_content_contracts`, final run `s4-final`): 17 of 17 pass — `every_real_world_lift_unit_fits_one_browser_turn` (**passes**, was failing on three units), `imports_keep_uninterpreted_streams_encoded_and_decode_every_stream_the_lift_reads` (new), `measured_program_extents_count_every_operator_the_reference_parser_types` (new), `a_page_with_forty_images_lifts_like_the_reference_lifter` (new), ledger (9 documents equal), navigation, fonts, glyph-order laws. Import crate: `the_walk_decodes_page_content_in_place_and_keeps_private_application_data_as_the_files_bytes`, `every_walk_unit_stays_inside_its_grant_and_cancellation_mid_decode_retires_everything_it_allocated` (new), retained-graph and compressed-length tests (11 more) pass; xref: `projected_graph_retains_encoded_bytes_whose_normalization_yields_the_typed_filter_pipeline` (respecified: the reference resolver keeps the encoded body, its normalization still yields the former decoded bytes and pipeline), `normal_projection_keeps_a_logical_codec_body_as_the_files_encoded_span` (new) and the length test (now over an unfiltered body) pass.

Largest units per document after slice 4 (law report; browser bounds: copying 524,160 B, empty reservation and release 1,048,576 B):

| document | largest copying unit | largest empty reservation | largest release |
| --- | --- | --- | --- |
| navigation | 126,266 (navigation) | 8,160 | 7,168 |
| fonts | 234,756 (type3-glyph) | 2,048 | 524,288 (was 5,473,073) |
| form resources | 31,424 (form) | 632 | 31,440 |
| bachelor thesis | 433,650 (form, page 38; was 1,293,321) | 540,000 (image) | 524,288 |
| site plan | 450,546 (reference font) | 1,048,576 (12,843-operator page; was a copying 1,048,576) | 524,288 |
| institution diagram | < 524,160 (was 784,324; fix 6) | 524,288 | 524,288 |

Registered browser route (snapshot-value crate, runs `s4-route`, `s4-route2`, `s4-route3`; `natural_source_route_opens_a_pdf_under_the_browser_turn_fuel`, navigation, nested tree and several-fonts-images tests pass unchanged):

| fixture | before slice 4 | after slice 4 | what now binds |
| --- | --- | --- | --- |
| bachelor thesis (6.3 MB) | copy stock (528 MB) spent inside prepare-file: decode 2,474,979 units / 467.5 MB copy, 49.8 MB live | prepare-file 1,473,166 units / 311,050,439 B copy (live graph 2.2 MB decoded + 5.1 MB encoded); the open still spends the scope's 528 MB stock before it settles | prepare cost is now framing, not decoding (temporary probe, per prepare phase): object framing 714,389 units / 99.5 MB, **a fresh 22 KB `NormalProjectionCursor` box per object 3,173 × = 71.8 MB**, interpreted decode 41.7 MB (a fresh Flate history and output pages per stream), retained-text pass 35.2 MB, xref structure 23.9 MB, raw encoded bodies 22.0 MB, walk 10.9 MB |
| site plan (30 KB) | one 1 MiB copying unit | decode 70,079 units / 10.7 MB copy, lift 561,062 units / 72.6 MB copy, both complete | the Pack export (Encode) of a 12,843-operator page and its role mirror does not finish inside the 1,024 turns the test allows (`512 + 512 × MiB`) |
| fonts fixture (71 KB) | close: one 5,473,073 B release | decode 37,522 units / 9.2 MB, lift completes all pages | `Lane::Release` asks 65,536 B while the scope's lifetime release stock has 57,435 B left: the lift credits more release than the host counted as retained (the quoted `cold_bytes` exceed what was charged); not yet found |

No bound was loosened: the route tests keep `512 + 512 × MiB` turns and `32 MiB + 24 × file` copy; the law keeps 524,160 B for copying units and states the release grant as the bound of empty reservations, which is how the importer already funds them (`allocation_boundary = maximum_release_bytes` for `reserves_empty_output`).

#### Not done (measured, next levers in order)

- **Thesis prepare**: reuse one `NormalProjectionCursor` box across objects (−71.8 MB of the 311 MB), one Flate history per walk instead of per stream, framing units at a bulk rate like body bytes.
- **Encode volume**: the Pack segment encoder is a per-byte state machine (`RetainedDeflateEncoder::advance(&mut Option<u8>)`: input, search, compare, token, insert, output events per byte, two passes); byte runs on the source side alone would not remove the per-byte deflate events, so stream payloads below the 256 KiB chunk threshold (180 × 64 KiB private streams, page and role operators) still cost ~7 events per byte. Needs a run-based deflate cursor or chunking of already-compressed stream octets; not started, events per page not re-measured.
- **Fonts release over-credit** (above).
- **Paged operator lists**: pages and forms above 16,383 operators (1 MiB / 64 B) still need one empty reservation above the release grant.
- **Forms lifted across units**: a form program is still one reference unit (bounded now by its measured size).
- **Compact samples**: 1-bit and low-depth images are still `Vec<u32>` per sample (schema change of `PdfImageBody::Samples` and its sqlite/native-JSON twins; not started).
- **Barriers**: images typed inside forms, page-level colour spaces, patterns and shadings, catalog `AcroForm`, `OutputIntents`, embedded files; the 32-image limit is gone.
- **Font programs** are still admitted whole (decoded in place by the walk, cloned into the codec).
- **Native-save regression candidate**: `the_large_type1_fixture_rotates_inserts_text_and_reopens` (48 generated pages, two unfiltered content streams each) ends its Save plan in `ReferenceRequired` in both runs after these changes (`s4-final`, and another agent's `r2-native-save`); `imported_text_edit_falls_back_to_the_reference_reconciliation_and_keeps_retained_objects` and `imported_file_order_roster_saves_a_rotation_and_carries_every_other_object` pass. Not yet attributed (the fixture has no filtered stream, so the encoded-stream change does not touch its graph; the bounded release and the empty-reservation flag are the changed paths the Save proof drives).
- The pymupdf oracle `real-world-oracle.py` still has no saved real-world file to check (no real-world fixture reaches Save on the route yet).

#### Slice 4 files

- Import: `🚪️io/📥️import/📖️interpreted-streams/🦀️.rs` (new, with `🧪️tests/🔬️unit/🦀️.rs`), `🚪️io/📥️import/🦀️.rs` (cold walk, image post-pass limited to non-logical chains), `🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs` (`PreparePhase::Interpret`, raw bodies and the walk as body bytes), `🔨️modules/🔗️xref/📦️normal-projection/🦀️.rs` and its tests, `🔨️modules/🔗️xref/🦀️.rs` (framed streams keep logical chains encoded; respecified test), `🔨️modules/🗜️filters/🦀️.rs` (`declares_logical_codec`).
- Images and artifacts: `🚪️io/📦️foreign-artifacts/🦀️.rs` (`encoded_sampled_image`, encoded identity, decoding resolve), `🔨️modules/🖼️images/🦀️.rs`, `🔨️modules/🖼️images/🎞️controlled/🦀️.rs` (encoded artifacts, growable tables, boxed role).
- Lift: `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (bounded release, measured program quotes, empty reservations, image page table) and its tests; `🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🧺️program/🦀️.rs` (`reserving_empty_backing`); `🔨️modules/🖋️content/🦀️.rs` (`content_extent`, `PdfContentExtent`); `🔨️modules/⬇️lift/🪟️resources-and-fonts/🦀️.rs`.
- Schema: `🧬️schema/🪪️stream-roles/🦀️.rs` (`Image { image: Box<PdfImage> }`), `🧬️schema/🔎️graph-projection/🪟️resources-and-fonts/🦀️.rs`, `🚪️io/🪶️sqlite/📸️snapshot/🪪️stream-roles/🦀️.rs`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (one comparison).
- Oracle: `🔮️oracles/🦀️.rs` (`reference_encoded_stream_bodies`).
- Ticket inputs: `run-s4.sh`, `s4-stream-volumes.py`, `s4-encoded-images.py`, `s4-many-images.py`. The temporary `[DEBUG]` cost probes in the importer are removed.

### Slice 5: native Save green again, bounded prepare, run-based Pack segments (2026-11-25)

Native, debug, every cargo run through `native-cargo.sh` (FIFO gate) wrapped by `watchdog-cargo.py` (`run-s4.sh`, `run-route.sh`; framework crates with `NATIVE_CARGO_DIR=🧰️framework`); no wasm rebuild. Logs `🗑️generated/s5-*.log`. Every temporary `[DEBUG]` trace of this slice is removed again (the `pdf-save route=` / `increment declined` / custody traces in `💾️native-save/🧱️program.rs`, `🧷️increment.rs` and `🗃️original-custody` belong to another session and were left alone).

#### 1. Native Save: 15 / 15 (`s5-native-save`, 1,153 s)

Cause, measured (`[DEBUG]` on the reconciled graph of `the_large_type1_fixture_rotates_inserts_text_and_reopens`): the graft refuses the text insert, the program falls back to the reference reconciliation, and `prepare_reference` refused **its own quote** on every turn: actual 531,443 B against `reference_extent` 494,546 B, until `REFERENCE_TURNS` ended the plan in `ReferenceRequired`. The quote counted every stream body by `data.len()`, but a `PdfStreamBytes` owns whole 4 KiB payload pages plus a 392-byte page frame: the 95 streams of the reconciled graph hold 3,108 payload bytes in 426,360 allocated bytes. The gap had been covered by the per-operator page term (`size_of::<PdfOp>() + 64`) until slice 3 shrank `PdfOp` from 376 to 64 bytes; it is not caused by the encoded-streams change.

Fix (`✏️editor/💾️native-save/🧬️graph.rs`): `reference_extent` quotes a copied retained stream by the pages its original already owns (`stream_pages`, exact), every page's fresh content stream by `paged_extent` (whole pages, one measured frame per page and per branch fan-out; the frame is measured once on a one-byte body, `paged_page_frame`) over the spelled-operator bound `SPELLED_OPERATOR_BYTES`, and keeps the doubling factor only for the trees that grow by doubling.

| test | quote / actual | result |
| --- | --- | --- |
| `the_large_type1_fixture_rotates_inserts_text_and_reopens` | 1,172,324 / 531,443 B | pass, 57 s (was `ReferenceRequired` after 0.3 s) |
| `thousands_of_objects_rotate_insert_text_and_reopen` | 28,676,868 / 10,237,747 B (TURN ceiling 33.5 MB) | pass, 134 s: rotation Save 64 s, text Save 66 s, three cold decodes < 2 s (was killed by the 15-minute watchdog) |
| whole `semio-s-artifact-stdio-pdf-native-save` lib | | **15 / 15**, 1,153 s |

#### 2. Thesis preparation: one projection box, framing at the body rate

- `NormalProjectionCursor::restart` (`🔗️xref/📦️normal-projection`): a closed cursor is re-armed in place for the next object (keeps its object-stream policy and ceiling). `GraphImportCursor` (`📥️retained-graph`) allocates one box for the whole import, closes its internals per object (`close_normal`) and retires the box once at `CloseOrder` or on cancellation.
- `PdfNormalObjectCursor::lexes_bytes` (`🔤️lexer/🧭️normal-object`) and `NormalProjectionCursor::frames_bytes`: header, value tokens, stream prefix and the `endstream` scan lex one source byte per unit; `PdfFilePreparationCursor::decoding_body_bytes` now includes them, so the importer runs them as bulk body units (64 cursor units per contract unit, 2 bytes of copy per byte, allocations still charged separately) instead of 128 bytes per byte.

Measured on the registered browser route (`s5-route2`, temporary per-phase trace):

| prepare phase | before (slice 4) | after |
| --- | --- | --- |
| whole prepare-file | 1,473,166 units / 311,050,439 B copy | **779,136 units / 150,725,252 B** |
| xref structure | 23.9 MB | 180,623 units / 23.9 MB |
| object projection (framing, boxes, raw bodies) | 99.5 + 71.8 + 22.0 MB | 234,447 units / 47.5 MB |
| retained-text pass | 35.2 MB | 274,892 units / 35.3 MB |
| interpreted decode and walk | 41.7 + 10.9 MB | 89,160 units / 43.2 MB |

The thesis open still does not settle: the lift starts at 151 MB copy and the open runs out of its 4,096 turns (`512 + 512 × MiB`) inside the lift. Remaining prepare levers: the text pass and the structure pass are charged 128 B per graph unit although they copy almost nothing (exact per-unit receipts), and the walk still opens a fresh Flate history per interpreted stream.

#### 3. Fonts route: no lift over-credit; the binding cost is the Pack export

New law clause in `every_real_world_lift_unit_fits_one_browser_turn` (testkit, passes): every unit of every ledger lift books its allocation and release; no lift ever credits more release than its units allocated before it (`Quotes::book`, `over_credit`). So the slice-4 "over-credit" hypothesis is refuted for the lift. The 512 MiB the slice-4 message referred to is the fixture scope's fixed lifetime release grant (`🎟️caller.json` `maximumReleaseBytes` 536,870,912), not a retained-based stock. On the route the fonts fixture now decodes (6.4 MB copy) and lifts (5.7 MB copy, 4.5 MB released) completely; the open then spends the rest of the scope (528 MB copy, `s5-route`) or its 4,096 turns (`s5-route2`) in the Pack export of the typed snapshot, whose volume is dominated by typed data far larger than the 71 KB file (1-bit Type 3 glyph images as `Vec<u32>` samples, decoded font programs). Next lever: compact samples (schema change of `PdfImageBody::Samples` and its twins).

#### 4. Pack export: run-based segment events (framework `🎒️pack/…/🫳️borrowed/🎒️document/⏳️cursor`)

One segment event is now a run: at most one original-source pull of up to `SEGMENT_RUN = 64` raw bytes into an inline stage (`staged`, replacing the one-byte `pending`), its body hash in one update, then every compressor step, compressed-output byte and segment close the remaining grant funds; the event stops after the compressor asks for input, before an allocating step and when the output is full; byte currencies are summed exactly and the event is one copied item. Under exact grants the event degenerates to one step, so the exact-receipt laws are unchanged. Output bytes are identical (canonical-oracle laws).

| measure | events |
| --- | --- |
| new law `borrowed_pack_document_segment_runs_match_the_canonical_oracle_in_events_per_run` (18,400-byte body, both passes) | **3,172** events with runs; 75,635 with one-byte pulls through the same loop (≈ 2 per byte); the former design took one event per compressor phase (≈ 5–7 per byte per pass) |
| site plan open (registered route, `s5-site`) | **201 intake + 803 settle turns**, inside the 1,024 the test allows (before: did not finish within 1,024) |

Pack cursor laws: 8 of 9 pass. `borrowed_pack_document_derived_expression_joins_original_symbol_and_default_codec` fails before the cursor runs, in its own reference oracle `encode_document_controlled` (`original work requires its original caller loan`): foreign, not caused by this slice. The chunk-frontier law named a `ChunkHash` phase that slice 3 merged into `ChunkRead`; the law and its fixture drop that frontier.

#### 5. Route status (registered browser route, browser grant contract)

| fixture | open | what binds now |
| --- | --- | --- |
| site plan (30,709 B) | settles: 1,004 turns, 330,730,041 B copied, 10,189,538 B retained, 4,418,190 B released, 56 s | copy bound `32 MiB + 24 × file` = 34,291,448 B: decode 11.2 MB, lift 72.5 MB, Pack export ≈ 247 MB. The deflate copy model is ≈ 40 B per raw byte per pass (insert 23, search 11) over a typed document (12,843 operators plus their role mirror) two orders of magnitude larger than the file |
| fonts fixture (71 KB) | decode and lift complete | Pack export of the typed snapshot exhausts the scope (see 3) |
| bachelor thesis (6.3 MB) | prepare 151 MB (was 311 MB) | the lift does not finish within 4,096 turns |
| institution diagram | not routed | |

No real-world fixture reached Save on the route, so `real-world-oracle.py` (pymupdf) still has no saved file to check; the proof (task 5) is not done. No bound was loosened.

#### Remaining barriers, in order

1. Pack export volume of large typed documents (copy ≈ 80 B per raw byte over two deflate passes): compact sample bodies, a typed snapshot proportional to the file, or a measured-once segment length so the second pass does not deflate again.
2. Thesis lift turns, then the prepare text and structure passes (exact receipts instead of 128 B per unit).
3. Lifter barriers not started: images inside forms, page-level colour spaces, patterns and shadings, `AcroForm`, `OutputIntents`, embedded files; paged operator lists above 16,383 operators.

#### Verification

`s5-native-save` 15 / 15; `s5-final` (after the traces were removed): testkit 22 / 22 filtered (`controlled_real_world` with the new ledger clause, `document_contracts`, `page_content_contracts`, xref index) and graph-projection 7 / 7 (retained-graph and stream-length tests over the reused projection box); framework Pack cursor 8 / 9 (`s5-pack`, the failure is the foreign oracle above). Not re-run: the interpreted-stream walk and normal-projection unit modules in their own crates, and the long regression scripts.

#### Shared-tree note

Builds on this machine were blocked by another session's temporary `mod dbg` hooks (`#[path = …/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs]`) in `🧳️owned/🏗️record-machine`, `🧳️original`, `🧬️protocol` and the store `🫴️record` crate: their generated file is not replicated to this machine. Their hooks are untouched; a local always-off stand-in `debug-ledger.rs` (`on() == false`) was placed in that session's `🗑️generated` folder so this machine compiles.

#### Slice 5 files

- Native Save: `✏️editor/💾️native-save/🧬️graph.rs` (`reference_extent`, `paged_extent`, `paged_page_frame`, `stream_pages`), `🧱️program.rs` (only temporary traces, removed), `🧪️tests/🔬️original-save/🦀️.rs` (only temporary traces, removed).
- Prepare: `🔨️modules/🔗️xref/📦️normal-projection/🦀️.rs` (`restart`, `frames_bytes`), `🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs` (one projection box, framing as body bytes), `🔨️modules/🔤️lexer/🧭️normal-object/🦀️.rs` (`lexes_bytes`), `🚪️io/📥️import/🔁️controlled/🦀️.rs` (only temporary traces, removed).
- Lift law: `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🧪️tests/🔬️unit/🦀️.rs` (release ledger clause).
- Route report: `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`OpenCost` reports retained and released bytes; `real_world_round_trip` prints the open and reopen cost).
- Framework Pack: `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs` (`segment_run`, `segment_step`, `SEGMENT_RUN`), its `🧪️tests/🦀️.rs` (new run law, staged-field assertions, stale `ChunkHash` frontier) and `🧪️tests/🧫️fixtures/🧱️chunks/🔣️.json`.

### Slice 6: packed image bodies, image codecs behind logical filters, per-stage cost census (2026-11-25)

Native, debug, every cargo run through `native-cargo.sh` (FIFO gate) wrapped by `watchdog-cargo.py` (`run-s4.sh`, new `run-s6-final.sh`); no wasm rebuild. Logs `🗑️generated/s6-*.log`. The temporary `[DEBUG]` per-stage import trace (`🔁️controlled`) and the Encode copy counter (framework `📥️natural-file`) are removed again; the framework file is back to its committed text.

#### 1. Compact typed snapshot: image samples at their real bit depth (schema change, every twin)

`PdfImageBody::Samples { values: Vec<u32> }` (one `u32` per logical sample) is now `PdfImageBody::Samples { packed: Vec<u8> }`: the native packed rows of §8.9.3 — height rows of width × components samples at the image's depth (1, 2, 4, 8, 16), most significant bit first, 16-bit big-endian, every row padded to a byte — which is exactly a decoded image stream, so the body costs what the file costs and lift and Save copy it without unpacking or repacking. The field is an intrinsic octet role (`pack::value::bytes`, canonical JSON `Octets`), so Pack carries it as one byte string instead of one value node per sample.

- Schema accessor (`🧬️schema/📸️snapshot/📐️model/🖼️xobjects`): `packed_sample_geometry`, `PdfPackedSamples` (a borrowed read view: `new` refuses a body whose length differs from its grid, `get`, `iter`, `len`), `pack_samples` (logical samples → rows; used by tests and fixtures only), `PdfImage::{packed, components, depth, packed_samples}`, and the same three accessors on `PdfInlineImage`. Constructors `rgb8` / `gray8` take their 8-bit rows directly; `PdfImage::samples` is gone.
- Consumers read through the accessor: canvas preview (`🎭️sampled-alpha`: `Samples::Packed(PdfPackedSamples)`), native Save (`💾️native-save/🧬️graph.rs` quotes a samples body by its bytes plus the paged stream it lowers to; `🔏️proof.rs` compares packed bytes in `COMPARE_WORK` runs), image lowering and inline-image printing write `packed` as the stream body after the grid check (`packed_samples_of`), the layout PDF interchange reads 8-bit rows directly, image authoring and `replace-image-samples` store rows.
- Lift: `PdfImageSampleCursor` (one `u32` written per unit) is replaced by `PdfImagePackedCursor` (one exact backing of the packed length, bounded byte runs of `grant.work`, partial reads looped, bounded close); `admit_packed_samples` validates and moves the decoded bytes (the inline-image parser no longer copies them). `inline_image_extent` drops its `4 × samples` term. `INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES` is now measured in packed bytes (`1 << 18`), which keeps the classification of every 8-bit image (the only depth the sampled-artifact path admits) exactly where it was.
- Twins: JSON schema (`packed`: integers 0..255), proto (`bytes packed = 2`), GraphQL (`packed: [Int!]`, type and input), TS schema, native-JSON reader, native-JSON schema TS; SQLite: tables `pdf_image_sample` and `pdf_inline_sample` are dropped, `pdf_image` and `pdf_inline_image` gain a last `packed BLOB` column with `CHECK ((body_kind = 'samples') = (packed IS NOT NULL))`; Rust and TS writers, readers and the admission census follow. Fixtures converted by hand-checked script (`s6-pack-fixtures.py`, schema edits `s6-schema-twins.py`): all consistent fixtures are 8-bit, so their `packed` equals their former values; the SQLite semantic census fixture lost the dropped-table rows (rows 1,289 → 1,286, bytes 337,504 → 337,411).

Measured (fonts fixture, registered route, `s6-route2`/`s6-route3` versus `s5`):

| lift of the fonts fixture | before (u32 samples) | after (packed) |
| --- | --- | --- |
| `type3-glyph` stage (42 glyph programs, 1-bit inline images up to 188 × 142) | 4,896,239 B copy, 4,890,863 B allocated | **605,775 B copy, 600,399 B allocated** (8.1×) |
| scope at failure: retained / released | 10,683,533 / 5,129,472 B | **6,284,589 / 725,752 B** |

Other inflation measured, not removed in this slice: every page's operators are held twice (typed `PdfPage.content` and the `Operators` stream role the graph projection renames from; site plan 12,843 operators → 821,996 B role mirror on top of the page), and the retained graph still holds every interpreted stream decoded next to its typed form. Removing the mirror needs the graph projection to rename from the page itself (schema of `PdfStreamRoleValue::Operators`); not started.

#### 2. Image codecs behind logical filters (regression of slice 4, found by `canvas_paint_matches_mupdf_probes`)

Since slice 4 an image whose chain is a logical codec followed by an image codec (`[/ASCIIHexDecode /CCITTFaxDecode]`, `… /JBIG2Decode`, `… /JPXDecode`, `… /DCTDecode`) was kept encoded: the controlled decoder (`🗜️filters/🎚️decoded-stream`) refused every image codec except a lone DCT, so the lifter typed the hex text as samples and the import failed ("image packed body does not match its declared geometry"). The decoder now admits DCT, CCITT, JPX and JBIG2 (without `JBIG2Globals`) as the **last** stage of a chain on an `/Subtype /Image` stream, as an identity stage that types the codec with its parameters (`ccitt_parameters`, the same reader as the reference `decode_stream`), so the walk yields the same `[AsciiHex, Ccitt]` pipeline the reference reader does. `canvas_paint_matches_mupdf_probes` passes again.

`mesh_move_shifts_the_decode_range` failed since slice 3's preserved-content guard: its mesh pointed at a fake artifact id, so the canvas could not paint it and the move was refused (`stdio.pdf.content-preserved`). The test now admits a real type-4 triangle (`admit_pdf_artifact`) with full decode ranges and asserts the moved decode and the same reference; it passes. The guard was not loosened.

#### 3. Where the copy goes (temporary per-stage census, removed)

| document | stage | units | copy charged |
| --- | --- | --- | --- |
| site plan (30,709 B) | prepare-file | 67,988 | 10.4 MB |
| | lift `content` (12,843 operators, 141 KB decoded) | 548,088 (≈ 3.9 per content byte, one per call) | **70.4 MB** (128 B per unit) |
| | lift `content-role` mirror | 12,847 | 1.6 MB |
| | Pack export (Encode) | | **246.1 MB** (exact, the whole Encode stage) |
| fonts (71 KB) | prepare-file | 22,620 | 5.6 MB |
| | lift (all stages) | ≈ 2,000 | ≈ 1.3 MB |
| | Pack export | | > 503 MB, the scope's 528 MB copy stock ends inside Encode |
| bachelor thesis (6.3 MB) | prepare-file | 779,136 | 149.8 MB |
| | lift `content` (still running at the end) | 2,020,813 | **276.0 MB** |
| | lift `image` (packed copy, one byte per call) | 360,030 | **46.1 MB** |
| | lift `type3-glyph` / `annotation` / `resources` | 646 / 951 / 544 | 11.8 / 5.6 / 4.3 MB |

So the binding costs are (a) byte-level lift units driven one per call and charged the 128-byte token rate (content lexing, image byte copy), and (b) the Pack export, which spends ≈ 24 B of copy per byte of the typed document per pass even for the small fonts snapshot.

#### 4. Route status (registered browser route, browser grant contract, `s6-route3`)

| fixture | open | what binds now |
| --- | --- | --- |
| site plan (30,709 B) | settles: 201 + 800 turns, 330,729,617 B copied, 10,184,930 B retained, 4,418,158 B released, 50 s | copy bound `32 MiB + 24 × file` = 34,291,448 B (assert unchanged): Pack export 246.1 MB + lift content 70.4 MB |
| fonts fixture (71 KB) | decode and lift complete (lift now 8× smaller in its glyph stage) | Pack export exhausts the 528 MB scope stock after 4,107 turns |
| bachelor thesis (6.3 MB) | prepare 149.8 MB | lift content and image units exhaust the 528 MB stock after 4,106 turns |
| institution diagram (648,725 B; route test new in this slice) | does not settle within its 1,024 turns (`512 + 512 × MiB`) | not yet broken down (the run with the census excluded it) |

No real-world document reached Save on the route, so the lopdf/pymupdf proof (`real-world-oracle.py`) still has no saved file; no bound was loosened.

#### Not done (in order of yield)

1. **Bulk lift units** (task 3): let the importer drive byte-level lift stages as body units (64 cursor units per contract unit, 2 bytes of copy per byte), as prepare already does: `PdfImagePackedCursor` and the sampled/DCT identity hashes already consume `grant.work` bytes per call (only a `ControlledDocumentLiftCursor::copies_body_bytes` predicate and the importer's `decodes_body_bytes` branch are missing, −46 MB on the thesis); content lexing needs the page-content cursor to take more than one token per call (−70 MB site plan, −276 MB+ thesis).
2. **Single-pass Pack export** (task 2): the measure pass deflates every segment only to learn its stored length; retaining the compressed segment (bounded by the output) and emitting it in the second pass halves the deflate work; already-encoded stream bodies should enter the segment as stored blocks. Framework cursor `🎒️document/⏳️cursor`; not started.
3. **Operator role mirror** (above) and the decoded copy of interpreted streams.
4. Lifter barriers of task 5 (images inside forms, page-level colour spaces, patterns, shadings, `AcroForm`, `OutputIntents`, embedded files, paged operator lists above 16,383 operators): not started.

#### Verification

`s6-check2`: `check --workspace --tests` of the PDF workspace, exit 0 (after the compile-memory agent's own derive fix of a foreign `OptionCursor<PdfPredictor>: Send` overflow in `semio-s-artifact-stdio-pdf-snapshot-2`). `s6-unit`: PDF lib 38/38, io-export 19/19, page-editor 67/69 (the two failures above, both fixed and passing in `s6-route3`). `s6-final` (after the traces were removed): PDF lib 38/38, io-export 19/19, page-editor 69/69 (incl. `canvas_paint_matches_mupdf_probes`, `mesh_move_shifts_the_decode_range`, `canvas_repaints_image_samples_after_replacement` with packed masks), `semio-s-artifact-stdio-pdf-snapshot-2` 2/2, testkit 64/64 (ledger, lift-unit law, measured program extents with the new inline-image quote): **192/192**. TS (delegated): the four SQLite TS test files touched pass under bun (8 + 15 tests). Route tests (`s6-route3`, debug native): site plan, fonts, thesis fail at the bounds stated above; the new institution-diagram test fails at its turn bound (`s6-route2`). Not verified: the layout plugin IO crate (`s6-layout` stopped earlier, in `semio-plugin-instance-app`, on another session's unlinked `semio_framework_trace` hook); the long regression scripts were not run.

#### Slice 6 files

- Schema: `🧬️schema/📸️snapshot/📐️model/🖼️xobjects/🦀️.rs`, `📐️model/🖋️content-operators/🦀️.rs`, `📐️model/🦀️.rs`; twins (by a delegated agent, reviewed): `🧬️schema/📸️snapshot/🔣️.json`, `🛰️.proto`, `🔗️.graphql`, `🖼️resource/🟦️.ts`, `🚪️io/📝️text/📸️snapshot/🪪️native-json/🧱️image-body/🟦️.ts`, `…/🪪️native-json/🧬️schema/🖼️resource/🟦️.ts`, `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql`, `…/🖼️resource/🟦️.ts`, `…/🖋️content/🟦️.ts`, their TS tests and fixtures (`🧪️tests/🖋️content/🟦️.ts`, `🧪️tests/🖼️resource/🟦️.ts`, five JSON fixtures, the semantic census fixture).
- Rust consumers: `🔨️modules/🖼️images/🦀️.rs` (+ unit tests), `🖼️images/🎞️controlled/🦀️.rs`, `🖼️images/🎭️sampled-alpha/🦀️.rs`, `🖼️images/🧪️tests/🔬️unit/🦀️.rs`, `🔨️modules/🖋️content/🦀️.rs` (+ `🧪️tests/🔬️unit/🦀️.rs`), `🚪️io/📦️foreign-artifacts/🦀️.rs`, `🚪️io/🪶️sqlite/📸️snapshot/🖼️resource/🦀️.rs`, `…/🖋️content/🦀️.rs`, `…/🛂️admission/🖼️resource/🦀️.rs`, `…/🛂️admission/🖋️content/🦀️.rs`, `✏️editor/💾️native-save/🧬️graph.rs`, `✏️editor/💾️native-save/🔏️proof.rs`, `✏️editor/🖼️page/🖱️object-edits/🦀️.rs`, `✏️editor/📄️document/🖼️image-authoring/🦀️.rs`, `…/📌️dct-placement.rs`, layout plugin `📏️layout/…/🚪️io/🔀️interchange/📕️pdf/🦀️.rs`.
- Decoder: `🔨️modules/🗜️filters/🎚️decoded-stream/🦀️.rs` (terminal image codecs).
- Tests: `✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs` (mask fixture packed, mesh test), `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (new `registered_real_world_institution_diagram_…`), `🔨️modules/⬇️lift/…/🧭️controlled/🧪️tests/🔬️unit/🦀️.rs`, `🚪️io/📤️export/🧪️tests/🔬️unit/🦀️.rs`, `🧬️schema/🪪️stream-roles/🧪️tests/🦀️.rs`.
- Ticket inputs: `run-s6-final.sh`, `s6-pack-fixtures.py`, `s6-schema-twins.py`.

### Slice 7: run-based lift, one-pass Pack export, a sorted-index Pack symbol table (2026-11-25)

Native, debug, every cargo run through `native-cargo.sh` (FIFO gate) wrapped by `watchdog-cargo.py` (`run-s4.sh`, new `run-s7.sh`, `run-s7-final.sh`; framework crates with `NATIVE_CARGO_DIR=🧰️framework`); no wasm rebuild. Logs `🗑️generated/s7-*.log`. The temporary `[DEBUG]` censuses of this slice (import stages in `🔁️controlled`, Pack document phases in `🎒️document/⏳️cursor`, Pack source phases in `🫳️borrowed/⏳️cursor`, the typed-volume probe test) are removed again. The `[DEBUG] start_segment …` detail now in `🎒️document/⏳️cursor::start_segment` and the new law `borrowed_pack_document_every_frame_split_and_body_length_restarts_with_complete_custody` belong to another session working on the same cursor and were left alone.

#### 1. Run-based lift (task 1)

- `ControlledDocumentLiftCursor::runs_bodies` / `advance_bodies` (`⬇️lift/…/🧭️controlled`): the page program and image byte stages run as **body runs** of up to `grant.work` run units of `RUN_UNIT_COPY = 2` bytes. A unit that moves source bytes (joining split contents, lexing) costs one run unit per byte; every other unit is charged exactly the bytes it writes (`unit_copy`, rounded up to run units, at least one): taking/storing/retiring an operand moves one `PdfObject`, applying/text/number steps write at most one operator or array item (`TYPED_UNIT_BYTES`), collecting/migrating moves one `PdfOp`, reserving the doubled vector writes nothing. A run stops before an empty reservation (so the importer still books it against the release grant), when the next unit no longer fits, at completion and at any event but `More`. `RUN_UNIT_MAXIMUM` is the floor the importer grants.
- Image stages already moved `grant.work` bytes per call (`PdfImagePackedCursor`, DCT/sampled identity hashes); `ControlledImageResources::copies_bytes` exposes them as runs.
- Importer (`🚪️io/📥️import/🔁️controlled`): `decodes_body_bytes` covers prepare bodies and lift runs; lift runs get at least `RUN_UNIT_MAXIMUM` units; `BODY_COPY_PER_BYTE` is the lift's `RUN_UNIT_COPY`.
- Content lexing no longer builds a whole `PdfCosCursor` (64 frames, ≈ 4.5 KB) for every operand and a second one for every retired operand: `PdfCosCursor::idle` / `rearm` / `rearm_retiring` (`🔤️lexer/🧱️containers`, `Reader::rearm`) re-arm one settled cursor in place; `ContentTokenCursor` and `ContentProgramCursor` own one each.
- New law `body_runs_lift_every_real_world_document_like_single_units_in_fewer_calls` (testkit): every ledger document reaches exactly the single-unit snapshot or barrier under runs of 4,096 units, every run inside its grant, no unit above one browser turn, no over-credit; calls (single units → runs): site plan 582,719 → 56,400, thesis 3,814,557 → 350,650, institution 395,477 → 61,203, fonts 2,116 → 532 (ledger total ≥ 4×).

| site plan, lift stage (registered route) | before (s6) | after |
| --- | --- | --- |
| `content` | 548,088 calls, 70.4 MB | **296 calls, 11.9 MB** (282 runs) |
| `content-role` mirror | 12,847 units, 1.6 MB | unchanged (task 3 not done) |

#### 2. Pack export: one projection and one deflate per segment (task 2, framework `🎒️pack/…/🫳️borrowed`)

- `BorrowedPackDocumentCursor` keeps every compressed byte of the measuring pass in 256-byte blocks (`store`, a `PagedList<Held<[u8; STORE_BLOCK]>>`, reserved and released under quotes like the frame catalog; bounded by the final output). The emitting pass no longer restarts the source projection or the compressor: after the framing the measured lengths determine, `emit_stored` lends the kept bytes a block run per event and checksums them. Output bytes are unchanged (canonical-oracle laws). The source-request law only applies to the measuring pass now; the run law is tightened from `events × 4 < raw` to `events × 8 < raw`; the feature file says what the cursor now does.
- **Why a 71 KB file cost > 503 MB to encode** (measured, `s7-run2`): the typed fonts snapshot is only ≈ 0.6 MB raw (35,090 value nodes; objects 230 KB, roles 206 KB, fonts 160 KB), but it holds 1,670 distinct strings (glyph and encoding names, font names, text), and the Pack source projection kept its symbol table sorted by **linear search** (every key and string occurrence compared against the table from the start, in discovery and again in the body) and by **insertion with whole-symbol swaps** (each `Symbol` carries a 64-slot source path, 3 × 560 B per swap, quadratic in distinct strings). Source pulls of the fonts export copied 340.4 MB in 245,597 pulls (1,386 B each) before the stock ran out. Removed: discovery and the body lookups (scalar text, table text, map keys, statement keywords) binary-search the sorted table (`rank` precomputed at selection), and the table is sorted through a `u32` order index (`order`, swaps of 3 × 4 B) while symbols stay in discovery order. The fonts export now completes: source pulls 9.2 MB (insert 7.0 MB of it), whole measuring pass 84.1 MB, emission 0.29 MB.
- What remains of Pack export cost is the compressor's own copy model (≈ 40 B per raw byte: site plan measuring pass 108.7 MB of which source pulls only 2.9 MB). Storing already-encoded PDF streams as stored blocks was **not** done: the canonical codec1 output is the oracle every twin reproduces, so a stored-block choice must first become part of the canonical format (schema-first), not a cursor shortcut.

#### 3. Route status (registered browser route, browser grant contract, `s7-run3`)

| fixture | open | what binds now |
| --- | --- | --- |
| site plan (30,709 B) | settles: **201 + 238 turns** (was 201 + 800; 201 + 241 in `s7-routes`), **136,046,357 B copied** (was 330,729,617), 7,871,882 B retained, 2,105,158 B released, 18.6 s (was 50 s) | copy bound 34,291,448 B (unchanged): Pack measuring pass 108.7 MB (deflate copy model ≈ 105.7 MB, source 2.9 MB), lift content 11.9 MB, prepare 10.4 MB, role mirror 1.6 MB |
| fonts fixture (71 KB) | **opens** (decode, lift and Pack export complete; was out of the 528 MB stock inside Encode) | Save poll refuses: "original lifecycle child receipt differs from its whole producer receipt" (framework receipt reconciliation; not yet attributed) |
| bachelor thesis (6.3 MB) | not re-run on the route in this slice (lift runs: 350,650 calls instead of 3,814,557 in the law) | — |
| institution diagram (648,725 B) | does not settle within 1,024 turns | broken down (task 4): one page, 1,345 form XObjects, one classic xref table of 4,043 entries at 567,769; at the turn limit the import is still in `prepare-file/bytes/structure` after 199,998 units / 26.4 MB — the xref structure pass runs one unit per call at the 128 B token rate (≈ 2.5 units per table byte) |

No bound was loosened; no real-world document reached the reopened Save, so the lopdf/pymupdf proof (task 5) still has no saved file.

#### Not done (in order of yield)

1. **Xref structure pass as body bytes** (institution diagram): `StructuredXrefCursor` (`🔗️xref/🛤️structured-file` → `🧭️revisions` → `📑️classic`) needs a `scans_bytes` predicate and `PdfFilePreparationCursor::decoding_body_bytes` its branch, exactly like slice 5's `lexes_bytes`; the retained-text pass (`prepare-file/text`, 22,412 units / 2.9 MB on the site plan) likewise.
2. **Compressor copy model** of the canonical deflate (≈ 40 B per raw byte) — now the largest open cost of every route; and a canonical stored-block rule for already-encoded streams (schema-first in the codec1 format).
3. **Operator role mirror** (task 3, ≈ 1.6 MB lift and half of the site plan's typed volume): one owner (`PdfPage.content`), the `Operators` role of page contents a reference; reference lifter, controlled lifter, graph projection, native Save and every twin change together. Not started.
4. Fonts fixture Save: attribute the receipt-reconciliation refusal (`🏛️ownership/⚠️failure::original_lifecycle_unposted`).
5. Lifter barriers of task 6 (images inside forms, page-level colour spaces, patterns, shadings, `AcroForm`, `OutputIntents`, embedded files, operator lists above 16,383) and the layout IO crate check: not started.

#### Verification

`s7-final` (after every trace was removed): PDF lib 38/38, io-export 19/19, page-editor 69/69, snapshot-2 2/2, testkit 65/65 (incl. the new run law, the ledger, the browser-grant law and the measured program extents): **193/193**. `s7-routes` (registered route, snapshot-value crate): draft publish/undo, page edit, nested page tree Save, outlines/names/links Save, several fonts and images open — **5/5 pass** with the one-pass Pack export behind them; site plan fails at its copy bound (201 + 241 turns, 136,046,749 B), fonts fixture at its Save receipt (as in §3). Framework Pack crate (`s7-pack`): 15/17 — the two failures are foreign and failed before this slice's changes too: `borrowed_pack_document_derived_expression_joins_original_symbol_and_default_codec` (its own reference oracle, see slice 5) and `borrowed_projected_pack_native_paged_key_and_text_cross_unicode_pages` (the value-text rig `staged_rig::encode_success::<_, 8194>` refuses with "text formatter exceeds its original capacity" before the cursor runs). Not run: the long regression scripts, the thesis and institution routes after the last change, the layout IO crate.

#### Slice 7 files

- Lift: `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (`RUN_UNIT_COPY`, `RUN_UNIT_MAXIMUM`, `runs_bodies`, `advance_bodies`, `program_run`), its `🧪️tests/🔬️unit/🦀️.rs` (`controlled_lift_driven`, new run law), `📜️page-content/🧭️controlled/🦀️.rs` and `🧺️program/🦀️.rs` (`unit_copy`, `UNIT_COPY_MAXIMUM`), `🔨️modules/🖋️content/🧵️program/🦀️.rs` (`unit_copy`, persistent retirement cursor), `🔨️modules/🔤️lexer/🎫️content-token/🦀️.rs` (persistent operand cursor), `🔨️modules/🔤️lexer/🧱️containers/🦀️.rs` and `🔡️tokens/🦀️.rs` (`idle`, `rearm`, `rearm_retiring`), `🔨️modules/🖼️images/🎞️controlled/🦀️.rs` (`copies_bytes`), `🚪️io/📥️import/🔁️controlled/🦀️.rs` (body runs, floor).
- Framework Pack: `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs` (kept compressed segments, `emit_stored`), its `🧪️tests/🦀️.rs` and `🥒️.feature`; `🫳️borrowed/⏳️cursor/🦀️.rs` (binary-searched symbols, `rank`, `order` index) and its `🧪️tests/🦀️.rs`.
- Ticket inputs: `run-s7.sh`, `run-s7-final.sh`.

### Slice 8: fonts Save receipt (2026-11-25)

Native, debug, every cargo run through `run-s8.sh` / `run-s4.sh` (FIFO gate + watchdog); logs `🗑️generated/s8-*.log`. All temporary `[DEBUG]` traces of this slice (media `retire_nonterminal_outcome_funded`, `🧱️program` route, `🧷️increment` page refusals, `🪡️graft` rounds) are removed again; the previous session's `unposted refusal` trace was no longer in the source.

#### Cause

1. **The receipt refusal did not reproduce on the current tree.** The three runs `s8-fonts3` … `s8-fonts9` never printed `original lifecycle child receipt differs from its whole producer receipt`: the Save poll of the fonts law completes (`ActiveMediaExport::retire_nonterminal_outcome` alternates `outcome_pending` / `resume_pending` with matching receipts: acknowledgement step = `original_lifecycle_delta(before, after)`, handback = 1 item, 65 B). The earlier refusal (`whole = 0`, `before.copied_items 28649`, `after 28650`, backtrace through `retire_nonterminal_outcome` → `original_lifecycle_step`) was a window where a handback poll that returned no progress while the context had moved by one item (the `resume_pending` AwaitingInput/zero-grant exits return `Progress(0)` but the surrounding loan can post one item); the concurrent framework changes of the other sessions in the same area (`original_lifecycle_step`, media export) removed it, and no framework code needed a change in this slice. Left as it is: `original_lifecycle_unposted` stays the strict four-axis check.
2. **The real fonts Save failure was in the PDF planner.** With the poll fixed, the law failed at `assert_appended`: "the saved file must keep every byte of its base" — Save had written a **217,547-byte compact rewrite** of the 81,372-byte file instead of an appended section. Traced (route, page refusal line, graft round): `IncrementPlan::lift_page` → `Graft::round` → `patch_page` → `lower_page_standalone` → `Lowering::new` → `FontCodec::new(font, artifacts)` for every typed font → `NativePdfArtifactResources::resolve` → `Unsupported("foreign PDF artifact pdf:s.stdio.font.type1:sha256:… has not been admitted")`, which `lift_page` maps to `Compact::Page`. The page-slice probe snapshot was built with `objects: Vec::new()`, but a foreign font program (embedded Type 1 / TrueType / CFF stream) is resolved by identity from `snapshot.objects` (`NativePdfArtifactResources::from_objects`). Every earlier incremental law used documents without embedded font programs, so the empty `objects` never mattered.

#### Fix

- `✏️editor/💾️native-save/🧷️increment.rs` (`lift_page`): the probe snapshot now carries `objects: final_reading.lanes.objects.clone()`, the foreign resources (font programs, image sources) the slice's own reading admitted. `Moved::between` ignores `objects`, so the lane comparison is unchanged; trailer, roles and every other lane stay empty as before. No compatibility path, no fallback: the planner states the page against the same artifact table its reading holds.
- Unrelated compile blocker fixed on the way (another session's in-progress edit in the oracle crate): `🔮️oracles/🦀️.rs` line 1155 compared `Result<&str, &lopdf::Error>` with `==` (`lopdf::Error` has no `PartialEq`); now `.is_ok_and(|text| text == "Widget")`.

#### Tests and results

- The law `registered_pdf_with_type3_embedded_and_composite_fonts_opens_edits_and_saves_like_the_reference_readers` pins the cause: it asserts `assert_appended(input, &saved, 16 KiB)` (saved starts with every base byte, appended section below 16 KiB), lopdf page count and per-page font declarations of the saved file, and reopen (title, fonts retained, `Hello` present). **Passes** (`s8-fonts10`, 32 s); saved file 82,075 B = 81,372 B base + 703 B section (was 217,547 B rewrite).
- pymupdf (`font-oracle.py` over `🗑️generated/real-world/fonts-saved.pdf`): both pages `fonts`, `kinds`, `programs`, `text` all OK for the original and the saved file.
- Neighbours (`run-s8-final.sh`, `s8-final`): PDF lib 38/38, io-export 19/19, page-editor 71/71, snapshot-2 2/2, testkit 66/66 = **196/196**. Registered routes (`s8-routes`, snapshot-value crate): draft publish/undo, page edit, nested page tree, outlines/names/links, several fonts/images, fonts fixture: **6/6 pass**. Not run: site plan, thesis and institution routes (copy-bound work of slice 7, unrelated to this change), wasm rebuild.

#### Files changed

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/💾️native-save/🧷️increment.rs`
- `…/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs` (compile fix)
- Ticket inputs: `run-s8-final.sh`.

### Slice 9: xref and text passes as body bytes (2026-11-25)

Native, debug, every cargo run through `native-cargo.sh` (FIFO gate) wrapped by `watchdog-cargo.py` (`run-s9.sh` over the replace-page unit crate `semio-s-artifact-stdio-pdf-mut-family-11`, which compiles the retained-graph tests; `run-s7.sh` for the registered routes); no wasm rebuild. Logs `🗑️generated/s9-*.log`. Every temporary `[DEBUG]` trace is removed again. Incident: one `pkill -f watchdog-cargo.py` of mine also ended other sessions' watchdogs; no further process was killed except my own PIDs.

#### 1. Cause

Slice 7 left the institution diagram (648,725 B, one classic xref table of 4,043 entries) in `prepare-file/bytes/structure` after 199,998 units / 26.4 MB: every footer, probe, table-row, validation and index-slot unit was one call at the 128 B token rate, and the retained-text pass (22,412 units / 2.9 MB on the site plan) likewise. Measured with the new law (single units): institution 344,598 and site plan 24,208 calls in these two passes, thesis 458,816.

#### 2. Fix

- Scanning units are body bytes exactly like slice 5's `lexes_bytes`: `XrefRevisionCursor::scans_bytes` (probe and classic rows, `ClassicSectionCursor::scans_rows`), `run_write_bytes` through `XrefAssemblyCursor`, `StructuredXrefCursor`, `PdfFileStructureCursor` (header, footer, validation scan 1 byte; index insert and merge `size_of::<Option<XrefRow>>` = 24 bytes), `PdfByteGraphCursor` and `PdfFilePreparationCursor::{run_write_bytes, writing_bytes, decoding_body_bytes}`. A 1-byte unit joins the bulk body run (2 B copy, 64 units per contract unit); a unit that fills a slot is a run of index units each charged its 24 bytes (`record` charges `copy_body` per cursor unit, the importer sets it to the written bytes, never above half the copy grant).
- Retained text: `RetainedTextCursor::run_write_bytes` (mark, walked value: 1 byte; discovered role queued: 16 bytes) and a run loop in `advance` that repeats the identical single unit while the next one only scans and nothing allocates, releases or converts; each unit is the unit a single-unit grant runs.
- Importer (`🚪️io/📥️import/🔁️controlled`): `writing_bytes`, `running`, and the allocation fix below.
- Found by running the institution diagram: the bulk work of a body run reserved up to half the copy grant (`work × 2`), which left the allocation boundary at 262,089 B while the object-2 dictionary vector needs 278,528 B: every call answered `NeedsAllocation` after one unit, one unit per turn, 1,150 turns without progress (slice 5's logic, hidden until the structure pass finished). Now the call after a `NeedsAllocation{bytes}` sizes its run so that `max_copy - run copy >= bytes` (`run_work`, field `demanded`).

#### 3. Tests

- `body_runs_prepare_every_real_world_document_like_single_units_in_fewer_calls` (retained-graph tests): every ledger document prepares to the same objects, trailer and header under runs of 4,096 units as under single units, no run copies more than one browser turn (512 KiB), whole ledger 14,834,215 -> 696,854 calls (21x), structure and text passes 863,091 -> 103,965 (8.3x, asserted at least 8x). Institution 2,146,811 -> 339,393 calls (structure 211,033 -> 16,359, text 133,565 -> 215); site plan 385,790 -> 41,535; thesis 11,681,320 -> 294,883.
- `a_bulk_run_leaves_the_demanded_allocation_fundable` (importer): for demanded allocations from 0 to the whole grant a run leaves the demand fundable, and a 278,528-byte demand shortens the run.
- Independent oracle: equality with the single-unit result (the reference path); the readers lopdf and pymupdf only see the saved file, which no real-world fixture reaches yet.

#### 4. Route table (registered browser route)

| fixture | before (slice 7) | after |
| --- | --- | --- |
| institution diagram | in `bytes/structure` at 199,998 units after 1,024 turns | structure pass ends before turn 16; projection to turn 208; interpret of the 1,345 forms turns 208 to 800 (about 600 turns, 73K units); lift from turn 800; limit 1,024 reached inside the lift (374K units) |
| site plan | 201 + 241 turns, 136,046,357 B | 201 + 240 turns, 133,128,508 B (copy bound 34,291,448 B still binds: Pack export 108.7 MB, deflate model) |
| thesis | prepare 151 MB, did not settle in 4,096 turns | still does not settle in 4,096 turns (`s9-final`); stage not yet attributed (the per-turn `[DEBUG]` census was only run on the institution diagram) |

#### 5. Not done

1. **Institution diagram, interpret pass** (about 600 of the 1,024 turns, units 230K to 303K): 1,345 small form streams each pay a fixed run of calls (visit, decoder, history reservation, close decoder, sanitize, swap, retire dictionary, retire body); the per-turn census showed the turn copy grant exhausted at turn start through most of the pass, so it is copy-bound, not unit-bound. A body-run predicate for the walk's skip and retire units is not enough on its own; the fixed per-stream cost (or a decoder reused across streams) must drop. Not started in code. After it the lift (from turn 800) still needs more than 224 turns.
2. **Thesis**: attribute the stage that exhausts 4,096 turns (run the per-turn census of this slice on it).
3. **Site plan copy bound**: unchanged, Pack export deflate (about 40 B per raw byte: Input 1, Search 11, Compare 2, Token 1, Insert 23); the Insert re-reads the head slot that Search just read (11 of 23 bytes) but sharing it would change quoted receipts of every exact-receipt law for at most about 15%, far from the 3x the bound needs, so it was not touched and no bound was loosened.

#### 6. Verification

`s9-law` (mut-family-11): `body_runs_prepare_every_real_world_document_like_single_units_in_fewer_calls` and `a_bulk_run_leaves_the_demanded_allocation_fundable` pass after the `[DEBUG]` removal. `s9-final` (registered routes): institution diagram and thesis do not settle (above), site plan at its copy bound. Not run: PDF lib, testkit, page-editor and framework suites after this slice.

#### 7. Files

- `🔨️modules/🔗️xref/` : `🧭️revisions`, `📋️section` (`scans_bytes`, `scans_rows`), `🧵️assembly`, `🛤️structured-file`, `🌐️file-structure` (`run_write_bytes`), `📥️retained-graph/🦀️.rs` (`PdfFilePreparationCursor::{run_write_bytes, writing_bytes, decoding_body_bytes}`, new law, `ledger_file`).
- `🚪️io/📝️text/📸️snapshot/🪪️retained-text/🦀️.rs` (`run_write_bytes`, run loop, `advance_unit`).
- `🚪️io/📥️import/🔁️controlled/🦀️.rs` (`writing_bytes`, `running`, `run_work`, `demanded`, new law).
- Ticket inputs: `run-s9.sh`.

### Slice 10: form images, page resources and AcroForm (2026-11-25)

Native, debug, every cargo run through `run-s4.sh` / `run-s7.sh` (gate plus watchdog); no wasm rebuild. Logs `🗑️generated/s10-*.log`. Inputs kept here: `form-images-fixture.py`, `page-resources-fixture.py`, `acro-form-fixture.py` (write the committed fixtures and their `spec.json`), `form-images-oracle.py` (pymupdf, over the fixtures and over the files the registered Save wrote), `run-s10-final.sh`, `run-s10-oracle.sh`.

#### Cause

Three constructs ended the controlled lift with `ReferenceRequired`, which the browser route cannot answer: `merge_typed` refused every image the reference builder typed inside a form (`XObject/Form/Resources/Image`); `Category` knew only `Font`, `XObject`, `ExtGState`, `Properties`, so page `ColorSpace`, `Shading` and `Pattern` resources stopped (`resource_shape`, `unsupported_resource`); the catalog `AcroForm` stopped (`catalog_entry`), and its widgets stopped one stage earlier (`cold_annotation` refused every annotation with an `/AP`). A fourth cause surfaced on Save, then reopen: Save writes every typed image into the page's flat pool, so a document Save produced (an Indexed image, a stencil mask) stopped at page level (`ControlledImageCursor` only types 8-bit DeviceGray/RGB).

#### Fix (controlled lifter `🧭️controlled`, reference lifter, images)

- Images in forms, patterns and appearances: `merge_typed` adopts the reference-typed images (`ControlledImageResources::adopt`, owner table kept aligned with `snapshot.images`), their roles merge before the content roles, quotes count the image bytes (`image_quote`, `form_quote` now tells images from forms, `reach_streams` for profiles, palettes, function samples).
- Page-level images the cursor cannot admit (any other colour space, stencil, name or object already typed) fall back to the reference builder (`ControlledImageResources::abandon`, `cold_form` now lifts forms and images) instead of stopping.
- Page `ColorSpace`, `Shading`, `Pattern`: new categories in reference order (`Stage::ColdResource`, `cold_resource`), direct entries under the page scope, renames recorded in the page's `ResourceMap` and applied by the existing `Rename` stage; reference side `Lifter::lift_colour_space_entry` extracted, `lift_shading_entry` and `lift_pattern` made public. `Encoding` in a resource dictionary is ignored like the reference.
- Annotations are lifted after the page content (reference order), annotations with appearance streams run over the persistent reference state (`cold_appearance`: forms, their fonts and images, roles), widget positions are recorded for the field tree (`annotation_refs`, quoted).
- AcroForm (indirect or direct in the catalog): new lane `Lane::Form` between page labels and viewer preferences. Default resources `/DR` run through the resource stages (`ResourceLocation::FormDefaults` for a direct one), the form's own entries through `Lifter::lift_acro_form_scalars`, then the field tree one field or widget-only kid per unit (`form_field`, frames, `Lifter::lift_form_field_head` and `form_widget_kid` split out of `lift_form_field`). Reference field lifting is now head first, then kids.

#### Tests and results

- Fixtures (committed under `🧪️tests/🧫️fixtures`): `🖼️form-images` (RGB, gray with soft mask, JPEG, Indexed, stencil; nested form; colliding names; shared form), `🎨️page-resources` (ICC, Indexed, Separation, device name; two shadings; shading and tiling patterns with an image; renamed and re-claimed names), `📝️acro-form` (pymupdf-built form with widgets and appearance streams, container, radio group, widget-only kids on two pages, `/DR` with `/Encoding`, direct `/AcroForm`). The three are ledger entries (`real-world/🔣️.json`, all `equal`).
- Laws: `form_images_page_resources_and_acro_form_lift_like_the_reference_lifter_and_both_readers` (controlled equals reference, units inside one browser turn, ids and counts equal spec, lopdf names the same image/form objects, resource names, field tree and widgets per page) passes; `real_world_documents_lift_like_the_reference_lifter_or_stop_at_their_ledgered_barrier`, `every_real_world_lift_unit_fits_one_browser_turn`, `body_runs_lift_every_real_world_document_like_single_units_in_fewer_calls` pass with the three new documents; `registered_pdf_with_form_images_page_resources_and_an_acro_form_opens_like_both_readers_saves_and_reopens` (registered browser route: open equals cold reader in every lane, lopdf, edit, Save, reopen with same ids, counts and field tree, lopdf field tree equal after Save) passes (`s10-run5`).
- pymupdf (`run-s10-oracle.sh`): the fixtures and the three saved files agree with their specs (2 pages, same images/forms, drawings, widgets `[8, 1]`, same field names and values).
- Slice 7 neighbours (`s10-routes`, registered route, snapshot-value crate): draft publish/undo, page edit, nested page tree Save, outlines/names/links Save, several fonts and images, Type 3/embedded/composite fonts and the new form-images law pass 7/8; the site plan fails exactly as in slice 7 at its copy bound (133,118,099 B copied against 34,291,448 B; slice 7: 136,046,749 B), not changed by this slice. Testkit ledger laws pass (`s10-run5`). The PDF lib/export/page-editor/snapshot-2 `--lib` group (`s10-final`) could not build: foreign compile error in `semio-s-artifact-stdio-binary-mutations`.

#### Not done

1. Images and forms inside forms are lifted atomically: one unit must fit the 512 KiB copy grant, so an image above about 120 KB inside a form still needs the paged cursor (artifact hash across units) before it can be admitted.
2. Same object under a different authored name for fonts and page-level controlled images, a different object under a used font/form name, `Resume::Walk` direct colour spaces/patterns, annotation `Popup`/`IRT`/`OC`, `OutputIntents` and `EmbeddedFiles` remain barriers.
3. A direct `/AcroForm` inside a catalog whose `/DR` holds direct fonts and a field that is itself a direct dictionary stay barriers (`AcroForm/Fields`).
4. `run-s10-final.sh`'s first command (`-p ...-io-export -p ...-page-editor-tests`) did not build: foreign compile error in `semio-s-artifact-stdio-binary-mutations` (`BinaryMutation: ArtifactPersistedOperation`), not in this slice.

#### Slice 10 files

- Lift: `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs` (+ its `🧪️tests/🔬️unit/🦀️.rs`), `🪟️resources-and-fonts/🦀️.rs`, `📚️catalog-and-page-mode/🦀️.rs`, `🔨️modules/🖼️images/🎞️controlled/🦀️.rs`.
- Oracles and tests: `🔮️oracles/🦀️.rs` (`reference_reachable_xobjects`, `reference_page_resource_names`, `reference_form_fields`), `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, fixtures and `real-world/🔣️.json`.
- Ticket inputs: the three `*-fixture.py`, `form-images-oracle.py`, `run-s10-final.sh`, `run-s10-oracle.sh`.

### Slice 11: the institution diagram and the thesis, attributed per host turn (2026-11-26)

Native, debug, every cargo run through the gate (`run-route.sh`, `run-s9.sh`, `run-s4.sh`). Logs `🗑️generated/s11-*.log`. Every temporary `[DEBUG]` trace (importer per-turn census in `🔁️controlled`, host-turn counter in `artifact-app-laws`, stage and fault traces in `natural-file`) is removed again.

#### 1. What the per-turn census showed (it contradicts slice 9's attribution)

Instrumented per advance: the grant is **1,048,463 B copy / 63 items / 4,096 fuel per turn** (not 512 KiB), and an interpreted-stream pass at that grant is cheap.

| institution diagram, registered route | decoder turns | what |
| --- | --- | --- |
| structure + project | 170 | retained graph |
| interpret (1,345 forms) | 85 | copy-bound at 0.85 to 1.0 MiB per turn, 16 turns of it the 9 KB `Decoding` box per stream |
| lift | about 140 | content, role mirror, rename, navigation |
| total | 397 decoder turns = **host turn 190** | two decoder steps per host turn |

So the import was not stuck in the interpret pass or the lift for 1,024 turns: it **failed at host turn 184** and the other 840 turns were the failed job draining.

#### 2. Root cause 1: a spent turn grant was read as a permanent refusal (fixed)

`lift:navigation` needs a 65,536 B release after `rename` had released 983 KB in the same turn. The decoder yielded once (available 65,326), the next turn released the rest of the 1 MiB grant, and two further calls in the same host turn saw `available = 0`. `blocked.replace((event, available)) == Some(...)` fired on the second `0`, the job failed ("PDF decoder needs a 65536-byte backing grant"), the fault was never delivered inside the bound and the route sat in its close path until the turn limit.

Fix (`🚪️io/📥️import/🔁️controlled/🦀️.rs`): the decoder keeps the largest allocation and release backing any of its calls was offered (`peak`); `refused_for_good` refuses only a demand that failed twice against that largest backing. A turn whose grant earlier calls already spent only yields. Law `a_demand_blocked_by_a_spent_turn_grant_yields_and_only_a_full_grant_refuses`.

After the fix the institution diagram prepares and lifts to completion at host turn about 190 of 1,024 and enters Pack export.

#### 3. Root cause 2 (binding now): Pack export, not prepare or lift

After the decode the institution diagram spends the remaining 830 host turns in `natural-file-import-encode` (3,161 encode steps, never completes): the canonical deflate of the typed snapshot, about 40 B of copy per raw byte (slice 7 §2). The route's copy bound for the open is 32 MiB + 24 B per file byte = 47.6 MB; the encode alone is far above it. Not changed: it needs the canonical stored-block rule for already-encoded streams in the codec1 format first (schema-first), as slice 7 states.

#### 4. Thesis (6.3 MB, 65 pages)

Attributed with the same census: **the decode finishes at host turn 285 and fails at its output ceiling, not on turns**: `natural-file decoder retained 55,096,972 output bytes; maximum is 33,554,432`. Retained bytes (live delta by stage): project 16.2 MB (2.6x the file: object streams, whole 4 KiB pages and frames), Type 3 glyph programs 12.2 MB (reference lifter, one whole page per small program), page contents 13.3 MB, operator role mirror 6.8 MB, annotations 1.1, page tree 1.1, the rest 4. The failure was then invisible for 3,800 host turns and 160,000 close turns (`registered fixture did not reach its exact terminal-empty witness`) because the lifetime copy stock of the scope (528 MB) was spent. Copy is also above the route's 184 MB bound before the encode starts (decode alone 233 MB).

Reducing 55 MB to 32 MiB needs the operator role mirror gone (about 13 MB of it across contents and glyph programs: slice 7 §Not done 3, a cross-crate change of reference lifter, controlled lifter, graph projection, native Save and every twin) and a page-granular retention fix for small programs; not started.

#### 5. Laws

- `real_world_documents_prepare_within_half_of_their_route_turn_bound` (retained-graph tests, mut-family-11): under the importer's accounting (copy 1,048,463, items 63, fuel 4,096 per turn; bulk runs for body bytes and slot writes; allocation charged as copy unless it only reserves empty backing) every ledger document prepares to exactly the file single units reach, in at most half of its route bound (512 + 512 per MiB, halved), and the interpreted-stream pass is at most a third of the turns. Model vs route: institution 248 vs 256 decoder turns measured, thesis 267 vs 337.
- `real_world_documents_lift_within_half_of_their_route_turn_bound` (testkit): same for the lift, equal to the single-unit snapshot.
- `a_demand_blocked_by_a_spent_turn_grant_yields_and_only_a_full_grant_refuses` (importer).
- `PdfFilePreparationCursor::trace` and `InterpretedStreamCursor::trace` now name the interpret sub-step, so census and failures carry it.

#### 6. Not done (in order of yield)

1. Pack export copy model and the canonical stored-block rule (binds the institution diagram, site plan and thesis).
2. Operator role mirror, page-granular small programs, object-stream retention (thesis output ceiling).
3. Per-stream interpret cost (decoder reuse across streams) was not changed: after the stall fix the interpret pass is 85 of 397 decoder turns and is not binding.

#### 7. Files

- `🚪️io/📥️import/🔁️controlled/🦀️.rs` (`peak`, `refused_for_good`, law); `🚪️io/📥️import/📖️interpreted-streams/🦀️.rs` (`trace`); `🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs` (`trace`, `prepare_turns`, law); `🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🧪️tests/🔬️unit/🦀️.rs` (`controlled_lift_turns`, law).
- Ticket logs: `s11-institution-route.log`, `s11-thesis-3.log` (before/after attribution).

### Slice 12: bounded canonical compressor and a decoded-volume copy bound (2026-11-26)

Native, debug, every cargo run through `native-cargo.sh` (`run-s4.sh`, new `run-s12.sh`, `retry-route.sh`); logs `🗑️generated/s12-*.log`. The temporary `[DEBUG]` phase census and raw-body dump of the Pack cursor are removed again.

#### 1. Decision

- **(a) does not apply to the open.** The 133 MB is the Pack export of the typed snapshot of the open, not the Save: the retained graph keeps Flate streams decoded (schema rule of `PdfStreamFilter`), so the Pack never sees an encoded stream to copy through. (A real defect of Save was found instead, see 5.)
- **(b) is the fix.** Measured on the site plan's Pack body (one frame, 857,633 B raw, 106,462 B stored): 24.5 M compare steps (54 MB), 19.7 MB of hash inserts and 28 MB of per-step bookkeeping, i.e. about 123 B per raw byte. The canonical compressor probed up to 128 chain candidates per position and compared each to full length, so work per byte was bounded only by 128 × 258.
- **Canonical policy** (zlib-style, same fixed-Huffman stream format, any inflater reads it): at most `MAX_CHAIN` = 16 probes per position, no probe after a match of `NICE_MATCH` = 32, and a match longer than `MAX_INSERT` = 16 inserts only its first position into the chains. Chosen on the dumped body and the committed packs: stored 112,061 B against 106,462 B (+5.3%, the 5% budget), committed pack bodies +3.8%; (16, 32, 16) against (8, 32, 8) at +8.2%.
- Twins byte-identical: Rust ordinary encoder, `DeflateMeasure`, retained cursor (`consume_end` separates consumed from inserted positions), TypeScript and Python (`record-document`); TS = Python on four corpora, Rust = TS on three new vectors.

#### 2. Measured effect (site plan, registered route)

| | before | after |
| --- | --- | --- |
| Pack deflate work (quoted bytes + 1 per step) | 105.8 MB | 18.2 MB (sim 18.18, route 18.6) |
| whole open copy | 133,128,508 B | **47,924,718 B** (2.8x), 201 + 143 turns (was 201 + 240) |

#### 3. Laws and oracles

- `deflate_retained_work_per_input_byte_is_bounded_by_the_chain_and_nice_limits` (`🗜️deflate/🛫️encode/⏳️cursor`): quoted copy bytes plus one per step stay under `MAX_CHAIN × (3 × NICE_MATCH + 6) + 40` per input byte on zeros, periodic, random and record-like inputs, and under 5 × `MAX_CHAIN` on those inputs; stream equals the ordinary producer and inflates through `miniz_oxide`.
- Language-agnostic vectors (`nice-match-stops-probing`, `long-match-inserts-only-its-first-position`, `chain-limit-bounds-the-probes`; each differs from the old policy) in the retained fixture, read by Rust, the TS and the Python tests; zlib inflates them.
- Third-party: system zlib inflate equality of every recoded pack body and every twin output; `miniz_oxide` in Rust.
- Results: deflate crate 34/34 (`s12-deflate3`), Pack cursor 18/18 (`s12-pack`, before the final constants and again unchanged code path), replication lib 2/2, TS record-document 17/19, Python 16/18. The two failures in each language fail on the committed packs' envelope token (`s.stdio.semio…` in HEAD against `stdio.semio…` in the spec), unrelated to this slice.

#### 4. Regeneration (one sweep, real recoder)

`recode-packs.ts` (TS twin: open, rebuild with the new codec, check body equality, rewrite): 69 of the 109 committed `*.pack.semio` the twin can read were rewritten (second dry run: 0 changed); `rows.ts` the record-document `deflate` row; `hexfix.ts` the 3 durable-group fixture packs; `b64fix.ts` the wfc bitmap genesis pack. The 40 other pack files are empty or another container. Not run: durable-group and wfc bitmap tests (their crates are not reachable through the gate), the truncated-pack fixture and other hex goldens not found by a search for the pack magic.

#### 5. Copy bound re-derived (never just raised)

The old bound `32 MiB + 24 × file` assumed work proportional to the encoded file; a Flate file of 30,709 B holds 141,403 decoded stream bytes (D, independent zlib count; lopdf `reference_stream_bodies` in the test) and a typed snapshot of about 5 raw Pack bytes per decoded byte, so cost is proportional to D. New bound: `4 MiB + 24 × file + (112 + 96 + 192) × D`, the three stage ceilings (lift runs, run-based preparation, Pack export with the deflate law above) each 1.25x their site plan measurement (84, 74, 152 B per decoded byte). Site plan: limit 61.4 MB, measured 47.9 MB. Lift and prepare have measurements, not laws of their own yet.

#### 6. Open: Save of real-world files is a full rewrite

With the open inside its bound the site plan test now fails one step later: `the saved file must keep every byte of its base` (Save does not append to the original; custody lookup for this graph falls back to a rewrite, which re-encodes streams). That is the (a) defect; it was hidden behind the copy bound. Not diagnosed in this slice. Thesis and institution routes not re-run.
