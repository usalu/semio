# Shard A7 — stdio gif/obj/dxf/las/bcf/avi subset splits

Territory: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎞️gif,🧊️obj,🖊️dxf,☁️las,💬️bcf,📼️avi}`.
Breach: `unsplit-artifact-subset` (116 baseline: gif 33, obj 22, dxf 19, las 15, bcf 14, avi 13).

## Verdicts

### GIF 87a — single (subsetPolicy: "single")
The CompuServe **GIF87a Specification** (1987) defines no extension mechanism and no conformance
classes: a stream is Header + Logical Screen Descriptor + optional Global Color Table + Image
Descriptor blocks, nothing narrower. Extension blocks (the only real internal structure GIF ever
grows) arrive only in GIF89a. Declared `"subsetPolicy": "single"` with rationale in
`🏅️standards/🔖️87a/🪆️subsets/🔣️.json`. No files moved.

### GIF 89a — split into 4: base, graphic-control, comment, application
The **GIF89a Specification** (1990) §23–§26 defines the Graphic Control (0x21 0xF9), Comment
(0x21 0xFE) and Application (0x21 0xFF) Extensions as self-contained optional blocks, distinct from
the core Logical Screen Descriptor / Image Descriptor / color-table / raster-data structure GIF87a
already had.
- `base` (12): no-mutation, set-snapshot, set-screen-size, set-global-color-table,
  set-background-color-index, set-pixel-aspect-ratio, insert/remove/move-frame,
  set-frame-geometry/pixels/interlace.
- `graphic-control` (4): set-frame-delay/disposal/transparency/user-input.
- `comment` (2): insert/remove-comment.
- `application` (3): add/remove-app-extension, set-loop-count (the de-facto NETSCAPE2.0 looping
  sub-block).

### OBJ 3.0 — split into 2: geometry, material
Wavefront **OBJ** has no conformance classes; the only real scope split is geometry statements
(v/vt/vn/f/g/o/s) vs. the `mtllib`/`usemtl` statements that reference a companion `.mtl` file rather
than declaring this document's own geometry.
- `geometry` (20): everything except the two below (incl. set-snapshot/no-mutation/
  set-smoothing-groups/set-unknown-statements).
- `material` (2): set-mtllib, set-usemtl.

### DXF R12 — split into 4: header, tables, blocks, entities
The **AutoCAD DXF Reference (R12)** (Autodesk, 1992) defines a file body as the normative section
sequence HEADER / TABLES / BLOCKS / ENTITIES / EOF.
- `header` (4): no-mutation, set-snapshot, set/remove-header-var.
- `tables` (9): LAYER/STYLE/LTYPE insert/remove/set (9 mutations).
- `blocks` (3): insert/remove/set-block.
- `entities` (3): insert/remove/set-entity.

### LAS 1.0 — split into 3: header, vlr, points
The **ASPRS LAS 1.0 Specification** defines exactly this 3-part file layout: Public Header Block
(§2.2), Variable Length Records (§2.3), Point Data Records (§2.4, format 0/1 in 1.0). The
`LasArtifact` schema struct already models `header`/`vlrs`/`points` as its three top-level fields.
- `header` (9): no-mutation, set-snapshot, set-version, set-system-identifier, set-software-info,
  set-creation-date, set-scale-and-offset, set-bounds, set-points-by-return.
- `vlr` (3): insert/remove-vlr, set-vlr-data.
- `points` (3): insert/remove-point, set-point.

### BCF 2.1 — split into 3: markup, viewpoint, snapshot
**buildingSMART BCF-XML 2.1** stores, per topic folder, `markup.bcf` (topic + comments),
`viewpoint.bcfv` (camera + visibility/coloring components) and `snapshot.png` as three distinct
files.
- `markup` (9): no-mutation, set-snapshot, set-version, insert/remove-topic, set-topic-markup,
  insert/remove-comment, set-comment.
- `viewpoint` (4): insert/remove-viewpoint, set-viewpoint-camera, set-viewpoint-components.
- `snapshot` (1): set-viewpoint-snapshot.

### AVI 1.0 — split into 3: hdrl, movi, idx1
**Video for Windows AVI 1.0** (RIFF `'AVI '` = `hdrl movi idx1`) defines exactly these three
top-level LIST/chunks. A video/audio *stream-type* split was considered (per the brief's hint) and
rejected: `strh`/`strf` are one shared record shape for both stream kinds, distinguished only by a
runtime `fccType` value, not a separate schema location — `hdrl`/`movi`/`idx1` is the format's real,
normative structure.
- `hdrl` (7): no-mutation, set-snapshot, set-main-header, insert/remove-stream,
  set-stream-header/format.
- `movi` (5): insert/remove-chunk, set-chunk-keyframe, add/remove-unknown-chunk.
- `idx1` (1): set-idx1-present.

## What moved, and what stayed shared

Per subset, the mutation's own `🧬️schema/🧬️mutations/<mutation>/` directory (payload struct +
tests) physically moved to its new subset folder, and each new subset got its own
`🧪️oracle/🔣️.json` (`mutationManifests` v2 with the moved mutations, subset field matching its own
folder, `mutationCatalogs` v1 kinds list, and — where a mutation's oracle differs from the base
subset's default reader — its own `@oracle`-matching capability declared on that oracle). The
`GifMutation`/`ObjMutation`/`DxfMutation`/`LasMutation`/`BcfMutation`/`AviMutation` enum, its
apply/diff dispatch and the real binary/text codec stayed in the base subset (renamed from `✳️any`):
these formats have no alternate standalone document profile the way PDF/A or STEP's conformance
classes do — a LAS/GIF/DXF/OBJ/BCF/AVI file is always ONE coherent document, so the whole-file
codec is one shared, always-co-resident implementation, reused rather than copied (same
family-module precedent this repo already uses for svg tiny/basic). The base aggregator's
`#[path]` mounts to the moved leaf files were repointed to the new sibling subset folders.

The artifact-level `🧪️tests/mutate-<artifact>-<std>/🥒️.feature` (+ its `🦀️.rs` adapter), which
previously exercised the WHOLE mutation vocabulary against one catalog, was split the same way PDF
already does it (`mutate-pdf-1-7-a`, `-h`, `-vt`, …): the base feature file's Examples tables were
trimmed to its own kinds, and one new `mutate-<artifact>-<std>-<subset>/` case was created per
satellite subset, each with its own `@capability`/`@oracle`/`@comparison`/`@mutations` tags and its
own Examples rows, plus a full copy of the base adapter (`🦀️.rs`) so oracle/subject dispatch keeps
working — dispatch didn't change, only the `KINDS`/registration scope differs by which scenarios
live in that particular file.

Live cross-references fixed for the rename: both stdio `📦️lib.rs`/`🦀️.rs` `#[path]` mount trees
(plugin + oracle crate), `✏️s/🔌️plugins/🔒️policy-allowlist.json`, the moved leaves' own `owner`
fields, a handful of `@see`-style doc comments in sibling artifacts (avi/docx/note referencing
bcf's old path), and the two DXF/OBJ artifact-level feature files' `asset://` fixture URIs.

## Compile evidence

- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` (standalone crate, mounts every one of
  these six artifacts' `🧪️oracle/🦀️.rs` files via `#[path]`, independent of the framework-plugin
  dependency chain): `cargo check --lib --features oracles` — **clean**, 3 pre-existing warnings
  unrelated to this shard (deprecated `quick_xml` method in svg/markup, unused docx const).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` (`semio-s-plugin-stdio`): could not get a clean full
  compile — blocked by a **pre-existing, unrelated** error in its dependency
  `semio-framework-plugin` (`dsl::io_schema::IoPayload: serde::Deserialize` not satisfied, in
  `⚛️reactor/💼️jobs/🦀️.rs`). Confirmed unrelated: `🧰️framework/…/🔌️plugin/🖥️host/🦀️.rs` shows
  modified/uncommitted in `git status` throughout this session (another live session's
  in-progress work), and grepping the full error output for gif/obj/dxf/las/bcf/avi or their
  mutation enum names returns zero hits both before and after every edit in this shard.

## Gate: before / after (`unsplit-artifact-subset`, `wildcard-subset-owner`, `duplicate-mutation-owner`)

| Artifact | Before | After |
|---|---|---|
| gif (87a+89a) | 33 | 0 |
| obj | 22 | 0 |
| dxf | 19 | 0 |
| las | 15 | 0 |
| bcf | 14 | 0 |
| avi | 13 | 0 |
| **Total** | **116** | **0** |

`wildcard-subset-owner` and `duplicate-mutation-owner`: 0 for all six both before and after.

Other breach classes touching these six paths after the final gate run, all confirmed pre-existing
/ unaffected by this shard's edits (identical counts across every re-run regardless of what this
shard changed): `stub-serializer` 25, `runtime-inventory-missing` 20 (every stdio subset here,
including untouched gif@87a, has never had `test inventory` run for it — requires the currently
blocked compile, out of this shard's scope), `binary-protocol-drift` 7, `fixture-digest-mismatch` 1
(obj `pattern-shell.obj`, byte-identical before and after a plain directory `mv`, pre-existing
drift unrelated to subset ownership).

## Files touched (non-exhaustive, by kind)

- `🏅️standards/🔖️<v>/🪆️subsets/🔣️.json` — rewritten per artifact (gif 87a, gif 89a, obj, dxf, las,
  bcf, avi).
- `🏅️standards/🔖️<v>/🪆️subsets/✳️<any→base-name>/…` — renamed folder per artifact + internal
  `✳️any`→new-name text fixed throughout.
- New `🏅️standards/🔖️<v>/🪆️subsets/✳️<satellite>/🧬️schema/🧬️mutations/<mutation>/…` +
  `🧪️oracle/🔣️.json` (+ `🧫️fixtures/<mutation>-applied/…` where a per-mutation fixture existed) for
  every satellite subset: gif (graphic-control, comment, application), obj (material), dxf (tables,
  blocks, entities), las (vlr, points), bcf (viewpoint, snapshot), avi (movi, idx1).
- `🗿️artifacts/<fmt>/🧪️tests/mutate-<fmt>-<std>/🥒️.feature` trimmed; new
  `mutate-<fmt>-<std>-<subset>/{🥒️.feature,🦀️.rs}` per satellite.
- `✏️s/🔌️plugins/🔒️policy-allowlist.json`,
  `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`,
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs`.

Scripts used for the mechanical parts of this shard are kept in this ticket folder:
`🔨️a7-split-feature-scenarios.py` (Gherkin Examples-table splitter) and
`🔨️a7-split-feature-scenarios-driver.py` (per-artifact config that drove it for all six).

## Final numbers

unsplit-artifact-subset / wildcard-subset-owner / duplicate-mutation-owner, my six artifacts:
**before 116, after 0** (gif 33→0, obj 22→0, dxf 19→0, las 15→0, bcf 14→0, avi 13→0).
