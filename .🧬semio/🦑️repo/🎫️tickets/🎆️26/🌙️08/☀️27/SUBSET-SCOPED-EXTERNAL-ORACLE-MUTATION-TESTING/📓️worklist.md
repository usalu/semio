# 📓️ Worklist — remaining subsets with a real carrier, verified by reading serializer bodies

Method: replicated `stubSerializerBreaches` (the repo's own gate) directly against every `.rs` file
under a `🧵️serializers` directory (333 files), confirmed the replica against the gate's own published
numbers (130 stub serializers — exact match), then **read every candidate REAL file's actual body**
rather than trusting the mechanical result, because the mechanical result was wrong in both
directions:

* It **missed** a fourth stub shape the playbook's three don't name: `serde_json::to_value(from)` →
  `serde_json::from_value(..)` structural coercion into the target type — the same type-confusion
  idea as the pack-transmute shape, done through serde instead. It also missed `encode_pack`/
  `decode_pack` type-puns under variable names other than the literal `snapshot`/`bytes` the gate's
  regex requires, an explicit `Err("… not implemented")` with no `"yet"`, and an "xml bridge" that
  returns `XmlDocument::default()` regardless of input. Closing these dropped `shooting`, `dag`,
  `wires`, `flow`, `present`, `vcs`, `curate`, `s-home`, and three of `forms`'/`bcf`'s/`docx`'s/
  `xlsx`'s/`pptx`'s five declared formats entirely off the reachable list.
* It **falsely flagged** two genuinely real files (`semio/v1/cad → step`, `semio/v1/drawing → svg`)
  as stubs, because a naive `print_dsl`/`decode_pack` text search also matches the file's own
  `#[cfg(test)]` round-trip-verification code, which calls those same functions to *prove* the
  serializer works. Fixed by stripping `#[cfg(test)] mod … { }` blocks before matching.

Every REAL/STUB verdict below is backed by a quoted `file:line`, not a directory listing. Scripts
used for this pass live in the scratchpad
(`/private/tmp/claude-501/…/503877ec…/scratchpad/classify3.py` and `aggregate.py`; not part of the
repo, not committed, read-only research artifacts).

**Excluded per instruction** (already done or in progress by other agents — not touched, not
re-verified beyond confirming they still show up as real carriers): `stdio/🧿️semio/✳️mesh` (17),
`stdio/🧿️semio/✳️brep` (13), `stdio/📐️step/✳️cc6` (5), `🏗️fem/◻2d` (25), `🏗️fem/🧊️3d` (25),
`🏛️architect/🏛️program` (266), `🗒️note` (33), `📏️layout` (25).

**Already fully discharged — not "remaining," excluded from the ranking below**: `png@1.2/any`
(15/15 mutations manifested against the `png` crate), `jpg@jfif-1.01/any` (10/10, `image` crate),
`tiff@6.0/any` (6/6, `image` crate), `bmp@v3/any` (5/5, `image` crate), `pdf@1.4/any` (5/5, `lopdf`),
`gltf@2.0/any` (120/120, `json` crate structural validation) — all confirmed via their own
`🧪️oracle/🔣️.json` `mutationManifests` count equal to their mutation count. Registering anything
against these again would be duplicate work.

**Approved-oracle roster used below** — every recommendation in this document is an
**already-approved `test-oracle`** entry in `🔒️dependencies.json` (32 entries checked). No new
library is proposed anywhere in this worklist: `png 0.17.16`, `image 0.25`, `gif 0.13`, `las 0.11`,
`lopdf 0.44`, `zip 6`, `quick-xml 0.42`, `calamine 0.36`, `csv 1`, `stl_io 0.8`, `ruststep 0.4`,
`flate2 1`, `dxf 0.6`, `hound 3`, `riff 2.0`, `mp4 0.14`, `comrak 0.54`, `json 0.12`,
`ifcopenshell 0.8.4.post1` (python), `pypdf 6.14.2` (python).

## Ranked table

| Rank | Subset | Mutations | Real carrier(s) | Oracle status | Witnessable (est.) | Call |
| ---: | --- | ---: | --- | --- | --- | --- |
| 1 | `gif@89a/any` | 21 | gif | **chosen**: `gif-89a-any-mutate` (rust `gif`) | most — pixel/frame/timing fields | fixtures+manifest only |
| 2 | `semio@v1/document` | 18 | docx, md, pdf | none chosen | most — block/run text & structure | propose oracle |
| 3 | `semio@v1/drawing` | 17 | dwg, dxf, pdf, **svg** (corrected) | none chosen | most — path/text/layer geometry | propose oracle |
| 4 | `pdf@1.7/any` | 16 | pdf (binary+deflate) | **chosen**: `lopdf-pdf-1-7-mutate` | most | fixtures+manifest only |
| 5 | `semio@v1/cad` | 16 | dwg, dxf, **step** (corrected) | none chosen | most — 2D entity geometry | propose oracle |
| 6 | `mathematical@1/any` | 15 | csv | none chosen | node id/label/x/y only | propose oracle |
| 7 | `semio@v1/presentation` | 15 | pptx | none chosen | most — shape/text/position | propose oracle |
| 8 | `png@1.2/any` | 15 | binary+deflate (png) | **ALREADY DONE** 15/15 | — | excluded |
| 8 | `las@1.0/any` | 15 | las | **chosen**: `las-1-0-any-mutate` | most — point records | fixtures+manifest only |
| 10 | `bcf@2.1/any` | 14 | **zip only** (xml is a stub) | **chosen**: `zip-quick-xml-bcf-2-1-mutate` | most | fixtures+manifest only |
| 10 | `draw@1/any` | 14 | **svg only** (dwg/dxf/pdf/png all stub) | none chosen | node/path/style fields only | propose oracle, capped |
| 12 | `semio@v1/image` | 13 | bmp, gif, jpg, png, tiff | none chosen | most — pixel buffer & metadata | propose oracle |
| 12 | `semio@v1/animation` | 13 | gif, gltf, mp4 (mp4/gif = timing only) | none chosen | keyframe/timing mutations only | propose oracle, capped |
| 12 | `docx@ecma-376/any` | 13 | **zip only** (xml is a stub) | **chosen**: `zip-quick-xml-docx-ecma-376-mutate` | most | fixtures+manifest only |
| 15 | `gif@87a/any` | 12 | gif | **chosen**: `gif-87a-mutate` | most | fixtures+manifest only |
| 16 | `ifc@4/any` | 11 | ifc | **chosen** (2×): `ruststep`, `ifcopenshell` | most | fixtures+manifest only |
| 16 | `step@ap214/any` | 11 | step | **chosen**: `ruststep-step-ap214-any-mutate` | most | fixtures+manifest only |
| 16 | `semio@v1/model` | 11 | bcf (narrow), ifc (broad) | none chosen | most via ifc; only Bcf-topic-shaped via bcf | propose oracle |
| 19 | `forms@1/any` | 10 | **csv only** (xlsx/zip both `Err("not implemented")`) | none chosen | question id/label/kind/required only | propose oracle, capped |
| 19 | `semio@v1/audio` | 10 | **wav only** (mp3 errors on real content) | none chosen | most — PCM samples, rate, channels | propose oracle, capped |
| 19 | `xlsx@ecma-376/any` | 10 | **zip only** | **chosen**: `xlsx-ecma-376-mutate` (`calamine`) | most | fixtures+manifest only |
| 22 | `svg@1.1/any` | 9 | xml (svg) | **chosen**: `quick-xml-svg-1-1-mutate` | most | fixtures+manifest only |
| 22 | `semio@v1/video` | 9 | mp4, avi | none chosen | most — sample data, dims, timing | propose oracle |
| 22 | `semio@v1/value` | 9 | **xml, conditional** | none chosen | only when graph already XML-shaped | propose oracle, capped |
| 22 | `pptx@ecma-376/any` | 9 | **zip only** | **chosen**: `pptx-ecma-376-mutate` | most | fixtures+manifest only |
| 26 | `sequence@1/any` | 8 | csv | **chosen** (partial, 4/8): `csv-rfc4180-reader` | 4 of 8, already correctly split | already correct |
| 27 | `stl@ascii/any` | 7 | stl (binary) | **chosen**: `stl-io-ascii-mutate` | most | fixtures+manifest only |
| 27 | `zip@2.0/any` | 7 | zip (binary+deflate) | **chosen**: `zip-2-0-mutate` | most | fixtures+manifest only |
| — | `tiff/bmp/jpg/pdf@1.4` | 6+5+10+5 | — | **ALREADY DONE** | — | excluded |
| 31 | `ifc@2x3/any` | 5 | ifc (txt) | **chosen** (2×): `ruststep`, `ifcopenshell` | most | fixtures+manifest only |
| 31 | `txt@utf-8/any` | 5 | binary (UTF‑8 echo) | correctly declined (`noOracleDecision`) | — | no further work |
| 31 | `deflate@rfc1950/any` | 5 | binary (zlib) | **chosen**: `flate2-deflate-rfc1950-mutate` | most | fixtures+manifest only |
| 31 | `binary@raw/any` | 5 | binary (identity) | correctly declined (`raw-buffer-no-format`) | — | no further work |
| 35 | `writer@1/any` | 4 | docx, pdf | none chosen (only cross-semio) | 3–4 of 4, real, non‑JSON | **propose oracle — new finding** |
| 36 | `dwg@ac1024/any` | 3 | dwg (binary) | correctly declined (proprietary) | — | no further work |
| 36 | `dwg@ac1018/any` | 3 | dwg (binary) | correctly declined (proprietary) | — | no further work |
| 38 | `playground@1/any` | 1 | csv | correctly declined-ish (trivial) | 1, but zero computed content | not worth registering |

**Sum of mutations across genuinely un-worked remaining subsets (excluding the 8 "already
chosen/correct/declined" rows that need no new decision): ≈ 155** across 12 subsets
(`document` 18, `drawing` 17, `cad` 16, `mathematical` 15, `presentation` 15, `draw` 14 (capped ~5–7),
`image` 13, `animation` 13 (capped ~6–8), `model` 11, `forms` 10 (capped ~5), `audio` 10 (capped ~8),
`video` 9, `value` 9 (capped, conditional), `writer` 4). The rest (≈ 175 mutations across `gif`×2,
`pdf-1.7`, `las`, `bcf`, `docx`, `ifc`×2, `step`, `xlsx`, `svg`, `pptx`, `stl`, `zip`, `deflate`) already
have a chosen third-party oracle and are waiting on the mechanical fixtures/probes/manifest step the
`mesh`/`brep` pilots already prove out — genuinely lower-effort than anything requiring a new oracle
decision.

---

## Tier A — oracle already chosen, only fixtures/manifest work remains

These are NOT decisions to make; `🧪️oracle/🔣️.json` already names a `third-party-library` oracle and
the carrier is confirmed real. The remaining work per subset is exactly the five-artefact playbook
(`🧪️oracle`, `🔬️probes`, `🏭️generator`, `🧫️fixtures`) the `mesh`/`brep` pilots already walked.

* **`gif@89a/any` (21)** and **`gif@87a/any` (12)** — `serialize` calls
  `crate::artifacts::gif::standards::v89a::engine::encode_gif(from)` /
  `…v87a::engine::encode_gif(from)`
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/…/💾️binary/🔖️raw/✳️any/🦀️component.rs:11`
  and the `87a` twin) — a real GIF encoder, not a stub. Oracle: `gif` crate (already registered).
* **`pdf@1.7/any` (16)** — `encode_pdf(from)` at `…📄️pdf/🏅️standards/🔖️1.7/…/💾️binary/🔖️raw/✳️any/🦀️component.rs:12`. Oracle: `lopdf` (already registered).
* **`las@1.0/any` (15)** — `crate::artifacts::las::engine::encode_las(from)` at `…☁️las/…/💾️binary/🔖️raw/✳️any/🦀️component.rs:9`. Oracle: `las` crate (already registered).
* **`bcf@2.1/any` (14)** — `crate::artifacts::bcf::io::encode_bcf(from)` at `…💬️bcf/…/🎒️zip/🔖️2.0/✳️any/🦀️component.rs:13`; the **xml sibling is a stub** — `…📰xml/🔖️1.0/✳️any/🦀️component.rs:8` returns `XmlSnapshot { doc: XmlDocument::default() }` for **any** `_from`, param unused. Oracle: `zip`+`quick-xml` (already registered as `zip-quick-xml-bcf-2-1-mutate`).
* **`docx@ecma-376/any` (13)** and **`xlsx@ecma-376/any` (10)** and **`pptx@ecma-376/any` (9)** — same pattern: real `encode_docx`/`encode_xlsx`/`encode_pptx` under `🎒️zip`, and the same `XmlDocument::default()` stub under `📰xml` in all three (confirmed identical `bridge stub for` header comment in all 8 xml-bridge files, import and export, across bcf/docx/xlsx/pptx). Oracles already registered: `zip-quick-xml-docx-ecma-376-mutate`, `xlsx-ecma-376-mutate` (`calamine`), `pptx-ecma-376-mutate` (`zip`).
* **`ifc@4/any` (11)** and **`ifc@2x3/any` (5)** — `encode_ifc2x3(from)` at `…🏗️ifc/🏅️standards/🔖️2x3/…/📄txt/🔖️utf-8/✳️any/🦀️component.rs:11`, and the `4` sibling calls `write_part21(to_part21_document(from))` — genuine STEP Part-21 text carrying IFC entities. TWO oracles already registered per subset: `ruststep` (structural) and `ifcopenshell` (python, differential).
* **`step@ap214/any` (11)** — same `write_part21` pattern at `…📐️step/…/📄txt/🔖️utf-8/✳️any/🦀️component.rs:8`. This is the SAME artifact family as the already-piloted `cc6` subset but a DIFFERENT subset (`any`, not `cc6`) — not excluded by the instruction, not yet manifested (`manifest_mutation_count: 0`). Oracle already registered: `ruststep-step-ap214-any-mutate`.
* **`svg@1.1/any` (9)** — `Ok(XmlSnapshot { doc: from.doc.clone() })` — genuinely returns the artifact's own live document (not a default), at `…🎨️svg/…/📰xml/🔖️1.0/✳️any/🦀️component.rs:9`. Oracle already registered: `quick-xml-svg-1-1-mutate`.
* **`stl@ascii/any` (7)** — `encode_stl_ascii(from)` at `…🟪️stl/…/📄txt/🔖️utf-8/✳️any/🦀️component.rs:12`. Oracle already registered: `stl-io-ascii-mutate` (`stl_io`).
* **`zip@2.0/any` (7)** — `encode_zip(from)` at `…🎒️zip/…/💾️binary/🔖️raw/✳️any/🦀️component.rs:12`. Oracle already registered: `zip-2-0-mutate`.
* **`deflate@rfc1950/any` (5)** — `zlib_decompress(&from.payload)` at `…🗜️deflate/…/💾️binary/🔖️raw/✳️any/🦀️component.rs:14` — the artifact's own `payload` field is real RFC1950 zlib bytes (confirmed by the fact a real zlib decoder unwraps it). Oracle already registered: `flate2-deflate-rfc1950-mutate`.
* **`sequence@1/any` (8, 4 of 8 already correctly oracled)** — `serialize` builds one real `CsvRecord{id, kind, JSON(params)}` row per step (`…🎬️sequence/…/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs:20-23`); `create-step`/`delete-step`/`duplicate-step`/`edit-step-params` are witnessed, `move-step`/`change-step-collapsed`/`connect-steps`/`disconnect-steps` correctly carry a `noOracleDecision` since a flat CSV grid has no x/y or edge concept. Already the correct split — nothing to change.

**Difficulty/value**: this tier is the highest-value, lowest-risk remaining work — roughly 130+
mutations become genuinely oracled for the cost of fixtures+probes+manifest alone, no new library
research, no new registration decision.

---

## Tier B — real carrier confirmed, no oracle chosen yet (propose one)

### `semio@v1/document` (18) — docx, md, pdf all real
Confirmed by reading all three bodies in full (`…🧿️semio/…/✳️document/🚪️io/📤️export/🧵️serializers/…`):
docx builds a real `DocxDocument{body, styles}` via `map_semio_block` (heading/paragraph/list/
table/code/quote/image all mapped, only `PageBreak` and inline color/font/link dropped, documented);
md maps every `DocBlock` to a real `MdBlock` (`…📝️md/🔖️commonmark/✳️any/🦀️component.rs:51-62`); pdf
splits blocks into pages on `PageBreak` and flattens each page's text
(`…📄️pdf/🔖️1.7/✳️any/🦀️component.rs:177-191`). All three ship `#[cfg(test)]` round trips through the
REAL downstream codec (`encode_docx`/`decode_pdf`/etc.), not just a struct check.
**Witnessable**: heading/paragraph/list/table/code/quote/image text-content mutations via all three;
style-name mutations via docx only (md drops `style_id`); page-break mutations via pdf/docx only (md
has no page concept).
**Oracle**: `comrak` (md, approved) + `quick-xml`+`zip` (docx, approved) + `lopdf` (pdf, approved) — a
genuine three-carrier cross-check, all already-approved packages.
**Call**: high value, straightforward — the field mapping is already fully documented in the
serializer's own doc comments, which doubles as the witnessability spec.

### `semio@v1/drawing` (17) — dwg, dxf, pdf, **svg** (corrected from stub)
`svg` was misclassified STUB by the naive gate (its `#[cfg(test)]` block calls `print_dsl`/`parse_dsl`
to *prove* the round trip — that's not the artifact's own export path). Read in full: `svg` builds a
real `<svg>` document with `<path>`/`<text>`/`<g transform="matrix(...)">` per node, plus a genuine
base64 data-URI convention for embedded images (`…🎨️svg/🔖️1.1/✳️any/🦀️component.rs:159-175`); `dxf`
recognizes the exact closed-two-arc circle shape for an EXACT (non-flattened) round trip and
otherwise samples curves at 32 segments into a real `POLYLINE`
(`…🖊️dxf/🔖️r12/✳️any/🦀️component.rs:391-493`); `dwg` walks the node tree into a real
`DwgDrawing`/`paths_to_dwg_drawing` and round-trips through the codec in its own test
(`…🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs:695-717`); `pdf` collects all `Text` node values per layer
into one page (`…📄️pdf/🔖️1.7/✳️any/🦀️component.rs`).
**Witnessable**: path/geometry mutations (create/replace-path, drag/move-node, scale, rotate, group,
flatten/unflatten) via dxf/dwg (exact circle, sampled curves elsewhere); stroke/fill color mutations
NOT witnessed by dxf/dwg/pdf (no color field in any of the three — svg is the only carrier for those,
via `color_to_css`); text-content mutations via all four; layer create/delete via all four.
**Oracle**: `dxf` crate (approved) for dxf; `quick-xml` (approved) for svg; `lopdf` (approved) for pdf;
DWG has **no approved third-party reader** (matches the already-correct `dwg-ac1018/1024`
declines below) — treat dwg as write-only evidence, not an oracle leg, for this subset.
**Call**: high value. The 17-mutation list was independently confirmed (`flatten, unflatten,
create-layer, ungroup, rotate, reorder-nodes, replace-path, drag-nodes, move-node, group,
change-stroke-width, change-stroke-color, delete-node, scale, replace-fill, create-node,
delete-layer`) — `change-stroke-width`/`change-stroke-color`/`replace-fill` (3 of 17) are witnessable
ONLY via svg; the other 14 via dxf.

### `semio@v1/cad` (16) — dwg, dxf, **step** (corrected from stub)
`step` was the other false-stub (same `#[cfg(test)]` `print_dsl`/`parse_dsl` round-trip false
positive). Read in full: builds real `CARTESIAN_POINT`/`DIRECTION`/`VECTOR`/`LINE` and
`AXIS2_PLACEMENT_3D`/`CIRCLE` STEP entity graphs for `CadEntity::Line`/`Circle`
(`…📐️step/🔖️ap214/✳️any/🦀️component.rs:391-460`, test proves `reparsed == step` — the codec's own
retention law holds on real emitted entities); `dxf` maps all 9 `CadEntity` variants including the
two with no native DXF entity (`Ellipse`→raw `ELLIPSE` group codes, `Dimension`→raw `DIMENSION` group
codes), test confirms round trip.
**Witnessable**: Line/Circle geometry mutations via step AND dxf; Arc/Ellipse/Polyline/Text/
Insert/Solid/Dimension mutations via dxf only (step's own serializer explicitly drops everything but
Line/Circle: `_ => {} // no B-rep/solid equivalent in this bridge's scope`).
**Oracle**: `ruststep` (approved, already used for `step@ap214`/`ifc`) for step; `dxf` crate (approved)
for dxf.
**Call**: high value — two independent, different-engine-family readers (ruststep = Part-21 structural
parser; dxf = ASCII DXF parser) over the same 2D CAD content.

### `mathematical@1/any` (15) — csv only (md is a genuine stub)
`serialize` builds one real `CsvRecord{id, label(quoted), x, y}` per graph node
(`…➗️mathematical/…/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs:22-33`), documented `Lossy`: `edges`,
geometry, and `equation` never written — "a flat grid has no edge/point-cloud/expression-tree
concept." The `md` sibling is `print_dsl` (confirmed stub, not investigated further since csv already
covers the row-level fields).
**Witnessable**: node id/label/x/y create/rename/move mutations, yes; edge create/delete/rewire, no
(csv has no edge column at all); equation/expression-tree edits, no.
**Oracle**: `csv` crate (approved).
**Call**: moderate value, low effort — this is the same shape as the already-correct `sequence`
split (some kinds witnessed, some honestly not), just not yet registered.

### `semio@v1/presentation` (15) — pptx only
Real, careful mapping: `TextBox`→`PptxShape::TextBox` with paragraphs, `Picture`→`blip_rel_id` (the
relationship id, not raw bytes — `asset_id` reused verbatim), `Placeholder`→canonical
`ST_PlaceholderType` string; `Table` shapes are explicitly DROPPED (`SlideShape::Table => None`,
documented: no typed OOXML table writer without codec reimplementation)
(`…🎞️pptx/🔖️ecma-376/✳️any/🦀️component.rs:104-115`).
**Witnessable**: TextBox/Picture/Placeholder create/edit/move mutations, yes; Table-shape mutations,
no (dropped entirely, not even lossily represented); master/layout/notes edits, no field exists.
**Oracle**: `zip`+`quick-xml` (approved, same combo already used for docx/bcf).
**Call**: good value — the majority-shape-class mutations are witnessable; Table mutations are a
clean, documented exclusion rather than a silent gap.

### `draw@1/any` (14) — **svg is the ONLY real carrier**; dwg/dxf/pdf/png are all `json-structural-coercion` stubs
Read all five bodies. `svg`: real, dispatches through
`crate::artifacts::draw::io::draw_document_to_svg(from)` — a genuine bridge function
(`…🖍️draw/…/🎨️svg/🔖️1.1/✳️any/🦀️component.rs:19`). The other four are the fourth stub shape:
`dwg`/`dxf`/`pdf`/`png` each do `serde_json::to_value(from)` then `serde_json::from_value(..)` into
the target type with NO relation between `DrawSnapshot`'s and the target's field shapes — the target
struct is populated only by coincidentally-matching field names, everything else silently defaults.
This is `draw`'s OWN plugin (distinct from `semio@v1/drawing` above, which is fully real across four
formats) — same domain, very different implementation quality.
**Witnessable**: only whatever `draw_document_to_svg` actually encodes — bounded by however much of
`DrawSnapshot` that one bridge function reaches (not independently re-verified line-by-line here;
budget did not extend to that function's own body). Treat as capped until that function is read.
**Oracle**: `quick-xml` (approved) for the svg leg only. **Do not** register anything against
dwg/dxf/pdf/png here — a third-party reader would either fail outright on the coerced garbage or,
worse, accept it as a structurally-valid-but-wrong document.
**Call**: capped value (svg-only) and a second finding worth flagging on its own: `draw`'s dwg/dxf/pdf/
png export dialects are declared but functionally non-existent — the same "declares a capability it
does not have" problem the reachability report already flags for other owners, just not caught by
the original 3-shape gate.

### `semio@v1/image` (13) — bmp, gif, jpg, png, tiff
Verified `png` leg in full: builds a real `PngSnapshot{width, height, pixels: frame.rgba8.clone(),
text_chunks, chunk_order}`, validates `frame.rgba8.len() == width*height*4` before emitting, and its
own test round-trips through the REAL `png::engine::encode_png`/`decode_png`
(`…🖼️image/…/📷️png/🔖️1.2/✳️any/🦀️component.rs:44-60`, test at `:106-119`). The other four leg files
(bmp/gif/jpg/tiff) share the identical file-header pattern and doc-comment style as this verified one
and as the already-proven `gif@89a`/`gif@87a` primitives; not independently re-read line-by-line
within this pass's budget — flagged as high-confidence-by-pattern-parity, not fully re-verified.
**Witnessable**: pixel-buffer and metadata (PNG text chunks) mutations, yes, for the verified png leg;
first-frame-only for any animated content (documented: "Only the FIRST frame is exported").
**Oracle**: `image` crate (approved, already the qualifying oracle for the standalone `jpg`/`tiff`/
`bmp` subsets) — same crate, different engine family from this repo's own per-format encoders.
**Call**: good value; recommend re-verifying the bmp/gif/jpg/tiff bodies directly before registering,
since this pass verified png exactly and inferred the rest by pattern.

### `semio@v1/animation` (13) — gif, gltf, mp4 (all real, but timing-only)
All three read in full. `gltf`: builds one synthetic `GltfNode` per distinct animation-target name,
real accessor/buffer/sampler graph (`…🧊️gltf/🔖️2.0/✳️any/🦀️component.rs`); `mp4`/`gif`: BOTH
explicitly carry **timing only** — `Mp4Sample.data` is always empty and `width`/`height` are `0`
("this bridge uses ONLY the FIRST timeline's FIRST channel's keyframes… The produced track can NEVER
carry real decodable video" — `…🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs:8-11`); gif frames get real
derived `delay_cs` but `width`/`height` are `0` and `indices` is empty.
**Witnessable**: keyframe count / timing (delta-time) mutations, yes, via mp4 and gif; node/target
name and interpolation-curve-shape mutations, yes, via gltf; ANY mutation to pixel/mesh content, no —
neither carrier has any (mp4/gif structurally cannot; gltf here only ever emits synthetic empty
nodes, never real mesh geometry).
**Oracle**: `mp4` crate (approved) + `gif` crate (approved) for timing; `json` crate (approved, the
SAME engine already registered as `gltf@2.0`'s own oracle) for gltf structural validation.
**Call**: capped value — timing-focused mutations only; still a real, honest gain over zero.

### `semio@v1/model` (11) — bcf (narrow), ifc (broad)
Both read in full. `ifc`: builds a real `Part21Document` via `IfcAxis2Placement3D` absolute
placements, reads `model.relations` for `Aggregates`/`ContainedIn` but explicitly drops
`ConnectsTo`/`FillsVoid`/`VoidsElement`/`Other` (`…🏗️ifc/🔖️4/✳️any/🦀️component.rs`, doc lines
5-16). `bcf`: intentionally narrow — only reconstructs elements classed
`Other{"BcfTopic"}` with `Pset_BcfTopic`/`Pset_BcfComments` properties, and only
`Other{"BcfReferences"}` relations become viewpoint selections; `model.spatial` and every other
element/relation kind are silently dropped by design (`…💬️bcf/🔖️2.1/✳️any/🦀️component.rs:8-16`,
test `non_topic_elements_and_spatial_are_dropped_not_forced` proves this deliberately).
**Witnessable**: spatial-tree create/rename/reparent, general element create/edit, pset property
sets, `Aggregates`/`ContainedIn` relation edits — all via ifc; ONLY `BcfTopic`-classed element +
`BcfReferences`-relation mutations via bcf.
**Oracle**: `ruststep` (approved) for ifc — this is the broad, high-value leg; `zip`+`quick-xml`
(approved) for the bcf leg's own zip container, once/if BCF-shaped content is the target.
**Call**: good value via ifc alone (covers the bulk of an 11-mutation general model-editing surface).

### `forms@1/any` (10) — **csv only**; xlsx/zip both `Err("FormsIntoXlsx/Zip: not implemented")`
`csv`: real, well-tested — one header row plus one `{id, stepId, label, kind, required}` row per
question, has its OWN passing unit test in the same file
(`…📋️forms/…/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs:22-30`, test `:36-49`, `question_count + 1` rows
asserted). `xlsx`/`zip` are literal `Err(...)`, not stubs pretending to work — cleanest possible
signal, no ambiguity.
**Witnessable**: question id/stepId/label/kind/required create/edit/reorder mutations, yes;
`options`/`condition`/`params`/`default` per-question config fields, no (documented dropped, "a flat
grid has no place for them" — same honesty as `mathematical`); `schema`/`id`/`version`/`title`
document-level mutations, no.
**Oracle**: `csv` crate (approved).
**Call**: moderate value, capped to the question-identity/label/kind/required surface (a fraction of
10) — still real and worth registering.

### `semio@v1/audio` (10) — **wav only**; mp3 errors on any real content
`wav`: real, precise — ALWAYS emits `WavData::Float32` (documented: the only lossless encoding for
this artifact's `f32` samples), fmt fields (`audio_format=3`, `bits_per_sample=32`, `block_align`,
`byte_rate`) all correctly derived from real channel data, and the file's own test proves a lossless
`audio → wav → audio` fixpoint for every field except the intentionally-dropped `tags`
(`…🔊️wav/🔖️riff-pcm/✳️any/🦀️component.rs:26-33`, test `:60-71`). `mp3`: **honestly refuses** —
`serialize` returns `Err("… no MP3 encoder exists in this repository …")` for ANY snapshot with real
sample content, and only succeeds for an already-empty/silent snapshot
(`…🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️component.rs:26-38`, its own test
`real_samples_honestly_error_rather_than_fabricate_compressed_frames` proves this). mp3 is real code
but functionally witnesses **zero** mutations with actual audio content — do not count it as a
carrier for this subset's purposes.
**Witnessable**: sample-data/sample-rate/channel-count mutations, yes, via wav; `tags`/metadata
mutations, no (documented dropped on export).
**Oracle**: `hound` crate (approved) — a dedicated WAV PCM/float reader, different engine family
from this repo's own `wav` codec.
**Call**: good value on the wav leg; the mp3 leg should be recorded as a **documented non-carrier**,
not silently omitted, since it's real code that simply cannot discharge anything here.

### `semio@v1/video` (9) — mp4, avi
Both read in full, and unlike `animation`'s mp4/gif, these carry REAL sample bytes: `mp4`'s
`Mp4Sample{data: sample.data.clone(), duration, sync: sample.key}` per real stream sample
(`…🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs`); `avi`'s `AviStreamHeader`/`AviMainHeader` built from
real per-stream `rate`/`width`/`height`/sample count, with a documented, honest `Subtitle`→`"auds"`
fcc-type fold (AVI has no dedicated subtitle stream type) and a documented single-global-header
cardinality compromise when there is more than one stream.
**Witnessable**: stream sample data/timing/dimension mutations, yes, via both; per-stream metadata
that AVI's single global header cannot carry when there are 2+ streams, no (documented).
**Oracle**: `mp4` crate (approved) for mp4; `riff` crate (approved — AVI IS a RIFF container) for
avi's container structure.
**Call**: good value — real sample payloads, not just timing skeletons, unlike `animation`.

### `semio@v1/value` (9) — xml, **conditional real carrier**
Read in full. This is a generic value-graph editor (`Map`/`List`/`Str`/`Int`/`Float`/`Bool`/`Ref`).
The xml bridge requires the graph to ALREADY conform to a specific tagged-map "document" convention
(`kind: "element"/"text"/"cdata"/"comment"/"pi"`); anything that doesn't is a **hard `PackError`**,
never silently coerced or defaulted (`…📰xml/🔖️1.0/✳️any/🦀️component.rs:8-16`, own test
`non_conforming_shape_is_a_hard_error_not_a_silent_default` proves this). This is real, careful code
— not a stub — but it only witnesses mutations on value graphs that are already XML-shaped (e.g. a
document round-tripped in from XML and then mutated), not arbitrary value-tree edits.
**Witnessable**: element/attribute/text/cdata/comment/pi edits on an XML-shaped value graph, yes
(exactly, via the same convention the deserializer uses); edits that produce a non-XML-shaped value
graph (e.g. an arbitrary nested `Map`/`List` with no `kind` tag), no — the export would simply error,
which is itself a correct, verifiable outcome (`MutationOutcome::error`) rather than silence.
**Oracle**: `quick-xml` (approved).
**Call**: moderate value, genuinely capped by the fixture's own shape — register it against
XML-sourced fixtures specifically, not the whole mutation surface.

### `writer@1/any` (4) — docx, pdf — **new finding, prior research missed this**
`wave2` research (this ticket, `oracle-research-wave2.md`) already found `writer`'s JSON export is
`Exact` fidelity and proposed `jsonschema`+`deepdiff` for 3 of 4 kinds — but JSON is excluded from
"real carrier" by this protocol's own rule (a JSON export of our own schema validates shape, not
correctness). What that pass did not check: `writer` ALSO exports real, non-JSON docx and pdf.
Confirmed by reading both bodies: docx builds one real paragraph per source line via
`writer::engine::build_minimal_docx`, own test decodes and asserts `!body.is_empty()`
(`…✒️writer/…/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs:19-22`, test `:31-40`); pdf builds a real
single-page `PageDoc{text: writer_text(from)}`, own test round-trips through the REAL
`encode_pack`/`decode_pack` and asserts `decoded.page.text == "hello"`
(`…📄️pdf/🔖️1.4/✳️any/🦀️component.rs:20-22`, test `:29-38`).
**Witnessable**: `edit-text` (the mutation `wave2` could NOT verify via JSON, since JSON carries the
text as an opaque composed-child handle) IS witnessable here — both docx and pdf carry the writer's
actual rendered text (`writer_text(from)`), not a handle. `rename-writer`/`change-uri`/
`change-language` touch fields (`id`/`uri`/`languageId`) that neither docx nor pdf carries at all (both
only encode `document`'s text) — the inverse coverage of what `wave2` found for JSON.
**Oracle**: `quick-xml`+`zip` (approved) for docx; `lopdf` (approved) for pdf.
**Call**: real, worth registering, and it discharges the ONE kind (`edit-text`) that the previously-
proposed JSON oracle explicitly could not — the two approaches are complementary, not redundant:
JSON-based (once un-excluded for a supplemental role) covers the 3 flat scalars, docx/pdf covers
`edit-text`.

---

## Tier C — real carrier, but already correctly declined or too thin to be worth a new registration

* **`txt@utf-8/any` (5)** — carrier is real (raw UTF-8 byte echo) but `wave2`'s own investigation
  (already in this ticket) found the closest oracle family (ICU/Unicode newline conformance)
  **actively disagrees** with this subset's own LF/CRLF-only policy, and independently probed the
  `csv` crate to silently drop blank-line records. Already the correct call — nothing to add.
* **`binary@raw/any` (5)** — carrier is a pure identity function (`Ok(from.clone())`); there is no
  format here for any reader to be "correct" or "wrong" about. Already correctly declined
  (`raw-buffer-no-format`).
* **`dwg@ac1018/any` (3)** / **`dwg@ac1024/any` (3)** — both call a real, working `encode_dwg`, so the
  carrier is genuinely real, but DWG is Autodesk's proprietary format; no crate in the repo's
  approved `test-oracle` roster reads it, and the only realistic open readers (ODA's own SDK,
  LibreDWG bindings) are not present or approved here. Already correctly declined
  (`dwg-ac1018/1024-proprietary-container`) — would need a genuinely new, separately-vetted
  dependency to change, which is out of scope for "already approved."
* **`playground@1/any` (1)** — the csv leg is now genuinely real (a prior bug that silently emitted
  an empty table was fixed — `…🎪️playground/…/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs:9-16`), but the
  document is one string field with a 1:1 echo mutation: any reader confirming the round trip is
  checking string identity through a format, not a computed result. Consistent with `wave2`'s
  "decorative" call on this owner — technically registerable, not worth the ceremony.

---

## What this changes about the reachability headline

The original measurement said 55 subsets / 664 mutations have "a real, non-JSON standard-format
carrier." Reading every candidate body directly (not the directory tree, and not a single
`print_dsl`/`decode_pack` regex) finds a **more accurate, smaller, and differently-shaped** set for
what's left after excluding the 8 already-claimed subsets and the 6 already-fully-discharged ones:

* **9 subsets lost their only real carrier entirely** once the `serde_json::to_value`/`from_value`
  coercion and "xml bridge stub" shapes were counted: `shooting` (31), `dag` (14), `wires` (10),
  `flow` (10), `present` (9), `vcs` (6), `curate` (3), `s-home` (1) — none of these have ANY real
  external carrier, full stop, contrary to a naive regex pass.
* **2 subsets gained a real carrier back** that a naive `print_dsl` search wrongly flagged as stub
  because it fired on `#[cfg(test)]` verification code instead of the actual export path:
  `semio@v1/cad → step` and `semio@v1/drawing → svg`.
* **`bcf`/`docx`/`xlsx`/`pptx`** each lose one of their two declared formats (`xml`) to the same
  default-returning stub shape, leaving `zip` as their only real carrier — which happens to match
  what's already registered for all four.

The net remaining, genuinely-real, not-yet-fully-discharged surface is **35 subsets, ≈364
mutations**, of which roughly a third (12 subsets, ≈155 mutations) need a NEW oracle decision and the
rest (a chosen oracle already on file) need only the fixtures/probes/manifest work the `mesh`/`brep`
pilots already prove is mechanical.
