# r12 exec - w2-wp14-sheets (WP-14 Sheets and printing)

Agent `w2-wp14-sheets`, wave W2. Ruling r9-decisions section 3: output only through the existing stdio dialects (`s.stdio.svg@1.1/*`, `s.stdio.pdf@1.7/*`), no new PDF writer.
Paths: `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `T` = this ticket folder.

## Status in one paragraph

Everything below is WRITTEN. NOTHING OF IT HAS BEEN COMPILED OR RUN IN RUST: `semio-framework-plugin` (lib) does not compile in this tree (282 errors at the first attempt, 100 at the last one, all in
`🧰️framework/.../🔌️plugin/.../🦀️.rs`, the store-retirement refactor of other agents), so `cargo check/test -p semio-s-artifact-bim-model` cannot reach my crate. What WAS run: the schema/leaf/mutation-facet/oracle/feature
generators (bun), the example generator, `r3-f1-check-names.ts` (none of my names flagged), and both python oracles of the inference case (`write` then `check`: "oracle agrees", shapely 2.1.2). Every Rust
claim is therefore "unverified"; the exact commands to verify are at the end.

## Footprint (before to after)

### Schema and mutations
- `T/r12-w2-wp14-model.ts` (new) spread into `T/r3-f1-gen-model.ts`: `Sheet`, `Viewport`, `SheetRevision` (+ `Paper`, `IsoSize`, `Orientation`), collections `sheets`, `viewports`, `sheet_revisions`; generated Rust/JSON/TS/GraphQL/proto facets rewritten ("wrote model: 65 structs, 46 enums, 47 collections").
- `S/🧬️schema/📸️snapshot/📄️sheets/🦀️.rs` (new, with unit tests): paper sizes, `Sheet::size`, `Viewport::standard`, `sheet_problem`/`viewport_problem`/`revision_problem`, `revisions_of`, `viewports_of`.
- Nine leaves (golden-leaf recipe: mutation/diff/inverse/schema/tests/fixtures) generated from `T/r12-w2-wp14-leaves.ts`, tags 14000..14008: `create-sheet`, `set-sheet`, `delete-sheet`, `create-viewport`, `set-viewport`, `delete-viewport`, `create-sheet-revision`, `set-sheet-revision`, `delete-sheet-revision`. Mounted by `T/r12-w2-wp14-mount.ts`; protocol tags and grammar rows by `r3-f1-gen-mutation-facets.ts` (151 leaves at the time).
- `🌊️cascade`: deleting a sheet removes its viewports and revision rows, deleting a view removes the viewports that show it, all with concrete inverses (`removal!` rows + `closure`); extra case `delete-view/cascades-its-viewports`. Cross-kind id uniqueness: the three collections are in `exists` and the `taken` noun list.
- `r3-f1-gen-oracle.ts` (registered 173 kinds, 1327 scenarios) and `r3-f1-gen-feature.ts` (173 kinds) re-run last. The after/diff fixtures of the nine leaves are placeholders until blessed (see Verify).

### Inference
- `S/🧬️schema/💡️inferences/📄️sheet-layout` (new): `PaperRect`, `ViewMap`, `PlacedViewport`, `TitleBlock`, `RevisionTable`, `SheetIssue` (en+de messages), `SheetLayout`, `layout_of`, honest `dependency()` (sheet, its viewports, its revisions, kind+name of the views it shows), `READS`, `metrics_json`; submodules `windows` (frame 20/10 mm, window = crop or drawing bounds + 5 mm, minimum 10 mm) and `title-block` (180 x 48 mm, revision rows 6 mm). Derived only, nothing stored.
- Model graph: `NodeKind::Sheet` (requires `View`), `ModelNode::Sheet`, `Data::Sheet`, plan/compute/projection/dirty arms. Tests `mod sheets` in `🕸️model-graph/🧪️tests/📈️incremental`: cache transparency (cold = warm = uncached), a project rename is gated and computes no sheet, a sheet diff recomputes only its sheet (no view/storey), a viewport diff recomputes the holding sheet, a model edit that redraws a view recomputes the sheet, deleting the last sheet removes node and entry.
- Diagnostics: `RefViewportSheet`, `RefViewportView`, `RefRevisionSheet` (error, en+de rows, `ReferenceView` extended, READS extended) plus a test in both languages.
- Text projections for oracles: `encode_inference_projection_json` slugs `sheets`, `sheet-svg`, `sheet-pdf`.

### IO (stdio dialects only)
- `S/🚪️io/📤️export/📄️sheets` (new): `ink` (marks drawn by both writers), `svg` (one SVG 1.1 file per sheet in paper mm: viewBox = paper, one `clipPath` window per viewport, the scaled linework of the view inside), `pdf` (`SheetsPdf::{begin,page,finish}` over the stdio PDF `DocumentStream`, page by page; arcs as cubic Beziers), `tables` (oracle tables), `ModelIntoSheetsPdf` serializer registered in `io()` (hop list test updated). `TitleLabels::english()` for the serializer.
- Refactor others may notice: `🎨️svg/🖍️drawing` got `view_layers` factored out of `view_group`, `lines_of` and `parse_crop` are now `pub`.
- Justified exclusions: IFC (sheets are not building elements; IFC 2x3/4 has no portable sheet entity in this subset), glTF (no 3D geometry), CSV (no tabular element rows). The SVG set is one file per sheet (no archive); the PDF set is the multi-page document.

### Editor
- Sheet window `bim-edit-sheet` (`🪟️windows/📄️sheet`, config, schema facets, accessible surface, en+de): paper canvas with frame, viewports (linework scaled by an affine composition), title block, revision table, selection handles; `set-view sheet`; not in the default layout.
- Gestures `🧵️gestures/🖼️viewports`: `Arrange` (select, move, drag a scale handle that snaps to the 1:1..1:1000 series, constrained by `nearest_scale`) and `Place` (place the picked view, utility `viewport`, hotkey shift+v, command `armViewport`); `Surface::Sheet` in session/overlay.
- Entities (`🧩️entities/📄️sheets`): fields, inferred rows (derived sheet layout facts), create defaults (`next_number`, `next_view`, `next_mark`), choices; properties/create/delete/rename come from the generic entity rows. Outliner: a Sheets group, viewports and revision rows nested under their sheet.
- Command `exportSheets` (`🎮️commands/📄️export-sheets`): `SheetsJob` (analysis steps, then one sheet or PDF page per step), `SheetsExportWork` (retained command: `Progress` per step with a bilingual preview, `is_cancelled` honoured, `Effect::DownloadMediaExport`); pattern of `T/r11-audit-w03-session.md`. Chrome measures: sheet select + export toggles. Labels en+de (about 60 rows in `🗣️terminology`).

### Examples
- `T/r12-w2-wp14-sheets-examples.ts` (new), hooked into `T/r4-x-examples-gen.ts`: the house gets `A-101` plans (A2), `A-201` sections (A3), `A-301` elevations (A2), 10 viewports, 2 revision rows; the office the same on A0/A1/A1 (11 viewports). Every viewport is cropped to the extent of its drawing at 1:100 and shelf-packed clear of the title block; the generator refuses a layout that would carry findings.
- The generator RAN: `🖼️assets/🏡️house|🏢️office/📸️snapshot.json` rewritten.
- `🧰️checks`: `dangling` checks the three collections with the same problem functions the leaves use; `replay` rows `create-sheet`, `create-viewport`, `create-sheet-revision`. House and office example tests added (counts, papers, no findings, every viewport cropped and drawn).

### Features and oracles (language agnostic + third party)
| Case | Third-party library | Table compared |
|---|---|---|
| `🧪️tests/📄️infer-bim-1-sheets` (`@id-sheets-room`) | shapely 2.1.2 (boxes, covers, intersection, metamorphic laws) | paper, frame, title block + texts, revision table, windows, findings; fixture `🧫️fixtures/💡️inferences/📄️sheet-layout/🏠️room` (snapshot written by `T/r12-w2-wp14-fixtures.ts`, expectation written by the oracle) |
| `🧪️tests/🖨️export-bim-1-sheets-svg` | lxml + shapely | root size = viewBox in mm, clip window per viewport = crop at scale, drawing lands on window, title/revision text runs |
| `🧪️tests/📖️export-bim-1-sheets-pdf` | pypdf 6.14.2 (declared in `✏️s/🔌️plugins/🏙️bim/🔮️oracles/🔣️.json` host packages, new oracle id `bim-1-pypdf-pdf`) | page count, media boxes in mm, extracted text shows number and title |

## Commands run and results

| Command | Result |
|---|---|
| `bun r3-f1-gen-model.ts` | wrote model: 65 structs, 46 enums, 47 collections |
| leaf, mount, mutation-facet generators | ran; 9 leaves mounted; "mutation facets: 151 leaves" |
| `bun r3-f1-gen-oracle.ts` / `bun r3-f1-gen-feature.ts` | registered 173 kinds, 1327 scenarios / written for 173 kinds |
| `bun r12-w2-wp14-fixtures.ts <room in> <out>` | wrote the sheet room snapshot |
| `PYTHONUTF8=1 .venv/Scripts/python.exe .../📄️infer-bim-1-sheets/🐍️.py write` then `check <fixtures>/📄️sheet-layout` | shapely 2.1.2, 3 sheets, 7 viewports; "oracle agrees" (audit incl. metamorphic laws passed) |
| `bun r4-x-examples-gen.ts` | house sheets 3 / viewports 10 / revisions 2; office 3 / 11 / 2 |
| `bun r3-f1-check-names.ts` | 17 pre-existing duplicate-emoji problems of other packages, none of mine |
| `🚦️gate.sh w2-wp14-sheets -- cargo check ... -p semio-s-artifact-bim-model --lib` | FAILS before my crate: `semio-framework-plugin` 100 errors (E0053 `build_document_store_owners` ...) |
| `cargo test --lib` counts, wasm32-wasip2 lib check | NOT RUN (blocked) |

## Verify (to do as soon as `semio-framework-plugin` compiles)
1. `🚦️gate.sh w2-wp14-sheets -- cargo check --manifest-path ✏️s/.../🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib --message-format=short` and fix compile slips in the files above.
2. Bless: `BIM_BLESS=1 cargo test -p semio-s-artifact-bim-model --lib create_sheet` (all nine leaves + `delete_view`), `BIM_BLESS=1 ... the_committed_room_files_are_the_current_export` (writes `🧫️fixtures/🚪️sheets/🏠️room/{A-101,A-901,A-902}.svg` and `sheets.pdf`), then `python 🐍️.py write` of `🖨️export-bim-1-sheets-svg` and `📖️export-bim-1-sheets-pdf` against `🧫️fixtures/🚪️sheets`, and `BIM_BLESS=1 cargo test bless_the_` for the DSL texts of the house and the office (their `🗣️.dsl.semio` are stale: the new collections are not in them yet).
3. `cargo test --lib -p semio-s-artifact-bim-model` (record the exact counts here), then `cargo check --target wasm32-wasip2 --lib`.
4. Re-run `r3-f1-gen-oracle.ts`, `r3-f1-gen-feature.ts`, `r3-f1-check-names.ts`.

## Known gaps and notes for peers and r13
- Viewport crops are applied as a clip window only; the editor draws no crop mask yet. The sheet-layout text facets (DSL grammar/ebnf/g4/proto of the snapshot text) are not regenerated for any package since the generator for them is not part of the F1 chain (annotation styles are missing there too).
- Tag range claimed: 14000..14008. Peers: `drawing.rs` (`view_layers`), `parse_crop` and `lines_of` visibility changed as noted above.
- A `ModelDiff::sheets/viewports/sheet_revisions` constructor is generated by `collections!`; tests rely on that.
