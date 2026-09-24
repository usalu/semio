# WP-T9: Spec-Conformant DWG Writer, DWG Reader Across Versions, stdio temp/ Fixtures

Slice: T9 (session 10). Captures: `.tmp-ticket/wp-t9/generated/`. Kept inputs: `wp-t9/run-batch.sh`, `wp-t9/crates-*.txt`.
Private target: `wp-t9/target`. All results below come from runs I made (native).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. DWG writer (spec-conformant AC1024; acadrust and LibreDWG open it) | landed | stdio-dwg 70/70 + 1 ignored (`t-5.txt`); LibreDWG `dwgread`/`dwg2dxf` exit 0 with every entity (`libredwg-every-entity-kind*.txt`) |
| 2. DWG reader (AC1018, AC1021, AC1024, AC1027, AC1032; T7's two ignored tests on) | landed | `this_subset_reads_what_acadrust_writes` passes for all five versions (`t-5.txt`) |
| 3. Owner DWG round trips (stdio, raster, cad, draw, lowpoly) + other DWG dependents | landed | `batch-1.txt` (stdio-semio 2657, raster 228, cad 433, draw 282, lowpoly 299), `batch-3.txt` |
| 4. stdio tests reading `temp/` → committed fixtures | landed | `batch-2.txt` (deflate 50, zip 55, mp4 46); no stdio test reads `temp/` any more |

## 1. Writer

**Target: AC1024 (AutoCAD 2010).** Reasons:
- It is this subset's own standard (`s.stdio.dwg@ac1024`), and the only one whose logical object model the
  codec already carried typed, frame for frame (tables, dictionaries, layouts, block records, LINE/ARC/LWPOLYLINE),
  each encoder already proven byte-exact against the committed AutoCAD drawing.
- AC1024 objects use the R2010 layout that AC1027/AC1032 keep (only one extra common bit), inside the
  R2004 page container that AC1018 also uses, so the writer and the reader share one container and one
  object layout family. AC1021 (R2007) would need a Reed–Solomon/R21-LZ writer for no reader gain.
- Every current CAD tool reads AC1024 (AutoCAD 2010+, ODA, LibreDWG, acadrust, BricsCAD, …).

**Schema-first design.** `dwg_to_bytes(drawing)` is now `encode_r2004_snapshot(DwgSnapshot::from_drawing(drawing))`:
- `DwgSnapshot::from_drawing` / `DwgLogicalDrawing::from_native` (schema, `🧬️schema/📸️snapshot/🦀️.rs`) build a
  complete new AC1024 document as typed values: the nine table controls with their standard records
  (layers incl. mandatory `0`, ByBlock/ByLayer/Continuous, Standard text + dimension style, ACAD appid,
  `*Active` VPORT), `*Model_Space`/`*Paper_Space` block records with BLOCK/ENDBLK and their LAYOUTs (+ xdics),
  the named-object dictionary (ACAD_COLOR/GROUP/LAYOUT/MATERIAL/MLINESTYLE/PLOTSETTINGS/PLOTSTYLENAME
  (with-default + Normal placeholder)/VISUALSTYLE), MLINESTYLE Standard, three classes, AutoCAD-2010 header
  defaults whose relations point into that graph, deterministic GUIDs (SHA-256 of the drawing) and epoch
  dates (byte-stable exports). Handle plan: `new_document_handles`.
- The drawing model's entity kinds got typed bodies (replacing the lossy `Geometry` carrier):
  `Point, Circle, Ellipse, Text, Spline, Face3d, Polyline3d, PolyfaceMesh, Vertex, PolyfaceFace, SequenceEnd`
  (+ Rust DSL spec). POLYLINE_3D / POLYLINE_PFACE are written as the real owner + VERTEX_3D / VERTEX_PFACE /
  VERTEX_PFACE_FACE + SEQEND object family; `entities()` re-assembles them by handle.
- New R2010 frame writers for those bodies (`encode_r2010_entity_body_frame`, ODA §20.4 field order).
- Generalised the fixture-bound pieces the writer needs: the R2004 container (page numbering, section info/
  map allocations, file-header ids computed instead of asserting the fixture's addresses), the header section
  (sizes/bit sizes computed; decoder no longer requires the fixture's strings), the preview (0×0 bitmap),
  and the AppInfoHistory decoder (parses property sets / product info instead of matching the fixture's text).
  The committed AutoCAD drawing still re-encodes byte for byte (ac1018/ac1024 example tests green).
- The private "AC1015" container (writer, reader, `dwg_decode_semio_entity*`, `read_t`/`write_t`) is deleted.

**Oracle results** (`🚪️io/🧪️tests/🔮️acadrust-oracle/🦀️.rs`, both formerly ignored laws now plain tests):
- `acadrust_reads_what_this_subset_writes`: all 11 kinds on 2 layers, ByLayer/ByBlock/ACI colours — acadrust's
  entity list equals ours (kind, layer, colour, coordinates to 1e-6).
- `a_written_document_reads_back_entity_for_entity_and_re_encodes_byte_for_byte`: our drawing reader returns the
  same entities; the lossless decoder returns exactly `from_drawing`'s snapshot; `encode_dwg` of it reproduces
  the file byte for byte.
- LibreDWG 0.13.3 (Homebrew, already installed, manual): `dwgread` and `dwg2dxf` exit 0 on the written file; the
  JSON dump lists every entity with our layers, colours and coordinates (`libredwg-every-entity-kind-entities.txt`).
  Its remaining diagnostics (`Invalid sections: 0 != numgaps+numsections`, `Object handle not found <HANDSEED>`,
  the non-fatal `SUCCESS 0x40 VALUEOUTOFBOUNDS`) are identical in kind for AutoCAD's own committed drawing
  (`libredwg-fixture-diagnostics.txt`).

## 2. Reader

`dwg_from_bytes` is one version-aware reader (`DwgObjectLayout` R2004/R2007/R2010/R2013):
- containers: R2004 pages (AC1018/1024/1027/1032) and a new R2007 container (`🔖️R2007Container`: Reed–Solomon
  de-interleave, R21 LZ77 decompressor, compressed file-header metadata, page map, section map);
- frames: BS type + RL data size (R2004/R2007) vs UMC handle-stream size + BOT type (R2010+); inline code-page
  strings (R2004) vs string stream (R2007+); `has_ds_data` bit (R2013+); material/shadow (R2007+) and visual-style
  (R2010+) common fields; preview graphic RL vs BLL;
- one shared entity decoder (`decode_entity_body`) for every modelled kind, used by both the drawing projection
  and the lossless AC1024 decoder (their duplicated LINE/ARC/LWPOLYLINE readers are gone); layer colours now come
  from the CMC (previously read as index 0 on AutoCAD files);
- `decode_dwg` (the artifact document): AC1024 goes through the lossless typed decoder (AutoCAD's drawing and this
  writer's output round-trip byte for byte); every other version is read by `dwg_from_bytes` and re-expressed as a new
  AC1024 document (`DwgSnapshot::from_drawing`), by version, not by failure.

## 3. Owners

All owners of the shared codec, `cargo test -p <crate> --lib`, private target:

| Crate | Result | Capture |
|---|---|---|
| stdio-dwg | 70 passed, 1 ignored (the sanctioned `zzz_write_demo_fixtures` writer) | `t-5.txt` |
| stdio-semio (`conversion-drawing,conversion-mesh,conversion-cad`) | 2657 passed, 1 ignored | `batch-semio-s-artifact-stdio-semio.txt` |
| raster / cad / draw / lowpoly | 228 / 433 (+1 ignored) / 282 / 299 | `batch-*.txt` |
| gismap / gisterrain / layout / note / puzzle-2d | 158 / 55 / 395 / 397 / 607 | `batch-3.txt` |
| generation3d / process3d / shooting | 158 / 360 (+3 ignored) / 357 | `batch-3.txt` |
| stdio-dwg `cargo check --target wasm32-wasip2` | clean | (run after the last edit) |

Callers that hand-built `DwgSnapshot { version: "AC1015", drawing: from_native(..) }` (stdio semio drawing/cad/mesh dwg
serializers and their tests, lowpoly dwg serializer) now use `DwgSnapshot::from_drawing`; their tests assert `AC1024`.
W1 request filed: `.tmp-ticket/wp-w1/requests/t9.txt` (every guest linking stdio-dwg).

## 4. stdio `temp/` fixtures

| Test | Was | Now |
|---|---|---|
| mp4 codec / snapshot / mutations / diff exact-fixture laws (4) | `temp/bauen-mit-bestand.mp4` (untracked) | the owner's committed `🎥️mp4/…/✳️any/🧫️fixtures/🎬️.mp4` via `include_bytes!` |
| deflate `illustrator_partial_flush_materialization_matches_fixture_stream` | searched `temp/📄️bachelor-thesis.pdf` | `🗜️deflate/…/🧫️fixtures/🎨️illustrator-partial-flush.zz`: the 3362-byte Illustrator stream cut by hand out of the committed `📖️pdf/…/🎓️bachelor-thesis.pdf` |
| deflate `exact_pptx_bin_policy` | PowerPoint's OLE members of the untracked `temp/domai-…pptx` | the source bytes are gone (not tracked anywhere; no committed PowerPoint archive holds an OLE member, and the committed pptx fixtures' members are not PowerPoint-compressed — probe `deflate-probe.txt`). Replaced by `compact_high_search_embedded_binary_is_standard_deflate`: a handcrafted CFB image, deterministic output, and a third-party inflater (`miniz_oxide` 0.8.9, test-only dev-dependency, `🔒️dependencies.json` updated) must return the input. The PowerPoint byte-equality oracle is lost with the file — see open items. |
| zip `…full metadata…` (OPC part) | `temp/domai-…pptx` (211 entries) | handcrafted `🎒️zip/…/🧱️base/🧫️fixtures/📦️opc.zip` (6-part OPC package: content types, rels, core props, presentation, part rels, a stored PNG) |

## Timeline notes

- Rule 13: an interim state (typed entity bodies landed before their frame writers) left stdio-dwg non-compiling for a
  few minutes; the coordinator flagged it, compile was restored immediately, every later step landed compiling.
- All source edits landed before the coordinator's stdio/gis freeze; nothing was edited after it.

## Open items

- The PowerPoint OLE compression policy (`deflate_raw_deterministic_compact_high_search`) has no third-party byte-equality
  oracle any more; a PowerPoint-authored archive with embedded OLE objects would have to be committed to restore it.
- `decode_dwg` is lossless only for AC1024 documents inside the typed object model (AutoCAD's fixture, this writer's
  output); its header/object decoders still pin several AutoCAD-2009 producer constants, so foreign AC1024 files (e.g.
  acadrust's) are read by `dwg_from_bytes` (geometry) but not losslessly by `decode_dwg`. Other versions are re-expressed
  as new AC1024 documents; `encode_dwg` refuses non-AC1024 snapshots.
- LibreDWG reports the same non-fatal `VALUEOUTOFBOUNDS` / section-count / HANDSEED warnings for our files as for AutoCAD's.

## Files changed

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `…/🧬️schema/📸️snapshot/🦀️.rs`
- `…/🚪️io/🧪️tests/{🔬️unit,🔮️acadrust-oracle}/🦀️.rs`
- `…/4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio` (regenerated with `zzz_write_demo_fixtures`; the entity-body record spec grew)
- `…/🧬️schema/{🟦️.ts,🔗️.graphql,🛰️.proto,📸️snapshot/{🔗️.graphql,🛰️.proto},🔺️diff/{🔗️.graphql,🛰️.proto},🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json}` (typed entity bodies)
- deflate: `🚪️io/🧪️tests/🔬️codec/🦀️.rs`, `🧫️fixtures/🎨️illustrator-partial-flush.zz` (new), `📦️packages/🦀️rust/Cargo.toml`; `🔒️dependencies.json`
- zip: `🚪️io/🧪️tests/🔬️codec/🦀️.rs`, `🧫️fixtures/📦️opc.zip` (new)
- mp4: `🚪️io/🧪️tests/🔬️codec/🦀️.rs`, `🧬️schema/{📸️snapshot,🧬️mutations,🔺️diff}/🧪️tests/🔬️unit/🦀️.rs`
- stdio semio dwg leaves (drawing/cad/mesh serializers + tests) and lowpoly dwg serializer: `DwgSnapshot::from_drawing`
  instead of hand-built "AC1015" snapshots.
